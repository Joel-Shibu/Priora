<template>
  <div :class="wrapperClass">
    <!-- Spinner -->
    <div v-if="variant === 'spinner'" class="flex flex-col items-center justify-center gap-3" :class="{ 'py-12': !inline }">
      <svg
        class="animate-spin text-primary-500"
        :class="spinnerSize"
        viewBox="0 0 24 24"
        fill="none"
      >
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
      </svg>
      <p v-if="text" class="text-sm text-[var(--color-text-secondary)]">{{ text }}</p>
    </div>

    <!-- Skeleton -->
    <div v-else-if="variant === 'skeleton'" class="space-y-3" :class="{ 'p-5': !inline }">
      <div v-for="i in lines" :key="i" class="h-4 rounded-lg bg-[var(--color-border)]/50 animate-pulse" :style="skeletonStyle(i)" />
    </div>

    <!-- Dots -->
    <div v-else-if="variant === 'dots'" class="flex items-center justify-center gap-1.5" :class="{ 'py-8': !inline }">
      <span v-for="i in 3" :key="i" class="w-2 h-2 rounded-full bg-primary-500 animate-bounce" :style="{ animationDelay: `${i * 0.15}s` }" />
    </div>
  </div>
</template>

<script setup lang="ts">
const props = withDefaults(defineProps<{
  variant?: 'spinner' | 'skeleton' | 'dots'
  size?: 'sm' | 'md' | 'lg'
  text?: string
  lines?: number
  inline?: boolean
  className?: string
}>(), {
  variant: 'spinner',
  size: 'md',
  lines: 4,
  inline: false,
  className: '',
})

const sizeMap: Record<string, string> = {
  sm: 'w-4 h-4',
  md: 'w-8 h-8',
  lg: 'w-12 h-12',
}

const spinnerSize = computed(() => sizeMap[props.size] || sizeMap.md)

const wrapperClass = computed(() => props.className || '')

function skeletonStyle(index: number) {
  const widths = [72, 88, 64, 78, 92, 60]
  return { width: `${widths[(index - 1) % widths.length]}%` }
}
</script>
