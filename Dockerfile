FROM docker.io/library/rust:1.72-alpine AS chef
RUN apk add --no-cache musl-dev
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG CARGO_BUILD_ARGS="--release"
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook ${CARGO_BUILD_ARGS} --recipe-path recipe.json
COPY . .
RUN cargo build ${CARGO_BUILD_ARGS} --bin controller

FROM scratch
COPY --from=builder /app/target/*/controller /controller
ENTRYPOINT ["/controller"]

