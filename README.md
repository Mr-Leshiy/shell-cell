<div align="center">

<img src="./logo.png" alt="logo" />

# **Shell-Cell**

Lightweight CLI tool that turns simple CUE blueprints into instant, isolated, and reproducible containerized development shell sessions.
</div>

> [!WARNING]
> Under heavy development, backwards compatibility is not guaranteed, and future versions may introduce breaking changes.

## 💡 How It Works

**Shell-Cell** reads a `scell.cue` blueprint, compiles it into a image, and launches a persistent container that acts as a "shell server". You can then attach interactive shell sessions to this warm, ready-to-use environment.

## 🚀 Quick Start

### Prerequisites

A running [Docker](https://www.docker.com/) (or [Podman](https://podman.io/)) daemon is required.

### Install

- Build for Unix 
<!-- cspell: disable -->
```shell
curl -fsSL https://github.com/Mr-Leshiy/shell-cell/releases/latest/download/shell-cell-installer.sh | sh
```
<!-- cspell: enable -->

- Build from source (any platform)

  **Prerequisites**: `Go 1.24+` — the Go toolchain is required.
```shell
cargo install shell-cell --locked
```

For socket configuration and other setup details, see the [Install and Configure](https://mr-leshiy.github.io/shell-cell/install) guide.

### Create a Blueprint

Run `scell init` to generate a minimal `scell.cue` in your project directory:

```shell
scell init
```

Or write one by hand (see the full [Blueprint Reference](https://mr-leshiy.github.io/shell-cell/blueprint) for all available instructions):

```cue
main: {
  from_image: "debian:bookworm"
  workspace:  "/app"
  shell:      "/bin/bash"
  hang:       "while true; do sleep 3600; done"
}
```

### Launch a Session

```shell
scell
```

That's it! **Shell-Cell** will find the `scell.cue` in your current directory, build the environment, and drop you into an interactive shell. For more CLI options and usage patterns, see the [CLI Reference](https://mr-leshiy.github.io/shell-cell/cli).

## 👨🏻‍👩🏻‍👦🏻‍👦🏻 Community

Our Discord server <https://discord.gg/URTBEuU5>

## 🎓 Want to know more ?

Follow the detailed documentation about how **Shell-Cell** works and how to use it:

👉 [Docs](https://mr-leshiy.github.io/shell-cell/)


## ➡️ Whats next?

Want to see what we’re working on? Check out our journey here:

👉 [Roadmap](./roadmap.md)

## ❤️ Contributing & Feedback

If you run into a bug or have a "what if" idea, don't be a stranger — open an issue, start a discussion or make a pull-request!

