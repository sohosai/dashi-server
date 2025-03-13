# Builder
FROM rust:1.81.0 AS builder

WORKDIR /app
COPY . /app
RUN cargo build --release --bin ping \
    && cargo build --release --bin init \
    && cargo build --release --bin presentation \
    && cd migration \
    && cargo build --release


# Release
FROM debian:bookworm-slim AS release
LABEL maintainer="sohosai"

WORKDIR /app
COPY --from=builder /app/target/release/migration /app/target/release/migration
COPY --from=builder /app/target/release/ping /app/target/release/ping
COPY --from=builder /app/target/release/init /app/target/release/init
COPY --from=builder /app/target/release/presentation /app/target/release/presentation
COPY --from=builder /app/crates/init/image/tsukuba.webp /app/crates/init/image/tsukuba.webp
COPY --from=builder /app/crates/init/csv/color.csv /app/crates/init/csv/color.csv
COPY --from=builder /app/crates/init/csv/connector.csv /app/crates/init/csv/connector.csv
COPY prod.sh /app/prod.sh
RUN touch /app/.env \
    && mkdir /app/init \
    && chmod +x /app/.env \
    && chmod +x /app/prod.sh \
    && chmod +x /app/init \
    && apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["/app/prod.sh"]
