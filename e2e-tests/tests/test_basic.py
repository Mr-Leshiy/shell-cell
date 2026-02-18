from scell_conf import scell_bin, SCELL_WINDOWN_HEIGHT, SCELL_WINDOWN_WIDTH

import pexpect


def test_scell_basic(scell_bin: str) -> None:
    child = pexpect.spawn(
        scell_bin,
        args=["--version"],
        dimensions=(SCELL_WINDOWN_HEIGHT, SCELL_WINDOWN_WIDTH),
        timeout=10,
    )
    child.expect("shell-cell 1.0.0")
    
    child.close()
    assert child.exitstatus == 0
