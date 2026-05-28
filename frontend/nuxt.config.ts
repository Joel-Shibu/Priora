// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2026-05-01',
  devtools: { enabled: true },  modules: [
    '@nuxtjs/color-mode',
    '@pinia/nuxt',
  ],

  // Nuxt 4 prefixes subdirectory components by default (ui/Badge.vue → UiBadge).
  // Disable prefixes so all templates using <Badge>, <Card>, <Loader> resolve correctly.
  components: [
    { path: '~/components', pathPrefix: false },
  ],

  css: ['~/styles/tokens.css', '~/styles/globals.css'],

  colorMode: {
    classSuffix: '',
    preference: 'dark',
    fallback: 'dark',
    disableTransition: false,
  },

  experimental: {
    viewTransition: false,
  },

  pageTransition: {
    name: 'page',
  },

  app: {
    head: {
      title: 'Priora — Study Smarter, Not Harder',
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
        { name: 'description', content: 'KTU exam priority companion. Instantly know what to study first based on question paper patterns and marks weighting.' },
        { name: 'theme-color', content: '#1A1C23' },
      ],
      link: [
        { rel: 'icon', type: 'image/svg+xml', href: '/favicon.svg' },
        { rel: 'preconnect', href: 'https://fonts.googleapis.com' },
        { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: 'anonymous' },
        { rel: 'stylesheet', href: 'https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500;700&display=swap' },
      ],
    },
  },

  nitro: {
    // ═══ Cloudflare Pages preset ═══
    // Uncomment for Cloudflare Pages deployment, OR set env var:
    // NITRO_PRESET=cloudflare_pages
    // preset: 'cloudflare_pages',

    routeRules: {
      '/**': { cors: true },
    },
  },

  runtimeConfig: {
    apiBase: process.env.NUXT_API_BASE || 'http://127.0.0.1:3001/api',
    public: {
      apiBase: '/api',
    },
  },

  postcss: {
    plugins: {
      '@tailwindcss/postcss': {},
    },
  },
})
