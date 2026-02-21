import os
import subprocess
import tempfile

from scell import assert_clean_exit, scell


def test_init(scell) -> None:
    with tempfile.TemporaryDirectory() as tmpdir:
        child = scell(args=["init", tmpdir])

        child.expect("Created")
        assert_clean_exit(child)

        # Start a Shell-Cell session from the initialized directory
        child = scell(args=[tmpdir])
        child.expect("'Shell-Cell' is up to date")
        child.expect("Starting 'Shell-Cell' session", timeout=120)
        child.expect("root@")
        child.expect("/my_project#")

        # Stop the session
        # Send Ctrl-D to the shell to end the session
        child.send('\x04')
        child.expect("Finished 'Shell-Cell' session")
        # scell shows "<Press any key to exit>" before quitting — send any key
        child.send(' ')
        assert_clean_exit(child)
