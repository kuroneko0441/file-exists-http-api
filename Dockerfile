FROM rust:1.86-alpine AS builder
ENV OPENSSL_STATIC=1
ENV OPENSSL_DIR=/usr

RUN apk add --no-cache \
  musl-dev \
  openssl-dev \
  openssl-libs-static \
  pkgconfig \
  build-base

WORKDIR /app
COPY . .

RUN rustup target add x86_64-unknown-linux-musl \
 && cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.21.3

RUN apk add --no-cache
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/file-exists-http-api /usr/bin/app
RUN chmod +x /usr/bin/app

EXPOSE 3000
ENTRYPOINT ["app"]
