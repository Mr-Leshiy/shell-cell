
# **Shell-Cell** CLI Reference

Now that you’ve [installed and configured](./install.md) **Shell-Cell**, you’re ready to launch your very first session!

To get started, create a blueprint `scell.yml` file in your project directory.
This file defines the environment your shell will live in.
(For a deep dive into the blueprint specification, check out the [Blueprint Guide](./blueprint.md)).

**Example `scell.yml`:**
```yml
main:
  from: debian:bookworm
  workspace: workdir
  shell: /bin/bash
  hang: while true; do sleep 3600; done
```

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

## ❓ Need more help ? 

If you want to explore the full list of commands, flags, and capabilities, our built-in help menu is always there for you:
```shell
scell --help
```
