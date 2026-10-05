"""Hold the toolchain mutation lock; forward only declared preparation settings."""
import fcntl
import os
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / 'conformance/children'))
from children import CARGO, child_env  # noqa: E402

lock = Path(sys.argv[1])
lock.parent.mkdir(parents=True, exist_ok=True)
with lock.open('a') as held:
    fcntl.flock(held, fcntl.LOCK_EX)
    settings = {name: value for name in ('THINKTHEN_TOOLCHAINS', 'THINKTHEN_DUCKDB_VERSION')
                if (value := os.environ.get(name)) is not None}
    env = child_env(CARGO + ('LANG', 'LC_ALL', 'TMPDIR', 'UV_CACHE_DIR', 'UV_PYTHON_INSTALL_DIR'),
                    THINKTHEN_DUCKDB_CACHE_LOCK_HELD=str(lock), **settings)
    result = subprocess.run(sys.argv[2:], env=env, check=False)
    sys.exit(result.returncode if result.returncode >= 0 else 128 - result.returncode)
