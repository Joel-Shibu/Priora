<template>
  <div class="max-w-3xl mx-auto px-4 sm:px-6 py-8">
    <NuxtLink to="/admin" class="inline-flex items-center gap-1.5 text-sm text-[var(--color-text-secondary)] hover:text-[var(--color-text-primary)] transition-colors mb-6">
      <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="15 18 9 12 15 6"/></svg>
      Back to Dashboard
    </NuxtLink>

    <Card>
      <template #header>
        <h2 class="text-xl font-bold text-[var(--color-text-primary)]">Upload Question Paper</h2>
        <p class="text-sm text-[var(--color-text-secondary)] mt-1">Add a previous year question paper for analysis</p>
      </template>

      <form @submit.prevent="handleUpload" class="space-y-5">
        <!-- Cascading subject selector -->
        <div class="grid sm:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-[var(--color-text-primary)] mb-1.5">Scheme</label>
            <select v-model="form.schemeId" @change="onSchemeChange" class="w-full rounded-xl border border-[var(--color-border)] bg-[var(--color-surface)] px-3.5 py-2.5 text-sm">
              <option value="" disabled>Select scheme</option>
              <option v-for="s in schemes" :key="s.id" :value="s.id">{{ s.name }}</option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-[var(--color-text-primary)] mb-1.5">Branch</label>
            <select v-model="form.branchId" @change="onBranchChange" :disabled="!branches.length" class="w-full rounded-xl border border-[var(--color-border)] bg-[var(--color-surface)] px-3.5 py-2.5 text-sm disabled:opacity-50">
              <option value="" disabled>Select branch</option>
              <option v-for="b in branches" :key="b.id" :value="b.id">{{ b.name }}</option>
            </select>
          </div>
        </div>

        <div class="grid sm:grid-cols-2 gap-4">
          <div>
            <label class="block text-sm font-medium text-[var(--color-text-primary)] mb-1.5">Semester</label>
            <select v-model="form.semesterId" @change="onSemesterChange" :disabled="!semesters.length" class="w-full rounded-xl border border-[var(--color-border)] bg-[var(--color-surface)] px-3.5 py-2.5 text-sm disabled:opacity-50">
              <option value="" disabled>Select semester</option>
              <option v-for="s in semesters" :key="s.id" :value="s.id">Semester {{ s.semester_number }}</option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-[var(--color-text-primary)] mb-1.5">
              Subject <span class="text-red-500">*</span>
            </label>
            <select v-model="form.subjectId" :disabled="!subjects.length" class="w-full rounded-xl border border-[var(--color-border)] bg-[var(--color-surface)] px-3.5 py-2.5 text-sm disabled:opacity-50">
              <option value="" disabled>Select subject</option>
              <option v-for="s in subjects" :key="s.id" :value="s.id">{{ s.subject_code }} — {{ s.subject_name }}</option>
            </select>
          </div>
        </div>

        <div class="grid sm:grid-cols-2 gap-4">
          <Input v-model.number="form.examYear" label="Exam Year" type="number" placeholder="e.g. 2025" :min="2010" :max="2026" required />
          <Input v-model="form.examTerm" label="Term" placeholder="e.g. Dec, May" helper="Optional" />
        </div>

        <Card variant="bordered" padding="md">
          <h3 class="text-sm font-semibold text-[var(--color-text-primary)] mb-4">Questions</h3>
          <div v-for="(q, idx) in form.questions" :key="idx" class="flex gap-3 mb-3 items-start">
            <span class="text-xs font-mono text-[var(--color-text-secondary)] mt-3 w-6 shrink-0">#{{ idx + 1 }}</span>
            <div class="flex-1 space-y-2">
              <textarea
                v-model="q.text"
                placeholder="Question text"
                class="w-full rounded-xl border border-[var(--color-border)] bg-[var(--color-surface)] px-3.5 py-2 text-sm text-[var(--color-text-primary)] focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                rows="2"
              />
              <Input v-model.number="q.marks" type="number" placeholder="Marks" :min="1" :max="20" />
            </div>
            <button type="button" @click="removeQuestion(idx)" class="p-1.5 rounded-lg text-red-500 hover:bg-red-50 dark:hover:bg-red-950/20 transition-colors mt-1" :disabled="form.questions.length <= 1">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/></svg>
            </button>
          </div>
          <Button variant="ghost" size="sm" @click="addQuestion" class="mt-2">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/></svg>
            Add Question
          </Button>
        </Card>

        <div class="flex justify-end gap-3 pt-2">
          <Button variant="secondary" type="button" @click="resetForm">Reset</Button>
          <Button type="submit" :loading="isUploading" :disabled="!isFormValid">{{ isUploading ? 'Uploading...' : 'Upload Paper' }}</Button>
        </div>
      </form>
    </Card>
  </div>
