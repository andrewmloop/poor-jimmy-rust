### Builder Image ###
FROM rust:1.82-alpine AS builder

# Install dependencies
RUN apk add --update \
    alpine-sdk \
    ffmpeg \
    yt-dlp \
    pkgconfig \
    cmake \
    musl-dev \
    openssl \
    libressl-dev

# Create a new project to hold the bot using cargo
RUN USER=root cargo new --bin poor-jimmy

# Set the new project directory as the working directory
WORKDIR /poor-jimmy

# Copy the project into the builder image
COPY . .

# Build the bot
RUN cargo build --release

### Final Image ###
# This final image is what is ultimately shipped. It just has the bot's binary
# and all the dependencies it needs. We leave behind all the build tools.
FROM alpine:latest

# Install dependencies to run the bot
RUN apk add --update \
    alpine-sdk \
    ffmpeg \
    yt-dlp \
    pkgconfig \
    cmake \
    musl-dev \
    openssl \
    libressl-dev

# Set the working directory for where the binary will live
WORKDIR /bot

# Copy the binary to our final image
COPY --from=builder /poor-jimmy/target/release/poor-jimmy ./

# Command to start the bot once the container starts
CMD [ "/bot/poor-jimmy" ]
