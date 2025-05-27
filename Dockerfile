FROM rust:1 AS builder
WORKDIR app

COPY . .
RUN cargo build -p sqlet_cli --release --all-features


FROM debian:stable-slim AS runtime
COPY --from=builder /app/target/release/sqlet /usr/local/bin
ENTRYPOINT ["/usr/local/bin/sqlet"]
