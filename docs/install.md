# Install and Configure **Shell-Cell**

**Shell-Cell** requires a running instance of either [Docker] or [Podman] daemon.
You have to firstly prepare and install [Docker] or [Podman], for your choice.

### Install

- Build for Unix 
<!-- cspell: disable -->
```shell
curl -fsSL https://github.com/Mr-Leshiy/shell-cell/releases/latest/download/shell-cell-installer.sh | sh
```
<!-- cspell: enable -->

- Build from source (any platform)
```shell
cargo install shell-cell --locked
```

**Shell-Cell** requires a running instance of either [Docker] or [Podman] daemon.

### UNIX socket configuration (UNIX)

To interact with the [Docker] or [Podman] daemon
**Shell-Cell** uses a UNIX socket connection on UNIX based operating systems.
The URL of this socket is read from the `DOCKER_HOST` environment variable.
Before running **Shell-Cell**, you should set the proper value of `DOCKER_HOST`

```shell
export DOCKER_HOST="<unix_socket_url>"
```


To find out the `*.sock` URL you could run
- for [Docker]
```shell
docker context inspect | grep sock
```
- for [Podman]
When you are starting a podman virtual machine `podman machine start`, it prints it in stdout, e.g.
<!-- cspell: disable -->
```shell
Starting machine "podman-machine-default"
API forwarding listening on: /var/folders/5m/2c6173tx1nb6m5mnkjz27gk00000gn/T/podman/podman-machine-default-api.sock

The system helper service is not installed; the default Docker API socket
address can't be used by podman. If you would like to install it, run the following commands:

        sudo /opt/homebrew/Cellar/podman/5.8.0/bin/podman-mac-helper install
        podman machine stop; podman machine start

You can still connect Docker API clients by setting DOCKER_HOST using the
following command in your terminal session:

        export DOCKER_HOST='unix:///var/folders/5m/2c6173tx1nb6m5mnkjz27gk00000gn/T/podman/podman-machine-default-api.sock'

Machine "podman-machine-default" started successfully
```
<!-- cspell: enable -->


[Podman]: https://podman.io
[Docker]: https://www.docker.com
