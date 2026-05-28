use sqlx::PgPool;
use uuid::Uuid;

use crate::models::analysis::TopicRanking;

/// Topic data loaded from the database for ranking computation
#[derive(Debug, Clone)]
pub struct TopicAnalysisData {
    pub topic_id: Uuid,
    pub topic_name: String,
    pub normalized_name: String,
    pub module_name: String,
    pub frequency_count: i32,
    pub total_marks: i32,
    pub avg_marks: f64,
    pub last_seen_year: Option<i32>,
}

/// Load all topics with their stats for a given subject
pub async fn load_topic_data(
    pool: &PgPool,
    subject_id: Uuid,
) -> Result<Vec<TopicAnalysisData>, crate::error::ApiError> {
    let rows = sqlx::query_as::<_, (Uuid, String, Option<String>, String, i32, i32, Option<f64>, Option<i32>)>(
        r#"SELECT 
            t.id,
            t.topic_name,
            t.normalized_name,
            m.module_name,
            COALESCE(ts.frequency_count, 0) as freq,
            COALESCE(ts.total_marks_count, 0) as total_marks,
            ts.avg_marks::float8 as avg_m,
            ts.last_seen_year
           FROM topics t
           JOIN modules m ON t.module_id = m.id
           LEFT JOIN topic_stats ts ON ts.topic_id = t.id
           WHERE m.subject_id = $1 AND t.active = true
           ORDER BY m.module_index, t.topic_name"#,
    )
    .bind(subject_id)
    .fetch_all(pool)
    .await?;

    let data = rows
        .into_iter()
        .map(
            |(id, name, norm, mod_name, freq, marks, avg, last_year)| {
                TopicAnalysisData {
                    topic_id: id,
                    topic_name: name.clone(),
                    normalized_name: norm.unwrap_or(name),
                    module_name: mod_name,
                    frequency_count: freq,
                    total_marks: marks,
                    avg_marks: avg.unwrap_or(0.0),
                    last_seen_year: last_year,
                }
            },
        )
        .collect();

    Ok(data)
}

/// Rank topics using a deterministic weighted scoring formula.
///
/// Score components:
/// - Frequency weight (25%): How often topic appears across papers
/// - Marks weight (30%): Total and average marks the topic carries
/// - Recency weight (25%): How recently the topic appeared
/// - Time pressure modifier (20%): Adjusts based on days remaining
///   (less days = more weight on high-frequency/high-marks topics)
pub fn rank_topics(
    topics: Vec<TopicAnalysisData>,
    days_remaining: i32,
) -> Vec<TopicRanking> {
    if topics.is_empty() {
        return vec![];
    }

    // Find max values for normalization
    let max_freq = topics
        .iter()
        .map(|t| t.frequency_count)
        .max()
        .unwrap_or(1)
        .max(1);
    let max_marks = topics
        .iter()
        .map(|t| t.total_marks)
        .max()
        .unwrap_or(1)
        .max(1);
    let current_year = 2026;

    let time_pressure = if days_remaining <= 7 {
        1.5 // high pressure - focus on safest topics
    } else if days_remaining <= 30 {
        1.2 // moderate pressure
    } else {
        1.0 // low pressure - can study broadly
    };

    topics
        .into_iter()
        .map(|t| {
            // 1. Frequency score (normalized 0-1)
            let freq_score = t.frequency_count as f64 / max_freq as f64;

            // 2. Marks score (normalized 0-1, weighted toward total)
            let marks_score = (t.total_marks as f64 / max_marks as f64) * 0.6
                + (t.avg_marks / 10.0).min(1.0) * 0.4;

            // 3. Recency score
            let recency_score = match t.last_seen_year {
                Some(year) => {
                    let years_ago = (current_year - year).max(0) as f64;
                    (1.0 - (years_ago / 5.0)).max(0.0)
                }
                None => 0.3, // no data = low confidence
            };

            // 4. Compute final priority score
            let priority_score = (freq_score * 0.25
                + marks_score * 0.30
                + recency_score * 0.25)
                * time_pressure
                + (t.frequency_count as f64 * 0.05) // small bonus for frequency
                + (t.total_marks as f64 * 0.05 / max_marks as f64 * time_pressure);

            // Generate explanation reasons
            let mut reasons = Vec::new();

            if t.frequency_count > 1 {
                reasons.push(format!(
                    "Appeared {} times across previous papers",
                    t.frequency_count
                ));
            } else if t.frequency_count == 1 {
                reasons.push("Appeared in at least one previous paper".to_string());
            }

            if t.avg_marks > 8.0 {
                reasons.push(format!(
                    "Carries high marks (avg {:.0} marks)",
                    t.avg_marks
                ));
            } else if t.avg_marks > 5.0 {
                reasons.push(format!(
                    "Carries moderate marks (avg {:.0} marks)",
                    t.avg_marks
                ));
            }

            if let Some(year) = t.last_seen_year {
                if year >= current_year - 1 {
                    reasons.push(format!("Recently appeared (last seen {})", year));
                }
            }

            if reasons.is_empty() {
                reasons.push("Foundational topic for this subject".to_string());
            }

            TopicRanking {
                topic_id: t.topic_id,
                topic_name: t.topic_name,
                normalized_name: t.normalized_name,
                module_name: t.module_name,
                priority_score: (priority_score * 100.0).round() / 100.0,
                frequency_count: t.frequency_count,
                total_marks: t.total_marks,
                avg_marks: (t.avg_marks * 100.0).round() / 100.0,
                last_seen_year: t.last_seen_year,
                reasons,
            }
        })
        .collect()
}
