# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Shell-Cell (`scell`) is a lightweight CLI tool that creates instant, isolated, and reproducible containerized development shell sessions. It uses YAML blueprint files (`scell.yml`) to define layered environments (called "targets") that compile into Docker images and run as persistent "shell server" containers.

**Key Concept**: Unlike standard containers that run a task and exit, Shell-Cell containers use a `hang` instruction to stay alive as background servers, allowing multiple shell sessions to attach to a warm, ready environment.

## Development Commands

### Building
```bash
cargo build
```

### Testing
```bash
# Run all tests (must use single thread)
cargo test -- --test-threads 1

# Run specific test
cargo test <test_name> -- --test-threads 1
```

### Linting & Formatting
```bash
# Format code (requires nightly toolchain)
cargo +nightly fmt

# Check formatting
cargo +nightly fmt --check

# Run clippy
cargo clippy --all-targets
```

### Running the Binary
```bash
# Build and run
cargo run

# Run with a specific scell.yml path
cargo run -- /path/to/directory

# List containers
cargo run -- ls

# Stop containers
cargo run -- stop

# Enable verbose mode
cargo run -- -v
```

The binary name is `scell` (defined in Cargo.toml).

## Architecture

### Core Pipeline: Blueprint → Image → Container → Shell

1. **Parser** (`src/scell/parser/`): Parses `scell.yml` files into `SCellFile` and `TargetStmt` structures
2. **Compiler** (`src/scell/compile/`): Resolves target references recursively, building a chain of `Link`s (Root + Nodes) to form an `SCell`
3. **Image Builder** (`src/scell/image.rs`): Generates a Dockerfile and tar artifact from the `SCell` link chain
4. **BuildKit** (`src/buildkit/`): Uses Bollard (Docker client) to build images and manage containers
5. **PTY** (`src/pty.rs`): Attaches an interactive shell session to the running container

### Key Data Structures

- **`SCell`** (`src/scell/mod.rs`): The compiled representation of a Shell-Cell, containing:
  - `links: Vec<Link>` - Chain from entry point down to base image
  - `shell: ShellStmt` - The shell binary path (e.g., `/bin/bash`)
  - `hang: String` - Command to keep container alive
  - `config: Option<ConfigStmt>` - Runtime configuration (mounts, etc.)

- **`Link`**: Either a `Root(ImageDef)` (base Docker image) or a `Node` (target with workspace, copy, build instructions)

- **Target Resolution**: Starting from entry point (`main` by default), the compiler follows `from` statements recursively:
  - `from: debian:bookworm` → Root image
  - `from: path/to/dir+target_name` → Reference to another target (potentially in another `scell.yml`)

- **Build Order**: Targets are parsed from entry→root, but Dockerfile generation happens in reverse (root→entry)

### Target Instructions (Blueprint Syntax)

Executed in strict order during image building:
1. `workspace` - Sets working directory (like `WORKDIR`)
2. `from` - Base image or target reference (like `FROM`)
3. `copy` - Copies files into image (like `COPY`)
4. `build` - Runs commands during build (like `RUN`)

Special instructions (only first encountered is used):
- `shell` - Defines shell binary for interactive sessions
- `hang` - Keeps container alive (becomes `ENTRYPOINT`)
- `config` - Runtime configuration (mounts, etc.)

### Error Handling

The project uses a custom error system (`src/error/`):
- **`UserError`**: Errors caused by user input (bad config, missing files, etc.) - show clean messages
- **`Report`**: Accumulates multiple errors before returning
- Traits: `WrapUserError`, `OptionUserError` - Convert stdlib errors to UserError
- Verbose mode (`-v`) shows full backtraces for non-user errors

### Container Naming

Containers are named `scell-<hash>` where `<hash>` is a MetroHash64 of the entire `SCell` structure. This ensures the same blueprint always produces the same container name.

Images are tagged with metadata labels:
- `scell-name`: Target name
- `scell-location`: Absolute path to the `scell.yml` file

## Linting Rules

This project has **strict linting** (see Cargo.toml lints):
- `unwrap_used`, `expect_used`, `panic`, `todo`, `unimplemented`, `unreachable` are all **denied**
- Use `?` and proper error handling instead
- `indexing_slicing` is denied - use `.get()` instead
- Clippy pedantic is enabled at deny level
- Missing docs in public items triggers warnings (but private items are allowed)

## Important Patterns

### Home Directory
```rust
// Use the provided helper
scell_home_dir() // Returns ~/.scell/
```

### Blueprint File Location
The CLI accepts a directory path, not a file path. It always looks for `scell.yml` in that directory.

### Docker Integration
All Docker operations go through `BuildKitD` (`src/buildkit/mod.rs`), which wraps the Bollard client. The project uses async/await with Tokio runtime.

## Testing

Tests use `test-case` crate for parameterized tests. Compilation tests are in `src/scell/compile/tests/` with subdirectories:
- `ok/` - Valid blueprint files that should compile successfully
- `err/` - Invalid blueprint files that should produce specific errors

**Critical**: Tests MUST run with `--test-threads 1` due to Docker state dependencies.
