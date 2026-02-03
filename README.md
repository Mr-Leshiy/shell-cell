<h1>🐚 Shell-Cell</h1>

![logo](./logo.png)

> Prototype, under heavy development

Lightweight CLI app designed to spin up instant, isolated, and reproducible development shell sessions.

## Table of contents
- [Table of contents](#table-of-contents)
- [Installation and configuration](#installation-and-configuration)
  - [UNIX socket configuration (UNIX)](#unix-socket-configuration-unix)
- [Run](#run)
- [How it works](#how-it-works)

## Installation and configuration

To install `Shell-Cell` use cargo
```shell
cargo isntall --git https://github.com/Mr-Leshiy/shell-cell.git --locked
```

`Shell-Cell` requires a running instace either [Docker] or [Podman] daemon.

### UNIX socket configuration (UNIX)

To interact with the [Docker] or [Podman] daemon
`Shell-Cell` uses a UNIX socket connection on UNIX based operating systems.

If `Shell-Cell` cannot locate the `docker.sock` file you could run
- for [Docker]
```shell
docker context inspect
```
- for [Podman]
```shell
???
```

And set the env var `DOCKER_HOST` (even if you are using [Podman]) with the found path location.

## Run

```shell
scell
```

## How it works

Curious about the check out the [architecture.md](/architecture.md)

[Docker]: https://www.docker.com
[Podman]: https://podman.io