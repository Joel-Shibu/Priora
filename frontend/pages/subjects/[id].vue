<template>
  <div class="max-w-4xl mx-auto px-4 sm:px-6 py-8">
    <NuxtLink to="/subjects" class="inline-flex items-center gap-1.5 text-sm text-[var(--color-text-secondary)] hover:text-[var(--color-text-primary)] transition-colors mb-6">
      <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
      All Subjects
    </NuxtLink>

    <!-- Loading -->
    <div v-if="isLoading" class="flex justify-center py-16">
      <Loader variant="spinner" text="Loading subject details..." />
    </div>

    <!-- Error -->
    <div v-else-if="error" class="text-center py-16">
      <p class="text-[var(--color-text-secondary)]">{{ error }}</p>
    </div>

    <!-- Content -->
    <template v-else-if="data">
      <!-- Header -->
      <div class="flex items-center justify-between mb-6">
        <div>
          <div class="flex items-center gap-3 mb-1">
            <Badge variant="primary">{{ data.subject.subject_code }}</Badge>
          </div>
          <h1 class="text-2xl sm:text-3xl font-bold text-[var(--color-text-primary)]">
            {{ data.subject.subject_name }}
          </h1>
        </div>
        <NuxtLink :to="`/?subject=${data.subject.id}&days=30`" class="shrink-0">
          <Button size="sm">Analyze This Subject</Button>
        </NuxtLink>
      </div>

      <!-- Modules & Topics -->
      <div class="space-y-5">
        <div v-for="mod in data.modules" :key="mod.id" class="rounded-2xl border border-[var(--color-border)] bg-[var(--color-surface)] overflow-hidden">
          <div class="px-5 py-4 bg-[var(--color-surface-alt)] flex items-center justify-between">
            <div>
              <h2 class="font-semibold text-sm text-[var(--color-text-primary)]">Module {{ mod.module_index }}: {{ mod.module_name }}</h2>
              <p v-if="mod.summary" class="text-xs text-[var(--color-text-secondary)] mt-0.5">{{ mod.summary }}</p>
            </div>
            <Badge>{{ mod.topics.length }} topic{{ mod.topics.length !== 1 ? 's' : '' }}</Badge>
          </div>
          <div class="divide-y divide-[var(--color-border)]">
            <NuxtLink
              v-for="topic in mod.topics"
              :key="topic.id"
              :to="`/subjects/${data.subject.id}/topics/${topic.id}`"
              class="flex items-center justify-between px-5 py-3 hover:bg-[var(--color-surface-alt)] transition-colors group"
            >
              <div class="flex items-center gap-3 min-w-0">
                <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 shrink-0 text-[var(--color-text-secondary)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/></svg>
                <span class="text-sm text-[var(--color-text-primary)] group-hover:text-primary-600 dark:group-hover:text-primary-400 transition-colors">{{ topic.topic_name }}</span>
              </div>
              <div class="flex items-center gap-2">
                <Badge v-if="topic.difficulty" :variant="difficultyVariant(topic.difficulty)" size="sm">{{ topic.difficulty }}</Badge>
                <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-[var(--color-text-secondary)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="9 18 15 12 9 6"/></svg>
              </div>
            </NuxtLink>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import type { SubjectDetail } from '~/types/api'

const route = useRoute()
const apiBase = useRuntimeConfig().public.apiBase

const data = ref<SubjectDetail | null>(null)
const isLoading = ref(true)
const error = ref<string | null>(null)

definePageMeta({
  title: computed(() => data.value ? `${data.value.subject.subject_name} — Priora` : 'Subject — Priora'),
  description: computed(() => data.value
    ? `View modules, topics, and priority analysis for ${data.value.subject.subject_name} (${data.value.subject.subject_code}). KTU 2024 Scheme.`
    : 'KTU subject details and topic analysis.'),
})

function difficultyVariant(d: string) {
  switch (d) {
    case 'easy': return 'success'
    case 'hard': return 'danger'
    default: return 'warning'
  }
}

onMounted(async () => {
  try {
    data.value = await $fetch<SubjectDetail>(`${apiBase}/subjects/${route.params.id}`)
  } catch (e: any) {
    error.value = e?.data?.error || 'Subject not found'
  } finally {
    isLoading.value = false
  }
})
</script>
