"""Share the pinned host Go runtime selected by the floor recipe."""
import os
from pathlib import Path
import re
import subprocess
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
    parent = TOOLS / 'gate-go' if sys.platform == 'linux' else TOOLS
    return parent / ('go' + version())


def command():
    if 'THINKTHEN_GO_HOME' in os.environ:
        return str(Path(os.environ['THINKTHEN_GO_HOME']) / 'bin/go')
    return str(home() / 'bin/go') if sys.platform in ('linux', 'darwin') else 'go'


def prepare():
    import fcntl
    # Keep the immutable native compiler image while updating its host runtime.
    destination = home()
    destination.parent.mkdir(parents=True, exist_ok=True)
    with (destination.parent / (destination.name + '.lock')).open('a') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX)
        if not destination.exists():
            architecture = os.uname().machine
            system = {'linux': 'linux', 'darwin': 'darwin'}[sys.platform]
            architecture = {'x86_64': 'amd64', 'aarch64': 'arm64', 'arm64': 'arm64'}[architecture]
            filename = destination.name + '.' + system + '-' + architecture + '.tar.gz'
            archive = fetch('https://go.dev/dl/' + filename, TOOLS / 'sources' / filename)
            with tempfile.TemporaryDirectory(dir=destination.parent) as temporary:
                with tarfile.open(archive) as source:
                    source.extractall(temporary, filter='data')
                (Path(temporary) / 'go').rename(destination)
        actual = subprocess.check_output([destination / 'bin/go', 'env', 'GOVERSION'],
                                         env=dict(os.environ, GOTOOLCHAIN='local'), text=True).strip()
        if actual != 'go' + version():
            raise ValueError('prepare the pinned Go floor compiler')
    os.environ['THINKTHEN_GO_HOME'] = str(destination)
