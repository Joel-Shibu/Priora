<template>
  <span
    :class="[
      'inline-flex items-center gap-1.5 font-medium transition-colors',
      sizeClasses[size],
      variantClasses[variant],
      dot ? 'pl-2.5' : '',
      className,
    ]"
  >
    <span v-if="dot" class="w-1.5 h-1.5 rounded-full" :class="dotColorClasses[variant]" />
    <slot />
  </span>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  variant?: 'default' | 'primary' | 'success' | 'warning' | 'danger'
  size?: 'sm' | 'md'
  dot?: boolean
  className?: string
}>(), {
  variant: 'default',
  size: 'sm',
  dot: false,
  className: '',
})

const sizeClasses: Record<string, string> = {
  sm: 'px-2 py-0.5 text-[10px] rounded-md',
  md: 'px-2.5 py-1 text-xs rounded-lg',
}

const variantClasses: Record<string, string> = {
  default: 'bg-[var(--color-surface-alt)] text-[var(--color-text-secondary)] border border-[var(--color-border)]',
  primary: 'bg-primary-50 dark:bg-primary-950/30 text-primary-700 dark:text-primary-300 border border-primary-200 dark:border-primary-700/50',
  success: 'bg-green-50 dark:bg-green-950/20 text-green-700 dark:text-green-300 border border-green-200 dark:border-green-800',
  warning: 'bg-amber-50 dark:bg-amber-950/20 text-amber-700 dark:text-amber-300 border border-amber-200 dark:border-amber-800',
  danger: 'bg-red-50 dark:bg-red-950/20 text-red-700 dark:text-red-300 border border-red-200 dark:border-red-800',
}

const dotColorClasses: Record<string, string> = {
  default: 'bg-[var(--color-text-secondary)]',
  primary: 'bg-primary-500',
  success: 'bg-green-500',
  warning: 'bg-amber-500',
  danger: 'bg-red-500',
}
</script>
