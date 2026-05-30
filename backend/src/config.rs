use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub cors_origin: String,
    pub max_body_size: usize,
    pub request_timeout_secs: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        dotenvy::dotenv().ok();

        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL must be set".to_string())?,
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            server_port: env::var("SERVER_PORT")
                .or_else(|_| env::var("PORT"))
                .unwrap_or_else(|_| "3001".into())
                .parse()
                .map_err(|_| "SERVER_PORT or PORT must be a valid port number".to_string())?,
            cors_origin: env::var("CORS_ORIGIN").unwrap_or_else(|_| "*".into()),
            max_body_size: env::var("MAX_BODY_SIZE")
                .unwrap_or_else(|_| "1048576".into()) // 1 MB default
                .parse()
                .unwrap_or(1_048_576),
            request_timeout_secs: env::var("REQUEST_TIMEOUT_SECS")
                .unwrap_or_else(|_| "30".into())
                .parse()
                .unwrap_or(30),
        })
    }
}
