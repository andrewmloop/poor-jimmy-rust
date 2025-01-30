# Poor Jimmy

Poor Jimmy is a Discord music bot written in Rust. This project is a re-write of the [existing Poor Jimmy written with Typescript](https://github.com/andrewmloop/poor-jimmy). This Discord bot utilizes Serenity, a powerful and flexible library for interacting with the Discord API, the Songbird library for handling audio playback, and Tokio for an asychronous runtime.

The main objectives of this project are:
- Get hands on experience with the Rust programming language
- Explore concurrency
- Explore containerization

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
   DISCORD_TOKEN=YOUR_DICORD_TOKEN
   ```

   Replace `YOUR_DISCORD_TOKEN` with your actual Discord bot token.

## Running Locally

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
   docker run --env-file ./.env poor-jimmy
   ```

## Deployment

### Raspberry Pi

Poor Jimmy is currently hosted on a Raspberry Pi. There are many ways to deploy to a device like a Raspberry Pi. The method this project uses includes setting up a Docker Hub repository.

1. Build the bot

    ```bash
    docker build -t poor-jimmy .
    ```
2. Tag the bot's image with your Docker Hub username and repository

    ```bash
    docker image tag poor-jimmy <docker_hub_username>/<repository_name>
    ```

    Optionally, give the image a versioning tag. If a versioning tag is not included, a version of "latest" is given.

    ```bash
    docker image tag poor-jimmy <docker_hub_username>/<repository_name>:<version>
    ```


3. Push the image to Docker Hub with the tag created in the previous step

    ```bash
    docker push <docker_hub_username>/<repository_name>
    ```

4. SSH into the Raspberry Pi and pull the bot's image from Docker Hub. This step assumes Docker has been installed on the Raspberry Pi.

    ```bash
    docker pull <docker_hub_username>/<repository_name>
    ```

5. Start the container. This step assumes the .env file with the Discord secret is stored somewhere on the Raspberry Pi.

    ```bash
    cd /path/to/where/.env/file/lives

    docker run --env-file ./.env <docker_hub_username>/<repository_name>
    ```

### Heroku

Poor Jimmy can be deployed as a container in any cloud environment. Poor Jimmy can be hosted on a Heroku dyno. To deploy a new version:

1. Build the bot for x86_64 (a Heroku requirement)
   
   ```bash
   docker build --platform linux/amd64 -t poor-jimmy .
   ```

2. Tag the image with registry.heroku.com/{heroku_app}/{heroku_process_type}

    ```bash
    docker tag a052b961e1d4 registry.heroku.com/poor-jimmy/worker
    ```

3. Push the image to Heroku's registry

    ```bash
    docker push registry.heroku.com/poor-jimmy/worker
    ```

4. Release the new image to a container on your Heroku dyno

    ```bash
    heroku container:release worker --app poor-jimmy
    ```
