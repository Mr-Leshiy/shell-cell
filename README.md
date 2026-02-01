# Shell-Cell

### Docker configuration (UNIX)

To run and work with the Shell-Cell, i

To locate the `docker.sock` file you could run
```shell
docker context inspect
```

And set the env var `DOCKER_HOST` with the found path location.