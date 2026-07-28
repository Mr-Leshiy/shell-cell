import os
import subprocess
from typing import Any

import pexpect
import pytest

SCELL_WINDOWN_WIDTH = 800
SCELL_WINDOWN_HEIGHT = 600

# 'tmux'-style command mode prefix
CTRL_B = "\x02"


class SCell:
    def __init__(self, process: pexpect.spawn) -> None:
        self._process = process
        # fout = open('mylog.txt','wb')
        # self._process.logfile = fout

    @property
    def exitstatus(self) -> int | None:
        return self._process.exitstatus

    def expect(self, pattern: Any, timeout: int = 30) -> int:
        return self._process.expect(pattern, timeout=timeout)

    def send(self, s: str) -> int:
        """Send a command line to the shell running inside the container."""
        return self._process.send(f"{s}\r")

    def send_key(self, key: str) -> int:
        """Send a raw keystroke to 'scell' itself, without a trailing newline."""
        return self._process.send(key)

    def close(self) -> None:
        self._process.close()


def get_scell_bin() -> str:
    scell_bin = os.environ.get("SCELL_BIN")
    assert scell_bin, "Set the 'SCELL_BIN' env var with the path to the 'scell' binary on your machine"
    return scell_bin


def assert_clean_exit(child: SCell) -> None:
    child.expect(pexpect.EOF, timeout=1)
    child.close()
    assert child.exitstatus == 0


def assert_scell_stop_session(scell: SCell) -> None:
    # Closing a session is a 'tmux'-style two-step flow:
    # 'Ctrl-B' enters the command mode, 'd' detaches and closes the session
    scell.send_key(CTRL_B)
    scell.send_key("d")
    scell.expect("Finished 'Shell-Cell' session")
    # scell shows "<Press any key to exit>" before quitting — send any key
    scell.send_key(" ")
    assert_clean_exit(scell)



@pytest.fixture(scope="session")
def spawn_scell():
    scell_bin = get_scell_bin()

    def spawn_scell(args: list[str], timeout: int = 10) -> SCell:
        scell_process = pexpect.spawn(
            scell_bin,
            args=args,
            dimensions=(SCELL_WINDOWN_HEIGHT, SCELL_WINDOWN_WIDTH),
            timeout=timeout,
        )
        return SCell(scell_process)

    return spawn_scell
