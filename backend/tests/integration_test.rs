// ── Priora Integration Tests ──────────────────────────────────────────
// Tests: schema constraints, seed data correctness, topic_stats, API endpoints

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

// ── Helpers ────────────────────────────────────────────────────────────

/// Connect to the test database using DATABASE_URL from env or dotenv.
async fn get_pool() -> PgPool {
    dotenvy::dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(2)
        .connect(&url)
        .await
        .expect("Failed to connect to database")
}

/// Base URL for the running API server.
fn api_base() -> String {
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "3001".into());
    let scheme = if host.starts_with("http://") || host.starts_with("https://") {
        String::new()
    } else {
        "http://".into()
    };
    format!("{scheme}{host}:{port}")
}

// ═══════════════════════════════════════════════════════════════════════
// 1. SCHEMA CONSTRAINTS
// ═══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_check_constraint_days_remaining() {
    let pool = get_pool().await;
    // Must fail: days_remaining = 0 (needs >= 1)
    let result = sqlx::query(
        "INSERT INTO analyses (id, subject_id, days_remaining, payload_json) \
         VALUES (uuid_generate_v4(), (SELECT id FROM subjects LIMIT 1), 0, '{}')",
    )
    .execute(&pool)
    .await;
    assert!(result.is_err(), "days_remaining=0 should violate CHECK constraint");

    // Must fail: days_remaining = 366 (needs <= 365)
    let result = sqlx::query(
        "INSERT INTO analyses (id, subject_id, days_remaining, payload_json) \
         VALUES (uuid_generate_v4(), (SELECT id FROM subjects LIMIT 1), 366, '{}')",
    )
    .execute(&pool)
    .await;
    assert!(result.is_err(), "days_remaining=366 should violate CHECK constraint");
}

