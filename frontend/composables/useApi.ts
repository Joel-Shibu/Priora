export function useApi() {
  const config = useRuntimeConfig()
  const baseUrl = config.public.apiBase || 'http://localhost:3001/api'

  async function fetchApi<T>(endpoint: string, options?: RequestInit): Promise<T> {
    const url = `${baseUrl}${endpoint}`
    const res = await fetch(url, {
      headers: {
        'Content-Type': 'application/json',
        ...options?.headers,
      },
      ...options,
    })

    if (!res.ok) {
      const error = await res.json().catch(() => ({ error: 'Request failed' }))
      throw new Error(error.error || `HTTP ${res.status}`)
    }

    return res.json()
  }

  return {
    fetchApi,
    baseUrl,
  }
}
