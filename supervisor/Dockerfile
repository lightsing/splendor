FROM lightsing/rust-builder:1.76-bookworm-cn AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lightsing/rust-builder:1.76-bookworm-cn AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --zigbuild --release --recipe-path recipe.json
COPY . .
RUN cargo zigbuild --release --package splendor-supervisor --bin splendor-supervisor

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y libc6 && rm -rf /var/lib/apt/lists/*

FROM runtime
COPY --from=builder /app/target/release/splendor-supervisor /app/splendor-supervisor

ENV RUST_LOG=debug
CMD ["/app/splendor-supervisor"]
