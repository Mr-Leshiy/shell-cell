from scell import assert_clean_exit, spawn_scell


def test_scell_basic(spawn_scell) -> None:
    scell = spawn_scell(args=["--version"])
    scell.expect("shell-cell 1.4.0")

    assert_clean_exit(scell)
