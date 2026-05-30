use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Scheme {
    pub id: Uuid,
    pub name: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for Scheme {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            active: row.try_get("active")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for Branch {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Semester {
    pub id: Uuid,
    pub scheme_id: Uuid,
    pub branch_id: Uuid,
    pub semester_number: i32,
    pub created_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for Semester {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            scheme_id: row.try_get("scheme_id")?,
            branch_id: row.try_get("branch_id")?,
            semester_number: row.try_get("semester_number")?,
            created_at: row.try_get("created_at")?,
        })
    }
}
