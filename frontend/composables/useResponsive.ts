import { breakpointsTailwind, useBreakpoints } from '@vueuse/core'

export function useResponsive() {
  const breakpoints = useBreakpoints(breakpointsTailwind)

  const isMobile = breakpoints.smaller('sm')
  const isTablet = breakpoints.between('sm', 'lg')
  const isDesktop = breakpoints.greater('lg')

  const isTouch = ref(false)

  onMounted(() => {
    isTouch.value = 'ontouchstart' in window || navigator.maxTouchPoints > 0
  })

  return {
    isMobile,
    isTablet,
    isDesktop,
    isTouch,
    breakpoints,
  }
}
