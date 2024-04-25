FROM rust:1.75-alpine as builder

RUN apk update && apk add build-base libc-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM alpine:3.19

RUN apk add --no-cache libc6-compat

COPY --from=builder /app/target/release/redirection-service /usr/local/bin/
RUN chmod +x /usr/local/bin/redirection-service

CMD ["redirection-service"]