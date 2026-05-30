# 🚀 Priora — Full-Stack Production Deployment Guide

> **Last updated:** May 2026  
> **Stack:** Nuxt 4 SSR (frontend) → Rust/Axum (backend API) → PostgreSQL (database)  
> **Best free platforms:** Cloudflare Pages (frontend) + Render (backend) + Neon (database)  

---

## Table of Contents

1. [Architecture Overview](#1-architecture-overview)
2. [Prerequisites & Accounts](#2-prerequisites--accounts)
3. [Database — Neon (PostgreSQL Free Tier)](#3-database--neon-postgresql-free-tier)
4. [Backend — Render (Rust/Axum Docker)](#4-backend--render-rustaxum-docker)
5. [Frontend — Cloudflare Pages (Nuxt 4 Free Tier)](#5-frontend--cloudflare-pages-nuxt-4-free-tier)
6. [Environment Variables Reference](#6-environment-variables-reference)
7. [Post-Deployment Checklist](#7-post-deployment-checklist)
8. [Troubleshooting](#8-troubleshooting)
9. [Updating Your Deployment](#9-updating-your-deployment)
10. [Production Readiness Checklist](#10-production-readiness-checklist)

---

## 1. Architecture Overview

### How Everything Connects

```
┌──────────────────────────────────────────────────────────────────┐
│                          USER'S BROWSER                          │
│  HTTPS://PRIORA.PAGES.DEV                                        │
└───────────────────┬──────────────────────────────────────────────┘
                    │
                    ▼
┌──────────────────────────────────────────────────────────────────┐
│              CLOUDFLARE PAGES (Free Tier)                         │
│                                                                   │
│  ┌─────────────────────────────────────────────────────────┐     │
│  │  Nuxt 4 SSR Frontend                                    │     │
│  │                                                         │     │
│  │  • HTML/CSS/JS rendered at edge                         │     │
│  │  • Unlimited bandwidth                                  │     │
│  │  • Global CDN (330+ cities)                             │     │
│  │  • DDoS protection                                      │     │
│  │  • No credit card needed                                │     │
│  └──────────────────┬──────────────────────────────────────┘     │
│                     │                                            │
│  /api/* requests ───┤  Proxied via Nuxt Nitro server routes      │
│                     │  (server/api/[...].ts)                     │
└─────────────────────┼────────────────────────────────────────────┘
                      │
                      ▼
┌──────────────────────────────────────────────────────────────────┐
│              RENDER (Free Tier — 0.1 CPU / 512MB RAM)             │
│                                                                   │
│  ┌─────────────────────────────────────────────────────────┐     │
│  │  Rust/Axum Backend API (Docker container)                │     │
│  │                                                         │     │
│  │  • /api/health           → Health check                 │     │
│  │  • /api/schemes          → List schemes                 │     │
│  │  • /api/schemes/:id/branches → Branches for scheme     │     │
│  │  • /api/branches/:id/semesters → Semesters for branch  │     │
│  │  • /api/semesters/:id/subjects → Subjects for semester │     │
│  │  • /api/subjects/:id    → Subject detail + modules     │     │
│  │  • /api/subjects/:id/analyze → POST: Run analysis      │     │
│  │  • /api/analyses/:id    → Get analysis results         │     │
│  │  • /api/admin/*         → Admin CRUD operations        │     │
│  │  • /api/feedback        → POST: Submit feedback        │     │
│  │                                                         │     │
│  │  Security:                                               │     │
│  │  • CORS middleware (configurable origin)                 │     │
│  │  • Security headers (CSP, HSTS, X-Frame-Options, etc.)  │     │
│  │  • Request body size limit (configurable, default 1MB)  │     │
│  │  • Request timeout (configurable, default 30s)          │     │
│  │  • Sensitive headers filtered from logs                 │     │
│  │  • Input validation on all admin endpoints              │     │
│  │  • Graceful shutdown on SIGTERM/SIGINT                  │     │
│  │  • Distroless Docker image (no shell, non-root user)   │     │
│  └──────────────────┬──────────────────────────────────────┘     │
│                     │                                            │
└─────────────────────┼────────────────────────────────────────────┘
                      │
                      ▼
┌──────────────────────────────────────────────────────────────────┐
│              NEON (Free Tier — 0.5GB DB, 100 compute hrs/mo)     │
│                                                                   │
│  ┌─────────────────────────────────────────────────────────┐     │
│  │  PostgreSQL 16/17 Serverless                            │     │
│  │                                                         │     │
│  │  • Connection pooling via PgBouncer                     │     │
│  │  • 100 compute hours/month (free)                       │     │
│  │  • Auto-suspends after 5 min idle (resumes on connect)  │     │
│  │  • Point-in-time restore with branching                 │     │
│  │  • Built-in query monitoring                            │     │
│  │  • No credit card needed                                │     │
│  └─────────────────────────────────────────────────────────┘     │
└──────────────────────────────────────────────────────────────────┘
```

### Data Flow

1. **User visits** `https://priora.pages.dev` → Cloudflare serves Nuxt SSR HTML
2. **User selects** a scheme/branch/semester/subject → Frontend calls `/api/...`  
3. **Nuxt Nitro proxy** (`server/api/[...].ts`) forwards to Render backend
4. **Rust/Axum** handles the request, queries Neon PostgreSQL
5. **Backend returns** JSON → Nitro proxy passes it to the browser
6. **Analysis requests** POST to `/api/subjects/:id/analyze` → Backend computes priority scores → Returns structured analysis with high/medium/low buckets

### Free Tier Limits at a Glance

| Service | Compute | Storage | Bandwidth | Auto-Sleep? | Card Needed? |
|---------|---------|---------|-----------|-------------|--------------|
| **Cloudflare Pages** | 500 builds/mo | Unlimited | Unlimited | No (always on) | ❌ No |
| **Render** | 0.1 CPU / 512MB RAM | 1GB | 100 GB/mo | Yes (15 min idle) | ✅ Yes |
| **Neon** | 100 hrs/mo | 0.5GB | 5GB/mo | Yes (5 min idle) | ❌ No |

> ⚠️ **Render requires a credit card** for identity verification. You will not be charged as long as you stay within free tier limits. Cloudflare Pages and Neon do not require a card.

---

## 2. Prerequisites & Accounts

### Services to Sign Up For

| Service | URL | Purpose | Card Needed? |
|---------|-----|---------|:------------:|
| [GitHub](https://github.com/signup) | Free | Code hosting + deployment triggers | ❌ |
| [Neon](https://console.neon.tech/sign-up) | Free | PostgreSQL database | ❌ |
| [Render](https://dashboard.render.com/register) | Free (card required) | Backend API hosting | ✅ |
| [Cloudflare](https://dash.cloudflare.com/sign-up) | Free | Frontend hosting + CDN | ❌ |

### Tools to Install

```bash
# ── 1. Git ──────────────────────────────────────────────────────────
git --version   # Should be 2.30+

# ── 2. Node.js + pnpm ────────────────────────────────────────────────
node --version  # Should be v20+
pnpm --version  # Should be 9+
# If pnpm missing: npm install -g pnpm

# ── 3. Rust (for local verification + seeding) ───────────────────────
rustup --version  # Should show latest
# Install: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# ── 4. sqlx-cli (for database migrations) ───────────────────────────
cargo install sqlx-cli --features rustls,postgres
```

---

## 3. Database — Neon (PostgreSQL Free Tier)

### Step 3.1: Create Project

1. Go to [console.neon.tech](https://console.neon.tech) and sign up (or log in)
2. Click **"Create a project"**
3. Name: `priora-db`
4. Region: Pick the closest to you (e.g., `US East (N. Virginia)` or `EU West (Frankfurt)`)
5. PostgreSQL version: **16** (or 17)
6. Neon Auth: **Leave OFF**
7. Click **"Create project"**

### Step 3.2: Get Connection String

1. On the project dashboard, click **"Connect"**
2. Copy the **connection string** — it looks like:
   ```
   postgresql://alex:AbC123...@ep-dark-forest-123456.us-east-2.aws.neon.tech/neondb
   ```
3. ⚠️ **Save this securely** — you'll need it for the backend. Never commit it to git.

### Step 3.3: Configure Backend `.env`

Create or edit `backend/.env`:

```bash
DATABASE_URL="postgresql://alex:AbC123...@ep-dark-forest-123456.us-east-2.aws.neon.tech/neondb"
```

> 🔒 **Security:** `backend/.env` is already in `.gitignore` — it will never be committed.

### Step 3.4: Run Migrations

```bash
cd /home/joel/Projects/Priora/backend

# Run the initial migration against Neon
source .env && sqlx migrate run

# Verify all 13 tables were created
source .env && sqlx migrate info
```

Expected output: The migration `20260526000001_initial_schema.sql` is applied. This creates all tables:
`users`, `schemes`, `branches`, `semesters`, `subjects`, `modules`, `topics`, `question_papers`, `questions`, `question_topic_map`, `topic_stats`, `analyses`, `analysis_feedback`

### Step 3.5: Seed Initial Data

The backend auto-seeds data on startup. To seed locally:

```bash
cd /home/joel/Projects/Priora/backend

# Start the backend — it will run migrations (idempotent) and seed data
source .env && cargo run --release
```

The seed creates:
- **1 scheme** (2024 Scheme)
- **1 branch** (CSE)
- **8 semesters** (S1–S8)
- **32 subjects** (6 in S1, 5 in S2, 5 in S3, 4 in S4, 4 in S5, 4 in S6, 3 in S7, 1 in S8)
- **128 modules** (4 per subject)
- **512 topics** (4 per module)
- **62 question papers** with **620 questions** and **744 question-topic mappings**
- **512 topic_stats rows** with computed priority scores

Once the server starts (you'll see `"Priora API starting on 0.0.0.0:3001"`), verify in another terminal:

```bash
curl http://localhost:3001/api/health
# Expected: {"status":"ok","service":"priora-api","timestamp":"..."}

curl http://localhost:3001/api/schemes
# Expected: JSON array with 1 scheme (2024 Scheme)

curl http://localhost:3001/api/stats
# Expected: {"subjects":32,"topics":512,"papers":62,"schemes":1}
```

Stop the server with `Ctrl+C` once verified.

---

## 4. Backend — Render (Rust/Axum Docker)

### Step 4.1: Prepare Repository

Before deploying to Render, make sure your repository is pushed to GitHub:

```bash
cd /home/joel/Projects/Priora

# Verify .gitignore is correct (excludes node_modules, target, .nuxt, .env, etc.)
cat .gitignore

# Check the git status
git status

# Commit and push
git add .
git commit -m "feat: production-ready backend with Docker+Render support"
git push origin main
```

> ✅ The project already has `backend/Dockerfile` — a multi-stage build producing a ~15MB distroless image.

### Step 4.2: Handle SQLx Offline Build Data (CRITICAL)

SQLx checks your database queries **at compile time** by default. On Render, there's no database available during the Docker build, so you need to provide offline query data.

**You have two options:**

#### Option A: Generate offline data (recommended)

```bash
cd /home/joel/Projects/Priora/backend

# Requires a running PostgreSQL with the schema applied
source .env && cargo sqlx prepare

# This creates: backend/.sqlx/ directory with query metadata
# COMMIT THIS TO GIT:
git add backend/.sqlx/
git commit -m "Add SQLx offline query data for Docker build"
git push origin main
```

#### Option B: Skip verification (fallback)

If you can't run `cargo sqlx prepare`, set this env var on Render instead:

| Variable | Value | Scope |
|----------|-------|-------|
| `SQLX_OFFLINE` | `true` | **Build** environment (not runtime) |

> ⚠️ Option B skips all compile-time query safety checks — the build will succeed even if a query is malformed.

### Step 4.3: Create Render Web Service

1. Go to [dashboard.render.com](https://dashboard.render.com) and sign up with GitHub
2. Click **"New +"** → **"Web Service"**
3. **Connect your GitHub account** and select the `priora` repository
4. Configure the service:

| Setting | Value |
|---------|-------|
| **Name** | `priora-api` |
| **Region** | Pick closest to you (e.g., `Frankfurt` or `Oregon`) |
| **Branch** | `main` |
| **Root Directory** | `backend` |
| **Runtime** | **Docker** |
| **Instance Type** | **Free** |

5. Click **"Create Web Service"**

> **Why Docker?** The backend's `Dockerfile` is located at `backend/Dockerfile`. Setting the root directory to `backend` tells Render where to find it. Render will automatically detect and build from it.

### Step 4.4: How the Dockerfile Works

The backend's `Dockerfile` uses a multi-stage build:

| Stage | Base Image | What Happens |
|-------|-----------|--------------|
| **builder** | `rust:1.85-slim-bookworm` | Compiles Rust binary with `pkg-config` + `libssl-dev` for SQLx |
| **runtime** | `gcr.io/distroless/cc-debian12` | ~15MB final image — no shell, no package manager, non-root user |

> **Security:** Distroless images remove shells, package managers, and setuid binaries — significantly reducing attack surface. Runs as UID 65532 (non-root).

The app reads the `PORT` env var (or `SERVER_PORT`) to determine which port to bind. Render automatically sets `PORT=10000` for free tier services.

### Step 4.5: Set Environment Variables

On your Render dashboard → `priora-api` → **Environment** → **Add Environment Variable**:

#### Required — must be set:

| Variable | Value | Purpose |
|----------|-------|---------|
| `DATABASE_URL` | `postgresql://alex:...@ep-...` | Your Neon connection string |
| `RUST_LOG` | `info` | Log level |

#### Optional — but recommended:

| Variable | Value | Purpose | Default |
|----------|-------|---------|---------|
| `CORS_ORIGIN` | `https://priora.pages.dev` | Allowed CORS origin (set after frontend deploys) | `*` (any origin) |
| `MAX_BODY_SIZE` | `1048576` | Max request body in bytes | 1MB |
| `REQUEST_TIMEOUT_SECS` | `30` | Request timeout | 30s |
| `SERVER_HOST` | `0.0.0.0` | Bind address | `0.0.0.0` |

> ⚠️ **Do NOT set `PORT`** — Render sets this automatically to `10000`. The app supports both `PORT` and `SERVER_PORT` env vars.

> 🔒 **Secrets note:** `DATABASE_URL` contains your password. Set it in Render's dashboard — never commit it to git.

#### If using Option B (no SQLx offline data):

Set this as a **Build** environment variable (not runtime):

| Variable | Value | Scope |
|----------|-------|-------|
| `SQLX_OFFLINE` | `true` | Build |

To set a Build env var: In Render dashboard → `priora-api` → **Environment** → **Add Environment Variable** → check **"Build environment variable"** checkbox.

### Step 4.6: Deploy

Render auto-deploys when you push to the connected branch. To trigger the first deployment:

```bash
git push origin main
```

Or click **"Manual Deploy"** → **"Deploy latest commit"** in the Render dashboard.

**First build takes 3–5 minutes** (Rust compilation + Docker image build).

### Step 4.7: Verify Backend

Once the deployment status shows **"Live"**:

```bash
# Health check
curl https://priora-api.onrender.com/api/health
# Expected: {"status":"ok","service":"priora-api","timestamp":"..."}

# List schemes
curl https://priora-api.onrender.com/api/schemes
# Expected: JSON array with 1 scheme (2024 Scheme)

# List branches
curl https://priora-api.onrender.com/api/schemes/<scheme-id>/branches
# Expected: JSON array with 1 branch (CSE)
```

> 🧊 **Cold start:** The first request after 15 minutes of inactivity may take 3–5 seconds (Render free tier spins down). Subsequent requests are fast (<200ms).

### Step 4.8: Backend Production Hardening (Already Applied ✅)

The following are already implemented in the codebase:

| Hardening | Implementation |
|-----------|---------------|
| ✅ **CORS middleware** | Configurable origin via `CORS_ORIGIN` env var |
| ✅ **Security headers** | `X-Content-Type-Options`, `X-Frame-Options: DENY`, `CSP`, `Referrer-Policy` |
| ✅ **Request body limit** | Configurable via `MAX_BODY_SIZE` (default 1MB) |
| ✅ **Request timeout** | Configurable via `REQUEST_TIMEOUT_SECS` (default 30s) |
| ✅ **Sensitive header filtering** | `authorization`, `cookie`, `x-api-key` not logged |
| ✅ **Input validation** | All admin endpoints validate strings, enums, ranges |
| ✅ **Graceful shutdown** | Handles SIGTERM/SIGINT — drains connections before exit |
| ✅ **Distroless Docker image** | ~15MB, no shell, no package manager, non-root user |
| ✅ **Connection pooling** | `max_connections: 20`, configurable |
| ✅ **Health check endpoint** | `/api/health` returns JSON with status + timestamp |
| ✅ **PORT env var support** | Falls back to `PORT` if `SERVER_PORT` is not set (Render compat) |

### Rate Limiting

Render's free tier does not include a built-in rate limiter. For production, consider these lightweight approaches:

1. **Cloudflare WAF** (if you proxy through Cloudflare) — Free: 10 rate limiting rules, 1M requests/mo
2. **Add `tower` middleware** to the Rust backend — We can add a simple in-memory rate limiter if needed
3. **Accept the trade-off** — For an MVP with low traffic, this is acceptable

---

## 5. Frontend — Cloudflare Pages (Nuxt 4 Free Tier)

### Step 5.1: Connect Repository

1. Go to [dash.cloudflare.com](https://dash.cloudflare.com)
2. Navigate to **Workers & Pages** → **Pages**
3. Click **"Connect to Git"**
4. Authorize GitHub if prompted
5. Select your `priora` repository

### Step 5.2: Configure Build

| Setting | Value |
|---------|-------|
| **Root directory** | `frontend` ⚠️ **Important** — the Nuxt app lives in `/frontend` |
| **Framework preset** | `Nuxt.js` |
| **Build command** | `pnpm install && pnpm build` |
| **Build output directory** | `.output/public` |
| **Node.js version** | `22` |

> **Why `.output/public`?** Nuxt 4 with Nitro generates production output in `.output/public/` when using the `cloudflare_pages` preset.

### Step 5.3: Set Environment Variables

In Cloudflare Pages → your project → **Settings** → **Environment variables** → **Production**:

| Variable | Value | Purpose |
|----------|-------|---------|
| `NODE_VERSION` | `22` | Ensures the right Node version |
| `NITRO_PRESET` | `cloudflare_pages` | Tells Nuxt's Nitro engine to build for Cloudflare edge runtime |
| `NUXT_API_BASE` | `https://priora-api.onrender.com/api` | Backend URL (used by Nuxt SSR server routes) |
| `NUXT_PUBLIC_API_BASE` | `/api` | Client-side API path (proxied by Nitro) |
| `NUXT_PUBLIC_SITE_URL` | `https://priora.pages.dev` | For SEO meta tags + canonical URLs |

> 🧠 **`NITRO_PRESET` scope:** Set this as a **Production**-only variable. Leave it unset for **Preview** deployments (so branch builds use the default Node.js preset for testing).

> 🧠 **API proxy strategy:** The frontend uses `server/api/[...].ts` to proxy `/api/*` requests to the backend. On Cloudflare, Nitro handles this proxy internally — no extra Workers needed. The browser calls `/api/...` and Nitro forwards to `https://priora-api.onrender.com/api/...`.

### Step 5.4: Deploy

1. Click **"Save and Deploy"**
2. First build takes 1–2 minutes
3. Cloudflare provides a `*.pages.dev` URL — e.g., `https://priora-abc123.pages.dev`

### Step 5.5: Custom Domain (Optional)

1. In Cloudflare Pages → your project → **Custom domains**
2. Add your domain (e.g., `priora.yourdomain.com`)
3. Follow Cloudflare's DNS setup instructions

### Step 5.6: Verify CORS

Since the frontend (Cloudflare Pages) and backend (Render) are on **different domains**, CORS must be configured on the **Rust backend**.

Set the `CORS_ORIGIN` env var on Render to match your frontend URL:

```bash
# In Render dashboard → Environment → CORS_ORIGIN
CORS_ORIGIN=https://priora.pages.dev
```

If using a custom domain, set it to that:

```bash
CORS_ORIGIN=https://priora.yourdomain.com
```

**Verify CORS works:**

```bash
curl -X OPTIONS https://priora-api.onrender.com/api/schemes \
  -H "Origin: https://priora.pages.dev" \
  -H "Access-Control-Request-Method: GET" \
  -I
# Expected: 204 No Content with Access-Control-Allow-Origin header
```

> **Troubleshooting CORS:** If you see CORS errors in the browser console:
> 1. Verify `CORS_ORIGIN` on Render exactly matches your frontend URL (no trailing slash)
> 2. Check that the backend is running: `curl https://priora-api.onrender.com/api/health`
> 3. Check if Neon compute is paused (it auto-suspends after 5 min idle)

---

## 6. Environment Variables Reference

### Frontend (`frontend/.env` — local dev)

| Variable | Required | Default | Runtime | Description |
|----------|----------|---------|---------|-------------|
| `NUXT_API_BASE` | Yes | `http://127.0.0.1:3001/api` | SSR | Backend API URL for server-side requests |
| `NUXT_PUBLIC_API_BASE` | No | `/api` | Client | Public API path (client-side `$fetch`) |
| `NUXT_PUBLIC_SITE_URL` | No | `https://priora.ktu` | Both | Canonical site URL for SEO |

### Frontend (Cloudflare Pages — production)

| Variable | Required | Default | Scope | Description |
|----------|----------|---------|-------|-------------|
| `NODE_VERSION` | Yes | — | Build | Sets Node.js version to `22` |
| `NITRO_PRESET` | Yes | — | **Production** Build | Sets Nitro to Cloudflare Pages preset |
| `NUXT_API_BASE` | Yes | — | Build | Backend URL: `https://priora-api.onrender.com/api` |
| `NUXT_PUBLIC_API_BASE` | No | `/api` | Build | Public API path |
| `NUXT_PUBLIC_SITE_URL` | No | — | Build | Canonical site URL |

### Backend (`backend/.env` — local dev / Render env vars)

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `DATABASE_URL` | **✅ Yes** | — | PostgreSQL connection string from Neon |
| `PORT` | No | `10000` | Render's dynamic port (set automatically) |
| `SERVER_PORT` | No | `3001` (local) / `10000` (Render via PORT fallback) | Port to listen on |
| `SERVER_HOST` | No | `0.0.0.0` | Bind address |
| `CORS_ORIGIN` | No | `*` | Allowed CORS origin. **Restrict in production.** |
| `MAX_BODY_SIZE` | No | `1048576` (1MB) | Maximum request body size in bytes |
| `REQUEST_TIMEOUT_SECS` | No | `30` | Request timeout in seconds |
| `RUST_LOG` | No | `info` | Log level: `info`, `debug`, `warn`, `error` |
| `SQLX_OFFLINE` | No | (unset) | Set to `true` to skip compile-time query verification |

---

## 7. Post-Deployment Checklist

### Step 7.1: Smoke Test All Routes

```bash
# ── Frontend (Cloudflare Pages) ────────────────────────────────────
FRONTEND_URL="https://priora.pages.dev"  # ← Replace with your URL

# Homepage
curl -s -o /dev/null -w '%{http_code}' "$FRONTEND_URL/"
# Expected: 200

# Subjects browse page
curl -s -o /dev/null -w '%{http_code}' "$FRONTEND_URL/subjects"
# Expected: 200

# Admin dashboard
curl -s -o /dev/null -w '%{http_code}' "$FRONTEND_URL/admin"
# Expected: 200

# robots.txt
curl -s "$FRONTEND_URL/robots.txt"
# Expected: text content with "Allow: /"

# ── Backend (Render) ───────────────────────────────────────────────
BACKEND_URL="https://priora-api.onrender.com"  # ← Replace with your URL

# Health check
curl -s "$BACKEND_URL/api/health" | python3 -m json.tool
# Expected: {"status": "ok", "service": "priora-api", ...}

# Schemes (should return 1 scheme)
curl -s "$BACKEND_URL/api/schemes" | python3 -m json.tool
# Expected: JSON array with 2024 Scheme

# Full traversal of the subject hierarchy
SCHEME_ID=$(curl -s "$BACKEND_URL/api/schemes" | \
  python3 -c "import sys,json; print(json.load(sys.stdin)[0]['id'])" 2>/dev/null)
echo "Scheme: $SCHEME_ID"

BRANCH_ID=$(curl -s "$BACKEND_URL/api/schemes/$SCHEME_ID/branches" | \
  python3 -c "import sys,json; d=json.load(sys.stdin); print(d[0]['id'])" 2>/dev/null)
echo "Branch: $BRANCH_ID"

SEMESTER_ID=$(curl -s "$BACKEND_URL/api/branches/$BRANCH_ID/semesters" | \
  python3 -c "import sys,json; d=json.load(sys.stdin); print(d[0]['id'])" 2>/dev/null)
echo "Semester: $SEMESTER_ID"

SUBJECT_ID=$(curl -s "$BACKEND_URL/api/semesters/$SEMESTER_ID/subjects" | \
  python3 -c "import sys,json; d=json.load(sys.stdin); print(d[0]['id'])" 2>/dev/null)
echo "Subject: $SUBJECT_ID"

# Get full subject detail
curl -s "$BACKEND_URL/api/subjects/$SUBJECT_ID" | python3 -m json.tool
# Expected: Full subject detail with 4 modules, 4 topics each

# Run analysis on a subject
curl -s -X POST "$BACKEND_URL/api/subjects/$SUBJECT_ID/analyze" \
  -H "Content-Type: application/json" \
  -d "{\"subject_id\":\"$SUBJECT_ID\",\"days_remaining\":30}" | python3 -m json.tool
# Expected: Analysis with priority buckets, high/medium/low topics

# 404 handling
curl -s -o /dev/null -w '%{http_code}' "$BACKEND_URL/api/subjects/00000000-0000-0000-0000-000000000000"
# Expected: 404

# Invalid input
curl -s -X POST "$BACKEND_URL/api/subjects/$SUBJECT_ID/analyze" \
  -H "Content-Type: application/json" \
  -d "{\"subject_id\":\"$SUBJECT_ID\",\"days_remaining\":0}"
# Expected: 400 Bad Request
```

### Step 7.2: Browser Checks

Open your frontend URL and verify:

| Test | Expected |
|------|----------|
| Homepage loads with intro animation | ✅ |
| Stats banner at bottom shows real numbers (32 subjects, 512 topics, etc.) | ✅ |
| Navigate to `/subjects` — dropdowns populate | ✅ |
| Select scheme → branch → semester → subject | ✅ |
| Enter 30 days → click "Analyze My Subjects" | ✅ |
| Analysis results show High/Medium/Low buckets with priority scores | ✅ |
| Navigate to `/admin` — dashboard loads with stat cards | ✅ |
| Theme toggle switches between dark/light | ✅ |
| DevTools Console — no errors, no warnings | ✅ |
| DevTools Network — all API calls return 200 | ✅ |

### Step 7.3: SEO & Performance

```bash
# Check meta tags
curl -s "$FRONTEND_URL" | grep -oP '<title>[^<]+</title>'
# Expected: Priora — Study Smarter, Not Harder

# Check robots.txt
curl -s "$FRONTEND_URL/robots.txt"
# Expected: Allow: /
```

### Step 7.4: Cold Start Test

```bash
# Wait 15 minutes (Render idle timeout), then:
time curl -s -o /dev/null -w 'HTTP %{http_code} | %{time_total}s\n' "$BACKEND_URL/api/health"
# Expected: 200, but may take 3-5 seconds for cold start
```

---

## 8. Troubleshooting

### 🟥 Frontend: 502 Bad Gateway

**Cause:** Backend is unreachable, crashed, or building.

**Fix:**
```bash
# Check Render dashboard → priora-api → "Logs"
# Look for startup errors or crashes

# If it's a first deploy: wait for the build to complete (check "Events" tab)

# Restart: Dashboard → priora-api → "Manual Deploy" → "Clear build cache & deploy"
```

### 🟥 Frontend: Pages Load but API Calls Fail

**Cause:** CORS misconfiguration or wrong API URL.

**Fix:**
1. Open browser DevTools → Console — look for CORS errors
2. Verify `NUXT_API_BASE` in Cloudflare Pages env vars
3. Test API directly: `curl https://priora-api.onrender.com/api/schemes`
4. Check CORS: `curl -I -X OPTIONS -H "Origin: https://priora.pages.dev" https://priora-api.onrender.com/api/schemes`

### 🟥 Backend: Cannot Connect to Database

**Cause:** Wrong `DATABASE_URL` or Neon compute is paused.

**Fix:**
```bash
# Check Render logs for database errors
# Dashboard → priora-api → Logs → search for "database" or "connect"

# Verify DATABASE_URL is set correctly in Render environment

# In Neon dashboard: check if compute is active
# Click "Resume" if paused
```

### 🟥 Backend: Docker Build Fails

**Cause:** Missing SQLx offline data or compile error.

**Fix:**
```bash
# ── Check Render build logs ────────────────────────────────────────
# Dashboard → priora-api → Events → Click the failed deploy → "Build" tab

# ── If the error is SQLx related ─────────────────────────────────────
# Set SQLX_OFFLINE=true as a Build environment variable

# ── If the error is a Rust compile error ─────────────────────────────
# Fix the code locally, push, and redeploy
```

### 🟥 Backend: Service Crashes on Startup

**Cause:** Port binding issue or missing env var.

**Fix:**
```bash
# The app reads PORT env var (Render's default) with fallback to SERVER_PORT
# Render sets PORT=10000 automatically — don't override it

# Check logs for:
# - "Failed to connect to database" → DATABASE_URL issue
# - "Failed to bind to port" → Port conflict
```

### 🟥 Database or Backend Cold Start Lag

**Cause:** Both Render and Neon free tiers spin down after inactivity.

**Symptoms:** First request after 15+ minutes takes 3–5 seconds.

**Mitigation options:**
1. **Accept it** — Cold starts are normal for free tier. Users may experience a slight delay on the first request after inactivity.
2. **Keep warm** — Create a cron job (e.g., [cron-job.org](https://cron-job.org) free tier) that pings the health endpoint every 10 minutes:
   ```bash
   # URL to ping: https://priora-api.onrender.com/api/health
   ```
3. **Upgrade Render** — Paid tier ($7/mo) has no spin-down
4. **Upgrade Neon** — Paid tier ($19/mo) has always-on compute

### 🟥 Build Fails on Cloudflare Pages

**Cause:** Node version mismatch or missing build dependencies.

**Fix:**
```bash
# Set these in Cloudflare Pages env vars (Production only):
#   NODE_VERSION = 22
#   NITRO_PRESET = cloudflare_pages

# Also verify:
#   Root directory is set to "frontend"
#   Build output directory is ".output/public"

# Try: Manual Deploy → "Clear cache and retry"
```

---

## 9. Updating Your Deployment

### Update Backend

```bash
cd /home/joel/Projects/Priora

# Make code changes, commit, and push:
git add .
git commit -m "Fix: improve analysis accuracy"
git push origin main

# Render auto-deploys on push to main branch
# Check: Dashboard → priora-api → "Events" for deployment progress
```

### Update Frontend

```bash
cd /home/joel/Projects/Priora/frontend

# Make code changes, commit, and push:
git add .
git commit -m "Feat: improve UI animations"
git push origin main

# Cloudflare Pages auto-deploys on push to main branch
# Check: Cloudflare Pages dashboard → Deployments
```

### Update Database Schema

```bash
cd /home/joel/Projects/Priora/backend

# 1. Create a new migration
sqlx migrate add add_exam_boards_table

# 2. Edit the generated SQL file
# (backend/migrations/<timestamp>_add_exam_boards_table.sql)

# 3. Apply the migration locally first
source .env && sqlx migrate run

# 4. Generate updated SQLx offline data
source .env && cargo sqlx prepare

# 5. Commit and redeploy
git add backend/.sqlx/ backend/migrations/
git commit -m "Add exam boards table"
git push origin main
```

### Rollback

**Cloudflare Pages:** Dashboard → Deployments → Click "..." on previous deploy → **"Rollback to this deployment"**

**Render:** Dashboard → `priora-api` → **"Manual Deploy"** → **"Deploy previous commit"** → Select the previous commit

**Neon:** Use the **Branching** feature to create a point-in-time restore before schema changes.

---

## 10. Production Readiness Checklist

### ✅ Security

| Check | Status | Notes |
|-------|--------|-------|
| Backend CORS restricted to frontend domain | ✅ Configured via `CORS_ORIGIN` env var | Set to `https://priora.pages.dev` on Render |
| Security headers on all responses | ✅ CSP, X-Frame-Options, X-Content-Type-Options, etc. | Implemented in `backend/src/main.rs` |
| Request body size limited | ✅ Default 1MB, configurable via `MAX_BODY_SIZE` | Prevents large payload attacks |
| Request timeout enforced | ✅ Default 30s, configurable via `REQUEST_TIMEOUT_SECS` | Prevents slow-loris attacks |
| Sensitive headers filtered from logs | ✅ `authorization`, `cookie`, `x-api-key` not logged | Prevents credential leakage |
| Input validation on API endpoints | ✅ All admin handlers validate strings, enums, ranges | Prevents injection/garbage data |
| Graceful shutdown handling | ✅ SIGTERM + SIGINT handled | Drains connections before exit |
| Docker distroless base image | ✅ `gcr.io/distroless/cc-debian12` | ~15MB, no shell, no package manager |
| Database connection pooling | ✅ `max_connections: 20` | Prevents connection exhaustion |
| Check constraints in DB schema | ✅ `days_remaining` (1–365), `rating` (1–5), `marks` (>0), `confidence` (0–1) | Database-level safety net |
| `.env` files in `.gitignore` | ✅ Never committed to git | Prevents credential leaks |

### ✅ Performance

| Check | Status | Notes |
|-------|--------|-------|
| Database indexes on all FK columns | ✅ 12 indexes created in migration | Fast JOINs across all entity relationships |
| Connection pool size configured | ✅ 20 connections | Matches Neon free tier limits |
| CDN caching for static assets | ✅ Cloudflare CDN (included with Pages) | 330+ edge locations worldwide |

### ✅ Reliability

| Check | Status | Notes |
|-------|--------|-------|
| Health check endpoint | ✅ `/api/health` returns JSON | Used by Render health monitoring |
| Graceful shutdown | ✅ Drains connections before exit | Prevents in-flight request failures |
| Database migrations on startup | ✅ `sqlx::migrate!()` runs at boot | No manual migration step needed |
| Fallback for missing env vars | ✅ All vars have sensible defaults | Won't crash on misconfiguration |
| PORT env var fallback | ✅ Reads `PORT` (Render) then `SERVER_PORT` | Platform-compatible |

### ✅ API Quality

| Check | Status | Notes |
|-------|--------|-------|
| Consistent JSON error responses | ✅ `ApiError` enum → `{ "error": "message" }` | Parseable by frontend |
| Proper HTTP status codes | ✅ 200, 400, 404, 500 | Follows REST conventions |
| CORS preflight (OPTIONS) support | ✅ Via `tower-http` CorsLayer | Browser preflight requests work |
| Input validation with clear error messages | ✅ All endpoints validate and return descriptive errors | Easy debugging |
| Health check with service info | ✅ Returns service name + timestamp | Useful for monitoring |

---

## Appendix: File Map

| File | Purpose |
|------|---------|
| `backend/Dockerfile` | Multi-stage Docker build for Rust/Axum (distroless runtime) |
| `backend/Cargo.toml` | Rust dependencies (Axum, SQLx with rustls TLS, tokio, tower-http) |
| `backend/src/main.rs` | Server entry point: routes, CORS, security, graceful shutdown |
| `backend/src/config.rs` | Config from environment variables (reads `PORT`/`SERVER_PORT`) |
| `backend/src/error.rs` | Unified error handling |
| `backend/src/handlers/` | API route handlers (subjects, analyze, admin, feedback) |
| `backend/src/services/analysis_engine.rs` | Priority scoring algorithm |
| `backend/src/models/` | Data models and request/response types |
| `backend/src/db/seed.rs` | Initial data seeding (32 subjects, 512 topics, 62 question papers) |
| `backend/migrations/` | SQL schema migration (13 tables) |
| `backend/tests/integration_test.rs` | 35+ integration tests |
| `frontend/nuxt.config.ts` | Nuxt 4 configuration |
| `frontend/server/api/[...].ts` | API proxy route (forwards to Render backend) |
| `frontend/server/routes/stats.get.ts` | Stats aggregator endpoint |
| `frontend/.env.example` | Frontend environment variable template |
| `frontend/public/robots.txt` | Crawler rules |
| `frontend/error.vue` | Custom error page (404, 500) |
| `docs/prd.md` | Product requirements document |
| `DEPLOY.md` | This deployment guide |

---

*Deployment guide v3.0 — Priora Team (Updated for Render)*
