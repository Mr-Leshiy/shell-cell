# e2e-tests

End-to-end tests for the `scell` binary. They spawn the real binary via a PTY using [pexpect](https://pexpect.readthedocs.io).

## Requirements

- [uv](https://docs.astral.sh/uv/)
- A built `scell` binary — run `cargo build` from the repo root first

## Running

```bash
SCELL_BIN=/path/to/scell uv run pytest
```

`SCELL_BIN` must point to the `scell` binary. There is no default — the tests will fail immediately with a clear message if it is not set.
