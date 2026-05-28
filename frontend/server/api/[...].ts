export default defineEventHandler(async (event) => {
  const path = event.path || getRequestURL(event).pathname
  const method = getMethod(event) || 'GET'
  const body = method !== 'GET' && method !== 'HEAD' ? await readBody(event).catch(() => undefined) : undefined

  const config = useRuntimeConfig(event)
  const apiPath = path.replace(/^\/api/, '')
  const targetUrl = `${config.apiBase}${apiPath}`

  try {
    const response = await $fetch.raw(targetUrl, {
      method: method as any,
      body,
      headers: {
        'Content-Type': 'application/json',
      },
    })

    if (response._data) {
      return response._data
    }

    return response.body
  } catch (error: any) {
    if (error.response) {
      setResponseStatus(event, error.response.status)
      return error.response._data || { error: 'Request failed' }
    }

    throw createError({
      statusCode: 502,
      statusMessage: 'Backend service unavailable',
    })
  }
})
