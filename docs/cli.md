
# **Shell-Cell** CLI Reference

Now that you’ve [installed and configured](./install.md) **Shell-Cell**, you’re ready to launch your very first session!

To get started, you need a blueprint `scell.yml` file in your project directory.
This file defines the environment your shell will live in.
(For a deep dive into the blueprint specification, check out the [Blueprint Guide](./blueprint.md)).

The quickest way to get one is to let **Shell-Cell** generate it for you:
```shell
scell init
```

This creates a minimal, ready-to-use `scell.yml` in the current directory.
You can then open and adjust it to your needs.

Once your file is ready, simply open your terminal in that directory and run:
```shell
scell
```

That’s it, simple as that!
**Shell-Cell** will automatically look for a file named `scell.yml` in your current location and start the **Shell-Cell** session on the spot.

It would try to locate an entry point target - `main`.

If you want to specify some other entry point target, rather than `main`,
you could pass a `-t`, `--target` CLI option.
```shell
scell -t <other-entrypoint-target>
```


If your configuration file is located elsewhere and you don't want to change directories, you can point **Shell-Cell** directly to it.
```shell
scell ./path/to/the/blueprint/directory
```

## Commands

### `init` — Create a Blueprint

```shell
scell init
```

Creates a minimal, functional `scell.yml` blueprint in the current directory (or in the directory passed as an argument).
Returns an error if a `scell.yml` already exists at that location.

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
An item is considered an orphan when it is no longer associated with any existing `scell.yml` blueprint file
(e.g., the blueprint was deleted or moved, or the blueprint contents changed so the container hash no longer matches).


## ❓ Need more help ?

If you want to explore the full list of commands, flags, and capabilities, our built-in help menu is always there for you:
```shell
scell --help
```
