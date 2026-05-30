use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
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

impl<'r> FromRow<'r, PgRow> for Subject {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            scheme_id: row.try_get("scheme_id")?,
            branch_id: row.try_get("branch_id")?,
            semester_id: row.try_get("semester_id")?,
            subject_code: row.try_get("subject_code")?,
            subject_name: row.try_get("subject_name")?,
            active: row.try_get("active")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}
