ARG CARGO_BUILD_ARGS="--release"
FROM docker.io/library/rust:1.72-alpine as builder
RUN apk add --no-cache musl-dev
WORKDIR /build
COPY Cargo.toml Cargo.lock /build/
RUN mkdir src && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN touch src/lib.rs
RUN cargo build --bin controller $CARGO_BUILD_ARGS
RUN rm -rf src target
COPY . /build
RUN cargo build --bin controller $CARGO_BUILD_ARGS

FROM scratch
COPY --from=builder /build/target/*/controller /controller
ENTRYPOINT ["/controller"]

