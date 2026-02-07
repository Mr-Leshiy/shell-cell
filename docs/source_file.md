# `Shell-Cell` Source File Reference

`Shell-Cell` builds your environment by reading a set of instructions from a `scell.yml` files.

`scell.yml` - is a [YAML](https://yaml.org) formatted file that contains everything needed to configure your session.
Here is a minimal functional example:
```yml
main:
  from: debian:bookworm
  workspace: workdir
  shell: /bin/bash
  hang: while true; do sleep 3600; done
```

`Shell-Cell` follows a strict logic when building your image.
It parses your target definitions into a Directed Linear Graph,
moving from your entry point (`main`) down to the base "bottom" target.

The actual image building process, on contrary, happens backwards.
Starts from the "bottom" target and works its way up to your entry point (`main`):
1. `bottom_target`
2. `target_3`
3. `target_2`
4. `target_1`
5. `main`

## `Shell-Cell` target

`Shell-Cell` are comprised of a series of target declarations and recipe definitions.

```yml
<target-name>:
    <recipe>
    ...
```

Inside each target, during the `Shell-Cell` image building process,
the instructions are executed in a specific, strict order:
1. `workspace`
2. `from`
3. `run`

### `from`

Similar to the Dockerfile [`FROM`](https://docs.docker.com/reference/dockerfile/#from) instruction,
it specifies the base of the `Shell-Cell` image.

It could be ither a plain image, or reference to other [`Shell-Cell` target](#shell-cell-target)

- Image with tag
```yml
from: <image>:<tag>
```

- `Shell-Cell` target reference
```yml
from: path/to/file+<taget_name>
```

### `shell`

A location to the shell, which would be abailable in the build image and running container.

Such shell would be used for a `Shell-Cell` session.

```yml
shell: /bin/bash
```

### `hang`

This instruction ensures your container stays active and doesn't exit immediately after it starts. This effectively transforms your `Shell-Cell` container into a persistent "shell server" that remains ready for you to jump in at any time.

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

Copies files into the `Shell-Cell` image.
Similar to the Dockerfile [`COPY`](https://docs.docker.com/reference/dockerfile/#workdir) instruction.

```yml
copy:
    - file1 .
    - file2 .
    - file3 file4 .
```

### `run` (optional)

Will execute any commands to create a new layer on top of the current image,
during the image building process.
Similar to the Dockerfile [`RUN`](https://docs.docker.com/reference/dockerfile/#run) instruction.

```yml
run:
    - <command_1>
    - <command_2>
```
