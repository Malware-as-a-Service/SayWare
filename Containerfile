# SPDX-FileCopyrightText: 2025 The SayWare development team
#
# SPDX-License-Identifier: CC0-1.0

FROM docker.io/clux/muslrust:stable as chef
RUN cargo install cargo-chef --version 0.1.73

FROM chef AS planner
WORKDIR /application/
COPY ./server/Cargo.toml ./
COPY ./Cargo.lock ./
COPY ./server/src/ ./src/
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
WORKDIR /application/
COPY --from=planner /application/recipe.json recipe.json
RUN cargo chef cook --bin sayware-server --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY ./server/Cargo.toml ./
COPY ./Cargo.lock ./
COPY ./server/src/ ./src/
RUN cargo build --bin sayware-server --release --target x86_64-unknown-linux-musl

FROM docker.io/library/alpine:3.22.0
RUN addgroup -g 1001 sayware && \
    adduser -D -u 1001 -G sayware sayware
USER sayware
WORKDIR /application/
COPY --from=builder /application/target/x86_64-unknown-linux-musl/release/sayware-server ./
CMD ["./sayware-server"]
