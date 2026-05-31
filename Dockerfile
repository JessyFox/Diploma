FROM rust:1.92.0-slim AS builder

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/config config
COPY --from=builder /usr/src/target/release/stat_api_rs-cli stat_api_rs-cli

ENTRYPOINT ["/usr/app/stat_api_rs-cli"]
