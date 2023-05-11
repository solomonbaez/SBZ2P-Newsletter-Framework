######################################################
FROM rust:1.68.2 AS builder

WORKDIR /app

RUN cargo install --locked --branch master \
    --git https://github.com/eeff/cargo-build-deps

COPY Cargo.toml Cargo.lock ./
RUN cargo build-deps --release

ENV SQLX_OFFLINE true

COPY . .

RUN cargo build --release --bin production_rust

######################################################
FROM debian:buster-slim AS runtime

WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

ENV APP_ENVIRONMENT production

ENTRYPOINT ["./production_rust"]

COPY configurations configuration

COPY --from=builder /app/target/release/production_rust production_rust
