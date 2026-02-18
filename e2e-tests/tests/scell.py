import os

import pytest
import pexpect

SCELL_WINDOWN_WIDTH = 800
SCELL_WINDOWN_HEIGHT = 600

@pytest.fixture(scope="session")
def scell():
    scell_bin = os.environ.get("SCELL_BIN")
    assert scell_bin, "Set the 'SCELL_BIN' env var with the path to the 'scell' binary on your machine"
    
    def spawn_scell(args: list[str], timeout: int = 10):
        scell_process = pexpect.spawn(
            scell_bin,
            args=args,
            dimensions=(SCELL_WINDOWN_HEIGHT, SCELL_WINDOWN_WIDTH),
            timeout=timeout,
        )
        return scell_process

    return spawn_scell


