import { defineStore } from 'pinia'

interface Scheme {
  id: string
  name: string
  active: boolean
}

interface Branch {
  id: string
  name: string
}

interface Semester {
  id: string
  scheme_id: string
  branch_id: string
  semester_number: number
}

interface Subject {
  id: string
  scheme_id: string
  branch_id: string
  semester_id: string
  subject_code: string
  subject_name: string
  active: boolean
}

interface Module {
  id: string
  module_index: number
  module_name: string
  summary: string | null
  topics: Topic[]
}

interface Topic {
  id: string
  topic_name: string
  normalized_name: string | null
  difficulty: string | null
}

export const useSubjectStore = defineStore('subjects', () => {
  const schemes = ref<Scheme[]>([])
  const branches = ref<Branch[]>([])
  const semesters = ref<Semester[]>([])
  const subjects = ref<Subject[]>([])
  const selectedSubjectDetail = ref<{ subject: Subject; modules: Module[] } | null>(null)

  const isLoading = ref(false)
  const error = ref<string | null>(null)

  const apiBase = useRuntimeConfig().public.apiBase

  async function loadSchemes() {
    isLoading.value = true
    try {
      schemes.value = await $fetch<Scheme[]>(`${apiBase}/schemes`)
    } catch (e: any) {
      error.value = 'Failed to load schemes'
    } finally {
      isLoading.value = false
    }
  }

  async function loadBranches(schemeId: string) {
    isLoading.value = true
    try {
      branches.value = await $fetch<Branch[]>(`${apiBase}/schemes/${schemeId}/branches`)
    } catch (e: any) {
      error.value = 'Failed to load branches'
    } finally {
      isLoading.value = false
    }
  }

  async function loadSemesters(branchId: string) {
    isLoading.value = true
    try {
      semesters.value = await $fetch<Semester[]>(`${apiBase}/branches/${branchId}/semesters`)
    } catch (e: any) {
      error.value = 'Failed to load semesters'
    } finally {
      isLoading.value = false
    }
  }

  async function loadSubjects(semesterId: string) {
    isLoading.value = true
    try {
      subjects.value = await $fetch<Subject[]>(`${apiBase}/semesters/${semesterId}/subjects`)
    } catch (e: any) {
      error.value = 'Failed to load subjects'
    } finally {
      isLoading.value = false
    }
  }

    async function loadSubjectDetail(subjectId: string) {
    isLoading.value = true
    try {
      const data = await $fetch<{ subject: Subject; modules: Module[] }>(`${apiBase}/subjects/${subjectId}`)
      selectedSubjectDetail.value = data
    } catch (e: any) {
      error.value = 'Failed to load subject detail'
    } finally {
      isLoading.value = false
    }
  }

  return {
    schemes, branches, semesters, subjects, selectedSubjectDetail,
    isLoading, error,
    loadSchemes, loadBranches, loadSemesters, loadSubjects, loadSubjectDetail,
  }
})