#[tokio::test]
async fn test_check_constraint_rating() {
    let pool = get_pool().await;
    // Create analysis first
    let analysis_id = sqlx::query_scalar::<_, uuid::Uuid>(
        "INSERT INTO analyses (id, subject_id, days_remaining, payload_json) \
         VALUES (uuid_generate_v4(), (SELECT id FROM subjects LIMIT 1), 30, '{}') \
         RETURNING id",
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to create test analysis");

    // Must fail: rating = 0 (needs >= 1)
    let result = sqlx::query(
        "INSERT INTO analysis_feedback (id, analysis_id, rating) \
         VALUES (uuid_generate_v4(), $1, 0)",
    )
    .bind(analysis_id)
    .execute(&pool)
    .await;
    assert!(result.is_err(), "rating=0 should violate CHECK constraint");

    // Must fail: rating = 6 (needs <= 5)
    let result = sqlx::query(
        "INSERT INTO analysis_feedback (id, analysis_id, rating) \
         VALUES (uuid_generate_v4(), $1, 6)",
    )
    .bind(analysis_id)
    .execute(&pool)
    .await;
    assert!(result.is_err(), "rating=6 should violate CHECK constraint");
}

#[tokio::test]
async fn test_check_constraint_marks() {
    let pool = get_pool().await;
    // Must fail: marks = 0 (needs > 0)
    let paper_id = sqlx::query_scalar::<_, uuid::Uuid>(
        "SELECT id FROM question_papers LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .expect("No question papers seeded");

    let result = sqlx::query(
        "INSERT INTO questions (id, question_paper_id, question_text, marks, order_index) \
         VALUES (uuid_generate_v4(), $1, 'test', 0, 1)",
    )
    .bind(paper_id)
    .execute(&pool)
    .await;
    assert!(result.is_err(), "marks=0 should violate CHECK constraint");
}

#[tokio::test]
async fn test_check_constraint_confidence() {
    let pool = get_pool().await;
    // Must fail: confidence = 1.5 (needs <= 1)
    let question_id = sqlx::query_scalar::<_, uuid::Uuid>(
        "SELECT id FROM questions LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .expect("No questions seeded");

    let topic_id = sqlx::query_scalar::<_, uuid::Uuid>(
        "SELECT id FROM topics LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .expect("No topics seeded");

    let result = sqlx::query(
        "INSERT INTO question_topic_map (id, question_id, topic_id, confidence) \
         VALUES (uuid_generate_v4(), $1, $2, 1.5)",
    )
    .bind(question_id)
    .bind(topic_id)
    .execute(&pool)
    .await;
    assert!(result.is_err(), "confidence=1.5 should violate CHECK constraint");
}

#[tokio::test]
async fn test_check_constraint_source_type() {
    let pool = get_pool().await;
    // Must fail: source_type = 'invalid'
    let subject_id = sqlx::query_scalar::<_, uuid::Uuid>(
        "SELECT id FROM subjects LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .expect("No subjects seeded");

    let result = sqlx::query(
        "INSERT INTO question_papers (id, subject_id, exam_year, exam_term, source_type) \
         VALUES (uuid_generate_v4(), $1, 2024, 'Dec', 'invalid')",
    )
    .bind(subject_id)
    .execute(&pool)
    .await;
    assert!(result.is_err(), "source_type='invalid' should violate CHECK constraint");
}

#[tokio::test]
async fn test_check_constraint_role() {
    let pool = get_pool().await;
    // Must fail: role = 'superadmin'
    let result = sqlx::query(
        "INSERT INTO users (id, email, role) \
         VALUES (uuid_generate_v4(), 'test@test.com', 'superadmin')",
    )
    .execute(&pool)
    .await;
    assert!(result.is_err(), "role='superadmin' should violate CHECK constraint");
}

#[tokio::test]
async fn test_check_constraint_difficulty() {
    let pool = get_pool().await;
    // Must fail: difficulty = 'expert'
    let module_id = sqlx::query_scalar::<_, uuid::Uuid>(
        "SELECT id FROM modules LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .expect("No modules seeded");

    let result = sqlx::query(
        "INSERT INTO topics (id, module_id, topic_name, difficulty) \
         VALUES (uuid_generate_v4(), $1, 'test topic', 'expert')",
    )
    .bind(module_id)
    .execute(&pool)
    .await;
    assert!(result.is_err(), "difficulty='expert' should violate CHECK constraint");
}

#[tokio::test]
async fn test_unique_constraint_semesters() {
    let pool = get_pool().await;
    // Get existing semester's scheme, branch, and number
    let (scheme_id, branch_id, sem_num): (uuid::Uuid, uuid::Uuid, i32) = sqlx::query_as(
        "SELECT scheme_id, branch_id, semester_number FROM semesters LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .expect("No semesters seeded");

    // Must fail: duplicate (scheme_id, branch_id, semester_number)
    let result = sqlx::query(
        "INSERT INTO semesters (id, scheme_id, branch_id, semester_number) \
         VALUES (uuid_generate_v4(), $1, $2, $3)",
    )
    .bind(scheme_id)
    .bind(branch_id)
    .bind(sem_num)
    .execute(&pool)
    .await;
    assert!(
        result.is_err(),
        "Duplicate semester (scheme,branch,number) should violate UNIQUE constraint"
    );
}

#[tokio::test]
async fn test_unique_constraint_modules() {
    let pool = get_pool().await;
    // Get existing module's subject_id and module_index
    let (subject_id, mod_idx): (uuid::Uuid, i32) = sqlx::query_as(
        "SELECT subject_id, module_index FROM modules LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .expect("No modules seeded");

    // Must fail: duplicate (subject_id, module_index)
    let result = sqlx::query(
        "INSERT INTO modules (id, subject_id, module_index, module_name) \
         VALUES (uuid_generate_v4(), $1, $2, 'Duplicate Module')",
    )
    .bind(subject_id)
    .bind(mod_idx)
    .execute(&pool)
    .await;
    assert!(
        result.is_err(),
        "Duplicate module (subject_id, module_index) should violate UNIQUE constraint"
    );
}

#[tokio::test]
async fn test_unique_constraint_topic_stats() {
    let pool = get_pool().await;
    let topic_id = sqlx::query_scalar::<_, uuid::Uuid>(
        "SELECT id FROM topics LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .expect("No topics seeded");

    // Must fail: duplicate topic_id in topic_stats
    let result = sqlx::query(
        "INSERT INTO topic_stats (id, topic_id) VALUES (uuid_generate_v4(), $1)",
    )
    .bind(topic_id)
    .execute(&pool)
    .await;
    assert!(
        result.is_err(),
        "Duplicate topic_id in topic_stats should violate UNIQUE constraint"
    );
}

#[tokio::test]
async fn test_fk_constraint_subject_ref() {
    let pool = get_pool().await;
    // Must fail: referencing non-existent subject_id
    let fake_id = uuid::Uuid::nil();
    let result = sqlx::query(
        "INSERT INTO modules (id, subject_id, module_index, module_name) \
         VALUES (uuid_generate_v4(), $1, 99, 'Orphan Module')",
    )
    .bind(fake_id)
    .execute(&pool)
    .await;
    assert!(
        result.is_err(),
        "Inserting module with non-existent subject_id should violate FK constraint"
    );
}

// ═══════════════════════════════════════════════════════════════════════
// 2. SEED DATA CORRECTNESS
// ═══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_seed_scheme() {
    let pool = get_pool().await;
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM schemes")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1, "Should have exactly 1 scheme");

    let (name, active): (String, bool) = sqlx::query_as(
        "SELECT name, active FROM schemes LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(name, "2024 Scheme");
    assert!(active, "Scheme should be active");
}

#[tokio::test]
async fn test_seed_branch() {
    let pool = get_pool().await;
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM branches")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1, "Should have exactly 1 branch");

    let (name,): (String,) = sqlx::query_as(
        "SELECT name FROM branches LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(name, "CSE");
}

#[tokio::test]
async fn test_seed_semesters() {
    let pool = get_pool().await;
    let rows: Vec<(i32,)> = sqlx::query_as(
        "SELECT semester_number FROM semesters ORDER BY semester_number",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    let nums: Vec<i32> = rows.into_iter().map(|r| r.0).collect();
    assert_eq!(nums.len(), 8, "Should have 8 semesters");
    assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7, 8], "Semesters should be 1-8");
}

#[tokio::test]
async fn test_seed_subjects() {
    let pool = get_pool().await;
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM subjects WHERE active = true")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 32, "Should have 32 active subjects");

    // Verify S1 has 6 subjects
    let (s1_count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM subjects s \
         JOIN semesters sem ON sem.id = s.semester_id \
         WHERE sem.semester_number = 1 AND s.active = true",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(s1_count, 6, "S1 should have 6 subjects");

    // Verify S8 has 1 subject (Major Project)
    let (s8_count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM subjects s \
         JOIN semesters sem ON sem.id = s.semester_id \
         WHERE sem.semester_number = 8 AND s.active = true",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(s8_count, 1, "S8 should have 1 subject (Major Project)");
}

#[tokio::test]
async fn test_seed_subject_codes() {
    let pool = get_pool().await;
    let codes: Vec<(String,)> = sqlx::query_as(
        "SELECT subject_code FROM subjects WHERE active = true ORDER BY subject_code",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    let codes: Vec<&str> = codes.iter().map(|c| c.0.as_str()).collect();

    // Check key subject codes are present
    assert!(codes.contains(&"GAMAT101"), "Missing GAMAT101");
    assert!(codes.contains(&"GAPHT121"), "Missing GAPHT121");
    assert!(codes.contains(&"GXCYT122"), "Missing GXCYT122");
    assert!(codes.contains(&"GMEST103"), "Missing GMEST103");
    assert!(codes.contains(&"GXEST104"), "Missing GXEST104");
    assert!(codes.contains(&"UCEST105"), "Missing UCEST105");
    assert!(codes.contains(&"PCCST301"), "Missing PCCST301");
    assert!(codes.contains(&"PCCST401"), "Missing PCCST401");
    assert!(codes.contains(&"PCCST501"), "Missing PCCST501");
    assert!(codes.contains(&"PCCST801"), "Missing PCCST801");

    // Verify no old codes remain
    assert!(!codes.contains(&"EST102"), "Old EST102 should not exist");
    assert!(!codes.contains(&"EST110"), "Old EST110 should not exist");
    assert!(!codes.contains(&"EST120"), "Old EST120 should not exist");
    assert!(!codes.contains(&"MAT101"), "Old MAT101 should not exist");
}

#[tokio::test]
async fn test_seed_modules() {
    let pool = get_pool().await;
    let (count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM modules",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, 128, "Should have 128 modules (4 per subject × 32 subjects)");

    // Verify each subject has exactly 4 modules
    let rows: Vec<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) FROM modules GROUP BY subject_id",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    for (cnt,) in rows {
        assert_eq!(cnt, 4, "Each subject should have exactly 4 modules");
    }
}

#[tokio::test]
async fn test_seed_topics() {
    let pool = get_pool().await;
    let (count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM topics WHERE active = true",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, 512, "Should have 512 topics (4 per module × 128 modules)");

    // Verify each module has exactly 4 topics
    let rows: Vec<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) FROM topics WHERE active = true GROUP BY module_id",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    for (cnt,) in rows {
        assert_eq!(cnt, 4, "Each module should have exactly 4 topics");
    }
}

#[tokio::test]
async fn test_seed_question_papers() {
    let pool = get_pool().await;
    let (count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM question_papers",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, 62, "Should have 62 question papers");

    // Verify each paper has 10 questions
    let rows: Vec<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) FROM questions GROUP BY question_paper_id",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    for (cnt,) in &rows {
        assert_eq!(*cnt, 10, "Each question paper should have exactly 10 questions");
    }
}

#[tokio::test]
async fn test_seed_questions() {
    let pool = get_pool().await;
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM questions")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 620, "Should have 620 questions (62 papers × 10 questions)");
}

#[tokio::test]
async fn test_seed_question_topic_maps() {
    let pool = get_pool().await;
    let (count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM question_topic_map",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    // Each question maps to 1-2 topics. With 620 questions, we expect 744 mappings
    // (roughly 124 questions have 2 topics = 496 + 248 = 744)
    assert_eq!(count, 744, "Should have 744 question-topic mappings");
}

#[tokio::test]
async fn test_seed_topic_stats() {
    let pool = get_pool().await;
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM topic_stats")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 512, "Should have 512 topic stats rows (one per topic)");
}

#[tokio::test]
async fn test_seed_topic_stats_has_scores() {
    let pool = get_pool().await;
    let (zero_scores,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM topic_stats WHERE recency_score = 0 AND priority_score = 0",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        zero_scores, 0,
        "All 512 topics should have computed recency_score and priority_score, not zero"
    );
}

#[tokio::test]
async fn test_seed_topic_stats_scores_vary() {
    let pool = get_pool().await;
    // Verify topic scores within a subject are NOT all equal
    let subject_id = sqlx::query_scalar::<_, uuid::Uuid>(
        "SELECT s.id FROM subjects s JOIN semesters sem ON sem.id = s.semester_id \
         WHERE s.subject_code = 'GAMAT101' LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .expect("GAMAT101 not found");

    let scores: Vec<(f64, f64)> = sqlx::query_as(
        "SELECT ts.priority_score, ts.recency_score FROM topic_stats ts \
         JOIN topics t ON t.id = ts.topic_id \
         JOIN modules m ON m.id = t.module_id \
         WHERE m.subject_id = $1 \
         ORDER BY ts.priority_score DESC",
    )
    .bind(subject_id)
    .fetch_all(&pool)
    .await
    .unwrap();

    assert!(scores.len() == 16, "GAMAT101 should have 16 topics");

    // Verify at least some scores differ (not all equal)
    let first_score = scores[0].0;
    let all_same = scores.iter().all(|(s, _)| (*s - first_score).abs() < 0.01);
    assert!(
        !all_same,
        "Priority scores within a subject should vary (not all equal)"
    );

    // Verify scores are in descending order
    for i in 1..scores.len() {
        assert!(
            scores[i - 1].0 >= scores[i].0,
            "Scores should be sorted descending"
        );
    }
}

#[tokio::test]
async fn test_seed_fk_integrity() {
    let pool = get_pool().await;
    // Verify all FK relationships resolve correctly

    // Semesters reference schemes and branches
    let (ok,): (bool,) = sqlx::query_as(
        "SELECT NOT EXISTS(SELECT 1 FROM semesters s \
         LEFT JOIN schemes sch ON sch.id = s.scheme_id \
         LEFT JOIN branches b ON b.id = s.branch_id \
         WHERE sch.id IS NULL OR b.id IS NULL)",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(ok, "All semesters must reference valid schemes and branches");

    // Subjects reference schemes, branches, and semesters
    let (ok,): (bool,) = sqlx::query_as(
        "SELECT NOT EXISTS(SELECT 1 FROM subjects s \
         LEFT JOIN schemes sch ON sch.id = s.scheme_id \
         LEFT JOIN branches b ON b.id = s.branch_id \
         LEFT JOIN semesters sem ON sem.id = s.semester_id \
         WHERE sch.id IS NULL OR b.id IS NULL OR sem.id IS NULL)",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(ok, "All subjects must reference valid schemes, branches, and semesters");

    // Modules reference subjects
    let (ok,): (bool,) = sqlx::query_as(
        "SELECT NOT EXISTS(SELECT 1 FROM modules m \
         LEFT JOIN subjects s ON s.id = m.subject_id \
         WHERE s.id IS NULL)",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(ok, "All modules must reference valid subjects");

    // Topics reference modules
    let (ok,): (bool,) = sqlx::query_as(
        "SELECT NOT EXISTS(SELECT 1 FROM topics t \
         LEFT JOIN modules m ON m.id = t.module_id \
         WHERE m.id IS NULL)",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(ok, "All topics must reference valid modules");
}

#[tokio::test]
async fn test_seed_each_subject_has_modules_topics() {
    let pool = get_pool().await;
    // Verify every active subject has at least one module and each module has topics
    let subjects: Vec<(uuid::Uuid, String)> = sqlx::query_as(
        "SELECT id, subject_code FROM subjects WHERE active = true",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    for (subject_id, code) in &subjects {
        let (mod_count,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM modules WHERE subject_id = $1",
        )
        .bind(subject_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert!(
            mod_count > 0,
            "Subject {} ({}) has no modules",
            code,
            subject_id
        );

        let (topic_count,): (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM topics t \
             JOIN modules m ON m.id = t.module_id \
             WHERE m.subject_id = $1 AND t.active = true",
        )
        .bind(subject_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert!(
            topic_count > 0,
            "Subject {} ({}) has no topics",
            code,
            subject_id
        );
    }
}

#[tokio::test]
async fn test_seed_question_papers_per_subject() {
    let pool = get_pool().await;
    // S1-S2 subjects should have 3 papers, S3-S4 should have 2, S5-S7 should have 1, S8 should have 0
    let rows: Vec<(i32, i64)> = sqlx::query_as(
        "SELECT sem.semester_number, COUNT(qp.id)::bigint \
         FROM subjects s \
         JOIN semesters sem ON sem.id = s.semester_id \
         LEFT JOIN question_papers qp ON qp.subject_id = s.id \
         GROUP BY sem.semester_number \
         ORDER BY sem.semester_number",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    for (sem_num, paper_count) in &rows {
        match sem_num {
            1 => assert_eq!(
                *paper_count, 18,
                "S1 should have 18 question papers (6 subjects × 3 papers)"
            ),
            2 => assert_eq!(
                *paper_count, 15,
                "S2 should have 15 question papers (5 subjects × 3 papers)"
            ),
            3 => assert_eq!(
                *paper_count, 10,
                "S3 should have 10 question papers (5 subjects × 2 papers)"
            ),
            4 => assert_eq!(
                *paper_count, 8,
                "S4 should have 8 question papers (4 subjects × 2 papers)"
            ),
            5 => assert_eq!(
                *paper_count, 4,
                "S5 should have 4 question papers (4 subjects × 1 paper)"
            ),
            6 => assert_eq!(
                *paper_count, 4,
                "S6 should have 4 question papers (4 subjects × 1 paper)"
            ),
            7 => assert_eq!(
                *paper_count, 3,
                "S7 should have 3 question papers (3 subjects × 1 paper)"
            ),
            8 => assert_eq!(
                *paper_count, 0,
                "S8 should have 0 question papers (Major Project)"
            ),
            _ => (),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// 3. TOPIC STATS COMPUTED VALUES
// ═══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_topic_stats_frequency_range() {
    let pool = get_pool().await;
    // S1 subjects have 3 papers, so max frequency should be <= 6 (per topic, max 2 per paper)
    let (min_freq, max_freq): (i32, i32) = sqlx::query_as(
        "SELECT COALESCE(MIN(ts.frequency_count), 0), COALESCE(MAX(ts.frequency_count), 0) \
         FROM topic_stats ts \
         JOIN topics t ON t.id = ts.topic_id \
         JOIN modules m ON m.id = t.module_id \
         JOIN subjects s ON s.id = m.subject_id \
         WHERE s.subject_code = 'GAMAT101'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert!(
        max_freq >= 1,
        "GAMAT101 should have topics with frequency >= 1 (has 3 papers)"
    );
    assert!(
        min_freq >= 0,
        "Min frequency should be >= 0"
    );
}

#[tokio::test]
async fn test_topic_stats_last_seen_year() {
    let pool = get_pool().await;
    // Topics with data should have last_seen_year between 2024-2026
    let rows: Vec<(i32,)> = sqlx::query_as(
        "SELECT DISTINCT ts.last_seen_year FROM topic_stats ts \
         WHERE ts.last_seen_year IS NOT NULL ORDER BY 1",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    for (year,) in &rows {
        assert!(
            *year >= 2024 && *year <= 2026,
            "last_seen_year should be between 2024-2026, got {}",
            year
        );
    }
}

#[tokio::test]
async fn test_topic_stats_priority_formula() {
    let pool = get_pool().await;
    // Verify priority_score is between 0 and ~2.0 (reasonable range for our formula)
    let (min_score, max_score, avg_score): (f64, f64, f64) = sqlx::query_as(
        "SELECT COALESCE(MIN(priority_score), 0), COALESCE(MAX(priority_score), 0), \
         COALESCE(AVG(priority_score), 0) FROM topic_stats",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    assert!(
        min_score >= 0.0,
        "Min priority_score should be >= 0, got {}",
        min_score
    );
    assert!(
        max_score <= 2.0,
        "Max priority_score should be <= 2.0, got {}",
        max_score
    );
    assert!(
        avg_score > 0.0,
        "Average priority_score should be > 0, got {}",
        avg_score
    );
}

#[tokio::test]
async fn test_topic_stats_not_all_zero() {
    let pool = get_pool().await;
    // For subjects with question papers, at least some topics should have non-zero stats
    for subject_code in &["GAMAT101", "PCCST301", "PCCST501"] {
        let (freq_sum, marks_sum): (i64, i64) = sqlx::query_as(
            "SELECT COALESCE(SUM(ts.frequency_count)::bigint, 0), COALESCE(SUM(ts.total_marks_count)::bigint, 0) \
             FROM topic_stats ts \
             JOIN topics t ON t.id = ts.topic_id \
             JOIN modules m ON m.id = t.module_id \
             JOIN subjects s ON s.id = m.subject_id \
             WHERE s.subject_code = $1",
        )
        .bind(subject_code)
        .fetch_one(&pool)
        .await
        .unwrap();

        assert!(
            freq_sum > 0,
            "Subject {} should have total frequency_count > 0",
            subject_code
        );
        assert!(
            marks_sum > 0,
            "Subject {} should have total marks > 0",
            subject_code
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════
// 4. API ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_api_health() {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/api/health", api_base()))
        .send()
        .await
        .expect("Health check request failed");
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["status"], "ok");
    assert_eq!(body["service"], "priora-api");
    assert!(body["timestamp"].is_string());
}

#[tokio::test]
async fn test_api_schemes() {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/api/schemes", api_base()))
        .send()
        .await
        .expect("Schemes request failed");
    assert_eq!(resp.status(), 200);

    let schemes: Vec<serde_json::Value> = resp.json().await.unwrap();
    assert_eq!(schemes.len(), 1);
    assert_eq!(schemes[0]["name"], "2024 Scheme");
    assert_eq!(schemes[0]["active"], true);
}

#[tokio::test]
async fn test_api_branches() {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("{}/api/schemes", api_base()))
        .send()
        .await
        .expect("Schemes request failed");
    let schemes: Vec<serde_json::Value> = resp.json().await.unwrap();
    let scheme_id = schemes[0]["id"].as_str().unwrap();

    let resp = client
        .get(format!("{}/api/schemes/{}/branches", api_base(), scheme_id))
        .send()
        .await
        .expect("Branches request failed");
    assert_eq!(resp.status(), 200);

    let branches: Vec<serde_json::Value> = resp.json().await.unwrap();
    assert_eq!(branches.len(), 1);
    assert_eq!(branches[0]["name"], "CSE");
}

#[tokio::test]
async fn test_api_semesters() {
    let client = reqwest::Client::new();

    // Get branch ID
    let schemes: Vec<serde_json::Value> = client
        .get(format!("{}/api/schemes", api_base()))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let scheme_id = schemes[0]["id"].as_str().unwrap();

    let branches: Vec<serde_json::Value> = client
        .get(format!("{}/api/schemes/{}/branches", api_base(), scheme_id))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let branch_id = branches[0]["id"].as_str().unwrap();

    let resp = client
        .get(format!("{}/api/branches/{}/semesters", api_base(), branch_id))
        .send()
        .await
        .expect("Semesters request failed");
    assert_eq!(resp.status(), 200);

    let semesters: Vec<serde_json::Value> = resp.json().await.unwrap();
    assert_eq!(semesters.len(), 8);
    let nums: Vec<i32> = semesters
        .iter()
        .map(|s| s["semester_number"].as_i64().unwrap() as i32)
        .collect();
    assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7, 8]);
}

#[tokio::test]
async fn test_api_subjects() {
    let client = reqwest::Client::new();

    // Get semester ID
    let schemes: Vec<serde_json::Value> = client
        .get(format!("{}/api/schemes", api_base()))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let scheme_id = schemes[0]["id"].as_str().unwrap();

    let branches: Vec<serde_json::Value> = client
        .get(format!("{}/api/schemes/{}/branches", api_base(), scheme_id))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let branch_id = branches[0]["id"].as_str().unwrap();

    let semesters: Vec<serde_json::Value> = client
        .get(format!("{}/api/branches/{}/semesters", api_base(), branch_id))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let sem1_id = semesters[0]["id"].as_str().unwrap();

    // Get S1 subjects
    let resp = client
        .get(format!("{}/api/semesters/{}/subjects", api_base(), sem1_id))
        .send()
        .await
        .expect("Subjects request failed");
    assert_eq!(resp.status(), 200);

    let subjects: Vec<serde_json::Value> = resp.json().await.unwrap();
    assert_eq!(subjects.len(), 6, "S1 should have 6 subjects");

    let codes: Vec<&str> = subjects
        .iter()
        .map(|s| s["subject_code"].as_str().unwrap())
        .collect();
    assert!(
        codes.contains(&"GAMAT101"),
        "S1 subjects should include GAMAT101"
    );
    assert!(
        codes.contains(&"UCEST105"),
        "S1 subjects should include UCEST105"
    );
}

#[tokio::test]
async fn test_api_get_subject() {
    let client = reqwest::Client::new();
    let pool = get_pool().await;

    let subject_id: uuid::Uuid = sqlx::query_scalar(
        "SELECT id FROM subjects WHERE subject_code = 'GAMAT101' LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let resp = client
        .get(format!("{}/api/subjects/{}", api_base(), subject_id))
        .send()
        .await
        .expect("Get subject failed");
    assert_eq!(resp.status(), 200);

    let data: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(data["subject"]["subject_code"], "GAMAT101");
    assert_eq!(
        data["subject"]["subject_name"],
        "Mathematics for Information Science-1"
    );

    let modules = data["modules"].as_array().unwrap();
    assert_eq!(modules.len(), 4, "GAMAT101 should have 4 modules");

    for module in modules {
        let topics = module["topics"].as_array().unwrap();
        assert_eq!(topics.len(), 4, "Each module should have 4 topics");
    }
}

#[tokio::test]
async fn test_api_subject_not_found() {
    let client = reqwest::Client::new();
    let fake_id = uuid::Uuid::nil();

    let resp = client
        .get(format!("{}/api/subjects/{}", api_base(), fake_id))
        .send()
        .await
        .expect("Request failed");
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn test_api_analyze_gamat101() {
    let client = reqwest::Client::new();
    let pool = get_pool().await;

    let subject_id: uuid::Uuid = sqlx::query_scalar(
        "SELECT id FROM subjects WHERE subject_code = 'GAMAT101' LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let resp = client
        .post(format!("{}/api/subjects/{}/analyze", api_base(), subject_id))
        .json(&serde_json::json!({
            "subject_id": subject_id,
            "days_remaining": 30,
        }))
        .send()
        .await
        .expect("Analyze request failed");
    assert_eq!(resp.status(), 200);

    let result: serde_json::Value = resp.json().await.unwrap();

    // Verify response structure
    assert_eq!(result["subject_code"], "GAMAT101");
    assert_eq!(
        result["subject_name"],
        "Mathematics for Information Science-1"
    );
    assert_eq!(result["days_remaining"], 30);
    assert_eq!(result["total_topics"], 16);
    assert_eq!(result["confidence"], "High");

    // Verify priority buckets exist and are non-empty
    let buckets = &result["priority_buckets"];
    let high = buckets["high"].as_array().unwrap();
    let medium = buckets["medium"].as_array().unwrap();
    let low = buckets["low"].as_array().unwrap();

    assert!(!high.is_empty(), "High priority bucket should not be empty");
    assert!(!medium.is_empty(), "Medium priority bucket should not be empty");
    assert!(!low.is_empty(), "Low priority bucket should not be empty");

    // Verify total topics across buckets
    assert_eq!(
        high.len() + medium.len() + low.len(),
        16,
        "All 16 topics should be distributed across buckets"
    );

    // Verify high priority topics have highest scores
    let high_scores: Vec<f64> = high
        .iter()
        .map(|t| t["priority_score"].as_f64().unwrap())
        .collect();
    let low_scores: Vec<f64> = low
        .iter()
        .map(|t| t["priority_score"].as_f64().unwrap())
        .collect();

    for h_score in &high_scores {
        for l_score in &low_scores {
            assert!(
                h_score > l_score,
                "High priority scores ({}) should exceed low priority scores ({})",
                h_score,
                l_score
            );
        }
    }

    // Verify each topic has reasons
    for topic in high.iter().chain(medium.iter()).chain(low.iter()) {
        let reasons = topic["reasons"].as_array().unwrap();
        assert!(!reasons.is_empty(), "Each topic should have at least one reason");
    }
}

#[tokio::test]
async fn test_api_analyze_invalid_days() {
    let client = reqwest::Client::new();
    let pool = get_pool().await;

    let subject_id: uuid::Uuid = sqlx::query_scalar(
        "SELECT id FROM subjects LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    // days_remaining = 0 should return 400
    let resp = client
        .post(format!("{}/api/subjects/{}/analyze", api_base(), subject_id))
        .json(&serde_json::json!({
            "subject_id": subject_id,
            "days_remaining": 0,
        }))
        .send()
        .await
        .expect("Request failed");
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn test_api_analyze_nonexistent_subject() {
    let client = reqwest::Client::new();
    let fake_id = uuid::Uuid::nil();

    let resp = client
        .post(format!("{}/api/subjects/{}/analyze", api_base(), fake_id))
        .json(&serde_json::json!({
            "subject_id": fake_id,
            "days_remaining": 30,
        }))
        .send()
        .await
        .expect("Request failed");
    assert_eq!(resp.status(), 404);
}

#[tokio::test]
async fn test_api_submit_feedback() {
    let client = reqwest::Client::new();
    let pool = get_pool().await;

    // First run an analysis to create a record
    let subject_id: uuid::Uuid = sqlx::query_scalar(
        "SELECT id FROM subjects WHERE subject_code = 'GAMAT101' LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let resp = client
        .post(format!("{}/api/subjects/{}/analyze", api_base(), subject_id))
        .json(&serde_json::json!({
            "subject_id": subject_id,
            "days_remaining": 30,
        }))
        .send()
        .await
        .expect("Analyze request failed");
    assert_eq!(resp.status(), 200);

    let analysis: serde_json::Value = resp.json().await.unwrap();
    let analysis_id = analysis["analysis_id"].as_str().unwrap();

    // Submit feedback
    let resp = client
        .post(format!("{}/api/feedback", api_base()))
        .json(&serde_json::json!({
            "analysis_id": analysis_id,
            "rating": 4,
            "comment": "Very helpful analysis",
        }))
        .send()
        .await
        .expect("Feedback request failed");
    assert_eq!(resp.status(), 200);

    // Submit feedback without comment (optional field)
    let resp = client
        .post(format!("{}/api/feedback", api_base()))
        .json(&serde_json::json!({
            "analysis_id": analysis_id,
            "rating": 5,
        }))
        .send()
        .await
        .expect("Feedback without comment failed");
    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn test_api_feedback_invalid_rating() {
    let client = reqwest::Client::new();
    let pool = get_pool().await;

    let subject_id: uuid::Uuid = sqlx::query_scalar(
        "SELECT id FROM subjects LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let resp = client
        .post(format!("{}/api/subjects/{}/analyze", api_base(), subject_id))
        .json(&serde_json::json!({
            "subject_id": subject_id,
            "days_remaining": 30,
        }))
        .send()
        .await
        .expect("Analyze failed");
    let analysis: serde_json::Value = resp.json().await.unwrap();
    let analysis_id = analysis["analysis_id"].as_str().unwrap();

    // Invalid rating
    let resp = client
        .post(format!("{}/api/feedback", api_base()))
        .json(&serde_json::json!({
            "analysis_id": analysis_id,
            "rating": 6,
        }))
        .send()
        .await
        .expect("Feedback request failed");
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn test_api_feedback_nonexistent_analysis() {
    let client = reqwest::Client::new();

    let resp = client
        .post(format!("{}/api/feedback", api_base()))
        .json(&serde_json::json!({
            "analysis_id": uuid::Uuid::nil(),
            "rating": 3,
        }))
        .send()
        .await
        .expect("Feedback request failed");
    assert_eq!(resp.status(), 404);
}

// ═══════════════════════════════════════════════════════════════════════
// 5. EDGE CASES
// ═══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_api_schemes_no_branches_fake_scheme() {
    let client = reqwest::Client::new();
    let fake_id = uuid::Uuid::nil();

    // Non-existent scheme should return empty array (not error)
    let resp = client
        .get(format!("{}/api/schemes/{}/branches", api_base(), fake_id))
        .send()
        .await
        .expect("Request failed");
    assert_eq!(resp.status(), 200);

    let branches: Vec<serde_json::Value> = resp.json().await.unwrap();
    assert!(
        branches.is_empty(),
        "Non-existent scheme should return empty branches"
    );
}

#[tokio::test]
async fn test_api_empty_semester_no_subjects_fake_semester() {
    let client = reqwest::Client::new();
    let fake_id = uuid::Uuid::nil();

    let resp = client
        .get(format!(
            "{}/api/semesters/{}/subjects",
            api_base(),
            fake_id
        ))
        .send()
        .await
        .expect("Request failed");
    assert_eq!(resp.status(), 200);

    let subjects: Vec<serde_json::Value> = resp.json().await.unwrap();
    assert!(
        subjects.is_empty(),
        "Non-existent semester should return empty subjects"
    );
}

#[tokio::test]
async fn test_api_analyze_different_time_pressures() {
    let client = reqwest::Client::new();
    let pool = get_pool().await;

    let subject_id: uuid::Uuid = sqlx::query_scalar(
        "SELECT id FROM subjects WHERE subject_code = 'GAMAT101' LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    // 5 days (high time pressure = 1.5x) — should produce higher scores
    let resp_urgent = client
        .post(format!("{}/api/subjects/{}/analyze", api_base(), subject_id))
        .json(&serde_json::json!({
            "subject_id": subject_id,
            "days_remaining": 5,
        }))
        .send()
        .await
        .expect("Request failed");
    assert_eq!(resp_urgent.status(), 200);

    // 90 days (low time pressure = 1.0x) — should produce lower scores
    let resp_relaxed = client
        .post(format!("{}/api/subjects/{}/analyze", api_base(), subject_id))
        .json(&serde_json::json!({
            "subject_id": subject_id,
            "days_remaining": 90,
        }))
        .send()
        .await
        .expect("Request failed");
    assert_eq!(resp_relaxed.status(), 200);

    let urgent: serde_json::Value = resp_urgent.json().await.unwrap();
    let relaxed: serde_json::Value = resp_relaxed.json().await.unwrap();

    let urgent_high = urgent["priority_buckets"]["high"]
        .as_array()
        .unwrap();
    let relaxed_high = relaxed["priority_buckets"]["high"]
        .as_array()
        .unwrap();

    // Urgent (5 days) should have higher first high-priority score than relaxed (90 days)
    let urgent_top = urgent_high[0]["priority_score"].as_f64().unwrap();
    let relaxed_top = relaxed_high[0]["priority_score"].as_f64().unwrap();
    assert!(
        urgent_top > relaxed_top,
        "Urgent analysis (5 days) should produce higher top score ({}) than relaxed (90 days) ({})",
        urgent_top,
        relaxed_top
    );
}

#[tokio::test]
async fn test_api_analyze_scores_consistent() {
    let client = reqwest::Client::new();
    let pool = get_pool().await;

    let subject_id: uuid::Uuid = sqlx::query_scalar(
        "SELECT id FROM subjects WHERE subject_code = 'GAMAT101' LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    // Run analysis twice, scores should be identical (deterministic)
    let resp1 = client
        .post(format!("{}/api/subjects/{}/analyze", api_base(), subject_id))
        .json(&serde_json::json!({
            "subject_id": subject_id,
            "days_remaining": 30,
        }))
        .send()
        .await
        .expect("Request failed");
    let resp2 = client
        .post(format!("{}/api/subjects/{}/analyze", api_base(), subject_id))
        .json(&serde_json::json!({
            "subject_id": subject_id,
            "days_remaining": 30,
        }))
        .send()
        .await
        .expect("Request failed");

    let result1: serde_json::Value = resp1.json().await.unwrap();
    let result2: serde_json::Value = resp2.json().await.unwrap();

    let scores1: Vec<f64> = result1["priority_buckets"]["high"]
        .as_array()
        .unwrap()
        .iter()
        .chain(result1["priority_buckets"]["medium"].as_array().unwrap())
        .chain(result1["priority_buckets"]["low"].as_array().unwrap())
        .map(|t| t["priority_score"].as_f64().unwrap())
        .collect();

    let scores2: Vec<f64> = result2["priority_buckets"]["high"]
        .as_array()
        .unwrap()
        .iter()
        .chain(result2["priority_buckets"]["medium"].as_array().unwrap())
        .chain(result2["priority_buckets"]["low"].as_array().unwrap())
        .map(|t| t["priority_score"].as_f64().unwrap())
        .collect();

    assert_eq!(
        scores1, scores2,
        "Analysis should be deterministic — same inputs must produce same scores"
    );
}

// ═══════════════════════════════════════════════════════════════════════
// 6. ADMIN API ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_api_admin_subject_crud() {
    let client = reqwest::Client::new();
    let pool = get_pool().await;

    // Pre-cleanup: remove any leftover from a previous run
    let _ = sqlx::query("DELETE FROM subjects WHERE subject_code = 'TEST101'")
        .execute(&pool)
        .await;

    let (scheme_id, branch_id, sem_id): (uuid::Uuid, uuid::Uuid, uuid::Uuid) = sqlx::query_as(
        "SELECT s.scheme_id, s.branch_id, s.semester_id FROM subjects s LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    // Create a new subject
    let resp = client
        .post(format!("{}/api/admin/subjects", api_base()))
        .json(&serde_json::json!({
            "scheme_id": scheme_id,
            "branch_id": branch_id,
            "semester_id": sem_id,
            "subject_code": "TEST101",
            "subject_name": "Test Subject",
        }))
        .send()
        .await
        .expect("Create subject failed");
    assert_eq!(resp.status(), 200);

    let result: serde_json::Value = resp.json().await.unwrap();
    assert!(result["id"].as_str().unwrap().len() > 0);
    assert_eq!(result["message"], "Subject created successfully");

    // Post-cleanup: remove test subject
    let _ = sqlx::query("DELETE FROM subjects WHERE subject_code = 'TEST101'")
        .execute(&pool)
        .await;
}

#[tokio::test]
async fn test_api_admin_module_crud() {
    let client = reqwest::Client::new();
    let pool = get_pool().await;

    // Pre-cleanup: remove any leftover test module
    let _ = sqlx::query(
        "DELETE FROM modules WHERE module_index = 5 AND subject_id = \
         (SELECT id FROM subjects WHERE subject_code = 'GAMAT101')",
    )
    .execute(&pool)
    .await;

    let subject_id: uuid::Uuid = sqlx::query_scalar(
        "SELECT id FROM subjects WHERE subject_code = 'GAMAT101' LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    // Create a new module
    let resp = client
        .post(format!("{}/api/admin/modules", api_base()))
        .json(&serde_json::json!({
            "subject_id": subject_id,
            "module_index": 5,
            "module_name": "Test Module",
            "summary": "A test module for integration testing",
        }))
        .send()
        .await
        .expect("Create module failed");
    assert_eq!(resp.status(), 200);

    let result: serde_json::Value = resp.json().await.unwrap();
    assert!(result["id"].as_str().unwrap().len() > 0);
    assert_eq!(result["message"], "Module created successfully");

    // Post-cleanup: remove test module
    let _ = sqlx::query(
        "DELETE FROM modules WHERE id = $1",
    )
    .bind(uuid::Uuid::parse_str(result["id"].as_str().unwrap()).unwrap())
    .execute(&pool)
    .await;
}

#[tokio::test]
async fn test_api_admin_topic_crud() {
    let client = reqwest::Client::new();
    let pool = get_pool().await;

    // Pre-cleanup: remove any leftover test topic
    let _ = sqlx::query(
        "DELETE FROM topics WHERE topic_name = 'Test Topic'",
    )
    .execute(&pool)
    .await;

    let module_id: uuid::Uuid = sqlx::query_scalar(
        "SELECT id FROM modules LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    // Create a new topic
    let resp = client
        .post(format!("{}/api/admin/topics", api_base()))
        .json(&serde_json::json!({
            "module_id": module_id,
            "topic_name": "Test Topic",
            "normalized_name": "Test Topic (normalized)",
            "difficulty": "hard",
        }))
        .send()
        .await
        .expect("Create topic failed");
    assert_eq!(resp.status(), 200);

    let result: serde_json::Value = resp.json().await.unwrap();
    assert!(result["id"].as_str().unwrap().len() > 0);
    assert_eq!(result["message"], "Topic created successfully");

    // Verify topic_stats was auto-created
    let topic_id = uuid::Uuid::parse_str(result["id"].as_str().unwrap()).unwrap();
    let (exists,): (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM topic_stats WHERE topic_id = $1)",
    )
    .bind(topic_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(exists, "topic_stats should be auto-created for new topic");

    // Post-cleanup: remove test topic (topic_stats cascades)
    let _ = sqlx::query("DELETE FROM topics WHERE id = $1")
        .bind(topic_id)
        .execute(&pool)
        .await;
}

#[tokio::test]
async fn test_api_admin_topic_invalid_difficulty() {
    let client = reqwest::Client::new();
    let pool = get_pool().await;

    let module_id: uuid::Uuid = sqlx::query_scalar(
        "SELECT id FROM modules LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    let resp = client
        .post(format!("{}/api/admin/topics", api_base()))
        .json(&serde_json::json!({
            "module_id": module_id,
            "topic_name": "Bad Topic",
            "difficulty": "expert",
        }))
        .send()
        .await
        .expect("Create topic with bad difficulty failed");
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn test_api_admin_question_paper() {
    let client = reqwest::Client::new();
    let pool = get_pool().await;

    let subject_id: uuid::Uuid = sqlx::query_scalar(
        "SELECT id FROM subjects WHERE subject_code = 'GAMAT101' LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    // Pre-cleanup: remove any leftover test question paper
    let _ = sqlx::query(
        "DELETE FROM question_papers WHERE exam_year = 2026 AND exam_term = 'May' AND subject_id = $1",
    )
    .bind(subject_id)
    .execute(&pool)
    .await;

    // Upload a question paper with 3 questions
    let resp = client
        .post(format!("{}/api/admin/question-papers", api_base()))
        .json(&serde_json::json!({
            "subject_id": subject_id,
            "exam_year": 2026,
            "exam_term": "May",
            "source_type": "manual",
            "questions": [
                {"question_text": "Q1: Test question?", "marks": 5, "order_index": 1},
                {"question_text": "Q2: Another question?", "marks": 10, "order_index": 2},
                {"question_text": "Q3: Final question?", "marks": 15, "order_index": 3},
            ],
        }))
        .send()
        .await
        .expect("Upload question paper failed");
    assert_eq!(resp.status(), 200);

    let result: serde_json::Value = resp.json().await.unwrap();
    assert!(result["id"].as_str().unwrap().len() > 0);
    assert!(result["message"].as_str().unwrap().contains("3 questions"));

    // Post-cleanup: remove test question paper (questions cascade)
    let _ = sqlx::query(
        "DELETE FROM question_papers WHERE id = $1",
    )
    .bind(uuid::Uuid::parse_str(result["id"].as_str().unwrap()).unwrap())
    .execute(&pool)
    .await;
}
