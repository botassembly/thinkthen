#!/bin/sh
# Shared loader and private database lifecycle for the site's literal SQL examples.
set -eu
if [ "${1:-}" != --database ]; then
    exec bash run.sh --database duckdb "$PWD/show.sql" ../../..
fi
shift
exec python3 -B - "$@" <<'PY'
import csv
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import uuid

dialect, shown, root = sys.argv[1:]
root = Path(root).resolve()
shown = Path(shown)
packages = Path(os.environ.get('SITE_PACKAGES', root / 'target/site-packages')).resolve()
sql = shown.read_text()
env = dict(os.environ)
env.setdefault('THINKTHEN_BACKEND', 'stub')


def run(args, text=None, **options):
    result = subprocess.run([str(arg) for arg in args], input=text, text=True,
                            env=env, **options)
    if result.returncode:
        if result.stdout:
            sys.stdout.write(result.stdout)
        if result.stderr:
            sys.stderr.write(result.stderr)
        result.check_returncode()
    return result


# Root hands only this private directory to nobody, outside its isolated 0700 home.
pg_parent = '/tmp' if dialect == 'postgres' and os.getuid() == 0 else None
with tempfile.TemporaryDirectory(prefix='thinkthen-site-sql-', dir=pg_parent) as scratch:
    work = Path(scratch)
    if dialect == 'duckdb':
        extension = packages / 'duckdb/thinkthen.duckdb_extension'
        if not extension.is_file():
            raise FileNotFoundError(extension)
        # Resolve the installation placeholder, leaving the shown query unchanged.
        sql = sql.replace('/absolute/path/to/thinkthen.duckdb_extension', str(extension))
        result = run([packages / 'bin/duckdb', '-unsigned', '-batch', '-bail', '-json', ':memory:'],
                     sql, capture_output=True)
    elif dialect == 'sqlite':
        extension = packages / 'sqlite/thinkthen_sqlite.so'
        if not extension.is_file():
            raise FileNotFoundError(extension)
        (work / 'thinkthen_sqlite.so').symlink_to(extension)
        result = run([packages / 'bin/sqlite3', '-batch', '-bail', ':memory:'],
                     sql, cwd=work, capture_output=True)
    else:
        pg = Path((packages / 'postgres/bindir').read_text().strip())
        data = work / 'data'
        as_user = []
        if os.getuid() == 0:
            shutil.chown(work, user='nobody')
            as_user = ['runuser', '-u', 'nobody', '--']
        run([*as_user, pg / 'initdb', '-D', data, '-U', 'postgres', '--auth=trust',
             '--no-locale', '-E', 'UTF8'], stdout=subprocess.DEVNULL)
        options = (f"-k {work} -c listen_addresses='' "
                   f"-c extension_control_path='{packages}/postgres/share:$system' "
                   f"-c dynamic_library_path='{packages}/postgres/lib:$libdir' "
                   f"-c thinkthen.allowed_backends='{env['THINKTHEN_BACKEND']}:*'")
        run([*as_user, pg / 'pg_ctl', '-D', data, '-l', work / 'log', '-w', '-s',
             '-o', options, 'start'], stdout=subprocess.DEVNULL)
        try:
            env.update(PGHOST=str(work), PGUSER='postgres', PGDATABASE='postgres')
            # CSV's default blank NULL is indistinguishable from an answered empty string.
            null_token = uuid.uuid4().hex
            psql = [pg / 'psql', '-X', '-q', '--csv', '-P', f'null={null_token}',
                    '-v', 'ON_ERROR_STOP=1']
            if shown.parent.name == 'db-postgres-grant' and env['THINKTHEN_BACKEND'] == 'stub':
                # The authored role allows systemone only. Routine stub must be denied.
                result = run([*psql, '-v', 'ON_ERROR_STOP=0'], sql, capture_output=True)
                if (result.stderr.count('ERROR:') != 2
                        or 'usage.sql-selection' not in result.stderr
                        or 'permission denied for function filter' not in result.stderr):
                    raise RuntimeError(f'expected backend-policy and filter permission denials: {result.stderr}')
            elif shown.parent.name == 'db-postgres-grant':
                # The shown last query demonstrates an intentional permission denial.
                prefix, denied = sql.rsplit('SELECT id AS msg_id', 1)
                result = run(psql, prefix, capture_output=True)
                expected = subprocess.run([str(arg) for arg in psql],
                    input='\\connect - app\nSELECT id AS msg_id' + denied,
                    text=True, env=env, capture_output=True)
                if expected.returncode != 3 or 'permission denied for function filter' not in expected.stderr:
                    raise RuntimeError(f'expected filter permission denial: {expected.stderr}')
                sys.stderr.write(expected.stderr)
            else:
                result = run(psql, sql, capture_output=True)
        finally:
            primary_failed = sys.exc_info()[0] is not None
            try:
                run([*as_user, pg / 'pg_ctl', '-D', data, '-m', 'immediate', '-w', '-s',
                     'stop'], stdout=subprocess.DEVNULL)
            except Exception as cleanup_error:
                if not primary_failed:
                    raise
                print(f'PostgreSQL cleanup also failed: {cleanup_error}', file=sys.stderr)
    sys.stdout.write(result.stdout.replace(null_token, 'NULL') if dialect == 'postgres'
                     else result.stdout)
    sys.stderr.write(result.stderr)

