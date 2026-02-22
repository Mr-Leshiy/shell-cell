# **Shell-Cell** Blueprint Reference

**Shell-Cell** builds your environment by reading a set of instructions from a `scell.yml` files.

`scell.yml` - is a [YAML](https://yaml.org) formatted file that contains everything needed to configure your session.

> The full formal definition of the blueprint schema is available at [`src/scell/types/scell_schema.cue`](../src/scell/types/scell_schema.cue).

Here is a minimal functional example:
```yml
main:
  from_image: debian:bookworm
  workspace: workdir
  shell: /bin/bash
  hang: while true; do sleep 3600; done
```

**Shell-Cell** follows a strict logic when building your image.
It parses your target definitions into a *chain*,
moving from your entry point (`main`) down to the base "bottom" target.

The actual image building process, on contrary, happens backwards.
Starts from the "bottom" target and works its way up to your entry point (`main`):
1. `bottom_target`
2. `target_3`
3. `target_2`
4. `target_1`
5. `main`

## **Shell-Cell** target

**Shell-Cell** are comprised of a series of target declarations and recipe definitions.

```yml
<target-name>:
    <recipe>
    ...
```

A valid target name must start with a lowercase letter and contain only lowercase letters, digits, hyphens, and underscores (pattern: `^[a-z][a-z0-9_-]*$`).

Inside each target, during the **Shell-Cell** image building process,
the instructions are executed in a specific, strict order:
1. `workspace`
2. `from` / `from_image` / `from_docker`
3. `env`
4. `copy`
5. `build`

### `from`, `from_image`, `from_docker`

Similar to the Dockerfile [`FROM`](https://docs.docker.com/reference/dockerfile/#from) instruction,
these statements specify the base of the **Shell-Cell** layer.

Only one of these statements must be present in the **Shell-Cell** target definition.

Either `from_image` or `from_docker` is required somewhere in the target chain — without one of them
there is no way to specify the basis of the image. `from` on its own only delegates to another target
and must eventually resolve to a `from_image` or `from_docker`.

#### `from_image`

Uses a Docker registry image as the base layer.

```yml
from_image: <image>:<tag>
```

#### `from_docker`

Uses a Dockerfile on the filesystem as the base layer.
The path is resolved relative to the `scell.yml` file.

```yml
from_docker: path/to/Dockerfile
```

#### `from`

References another [**Shell-Cell** target](#shell-cell-target), resolved recursively.
Use `+<target_name>` to reference a target in the same file, or `path/to/dir+<target_name>`
to reference a target in another `scell.yml`.

```yml
from: +<target_name>
```
```yml
from: path/to/dir+<target_name>
```

### `shell`

A location to the shell, which would be available in the build image and running container.

Such shell would be used for a **Shell-Cell** session.

Only the first `shell` statement encountered in the target chain (starting from the entry point) is used.

```yml
shell: /bin/bash
```

### `hang`

This instruction ensures your container stays active and doesn't exit immediately after it starts. This effectively transforms your **Shell-Cell** container into a persistent "shell server" that remains ready for you to jump in at any time.

Only the first `hang` statement encountered in the target chain (starting from the entry point) is used.

To work correctly, you must specify a command that keeps the container running indefinitely.
The most recommended approach is a simple infinite loop:
```yaml
hang: while true; do sleep 3600; done
```

This command would be placed as a Dockerfile [`ENTRYPOINT`](https://docs.docker.com/reference/dockerfile/#entrypoint) instruction.

### `workspace` (optional)

Similar to the Dockerfile [`WORKDIR`](https://docs.docker.com/reference/dockerfile/#workdir) instruction.

```yml
workspace: /path/to/workspace
```

### `copy` (optional)

Copies files into the **Shell-Cell** image.
Similar to the Dockerfile [`COPY`](https://docs.docker.com/reference/dockerfile/#workdir) instruction.

```yml
copy:
    - file1 .
    - file2 .
    - file3 file4 .
```

### `env` (optional)

Sets environment variables in the **Shell-Cell** image.
Similar to the Dockerfile [`ENV`](https://docs.docker.com/reference/dockerfile/#env) instruction.

Each item follows the list format `<KEY>=<VALUE>`:

```yml
env:
    - DB_HOST=localhost
    - DB_PORT=5432
    - DB_NAME=db
    - DB_DESCRIPTION="My Database"
```

### `build` (optional)

Will execute any commands to create a new layer on top of the current image,
during the image building process.
Similar to the Dockerfile [`RUN`](https://docs.docker.com/reference/dockerfile/#run) instruction.

```yml
build:
    - <command_1>
    - <command_2>
```

### `config` (optional)

Runtime configuration for the **Shell-Cell** container.
Unlike `build`, `copy`, and `workspace`, which affect the image building process,
`config` defines how the container behaves when it runs.

All `config` statements are optional.

Only the first `config` statement encountered in the target chain (starting from the entry point) is used.

```yml
config:
    mounts:
        - <host_path>:<container_absolute_path>
    ports:
        - "<host_port>:<container_port>"
```

#### `mounts`

Bind-mounts host directories into the running container.
Each mount item follows the format `<host_path>:<container_absolute_path>`.

- The **host path** can be relative (resolved relative to the `scell.yml` file location) or absolute.
  Relative host paths are canonicalized during compilation, so the referenced directory must exist.
- The **container path** must be an absolute path.

```yml
config:
    mounts:
        - ./src:/app/src
        - /data:/container/data
```

#### `ports`

Publishes container ports to the host.
Partially follows the [Docker Compose short form syntax](https://docs.docker.com/reference/compose-file/services/#ports).

Each item can be one of:

| Format | Description |
|---|---|
| `HOST_PORT:CONTAINER_PORT` | Map a specific host port to a container port |
| `HOST_IP:HOST_PORT:CONTAINER_PORT` | Map with a specific host IP and port |
| `HOST_IP::CONTAINER_PORT` | Bind to a host IP with a random host port |

Append `/tcp` or `/udp` to any format to specify the protocol (default: `tcp`).

```yml
config:
    ports:
        - "8080:80"
        - "127.0.0.1:9000:9000"
        - "6060:6060/udp"
```
