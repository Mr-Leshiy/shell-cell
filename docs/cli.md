
# **Shell-Cell** CLI Reference

Now that you’ve [installed and configured](./install.md) **Shell-Cell**, you’re ready to launch your very first session!

To get started, you need a blueprint `scell.cue` file in your project directory.
This file defines the environment your shell will live in.
(For a deep dive into the blueprint specification, check out the [Blueprint Guide](./blueprint.md)).

The quickest way to get one is to let **Shell-Cell** generate it for you:
```shell
scell init
```

This creates a minimal, ready-to-use `scell.cue` in the current directory.
You can then open and adjust it to your needs.

## Commands

### `run` — Start a Shell-Cell Session

```shell
scell
```

**Shell-Cell** will automatically look for a file named `scell.cue` in your current location and start the **Shell-Cell** session on the spot.

#### Custom entry point (`-t`, `--target`)

By default, **Shell-Cell** tries to locate an entry point target named `main`.
If you want to use a different entry point, pass the `-t`, `--target` option.
```shell
scell -t <other-entrypoint-target>
```

#### Detach mode (`-d`, `--detach`)

If you want to start the container without attaching to the shell session,
pass the `-d`, `--detach` flag.
```shell
scell -d
```

This is useful for pre-warming containers in the background — the container will be started and kept alive,
but no interactive shell will be opened.

#### Custom blueprint path

If your configuration file is located elsewhere and you don’t want to change directories, you can point **Shell-Cell** directly to it.
```shell
scell ./path/to/the/blueprint/directory
```

### `init` — Create a Blueprint

```shell
scell init
```

Creates a minimal, functional `scell.cue` blueprint in the current directory (or in the directory passed as an argument).
Returns an error if a `scell.cue` already exists at that location.

```shell
scell init ./path/to/directory
```

### `ls` — List Shell-Cell Containers

```shell
scell ls
```

Displays an interactive table of all existing **Shell-Cell** containers.

### `stop` — Stop All Running Shell-Cell Containers

```shell
scell stop
```

Stops **all** running **Shell-Cell** containers (only **Shell-Cell** related containers, not any others).

Press `Ctrl-C` or `Ctrl-D` to abort early.

### `cleanup` — Remove Orphan Containers and Images

```shell
scell cleanup
```

Cleans up **orphan** **Shell-Cell** containers with their corresponding images and just images.
An item is considered an orphan when it is no longer associated with any existing `scell.cue` blueprint file
(e.g., the blueprint was deleted or moved, or the blueprint contents changed so the container hash no longer matches).


## ❓ Need more help ?

If you want to explore the full list of commands, flags, and capabilities, our built-in help menu is always there for you:
```shell
scell --help
```
