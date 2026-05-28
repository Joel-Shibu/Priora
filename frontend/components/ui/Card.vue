<template>
  <div
    :class="[
      'rounded-2xl border transition-all duration-150',
      variantClasses[variant],
      paddingClasses[padding],
      hoverable ? 'hover:shadow-md hover:-translate-y-0.5 cursor-pointer' : '',
      className,
    ]"
    v-bind="$attrs"
  >
    <div v-if="$slots.header || title" class="mb-4">
      <slot name="header">
        <h3 v-if="title" class="font-semibold text-[var(--color-text-primary)]">
          {{ title }}
        </h3>
        <p v-if="subtitle" class="text-sm text-[var(--color-text-secondary)] mt-0.5">
          {{ subtitle }}
        </p>
      </slot>
    </div>
    <slot />
    <div v-if="$slots.footer" class="mt-4 pt-4 border-t border-[var(--color-border)]">
      <slot name="footer" />
    </div>
  </div>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  variant?: 'default' | 'elevated' | 'bordered'
  padding?: 'none' | 'sm' | 'md' | 'lg'
  hoverable?: boolean
  title?: string
  subtitle?: string
  className?: string
}>(), {
  variant: 'default',
  padding: 'md',
  hoverable: false,
  className: '',
})

const variantClasses: Record<string, string> = {
  default: 'bg-[var(--color-surface)] border-[var(--color-border)]',
  elevated: 'bg-[var(--color-surface)] border-[var(--color-border)] shadow-md',
  bordered: 'bg-transparent border-[var(--color-border)] border-2',
}

const paddingClasses: Record<string, string> = {
  none: 'p-0',
  sm: 'p-4',
  md: 'p-5 sm:p-7',
  lg: 'p-6 sm:p-8',
}
</script>
