# ------- BUILD STAGE ------- #
FROM rust:slim-buster as build

# create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release & rm src/*.rs

# copy your source tree
COPY ./src ./src
COPY ./.env ./.env
COPY ./migrations ./migrations

# build for release
RUN cargo build --release

# ------- FINAL STAGE ------- #

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /app/target/release/app .
COPY ./migrations ./migrations

# set the startup command to run your binary
CMD ["./app"]