</template>

<script setup lang="ts">
import type { Scheme, Branch, Semester, Subject } from '~/types/api'

definePageMeta({
  layout: 'default',
  title: 'Upload Question Paper — Priora Admin',
  description: 'Upload KTU previous year question papers for subject analysis and topic mapping.',
})

const apiBase = useRuntimeConfig().public.apiBase
const uiStore = useUiStore()

const schemes = ref<Scheme[]>([])
const branches = ref<Branch[]>([])
const semesters = ref<Semester[]>([])
const subjects = ref<Subject[]>([])

const form = ref({
  schemeId: '',
  branchId: '',
  semesterId: '',
  subjectId: '',
  examYear: new Date().getFullYear(),
  examTerm: '',
  questions: [] as { text: string; marks: number }[],
})

// Initialize with one empty question
form.value.questions = [{ text: '', marks: 5 }]

const isUploading = ref(false)

const isFormValid = computed(() =>
  form.value.subjectId
  && form.value.examYear >= 2010
  && form.value.examYear <= 2026
  && form.value.questions.some(q => q.text.trim().length > 0)
)

async function loadSchemes() {
  try {
    schemes.value = await $fetch<Scheme[]>(`${apiBase}/schemes`)
    const first = schemes.value[0]
    if (first) {
      form.value.schemeId = first.id
      await onSchemeChange()
    }
  } catch {
    uiStore.showToast('Failed to load schemes', 'error')
  }
}

async function onSchemeChange() {
  form.value.branchId = ''
  form.value.semesterId = ''
  form.value.subjectId = ''
  branches.value = []
  semesters.value = []
  subjects.value = []
  if (form.value.schemeId) {
    try {
      branches.value = await $fetch<Branch[]>(`${apiBase}/schemes/${form.value.schemeId}/branches`)
    } catch { uiStore.showToast('Failed to load branches', 'error') }
  }
}

async function onBranchChange() {
  form.value.semesterId = ''
  form.value.subjectId = ''
  semesters.value = []
  subjects.value = []
  if (form.value.branchId) {
    try {
      semesters.value = await $fetch<Semester[]>(`${apiBase}/branches/${form.value.branchId}/semesters`)
    } catch { uiStore.showToast('Failed to load semesters', 'error') }
  }
}

async function onSemesterChange() {
  form.value.subjectId = ''
  subjects.value = []
  if (form.value.semesterId) {
    try {
      subjects.value = await $fetch<Subject[]>(`${apiBase}/semesters/${form.value.semesterId}/subjects`)
    } catch { uiStore.showToast('Failed to load subjects', 'error') }
  }
}

function addQuestion() {
  form.value.questions.push({ text: '', marks: 5 })
}

function removeQuestion(idx: number) {
  if (form.value.questions.length > 1) {
    form.value.questions.splice(idx, 1)
  }
}

function resetForm() {
  form.value.schemeId = schemes.value[0]?.id || ''
  form.value.branchId = ''
  form.value.semesterId = ''
  form.value.subjectId = ''
  form.value.examYear = new Date().getFullYear()
  form.value.examTerm = ''
  form.value.questions = [{ text: '', marks: 5 }]
  loadSchemes()
}

async function handleUpload() {
  if (!isFormValid.value) return
  isUploading.value = true
  try {
    await $fetch(`${apiBase}/admin/question-papers`, {
      method: 'POST',
      body: {
        subject_id: form.value.subjectId,
        exam_year: form.value.examYear,
        exam_term: form.value.examTerm || null,
        source_type: 'manual',
        questions: form.value.questions
          .filter(q => q.text.trim().length > 0)
          .map((q, i) => ({
            question_text: q.text,
            marks: q.marks,
            order_index: i + 1,
          })),
      },
    })
    uiStore.showToast('Question paper uploaded successfully!', 'success')
    resetForm()
  } catch (e: any) {
    uiStore.showToast(e?.data?.error || 'Upload failed', 'error')
  } finally {
    isUploading.value = false
  }
}

onMounted(loadSchemes)
</script>
