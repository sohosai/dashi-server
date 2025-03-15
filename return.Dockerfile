# Builder
FROM rust:1.85.0 AS builder

WORKDIR /app
COPY . /app
RUN cargo build --release --bin presentation


# Release
FROM debian:bookworm-slim AS release
LABEL maintainer="sohosai"

WORKDIR /app
COPY --from=builder /app/target/release/presentation /app/target/release/presentation
COPY --from=builder /app/return.sh /app/return.sh
RUN touch /app/.env \
    && chmod +x /app/.env \
    && chmod +x /app/return.sh \
    && apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["/app/return.sh"]
