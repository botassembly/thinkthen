"""Load the delivered extension in the official command and Python wheel."""
import json
import os
import re
from pathlib import Path
import subprocess
import sys
import tempfile
import zipfile

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from testing import child_environment
from cases import quote

package, deps = map(Path, sys.argv[1:])
root = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(root / 'scripts'))
from package_inventory import INPUTS, DUCKDB_TERMS
wheel = deps / (deps / 'python-wheel').read_text().strip()
python_tag, abi_tag = wheel.name.split('-')[2:4]
if python_tag != f'cp{sys.version_info.major}{sys.version_info.minor}':
    # The prepared Linux image carries the interpreters for its pinned wheels.
    interpreter = Path('/opt/python') / f'{python_tag}-{abi_tag}' / 'bin/python3'
    os.execv(str(interpreter), [str(interpreter), *sys.argv])
extension = package / 'thinkthen.duckdb_extension'
env = child_environment()
assert {p.name for p in package.iterdir()} == {
    extension.name, *DUCKDB_TERMS, *(destination for _, destination in INPUTS['duckdb'])}
# Inspect the same installed file that each official host will load.
if sys.platform == 'darwin':
    dependencies = subprocess.check_output(['otool', '-L', str(extension)], env=env, text=True)
    sections = subprocess.check_output(['otool', '-l', str(extension)], env=env, text=True)
    symbols = subprocess.check_output(['nm', '-gU', str(extension)], env=env, text=True)
    assert '__debug_' not in sections
else:
    dependencies = subprocess.check_output(['readelf', '-d', str(extension)], env=env, text=True)
    sections = subprocess.check_output(['readelf', '-S', str(extension)], env=env, text=True)
    symbols = subprocess.check_output(['nm', '-D', '--defined-only', str(extension)], env=env, text=True)
    assert '.debug_' not in sections
    versions = subprocess.check_output(['readelf', '-V', str(extension)], env=env, text=True)
    libc = max(tuple(map(int, value.split('.'))) for value in re.findall(r'GLIBC_([0-9.]+)', versions))
    cxx = max(tuple(map(int, value.split('.'))) for value in re.findall(r'GLIBCXX_([0-9.]+)', versions))
    assert libc <= (2, 39), f'package exceeds the glibc 2.39 floor: {libc}'
    print('Linux symbol requirements: GLIBC_' + '.'.join(map(str, libc))
          + ', GLIBCXX_' + '.'.join(map(str, cxx)))
assert 'libduckdb' not in dependencies.lower(), dependencies
exported = [line.split()[-1].lstrip('_') for line in symbols.splitlines() if line.split()]
assert exported == ['thinkthen_duckdb_cpp_init'], exported
print(dependencies)
cli = deps / 'cli/duckdb'
assert subprocess.check_output([str(cli), '--version'], env=env, text=True).startswith('v1.4.3 ')

with tempfile.TemporaryDirectory() as temporary:
    helper = root / 'scripts/surface-example'
    subprocess.run([sys.executable, str(helper), 'prepare', 'duckdb', temporary, str(extension)], env=env, check=True)
    subprocess.run([sys.executable, str(helper), 'check', 'duckdb', str(cli), '-unsigned', '-json',
                    '-init', '/dev/null', ':memory:', '-f', str(Path(temporary) / 'example.sql')],
                   env=env, cwd=temporary, check=True)

prefix = f"LOAD {quote(str(extension))}; SET thinkthen_backend = 'stub';"
query = "SELECT position,status,error_code::VARCHAR AS error_code FROM thinkthen_decide((SELECT NULL::VARCHAR AS record), 'Ready?')"
output = subprocess.check_output([str(cli), '-unsigned', '-json', '-init', '/dev/null', ':memory:', prefix + query], env=env, text=True)
actual = json.loads(output)
assert len(actual) == 1 and actual[0]['status'] == 'failed' and actual[0]['error_code'] == 'record.missing', actual
statements = (root / 'databases/duckdb/examples/all-ten.sql').read_text().splitlines()
counts = (1, 1, 1, 1, 2, 2, 1, 1, 1, 1)
for statement, count in zip(statements, counts, strict=True):
    output = subprocess.check_output([str(cli), '-unsigned', '-json', '-init', '/dev/null', ':memory:', prefix + statement], env=env, text=True)
    result = json.loads(output)
    assert len(result) == count and all(row['status'] != 'failed' for row in result), result
with tempfile.TemporaryDirectory() as temporary:
    venv = Path(temporary) / 'venv'
    subprocess.run([sys.executable, '-m', 'venv', '--without-pip', str(venv)], env=env, check=True)
    python = venv / 'bin/python3'
    site = Path(subprocess.check_output([str(python), '-c', 'import sysconfig; print(sysconfig.get_path("purelib"))'], env=env, text=True).strip())
    with zipfile.ZipFile(wheel) as archive:
        archive.extractall(site)
    script = """import duckdb,json,sys
with duckdb.connect(config={'allow_unsigned_extensions': 'true'}) as connection:
    connection.execute(sys.argv[1])
    rows = connection.execute(sys.argv[2]).fetchall()
    assert len(rows) == 1 and rows[0][1] == 'failed' and rows[0][2] == 'record.missing', rows
    for statement,count in zip(json.loads(sys.argv[3]),(1,1,1,1,2,2,1,1,1,1),strict=True):
        cursor = connection.execute(statement)
        status = [column[0] for column in cursor.description].index('status')
        rows = cursor.fetchall()
        assert len(rows) == count and all(row[status] != 'failed' for row in rows), rows
"""
    subprocess.run([str(python), '-I', '-c', script, prefix, query, json.dumps(statements)], env=env, check=True)
print('Official DuckDB command and Python load the packaged canonical ABI')
