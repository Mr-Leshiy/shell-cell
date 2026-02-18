from scell import assert_clean_exit, scell


def test_scell_basic(scell) -> None:
    child = scell(args=["--version"])
    child.expect("shell-cell 1.0.1")

    assert_clean_exit(child)
