"""Hold the selected toolchain cache mutation lock for a preparation child."""
import fcntl
import os
import subprocess
import sys
from pathlib import Path

lock = Path(sys.argv[1])
lock.parent.mkdir(parents=True, exist_ok=True)
with lock.open('a') as held:
    fcntl.flock(held, fcntl.LOCK_EX)
    env = {**os.environ, 'THINKTHEN_DUCKDB_CACHE_LOCK_HELD': str(lock)}
    result = subprocess.run(sys.argv[2:], env=env, check=False)
    sys.exit(result.returncode if result.returncode >= 0 else 128 - result.returncode)
