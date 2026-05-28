# ═══ Priora Backend — Dockerfile ═══════════════════════════════════════
# Multi-stage build for production deployment on Fly.io / Railway / any container runtime
# ──────────────────────────────────────────────────────────────────────────
# Base:   rust:1.85-slim-bookworm  (build stage, ~1.5GB)
# Runner: gcr.io/distroless/cc      (runtime, ~15MB, no shell)
# ──────────────────────────────────────────────────────────────────────────

# ── Stage 1: Builder ──────────────────────────────────────────────────
FROM rust:1.85-slim-bookworm AS builder

# Install build dependencies (pkg-config + libssl for sqlx compile-time checks)
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      pkg-config \
      libssl-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs to cache dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs && \
    mkdir -p migrations && echo "-- dummy" > migrations/20260526000001_initial_schema.sql && \
    cargo build --release 2>/dev/null || true

# Copy the real source code
COPY src/ src/
COPY migrations/ migrations/

# ═══ SQLx Offline Mode ═══
# If you haven't run `cargo sqlx prepare`, set SQLX_OFFLINE=true
# so the build does NOT need a running PostgreSQL database.
ARG SQLX_OFFLINE=true
ENV SQLX_OFFLINE=${SQLX_OFFLINE}

# Build the release binary
RUN cargo build --release

# ── Stage 2: Runtime ──────────────────────────────────────────────────
# Distroless CC: contains glibc + libssl but no shell, no package manager
FROM gcr.io/distroless/cc-debian12:latest AS runtime

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/priora-api /app/priora-api

# Copy migrations (needed at runtime for sqlx::migrate!())
COPY --from=builder /app/migrations/ ./migrations/

# Create non-root user at runtime (distroless has no adduser, so we do it differently)
# Distroless images run as non-root by default (UID 65532)
USER 65532:65532

# NOTE: Fly.io uses its own [http_service] health checks defined in fly.toml.
# Dockerfile HEALTHCHECK is not used by Fly.io and would fail here since
# the binary can't bind to the port twice. Fly.io checks /api/health via HTTP.

EXPOSE 8080

ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080

ENTRYPOINT ["/app/priora-api"]
