FROM rust:1.87-alpine3.21 AS builder
RUN apk add musl-dev openssl-dev openssl-libs-static
WORKDIR /usr/src/app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# Set target based on architecture and add target
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
    "linux/amd64") TARGET="x86_64-unknown-linux-musl" ;; \
    "linux/arm64") TARGET="aarch64-unknown-linux-musl" ;; \
    *) echo "Unsupported platform: $TARGETPLATFORM" && exit 1 ;; \
    esac && \
    rustup target add $TARGET && \
    echo $TARGET > /tmp/target

COPY src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/src/app/target \
    TARGET=$(cat /tmp/target) && \
    cargo build --release --target=$TARGET && \
    cp target/$TARGET/release/sexo /usr/local/bin/sexo

FROM alpine:3.21
WORKDIR /app
COPY --from=builder /usr/local/bin/sexo /usr/local/bin/sexo
CMD ["sexo"]
