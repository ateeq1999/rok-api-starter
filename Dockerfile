FROM rust:1.85-slim AS builder
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY Cargo.toml ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release 2>/dev/null || true
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
RUN useradd -m -u 1001 appuser
WORKDIR /app
COPY --from=builder /app/target/release/rok-api-test ./
COPY --from=builder /app/database/migrations ./migrations
RUN chown -R appuser:appuser /app
USER appuser
ENV LISTEN_ADDR=0.0.0.0:3000
EXPOSE 3000
ENTRYPOINT ["./rok-api-test"]
