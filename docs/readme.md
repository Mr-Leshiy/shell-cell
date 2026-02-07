# `Shell-Cell` Documentation

- [Install and Configure](./install.md)
- [CLI Reference](./cli.md)
- [Blueprint File Reference](./blueprint_file.md)

## 💡 The Core Idea

`Shell-Cell` is a lightweight container orchestrator that turns simple [YAML](https://yaml.org) blueprints into instant, isolated development environments.

It could be really handy, when you want to have secure, isolated place for your development.

## 🏛️ Architecture conecepts

1. **The blueprint file (`scell.yml`)**.<br>
    Everything starts with the configuration file. It defines your `Shell-Cell` targets (the environment layers).

2. **`Shell-Cell` targets.**<br>
    Think of *targets* as named function, instead of one giant, monolithic `Dockerfile`,
    `Shell-Cell` encourages you to break your setup into logical pieces.

3. **"Shell Server" Model.**<br>
    Unlike a standard container that runs a single task and exits, a Shell-Cell is designed to hang.
    By using the `hang` instruction, the container stays alive in the background, acting as a persistent server.
    This allows you to attach multiple `Shell-Cell` sessions to a warm, ready-to-use environment instantly.