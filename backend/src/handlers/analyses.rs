use axum::{
    extract::{Extension, Path},
    Json,
};
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::ApiError;

pub async fn get_analysis(
    Extension(pool): Extension<PgPool>,
    Path(analysis_id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let row: (Value,) = sqlx::query_as(
        "SELECT payload_json FROM analyses WHERE id = $1",
    )
    .bind(analysis_id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| ApiError::NotFound("Analysis not found".into()))?;

    Ok(Json(row.0))
}
