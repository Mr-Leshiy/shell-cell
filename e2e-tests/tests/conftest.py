import subprocess

import pytest

from scell import get_scell_bin


@pytest.fixture(autouse=True)
def stop_containers():
    yield
    scell_bin = get_scell_bin()
    result = subprocess.run([scell_bin, "stop"], check=False)
    assert result.returncode == 0, f"'scell stop' failed with exit code {result.returncode}"
