# Multi-stage API image — api-http + fullsales-migrate
# Build: docker build -f backend/Dockerfile -t fullsales-api:local backend

FROM rust:bookworm AS builder
WORKDIR /app

RUN apt-get update \
  && apt-get install -y --no-install-recommends pkg-config libssl-dev \
  && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates ./crates
COPY migrations ./migrations

RUN cargo build --release -p api-http \
  && cargo build --release -p infra-postgres --bin fullsales-migrate \
  && strip target/release/api-http target/release/fullsales-migrate

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates curl \
  && rm -rf /var/lib/apt/lists/* \
  && useradd --system --uid 10001 --create-home appuser

COPY --from=builder /app/target/release/api-http /usr/local/bin/api-http
COPY --from=builder /app/target/release/fullsales-migrate /usr/local/bin/fullsales-migrate

USER appuser
EXPOSE 8080
ENV API_HOST=0.0.0.0 \
    API_PORT=8080

# Liveness: GET /health → {"status":"ok"} (K8s probes preferred over Docker HEALTHCHECK)
HEALTHCHECK --interval=30s --timeout=3s --start-period=20s --retries=3 \
  CMD curl -fsS "http://127.0.0.1:${API_PORT}/health" || exit 1

ENTRYPOINT ["/usr/local/bin/api-http"]
