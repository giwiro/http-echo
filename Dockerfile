FROM rust:1.89-alpine AS builder

RUN apk add --no-cache musl-dev libc-dev gcc

WORKDIR /app
COPY . .

RUN cargo build --release

FROM alpine:3.20

RUN apk add --no-cache ca-certificates

WORKDIR /app
COPY --from=builder /app/target/release/http-echo /usr/local/bin/http-echo

ENTRYPOINT ["http-echo"]
