<template>
  <div class="max-w-5xl mx-auto px-4 sm:px-6 py-8">
    <NuxtLink to="/admin" class="inline-flex items-center gap-1.5 text-sm text-[var(--color-text-secondary)] hover:text-[var(--color-text-primary)] transition-colors mb-6">
      <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
      Back to Dashboard
    </NuxtLink>

    <Card>
      <template #header>
        <h2 class="text-xl font-bold text-[var(--color-text-primary)]">Question-Topic Mappings</h2>
        <p class="text-sm text-[var(--color-text-secondary)] mt-1">Map questions to topics for analysis accuracy</p>
      </template>

      <!-- Subject selector -->
      <div class="grid sm:grid-cols-4 gap-3 mb-6">
        <div>
          <label class="block text-xs font-medium text-[var(--color-text-secondary)] mb-1.5">Scheme</label>
          <select v-model="selectedScheme" @change="onSchemeChange" class="w-full rounded-xl border border-[var(--color-border)] bg-[var(--color-surface)] px-3 py-2 text-sm">
            <option value="">Select</option>
            <option v-for="s in schemes" :key="s.id" :value="s.id">{{ s.name }}</option>
          </select>
        </div>
        <div>
          <label class="block text-xs font-medium text-[var(--color-text-secondary)] mb-1.5">Branch</label>
          <select v-model="selectedBranch" @change="onBranchChange" :disabled="!branches.length" class="w-full rounded-xl border border-[var(--color-border)] bg-[var(--color-surface)] px-3 py-2 text-sm disabled:opacity-50">
            <option value="">Select</option>
            <option v-for="b in branches" :key="b.id" :value="b.id">{{ b.name }}</option>
          </select>
        </div>
        <div>
          <label class="block text-xs font-medium text-[var(--color-text-secondary)] mb-1.5">Semester</label>
          <select v-model="selectedSemester" @change="onSemesterChange" :disabled="!semesters.length" class="w-full rounded-xl border border-[var(--color-border)] bg-[var(--color-surface)] px-3 py-2 text-sm disabled:opacity-50">
            <option value="">Select</option>
            <option v-for="s in semesters" :key="s.id" :value="s.id">Semester {{ s.semester_number }}</option>
          </select>
        </div>
        <div>
          <label class="block text-xs font-medium text-[var(--color-text-secondary)] mb-1.5">Subject</label>
          <select v-model="selectedSubject" @change="onSubjectChange" :disabled="!subjects.length" class="w-full rounded-xl border border-[var(--color-border)] bg-[var(--color-surface)] px-3 py-2 text-sm disabled:opacity-50">
            <option value="">Select</option>
            <option v-for="s in subjects" :key="s.id" :value="s.id">{{ s.subject_code }}</option>
          </select>
        </div>
      </div>

      <!-- Topics loaded -->
      <div v-if="topics.length > 0" class="space-y-4">
        <Card variant="bordered" padding="sm">
          <div class="flex items-center justify-between">
            <span class="text-sm font-medium text-[var(--color-text-primary)]">{{ topics.length }} topics loaded</span>
            <Badge variant="success">{{ topics.length }} available</Badge>
          </div>
        </Card>

        <!-- Mapping endpoint notice -->
        <div class="p-4 rounded-xl bg-amber-50 dark:bg-amber-950/20 border border-amber-200 dark:border-amber-800">
          <div class="flex gap-3">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 shrink-0 text-amber-600 dark:text-amber-400 mt-0.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
            <div>
              <p class="text-sm font-medium text-amber-800 dark:text-amber-300">Mapping API not yet available</p>
              <p class="text-xs text-amber-700 dark:text-amber-400 mt-1">
                The backend endpoint to list individual questions per subject is not yet implemented. Once <code class="text-xs font-mono bg-amber-100 dark:bg-amber-900/30 px-1 rounded">/api/subjects/:id/questions</code> is added, you'll be able to select questions and map them to topics here.
              </p>
            </div>
          </div>
        </div>

        <!-- Topic list preview -->
        <details class="rounded-xl border border-[var(--color-border)] overflow-hidden group">
          <summary class="px-4 py-3 text-sm font-medium text-[var(--color-text-primary)] cursor-pointer hover:bg-[var(--color-surface-alt)] transition-colors flex items-center justify-between">
            <span>Preview Available Topics</span>
            <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-[var(--color-text-secondary)] transition-transform group-open:rotate-180" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="6 9 12 15 18 9"/></svg>
          </summary>
          <div class="divide-y divide-[var(--color-border)]">
            <div v-for="topic in topics" :key="topic.id" class="px-4 py-2 text-sm text-[var(--color-text-primary)]">
              {{ topic.topic_name }}
            </div>
          </div>
        </details>
      </div>

      <!-- Empty states -->
      <div v-if="!selectedSubject && !isLoadingSubjects" class="text-center py-8 text-sm text-[var(--color-text-secondary)]">
        Select a subject to view available topics
      </div>
      <div v-if="selectedSubject && topics.length === 0 && !isLoadingSubjects" class="text-center py-8 text-sm text-[var(--color-text-secondary)]">
        No topics found for this subject
      </div>

      <Loader v-if="isLoadingSubjects" variant="spinner" text="Loading topics..." inline />
    </Card>
  </div>
