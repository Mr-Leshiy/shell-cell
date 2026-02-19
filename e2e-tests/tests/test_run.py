from dataclasses import dataclass

import pytest

from scell import assert_clean_exit, scell


@dataclass
class ScellEnv:
    data_dir: str
    port: int


ENVS = [
    ScellEnv("data/common", 4321),
    ScellEnv("data/from_docker", 4322),
]


@pytest.mark.parametrize("env", ENVS)
def test_scell_simple_run(scell, env: ScellEnv) -> None:
    child = scell(args=[env.data_dir])

    assert_scell_prepare_session(child)
    assert_scell_stop_session(child)


@pytest.mark.parametrize("env", ENVS)
def test_scell_run_check_workspace(scell, env: ScellEnv) -> None:
    child = scell(args=[env.data_dir])

    assert_scell_prepare_session(child)
    child.sendline("pwd")
    child.expect("/app")
    assert_scell_stop_session(child)


@pytest.mark.parametrize("env", ENVS)
def test_scell_run_copy(scell, env: ScellEnv) -> None:
    child = scell(args=[env.data_dir])

    assert_scell_prepare_session(child)
    child.sendline("cat copy_test.txt")
    child.expect("copy")
    child.expect("works!")
    child.sendline("cat cp/copy_test.txt")
    child.expect("copy")
    child.expect("works!")
    assert_scell_stop_session(child)


@pytest.mark.parametrize("env", ENVS)
def test_scell_run_env(scell, env: ScellEnv) -> None:
    child = scell(args=[env.data_dir])

    assert_scell_prepare_session(child)
    child.sendline("echo $ENV_TEST")
    child.expect("env")
    child.expect("works!")
    assert_scell_stop_session(child)


@pytest.mark.parametrize("env", ENVS)
def test_scell_run_build(scell, env: ScellEnv) -> None:
    child = scell(args=[env.data_dir])

    assert_scell_prepare_session(child)
    child.sendline("cat build_test.txt")
    child.expect("build")
    child.expect("works!")
    assert_scell_stop_session(child)


@pytest.mark.parametrize("env", ENVS)
def test_scell_run_mount(scell, env: ScellEnv) -> None:
    child = scell(args=[env.data_dir])

    assert_scell_prepare_session(child)
    child.sendline("cat mnt/mount_test.txt")
    child.expect("mount")
    child.expect("works!")
    assert_scell_stop_session(child)


@pytest.mark.parametrize("env", ENVS)
def skip_test_scell_run_ports(scell, env: ScellEnv) -> None:
    child = scell(args=[env.data_dir])

    assert_scell_prepare_session(child)
    import requests
    resp = requests.get(f"http://0.0.0.0:{env.port}", timeout=30)
    assert resp.status_code == 200
    assert_scell_stop_session(child)


def assert_scell_prepare_session(child):
    child.expect("'Shell-Cell' is up to date")
    child.expect("Starting 'Shell-Cell' session", timeout=120)
    child.expect("root@")
    child.expect("/app#")


def assert_scell_stop_session(child):
    # Send Ctrl-D to the shell to end the session
    child.send('\x04')
    child.expect("Finished 'Shell-Cell' session")
    # scell shows "<Press any key to exit>" before quitting — send any key
    child.send(' ')
    assert_clean_exit(child)
