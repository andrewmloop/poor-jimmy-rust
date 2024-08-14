# Builder Image
FROM rust:1.74-alpine AS builder

RUN apk add --update \
    alpine-sdk \
    ffmpeg \
    youtube-dl \
    pkgconfig \
    cmake \
    musl-dev \
    openssl \
    libressl-dev

RUN USER=root cargo new --bin poor-jimmy
WORKDIR /poor-jimmy

COPY Cargo.lock Cargo.toml ./
RUN cargo build --release && \
    rm src/*.rs && \    
    rm ./target/release/deps/poor_jimmy*

COPY . .
RUN cargo build --release

# Final Image
FROM alpine:3.20.2

# Install dependencies to compile and run the bot
RUN apk add --update \
    alpine-sdk \
    ffmpeg \
    youtube-dl \
    pkgconfig \
    cmake \
    musl-dev \
    openssl \
    libressl-dev

COPY --from=builder /poor-jimmy/target/release/poor-jimmy ./target/release/poor-jimmy

# Command to start the bot once the container starts
CMD [ "./target/release/poor-jimmy" ]
