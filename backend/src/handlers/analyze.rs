use axum::{extract::Extension, Json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::ApiError;
use crate::models::analysis::{AnalyzeRequest, AnalyzeResponse, PriorityBuckets, TopicRanking};
use crate::services::analysis_engine;

pub async fn analyze_subject(
    Extension(pool): Extension<PgPool>,
    Json(req): Json<AnalyzeRequest>,
) -> Result<Json<AnalyzeResponse>, ApiError> {
    // Validate input
    if req.days_remaining < 1 || req.days_remaining > 365 {
        return Err(ApiError::BadRequest(
            "Days remaining must be between 1 and 365".into(),
        ));
    }

    // Get subject info
    let subject_info = sqlx::query_as::<_, (String, String)>(
        "SELECT subject_name, subject_code FROM subjects WHERE id = $1 AND active = true",
    )
    .bind(req.subject_id)
    .fetch_optional(&pool)
    .await?
    .ok_or_else(|| ApiError::NotFound("Subject not found".into()))?;

    let (subject_name, subject_code) = subject_info;

    // Get modules and topics with stats
    let topics_data = analysis_engine::load_topic_data(&pool, req.subject_id).await?;

    // Rank topics
    let mut ranked_topics = analysis_engine::rank_topics(topics_data, req.days_remaining);

    // Sort by priority score descending
    ranked_topics.sort_by(|a, b| {
        b.priority_score
            .partial_cmp(&a.priority_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Assign to priority buckets
    let total = ranked_topics.len();
    let high_count = if total <= 3 {
        total
    } else {
        (total as f64 * 0.20).ceil() as usize
    };
    let low_count = if total <= 3 {
        0
    } else {
        (total as f64 * 0.30).ceil() as usize
    };

    let high: Vec<TopicRanking> = ranked_topics.iter().take(high_count).cloned().collect();
    let low: Vec<TopicRanking> = ranked_topics
        .iter()
        .rev()
        .take(low_count)
        .cloned()
        .collect();
    let medium: Vec<TopicRanking> = ranked_topics
        .iter()
        .skip(high_count)
        .take(total - high_count - low_count)
        .cloned()
        .collect();

    // Determine confidence
    let total_questions: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM questions q \
         JOIN question_papers qp ON q.question_paper_id = qp.id \
         WHERE qp.subject_id = $1",
    )
    .bind(req.subject_id)
    .fetch_one(&pool)
    .await?;

    let confidence = if total_questions.0 > 20 {
        "High".to_string()
    } else if total_questions.0 > 5 {
        "Medium".to_string()
    } else {
        "Low".to_string()
    };

    let response = AnalyzeResponse {
        analysis_id: Uuid::new_v4(),
        subject_name,
        subject_code,
        days_remaining: req.days_remaining,
        total_topics: total,
        confidence,
        priority_buckets: PriorityBuckets {
            high,
            medium,
            low,
        },
        topics: ranked_topics,
        generated_at: chrono::Utc::now(),
    };

    // Store analysis in database
    sqlx::query(
        "INSERT INTO analyses (id, subject_id, days_remaining, payload_json) VALUES ($1, $2, $3, $4)",
    )
    .bind(response.analysis_id)
    .bind(req.subject_id)
    .bind(req.days_remaining)
    .bind(serde_json::to_value(&response).unwrap_or_default())
    .execute(&pool)
    .await?;

    Ok(Json(response))
}
