"""Prepare package validators separately from the offline registry staging."""
from pathlib import Path
import shutil
import sys
from release_workflow import run, prepared

def prepare():
    if not shutil.which('uv'):
        from release_prepare_common import PINS
        run(sys.executable, '-m', 'pip', 'install', '--disable-pip-version-check', 'uv==' + PINS['uv'])
    tools = Path.home() / '.local/share/thinkthen-build-tools/registry-validators/python'
    prepared('uv', 'venv', '--allow-existing', '--python', '3.13', tools)
    prepared('uv', 'pip', 'install', '--python', tools / 'bin/python', 'twine==6.2.0')
