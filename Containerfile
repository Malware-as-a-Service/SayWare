# SPDX-FileCopyrightText: 2025 The SayWare development team
#
# SPDX-License-Identifier: CC0-1.0

FROM docker.io/library/rust:1.88.0 as chef
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install sccache --version 0.10.0 && \
    cargo install cargo-chef --version 0.1.71 && \
    apt-get update && \
    apt-get install -y --no-install-recommends musl-tools=1.2.3-1 && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache

FROM chef AS planner
WORKDIR /application/
COPY ./server/Cargo.toml ./
COPY ./Cargo.lock ./
COPY ./server/src/ ./src/
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef prepare --recipe-path recipe.json

FROM chef as builder
WORKDIR /application/
COPY --from=planner /application/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --bin sayware-server --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY ./server/Cargo.toml ./
COPY ./Cargo.lock ./
COPY ./server/src/ ./src/
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --bin sayware-server --release --target x86_64-unknown-linux-musl

FROM docker.io/library/alpine:3.22.0
RUN addgroup -g 1001 sayware && \
    adduser -D -u 1001 -G sayware sayware
USER sayware
WORKDIR /application/
COPY --from=builder /application/target/x86_64-unknown-linux-musl/release/sayware-server ./
CMD ["./sayware-server"]
