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

interface PriorityBuckets {
  high: TopicRanking[]
  medium: TopicRanking[]
  low: TopicRanking[]
}

interface AnalysisResult {
  analysis_id: string
  subject_name: string
  subject_code: string
  days_remaining: number
  total_topics: number
  confidence: string
  priority_buckets: PriorityBuckets
  topics: TopicRanking[]
  generated_at: string
}

export const useSelectionStore = defineStore('selection', () => {
  // Selection state
  const schemes = ref<Scheme[]>([])
  const branches = ref<Branch[]>([])
  const semesters = ref<Semester[]>([])
  const subjects = ref<Subject[]>([])

  const selectedScheme = ref<Scheme | null>(null)
  const selectedBranch = ref<Branch | null>(null)
  const selectedSemester = ref<Semester | null>(null)
  const selectedSubject = ref<Subject | null>(null)
  const daysRemaining = ref<number>(30)

  const isLoadingSchemes = ref(false)
  const isLoadingBranches = ref(false)
  const isLoadingSemesters = ref(false)
  const isLoadingSubjects = ref(false)

  const isAnalyzing = ref(false)
  const analysisResult = ref<AnalysisResult | null>(null)
  const error = ref<string | null>(null)

  const apiBase = useRuntimeConfig().public.apiBase

  // Load schemes on init
  async function loadSchemes() {
    isLoadingSchemes.value = true
    error.value = null
    try {
      const data = await $fetch<Scheme[]>(`${apiBase}/schemes`)
      schemes.value = data
      if (data.length > 0) {
        const first = data[0]
        if (first) {
          selectedScheme.value = first
          await loadBranches(first.id)
        }
      }
    } catch (e: any) {
      error.value = 'Failed to load schemes. Please try again.'
      console.error('Failed to load schemes:', e)
    } finally {
      isLoadingSchemes.value = false
    }
  }

  async function loadBranches(schemeId: string) {
    isLoadingBranches.value = true
    error.value = null
    selectedBranch.value = null
    selectedSemester.value = null
    selectedSubject.value = null
    semesters.value = []
    subjects.value = []
    try {
      const data = await $fetch<Branch[]>(`${apiBase}/schemes/${schemeId}/branches`)
      branches.value = data
    } catch (e: any) {
      error.value = 'Failed to load branches.'
      console.error('Failed to load branches:', e)
    } finally {
      isLoadingBranches.value = false
    }
  }

  async function loadSemesters(branchId: string) {
    isLoadingSemesters.value = true
    error.value = null
    selectedSemester.value = null
    selectedSubject.value = null
    subjects.value = []
    try {
      const data = await $fetch<Semester[]>(`${apiBase}/branches/${branchId}/semesters`)
      semesters.value = data
    } catch (e: any) {
      error.value = 'Failed to load semesters.'
      console.error('Failed to load semesters:', e)
    } finally {
      isLoadingSemesters.value = false
    }
  }

  async function loadSubjects(semesterId: string) {
    isLoadingSubjects.value = true
    error.value = null
    selectedSubject.value = null
    try {
      const data = await $fetch<Subject[]>(`${apiBase}/semesters/${semesterId}/subjects`)
      subjects.value = data
    } catch (e: any) {
      error.value = 'Failed to load subjects.'
      console.error('Failed to load subjects:', e)
    } finally {
      isLoadingSubjects.value = false
    }
  }

  async function onSchemeChange(schemeId: string | null) {
    const scheme = schemeId ? (schemes.value.find(s => s.id === schemeId) ?? null) : null
    selectedScheme.value = scheme
    branches.value = []
    semesters.value = []
    subjects.value = []
    selectedBranch.value = null
    selectedSemester.value = null
    selectedSubject.value = null
    if (scheme) {
      await loadBranches(scheme.id)
    }
  }

  async function onBranchChange(branchId: string | null) {
    const branch = branchId ? (branches.value.find(b => b.id === branchId) ?? null) : null
    selectedBranch.value = branch
    semesters.value = []
    subjects.value = []
    selectedSemester.value = null
    selectedSubject.value = null
    if (branch) {
      await loadSemesters(branch.id)
    }
  }

  async function onSemesterChange(semesterId: string | null) {
    const semester = semesterId ? (semesters.value.find(s => s.id === semesterId) ?? null) : null
    selectedSemester.value = semester
    subjects.value = []
    selectedSubject.value = null
    if (semester) {
      await loadSubjects(semester.id)
    }
  }

  function onSubjectChange(subjectId: string | null) {
    selectedSubject.value = subjectId ? (subjects.value.find(s => s.id === subjectId) ?? null) : null
  }

  async function analyze() {
    if (!selectedSubject.value || !daysRemaining.value) return

    isAnalyzing.value = true
    error.value = null
    analysisResult.value = null

    try {
      const data = await $fetch<AnalysisResult>(`${apiBase}/subjects/${selectedSubject.value.id}/analyze`, {
        method: 'POST',
        body: {
          subject_id: selectedSubject.value.id,
          days_remaining: daysRemaining.value,
        },
      })
      analysisResult.value = data
      return data
    } catch (e: any) {
      error.value = e?.data?.error || 'Analysis failed. Please try again.'
      console.error('Analysis failed:', e)
      return null
    } finally {
      isAnalyzing.value = false
    }
  }

  const isValid = computed(() => {
    return selectedScheme.value &&
      selectedBranch.value &&
      selectedSemester.value &&
      selectedSubject.value &&
      daysRemaining.value >= 1 &&
      daysRemaining.value <= 365
  })

  return {
    schemes, branches, semesters, subjects,
    selectedScheme, selectedBranch, selectedSemester, selectedSubject, daysRemaining,
    isLoadingSchemes, isLoadingBranches, isLoadingSemesters, isLoadingSubjects,
    isAnalyzing, analysisResult, error, isValid,
    loadSchemes, onSchemeChange, onBranchChange, onSemesterChange, onSubjectChange, analyze,
  }
})
