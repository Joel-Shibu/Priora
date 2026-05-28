<template>
  <div
    class="fun-loader"
    :class="`fun-loader--${variant}`"
  >
    <!-- ═══ ANIMATED BOOK ICON ═══ -->
    <div class="fun-loader-icon">
      <!-- Book body -->
      <svg
        class="fun-loader-book"
        viewBox="0 0 48 48"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <!-- Back cover -->
        <rect x="6" y="6" width="36" height="34" rx="3" fill="currentColor" class="fun-loader-cover-back" />

        <!-- Pages (animated stack) -->
        <rect x="8" y="8" width="32" height="30" rx="2" fill="var(--color-surface)" class="fun-loader-pages" />

        <!-- Spine line -->
        <line x1="24" y1="8" x2="24" y2="38" stroke="currentColor" stroke-width="1" opacity="0.15" />

        <!-- Front cover (slides open) -->
        <path
          d="M8 8h32v30H8z"
          fill="currentColor"
          class="fun-loader-cover-front"
          rx="2"
        />

        <!-- Page lines (animated flip) -->
        <line x1="12" y1="16" x2="20" y2="16" stroke="var(--color-text-tertiary)" stroke-width="1.5" stroke-linecap="round" class="fun-loader-line" />
        <line x1="12" y1="21" x2="22" y2="21" stroke="var(--color-text-tertiary)" stroke-width="1.5" stroke-linecap="round" class="fun-loader-line" style="animation-delay: 0.15s" />
        <line x1="12" y1="26" x2="18" y2="26" stroke="var(--color-text-tertiary)" stroke-width="1.5" stroke-linecap="round" class="fun-loader-line" style="animation-delay: 0.3s" />

        <!-- Bookmark ribbon -->
        <path
          d="M24 8v6l-2-1.5-2 1.5V8"
          stroke="var(--color-primary-500)"
          stroke-width="1.5"
          stroke-linecap="round"
          stroke-linejoin="round"
          fill="var(--color-primary-500)"
          fill-opacity="0.3"
          class="fun-loader-ribbon"
        />

        <!-- Sparkle dots (orbit) -->
        <circle cx="8" cy="8" r="2" fill="var(--color-primary-400)" class="fun-loader-sparkle" />
        <circle cx="40" cy="8" r="1.5" fill="var(--color-primary-500)" class="fun-loader-sparkle" style="animation-delay: 0.4s" />
        <circle cx="8" cy="40" r="1.5" fill="var(--color-primary-400)" class="fun-loader-sparkle" style="animation-delay: 0.8s" />
        <circle cx="40" cy="40" r="2" fill="var(--color-primary-500)" class="fun-loader-sparkle" style="animation-delay: 1.2s" />
      </svg>

      <!-- Glow behind the book -->
      <div class="fun-loader-glow" />
    </div>

    <!-- ═══ CYCLING MESSAGES ═══ -->
    <div class="fun-loader-text">
      <p class="fun-loader-primary">{{ currentMessage }}</p>
      <p v-if="variant === 'page'" class="fun-loader-secondary">Give me just a moment 🤓</p>
    </div>

    <!-- ═══ PROGRESS DOTS (page variant only) ═══ -->
    <div v-if="variant === 'page'" class="fun-loader-dots">
      <span
        v-for="n in 5"
        :key="n"
        class="fun-loader-dot"
        :style="{ animationDelay: `${n * 0.2}s` }"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
const props = withDefaults(defineProps<{
  /** 'inline' for buttons, 'page' for full loading screen */
  variant?: 'inline' | 'page'
  /** Custom messages to cycle through */
  messages?: string[]
}>(), {
  variant: 'page',
  messages: () => [
    'Analyzing question paper patterns...',
    'Ranking topics by priority...',
    'Crunching the numbers...',
    'Polishing your study plan...',
    'Almost there!',
  ],
})

const currentIndex = ref(0)
let interval: ReturnType<typeof setInterval> | null = null

onMounted(() => {
  // Respect reduced motion — only show first message, no cycling
  const prefersReducedMotion = import.meta.client
    ? window.matchMedia('(prefers-reduced-motion: reduce)').matches
    : false

  if (!prefersReducedMotion) {
    interval = setInterval(() => {
      currentIndex.value = (currentIndex.value + 1) % props.messages.length
    }, 2500)
  }
})

onUnmounted(() => {
  if (interval) clearInterval(interval)
})
const currentMessage = computed(() => props.messages[currentIndex.value])
</script>
