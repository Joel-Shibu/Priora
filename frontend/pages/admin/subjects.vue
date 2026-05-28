<template>
  <div class="max-w-5xl mx-auto px-4 sm:px-6 py-8">
    <NuxtLink to="/admin" class="inline-flex items-center gap-1.5 text-sm text-[var(--color-text-secondary)] hover:text-[var(--color-text-primary)] transition-colors mb-6">
      <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
      Back to Dashboard
    </NuxtLink>

    <Card>
      <template #header>
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-xl font-bold text-[var(--color-text-primary)]">Manage Subjects</h2>
            <p class="text-sm text-[var(--color-text-secondary)] mt-1">Add and edit subjects, modules, and topics</p>
          </div>
        </div>
      </template>

      <!-- Subject Selector -->
      <div class="grid sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
        <div>
          <label class="block text-xs font-medium text-[var(--color-text-secondary)] mb-1.5">Scheme</label>
          <select v-model="selectedScheme" class="w-full input-depth appearance-none" @change="onSchemeChange">
            <option v-for="s in store.schemes" :key="s.id" :value="s.id">{{ s.name }}</option>
          </select>
        </div>
        <div>
          <label class="block text-xs font-medium text-[var(--color-text-secondary)] mb-1.5">Branch</label>
          <select v-model="selectedBranch" class="w-full input-depth appearance-none" @change="onBranchChange">
            <option value="">Select</option>
            <option v-for="b in store.branches" :key="b.id" :value="b.id">{{ b.name }}</option>
          </select>
        </div>
        <div>
          <label class="block text-xs font-medium text-[var(--color-text-secondary)] mb-1.5">Semester</label>
          <select v-model="selectedSemester" class="w-full input-depth appearance-none" @change="onSemesterChange">
            <option value="">Select</option>
            <option v-for="s in store.semesters" :key="s.id" :value="s.id">Semester {{ s.semester_number }}</option>
          </select>
        </div>
        <div>
          <label class="block text-xs font-medium text-[var(--color-text-secondary)] mb-1.5">Subject</label>
          <select v-model="selectedSubject" class="w-full input-depth appearance-none">
            <option value="">Select</option>
            <option v-for="s in store.subjects" :key="s.id" :value="s.id">{{ s.subject_code }}</option>
          </select>
        </div>
      </div>

      <!-- Loading indicator for subject detail -->
      <div v-if="isLoadingDetail" class="flex items-center justify-center py-10">
        <span class="inline-block w-6 h-6 rounded-full border-2 border-[var(--color-primary-500)]/20 border-t-[var(--color-primary-500)] animate-spin" />
        <span class="ml-3 text-sm text-[var(--color-text-secondary)]">Loading subject details...</span>
      </div>

      <!-- Subject Detail -->
      <div v-else-if="selectedSubjectDetail" class="space-y-4">
        <div class="flex items-center justify-between">
          <h3 class="font-semibold text-[var(--color-text-primary)]">
            {{ selectedSubjectDetail.subject.subject_code }} — {{ selectedSubjectDetail.subject.subject_name }}
          </h3>
        </div>

        <div v-for="mod in selectedSubjectDetail.modules" :key="mod.id" class="rounded-xl border border-[var(--color-border)] overflow-hidden">
          <div class="px-4 py-3 bg-[var(--color-surface-alt)] flex items-center justify-between">
            <span class="text-sm font-medium text-[var(--color-text-primary)]">Module {{ mod.module_index }}: {{ mod.module_name }}</span>
            <Badge variant="default" size="sm">{{ mod.topics.length }} topics</Badge>
          </div>
          <div class="divide-y divide-[var(--color-border)]">
            <div v-for="topic in mod.topics" :key="topic.id" class="px-4 py-2.5 flex items-center justify-between">
              <span class="text-sm text-[var(--color-text-primary)]">{{ topic.topic_name }}</span>
              <Badge v-if="topic.difficulty" :variant="difficultyVariant(topic.difficulty)" size="sm">
                {{ topic.difficulty }}
              </Badge>
            </div>
          </div>
        </div>
      </div>
    </Card>
  </div>
</template>

<script setup lang="ts">
import type { SubjectDetail } from '~/types/api'

definePageMeta({
  layout: 'default',
  title: 'Manage Subjects — Priora Admin',
  description: 'Add and edit KTU subjects, modules, and topics for the 2024 scheme.',
})

const store = useSubjectStore()

const apiBase = useRuntimeConfig().public.apiBase

const selectedScheme = ref('')
const selectedBranch = ref('')
const selectedSemester = ref('')
const selectedSubject = ref('')
const selectedSubjectDetail = ref<SubjectDetail | null>(null)
const isLoadingDetail = ref(false)

async function onSchemeChange() {
  if (selectedScheme.value) await store.loadBranches(selectedScheme.value)
  selectedBranch.value = ''
  selectedSemester.value = ''
  selectedSubject.value = ''
  selectedSubjectDetail.value = null
}

async function onBranchChange() {
  if (selectedBranch.value) await store.loadSemesters(selectedBranch.value)
  selectedSemester.value = ''
  selectedSubject.value = ''
  selectedSubjectDetail.value = null
}

async function onSemesterChange() {
  if (selectedSemester.value) await store.loadSubjects(selectedSemester.value)
  selectedSubject.value = ''
  selectedSubjectDetail.value = null
}

watch(selectedSubject, async (id) => {
  if (typeof id === 'string' && id) {
    isLoadingDetail.value = true
    try {
      const data = await $fetch<SubjectDetail>(`${apiBase}/subjects/${id}`)
      selectedSubjectDetail.value = data
    } catch {
      selectedSubjectDetail.value = null
    } finally {
      isLoadingDetail.value = false
    }
  } else {
    selectedSubjectDetail.value = null
    isLoadingDetail.value = false
  }
})

function difficultyVariant(d: string) {
  switch (d) {
    case 'easy': return 'success'
    case 'hard': return 'danger'
    default: return 'warning'
  }
}

onMounted(async () => {
  await store.loadSchemes()
  if (store.schemes.length > 0) {
    const first = store.schemes[0]
    if (first) {
      selectedScheme.value = first.id
      await onSchemeChange()
    }
  }
})
</script>
