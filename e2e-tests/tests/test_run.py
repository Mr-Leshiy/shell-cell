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
    child.expect("/app", timeout=10)
    assert_scell_stop_session(child)


def test_scell_run_copy(scell) -> None:
    child = scell(args=["data"])

    assert_scell_prepare_session(child)
    child.sendline("cat copy_test.txt")
    child.expect("copy", timeout=10)
    child.expect("works!", timeout=10)
    assert_scell_stop_session(child)


def test_scell_run_env(scell) -> None:
    child = scell(args=["data"])

    assert_scell_prepare_session(child)
    child.sendline("echo $ENV_TEST")
    child.expect("env", timeout=10)
    child.expect("works!", timeout=10)
    assert_scell_stop_session(child)


def test_scell_run_build(scell) -> None:
    child = scell(args=["data"])

    assert_scell_prepare_session(child)
    child.sendline("cat build_test.txt")
    child.expect("build", timeout=10)
    child.expect("works!", timeout=10)
    assert_scell_stop_session(child)


def test_scell_run_mount(scell) -> None:
    child = scell(args=["data"])

    assert_scell_prepare_session(child)
    child.sendline("cat mnt/mount_test.txt")
    child.expect("mount", timeout=10)
    child.expect("works!", timeout=10)
    assert_scell_stop_session(child)

def test_scell_run_ports(scell) -> None:
    child = scell(args=["data"])

    assert_scell_prepare_session(child)
    child.sendline("python3 -m http.server 4321")
    child.expect("Serving", timeout=10)
    child.expect("HTTP", timeout=10)
    import requests
    resp = requests.get("http://0.0.0.0:4321", timeout=10)
    assert resp.status_code == 200
    # Send Ctrl-c to stop the python3 HTTP server
    child.send('\x03')
    assert_scell_stop_session(child)


def assert_scell_prepare_session(child):
    child.expect("'Shell-Cell' is up to date", timeout=5)
    child.expect("Starting 'Shell-Cell' session", timeout=PREPARE_SESSION_TIMEOUT)
    child.expect("root@", timeout=1)
    child.expect("/app#", timeout=1)


def assert_scell_stop_session(child):
    # Send Ctrl-D to the shell to end the session
    child.send('\x04')
    child.expect("Finished 'Shell-Cell' session", timeout=5)
    # scell shows "<Press any key to exit>" before quitting — send any key
    child.send(' ')
    assert_clean_exit(child)
