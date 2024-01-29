# Base image
FROM rust:1.74-alpine

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

# Create the bots directory
WORKDIR /poor-jimmy

# Build the bot
COPY . .
RUN cargo build --release

# Command to start the bot once the container starts
CMD [ "./target/release/poor-jimmy" ]
