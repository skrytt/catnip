# catnip

A Discord chatbot written in Rust, using Serenity and SQLite.

## Setup

These instructions describe the basic setup process:

- Create a subdirectory "mount" that will be used as a Docker mount volume
- Create the SQLite database: `sqlite3 mount/catnip.db3 ".read sqlite/setup.sql"`
- Create env file `mount/env` with contents like: ```
DISCORD_TOKEN="YOUR_BOT_TOKEN_GOES_HERE"
RUST_LOG=warn,catnip=debug
```
- Build the Docker image: `sudo docker build -t catnip .`
- Run the Docker container: `sudo docker run -v "$(pwd)/mount":/catnip/mount -it --rm --name catnip catnip`

## Contributors

- [Conundris](https://github.com/Conundris)
