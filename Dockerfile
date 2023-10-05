FROM --platform=$BUILDPLATFORM debian AS x
#RUN apk add --no-cache clang curl llvm lld musl-tools
RUN apt-get update && apt-get install -y clang curl llvm lld musl-tools gcc-multilib g++-multilib
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add aarch64-unknown-linux-musl
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /app

#FROM chef AS planner
#COPY . .
#RUN cargo chef prepare --recipe-path recipe.json

FROM --platform=$BUILDPLATFORM x AS builder
ARG CARGO_PROFILE="release"
#COPY --from=planner /app/recipe.json recipe.json
#RUN cargo chef cook --profile ${CARGO_PROFILE} --recipe-path recipe.json
COPY . .
ENV CC_aarch64_unknown_linux_musl=clang
ENV AR_aarch64_unknown_linux_musl=llvm-ar
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"
ENV CC_x86_64_unknown_linux_musl=clang
ENV AR_x86_64_unknown_linux_musl=llvm-ar
ENV CARGO_TARGET_x86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"
ENV RUSTC_BOOTSTRAP=compiler_builtins
ARG TARGETARCH
RUN echo ${TARGETARCH} | sed s/arm64/aarch64/ | sed s/amd64/x86_64/ > /tmp/targetarch
RUN cargo  build --profile ${CARGO_PROFILE} --bin controller --target=`cat /tmp/targetarch`-unknown-linux-musl
RUN mkdir -p /${TARGETARCH}
RUN cp /app/target/`cat /tmp/targetarch`-unknown-linux-musl/${CARGO_PROFILE}/controller /${TARGETARCH}/controller


#FROM --platform=$TARGETPLATFORM gcr.io/distroless/cc-debian12 AS runtime
FROM scratch AS runtime
#COPY --from=builder /app/target/**/controller /controller
#COPY --from=builder /usr/local/cargo/bin/controller /usr/local/cargo/bin/controller
ARG TARGETARCH
COPY --from=builder /${TARGETARCH}/controller /controller
CMD ["/controller"]

