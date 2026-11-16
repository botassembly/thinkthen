"""Shared by the release job scripts: prepared inputs and job identity."""
import json
import os
from pathlib import Path

TABLE = Path(__file__).resolve().parent.parent / 'prepared.json'
TOOLS = '.local/share/thinkthen-build-tools'


class MissingInput(Exception):
    pass


def prepared(name):
    """Return one row's value. A tool path resolves under this machine's tools folder."""
    table = json.loads(TABLE.read_text())
    if name not in table:
        raise MissingInput('prepared input ' + name + ' is not in prepared.json')
    row = table[name]
    if row['kind'] not in ('path', 'location'):
        return row['value']
    path = Path.home() / TOOLS / row['value']
    if row['kind'] == 'path' and not path.exists():
        raise MissingInput('prepared input ' + name + ' is missing: ' + str(path))
    return str(path)


def job_id():
    return os.environ['PM_JOB_ID']
