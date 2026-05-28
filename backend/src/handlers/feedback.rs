use axum::{extract::Extension, Json};
use sqlx::PgPool;

use crate::error::ApiError;
use crate::models::analysis::FeedbackRequest;

pub async fn submit_feedback(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<FeedbackRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if req.rating < 1 || req.rating > 5 {
        return Err(ApiError::BadRequest("Rating must be between 1 and 5".into()));
    }

    // Check if analysis exists
    let exists: (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM analyses WHERE id = $1)",
    )
    .bind(req.analysis_id)
    .fetch_one(&pool)
    .await?;

    if !exists.0 {
        return Err(ApiError::NotFound("Analysis not found".into()));
    }

    // Insert feedback.
    // Note: Auth/user system not yet implemented, so user_id is always NULL.
    // The UNIQUE(analysis_id, user_id) constraint requires user_id to be NOT NULL
    // for proper upsert behavior. Until auth is added, we always insert new rows.
    sqlx::query(
        r#"INSERT INTO analysis_feedback (id, analysis_id, rating, comment)
           VALUES (uuid_generate_v4(), $1, $2, $3)"#,
    )
    .bind(req.analysis_id)
    .bind(req.rating)
    .bind(&req.comment)
    .execute(&pool)
    .await?;

    Ok(Json(serde_json::json!({
        "message": "Feedback submitted successfully",
    })))
}
