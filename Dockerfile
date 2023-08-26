FROM docker.io/library/rust:1.72-alpine as builder
RUN apk add --no-cache musl-dev
WORKDIR /build
COPY . /build
RUN cargo build --bins --release

FROM scratch
COPY --from=builder /build/target/release/controller /controller
ENTRYPOINT ["/controller"]

