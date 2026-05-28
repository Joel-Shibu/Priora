export interface Scheme {
  id: string
  name: string
  active: boolean
}

export interface Branch {
  id: string
  name: string
}

export interface Semester {
  id: string
  scheme_id: string
  branch_id: string
  semester_number: number
}

export interface Subject {
  id: string
  scheme_id: string
  branch_id: string
  semester_id: string
  subject_code: string
  subject_name: string
  active: boolean
}

export interface Topic {
  id: string
  topic_name: string
  normalized_name: string | null
  difficulty: string | null
}

export interface Module {
  id: string
  module_index: number
  module_name: string
  summary: string | null
  topics: Topic[]
}

export interface SubjectDetail {
  subject: Subject
  modules: Module[]
}

export interface Question {
  id: string
  question_paper_id: string
  question_text: string
  marks: number
  order_index: number
  normalized_question: string | null
}

export interface QuestionPaper {
  id: string
  subject_id: string
  exam_year: number
  exam_term: string | null
  source_type: string
  questions: Question[]
}

export interface TopicRanking {
  topic_id: string
  topic_name: string
  normalized_name: string
  module_name: string
  priority_score: number
  frequency_count: number
  total_marks: number
  avg_marks: number
  last_seen_year: number | null
  reasons: string[]
}

export interface PriorityBuckets {
  high: TopicRanking[]
  medium: TopicRanking[]
  low: TopicRanking[]
}

export interface AnalysisResult {
  analysis_id: string
  subject_name: string
  subject_code: string
  days_remaining: number
  total_topics: number
  confidence: string
  priority_buckets: PriorityBuckets
  topics: TopicRanking[]
  generated_at: string
}

export interface TopicStats {
  frequency_count: number
  total_marks_count: number
  avg_marks: number
  last_seen_year: number | null
  priority_score: number | null
}

export interface AdminStats {
  subjects: number
  papers: number
  topics: number
  mappings: number
}