if 'SITE_CASE_FILE' not in env:
    sys.exit(0)
fn = Path(env['SITE_CASE_FILE']).parent.name
rows = (list(csv.DictReader(io.StringIO(result.stdout))) if dialect == 'postgres'
        else json.loads(result.stdout) if result.stdout.strip() else [])
if dialect == 'postgres':
    rows = [{key: None if value == null_token else value for key, value in row.items()}
            for row in rows]
rows.sort(key=lambda row: int(row['msg_id']))


def decoded(value):
    if not isinstance(value, str):
        return value
    try:
        return json.loads(value)
    except json.JSONDecodeError:
        return value


def boolean(value):
    value = decoded(value)
    if value is None:
        return None
    if value in (True, 1, 't'):
        return True
    if value in (False, 0, 'f'):
        return False
    raise ValueError(f'not a boolean: {value!r}')


if fn == 'filter':
    answers = [row['body'] for row in rows]
elif fn == 'decide':
    answers = [boolean(row['is_refund']) for row in rows]
elif fn == 'choose':
    answers = [row['topic'] for row in rows]
elif fn == 'tag':
    answers = []
    for row in rows:
        value = decoded(row['topics'])
        if value is None:
            answers.append(None)
            continue
        if dialect == 'postgres':
            value = next(csv.reader([value[1:-1]])) if value != '{}' else []
        answers.append(sorted(value))
elif fn in ('score', 'rank'):
    key = 'urgency' if fn == 'score' else 'urgency_rank'
    answers = [decoded(row[key]) for row in rows]
elif fn == 'find':
    answers = [decoded(row['requested_action']) for row in rows]
elif fn == 'annotate':
    answers = [None if row['status'] != 'answered' else
               {'refund': boolean(row['is_refund']), 'topic': row['topic'],
                'urgency': decoded(row['urgency'])} for row in rows]
elif fn in ('recognize', 'relate'):
    if dialect == 'postgres':
        if fn == 'recognize':
            names = [{'text': row['name'], 'kind': row['kind']} for row in rows if row['name'] is not None]
        else:
            names = [{'source': row['person'], 'relation': row['relation'],
                      'target': row['organization']} for row in rows if row['person'] is not None]
        answers = [None if any(row['status'] != 'answered' for row in rows) else names]
    else:
        key = 'names' if fn == 'recognize' else 'works_for'
        answers = []
        for row in rows:
            values = decoded(row[key])
            if values is None:
                answers.append(None)
                continue
            if fn == 'recognize':
                answers.append([{'text': value['text'], 'kind': value['kind']} for value in values])
            else:
                answers.append([{'source': value['source']['text'], 'relation': value['relation'],
                                 'target': value['target']['text']} for value in values])
else:
    raise ValueError(fn)
Path(env['SITE_ANSWERS_FILE']).write_text(json.dumps(answers))
PY
