FROM lukemathwalker/cargo-chef:latest-rust-alpine AS chef
WORKDIR /src

FROM chef AS planner
COPY . .
# Create dependencies
RUN cargo chef prepare --recipe-path /src/recipe.json

FROM chef AS builder
COPY --from=planner /src/recipe.json /src/recipe.json
# Build dependencies
RUN cargo chef cook --release --recipe-path /src/recipe.json
# Build application
COPY . .
ARG GIT_HASH
ENV GIT_HASH=${GIT_HASH}
RUN cargo build --release --bin player-Rust

# Use scratch image to reduce image size
FROM scratch
ARG version=0.1.0
ARG release=unreleased
LABEL name="player-Rust" \
      maintainer="Fachschaft IT, HS-Esslingen" \
      version=${version} \
      release=${release} \
      summary="A client for BitWars written in Rust using Tokio" \
      description="A client for BitWars written in Rust using the Axum webserver with Tokio runtime"
EXPOSE 3000

COPY --from=builder /src/target/release/player-Rust /
ENTRYPOINT ["./player-Rust"]
