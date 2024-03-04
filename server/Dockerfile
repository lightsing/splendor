FROM lightsing/rust-builder:1.76-bookworm-cn AS planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lightsing/rust-builder:1.76-bookworm-cn AS builder
WORKDIR /app
ARG SUPERVISED=false
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN if [ "$SUPERVISED" = "true" ]; then \
    cargo chef cook --zigbuild --release --recipe-path recipe.json --features supervisor; \
    else \
    cargo chef cook --zigbuild --release --recipe-path recipe.json; \
    fi
# Build application
COPY . .
RUN if [ "$SUPERVISED" = "true" ]; then \
    cargo zigbuild --release -p splendor-server --features supervisor; \
    else \
    cargo zigbuild --release -p splendor-server; \
    fi

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
