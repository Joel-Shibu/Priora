<template>
  <div class="surface-raised overflow-hidden transition-all animate-fade-up">
    <!-- Header -->
    <div class="px-5 sm:px-7 py-4 flex items-center gap-3 border-b border-[var(--color-border)]"
      :style="{ background: headerBg }"
    >
      <!-- Priority Icon -->
      <div class="w-9 h-9 rounded-xl flex items-center justify-center shrink-0"
        :style="{ background: iconBg, color: iconColor }"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4.5 h-4.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path v-if="variant === 'high'" d="M12 20V10M18 20V4M6 20v-4"/>
          <path v-if="variant === 'medium'" d="M12 20V10M18 20v-4M6 20v-4"/>
          <path v-if="variant === 'low'" d="M12 20V10M18 20v-4M6 20v-4"/>
        </svg>
      </div>
      <div class="min-w-0">
        <h3 class="font-semibold text-sm" :style="{ color: titleColor }">{{ title }}</h3>
        <p class="text-xs text-[var(--color-text-tertiary)] truncate">{{ subtitle }}</p>
      </div>
      <span class="ml-auto shrink-0 text-xs font-mono px-2.5 py-0.5 rounded-full"
        :style="{ background: badgeBg, color: badgeColor }"
      >
        {{ topics.length }} topic{{ topics.length !== 1 ? 's' : '' }}
      </span>
    </div>

    <!-- Topics -->
    <div class="divide-y divide-[var(--color-border)]">
      <PriorityItem
        v-for="(topic, idx) in topics"
        :key="topic.topic_id"
        :topic="topic"
        :rank="idx + 1"
        :variant="variant"
      />
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

const props = defineProps<{
  title: string
  subtitle: string
  topics: TopicRanking[]
  variant: 'high' | 'medium' | 'low'
}>()

const accentMap = {
  high: { bg: 'var(--priority-high)', text: 'var(--priority-high)' },
  medium: { bg: 'var(--priority-medium)', text: 'var(--priority-medium)' },
  low: { bg: 'var(--priority-low)', text: 'var(--priority-low)' },
}

const headerBg = computed(() => {
  switch (props.variant) {
    case 'high': return 'oklch(0.58 0.16 25 / 0.04)'
    case 'medium': return 'oklch(0.68 0.14 75 / 0.04)'
    case 'low': return 'oklch(0.55 0.02 260 / 0.03)'
  }
})

const iconBg = computed(() => {
  switch (props.variant) {
    case 'high': return 'oklch(0.58 0.16 25 / 0.12)'
    case 'medium': return 'oklch(0.68 0.14 75 / 0.12)'
    case 'low': return 'oklch(0.55 0.02 260 / 0.08)'
  }
})

const iconColor = computed(() => accentMap[props.variant].text)

const titleColor = computed(() => accentMap[props.variant].text)

const badgeBg = computed(() => {
  switch (props.variant) {
    case 'high': return 'oklch(0.58 0.16 25 / 0.1)'
    case 'medium': return 'oklch(0.68 0.14 75 / 0.1)'
    case 'low': return 'oklch(0.55 0.02 260 / 0.06)'
  }
})

const badgeColor = computed(() => accentMap[props.variant].text)
</script>
