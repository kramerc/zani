FROM rust:1.85-alpine3.21 AS builder
RUN apk add musl-dev openssl-dev openssl-libs-static
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path . --target=x86_64-unknown-linux-musl

FROM alpine:3.21
COPY --from=builder /usr/local/cargo/bin/sexo /usr/local/bin/sexo
CMD ["sexo"]
