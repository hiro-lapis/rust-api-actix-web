# create cache
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# compile rust while skipping to install dependencies written in recipe.json
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# If you want to disable cache build, delete above and use below
# FROM rust:1.84-slim-bookworm AS builder
# WORKDIR /app

# copy all (include env setting)
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
# workdir setting needs on each FROM
WORKDIR /app
# add user of 'book'
RUN adduser appuser && chown -R appuser /app
USER appuser
COPY --from=builder ./app/target/release/app ./target/release/app

# set env
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}
ENV PORT 8080
EXPOSE $PORT
ENTRYPOINT [ "./target/release/app" ]
