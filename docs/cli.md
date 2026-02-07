
# `Shell-Cell` CLI Reference

Now that you’ve [installed and configured](./install.md) **Shell-Cell**, you’re ready to launch your very first session!

To get started, place a `scell.yml` file in your project directory.
This file defines the environment your shell will live in.
(For a deep dive into the source file specification, check out the [Source File Guide](./source_file.md)).

**Example `scell.yml`:**
```yml
rust-base:
  from: rust:1.93-trixie
  shell: /bin/bash
  hang: while true; do sleep 3600; done

main:
  from: +rust-base
  workspace: my_project
```

Once your file is ready, simply open your terminal in that directory and run:
```shell
scell
```

That’s it, simple as that!
`Shell-Cell` will automatically look for a file named `scell.yml` in your current location and start the `Shell-Cell` session on the spot.


If your configuration file is located elsewhere and you don't want to change directories, you can point `Shell-Cell` directly to it:

```shell
scell ./path/to/the/source/file/scell.yml
```

## ❓ Need more help ? 

If you want to explore the full list of commands, flags, and capabilities, our built-in help menu is always there for you:
```shell
scell --help
```
