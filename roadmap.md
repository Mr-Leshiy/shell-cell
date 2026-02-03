# Features Roadmap

- [X] Recursively parse `Cells` based on the `SCell` files, forming a linked list which would be executed in the reverse order than it was added.
- [ ] Add `copy` statement.
- [ ] Process global `scell` source file  `~/.scell/global.yml`.
- [ ] Add docker-compose like configuration to each Shell-Cell.
- [ ] Integrate metadata information into the images.
- [ ] CLI features e.g. `ls`, `rm`.
- [ ] After stopping the session stop the container, dont remove, just stop.
- [ ] Use BuildKit cache feature, to cache each Cell individually.
