FROM rust:1.75-slim AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /app/target/release/gaddy /usr/local/bin/gaddy

EXPOSE 7878

CMD ["gaddy"]