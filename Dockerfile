FROM rust:1.70-alpine as builder

WORKDIR /usr/src/app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release

# Path: Dockerfile
FROM alpine:latest as runner

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/ .
CMD ["./archi"]
