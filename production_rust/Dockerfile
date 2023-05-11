FROM rust:1.68.2 AS planner

WORKDIR /app

RUN cargo install cargo-chef

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

######################################################
FROM rust:1.68.2 AS cacher 

WORKDIR /app

RUN cargo install cargo-chef

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

######################################################
FROM rust:1.68.2 AS builder

WORKDIR /app

COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME

ENV SQLX_OFFLINE true

COPY . .

RUN cargo build --release --bin production_rust

######################################################
FROM debian:bullseye-slim AS runtime

WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

ENV APP_ENVIRONMENT production

ENTRYPOINT ["./production_rust"]

COPY configurations configuration

COPY --from=builder /app/target/release/production_rust production_rust
