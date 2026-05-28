export function useTheme() {
  const colorMode = useColorMode()

  const isDark = computed(() => colorMode.value === 'dark')
  const isLight = computed(() => colorMode.value === 'light')

  function toggle() {
    colorMode.preference = isDark.value ? 'light' : 'dark'
  }

  function setDark() {
    colorMode.preference = 'dark'
  }

  function setLight() {
    colorMode.preference = 'light'
  }

  // Respect system preference
  function setSystem() {
    colorMode.preference = 'system'
  }

  return {
    isDark,
    isLight,
    toggle,
    setDark,
    setLight,
    setSystem,
    preference: computed(() => colorMode.preference),
  }
}
