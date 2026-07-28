import os
import subprocess
import tempfile

from scell import assert_clean_exit, assert_scell_stop_session, spawn_scell


def test_init(spawn_scell) -> None:
    with tempfile.TemporaryDirectory() as tmpdir:
        scell = spawn_scell(args=["init", tmpdir])

        scell.expect("Created")
        assert_clean_exit(scell)

        # Start a Shell-Cell session from the initialized directory
        scell = spawn_scell(args=[tmpdir])
        scell.expect("'Shell-Cell' is up to date")
        scell.expect("Starting 'Shell-Cell' session", timeout=120)
        scell.expect("root@")
        scell.expect("/my_project#")

        # Stop the session
        assert_scell_stop_session(scell)
