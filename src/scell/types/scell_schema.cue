// Target is a single named build unit in the blueprint.
// Targets are chained via the `from` field to compose layered environments.
//
// Inside a target, instructions are executed in this fixed order during image build:
// 1. workspace
// 2. from/from_image/from_docker
// 3. env
// 4. copy 
// 5. build
#Target: {}

// Similar to the Dockerfile [`FROM`](https://docs.docker.com/reference/dockerfile/#from) instruction,
// these statements specify the base of the **Shell-Cell** layer.
//
// Only one of these statements must be present in the **Shell-Cell** target definition.
#Target: {
    // References another **Shell-Cell** target, resolved recursively.
    // Use "+<target_name>" to reference a target in the same blueprint file,
    // or "path/to/dir+<target_name>" to reference a target in another blueprint file.
    // Must eventually resolve to a target that has from_image or from_docker.
    from: string
} | {
    // Specifies a Docker registry image as the base layer for this target.
    // Format: "<image>:<tag>" (e.g. "debian:bookworm", "ubuntu:22.04").
    // Equivalent to the Dockerfile FROM instruction.
    from_image: string
} | {
    // Specifies a path to a local Dockerfile to use as the base layer.
    // The path is resolved relative to the blueprint's file location.
    from_docker: string
}

// Despite the fact that for a specific target definition these fields could be ommited,
// they must appear at least once in the target chain.
// Only the first statement encountered in the target chain (starting from the entry point) is used.
#Target: {
    // The path to the shell binary inside the built image.
    // This shell is used for interactive **Shell-Cell** sessions.
    // Only the first occurrence in the target chain (from the entry point) takes effect.
    // Example: "/bin/bash", "/bin/sh", "/usr/bin/zsh"
    shell?: string

    // A shell command that keeps the container running indefinitely.
    // It is set as the container ENTRYPOINT and must never exit.
    // Only the first occurrence in the target chain (from the entry point) takes effect.
    // Recommended value: "while true; do sleep 3600; done"
    hang?: string
}


#Target:  {
    // Sets the working directory inside the image.
    // Equivalent to the Dockerfile WORKDIR instruction.
    // Applied as the first step during the image build for this target.
    workspace?: string

    // A list of environment variables to set in the image.
    // Equivalent to the Dockerfile ENV instruction.
    // Each item must follow the format "KEY=VALUE".
    // Values containing spaces must be quoted (e.g. 'DB_DESC="My Database"').
    env?: [...string]

    // A list of file copy instructions for the image.
    // Equivalent to the Dockerfile COPY instruction.
    // Each item is a space-separated string of one or more source paths followed
    // by a destination path (e.g. "file1 .", "src/ dest/", "a b destdir/").
    copy?: [...string]

    // A list of shell commands to run during the image build process.
    // Each command creates a new layer on top of the current image.
    // Equivalent to the Dockerfile RUN instruction.
    build?: [...string]

    // Runtime configuration for the container.
    // Unlike build/copy/workspace, these settings affect how the container runs,
    // not how the image is built.
    // Only the first config block encountered in the target chain is used.
    config?: #Config
}

// Config defines runtime behaviour of the **Shell-Cell** container.
#Config: {
    // mounts is a list of bind-mount declarations for the running container.
    // Each item follows the format "<host_path>:<container_absolute_path>".
    // The host path may be relative (resolved relative to scell.yml) or absolute.
    //   - Relative host paths are canonicalized at compile time and must already exist.
    //   - The container path must be an absolute path.
    // Examples: "./src:/app/src", "/data:/container/data"
    mounts?: [...string]

    // ports is a list of port-mapping declarations for the running container.
    // Partially follows Docker Compose short-form syntax.
    // Supported formats (optional "/tcp" or "/udp" suffix, default is tcp):
    //   "HOST_PORT:CONTAINER_PORT"                — map a specific host port
    //   "HOST_IP:HOST_PORT:CONTAINER_PORT"        — map with a specific host IP
    //   "HOST_IP::CONTAINER_PORT"                 — random host port on a given IP
    // Examples: "8080:80", "127.0.0.1:9000:9000", "6060:6060/udp"
    ports?: [...string]
}

// defining the final contraint 
{
  [string]: #Target
}
