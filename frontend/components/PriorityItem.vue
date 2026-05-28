<template>
  <div class="relative">
    <button
      @click="expanded = !expanded"
      class="w-full flex items-start gap-3 px-5 sm:px-7 py-4 text-left hover:bg-[var(--color-surface-hover)] transition-colors group"
    >
      <!-- Rank badge -->
      <span class="shrink-0 w-7 h-7 rounded-lg flex items-center justify-center text-xs font-mono font-bold mt-0.5"
        :style="{ background: rankBadgeBg, color: rankBadgeColor }"
      >
        {{ rank }}
      </span>

      <!-- Content -->
      <div class="flex-1 min-w-0">
        <div class="flex items-center gap-2 mb-1">
          <p class="text-sm font-semibold text-[var(--color-text-primary)] truncate">
            {{ topic.topic_name }}
          </p>
          <!-- Score bar -->
          <div class="score-bar-track flex-1 max-w-[80px] hidden sm:block">
            <div
              class="score-bar-fill"
              :style="{ width: `${Math.min(scorePercent, 100)}%`, background: scoreColor }"
            />
          </div>
        </div>
        <p class="text-xs text-[var(--color-text-tertiary)]">
          {{ topic.module_name }}
        </p>
        <!-- Reason chips -->
        <div class="flex flex-wrap gap-1.5 mt-2">
          <span v-for="reason in topic.reasons.slice(0, 3)" :key="reason"
            class="inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[10px] font-medium"
            :style="{ background: reasonChipBg, color: reasonChipColor }"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="20 6 9 17 4 12"/>
            </svg>
            {{ reason }}
          </span>
          <span v-if="topic.reasons.length > 3"
            class="text-[10px] text-[var(--color-text-tertiary)] ml-0.5"
          >+{{ topic.reasons.length - 3 }} more</span>
        </div>
      </div>

      <!-- Score + expand -->
      <div class="shrink-0 flex flex-col items-end gap-1">
        <span class="text-sm font-mono font-bold" :style="{ color: scoreTextColor }">
          {{ topic.priority_score.toFixed(1) }}
        </span>
        <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5 text-[var(--color-text-tertiary)] transition-transform group-hover:scale-110"
          :class="{ 'rotate-90': expanded }"
          viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
        >
          <polyline points="9 18 15 12 9 6"/>
        </svg>
      </div>
    </button>

    <!-- Expanded details -->
    <Transition name="expand">
      <div v-if="expanded" class="px-5 sm:px-7 pb-5 pt-1 border-t border-[var(--color-border)] bg-[var(--color-surface-alt)]/50">
        <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 mt-3">
          <div class="bg-[var(--color-surface)] rounded-xl p-3.5 border border-[var(--color-border)]">
            <p class="text-[10px] font-medium text-[var(--color-text-tertiary)] uppercase tracking-wider">Frequency</p>
            <p class="text-xl font-bold text-[var(--color-text-primary)] mt-0.5">{{ topic.frequency_count }}</p>
            <p class="text-xs text-[var(--color-text-tertiary)]">appearances</p>
          </div>
          <div class="bg-[var(--color-surface)] rounded-xl p-3.5 border border-[var(--color-border)]">
            <p class="text-[10px] font-medium text-[var(--color-text-tertiary)] uppercase tracking-wider">Avg Marks</p>
            <p class="text-xl font-bold text-[var(--color-text-primary)] mt-0.5">{{ topic.avg_marks.toFixed(1) }}</p>
            <p class="text-xs text-[var(--color-text-tertiary)]">per question</p>
          </div>
          <div class="bg-[var(--color-surface)] rounded-xl p-3.5 border border-[var(--color-border)]">
            <p class="text-[10px] font-medium text-[var(--color-text-tertiary)] uppercase tracking-wider">Total Marks</p>
            <p class="text-xl font-bold text-[var(--color-text-primary)] mt-0.5">{{ topic.total_marks }}</p>
            <p class="text-xs text-[var(--color-text-tertiary)]">across papers</p>
          </div>
          <div class="bg-[var(--color-surface)] rounded-xl p-3.5 border border-[var(--color-border)]">
            <p class="text-[10px] font-medium text-[var(--color-text-tertiary)] uppercase tracking-wider">Last Seen</p>
            <p class="text-xl font-bold text-[var(--color-text-primary)] mt-0.5">{{ topic.last_seen_year || '—' }}</p>
            <p class="text-xs text-[var(--color-text-tertiary)]">year</p>
          </div>
        </div>
      </div>
    </Transition>
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

const props = defineProps<{
  topic: TopicRanking
  rank: number
  variant: 'high' | 'medium' | 'low'
}>()

const expanded = ref(false)

const maxScore = 100 // Normalize score to percentage

const scorePercent = computed(() => {
  return Math.min((props.topic.priority_score / maxScore) * 100, 100)
})

const variantColor = computed(() => {
  switch (props.variant) {
    case 'high': return 'var(--priority-high)'
    case 'medium': return 'var(--priority-medium)'
    case 'low': return 'var(--priority-low)'
  }
})

const rankBadgeBg = computed(() => {
  switch (props.variant) {
    case 'high': return 'oklch(0.58 0.16 25 / 0.1)'
    case 'medium': return 'oklch(0.68 0.14 75 / 0.1)'
    case 'low': return 'oklch(0.55 0.02 260 / 0.06)'
  }
})

const rankBadgeColor = computed(() => variantColor.value)

const scoreColor = computed(() => variantColor.value)
const scoreTextColor = computed(() => variantColor.value)

const reasonChipBg = computed(() => {
  switch (props.variant) {
    case 'high': return 'oklch(0.58 0.16 25 / 0.08)'
    case 'medium': return 'oklch(0.68 0.14 75 / 0.08)'
    case 'low': return 'oklch(0.55 0.02 260 / 0.04)'
  }
})

const reasonChipColor = computed(() => variantColor.value)
</script>

<style scoped>
.expand-enter-active { transition: all 0.2s var(--ease-out-expo); }
.expand-leave-active { transition: all 0.15s ease-in; }
.expand-enter-from,
.expand-leave-to { opacity: 0; max-height: 0; padding-top: 0; padding-bottom: 0; }
</style>
