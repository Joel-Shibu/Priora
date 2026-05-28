<template>
  <div class="min-h-screen bg-[var(--color-bg)] flex flex-col items-center justify-center px-4">
    <div class="text-center max-w-md">
      <!-- Error Icon -->
      <div class="w-16 h-16 mx-auto mb-6 rounded-2xl bg-[var(--color-surface-alt)] border border-[var(--color-border)] flex items-center justify-center">
        <svg v-if="error.statusCode === 404" xmlns="http://www.w3.org/2000/svg" class="w-7 h-7 text-[var(--color-text-tertiary)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="10"/><path d="M16 16s-1.5-2-4-2-4 2-4 2"/><line x1="9" y1="9" x2="9.01" y2="9"/><line x1="15" y1="9" x2="15.01" y2="9"/>
        </svg>
        <svg v-else xmlns="http://www.w3.org/2000/svg" class="w-7 h-7 text-[var(--color-text-tertiary)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/>
        </svg>
      </div>

      <!-- Error code -->
      <p class="text-sm font-mono font-bold text-[var(--color-primary-500)] mb-2">
        {{ error.statusCode === 404 ? '404' : error.statusCode || 'Error' }}
      </p>

      <!-- Title -->
      <h1 class="text-2xl font-bold text-[var(--color-text-primary)] mb-2">
        {{ error.statusCode === 404 ? 'Page not found' : 'Something went wrong' }}
      </h1>

      <!-- Message -->
      <p class="text-sm text-[var(--color-text-secondary)] mb-8">
        {{ error.statusCode === 404
          ? 'This page doesn\'t exist or has been moved. Let\'s get you back on track.'
          : 'An unexpected error occurred. Our team has been notified.' }}
      </p>

      <!-- Actions -->
      <div class="flex items-center justify-center gap-3">
        <NuxtLink to="/"
          class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-gradient-to-r from-[var(--color-primary-500)] to-[var(--color-primary-600)] text-white font-medium text-sm shadow-md hover:shadow-lg hover:-translate-y-[1px] active:translate-y-[1px] transition-all"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/>
          </svg>
          Go Home
        </NuxtLink>
        <button @click="handleClear"
          class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl border border-[var(--color-border)] bg-[var(--color-surface)] text-sm font-medium text-[var(--color-text-primary)] hover:bg-[var(--color-surface-hover)] transition-all"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="23 4 23 10 17 10"/><path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"/>
          </svg>
          Try Again
        </button>
      </div>
    </div>

    <!-- Footer -->
    <p class="mt-12 text-xs text-[var(--color-text-tertiary)]">
      &copy; {{ new Date().getFullYear() }} Priora &middot; KTU Exam Companion
    </p>
  </div>
</template>

<script setup lang="ts">
import type { NuxtError } from '#app'

const props = defineProps({
  error: {
    type: Object as () => NuxtError,
    required: true,
  },
})

const handleClear = () => clearError({ redirect: '/' })

useHead({
  title: props.error.statusCode === 404
    ? 'Page Not Found — Priora'
    : 'Error — Priora',
})
</script>
