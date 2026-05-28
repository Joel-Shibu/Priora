<template>
  <div class="max-w-3xl mx-auto px-4 sm:px-6 py-8">
    <NuxtLink :to="`/subjects/${subjectId}`" class="inline-flex items-center gap-1.5 text-sm text-[var(--color-text-secondary)] hover:text-[var(--color-text-primary)] transition-colors mb-6">
      <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
      Back to Subject
    </NuxtLink>

    <div v-if="isLoading" class="flex justify-center py-16">
      <Loader variant="spinner" text="Loading topic details..." />
    </div>

    <Card v-else-if="topicDetail">
      <template #header>
        <h1 class="text-xl font-bold text-[var(--color-text-primary)]">{{ topicDetail.topic_name }}</h1>
        <p v-if="topicDetail.module_name" class="text-xs text-[var(--color-text-secondary)] mt-0.5">Module: {{ topicDetail.module_name }}</p>
        <p v-if="topicDetail.normalized_name && topicDetail.normalized_name !== topicDetail.topic_name" class="text-xs text-[var(--color-text-secondary)] mt-0.5">
          Also known as: {{ topicDetail.normalized_name }}
        </p>
      </template>

      <!-- Has data -->
      <template v-if="hasData">
        <div class="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-6">
          <div class="p-3 rounded-xl bg-[var(--color-surface-alt)] border border-[var(--color-border)]">
            <p class="text-[10px] font-medium text-[var(--color-text-secondary)] uppercase tracking-wider">Frequency</p>
            <p class="text-xl font-bold text-[var(--color-text-primary)]">{{ stats.frequency_count }}</p>
            <p class="text-xs text-[var(--color-text-secondary)]">appearances</p>
          </div>
          <div class="p-3 rounded-xl bg-[var(--color-surface-alt)] border border-[var(--color-border)]">
            <p class="text-[10px] font-medium text-[var(--color-text-secondary)] uppercase tracking-wider">Avg Marks</p>
            <p class="text-xl font-bold text-[var(--color-text-primary)]">{{ stats.avg_marks?.toFixed(1) || '—' }}</p>
            <p class="text-xs text-[var(--color-text-secondary)]">per question</p>
          </div>
          <div class="p-3 rounded-xl bg-[var(--color-surface-alt)] border border-[var(--color-border)]">
            <p class="text-[10px] font-medium text-[var(--color-text-secondary)] uppercase tracking-wider">Total Marks</p>
            <p class="text-xl font-bold text-[var(--color-text-primary)]">{{ stats.total_marks_count }}</p>
            <p class="text-xs text-[var(--color-text-secondary)]">across papers</p>
          </div>
          <div class="p-3 rounded-xl bg-[var(--color-surface-alt)] border border-[var(--color-border)]">
            <p class="text-[10px] font-medium text-[var(--color-text-secondary)] uppercase tracking-wider">Last Seen</p>
            <p class="text-xl font-bold text-[var(--color-text-primary)]">{{ stats.last_seen_year || '—' }}</p>
            <p class="text-xs text-[var(--color-text-secondary)]">year</p>
          </div>
        </div>

        <Card v-if="stats.priority_score !== null" variant="bordered" padding="sm">
          <div class="flex items-center justify-between">
            <span class="text-sm text-[var(--color-text-primary)]">Priority Score</span>
            <span class="text-lg font-bold text-primary-500">{{ stats.priority_score.toFixed(2) }}</span>
          </div>
        </Card>
      </template>

      <!-- No data state -->
      <div v-else class="py-8 text-center">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-12 h-12 mx-auto text-[var(--color-text-secondary)]/40 mb-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/>
        </svg>
        <p class="text-sm font-medium text-[var(--color-text-primary)] mb-1">No Question Paper Data Yet</p>
        <p class="text-xs text-[var(--color-text-secondary)] max-w-sm mx-auto">
          This topic exists in the syllabus but hasn't been mapped to any question papers yet.
          Once an admin uploads and maps question data, statistics will appear here.
        </p>
      </div>
    </Card>

    <Card v-else padding="lg">
      <p class="text-center text-[var(--color-text-secondary)]">Topic not found.</p>
    </Card>
  </div>
</template>

<script setup lang="ts">
import type { TopicStats } from '~/types/api'

const route = useRoute()
const apiBase = useRuntimeConfig().public.apiBase

const subjectId = computed(() => route.params.id as string)
const topicId = computed(() => route.params.topicId as string)

interface TopicDetail {
  topic_name: string
  normalized_name: string | null
  module_name: string
  difficulty: string | null
}

interface SubjectDetailResponse {
  modules: Array<{
    id: string
    module_name: string
    topics: Array<{
      id: string
      topic_name: string
      normalized_name: string | null
      difficulty: string | null
    }>
  }>
}

const topicDetail = ref<TopicDetail | null>(null)
const stats = ref<TopicStats>({ frequency_count: 0, avg_marks: 0, total_marks_count: 0, last_seen_year: null, priority_score: null })
const isLoading = ref(true)

const hasData = computed(() => stats.value.frequency_count > 0 || stats.value.total_marks_count > 0)

definePageMeta({
  title: computed(() => topicDetail.value ? `${topicDetail.value.topic_name} — Priora` : 'Topic — Priora'),
})

onMounted(async () => {
  try {
    const subjectData = await $fetch<SubjectDetailResponse>(`${apiBase}/subjects/${subjectId.value}`)
    for (const mod of subjectData.modules || []) {
      const found = mod.topics?.find(t => t.id === topicId.value)
      if (found) {
        topicDetail.value = {
          topic_name: found.topic_name,
          normalized_name: found.normalized_name,
          module_name: mod.module_name,
          difficulty: found.difficulty,
        }
        break
      }
    }
  } catch {
    // topic not found
  } finally {
    isLoading.value = false
  }
})
</script>