</template>

<script setup lang="ts">
import type { Scheme, Branch, Semester, Subject, Topic } from '~/types/api'

definePageMeta({
  layout: 'default',
  title: 'Question-Topic Mappings — Priora Admin',
  description: 'Map KTU question paper questions to syllabus topics for accurate analysis.',
})

const apiBase = useRuntimeConfig().public.apiBase

const schemes = ref<Scheme[]>([])
const branches = ref<Branch[]>([])
const semesters = ref<Semester[]>([])
const subjects = ref<Subject[]>([])

const selectedScheme = ref('')
const selectedBranch = ref('')
const selectedSemester = ref('')
const selectedSubject = ref('')

const topics = ref<Topic[]>([])
const isLoadingSubjects = ref(false)

async function onSchemeChange() {
  selectedBranch.value = ''
  selectedSemester.value = ''
  selectedSubject.value = ''
  branches.value = []
  semesters.value = []
  subjects.value = []
  topics.value = []
  if (selectedScheme.value) {
    try { branches.value = await $fetch<Branch[]>(`${apiBase}/schemes/${selectedScheme.value}/branches`) }
    catch { /* ignore */ }
  }
}

async function onBranchChange() {
  selectedSemester.value = ''
  selectedSubject.value = ''
  semesters.value = []
  subjects.value = []
  topics.value = []
  if (selectedBranch.value) {
    try { semesters.value = await $fetch<Semester[]>(`${apiBase}/branches/${selectedBranch.value}/semesters`) }
    catch { /* ignore */ }
  }
}

async function onSemesterChange() {
  selectedSubject.value = ''
  subjects.value = []
  topics.value = []
  if (selectedSemester.value) {
    try { subjects.value = await $fetch<Subject[]>(`${apiBase}/semesters/${selectedSemester.value}/subjects`) }
    catch { /* ignore */ }
  }
}

interface SubjectDetailResponse {
  subject: Subject
  modules: Array<{
    id: string
    module_index: number
    module_name: string
    summary: string | null
    topics: Topic[]
  }>
}

async function onSubjectChange() {
  topics.value = []
  if (!selectedSubject.value) return

  isLoadingSubjects.value = true
  try {
    const detail = await $fetch<SubjectDetailResponse>(`${apiBase}/subjects/${selectedSubject.value}`)
    const allTopics: Topic[] = []
    for (const mod of detail.modules || []) {
      for (const t of mod.topics || []) {
        allTopics.push(t)
      }
    }
    topics.value = allTopics
  } catch {
    // silently fail
  } finally {
    isLoadingSubjects.value = false
  }
}

onMounted(async () => {
  try { schemes.value = await $fetch<Scheme[]>(`${apiBase}/schemes`) }
  catch { /* ignore */ }
})
</script>
