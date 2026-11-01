"""Share the pinned host Go runtime selected by the floor recipe."""
import os
from pathlib import Path
import re
import subprocess
import sys

from release_downloads import fetch, prepare_tool

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
        def archive():
            architecture = os.uname().machine
            system = {'linux': 'linux', 'darwin': 'darwin'}[sys.platform]
            architecture = {'x86_64': 'amd64', 'aarch64': 'arm64', 'arm64': 'arm64'}[architecture]
            filename = destination.name + '.' + system + '-' + architecture + '.tar.gz'
            return fetch('https://go.dev/dl/' + filename, TOOLS / 'sources' / filename)
        prepare_tool(destination, 'bin/go', archive)
        actual = subprocess.check_output([destination / 'bin/go', 'env', 'GOVERSION'],
                                         env=dict(os.environ, GOTOOLCHAIN='local'), text=True).strip()
        if actual != 'go' + version():
            raise ValueError('prepare the pinned Go floor compiler')
    os.environ['THINKTHEN_GO_HOME'] = str(destination)
