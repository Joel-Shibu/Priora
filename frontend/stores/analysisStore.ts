import { defineStore } from 'pinia'

interface TopicRanking {
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

interface PriorityBuckets {
  high: TopicRanking[]
  medium: TopicRanking[]
  low: TopicRanking[]
}

interface AnalysisResult {
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

export const useAnalysisStore = defineStore('analysis', () => {
  const recentAnalyses = ref<AnalysisResult[]>([])
  const currentAnalysis = ref<AnalysisResult | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  const apiBase = useRuntimeConfig().public.apiBase

  async function fetchAnalysis(subjectId: string, daysRemaining: number) {
    isLoading.value = true
    error.value = null
    try {
      const data = await $fetch<AnalysisResult>(`${apiBase}/subjects/${subjectId}/analyze`, {
        method: 'POST',
        body: { subject_id: subjectId, days_remaining: daysRemaining },
      })
      currentAnalysis.value = data
      recentAnalyses.value.unshift(data)
      if (recentAnalyses.value.length > 10) {
        recentAnalyses.value = recentAnalyses.value.slice(0, 10)
      }
      return data
    } catch (e: any) {
      error.value = e?.data?.error || 'Analysis failed'
      return null
    } finally {
      isLoading.value = false
    }
  }

  function clearCurrent() {
    currentAnalysis.value = null
  }

  return {
    recentAnalyses,
    currentAnalysis,
    isLoading,
    error,
    fetchAnalysis,
    clearCurrent,
  }
})
