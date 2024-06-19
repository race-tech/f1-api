FROM rust:1.76.0-buster AS chef

RUN cargo install cargo-chef

WORKDIR /usr/f1-tech
COPY Cargo.toml Cargo.lock ./

# Create the all architecture tree
RUN mkdir bin
RUN mkdir crates
RUN cargo new --bin bin/api
RUN cargo new --lib crates/api
RUN cargo new --lib crates/application
RUN cargo new --lib crates/infrastructure
RUN cargo new --lib crates/shared
RUN cargo new --lib crates/logger

# Copy the Cargo.toml files
COPY bin/api/Cargo.toml bin/api/Cargo.toml
COPY crates/api/Cargo.toml crates/api/Cargo.toml
COPY crates/application/Cargo.toml crates/application/Cargo.toml
COPY crates/infrastructure/Cargo.toml crates/infrastructure/Cargo.toml
COPY crates/shared/Cargo.toml crates/shared/Cargo.toml
COPY crates/logger/Cargo.toml crates/logger/Cargo.toml

FROM chef AS planner

RUN cargo chef prepare  --recipe-path recipe.json

FROM planner AS buidler

COPY --from=planner /usr/f1-tech/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --recipe-path recipe.json

RUN rm -rf src bin/* crates/*

COPY bin bin
COPY crates crates
COPY resources/dockerfiles/test/config.yml .

RUN cargo build
