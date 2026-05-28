import { gsap } from 'gsap'
import { ScrollTrigger } from 'gsap/ScrollTrigger'
import { SplitText } from 'gsap/SplitText'

let registered = false
function ensurePlugins() {
  if (!registered) {
    gsap.registerPlugin(ScrollTrigger, SplitText)
    registered = true
  }
}

/**
 * GSAP Scroll Reveal — animate elements when they scroll into view.
 * More powerful than CSS-only reveals: supports pin, scrub, stagger control,
 * direction variants, and timeline sequencing.
 *
 * All animations use transform/opacity — compositor-friendly.
 * Cleans up ScrollTrigger instances on unmount.
 */
export function useGsapScrollReveal(
  target: MaybeRefOrGetter<HTMLElement | null | undefined>,
  options?: {
    from?: gsap.TweenVars
    to?: gsap.TweenVars
    /** ScrollTrigger start position. Default: 'top 85%' */
    start?: string
    /** ScrollTrigger end position. Default: 'top 25%' */
    end?: string
    /** Scrub tie scroll position to animation progress. Default: false (play once) */
    scrub?: boolean | number
    /** Toggle markers for debugging */
    markers?: boolean
    /** Toggle pin */
    pin?: boolean
    /** How much to offset from bottom. Default: '-80px' */
    rootMargin?: string
    /** Once toggle (default true) */
    once?: boolean
  },
) {
  ensurePlugins()
  const el = toRef(target)
  let st: ScrollTrigger | null = null

  onMounted(() => {
    if (!el.value) return

    const {
      from = { opacity: 0, y: 40, scale: 0.96 },
      to = { opacity: 1, y: 0, scale: 1 },
      start = 'top 85%',
      end = 'top 25%',
      scrub = false,
      markers = false,
      pin = false,
      once = true,
    } = options ?? {}

    st = ScrollTrigger.create({
      trigger: el.value,
      start,
      end,
      scrub,
      markers,
      pin,
      once,
      onEnter: () => {
        if (scrub) return // scrub handles its own animation
        gsap.fromTo(el.value!, from, {
          ...to,
          duration: 0.7,
          ease: 'power3.out',
          overwrite: 'auto',
        })
      },
      onLeave: () => {
        if (!once && !scrub) {
          gsap.set(el.value!, { clearProps: 'all' })
        }
      },
      onEnterBack: () => {
        if (!once && !scrub && el.value) {
          gsap.fromTo(el.value!, from, {
            ...to,
            duration: 0.7,
            ease: 'power3.out',
            overwrite: 'auto',
          })
        }
      },
    })
  })

  onUnmounted(() => {
    if (st) st.kill()
  })

  return { st }
}

/**
 * Use a GSAP timeline with ScrollTrigger for sequenced multi-step animations.
 * Perfect for staggered card entrances, multi-element reveals.
 */
export function useGsapTimeline(
  container: MaybeRefOrGetter<HTMLElement | null | undefined>,
  options?: {
    /** ScrollTrigger start. Default: 'top 85%' */
    start?: string
    /** ScrollTrigger end. Default: 'top 25%' */
    end?: string
    /** Scrub. Default: false */
    scrub?: boolean | number
    markers?: boolean
    once?: boolean
  },
) {
  ensurePlugins()
  const el = toRef(container)
  let st: ScrollTrigger | null = null
  let tl: gsap.core.Timeline | null = null

  function addToTimeline(targets: gsap.TweenTarget, vars: gsap.TweenVars, position?: gsap.Position) {
    if (!tl) return
    tl.to(targets, vars, position)
    return { tl }
  }

  function addFromTo(
    targets: gsap.TweenTarget,
    fromVars: gsap.TweenVars,
    toVars: gsap.TweenVars,
    position?: gsap.Position,
  ) {
    if (!tl) return
    tl.fromTo(targets, fromVars, toVars, position)
    return { tl }
  }

  onMounted(() => {
    if (!el.value) return

    const {
      start = 'top 85%',
      end = 'top 25%',
      scrub = false,
      markers = false,
      once = true,
    } = options ?? {}

    tl = gsap.timeline({
      paused: true,
      defaults: { duration: 0.6, ease: 'power3.out' },
    })

    st = ScrollTrigger.create({
      trigger: el.value,
      start,
      end,
      scrub,
      markers,
      once,
      onEnter: () => {
        if (!scrub && tl) tl.play()
      },
      onEnterBack: () => {
        if (!once && !scrub && tl) tl.play(0)
      },
      onLeave: () => {
        if (!once && !scrub && tl) tl.pause()
      },
    })
  })

  onUnmounted(() => {
    if (st) st.kill()
    if (tl) tl.kill()
  })

  return { addToTimeline, addFromTo, tl, st }
}

/**
 * Pin a section while animating content inside it.
 * The section stays fixed while the user scrolls through the animation range.
 */
