FROM rust:1.81.0 AS builder

WORKDIR /app
COPY . /app

RUN touch /app/.env \
    && chmod +x /app/.env \
    && chmod +x /app/dev.sh \
    && cargo build --release --bin ping \
    && cargo build --release --bin healthcheck

ENTRYPOINT ["/app/dev.sh"]
