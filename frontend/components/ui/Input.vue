<template>
  <div class="space-y-1.5">
    <label
      v-if="label"
      :for="inputId"
      class="block text-sm font-medium text-[var(--color-text-primary)]"
    >
      {{ label }}
      <span v-if="required" class="text-red-500 ml-0.5">*</span>
    </label>
    <div class="relative">
      <div
        v-if="$slots.prefix"
        class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none text-[var(--color-text-secondary)]"
      >
        <slot name="prefix" />
      </div>
      <input
        :id="inputId"
        :value="modelValue"
        @input="onInput"
        v-bind="{
          type,
          placeholder,
          disabled,
          min,
          max,
          step,
          maxlength,
          autocomplete,
        }"
        :class="[
          'w-full rounded-xl border bg-[var(--color-surface)] px-3.5 py-2.5 text-sm text-[var(--color-text-primary)]',
          'placeholder:text-[var(--color-text-secondary)]/60',
          'transition-all duration-150',
          'focus:ring-2 focus:ring-primary-500/40 focus:border-primary-500 focus:outline-none',
          hasPrefix ? 'pl-10' : '',
          hasSuffix ? 'pr-10' : '',
          error
            ? 'border-red-400 focus:ring-red-500/40 focus:border-red-500'
            : 'border-[var(--color-border)]',
          disabled ? 'opacity-50 cursor-not-allowed bg-[var(--color-surface-alt)]' : '',
        ]"
      />
      <div
        v-if="$slots.suffix"
        class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none text-[var(--color-text-secondary)]"
      >
        <slot name="suffix" />
      </div>
    </div>
    <p v-if="error" class="text-xs text-red-500">{{ error }}</p>
    <p v-else-if="helper" class="text-xs text-[var(--color-text-secondary)]">{{ helper }}</p>
  </div>
</template>

<script setup lang="ts">
const props = withDefaults(defineProps<{
  modelValue: string | number
  label?: string
  type?: string
  placeholder?: string
  error?: string
  helper?: string
  disabled?: boolean
  required?: boolean
  min?: number
  max?: number
  step?: number
  maxlength?: number
  autocomplete?: string
}>(), {
  type: 'text',
  placeholder: '',
  disabled: false,
  required: false,
})

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const slots = useSlots()
const hasPrefix = computed(() => !!slots.prefix)
const hasSuffix = computed(() => !!slots.suffix)

const inputId = computed(() => `input-${Math.random().toString(36).slice(2, 9)}`)

function onInput(e: Event) {
  const target = e.target as HTMLInputElement
  emit('update:modelValue', target.value)
}
</script>
