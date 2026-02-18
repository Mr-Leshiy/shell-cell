import os

import pytest

SCELL_WINDOWN_WIDTH = 800
SCELL_WINDOWN_HEIGHT = 600

@pytest.fixture(scope="session")
def scell_bin() -> str:
    path = os.environ.get("SCELL_BIN")
    assert path, "Set the SCELL_BIN env var with the path to the 'scell' binary on your machine"
    return path
