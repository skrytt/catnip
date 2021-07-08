# Archived

More development may continue on https://github.com/Conundris/catnip , however, I've decided to focus my time on other projects.

# catnip

A Discord chatbot written in Rust, using Serenity and SQLite!

This is presently used in [Skrytt](https://github.com/skrytt)'s Discord guild.

## Setup

These instructions describe the basic setup process:

- Create a configuration file `mount/env` with contents like:
```
DISCORD_TOKEN=YOUR_BOT_TOKEN_GOES_HERE
RUST_LOG=warn,catnip=debug
```
- Build the Docker image: `docker build -t catnip .`
- Run the Docker container: `docker run -v "$(pwd)/mount":/catnip/mount -it --rm --name catnip catnip`

## Contributing Guidelines

Thanks for your interest in helping!

### Finding something to work on

- Please join the Discord guild (ask Skrytt for an instant invite link).
- [GitHub issues](https://github.com/skrytt/catnip/issues) is where we formally document and discuss the project design and ideas for changes. You can create new issues if you wish.
- Before making any changes, comment on the ticket you wish to work on, providing a rough idea of what you intend to do and request the ticket be assigned to you. Ticket assignment is considered approval to work on that ticket.
- Please understand we cannot accept pull requests for unticketed changes, or tickets that were not assigned to you.

### Making changes

- We use a feature branch workflow. If you're new to feature branches, that's ok! Have a read of [this guide](https://gist.github.com/vlandham/3b2b79c40bc7353ae95a) and ask for help in the Discord as you need it.
- When you create the Pull Request as described in the above guide, a project collaborator will help to review your code and suggest improvements. Often we will ask you to make further changes before your change can be merged to our master branch. We use GitHub's review tools for review comments; however, we also may use the Discord for more informal discussion and collaboration as needed.
- Once your code passes the review stage, we will merge it to master! :fireworks:

### Deployment of new code in master

Currently, deployment is done manually by Skrytt. Skrytt is looking to make it possible for other contributors to do this also, and eventually automate it. In the meantime, nudge Skrytt on Discord if your merged changes need to be installed to the live bot :smile:

## Contributors

- [Conundris](https://github.com/Conundris)
- [Alexisnotonffire](https://github.com/alexisnotonffire)
