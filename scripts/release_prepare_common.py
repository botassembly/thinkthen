"""Bounded downloads and prepared-tool execution for release preparation."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile

from release_downloads import fetch
from release_home import checked_home

ROOT = Path(__file__).resolve().parent.parent
TOOLS = checked_home() / '.local/share/thinkthen-build-tools'
PINS = json.loads((ROOT / 'scripts/release-tools.json').read_text())
def run(*args, **options):
    options.setdefault('cwd', ROOT)
    if options.get('capture_output'):
        return subprocess.run(list(map(str, args)), check=True, **options)
    with tempfile.TemporaryFile(mode='w+') as log:
        result = subprocess.run(list(map(str, args)), stdout=log, stderr=subprocess.STDOUT, **options)
        if result.returncode:
            log.seek(0)
            print('\n'.join(log.read().splitlines()[-30:]), file=sys.stderr)
            result.check_returncode()
        return result


def prepared(*args, **options):
    directory = options.pop('cwd', ROOT)
    return run('sh', '-eu', '-c', '. scripts/native-tools-env; cd "$1"; shift; exec "$@"',
               'release-prepare', directory, *args, **options)


