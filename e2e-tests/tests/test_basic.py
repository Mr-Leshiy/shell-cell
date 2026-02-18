from scell import scell

import pexpect


def test_scell_basic(scell) -> None:
    child = scell(args=["--version"])
    child.expect("shell-cell 1.0.1")
    child.expect(pexpect.EOF)
    child.close()
    assert child.exitstatus == 0
