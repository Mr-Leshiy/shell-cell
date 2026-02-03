# Features Roadmap

- [X] Recursively parse `Cells` based on the `SCell` files, forming a linked list which would be executed in the reverse order than it was added.
- [X] Add `copy` statement.
- [X] Add `workspace` statement.
- [ ] Add "a new updates" notification message at the start of the application.
- [X] Process global `Shell-Cell` file  `~/.scell/global.yml`.
- [ ] Add docker-compose like configuration to each Shell-Cell.
- [ ] Integrate metadata information into the images.
- [ ] CLI features e.g. `ls`, `rm`.
- [ ] Detect cycle dependencies during `SCell::compile` step.
- [ ] After stopping the session stop the container, dont remove, just stop.
- [ ] Use BuildKit cache feature, to cache each Cell individually.
- [ ] Allow Github for the `from` statement `Shell-Cell` file location.
