export const useUiStore = defineStore('ui', () => {
  // Global loading state
  const isGlobalLoading = ref(false)
  const activeModal = ref<string | null>(null)

  // Toast notifications
  interface Toast {
    id: string
    message: string
    type: 'success' | 'error' | 'info' | 'warning'
    duration: number
  }
  const toasts = ref<Toast[]>([])

  let toastCounter = 0

  function showToast(
    message: string,
    type: Toast['type'] = 'info',
    duration = 4000,
  ) {
    const id = `toast-${++toastCounter}`
    toasts.value.push({ id, message, type, duration })
    setTimeout(() => {
      toasts.value = toasts.value.filter((t) => t.id !== id)
    }, duration)
  }

  function dismissToast(id: string) {
    toasts.value = toasts.value.filter((t) => t.id !== id)
  }

  function setGlobalLoading(v: boolean) {
    isGlobalLoading.value = v
  }

  function openModal(name: string) {
    activeModal.value = name
  }

  function closeModal() {
    activeModal.value = null
  }

  return {
    isGlobalLoading,
    activeModal,
    toasts,
    showToast,
    dismissToast,
    setGlobalLoading,
    openModal,
    closeModal,
  }
})
