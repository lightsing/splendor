FROM lukemathwalker/cargo-chef:latest-rust-1.76-bookworm as chef

RUN mkdir ~/.cargo
COPY .cargo-rsproxy.toml ~/.cargo/config.toml
RUN apt-get update && apt-get install -y python3 python3-pip
RUN pip3 install --break-system-packages ziglang
RUN cargo install cargo-zigbuild

WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --zigbuild --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo zigbuild --release -p splendor-server

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y libc6 && rm -rf /var/lib/apt/lists/*

FROM runtime
WORKDIR /app
COPY --from=builder /app/target/release/splendor-server /app/splendor-server
ENV RUST_LOG=info
ENV SECRETS_PATH=/app/secrets
ENV N_PLAYERS=4
ENV SERVER_ADDR=0.0.0.0:8080
EXPOSE 8080
CMD ["/app/splendor-server"]
