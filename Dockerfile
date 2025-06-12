FROM rust:1.87-alpine3.21 AS builder
RUN apk add musl-dev openssl-dev openssl-libs-static
WORKDIR /usr/src/app
COPY . .

# Set target based on architecture
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
    "linux/amd64") TARGET="x86_64-unknown-linux-musl" ;; \
    "linux/arm64") TARGET="aarch64-unknown-linux-musl" ;; \
    *) echo "Unsupported platform: $TARGETPLATFORM" && exit 1 ;; \
    esac && \
    rustup target add $TARGET && \
    cargo install --path . --target=$TARGET

FROM alpine:3.21
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/zani /usr/local/bin/zani
CMD ["zani"]
