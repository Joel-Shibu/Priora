use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Subject {
    pub id: Uuid,
    pub scheme_id: Uuid,
    pub branch_id: Uuid,
    pub semester_id: Uuid,
    pub subject_code: String,
    pub subject_name: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


