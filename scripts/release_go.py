"""Share the prepared Linux Go runtime selected by the floor recipe."""
import os
from pathlib import Path
import re
import sys
import tarfile
import tempfile

from release_downloads import fetch

ROOT = Path(__file__).resolve().parent.parent
TOOLS = Path.home() / '.local/share/thinkthen-build-tools'


def version():
    return re.search(r'go([0-9.]+)\.linux-amd64\.tar\.gz',
                     (ROOT / 'scripts/floors/Dockerfile').read_text())[1]


def home():
    return TOOLS / 'gate-go' / ('go' + version())


def command():
    return str(home() / 'bin/go') if sys.platform == 'linux' else 'go'


def prepare():
    import fcntl
    # Keep the immutable native compiler image while updating its host runtime.
    destination = home()
    destination.parent.mkdir(parents=True, exist_ok=True)
    with (destination.parent / (destination.name + '.lock')).open('a') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX)
        if not destination.exists():
            architecture = os.uname().machine
            filename = destination.name + '.linux-' + {'x86_64': 'amd64', 'aarch64': 'arm64'}[architecture] + '.tar.gz'
            archive = fetch('https://go.dev/dl/' + filename, TOOLS / 'sources' / filename)
            with tempfile.TemporaryDirectory(dir=destination.parent) as temporary:
                with tarfile.open(archive) as source:
                    source.extractall(temporary, filter='data')
                (Path(temporary) / 'go').rename(destination)
    os.environ['THINKTHEN_GO_HOME'] = str(destination)
