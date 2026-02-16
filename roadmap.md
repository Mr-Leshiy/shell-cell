# Features Roadmap

## Version `1.0.0`
- [X] Recursively parse `Cells` based on the `SCell` files, forming a linked list which would be executed in the reverse order than it was added.
- [X] Add `copy` statement.
- [X] Add `workspace` statement.
- [X] Add "a new updates" notification message at the start of the application.
- [X] Add some basic docker-compose like configuration to each Shell-Cell.
- [X] Integrate metadata information into the images.
- [X] `ls` CLI feature.
- [X] `cleanup` CLI feature.
- [X] `stop` CLI feature
- [X] Detect cycle dependencies during `SCell::compile` step.
- [X] Manage distribution channels for the `scell` binary, so it would be possible to install it easily on different platforms.
- [ ] Test how it works with `Podman`

## Version `2.0.0`

- [ ] Performance improvements
- [ ] BuildKit usage.
- [ ] Allow Github for the `from` statement `Shell-Cell` file location.
- [ ] `Dockerfile` support (???)
- [ ] `Earthfile` support (???)
- [ ] Dager files support (???)
- [ ] UI themes 
