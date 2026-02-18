import sys

from scell import assert_clean_exit, scell

PREPARE_SESSION_TIMEOUT = 60


def test_scell_simple_run(scell) -> None:
    child = scell(args=["data"])

    assert_scell_prepare_session(child)
    assert_scell_stop_session(child)


def test_scell_run_check_workspace(scell) -> None:
    child = scell(args=["data"])
    
    assert_scell_prepare_session(child)
    child.sendline("pwd")
    child.expect("/app", timeout = 1)
    assert_scell_stop_session(child)

def assert_scell_prepare_session(child):
    child.expect("'Shell-Cell' is up to date", timeout = 5)
    # Waite until the Shell-Cell session would be ready
    # Final step before immediately start Shell-Cell session
    child.expect("Starting 'Shell-Cell' session", timeout = PREPARE_SESSION_TIMEOUT)

def assert_scell_stop_session(child):
    # Send Ctrl-D to the shell to end the session
    child.send('\x04')
    child.expect("Finished 'Shell-Cell' session", timeout=5)
    # scell shows "<Press any key to exit>" before quitting — send any key
    child.send(' ')
    assert_clean_exit(child)