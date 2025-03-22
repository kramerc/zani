FROM rust:1.85-alpine3.21

RUN apk add musl-dev openssl-dev openssl-libs-static

WORKDIR /app
COPY . .
RUN cargo install --path .

RUN apk del musl-dev openssl-dev

CMD ["sexo"]
