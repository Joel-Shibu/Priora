<template>
  <button
    :disabled="disabled || loading"
    :class="[
      'inline-flex items-center justify-center gap-2 font-medium transition-all duration-150 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--color-primary-500)]',
      sizeClasses[size],
      variantClasses[variant],
      { 'opacity-50 cursor-not-allowed pointer-events-none': disabled || loading },
      { 'active:scale-[0.97]': !disabled && !loading },
      className,
    ]"
    v-bind="$attrs"
  >
    <svg
      v-if="loading"
      class="animate-spin -ml-1"
      :class="loadingSizeClass"
      viewBox="0 0 24 24"
      fill="none"
    >
      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
      <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
    </svg>
    <slot />
  </button>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  variant?: 'primary' | 'secondary' | 'ghost' | 'danger'
  size?: 'sm' | 'md' | 'lg'
  disabled?: boolean
  loading?: boolean
  className?: string
}>(), {
  variant: 'primary',
  size: 'md',
  disabled: false,
  loading: false,
  className: '',
})

const sizeClasses: Record<string, string> = {
  sm: 'px-3 py-1.5 text-xs rounded-lg',
  md: 'px-4 py-2.5 text-sm rounded-xl',
  lg: 'px-6 py-3 text-base rounded-xl',
}

const variantClasses: Record<string, string> = {
  primary:
    'bg-[var(--color-primary-500)] text-white hover:bg-[var(--color-primary-600)] shadow-lg shadow-[var(--color-primary-500)]/20',
  secondary:
    'bg-[var(--color-surface-alt)] text-[var(--color-text-primary)] hover:bg-[var(--color-border)] border border-[var(--color-border)]',
  ghost:
    'text-[var(--color-text-secondary)] hover:text-[var(--color-text-primary)] hover:bg-[var(--color-surface-alt)]',
  danger:
    'bg-red-500 text-white hover:bg-red-600 shadow-lg shadow-red-500/20',
}

const loadingSizeClass = 'w-4 h-4'
</script>