export function useGsapPinSection(
  section: MaybeRefOrGetter<HTMLElement | null | undefined>,
  options?: {
    /** How much scroll distance the pin covers. Default: '+=2000' */
    end?: string
    /** Start position. Default: 'top top' */
    start?: string
    /** Padding to add when pinned. Default: '0px' */
    pinSpacing?: boolean | string
    /** Prevent overlap with other pins */
    anticipatePin?: number
    markers?: boolean
  },
) {
  ensurePlugins()
  const el = toRef(section)
  let st: ScrollTrigger | null = null
  const pinTl = ref<gsap.core.Timeline | null>(null)

  onMounted(() => {
    if (!el.value) return

    const {
      end = '+=2000',
      start = 'top top',
      pinSpacing = true,
      anticipatePin = 1,
      markers = false,
    } = options ?? {}

    const tl = gsap.timeline()
    pinTl.value = tl

    st = ScrollTrigger.create({
      trigger: el.value,
      start,
      end,
      pin: true,
      pinSpacing,
      anticipatePin,
      markers,
      animation: tl,
      scrub: 1,
    })
  })

  onUnmounted(() => {
    if (st) st.kill()
    if (pinTl.value) pinTl.value.kill()
  })

  return { st, pinTl }
}

/**
 * Text split animation — splits text into characters, words, or lines
 * and animates them in with stagger.
 */
export function useGsapTextSplit(
  target: MaybeRefOrGetter<HTMLElement | null | undefined>,
  options?: {
    /** What to split: 'chars', 'words', 'lines'. Default: 'chars' */
    type?: 'chars' | 'words' | 'lines' | string
    /** Stagger delay between elements. Default: 0.03 */
    stagger?: number
    /** Direction: 'up', 'down', 'left', 'right'. Default: 'up' */
    direction?: 'up' | 'down' | 'left' | 'right'
    /** Duration of each element animation. Default: 0.5 */
    duration?: number
    /** Ease. Default: 'power3.out' */
    ease?: string
    /** Play on mount immediately. Default: true */
    playOnMount?: boolean
    /** Trigger on scroll instead of on mount */
    scrollTrigger?: boolean | { start?: string; end?: string; scrub?: boolean | number }
  },
) {
  ensurePlugins()
  const el = toRef(target)
  let split: SplitText | null = null
  let st: ScrollTrigger | null = null

  function animate(dir: string) {
    if (!split) return
    const parts = split.chars || split.words || split.lines
    if (!parts) return

    const fromMap: Record<string, gsap.TweenVars> = {
      up: { opacity: 0, y: 30 },
      down: { opacity: 0, y: -30 },
      left: { opacity: 0, x: -30 },
      right: { opacity: 0, x: 30 },
    }

    const {
      stagger = 0.03,
      duration = 0.5,
      ease = 'power3.out',
    } = options ?? {}

    const fromVars: gsap.TweenVars = (fromMap[dir] ?? fromMap.up) as gsap.TweenVars

    return gsap.fromTo(
      parts,
      fromVars,
      {
        opacity: 1,
        y: 0,
        x: 0,
        duration,
        ease,
        stagger,
      },
    )
  }

  onMounted(() => {
    if (!el.value) return

    const {
      type = 'chars',
      direction = 'up',
      playOnMount = true,
      scrollTrigger: scrollOpts,
    } = options ?? {}

    split = new SplitText(el.value, {
      type,
      charsClass: 'gsap-split-char',
      wordsClass: 'gsap-split-word',
      linesClass: 'gsap-split-line',
    })

    // Hide initially
    const parts = split.chars || split.words || split.lines
    if (parts) gsap.set(parts, { opacity: 0 })

    if (scrollOpts) {
      const ss = typeof scrollOpts === 'boolean' ? {} : scrollOpts
      st = ScrollTrigger.create({
        trigger: el.value,
        start: ss.start || 'top 85%',
        end: ss.end || 'top 25%',
        scrub: ss.scrub ?? false,
        once: true,
        onEnter: () => animate(direction),
      })
    } else if (playOnMount) {
      animate(direction)
    }
  })

  onUnmounted(() => {
    if (st) st.kill()
    if (split) split.revert()
  })

  return { animate, split }
}

/**
 * Progress-driven bar reveal — animates width from 0 to target percentage
 * when element scrolls into view. Great for skill bars, score bars.
 */
export function useGsapProgressBar(
  target: MaybeRefOrGetter<HTMLElement | null | undefined>,
  options?: {
    targetPercent?: number
    duration?: number
    start?: string
    ease?: string
  },
) {
  ensurePlugins()
  const el = toRef(target)
  let st: ScrollTrigger | null = null

  onMounted(() => {
    if (!el.value) return

    const {
      targetPercent = 100,
      duration = 0.8,
      start = 'top 85%',
      ease = 'power3.out',
    } = options ?? {}

    gsap.set(el.value, { width: '0%' })

    st = ScrollTrigger.create({
      trigger: el.value,
      start,
      once: true,
      onEnter: () => {
        gsap.to(el.value!, {
          width: `${targetPercent}%`,
          duration,
          ease,
          overwrite: 'auto',
        })
      },
    })
  })

  onUnmounted(() => {
    if (st) st.kill()
  })

  return { st }
}

