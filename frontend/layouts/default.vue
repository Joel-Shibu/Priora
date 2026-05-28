<template>
  <div class="min-h-screen bg-[var(--color-bg)] flex flex-col relative">
    <!-- Grain Texture Overlay -->
    <div class="grain-overlay" />

    <!-- Ambient Glow Background — will-change for GPU compositing -->
    <div class="ambient-glow top-[-200px] right-[-200px] bg-[var(--color-primary-500)]" style="will-change: transform, opacity" />
    <div class="ambient-glow bottom-[-300px] left-[-200px] bg-[oklch(0.5_0.08_220)]" style="will-change: transform, opacity" />

    <!-- Navbar -->
    <header class="sticky top-0 z-50 border-b border-[var(--color-border)] bg-[var(--color-bg)]/85 backdrop-blur-xl supports-backdrop-blur:bg-[var(--color-bg)]/85">
      <!-- Scroll Progress Bar — GSAP-driven via ScrollTrigger onUpdate
           CSS transition handles the smooth movement (compositor-friendly).
           width set via CSS variable, updated at ~60fps via requestAnimationFrame. -->
      <div class="absolute bottom-0 left-0 right-0 h-[2px] bg-[var(--color-border)]/30">
        <div ref="scrollProgressRef" class="scroll-progress-bar h-full rounded-full bg-gradient-to-r from-[var(--color-primary-400)] to-[var(--color-primary-600)]" />
      </div>

      <div class="max-w-6xl mx-auto px-4 sm:px-8 h-16 flex items-center justify-between">
        <!-- Logo -->
        <NuxtLink to="/" class="flex items-center gap-3 group">
          <div class="relative w-9 h-9">
            <div class="absolute inset-0 rounded-xl bg-gradient-to-br from-[var(--color-primary-500)] to-[var(--color-primary-700)] opacity-90 group-hover:opacity-100 transition-opacity" />
            <div class="absolute inset-0 rounded-xl flex items-center justify-center text-white font-bold text-sm">
              P
            </div>
          </div>
          <span ref="prioraRef" class="font-semibold text-lg text-[var(--color-text-primary)] tracking-tight priora-brand">Priora</span>
        </NuxtLink>

        <!-- Nav Right -->
        <nav class="flex items-center gap-3">
          <NuxtLink to="/"
            class="hidden sm:inline-flex px-3.5 py-2 text-sm font-medium text-[var(--color-text-secondary)] hover:text-[var(--color-text-primary)] transition-colors rounded-lg hover:bg-[var(--color-surface-alt)]"
          >
            Home
          </NuxtLink>

          <!-- Theme Toggle Switch (wrapped for 44px min touch target) -->
          <!-- ═══ THEME TOGGLE ═══
               Static aria-label avoids hydration mismatch (colorMode.value differs
               between SSR=dark and client=light). Current mode is visually indicated
               by the toggle-indicator position (CSS via :root.dark/:root.light) and
               the toggle-icon--light/toggle-icon--dark active state.
               Attributes are static strings — never depend on colorMode.value. -->
          <button
            @click="colorMode.preference = colorMode.value === 'dark' ? 'light' : 'dark'"
            class="toggle-switch"
            aria-label="Toggle dark/light mode"
            title="Toggle dark/light mode"
          >
            <span class="flex items-center justify-center w-full h-full gap-0">
              <!-- Sun icon (left) — active via :root.light selector (no hydration mismatch) -->
              <span class="toggle-icon toggle-icon--light">
                <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/>
                  <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
                  <line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/>
                  <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
                </svg>
              </span>

              <!-- Sliding indicator -->
              <span class="toggle-indicator" />

              <!-- Moon icon (right) — active via :root.dark selector (no hydration mismatch) -->
              <span class="toggle-icon toggle-icon--dark">
                <svg xmlns="http://www.w3.org/2000/svg" class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
                </svg>
              </span>
            </span>
          </button>
        </nav>
      </div>
    </header>

    <!-- Main Content -->
    <main class="flex-1">
      <slot />
    </main>

    <!-- Footer -->
    <footer class="border-t border-[var(--color-border)] py-8 mt-auto">
      <div class="max-w-6xl mx-auto px-4 sm:px-8 flex flex-col sm:flex-row items-center justify-between gap-3">
        <div class="flex items-center gap-3 text-sm text-[var(--color-text-tertiary)]">
          <span>&copy; {{ new Date().getFullYear() }} Priora</span>
          <span class="w-1 h-1 rounded-full bg-[var(--color-border)]" />
          <span>KTU Exam Companion</span>
        </div>
        <p class="text-xs text-[var(--color-text-tertiary)]">
          2024 Scheme &middot; CSE &middot; S1&ndash;S8
        </p>
      </div>
    </footer>

    <!-- Mobile Bottom Bar (no theme toggle — it's in the navbar) -->
    <nav class="sm:hidden fixed bottom-0 left-0 right-0 z-50 border-t border-[var(--color-border)] bg-[var(--color-bg)]/90 backdrop-blur-lg pb-safe">
      <div class="flex items-center justify-center h-14 px-4">
        <NuxtLink to="/" class="flex flex-col items-center gap-0.5 text-xs font-medium text-[var(--color-text-secondary)]">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/>
          </svg>
          Home
        </NuxtLink>
      </div>
    </nav>
  </div>
</template>

<script setup lang="ts">
const colorMode = useColorMode()

const prioraRef = ref<HTMLElement | null>(null)
const scrollProgressRef = ref<HTMLElement | null>(null)

// Desktop-only subtle mouse parallax on the Priora brand
// Disabled on touch devices by useGsapMouseParallax internally
useGsapMouseParallax(prioraRef, { strengthX: 0.05, strengthY: 0.025, clampPx: 6 })

// ═══ SCROLL PROGRESS BAR — GSAP ScrollTrigger ═══
// Tracks page scroll progress and updates CSS variable.
// 50ms CSS transition prevents jitter while keeping responsiveness.
// NOTE: import inside onMounted to avoid top-level await (which causes async layout component).
// Top-level await in <script setup> creates a Suspense boundary that can trigger
// "_context2 is not a function" hydration errors in Nuxt 4.4.6.
onMounted(async () => {
  if (!scrollProgressRef.value) return

  try {
    const { ScrollTrigger } = await import('gsap/ScrollTrigger')

    ScrollTrigger.create({
      trigger: document.documentElement,
      start: 'top top',
      end: 'bottom bottom',
      onUpdate: (self) => {
        if (scrollProgressRef.value) {
          scrollProgressRef.value.style.setProperty('--scroll-progress', `${self.progress * 100}%`)
        }
      },
    })
  } catch {
    // GSAP ScrollTrigger is non-critical — gracefully degrade
  }
})
</script>
