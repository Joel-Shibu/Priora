use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// API request/response types

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzeRequest {
    pub subject_id: Uuid,
    pub days_remaining: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzeResponse {
    pub analysis_id: Uuid,
    pub subject_name: String,
    pub subject_code: String,
    pub days_remaining: i32,
    pub total_topics: usize,
    pub confidence: String,
    pub priority_buckets: PriorityBuckets,
    pub topics: Vec<TopicRanking>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriorityBuckets {
    pub high: Vec<TopicRanking>,
    pub medium: Vec<TopicRanking>,
    pub low: Vec<TopicRanking>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TopicRanking {
    pub topic_id: Uuid,
    pub topic_name: String,
    pub normalized_name: String,
    pub module_name: String,
    pub priority_score: f64,
    pub frequency_count: i32,
    pub total_marks: i32,
    pub avg_marks: f64,
    pub last_seen_year: Option<i32>,
    pub reasons: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedbackRequest {
    pub analysis_id: Uuid,
    pub rating: i32,
    pub comment: Option<String>,
}
