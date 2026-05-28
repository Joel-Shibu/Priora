use axum::{extract::Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::ApiError;

#[derive(Debug, Deserialize)]
pub struct CreateSubjectRequest {
    pub scheme_id: Uuid,
    pub branch_id: Uuid,
    pub semester_id: Uuid,
    pub subject_code: String,
    pub subject_name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateModuleRequest {
    pub subject_id: Uuid,
    pub module_index: i32,
    pub module_name: String,
    pub summary: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTopicRequest {
    pub module_id: Uuid,
    pub topic_name: String,
    pub normalized_name: Option<String>,
    pub difficulty: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UploadQuestionPaperRequest {
    pub subject_id: Uuid,
    pub exam_year: i32,
    pub exam_term: Option<String>,
    pub source_type: String,
    pub questions: Vec<QuestionInput>,
}

#[derive(Debug, Deserialize)]
pub struct QuestionInput {
    pub question_text: String,
    pub marks: i32,
    pub order_index: i32,
}

#[derive(Debug, Deserialize)]
pub struct QuestionTopicMapRequest {
    pub question_id: Uuid,
    pub topic_id: Uuid,
    pub confidence: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct CreatedResponse {
    pub id: Uuid,
    pub message: String,
}

/// Validate that a string field is non-empty and within a max length.
fn validate_field(value: &str, name: &str, max_len: usize) -> Result<(), ApiError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(ApiError::BadRequest(format!("{} must not be empty", name)));
    }
    if trimmed.len() > max_len {
        return Err(ApiError::BadRequest(format!(
            "{} must not exceed {} characters",
            name, max_len
        )));
    }
    Ok(())
}

pub async fn create_subject(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<CreateSubjectRequest>,
) -> Result<Json<CreatedResponse>, ApiError> {
    validate_field(&req.subject_code, "Subject code", 20)?;
    validate_field(&req.subject_name, "Subject name", 255)?;
    let id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO subjects (id, scheme_id, branch_id, semester_id, subject_code, subject_name, active) \
         VALUES (uuid_generate_v4(), $1, $2, $3, $4, $5, true) RETURNING id",
    )
    .bind(req.scheme_id)
    .bind(req.branch_id)
    .bind(req.semester_id)
    .bind(&req.subject_code)
    .bind(&req.subject_name)
    .fetch_one(&pool)
    .await?;

    Ok(Json(CreatedResponse {
        id,
        message: "Subject created successfully".into(),
    }))
}

pub async fn create_module(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<CreateModuleRequest>,
) -> Result<Json<CreatedResponse>, ApiError> {
    validate_field(&req.module_name, "Module name", 255)?;
    if req.module_index < 1 || req.module_index > 20 {
        return Err(ApiError::BadRequest(
            "Module index must be between 1 and 20".into(),
        ));
    }
    let id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO modules (id, subject_id, module_index, module_name, summary) \
         VALUES (uuid_generate_v4(), $1, $2, $3, $4) RETURNING id",
    )
    .bind(req.subject_id)
    .bind(req.module_index)
    .bind(&req.module_name)
    .bind(&req.summary)
    .fetch_one(&pool)
    .await?;

    Ok(Json(CreatedResponse {
        id,
        message: "Module created successfully".into(),
    }))
}

pub async fn create_topic(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<CreateTopicRequest>,
) -> Result<Json<CreatedResponse>, ApiError> {
    validate_field(&req.topic_name, "Topic name", 255)?;

    if let Some(ref diff) = req.difficulty {
        if !["easy", "medium", "hard"].contains(&diff.as_str()) {
            return Err(ApiError::BadRequest(
                "Difficulty must be easy, medium, or hard".into(),
            ));
        }
    }

    let id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO topics (id, module_id, topic_name, normalized_name, difficulty, active) \
         VALUES (uuid_generate_v4(), $1, $2, $3, $4, true) RETURNING id",
    )
    .bind(req.module_id)
    .bind(&req.topic_name)
    .bind(&req.normalized_name)
    .bind(&req.difficulty)
    .fetch_one(&pool)
    .await?;

    // Initialize topic_stats
    sqlx::query("INSERT INTO topic_stats (id, topic_id) VALUES (uuid_generate_v4(), $1)")
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(Json(CreatedResponse {
        id,
        message: "Topic created successfully".into(),
    }))
}

