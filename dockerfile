FROM rust:1.75-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/clip*

COPY src ./src

RUN cargo build --release

FROM alpine:latest

RUN apk add --no-cache libgcc

COPY --from=builder /app/target/release/clip /usr/local/bin/clip

ENTRYPOINT ["clip"]