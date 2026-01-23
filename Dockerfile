FROM rust:alpine AS chef

RUN apk add --no-cache musl-dev openssl-libs-static openssl-dev

RUN cargo install cargo-chef

WORKDIR /usr/f1-api
COPY Cargo.toml Cargo.lock ./

# Create the all architecture tree
RUN mkdir bin
RUN mkdir crates
RUN cargo new --bin bin/api
RUN cargo new --lib crates/entities
RUN cargo new --lib crates/infrastructure
RUN cargo new --lib crates/error
RUN cargo new --lib crates/logger

# Copy the Cargo.toml files
COPY bin/api/Cargo.toml bin/api/Cargo.toml
COPY crates/entities/Cargo.toml crates/entities/Cargo.toml
COPY crates/infrastructure/Cargo.toml crates/infrastructure/Cargo.toml
COPY crates/error/Cargo.toml crates/error/Cargo.toml
COPY crates/logger/Cargo.toml crates/logger/Cargo.toml

FROM chef AS planner

RUN cargo chef prepare  --recipe-path recipe.json

FROM planner AS builder

COPY --from=planner /usr/f1-api/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

RUN rm -rf src bin/* crates/*

COPY bin bin
COPY crates crates

RUN cargo build --release

FROM alpine AS runtime

LABEL maintainer="Thibault C. <thibault.chene23@gmail.com>"

COPY --from=builder /usr/f1-api/target/release/f1-api /usr/local/bin

CMD ["/usr/local/bin/f1-api"]
