# # Builder
# FROM rust:1.81.0 AS builder

# WORKDIR /app
# COPY . /app
# RUN cargo build --release --bin ping \
#     && cargo build --release --bin init


# # Release
# FROM debian:bookworm-slim AS release
# LABEL maintainer="sohosai"

# WORKDIR /app
# COPY --from=builder /app/migration /app/migration
# COPY --from=builder /app/target/release/ping /app/target/release/ping
# COPY --from=builder /app/target/release/init /app/target/release/init
# COPY --from=builder /app/crates/init/image/tsukuba.webp /app/crates/init/image/tsukuba.webp
# COPY dev.sh /app/dev.sh
# RUN touch /app/.env \
#     && chmod +x /app/.env \
#     && chmod +x /app/dev.sh \
#     && apt-get update \
#     && DEBIAN_FRONTEND=noninteractive apt-get install -y ca-certificates \
#     && rm -rf /var/lib/apt/lists/*

# ENTRYPOINT ["/app/dev.sh"]

# Builder
FROM rust:1.81.0 AS builder

WORKDIR /app
COPY . /app
RUN cargo build --release --bin ping \
    && cargo build --release --bin init \
    && cd migration \
    && cargo build --release


# Release
FROM debian:bookworm-slim AS release
LABEL maintainer="sohosai"

WORKDIR /app
COPY --from=builder /app/target/release/migration /app/target/release/migration
COPY --from=builder /app/target/release/ping /app/target/release/ping
COPY --from=builder /app/target/release/init /app/target/release/init
COPY --from=builder /app/crates/init/image/tsukuba.webp /app/crates/init/image/tsukuba.webp
COPY dev.sh /app/dev.sh
RUN touch /app/.env \
    && chmod +x /app/.env \
    && chmod +x /app/dev.sh \
    && apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["/app/dev.sh"]