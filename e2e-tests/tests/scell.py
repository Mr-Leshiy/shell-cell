import os
from typing import Any

import pexpect
import pytest

SCELL_WINDOWN_WIDTH = 800
SCELL_WINDOWN_HEIGHT = 600


class SCell:
    def __init__(self, process: pexpect.spawn) -> None:
        self._process = process
        # fout = open('mylog.txt','wb')
        # self._process.logfile = fout

    @property
    def exitstatus(self) -> int | None:
        return self._process.exitstatus

    def expect(self, pattern: Any, timeout: int | None = None) -> int:
        if timeout is not None:
            return self._process.expect(pattern, timeout=timeout)
        return self._process.expect(pattern)

    def send(self, s: str) -> int:
        return self._process.send(f"{s}\r")

    def close(self) -> None:
        self._process.close()


def assert_clean_exit(child: SCell) -> None:
    child.expect(pexpect.EOF, timeout=1)
    child.close()
    assert child.exitstatus == 0


@pytest.fixture(scope="session")
def spawn_scell():
    scell_bin = os.environ.get("SCELL_BIN")
    assert scell_bin, "Set the 'SCELL_BIN' env var with the path to the 'scell' binary on your machine"

    def spawn_scell(args: list[str], timeout: int = 10) -> SCell:
        scell_process = pexpect.spawn(
            scell_bin,
            args=args,
            dimensions=(SCELL_WINDOWN_HEIGHT, SCELL_WINDOWN_WIDTH),
            timeout=timeout,
        )
        return SCell(scell_process)

    return spawn_scell
