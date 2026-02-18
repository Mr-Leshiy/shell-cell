from scell import scell, SCELL_WINDOWN_HEIGHT, SCELL_WINDOWN_WIDTH

import pexpect


def test_scell_basic(scell) -> None:
    child = scell(args=["--version"])
    child.expect("shell-cell 1.0.1")
    child.close()
    assert child.exitstatus == 0
