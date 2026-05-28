-- Priora Database Schema - Initial Migration
-- KTU Academic Decision Intelligence Platform

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 1. Users
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email TEXT UNIQUE,
    display_name TEXT,
    role TEXT NOT NULL DEFAULT 'student' CHECK (role IN ('student', 'admin', 'editor')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 2. Schemes
CREATE TABLE schemes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 3. Branches
CREATE TABLE branches (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 4. Semesters
CREATE TABLE semesters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    scheme_id UUID NOT NULL REFERENCES schemes(id) ON DELETE CASCADE,
    branch_id UUID NOT NULL REFERENCES branches(id) ON DELETE CASCADE,
    semester_number INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(scheme_id, branch_id, semester_number)
);

-- 5. Subjects
CREATE TABLE subjects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    scheme_id UUID NOT NULL REFERENCES schemes(id) ON DELETE CASCADE,
    branch_id UUID NOT NULL REFERENCES branches(id) ON DELETE CASCADE,
    semester_id UUID NOT NULL REFERENCES semesters(id) ON DELETE CASCADE,
    subject_code TEXT NOT NULL,
    subject_name TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(scheme_id, branch_id, semester_id, subject_code)
);

-- 6. Modules
CREATE TABLE modules (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    subject_id UUID NOT NULL REFERENCES subjects(id) ON DELETE CASCADE,
    module_index INT NOT NULL,
    module_name TEXT NOT NULL,
    summary TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(subject_id, module_index)
);

-- 7. Topics
CREATE TABLE topics (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    module_id UUID NOT NULL REFERENCES modules(id) ON DELETE CASCADE,
    topic_name TEXT NOT NULL,
    normalized_name TEXT,
    difficulty TEXT CHECK (difficulty IN ('easy', 'medium', 'hard')),
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 8. Question Papers
CREATE TABLE question_papers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    subject_id UUID NOT NULL REFERENCES subjects(id) ON DELETE CASCADE,
    exam_year INT NOT NULL,
    exam_term TEXT,
    source_file_url TEXT,
    source_type TEXT NOT NULL DEFAULT 'manual' CHECK (source_type IN ('pdf', 'text', 'manual')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 9. Questions
CREATE TABLE questions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    question_paper_id UUID NOT NULL REFERENCES question_papers(id) ON DELETE CASCADE,
    question_text TEXT NOT NULL,
    marks INT NOT NULL CHECK (marks > 0),
    order_index INT NOT NULL,
    normalized_question TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 10. Question-Topic Mapping
CREATE TABLE question_topic_map (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    question_id UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    topic_id UUID NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    confidence DOUBLE PRECISION NOT NULL DEFAULT 1.0 CHECK (confidence >= 0 AND confidence <= 1),
    verified_by_user_id UUID REFERENCES users(id),
    verified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(question_id, topic_id)
);

-- 11. Topic Stats
CREATE TABLE topic_stats (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    topic_id UUID NOT NULL UNIQUE REFERENCES topics(id) ON DELETE CASCADE,
    frequency_count INT NOT NULL DEFAULT 0,
    total_marks_count INT NOT NULL DEFAULT 0,
    avg_marks DOUBLE PRECISION NOT NULL DEFAULT 0,
    last_seen_year INT,
    recency_score DOUBLE PRECISION NOT NULL DEFAULT 0,
    priority_score DOUBLE PRECISION NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 12. Analyses
CREATE TABLE analyses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id),
    subject_id UUID NOT NULL REFERENCES subjects(id),
    days_remaining INT NOT NULL CHECK (days_remaining >= 1 AND days_remaining <= 365),
    generated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    payload_json JSONB NOT NULL DEFAULT '{}'::jsonb
);

-- 13. Analysis Feedback
CREATE TABLE analysis_feedback (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    analysis_id UUID NOT NULL REFERENCES analyses(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id),
    rating INT NOT NULL CHECK (rating >= 1 AND rating <= 5),
    comment TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(analysis_id, user_id)
);

-- Indexes for performance
CREATE INDEX idx_semesters_scheme_branch ON semesters(scheme_id, branch_id);
CREATE INDEX idx_subjects_semester ON subjects(semester_id);
CREATE INDEX idx_subjects_scheme_branch ON subjects(scheme_id, branch_id);
CREATE INDEX idx_modules_subject ON modules(subject_id);
CREATE INDEX idx_topics_module ON topics(module_id);
CREATE INDEX idx_question_papers_subject ON question_papers(subject_id);
CREATE INDEX idx_questions_paper ON questions(question_paper_id);
CREATE INDEX idx_question_topic_map_question ON question_topic_map(question_id);
CREATE INDEX idx_question_topic_map_topic ON question_topic_map(topic_id);
CREATE INDEX idx_topic_stats_topic ON topic_stats(topic_id);
CREATE INDEX idx_analyses_user ON analyses(user_id);
CREATE INDEX idx_analyses_subject ON analyses(subject_id);
