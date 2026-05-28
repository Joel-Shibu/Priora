<template>
  <div class="max-w-5xl mx-auto px-4 sm:px-6 py-8">
    <!-- Header -->
    <div class="flex items-center justify-between mb-8">
      <div>
        <h1 class="text-2xl font-bold text-[var(--color-text-primary)]">Admin Dashboard</h1>
        <p class="text-sm text-[var(--color-text-secondary)] mt-1">Manage subjects, question papers, and topic mappings</p>
      </div>
    </div>

    <Loader v-if="isLoading" variant="spinner" text="Loading dashboard..." />

    <template v-else>
      <!-- Stats Grid -->
      <div class="grid sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
        <Card variant="elevated" padding="md">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-xl bg-primary-100 dark:bg-primary-900/30 flex items-center justify-center text-primary-600 dark:text-primary-400">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/></svg>
            </div>
            <div>
              <p class="text-2xl font-bold text-[var(--color-text-primary)]">{{ stats.subjects }}</p>
              <p class="text-xs text-[var(--color-text-secondary)]">Subjects</p>
            </div>
          </div>
        </Card>
        <Card variant="elevated" padding="md">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-xl bg-green-100 dark:bg-green-900/30 flex items-center justify-center text-green-600 dark:text-green-400">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/></svg>
            </div>
            <div>
              <p class="text-2xl font-bold text-[var(--color-text-primary)]">{{ stats.papers ?? '—' }}</p>
              <p class="text-xs text-[var(--color-text-secondary)]">Question Papers</p>
            </div>
          </div>
        </Card>
        <Card variant="elevated" padding="md">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-xl bg-amber-100 dark:bg-amber-900/30 flex items-center justify-center text-amber-600 dark:text-amber-400">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/></svg>
            </div>
            <div>
              <p class="text-2xl font-bold text-[var(--color-text-primary)]">{{ stats.topics }}</p>
              <p class="text-xs text-[var(--color-text-secondary)]">Topics</p>
            </div>
          </div>
        </Card>
        <Card variant="elevated" padding="md">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-xl bg-purple-100 dark:bg-purple-900/30 flex items-center justify-center text-purple-600 dark:text-purple-400">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>
            </div>
            <div>
              <p class="text-2xl font-bold text-[var(--color-text-primary)]">{{ stats.mappings ?? '—' }}</p>
              <p class="text-xs text-[var(--color-text-secondary)]">Mappings</p>
            </div>
          </div>
        </Card>
      </div>

      <!-- Quick Actions -->
      <h2 class="text-lg font-semibold text-[var(--color-text-primary)] mb-4">Quick Actions</h2>
      <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-4 mb-12">
        <NuxtLink to="/admin/uploads"
          class="flex items-center gap-3 p-4 rounded-xl bg-[var(--color-surface)] border border-[var(--color-border)] hover:border-primary-300 dark:hover:border-primary-700 transition-all hover:shadow-sm group"
        >
          <div class="w-10 h-10 rounded-xl bg-primary-100 dark:bg-primary-900/30 flex items-center justify-center text-primary-600 dark:text-primary-400 group-hover:scale-110 transition-transform">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg>
          </div>
          <div>
            <p class="font-medium text-sm text-[var(--color-text-primary)]">Upload Question Paper</p>
            <p class="text-xs text-[var(--color-text-secondary)]">Add PYQs for analysis</p>
          </div>
        </NuxtLink>

        <NuxtLink to="/admin/subjects"
          class="flex items-center gap-3 p-4 rounded-xl bg-[var(--color-surface)] border border-[var(--color-border)] hover:border-primary-300 dark:hover:border-primary-700 transition-all hover:shadow-sm group"
        >
          <div class="w-10 h-10 rounded-xl bg-green-100 dark:bg-green-900/30 flex items-center justify-center text-green-600 dark:text-green-400 group-hover:scale-110 transition-transform">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 5v14"/><path d="M5 12h14"/></svg>
          </div>
          <div>
            <p class="font-medium text-sm text-[var(--color-text-primary)]">Manage Subjects</p>
            <p class="text-xs text-[var(--color-text-secondary)]">Add/edit subjects, modules, topics</p>
          </div>
        </NuxtLink>

        <NuxtLink to="/admin/mappings"
          class="flex items-center gap-3 p-4 rounded-xl bg-[var(--color-surface)] border border-[var(--color-border)] hover:border-primary-300 dark:hover:border-primary-700 transition-all hover:shadow-sm group"
        >
          <div class="w-10 h-10 rounded-xl bg-amber-100 dark:bg-amber-900/30 flex items-center justify-center text-amber-600 dark:text-amber-400 group-hover:scale-110 transition-transform">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 8h1a4 4 0 0 1 0 8h-1"/><path d="M2 8h16v9a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V8z"/><line x1="6" y1="1" x2="6" y2="4"/><line x1="10" y1="1" x2="10" y2="4"/><line x1="14" y1="1" x2="14" y2="4"/></svg>
          </div>
          <div>
            <p class="font-medium text-sm text-[var(--color-text-primary)]">Topic Mappings</p>
            <p class="text-xs text-[var(--color-text-secondary)]">Map questions to topics</p>
          </div>
        </NuxtLink>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import type { AdminStats } from '~/types/api'
import Card from '~/components/ui/Card.vue'
import Loader from '~/components/ui/Loader.vue'

definePageMeta({
  layout: 'default',
  title: 'Admin Dashboard — Priora',
  description: 'Manage KTU subjects, question papers, and topic mappings for Priora analysis.',
})

const isLoading = ref(true)
const stats = ref<AdminStats>({ subjects: 0, papers: 0, topics: 0, mappings: 0 })

onMounted(async () => {
  try {
    // Use the centralized /stats endpoint (accurate cascade through the full hierarchy)
    const data = await $fetch<{ subjects: number; topics: number; papers: number; schemes: number }>('/stats')
    stats.value = {
      subjects: data.subjects ?? 0,
      topics: data.topics ?? 0,
      papers: data.papers ?? 0,
      mappings: 0, // mappings count requires a separate endpoint
    }
  } catch {
    console.warn('Failed to load admin stats — backend may be unreachable')
  } finally {
    isLoading.value = false
  }
})
</script>
