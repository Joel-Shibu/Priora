interface AnalyzeInput {
  subjectId: string
  subjectName?: string
  subjectCode?: string
  daysRemaining: number
}

export function useAnalyze() {
  const selectionStore = useSelectionStore()
  const analysisStore = useAnalysisStore()
  const uiStore = useUiStore()
  const router = useRouter()

  async function runAnalysis(input: AnalyzeInput) {
    if (input.daysRemaining < 1 || input.daysRemaining > 365) {
      uiStore.showToast('Days remaining must be between 1 and 365', 'error')
      return null
    }

    const result = await analysisStore.fetchAnalysis(
      input.subjectId,
      input.daysRemaining,
    )

    if (result) {
      uiStore.showToast('Analysis complete!', 'success')
      return result
    }

    if (analysisStore.error) {
      uiStore.showToast(analysisStore.error, 'error')
    }

    return null
  }

  async function runAndNavigate(input: AnalyzeInput) {
    const result = await runAnalysis(input)
    if (result) {
      router.push(`/analyze/${result.analysis_id}`)
    }
    return result
  }

  return {
    isAnalyzing: computed(() => analysisStore.isLoading),
    error: computed(() => analysisStore.error),
    currentResult: computed(() => analysisStore.currentAnalysis),
    recentResults: computed(() => analysisStore.recentAnalyses),
    runAnalysis,
    runAndNavigate,
    clearResult: analysisStore.clearCurrent,
  }
}
