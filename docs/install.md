# Install and Configure `Shell-Cell`

`Shell-Cell` requires a running instance of either [Docker] or [Podman] daemon.
So firstly prepare and install [Docker] or [Podman] daemons.

To install `Shell-Cell` use cargo
```shell
cargo install --git https://github.com/Mr-Leshiy/shell-cell.git --locked
```

`Shell-Cell` requires a running instance of either [Docker] or [Podman] daemon.

> ⚠️ Theoretically it should work with [Podman], but that integration wasn't tested yet.
> So feel free to raise an issue if you found any issues with running it with [Podman] and we will try to fix it ASAP!

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
TODO