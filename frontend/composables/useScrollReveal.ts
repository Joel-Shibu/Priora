import { useIntersectionObserver } from '@vueuse/core'

/**
 * Scroll reveal composable — triggers visible animations when elements scroll into view.
 * Uses IntersectionObserver (compositor-friendly, no scroll listeners).
 * Zero extra bundle cost — built on @vueuse/core (already installed).
 */
export function useScrollReveal(
  target: MaybeRefOrGetter<HTMLElement | null | undefined>,
  options?: {
    threshold?: number
    rootMargin?: string
    once?: boolean
  },
) {
  const isVisible = ref(false)
  const hasAnimated = ref(false)
  const { threshold = 0.1, rootMargin = '0px 0px -40px 0px', once = true } = options ?? {}

  useIntersectionObserver(
    target,
    ([entry]) => {
      if (!entry) return
      if (entry.isIntersecting) {
        isVisible.value = true
        if (once) hasAnimated.value = true
      } else if (!once) {
        isVisible.value = false
      }
    },
    { threshold, rootMargin, immediate: true },
  )

  const shouldAnimate = computed(() => isVisible.value || hasAnimated.value)

  return {
    isVisible: readonly(isVisible),
    shouldAnimate: readonly(shouldAnimate),
    hasAnimated: readonly(hasAnimated),
  }
}

/**
 * Parallax scroll effect — ties element translation Y to scroll progress.
 * Uses passive scroll listener + requestAnimationFrame throttling → 60fps.
 * Pass the template ref directly as the first argument.
 */
export function useParallax(
  targetRef: MaybeRefOrGetter<HTMLElement | null | undefined>,
  speed: number = 0.3,
) {
  const y = ref(0)
  const el = toRef(targetRef)

  if (import.meta.client) {
    let ticking = false

    const onScroll = () => {
      if (ticking) return
      requestAnimationFrame(() => {
        if (el.value) {
          const rect = el.value.getBoundingClientRect()
          const vh = window.innerHeight
          const viewportCenter = vh / 2
          const elementCenter = rect.top + rect.height / 2
          const offset = (elementCenter - viewportCenter) / vh
          y.value = offset * speed * 100
        }
        ticking = false
      })
      ticking = true
    }

    onMounted(() => {
      window.addEventListener('scroll', onScroll, { passive: true })
      onScroll()
    })

    onUnmounted(() => {
      window.removeEventListener('scroll', onScroll)
    })
  }

  const style = computed(() => {
    const val = y.value
    if (val === 0) return {}
    return {
      transform: `translateY(${val.toFixed(1)}px)`,
      willChange: 'transform' as const,
    }
  })

  return { style, y }
}
