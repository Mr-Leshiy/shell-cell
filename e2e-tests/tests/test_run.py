from dataclasses import dataclass

import pytest

from scell import assert_clean_exit, spawn_scell, SCell


@dataclass
class ScellEnv:
    data_dir: str
    port: int


ENVS = [
    ScellEnv("data/common", 4321),
    ScellEnv("data/from_docker", 4322),
]


@pytest.mark.parametrize("env", ENVS)
def test_scell_simple_run(spawn_scell, env: ScellEnv) -> None:
    scell = spawn_scell(args=[env.data_dir])

    assert_scell_prepare_session(scell)
    assert_scell_stop_session(scell)


@pytest.mark.parametrize("env", ENVS)
def test_scell_run_check_workspace(spawn_scell, env: ScellEnv) -> None:
    scell = spawn_scell(args=[env.data_dir])

    assert_scell_prepare_session(scell)
    scell.send("pwd")
    scell.expect("/app")
    assert_scell_stop_session(scell)


@pytest.mark.parametrize("env", ENVS)
def test_scell_run_copy(spawn_scell, env: ScellEnv) -> None:
    scell = spawn_scell(args=[env.data_dir])

    assert_scell_prepare_session(scell)
    scell.send("cat copy_test.txt")
    scell.expect("copy")
    scell.expect("works!")
    scell.send("cat cp/copy_test.txt")
    scell.expect("copy")
    scell.expect("works!")
    assert_scell_stop_session(scell)


@pytest.mark.parametrize("env", ENVS)
def test_scell_run_env(spawn_scell, env: ScellEnv) -> None:
    scell = spawn_scell(args=[env.data_dir])

    assert_scell_prepare_session(scell)
    scell.send("echo $ENV_TEST")
    scell.expect("env")
    scell.expect("works!")
    assert_scell_stop_session(scell)


@pytest.mark.parametrize("env", ENVS)
def test_scell_run_build(spawn_scell, env: ScellEnv) -> None:
    scell = spawn_scell(args=[env.data_dir])

    assert_scell_prepare_session(scell)
    scell.send("cat build_test.txt")
    scell.expect("build")
    scell.expect("works!")
    assert_scell_stop_session(scell)


@pytest.mark.parametrize("env", ENVS)
def test_scell_run_mount(spawn_scell, env: ScellEnv) -> None:
    scell = spawn_scell(args=[env.data_dir])

    assert_scell_prepare_session(scell)
    scell.send("cat mnt/mount_test.txt")
    scell.expect("mount")
    scell.expect("works!")
    assert_scell_stop_session(scell)


@pytest.mark.parametrize("env", ENVS)
def skip_test_scell_run_ports(spawn_scell, env: ScellEnv) -> None:
    scell = spawn_scell(args=[env.data_dir])

    assert_scell_prepare_session(scell)
    import requests
    resp = requests.get(f"http://0.0.0.0:{env.port}", timeout=30)
    assert resp.status_code == 200
    assert_scell_stop_session(scell)


def assert_scell_prepare_session(scell: SCell):
    scell.expect("'Shell-Cell' is up to date")
    scell.expect("Starting 'Shell-Cell' session", timeout=120)
    scell.expect("root@")
    scell.expect("/app#")


def assert_scell_stop_session(scell):
    # Send Ctrl-D to the shell to end the session
    scell.send('\x04')
    scell.expect("Finished 'Shell-Cell' session")
    # scell shows "<Press any key to exit>" before quitting — send any key
    scell.send(' ')
    assert_clean_exit(scell)
