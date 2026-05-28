<template>
  <div class="space-y-1.5">
    <label
      v-if="label"
      :for="selectId"
      class="block text-sm font-medium text-[var(--color-text-primary)]"
    >
      {{ label }}
      <span v-if="required" class="text-red-500 ml-0.5">*</span>
    </label>
    <div class="relative">
      <select
        :id="selectId"
        :value="modelValue"
        @change="onChange"
        :disabled="disabled"
        :class="[
          'w-full rounded-xl border bg-[var(--color-surface)] px-3.5 py-2.5 text-sm text-[var(--color-text-primary)] appearance-none',
          'transition-all duration-150',
          'focus:ring-2 focus:ring-primary-500/40 focus:border-primary-500 focus:outline-none',
          'disabled:opacity-50 disabled:cursor-not-allowed disabled:bg-[var(--color-surface-alt)]',
          error
            ? 'border-red-400 focus:ring-red-500/40 focus:border-red-500'
            : 'border-[var(--color-border)]',
        ]"
      >
        <option v-if="placeholder" value="" disabled>{{ placeholder }}</option>
        <slot />
      </select>
      <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none text-[var(--color-text-secondary)]">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="6 9 12 15 18 9" />
        </svg>
      </div>
    </div>
    <p v-if="error" class="text-xs text-red-500">{{ error }}</p>
  </div>
</template>

<script setup lang="ts">
const props = withDefaults(defineProps<{
  modelValue: string
  label?: string
  placeholder?: string
  error?: string
  disabled?: boolean
  required?: boolean
}>(), {
  placeholder: '',
  disabled: false,
  required: false,
})

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const selectId = computed(() => `select-${Math.random().toString(36).slice(2, 9)}`)

function onChange(e: Event) {
  const target = e.target as HTMLSelectElement
  emit('update:modelValue', target.value)
}
</script>
