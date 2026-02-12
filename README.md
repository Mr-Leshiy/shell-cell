<div align="center">

<img src="./logo.png" alt="logo" />

# `Shell-Cell`

Lightweight CLI tool that turns simple YAML blueprints into instant, isolated, and reproducible containerized development shell sessions.

> ⚠️ Prototype, under heavy development

</div>

## 💡 How It Works

`Shell-Cell` reads a `scell.yml` blueprint file, compiles it into a image, and launches a persistent container that acts as a "shell server". You can then attach interactive shell sessions to this warm, ready-to-use environment.

Unlike standard containers that run a task and exit, `Shell-Cell` containers stay alive in the background, so you can jump in and out instantly.

## 🚀 Quick Start

### Prerequisites

A running [Docker](https://www.docker.com/) (or [Podman](https://podman.io/)) daemon is required.

### Install

```shell
cargo install shell-cell --locked
```

For socket configuration and other setup details, see the [Install and Configure](./docs/install.md) guide.

### Create a Blueprint

Place a `scell.yml` file in your project directory (see the full [Blueprint File Reference](./docs/blueprint.md) for all available instructions):

```yml
main:
  from: debian:bookworm
  workspace: /app
  shell: /bin/bash
  hang: while true; do sleep 3600; done
```

### Launch a Session

```shell
scell
```

That's it! `Shell-Cell` will find the `scell.yml` in your current directory, build the environment, and drop you into an interactive shell. For more CLI options and usage patterns, see the [CLI Reference](./docs/cli.md).

## 👨🏻‍👩🏻‍👦🏻‍👦🏻 Community

Our Discord server <https://discord.gg/URTBEuU5>

## 🎓 Want to know more ?

Follow the detailed documentation about how `Shell-Cell` works and how to use it:

👉 [Docs](./docs/readme.md)


## ➡️ Whats next?

Want to see what we’re working on? Check out our journey here:

👉 [Roadmap](./roadmap.md)

## ❤️ Contributing & Feedback

If you run into a bug or have a "what if" idea, don't be a stranger — open an issue, start a discussion or make a pull-request!

