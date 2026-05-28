<template>
  <!-- ═══ INTRO LOADER — Fun book animation centered on first visit ═══ -->
  <Teleport to="body">
    <Transition name="intro-loader">
      <div
        v-if="showIntroLoader"
        class="intro-loader-overlay"
      >
        <FunLoader variant="page" />
      </div>
    </Transition>
  </Teleport>

  <div class="relative">
    <!-- ═══ HERO — Compact, accessible ───
         No pin, no full-viewport. Form is
         visible immediately. Subtle entrance
         animations on load (char split + stagger).
         Tagline has a gentle interactive glow. -->
    <section class="max-w-6xl mx-auto px-4 sm:px-8 pt-20 sm:pt-28 pb-8 text-center">
      <!-- Badge -->
      <div ref="heroBadgeRef" class="inline-flex items-center gap-2 px-3 py-1 rounded-full border border-[var(--color-primary-500)]/20 bg-[var(--color-primary-500)]/5 text-[var(--color-primary-500)] text-xs font-medium mb-6" style="opacity: 0">
        <span class="w-1.5 h-1.5 rounded-full bg-[var(--color-primary-500)] animate-pulse-glow" />
        KTU 2024 Scheme
      </div>

      <!-- Headline (flat text for clean SplitText) -->
      <h1 ref="heroHeadlineRef" class="text-fluid-hero font-bold text-[var(--color-text-primary)] max-w-3xl mx-auto">
        Know exactly
        <span class="text-gradient-highlight">what to study first</span>
      </h1>

      <!-- Interactive tagline: subtle glow wave animation + responsive hover -->
      <p ref="heroSubRef" class="mt-5 text-fluid-body max-w-xl mx-auto interactive-tagline" style="opacity: 0">
        <span class="tagline-text">
          Priora analyzes KTU question paper patterns to tell you exactly which topics to prioritize — so you stop guessing and start studying smarter.
        </span>
        <span class="tagline-glow" aria-hidden="true">
          Priora analyzes KTU question paper patterns to tell you exactly which topics to prioritize — so you stop guessing and start studying smarter.
        </span>
      </p>
    </section>

    <!-- ═══ SELECTION FORM (Bento Grid) ═══ -->
    <section class="max-w-6xl mx-auto px-4 sm:px-8 pb-8">
      <div class="grid grid-cols-1 lg:grid-cols-5 gap-4 sm:gap-6">
        <!-- Form Panel -->
        <div class="lg:col-span-2">
          <div ref="formRef" class="surface-raised card-accent p-5 sm:p-7">
            <h2 class="text-sm font-semibold text-[var(--color-text-primary)] mb-5 tracking-tight flex items-center gap-2">
              <span class="w-1.5 h-1.5 rounded-full bg-[var(--color-primary-500)]" />
              Start Your Analysis
            </h2>

            <div class="space-y-3.5">
              <!-- Scheme -->
              <div>
                <label class="block text-xs font-medium text-[var(--color-text-secondary)] mb-1.5 uppercase tracking-wider">Scheme</label>
                <select
                  :value="store.selectedScheme?.id ?? ''"
                  @change="store.onSchemeChange(($event.target as HTMLSelectElement).value || null)"
                  class="w-full input-depth appearance-none"
                >
                  <option v-if="store.isLoadingSchemes" value="" disabled>Loading...</option>
                  <option v-for="s in store.schemes" :key="s.id" :value="s.id">{{ s.name }}</option>
                </select>
              </div>

              <!-- Branch -->
              <div>
                <label class="block text-xs font-medium text-[var(--color-text-secondary)] mb-1.5 uppercase tracking-wider">Branch</label>
                <select
                  :value="store.selectedBranch?.id ?? ''"
                  @change="store.onBranchChange(($event.target as HTMLSelectElement).value || null)"
                  :disabled="!store.branches.length"
                  class="w-full input-depth appearance-none disabled:opacity-40 disabled:cursor-not-allowed"
                >
                  <option value="" disabled>{{ store.isLoadingBranches ? 'Loading...' : 'Select branch' }}</option>
                  <option v-for="b in store.branches" :key="b.id" :value="b.id">{{ b.name }}</option>
                </select>
              </div>

              <!-- Semester -->
              <div>
                <label class="block text-xs font-medium text-[var(--color-text-secondary)] mb-1.5 uppercase tracking-wider">Semester</label>
                <select
                  :value="store.selectedSemester?.id ?? ''"
                  @change="store.onSemesterChange(($event.target as HTMLSelectElement).value || null)"
                  :disabled="!store.semesters.length"
                  class="w-full input-depth appearance-none disabled:opacity-40 disabled:cursor-not-allowed"
                >
                  <option value="" disabled>{{ store.isLoadingSemesters ? 'Loading...' : 'Select semester' }}</option>
                  <option v-for="s in store.semesters" :key="s.id" :value="s.id">Semester {{ s.semester_number }}</option>
                </select>
              </div>

              <!-- Subject -->
              <div>
                <label class="block text-xs font-medium text-[var(--color-text-secondary)] mb-1.5 uppercase tracking-wider">Subject</label>
                <select
                  :value="store.selectedSubject?.id ?? ''"
                  @change="store.onSubjectChange(($event.target as HTMLSelectElement).value || null)"
                  :disabled="!store.subjects.length"
                  class="w-full input-depth appearance-none disabled:opacity-40 disabled:cursor-not-allowed"
                >
                  <option value="" disabled>{{ store.isLoadingSubjects ? 'Loading...' : 'Select subject' }}</option>
                  <option v-for="s in store.subjects" :key="s.id" :value="s.id">{{ s.subject_code }} &mdash; {{ s.subject_name }}</option>
                </select>
              </div>

              <!-- Days Remaining -->
              <div>
                <label class="block text-xs font-medium text-[var(--color-text-secondary)] mb-1.5 uppercase tracking-wider">Days Until Exam</label>
                <div class="relative">
                  <input
                    v-model.number="store.daysRemaining"
                    type="number"
                    min="1"
                    max="365"
                    placeholder="30"
                    class="w-full input-depth [&::-webkit-inner-spin-button]:opacity-50"
                  />
                </div>
                <p v-if="store.daysRemaining < 1 || store.daysRemaining > 365" class="mt-1 text-xs text-[var(--color-error)]">
                  Enter a number between 1 and 365
                </p>
              </div>

              <!-- Analyze Button -->
              <button
                @click="handleAnalyze"
                :disabled="!store.isValid || store.isAnalyzing"
                class="w-full mt-3 px-6 py-3 rounded-xl bg-gradient-to-r from-[var(--color-primary-500)] to-[var(--color-primary-600)] text-white font-semibold text-sm shadow-md transition-all duration-200 hover:from-[var(--color-primary-400)] hover:to-[var(--color-primary-600)] hover:shadow-[var(--shadow-glow)] hover:-translate-y-[1px] active:translate-y-[1px] active:shadow-sm disabled:opacity-40 disabled:cursor-not-allowed disabled:hover:shadow-md disabled:hover:translate-y-0 disabled:active:shadow-md disabled:active:translate-y-0"
              >
                <span v-if="store.isAnalyzing" class="flex items-center justify-center gap-2">
                  <span class="inline-block w-4 h-4 rounded-full border-2 border-white/30 border-t-white animate-spin" />
                  Analyzing...
                </span>
                <span v-else class="flex items-center justify-center gap-2">
                  Analyze My Subjects
                  <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/>
                  </svg>
                </span>
              </button>
            </div>

            <!-- Error display -->
            <div v-if="store.error" class="mt-4 p-3 rounded-xl border border-[var(--priority-high-border)] bg-[var(--priority-high-bg)] text-[var(--priority-high)] text-sm animate-scale-in">
              <div class="flex items-start gap-2">
                <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 mt-0.5 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/>
                </svg>
                <span>{{ store.error }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Results / Preview Panel -->
        <div class="lg:col-span-3">
          <div v-if="store.analysisResult" class="animate-scale-in">
            <AnalysisResults :result="store.analysisResult" />
          </div>

          <!-- Empty state -->
          <div v-else ref="previewRef" class="surface-raised card-accent p-7 sm:p-10 h-full flex flex-col items-center justify-center text-center">
            <div class="w-14 h-14 rounded-2xl border border-[var(--color-border)] bg-[var(--color-surface-alt)] flex items-center justify-center mb-5">
              <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 text-[var(--color-text-tertiary)]" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                <path d="M9 19v-6a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h2a2 2 0 0 0 2-2zm0 0V9a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v10m-6 0a2 2 0 0 0 2 2h2a2 2 0 0 0 2-2m0 0V5a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-2a2 2 0 0 1-2-2z"/>
              </svg>
            </div>
            <h3 class="text-base font-semibold text-[var(--color-text-primary)] mb-1.5">Ready to analyze</h3>
            <p class="text-sm text-[var(--color-text-secondary)] max-w-xs">
              Select your subject and enter your exam timeline to get a ranked study plan.
            </p>
            <div class="grid grid-cols-3 gap-3 w-full max-w-sm mt-6">
              <div class="text-center p-3 rounded-xl bg-[var(--color-surface-alt)] border border-[var(--color-border)]">
                <p class="text-xs font-mono font-bold text-[var(--color-primary-500)]">1</p>
                <p class="text-[10px] text-[var(--color-text-tertiary)] mt-0.5">Select Subject</p>
              </div>
              <div class="text-center p-3 rounded-xl bg-[var(--color-surface-alt)] border border-[var(--color-border)]">
                <p class="text-xs font-mono font-bold text-[var(--color-primary-500)]">2</p>
                <p class="text-[10px] text-[var(--color-text-tertiary)] mt-0.5">Enter Days</p>
              </div>
              <div class="text-center p-3 rounded-xl bg-[var(--color-surface-alt)] border border-[var(--color-border)]">
                <p class="text-xs font-mono font-bold text-[var(--color-primary-500)]">3</p>
                <p class="text-[10px] text-[var(--color-text-tertiary)] mt-0.5">Get Priorities</p>
              </div>
            </div>

            <!-- Demo score bar showing GSAP progress bar -->
            <div class="w-full max-w-sm mt-6 pt-4 border-t border-[var(--color-border)] opacity-50">
              <div class="flex items-center justify-between text-[10px] text-[var(--color-text-tertiary)] mb-1.5">
                <span>Topic Coverage Preview</span>
                <span>78%</span>
              </div>
              <div class="score-bar-track">
                <div ref="scoreBarRef" class="score-bar-fill--gsap bg-priority-medium" 
                     style="width: 0%; height: 100%; border-radius: 3px; transition: none;" />
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- ═══ HOW IT WORKS ═══ -->
    <section class="max-w-6xl mx-auto px-4 sm:px-8 py-14 sm:py-18">
      <div class="text-center mb-9">
        <h2 ref="hiwHeadingRef" class="text-fluid-h2 font-bold text-[var(--color-text-primary)]">How It Works</h2>
        <p class="mt-2 text-sm text-[var(--color-text-secondary)] max-w-md mx-auto">
          Three simple steps to smarter studying
        </p>
      </div>

      <div ref="hiwCardsRef" class="grid sm:grid-cols-3 gap-4 sm:gap-6 perspective-container">
        <div ref="hiwCard1Ref" class="surface-raised card-accent p-6 text-center gsap-hiw-card tilt-card">
          <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-[var(--color-primary-500)] to-[var(--color-primary-600)] flex items-center justify-center text-white font-bold text-lg mx-auto mb-4">
            1
          </div>
          <h3 class="font-semibold text-sm text-[var(--color-text-primary)] mb-1.5">Select Your Subject</h3>
          <p class="text-xs text-[var(--color-text-secondary)] leading-relaxed">Choose your scheme, branch, semester, and subject from the KTU syllabus.</p>
        </div>
        <div ref="hiwCard2Ref" class="surface-raised card-accent p-6 text-center gsap-hiw-card tilt-card">
          <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-[var(--color-primary-500)] to-[var(--color-primary-600)] flex items-center justify-center text-white font-bold text-lg mx-auto mb-4">
            2
          </div>
          <h3 class="font-semibold text-sm text-[var(--color-text-primary)] mb-1.5">Enter Time Remaining</h3>
          <p class="text-xs text-[var(--color-text-secondary)] leading-relaxed">Tell us your exam timeline to adjust priority weights for your schedule.</p>
        </div>
        <div ref="hiwCard3Ref" class="surface-raised card-accent p-6 text-center gsap-hiw-card tilt-card">
          <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-[var(--color-primary-500)] to-[var(--color-primary-600)] flex items-center justify-center text-white font-bold text-lg mx-auto mb-4">
            3
          </div>
          <h3 class="font-semibold text-sm text-[var(--color-text-primary)] mb-1.5">Get Priorities</h3>
          <p class="text-xs text-[var(--color-text-secondary)] leading-relaxed">See ranked topics with clear explanations so you know exactly what to study.</p>
        </div>
      </div>
    </section>

    <!-- ═══ STATS BANNER ═══ -->
    <section ref="statsRef" class="max-w-6xl mx-auto px-4 sm:px-8 pb-20">
      <div class="rounded-2xl border border-[var(--color-border)] bg-gradient-to-br from-[var(--color-surface-alt)] to-[var(--color-surface)] p-6 sm:p-8">
        <div ref="statNumbersRef" class="grid grid-cols-2 sm:grid-cols-4 gap-6 sm:gap-8 text-center">
          <div v-for="stat in stats" :key="stat.label">
            <p class="text-2xl sm:text-3xl font-bold text-[var(--color-text-primary)]">
              <span class="gsap-stat-value" :data-target="stat.value">0</span>
            </p>
            <p class="text-xs text-[var(--color-text-tertiary)] mt-1">{{ stat.label }}</p>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { gsap } from 'gsap'

const store = useSelectionStore()
const router = useRouter()

async function handleAnalyze() {
  const result = await store.analyze()
  if (result) {
    router.push(`/analyze/${result.analysis_id}`)
  }
}

// ═══ REFS ═══
const heroHeadlineRef = ref<HTMLElement | null>(null)
const heroBadgeRef = ref<HTMLElement | null>(null)
const heroSubRef = ref<HTMLElement | null>(null)
const formRef = ref<HTMLElement | null>(null)
const previewRef = ref<HTMLElement | null>(null)
const hiwHeadingRef = ref<HTMLElement | null>(null)
const hiwCardsRef = ref<HTMLElement | null>(null)
const hiwCard1Ref = ref<HTMLElement | null>(null)
const hiwCard2Ref = ref<HTMLElement | null>(null)
const hiwCard3Ref = ref<HTMLElement | null>(null)
const statsRef = ref<HTMLElement | null>(null)
const statNumbersRef = ref<HTMLElement | null>(null)
const scoreBarRef = ref<HTMLElement | null>(null)

// ════════════════════════════════════════════
// HERO — TEXT SPLIT (CHAR STAGGER ENTRANCE)
// Characters of the headline animate up on load
// ════════════════════════════════════════════
useGsapTextSplit(heroHeadlineRef, {
  type: 'chars',
  stagger: 0.025,
  duration: 0.5,
  direction: 'up',
  playOnMount: true,
})

// ════════════════════════════════════════════
// HERO — MOUSE PARALLAX (INTERACTIVE DEPTH)
// Elements move subtly based on pointer position
// Only activates on mouse devices, not touch.
// ════════════════════════════════════════════
// Gentle interactive depth on hero elements
useGsapMouseParallax(heroBadgeRef, { strengthX: 0.02, strengthY: 0.01, clampPx: 6 })
useGsapMouseParallax(heroHeadlineRef, { strengthX: 0.03, strengthY: 0.015, clampPx: 10 })
// Tagline gets the most pronounced interactive effect
useGsapMouseParallax(heroSubRef, { strengthX: 0.08, strengthY: 0.04, clampPx: 8 })

// ════════════════════════════════════════════
// HOW IT WORKS — TEXT SPLIT ON HEADING
// Scroll-triggered word split animation
// ════════════════════════════════════════════
useGsapTextSplit(hiwHeadingRef, {
  type: 'words',
  stagger: 0.08,
  duration: 0.4,
  direction: 'up',
  scrollTrigger: { start: 'top 85%' },
})

// ════════════════════════════════════════════
// HOW IT WORKS — 3D TILT ON CARDS
// Interactive hover tilt effect on desktop
// ════════════════════════════════════════════
useGsapTiltCard(hiwCard1Ref, { maxRotate: 5, scale: 1.02 })
useGsapTiltCard(hiwCard2Ref, { maxRotate: 5, scale: 1.02 })
useGsapTiltCard(hiwCard3Ref, { maxRotate: 5, scale: 1.02 })

// ─── Form panel: GSAP scroll reveal ───
useGsapScrollReveal(formRef, {
  from: { opacity: 0, x: -40, scale: 0.95 },
  to: { opacity: 1, x: 0, scale: 1 },
  start: 'top 85%',
})

// ─── Preview panel: GSAP scroll reveal ───
useGsapScrollReveal(previewRef, {
  from: { opacity: 0, x: 40, scale: 0.95 },
  to: { opacity: 1, x: 0, scale: 1 },
  start: 'top 85%',
})

// ─── How It Works: card stagger timeline ───
const { addFromTo: hiwAddFromTo } = useGsapTimeline(hiwCardsRef, {
  start: 'top 85%',
})

// ─── Stats: scale reveal + number count-up ───
useGsapScrollReveal(statsRef, {
  from: { opacity: 0, scale: 0.88 },
  to: { opacity: 1, scale: 1 },
  start: 'top 70%',
})

// ─── Score bar: GSAP progress bar reveal ───
useGsapProgressBar(scoreBarRef, {
  targetPercent: 78,
  duration: 1.0,
  start: 'top 85%',
})

// ─── Stats Data — initialized with "—" to avoid flash of zeros ───
const stats = ref<{ value: string; label: string }[]>([
  { value: '—', label: 'Subjects' },
  { value: '—', label: 'Topics' },
  { value: '—', label: 'Question Papers' },
  { value: '—', label: 'Scheme' },
])

// ═══ INTRO LOADER STATE ═══
const showIntroLoader = ref(true)

// ════════════════════════════════════════════
// ON MOUNTED — Single consolidated lifecycle
// ════════════════════════════════════════════
onMounted(async () => {
  // ── 1. Badge + subtitle entrance animation ──
  if (heroBadgeRef.value && heroSubRef.value) {
    gsap.fromTo(
      [heroBadgeRef.value, heroSubRef.value],
      { opacity: 0, y: 20 },
      {
        opacity: 1,
        y: 0,
        duration: 0.5,
        stagger: 0.12,
        ease: 'power3.out',
        delay: 0.2,
      },
    )
  }

  // ── 2. How It Works: card stagger timeline ──
  if (hiwCardsRef.value) {
    const cards = hiwCardsRef.value.querySelectorAll(':scope > .gsap-hiw-card')
    if (cards.length) {
      hiwAddFromTo(
        cards,
        { opacity: 0, y: 50, scale: 0.9 },
        { opacity: 1, y: 0, scale: 1, stagger: 0.12, duration: 0.55 },
      )
    }
  }

  // ── 3. Store init + stats fetch ──
  store.analysisResult = null
  store.loadSchemes()

  try {
    const data = await $fetch<{ subjects: number; topics: number; papers: number; schemes: number }>('/stats')
    if (data) {
      stats.value = [
        { value: String(data.subjects || 0), label: 'Subjects' },
        { value: String(data.topics || 0), label: 'Topics' },
        { value: String(data.papers || 0), label: 'Question Papers' },
        { value: String(data.schemes || 0), label: 'Scheme' },
      ]
    }
  } catch {
    console.warn('Failed to load stats — backend may be unreachable')
  }

  // ── 4. Stats count-up (responsive: scrub on desktop, once on mobile) ──
  await nextTick()

  if (!statNumbersRef.value) return
  const statEls = statNumbersRef.value.querySelectorAll('.gsap-stat-value')
  if (!statEls.length) return

  const mm = gsap.matchMedia()

  // Desktop: scrub-linked count-up (scroll-driven, smoother)
  mm.add('(min-width: 768px)', () => {
    statEls.forEach((el) => {
      const targetVal = parseInt(el.getAttribute('data-target') || '0', 10)
      if (!targetVal || targetVal <= 0) return
      gsap.fromTo(
        el,
        { textContent: 0 },
        {
          textContent: targetVal,
          duration: 1.5,
          ease: 'power2.out',
          snap: { textContent: 1 },
          scrollTrigger: {
            trigger: el,
            start: 'top 80%',
            once: true,
          },
        },
      )
    })
  })

  // Mobile: simpler once-based count-up (no scrub for performance)
  mm.add('(max-width: 767px)', () => {
    statEls.forEach((el) => {
      const targetVal = parseInt(el.getAttribute('data-target') || '0', 10)
      if (!targetVal || targetVal <= 0) return
      gsap.fromTo(
        el,
        { textContent: 0 },
        {
          textContent: targetVal,
          duration: 1.0,
          ease: 'power2.out',
          snap: { textContent: 1 },
          scrollTrigger: {
            trigger: el,
            start: 'top 85%',
            once: true,
          },
        },
      )
    })
  })

  // ── 5. Intro loader — only on first visit per session ──
  const hasSeenIntro = sessionStorage.getItem('priora_intro_seen')
  if (hasSeenIntro) {
    showIntroLoader.value = false
    return
  }

  sessionStorage.setItem('priora_intro_seen', '1')
  setTimeout(() => {
    showIntroLoader.value = false
  }, 3200)
})
</script>
