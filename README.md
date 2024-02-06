# Poor Jimmy

Poor Jimmy is a Discord music bot written in Rust. This project is a re-write of the [existing Poor Jimmy written with Typescript](https://github.com/andrewmloop/poor-jimmy). This Discord bot utilizes Serenity, a powerful and flexible library for interacting with the Discord API, the Songbird library for handling audio playback, and Tokio for an asychronous runtime.

The main objectives of this project are:
- Learn the Rust programming language
- Explore containerization
- Explore cloud hosting

## Dependencies

- [Rust v1.74](https://www.rust-lang.org/learn)
- [Serenity v0.11.7](https://docs.rs/serenity/0.11.7/serenity/index.html)
- [Songbird v0.3.2](https://docs.rs/songbird/0.3.2/songbird/struct.Songbird.html)
- [Tokio v1.17.0](https://tokio.rs/)

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install): Ensure that Rust is installed on your system.
- [Docker](https://www.docker.com/get-started): Docker is used for containerization.

### Configuration

1. Create a `.env` file in the project directory with the following content:

   ```bash
   DISCORD_TOKEN = "YOUR_DISCORD_BOT_TOKEN"
   ```

   Replace `YOUR_DISCORD_BOT_TOKEN` with your actual Discord bot token.

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/andrewmloop/poor-jimmy-rust.git
   cd poor-jimmy-rust
   ```

2. Build the Docker image:

   ```bash
   docker build -t poor-jimmy .
   ```

3. Run the Docker container:

   ```bash
   docker run -d --name poor-jimmy poor-jimmy
   ```

### Usage

1. Start the Docker container:

   ```bash
   docker start poor-jimmy
   ```

2. Interact with your bot on Discord.
