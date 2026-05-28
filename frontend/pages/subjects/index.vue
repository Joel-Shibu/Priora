<template>
  <div class="max-w-5xl mx-auto px-4 sm:px-6 py-8">
    <!-- Hero -->
    <section class="text-center mb-10">
      <h1 class="text-3xl sm:text-4xl font-extrabold text-[var(--color-text-primary)] tracking-tight">
        Browse Subjects
      </h1>
      <p class="mt-3 text-base text-[var(--color-text-secondary)] max-w-xl mx-auto">
        Select a scheme, branch, and semester to explore subjects and their topic priorities.
      </p>
    </section>

    <!-- Filters -->
    <div class="max-w-2xl mx-auto grid sm:grid-cols-3 gap-4 mb-10">
      <div>
        <label class="block text-xs font-medium text-[var(--color-text-secondary)] mb-1.5">Scheme</label>
        <select v-model="selectedScheme" @change="onSchemeChange" class="w-full input-depth appearance-none">
          <option value="">All Schemes</option>
          <option v-for="s in schemes" :key="s.id" :value="s.id">{{ s.name }}</option>
        </select>
      </div>
      <div>
        <label class="block text-xs font-medium text-[var(--color-text-secondary)] mb-1.5">Branch</label>
        <select v-model="selectedBranch" @change="onBranchChange" :disabled="!branches.length" class="w-full input-depth appearance-none disabled:opacity-40 disabled:cursor-not-allowed">
          <option value="">All Branches</option>
          <option v-for="b in branches" :key="b.id" :value="b.id">{{ b.name }}</option>
        </select>
      </div>
      <div>
        <label class="block text-xs font-medium text-[var(--color-text-secondary)] mb-1.5">Semester</label>
        <select v-model="selectedSemester" @change="onSemesterChange" :disabled="!semesters.length" class="w-full input-depth appearance-none disabled:opacity-40 disabled:cursor-not-allowed">
          <option value="">All Semesters</option>
          <option v-for="s in semesters" :key="s.id" :value="s.id">Semester {{ s.semester_number }}</option>
        </select>
      </div>
    </div>

    <!-- Subject Grid -->
    <div v-if="subjects.length > 0" class="grid sm:grid-cols-2 lg:grid-cols-3 gap-4">
      <NuxtLink
        v-for="subject in subjects"
        :key="subject.id"
        :to="`/subjects/${subject.id}`"
        class="p-5 rounded-2xl bg-[var(--color-surface)] border border-[var(--color-border)] hover:border-primary-300 dark:hover:border-primary-700 transition-all hover:shadow-md hover:-translate-y-0.5 group"
      >
        <div class="flex items-start justify-between mb-3">
          <Badge variant="primary" size="sm">{{ subject.subject_code }}</Badge>
        </div>
        <h3 class="font-semibold text-sm text-[var(--color-text-primary)] mb-1 group-hover:text-primary-600 dark:group-hover:text-primary-400 transition-colors">
          {{ subject.subject_name }}
        </h3>
        <p class="text-xs text-[var(--color-text-secondary)]">Click to view topic analysis</p>
      </NuxtLink>
    </div>

    <!-- Empty state -->
    <div v-else-if="!isLoading" class="text-center py-16">
      <svg xmlns="http://www.w3.org/2000/svg" class="w-12 h-12 mx-auto text-[var(--color-text-secondary)]/40 mb-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M12 2L2 7l10 5 10-5-10-5z"/><path d="M2 17l10 5 10-5"/><path d="M2 12l10 5 10-5"/></svg>
      <p class="text-sm text-[var(--color-text-secondary)]">Select a scheme, branch, and semester to browse subjects.</p>
    </div>

    <!-- Loading -->
    <div v-if="isLoading" class="flex justify-center py-12">
      <Loader variant="spinner" text="Loading subjects..." />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Scheme, Branch, Semester, Subject } from '~/types/api'

definePageMeta({
  title: 'Browse KTU Subjects — Priora',
  description: 'Browse KTU 2024 Scheme subjects by branch and semester. Get priority-ranked topic analysis for efficient exam preparation.',
})

const apiBase = useRuntimeConfig().public.apiBase

const schemes = ref<Scheme[]>([])
const branches = ref<Branch[]>([])
const semesters = ref<Semester[]>([])
const subjects = ref<Subject[]>([])

const selectedScheme = ref('')
const selectedBranch = ref('')
const selectedSemester = ref('')

const isLoading = ref(false)

async function onSchemeChange() {
  selectedBranch.value = ''
  selectedSemester.value = ''
  subjects.value = []
  if (selectedScheme.value) {
    isLoading.value = true
    try {
      branches.value = await $fetch<Branch[]>(`${apiBase}/schemes/${selectedScheme.value}/branches`)
    } catch { branches.value = [] }
    isLoading.value = false
  } else {
    branches.value = []
  }
}

async function onBranchChange() {
  selectedSemester.value = ''
  subjects.value = []
  if (selectedBranch.value) {
    isLoading.value = true
    try {
      semesters.value = await $fetch<Semester[]>(`${apiBase}/branches/${selectedBranch.value}/semesters`)
    } catch { semesters.value = [] }
    isLoading.value = false
  } else {
    semesters.value = []
  }
}

async function onSemesterChange() {
  subjects.value = []
  if (selectedSemester.value) {
    isLoading.value = true
    try {
      subjects.value = await $fetch<Subject[]>(`${apiBase}/semesters/${selectedSemester.value}/subjects`)
    } catch { subjects.value = [] }
    isLoading.value = false
  }
}

onMounted(async () => {
  try {
    schemes.value = await $fetch<Scheme[]>(`${apiBase}/schemes`)
    if (schemes.value.length > 0) {
      const first = schemes.value[0]
      if (first) {
        selectedScheme.value = first.id
        await onSchemeChange()
      }
    }
  } catch { /* ignore */ }
})
</script>
