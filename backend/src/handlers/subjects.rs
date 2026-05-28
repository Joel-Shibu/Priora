use axum::{
    extract::{Extension, Path},
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::ApiError;
use crate::models::scheme::{Branch, Scheme, Semester};
use crate::models::subject::Subject;

pub async fn list_schemes(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Scheme>>, ApiError> {
    let schemes = sqlx::query_as::<_, Scheme>(
        "SELECT id, name, active, created_at FROM schemes WHERE active = true ORDER BY name",
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(schemes))
}

pub async fn list_branches_for_scheme(
    Extension(pool): Extension<PgPool>,
    Path(scheme_id): Path<Uuid>,
) -> Result<Json<Vec<Branch>>, ApiError> {
    let branches = sqlx::query_as::<_, Branch>(
        r#"SELECT DISTINCT b.id, b.name, b.created_at
           FROM branches b
           JOIN semesters s ON s.branch_id = b.id
           WHERE s.scheme_id = $1
           ORDER BY b.name"#,
    )
    .bind(scheme_id)
    .fetch_all(&pool)
    .await?;

    Ok(Json(branches))
}

pub async fn list_semesters_for_branch(
    Extension(pool): Extension<PgPool>,
    Path(branch_id): Path<Uuid>,
) -> Result<Json<Vec<Semester>>, ApiError> {
    let semesters = sqlx::query_as::<_, Semester>(
        "SELECT id, scheme_id, branch_id, semester_number, created_at \
         FROM semesters WHERE branch_id = $1 ORDER BY semester_number",
    )
    .bind(branch_id)
    .fetch_all(&pool)
    .await?;

    Ok(Json(semesters))
}

pub async fn list_subjects_for_semester(
    Extension(pool): Extension<PgPool>,
    Path(semester_id): Path<Uuid>,
) -> Result<Json<Vec<Subject>>, ApiError> {
    let subjects = sqlx::query_as::<_, Subject>(
        "SELECT id, scheme_id, branch_id, semester_id, subject_code, subject_name, active, created_at, updated_at \
         FROM subjects WHERE semester_id = $1 AND active = true ORDER BY subject_code",
    )
    .bind(semester_id)
    .fetch_all(&pool)
    .await?;

    Ok(Json(subjects))
}

pub async fn get_subject(
    Extension(pool): Extension<PgPool>,
    Path(subject_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let subject = sqlx::query_as::<_, Subject>(
        "SELECT id, scheme_id, branch_id, semester_id, subject_code, subject_name, active, created_at, updated_at \
         FROM subjects WHERE id = $1 AND active = true",
    )
    .bind(subject_id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| ApiError::NotFound("Subject not found".into()))?;

    let modules = sqlx::query_as::<_, (uuid::Uuid, i32, String, Option<String>)>(
        "SELECT id, module_index, module_name, summary FROM modules WHERE subject_id = $1 ORDER BY module_index",
    )
    .bind(subject_id)
    .fetch_all(&pool)
    .await?;

    let mut modules_with_topics = Vec::new();
    for (mod_id, mod_idx, mod_name, summary) in &modules {
        let topics = sqlx::query_as::<_, (uuid::Uuid, String, Option<String>, Option<String>)>(
            "SELECT id, topic_name, normalized_name, difficulty FROM topics WHERE module_id = $1 AND active = true ORDER BY topic_name",
        )
        .bind(mod_id)
        .fetch_all(&pool)
        .await?;

        modules_with_topics.push(serde_json::json!({
            "id": mod_id,
            "module_index": mod_idx,
            "module_name": mod_name,
            "summary": summary,
            "topics": topics.into_iter().map(|(id, name, norm, diff)| {
                serde_json::json!({
                    "id": id,
                    "topic_name": name,
                    "normalized_name": norm,
                    "difficulty": diff,
                })
            }).collect::<Vec<_>>(),
        }));
    }

    Ok(Json(serde_json::json!({
        "subject": subject,
        "modules": modules_with_topics,
    })))
}
