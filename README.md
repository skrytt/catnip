# Archived

More development may continue on https://github.com/Conundris/catnip , however, I've decided to focus my time on other projects.

# catnip

A Discord chatbot written in Rust, using Serenity and SQLite.

## Setup

These instructions describe the basic setup process:

- Create a configuration file `mount/env` with contents like:
```
DISCORD_TOKEN=YOUR_BOT_TOKEN_GOES_HERE
RUST_LOG=warn,catnip=debug
```
- Build the Docker image: `docker build -t catnip .`
- Run the Docker container: `docker run -v "$(pwd)/mount":/catnip/mount -it --rm --name catnip catnip`

## Contributors

- [Conundris](https://github.com/Conundris)
