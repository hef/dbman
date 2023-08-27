ARG CARGO_BUILD_ARGS="--release"
FROM docker.io/library/rust:1.72-alpine as builder
RUN apk add --no-cache musl-dev
WORKDIR /build
COPY . /build
RUN cargo build --bin controller 

FROM scratch
COPY --from=builder /build/target/*/controller /controller
ENTRYPOINT ["/controller"]

