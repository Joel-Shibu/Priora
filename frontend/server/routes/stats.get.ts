// @ts-nocheck — Nitro route type resolution hits TS recursive depth limit
// when combined with the catch-all proxy route. The runtime works correctly.

// ── Stats Aggregator ──────────────────────────────────────────────────────
// Aggregates real counts from the backend API by cascading through:
//   schemes → branches → semesters → subjects → modules → topics
// Papers are counted per-subject by semester rules (S1-2: 3, S3-4: 2, S5-7: 1).
// ──────────────────────────────────────────────────────────────────────────

export default defineEventHandler(async () => {
  const config = useRuntimeConfig()
  const apiBase = config.apiBase || 'http://127.0.0.1:3001/api'

  // ── 1. Fetch schemes ──────────────────────────────────────────────
  const schemes = (await $fetch(`${apiBase}/schemes`).catch(() => [])) as Record<string, unknown>[]
  const schemeCount = schemes.length

  let subjectCount = 0
  let topicCount = 0
  let paperCount = 0

  // ── 2. Cascade through the hierarchy ──────────────────────────────
  for (const scheme of schemes) {
    const branches = (await $fetch(`${apiBase}/schemes/${scheme.id}/branches`).catch(() => [])) as Record<string, unknown>[]

    for (const branch of branches) {
      const semesters = (await $fetch(`${apiBase}/branches/${branch.id}/semesters`).catch(() => [])) as Record<string, unknown>[]

      for (const semester of semesters) {
        const subjects = (await $fetch(`${apiBase}/semesters/${semester.id}/subjects`).catch(() => [])) as Record<string, unknown>[]
        subjectCount += subjects.length

        // Count papers per semester based on semester number
        const semNum = Number(semester.semester_number)
        if ([1, 2].includes(semNum)) {
          paperCount += subjects.length * 3
        } else if ([3, 4].includes(semNum)) {
          paperCount += subjects.length * 2
        } else if ([5, 6, 7].includes(semNum)) {
          paperCount += subjects.length * 1
        }

        // Count topics — fetch detail for each subject for accuracy
        for (const subject of subjects) {
          try {
            const detail = (await $fetch(`${apiBase}/subjects/${subject.id}`)) as {
              modules?: { topics?: unknown[] }[]
            }
            if (detail?.modules) {
              for (const mod of detail.modules) {
                topicCount += mod.topics?.length ?? 0
              }
            }
          } catch {
            // Skip subjects without detail
          }
        }
      }
    }
  }

  return {
    subjects: subjectCount,
    topics: topicCount,
    papers: paperCount,
    schemes: schemeCount,
  }
})