pub async fn upload_question_paper(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<UploadQuestionPaperRequest>,
) -> Result<Json<CreatedResponse>, ApiError> {
    if !["pdf", "text", "manual"].contains(&req.source_type.as_str()) {
        return Err(ApiError::BadRequest(
            "source_type must be pdf, text, or manual".into(),
        ));
    }

    if req.questions.is_empty() {
        return Err(ApiError::BadRequest(
            "Question paper must contain at least one question".into(),
        ));
    }

    if req.questions.len() > 100 {
        return Err(ApiError::BadRequest(
            "Question paper cannot exceed 100 questions".into(),
        ));
    }

    for (i, q) in req.questions.iter().enumerate() {
        if q.marks < 1 || q.marks > 100 {
            return Err(ApiError::BadRequest(format!(
                "Question {} has invalid marks: must be between 1 and 100",
                i + 1
            )));
        }
        if q.question_text.trim().is_empty() {
            return Err(ApiError::BadRequest(format!(
                "Question {} has empty text",
                i + 1
            )));
        }
        if q.question_text.len() > 2000 {
            return Err(ApiError::BadRequest(format!(
                "Question {} text exceeds 2000 characters",
                i + 1
            )));
        }
    }

    if req.exam_year < 2000 || req.exam_year > 2030 {
        return Err(ApiError::BadRequest(
            "Exam year must be between 2000 and 2030".into(),
        ));
    }

    let paper_id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO question_papers (id, subject_id, exam_year, exam_term, source_type) \
         VALUES (uuid_generate_v4(), $1, $2, $3, $4) RETURNING id",
    )
    .bind(req.subject_id)
    .bind(req.exam_year)
    .bind(&req.exam_term)
    .bind(&req.source_type)
    .fetch_one(&pool)
    .await?;

    for q in &req.questions {
        sqlx::query(
            "INSERT INTO questions (id, question_paper_id, question_text, marks, order_index) \
             VALUES (uuid_generate_v4(), $1, $2, $3, $4)",
        )
        .bind(paper_id)
        .bind(&q.question_text)
        .bind(q.marks)
        .bind(q.order_index)
        .execute(&pool)
        .await?;
    }

    Ok(Json(CreatedResponse {
        id: paper_id,
        message: format!("Question paper with {} questions created", req.questions.len()),
    }))
}

pub async fn create_question_topic_map(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<QuestionTopicMapRequest>,
) -> Result<Json<CreatedResponse>, ApiError> {
    let confidence = req.confidence.unwrap_or(1.0);
    if !(0.0..=1.0).contains(&confidence) {
        return Err(ApiError::BadRequest(
            "Confidence must be between 0 and 1".into(),
        ));
    }

    let id = sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO question_topic_map (id, question_id, topic_id, confidence) \
         VALUES (uuid_generate_v4(), $1, $2, $3) RETURNING id",
    )
    .bind(req.question_id)
    .bind(req.topic_id)
    .bind(confidence)
    .fetch_one(&pool)
    .await?;

    // Update topic_stats
    update_topic_stats(&pool, req.topic_id).await?;

    Ok(Json(CreatedResponse {
        id,
        message: "Question-topic mapping created".into(),
    }))
}

async fn update_topic_stats(pool: &PgPool, topic_id: Uuid) -> Result<(), ApiError> {
    let stats = sqlx::query_as::<_, (i64, i64, Option<f64>, Option<i32>)>(
        r#"SELECT 
            COUNT(DISTINCT q.id) as freq,
            COALESCE(SUM(q.marks), 0) as total_marks,
            AVG(q.marks::float) as avg_m,
            MAX(qp.exam_year) as last_year
           FROM question_topic_map qtm
           JOIN questions q ON qtm.question_id = q.id
           JOIN question_papers qp ON q.question_paper_id = qp.id
           WHERE qtm.topic_id = $1"#,
    )
    .bind(topic_id)
    .fetch_one(pool)
    .await?;

    let (freq, total_marks, avg_marks, last_year) = stats;

    sqlx::query(
        "UPDATE topic_stats SET 
         frequency_count = $1, total_marks_count = $2, 
         avg_marks = $3, last_seen_year = $4,
         updated_at = NOW()
         WHERE topic_id = $5",
    )
    .bind(freq as i32)
    .bind(total_marks as i32)
    .bind(avg_marks.unwrap_or(0.0))
    .bind(last_year)
    .bind(topic_id)
    .execute(pool)
    .await?;

    Ok(())
}
