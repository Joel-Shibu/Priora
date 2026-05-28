<template>
  <div class="max-w-4xl mx-auto px-4 sm:px-8 py-6 sm:py-8">
    <!-- Back link -->
    <NuxtLink to="/"
      class="inline-flex items-center gap-1.5 text-xs font-medium text-[var(--color-text-tertiary)] hover:text-[var(--color-text-primary)] transition-colors mb-6 group"
    >
      <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5 transition-transform group-hover:-translate-x-0.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="15 18 9 12 15 6"/>
      </svg>
      Back to Home
    </NuxtLink>

    <!-- Page heading -->
    <h1 ref="pageHeadingRef" class="text-fluid-h2 font-bold text-[var(--color-text-primary)] mb-2">Analysis Results</h1>

    <!-- Error state -->
    <div v-if="errorMsg" class="text-center py-16">
      <div class="w-14 h-14 rounded-2xl border border-[var(--color-border)] bg-[var(--color-surface-alt)] flex items-center justify-center mx-auto mb-4">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 text-[var(--color-text-tertiary)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/>
        </svg>
      </div>
      <p class="text-sm text-[var(--color-text-secondary)]">{{ errorMsg }}</p>
      <NuxtLink to="/" class="inline-flex items-center gap-1.5 mt-3 text-sm font-medium text-[var(--color-primary-500)] hover:underline">
        Start a new analysis
        <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/>
        </svg>
      </NuxtLink>
    </div>

    <!-- Loading -->
    <div v-else-if="isLoading" class="flex flex-col items-center justify-center py-20 gap-3">
      <span class="inline-block w-8 h-8 rounded-full border-3 border-[var(--color-primary-500)]/20 border-t-[var(--color-primary-500)] animate-spin" />
      <p class="text-sm text-[var(--color-text-secondary)]">Loading analysis...</p>
    </div>

    <!-- Results -->
    <template v-else-if="analysisResult">
      <AnalysisResults :result="analysisResult" />
    </template>
  </div>
</template>

<script setup lang="ts">
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

definePageMeta({
  title: 'Analysis Results — Priora',
  description: 'View priority-ranked topic analysis with study recommendations based on KTU question paper patterns.',
})

const route = useRoute()
const apiBase = useRuntimeConfig().public.apiBase

const pageHeadingRef = ref<HTMLElement | null>(null)

useGsapTextSplit(pageHeadingRef, {
  type: 'chars',
  stagger: 0.03,
  duration: 0.5,
  direction: 'up',
  playOnMount: true,
})

const analysisResult = ref<AnalysisResult | null>(null)
const isLoading = ref(true)
const errorMsg = ref<string | null>(null)

onMounted(async () => {
  const analysisId = route.params.id as string

  if (!analysisId) {
    errorMsg.value = 'No analysis ID provided.'
    isLoading.value = false
    return
  }

  try {
    const data = await $fetch<AnalysisResult>(`${apiBase}/analyses/${analysisId}`)
    analysisResult.value = data
  } catch (e: any) {
    console.error('Failed to fetch analysis:', e)
    errorMsg.value = e?.response?.status === 404
      ? 'Analysis not found. It may have expired or the ID is invalid.'
      : 'Could not load analysis. Please try again.'
  } finally {
    isLoading.value = false
  }
})
</script>
