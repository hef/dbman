FROM docker.io/library/rust:1.72.1 AS chef
#RUN apk add --no-cache clang llvm musl-dev libgcc
#RUN cargo install cargo-chef
#WORKDIR /app

#FROM chef AS planner
#COPY . .
#RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG CARGO_PROFILE="release"
#COPY --from=planner /app/recipe.json recipe.json
#RUN cargo chef cook --profile ${CARGO_PROFILE} --recipe-path recipe.json
COPY . .
RUN cargo install --profile ${CARGO_PROFILE} --bin controller --path .


FROM gcr.io/distroless/cc-debian12 AS runtime
#COPY --from=builder /app/target/*/controller /controller
COPY --from=builder /usr/local/cargo/bin/controller /usr/local/cargo/bin/controller
CMD ["/usr/local/cargo/bin/controller"]