/**
 * Mouse parallax — elements move subtly in response to pointer position.
 * Creates a 3D depth effect. Automatically disabled on touch devices.
 * Uses requestAnimationFrame for smooth 60fps performance.
 */
export function useGsapMouseParallax(
  target: MaybeRefOrGetter<HTMLElement | null | undefined>,
  options?: {
    /** Movement strength multiplier. Default: 0.04 (4% of distance from center) */
    strengthX?: number
    /** Movement strength multiplier for Y. Defaults to strengthX */
    strengthY?: number
    /** Maximum pixel movement. Default: 12 */
    clampPx?: number
    /** Invert direction */
    invert?: boolean
  },
) {
  const el = toRef(target)
  const state = reactive({ x: 0, y: 0 })
  let ticking = false
  let cleanup: (() => void) | null = null

  // Check for coarse pointer (touch device)
  const isFinePointer = import.meta.client
    ? window.matchMedia('(pointer: fine)').matches
    : true

  onMounted(() => {
    if (!el.value || !isFinePointer) return

    const {
      strengthX = 0.04,
      strengthY: strengthYOpt,
      clampPx = 12,
      invert = false,
    } = options ?? {}
    const strengthY = strengthYOpt ?? strengthX
    const dir = invert ? -1 : 1

    const handler = (e: PointerEvent) => {
      if (ticking) return
      ticking = true
      requestAnimationFrame(() => {
        if (!el.value) { ticking = false; return }
        const rect = el.value.getBoundingClientRect()
        const centerX = rect.left + rect.width / 2
        const centerY = rect.top + rect.height / 2
        const dx = (e.clientX - centerX) * strengthX * dir
        const dy = (e.clientY - centerY) * strengthY * dir
        state.x = Math.min(Math.max(dx, -clampPx), clampPx)
        state.y = Math.min(Math.max(dy, -clampPx), clampPx)
        el.value!.style.transform = `translate3d(${state.x.toFixed(1)}px, ${state.y.toFixed(1)}px, 0)`
        ticking = false
      })
    }

    const resetHandler = () => {
      if (el.value) {
        el.value.style.transform = 'translate3d(0, 0, 0)'
      }
      state.x = 0
      state.y = 0
    }

    document.addEventListener('pointermove', handler, { passive: true })
    document.addEventListener('pointerleave', resetHandler, { passive: true })

    cleanup = () => {
      document.removeEventListener('pointermove', handler)
      document.removeEventListener('pointerleave', resetHandler)
    }
  })

  onUnmounted(() => {
    if (cleanup) cleanup()
  })

  return { state }
}

/**
 * 3D Tilt on hover — card tilts toward pointer position for depth.
 * Only activates on fine-pointer (non-touch) devices.
 */
export function useGsapTiltCard(
  target: MaybeRefOrGetter<HTMLElement | null | undefined>,
  options?: {
    /** Max rotation in degrees. Default: 6 */
    maxRotate?: number
    /** Perspective in px. Default: 800 */
    perspective?: number
    /** Scale on hover. Default: 1.02 */
    scale?: number
  },
) {
  const el = toRef(target)
  let cleanup: (() => void) | null = null

  const isFinePointer = import.meta.client
    ? window.matchMedia('(pointer: fine)').matches
    : true

  onMounted(() => {
    if (!el.value || !isFinePointer) return

    const { maxRotate = 6, perspective = 800, scale = 1.02 } = options ?? {}
    const card = el.value

    const enterHandler = () => {
      card.style.transition = 'transform 0.15s ease-out'
    }

    const moveHandler = (e: PointerEvent) => {
      const rect = card.getBoundingClientRect()
      const x = (e.clientX - rect.left) / rect.width
      const y = (e.clientY - rect.top) / rect.height
      const rotX = (y - 0.5) * -maxRotate
      const rotY = (x - 0.5) * maxRotate
      card.style.transform = `perspective(${perspective}px) rotateX(${rotX.toFixed(1)}deg) rotateY(${rotY.toFixed(1)}deg) scale3d(${scale}, ${scale}, ${scale})`
    }

    const leaveHandler = () => {
      card.style.transition = 'transform 0.3s ease-out'
      card.style.transform = 'perspective(800px) rotateX(0deg) rotateY(0deg) scale3d(1, 1, 1)'
    }

    card.addEventListener('pointerenter', enterHandler, { passive: true })
    card.addEventListener('pointermove', moveHandler, { passive: true })
    card.addEventListener('pointerleave', leaveHandler, { passive: true })

    cleanup = () => {
      card.removeEventListener('pointerenter', enterHandler)
      card.removeEventListener('pointermove', moveHandler)
      card.removeEventListener('pointerleave', leaveHandler)
    }
  })

  onUnmounted(() => {
    if (cleanup) cleanup()
  })
}

export { gsap, ScrollTrigger, SplitText }
