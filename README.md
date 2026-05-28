# Priora 🎯

**A decision intelligence engine for KTU 2024 Scheme students that ranks what to study first, what to deprioritize, and why — using syllabus structure, previous question paper patterns, marks weighting, and time-remaining constraints.**

> *"Now I know exactly what to study first."*

---

## Features

- **Smart Topic Ranking** — Deterministic scoring based on frequency, marks weight, recency, and time pressure
- **Three Priority Buckets** — High / Medium / Low — instantly see where to focus
- **Explainable Results** — Every topic shows *why* it's ranked where it is
- **60+ Real Question Papers** — Pre-seeded with KTU 2024 Scheme CSE data (S1–S8, 32 subjects)
- **512 Topics Across 128 Modules** — Fully mapped with topic_stats computed
- **Mobile-First UI** — Tailwind CSS, Nuxt 3 SSR, fast interactions
- **Admin Tools** — Upload question papers, manage subjects/modules/topics
- **Production Hardened** — CORS, rate limiting, security headers, request timeouts, body limits
- **53 Integration Tests** — Schema constraints, seed data correctness, API endpoints, edge cases

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| **Frontend** | Nuxt 3, Vue 3, Tailwind CSS, Pinia |
| **Backend** | Rust, Axum, Tokio, SQLx |
| **Database** | PostgreSQL 14+ |
| **Container** | Docker (backend) |
| **Auth** | Supabase Auth (optional) |

---

## Quick Start

### Prerequisites

- Rust 1.75+ (`cargo` installed)
- Node.js 20+ / pnpm
- PostgreSQL 14+
- Docker (optional, for the database)

### 1. Database Setup

```bash
# Start PostgreSQL (or use Docker)
docker run -d \
  --name priora-db \
  -e POSTGRES_USER=priora \
  -e POSTGRES_PASSWORD=priora_dev_2024 \
  -e POSTGRES_DB=priora \
  -p 5433:5432 \
  postgres:16-alpine
```

### 2. Backend

```bash
cd backend
cp .env.example .env        # Edit if needed
cargo run --release          # Builds + runs, seeds data automatically
```

The server starts on `http://127.0.0.1:3001`. On first launch, it:
1. Runs database migrations
2. Seeds the 2024 Scheme CSE curriculum (32 subjects, 128 modules, 512 topics)
3. Seeds 62 real question papers (620 questions, 744 topic mappings)
4. Computes topic_stats (frequency, marks, recency, priority scores)

### 3. Frontend

```bash
cd frontend
pnpm install
pnpm dev --port 3000
```

Open `http://localhost:3000` and start analyzing.

> **Note:** The frontend proxies API requests to `http://127.0.0.1:3001` by default. See `nuxt.config.ts` → `apiBase`.

---

## Deployment

### Backend (Docker)

```dockerfile
FROM rust:1.85-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin priora-api

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/priora-api /usr/local/bin/
EXPOSE 3001
CMD ["priora-api"]
```

```bash
docker build -t priora-api ./backend
docker run -d \
  -e DATABASE_URL=postgres://user:pass@host:5432/priora \
  -e CORS_ORIGIN=https://your-frontend.com \
  -e RATE_LIMIT_RPM=60 \
  -e MAX_BODY_SIZE=5242880 \
  -e REQUEST_TIMEOUT_SECS=30 \
  -p 3001:3001 \
  priora-api
```

### Frontend (Vercel / Cloudflare Pages)

```bash
cd frontend
pnpm build
# Deploy .output/public to your static host
```

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `postgres://priora:priora_dev_2024@localhost:5433/priora` | PostgreSQL connection string |
| `SERVER_HOST` | `0.0.0.0` | API bind address |
| `SERVER_PORT` | `3001` | API port |
| `CORS_ORIGIN` | `*` | Allowed CORS origin (set to frontend URL in production) |
| `RATE_LIMIT_RPM` | `120` | Requests per minute per IP |
| `MAX_BODY_SIZE` | `5242880` | Max request body in bytes (5 MB) |
| `REQUEST_TIMEOUT_SECS` | `30` | Request timeout in seconds |

---

## API Reference

### Public Endpoints

#### `GET /api/health`
Health check. Returns `ok`.

#### `GET /api/schemes`
List active schemes.

#### `GET /api/schemes/:schemeId/branches`
List branches for a scheme.

#### `GET /api/branches/:branchId/semesters`
List semesters for a branch.

#### `GET /api/semesters/:semesterId/subjects`
List subjects for a semester.

#### `GET /api/subjects/:subjectId`
Get subject with modules and topics.

#### `POST /api/subjects/:subjectId/analyze`
Run a priority analysis.

**Request body:**
```json
{
  "subject_id": "uuid",
  "days_remaining": 30
}
```

**Response:**
```json
{
  "analysis_id": "uuid",
  "subject_code": "GAMAT101",
  "subject_name": "Mathematics for Information Science-1",
  "days_remaining": 30,
  "total_topics": 16,
  "confidence": "High",
  "priority_buckets": {
    "high": [{ "topic_name": "...", "priority_score": 1.42, "reasons": ["..."] }],
    "medium": [...],
    "low": [...]
  }
}
```

#### `POST /api/feedback`
Submit feedback on an analysis.

```json
{
  "analysis_id": "uuid",
  "rating": 4,
  "comment": "Very helpful"
}
```

### Admin Endpoints

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/api/admin/subjects` | Create subject |
| `POST` | `/api/admin/modules` | Create module |
| `POST` | `/api/admin/topics` | Create topic (auto-creates topic_stats) |
| `POST` | `/api/admin/question-papers` | Upload question paper with questions |

---

## Testing

```bash
cd backend

# All tests (seed data, schema constraints, API endpoints)
cargo test -- --test-threads=1

# Specific test groups
cargo test seed -- --test-threads=1
cargo test api -- --test-threads=1
cargo test constraint -- --test-threads=1

# Run with the API server running (required for API tests)
cargo run --release &
cargo test api -- --test-threads=1
```

> `--test-threads=1` ensures test isolation since all integration tests share a single database.

---

## Project Structure

```
priora/
├── backend/                    # Rust API server
│   ├── src/
│   │   ├── main.rs             # Server setup, middleware, routes
│   │   ├── config.rs           # Environment configuration
│   │   ├── error.rs            # API error types
│   │   ├── db/
│   │   │   ├── migrations/     # SQLx migrations
│   │   │   └── seed.rs         # Curriculum & question paper seeding
│   │   ├── handlers/           # Request handlers
│   │   ├── models/             # Data types & validation
│   │   └── services/
│   │       └── analysis_engine.rs  # Core ranking algorithm
│   └── tests/
│       └── integration_test.rs  # 53 integration tests
├── frontend/                   # Nuxt 3 SSR app
│   ├── pages/                  # Route pages
│   ├── components/             # Vue components
│   ├── stores/                 # Pinia stores
│   ├── types/                  # TypeScript types
│   └── nuxt.config.ts
└── docs/
    └── prd.md                  # Full product requirements
```

---

## Ranking Algorithm

The priority score for each topic is computed deterministically:

| Component | Weight | Description |
|-----------|--------|-------------|
| Frequency | 25% | How often the topic appears across question papers |
| Marks | 30% | Total marks the topic carries (with avg marks bonus) |
| Recency | 25% | How recently the topic appeared (last seen year) |
| Time Pressure | 20% | Modifier: 1.0x (relaxed) → 1.5x (urgent) based on days remaining |

All computations are deterministic — the same inputs always produce the same scores.

---

## License

MIT
