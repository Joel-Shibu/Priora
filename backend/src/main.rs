use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use std::path::Path;
use std::net::SocketAddr;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer;
use axum::http::{HeaderName, HeaderValue};
use tracing_subscriber::EnvFilter;

mod config;
mod db;
mod error;
mod handlers;
mod models;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    // Load config
    let cfg = config::Config::from_env().expect("Failed to load configuration");

    // Connect to database
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&cfg.database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations (runtime path — avoids duplicate sqlx-core without TLS from migrate! macro)
    let migrations_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    Migrator::new(migrations_dir)
        .await
        .expect("Failed to load migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    // Seed data if empty (schemes, branches, semesters, subjects, modules, topics)
    db::seed::seed_if_empty(&pool)
        .await
        .expect("Failed to seed initial data");

    // Seed question paper data for meaningful analysis scores
    db::seed::seed_question_papers(&pool)
        .await
        .expect("Failed to seed question papers");

    tracing::info!("Database connected, migrations complete, and data seeded");

    // Build CORS layer — restrict origin in production via CORS_ORIGIN env var
    let cors = if cfg.cors_origin == "*" {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    } else {
        CorsLayer::new()
            .allow_origin(
                cfg.cors_origin
                    .parse::<HeaderValue>()
                    .expect("Invalid CORS_ORIGIN value"),
            )
            .allow_methods(Any)
            .allow_headers(Any)
    };

    // Security headers
    let x_content_type = SetResponseHeaderLayer::overriding(
        HeaderName::from_static("x-content-type-options"),
        HeaderValue::from_static("nosniff"),
    );
    let x_frame = SetResponseHeaderLayer::overriding(
        HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("DENY"),
    );
    let xss_protection = SetResponseHeaderLayer::overriding(
        HeaderName::from_static("x-xss-protection"),
        HeaderValue::from_static("1; mode=block"),
    );
    let referrer_policy = SetResponseHeaderLayer::overriding(
        HeaderName::from_static("referrer-policy"),
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    let csp = SetResponseHeaderLayer::overriding(
        HeaderName::from_static("content-security-policy"),
        HeaderValue::from_static(
            "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self' fonts.gstatic.com; connect-src 'self'"
        ),
    );

    // Build middleware stack
    let middleware = ServiceBuilder::new()
        // Set sensitive headers (authorization, etc.) to not be logged
        .layer(SetSensitiveRequestHeadersLayer::new(
            vec![
                HeaderName::from_static("authorization"),
                HeaderName::from_static("cookie"),
                HeaderName::from_static("x-api-key"),
            ],
        ))
        // Request body size limit
        .layer(RequestBodyLimitLayer::new(cfg.max_body_size))
        // Request timeout
        .layer(TimeoutLayer::new(Duration::from_secs(cfg.request_timeout_secs)))
        // Security headers
        .layer(x_content_type)
        .layer(x_frame)
        .layer(xss_protection)
        .layer(referrer_policy)
        .layer(csp)
        .into_inner();

    // Build router
    let app = Router::new()
        // Health check
        .route("/api/health", get(|| async {
            axum::Json(serde_json::json!({
                "status": "ok",
                "service": "priora-api",
                "timestamp": chrono::Utc::now().to_rfc3339(),
            }))
        }))
        // Subject routes
        .route(
            "/api/schemes",
            get(handlers::subjects::list_schemes),
        )
        .route(
            "/api/schemes/:id/branches",
            get(handlers::subjects::list_branches_for_scheme),
        )
        .route(
            "/api/branches/:id/semesters",
            get(handlers::subjects::list_semesters_for_branch),
        )
        .route(
            "/api/semesters/:id/subjects",
            get(handlers::subjects::list_subjects_for_semester),
        )
        .route(
            "/api/subjects/:id",
            get(handlers::subjects::get_subject),
        )
        .route(
            "/api/subjects/:id/analyze",
            post(handlers::analyze::analyze_subject),
        )
        // Analysis retrieval
        .route(
            "/api/analyses/:id",
            get(handlers::analyses::get_analysis),
        )
        // Admin routes
        .route("/api/admin/subjects", post(handlers::admin::create_subject))
        .route("/api/admin/modules", post(handlers::admin::create_module))
        .route("/api/admin/topics", post(handlers::admin::create_topic))
        .route(
            "/api/admin/question-papers",
            post(handlers::admin::upload_question_paper),
        )
        .route(
            "/api/admin/question-topic-maps",
            post(handlers::admin::create_question_topic_map),
        )
        // Feedback routes
        .route("/api/feedback", post(handlers::feedback::submit_feedback))
        .layer(cors)
        .layer(middleware)
        .layer(Extension(pool));

    // Start server
    let addr: SocketAddr = format!("{}:{}", cfg.server_host, cfg.server_port)
        .parse()
        .expect("Invalid server address");

    tracing::info!("Priora API starting on {}", addr);
    tracing::info!(
        "Production config: cors_origin={}, timeout={}s, max_body={}B",
        cfg.cors_origin, cfg.request_timeout_secs, cfg.max_body_size
    );
    tracing::info!("Note: Rate limiting should be configured at the infrastructure level (Nginx/Cloudflare)");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // Graceful shutdown — handles SIGTERM (Fly.io, K8s) and SIGINT (Ctrl+C)
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    tracing::info!("Shutdown complete");
}

/// Wait for SIGTERM or SIGINT to initiate graceful shutdown.
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutdown signal received, draining connections...");
}
