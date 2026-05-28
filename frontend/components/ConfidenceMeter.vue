<template>
  <div class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg"
    :style="{ background: bgColor, border: `1px solid ${borderColor}` }"
  >
    <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" :style="{ color: iconColor }" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
      <circle cx="12" cy="12" r="10"/>
      <line v-if="confidence === 'Low'" x1="12" y1="8" x2="12" y2="12"/>
      <line v-if="confidence === 'Low'" x1="12" y1="16" x2="12.01" y2="16"/>
      <path v-if="confidence === 'Medium'" d="M12 9v4"/><path v-if="confidence === 'Medium'" d="M12 17h.01"/>
      <path v-if="confidence === 'High'" d="M9 12l2 2 4-4"/>
    </svg>
    <span class="text-[11px] font-semibold" :style="{ color: textColor }">{{ confidence }}</span>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  confidence: string
}>()

const colors = computed(() => {
  switch (props.confidence) {
    case 'High':
      return {
        bg: 'oklch(0.65 0.15 155 / 0.08)',
        border: 'oklch(0.65 0.15 155 / 0.15)',
        text: 'oklch(0.55 0.14 155)',
        icon: 'oklch(0.55 0.14 155)',
      }
    case 'Medium':
      return {
        bg: 'oklch(0.70 0.15 75 / 0.08)',
        border: 'oklch(0.70 0.15 75 / 0.15)',
        text: 'oklch(0.60 0.14 75)',
        icon: 'oklch(0.60 0.14 75)',
      }
    case 'Low':
      return {
        bg: 'oklch(0.60 0.18 25 / 0.08)',
        border: 'oklch(0.60 0.18 25 / 0.15)',
        text: 'oklch(0.55 0.16 25)',
        icon: 'oklch(0.55 0.16 25)',
      }
    default:
      return {
        bg: 'oklch(0.55 0.02 260 / 0.05)',
        border: 'oklch(0.55 0.02 260 / 0.1)',
        text: 'var(--color-text-tertiary)',
        icon: 'var(--color-text-tertiary)',
      }
  }
})

const bgColor = computed(() => colors.value.bg)
const borderColor = computed(() => colors.value.border)
const textColor = computed(() => colors.value.text)
const iconColor = computed(() => colors.value.icon)
</script>
