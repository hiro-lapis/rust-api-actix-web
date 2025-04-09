# build container for ecr

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
# copy all (without env setting)
COPY . .
# enable offline mode to compile sqlx code without db connection
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM debian:bookworm-slim
# workdir setting needs on each FROM
WORKDIR /app
# add user of 'book'
RUN adduser book && chown -R book /app
USER book
COPY --from=builder ./app/target/release/app ./target/release/app
# skip setting env
# env is preset in infra/app/main.tf and infra/secrets/main.tf
EXPOSE 8080
ENTRYPOINT [ "./target/release/app" ]
