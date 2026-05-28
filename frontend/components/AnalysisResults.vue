<template>
  <div class="space-y-5 animate-fade-up">
    <!-- ═══ HEADER CARD ═══ -->
    <div class="surface-raised p-5 sm:p-7">
      <div class="flex flex-col sm:flex-row sm:items-start sm:justify-between gap-4">
        <div class="min-w-0">
          <div class="flex items-center gap-2.5 mb-1">
            <span class="text-xs font-mono font-medium px-2 py-0.5 rounded-md bg-[var(--color-primary-500)]/10 text-[var(--color-primary-500)] border border-[var(--color-primary-500)]/15">
              {{ result.subject_code }}
            </span>
            <span class="text-xs text-[var(--color-text-tertiary)]">/</span>
            <ConfidenceMeter :confidence="result.confidence" />
          </div>
          <h2 class="text-xl sm:text-2xl font-bold text-[var(--color-text-primary)] tracking-tight mt-1">
            {{ result.subject_name }}
          </h2>
          <p class="text-sm text-[var(--color-text-secondary)] mt-1">
            {{ result.days_remaining }} days until exam &middot; {{ result.total_topics }} topics analyzed
          </p>
        </div>

        <!-- Days Badge -->
        <div class="shrink-0 flex items-center gap-2 px-3.5 py-2 rounded-xl border border-[var(--color-border)] bg-[var(--color-surface-alt)]">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-[var(--color-primary-500)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/>
          </svg>
          <div class="text-right">
            <p class="text-lg font-bold text-[var(--color-text-primary)] leading-none">{{ result.days_remaining }}</p>
            <p class="text-[10px] text-[var(--color-text-tertiary)] uppercase tracking-wider">Days Left</p>
          </div>
        </div>
      </div>
    </div>

    <!-- ═══ STUDY ORDER RECOMMENDATION ═══ -->
    <div class="rounded-2xl border border-[var(--color-primary-500)]/15 bg-gradient-to-br from-[var(--color-primary-500)]/10 via-[var(--color-primary-500)]/5 to-transparent p-5 sm:p-7 animate-scale-in">
      <div class="flex items-start gap-3.5">
        <div class="w-9 h-9 rounded-xl bg-[var(--color-primary-500)] flex items-center justify-center shrink-0 mt-0.5">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="20 6 9 17 4 12"/>
          </svg>
        </div>
        <div>
          <p class="text-xs font-semibold text-[var(--color-primary-500)] uppercase tracking-wider mb-1.5">Recommended Study Order</p>
          <p class="text-base sm:text-lg font-bold text-[var(--color-text-primary)] leading-snug">
            Start with
            <span class="text-[var(--color-primary-500)] underline decoration-[var(--color-primary-500)]/30 underline-offset-4">
              {{ result.priority_buckets.high[0]?.topic_name || 'topics' }}
            </span>
            <template v-if="result.priority_buckets.high.length > 1">
              , then focus on
              {{ result.priority_buckets.high.slice(1, 3).map(t => t.topic_name).join(', ') }}
            </template>
          </p>
        </div>
      </div>
    </div>

    <!-- ═══ PRIORITY BUCKETS ═══ -->
    <div class="space-y-4">
      <PriorityList
        title="High Priority"
        subtitle="Highest exam impact — start here"
        :topics="result.priority_buckets.high"
        variant="high"
      />
      <PriorityList
        title="Medium Priority"
        subtitle="Solid exam weight — study after high priority"
        :topics="result.priority_buckets.medium"
        variant="medium"
      />
      <PriorityList
        title="Low Priority"
        subtitle="Lower exam probability — review if time permits"
        :topics="result.priority_buckets.low"
        variant="low"
      />
    </div>

    <!-- ═══ FULL RANKING (Collapsible) ═══ -->
    <details class="details-depth group">
      <summary class="flex items-center justify-between cursor-pointer p-5 sm:p-7 text-sm font-medium text-[var(--color-text-primary)] hover:bg-[var(--color-surface-hover)] transition-colors rounded-2xl [&::-webkit-details-marker]:hidden">
        <span class="flex items-center gap-2">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-[var(--color-text-tertiary)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <line x1="4" y1="6" x2="20" y2="6"/><line x1="4" y1="12" x2="20" y2="12"/><line x1="4" y1="18" x2="20" y2="18"/>
          </svg>
          <span>All {{ result.topics.length }} Topics &mdash; Full Ranking</span>
        </span>
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-[var(--color-text-tertiary)] transition-transform duration-200 group-open:rotate-180" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="6 9 12 15 18 9"/>
        </svg>
      </summary>
      <div class="px-5 sm:px-7 pb-5 sm:pb-7 border-t border-[var(--color-border)]">
        <div class="divide-y divide-[var(--color-border)]">
          <div v-for="(topic, idx) in result.topics" :key="topic.topic_id"
            class="flex items-center justify-between py-2.5 px-3 rounded-xl hover:bg-[var(--color-surface-alt)] transition-colors -mx-1"
          >
            <div class="flex items-center gap-3 min-w-0">
              <span class="text-xs font-mono text-[var(--color-text-tertiary)] w-7 shrink-0">#{{ idx + 1 }}</span>
              <div class="min-w-0">
                <p class="text-sm font-medium text-[var(--color-text-primary)] truncate">{{ topic.topic_name }}</p>
                <p class="text-xs text-[var(--color-text-tertiary)] truncate">{{ topic.module_name }}</p>
              </div>
            </div>
            <div class="flex items-center gap-3 shrink-0">
              <!-- Mini score bar -->
              <div class="w-16 sm:w-24 score-bar-track hidden sm:block">
                <div class="score-bar-fill bg-[var(--color-primary-500)]" :style="{ width: `${Math.min(topic.priority_score * 10, 100)}%` }" />
              </div>
              <span class="text-xs font-mono font-medium text-[var(--color-text-secondary)] w-12 text-right">{{ topic.priority_score.toFixed(2) }}</span>
            </div>
          </div>
        </div>
      </div>
    </details>

    <!-- ═══ FEEDBACK ═══ -->
    <div class="surface-raised p-5 sm:p-7">
      <h3 class="text-sm font-semibold text-[var(--color-text-primary)] mb-3">Was this helpful?</h3>
      <div class="flex items-center gap-1.5">
        <button
          v-for="star in 5" :key="star"
          @click="submitFeedback(star)"
          @mouseenter="hoverRating = star"
          @mouseleave="hoverRating = 0"
          class="flex items-center justify-center min-w-[44px] min-h-[44px] p-1.5 rounded-lg hover:bg-[var(--color-surface-alt)] hover-mix-bg transition-all duration-150"
          :aria-label="`Rate ${star} star${star === 1 ? '' : 's'}`"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 transition-colors duration-150"
            :class="star <= (hoverRating || feedbackRating) ? 'text-[var(--color-warning)] fill-[var(--color-warning)]' : 'text-[var(--color-border)] fill-transparent'"
            viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5"
          >
            <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/>
          </svg>
        </button>
        <span v-if="feedbackSubmitted" class="ml-2 text-xs text-[var(--color-success)] animate-fade-in">Thanks for your feedback!</span>
      </div>
    </div>

    <!-- ═══ NEW ANALYSIS CTA ═══ -->
    <div class="flex justify-center pt-2 pb-8">
      <NuxtLink to="/"
        class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl border border-[var(--color-border)] bg-[var(--color-surface)] text-sm font-medium text-[var(--color-text-primary)] hover:bg-[var(--color-surface-hover)] hover:border-[var(--color-border-hover)] transition-all"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>
        </svg>
        New Analysis
      </NuxtLink>
    </div>
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

const props = defineProps<{
  result: AnalysisResult
}>()

const feedbackRating = ref(0)
const hoverRating = ref(0)
const feedbackSubmitted = ref(false)
const apiBase = useRuntimeConfig().public.apiBase

async function submitFeedback(rating: number) {
  feedbackRating.value = rating
  try {
    await $fetch(`${apiBase}/feedback`, {
      method: 'POST',
      body: { analysis_id: props.result.analysis_id, rating, comment: null },
    })
  } catch {
    // Silently handle feedback failure
  }
  feedbackSubmitted.value = true
}
</script>
