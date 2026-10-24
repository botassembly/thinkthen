"""Load the delivered extension in the official command and Python wheel."""
import json
import re
from pathlib import Path
import subprocess
import sys
import tempfile
import zipfile

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from testing import child_environment
from cases import invocation, quote, rows, expected

package, deps = map(Path, sys.argv[1:])
root = Path(__file__).resolve().parents[3]
extension = package / 'thinkthen.duckdb_extension'
env = child_environment()
assert {p.name for p in package.iterdir()} - {'README.md'} == {
    'thinkthen.duckdb_extension', 'THINKTHEN-LICENSE', 'DUCKDB-LICENSE', 'YYJSON-LICENSE'}
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
prefix = f"LOAD {quote(str(extension))}; SET thinkthen_settings = '{{\"adapter\":\"stub\"}}';"
cases = json.loads((root / 'construction/stub.json').read_text())
queries = ['SELECT status, answer::VARCHAR AS answer, p, error::VARCHAR AS error, detail::VARCHAR AS detail FROM '
           + invocation(case['request']) for case in cases]
for case, query in zip(cases, queries):
    output = subprocess.check_output([str(cli), '-unsigned', '-json', '-init', '/dev/null', ':memory:', prefix + query],
                                     env=env, text=True)
    actual = rows(json.loads(output))
    assert actual == expected(case), ('official CLI', case['request']['function'], actual, expected(case))
rejections = [("SET GLOBAL thinkthen_settings = '{}'", 'session-only'),
              ("SELECT * FROM thinkthen_decide((SELECT 'x' AS record), '{bad')", 'usage.question')]
for case in json.loads((root / 'conformance/public-contract/diagnostics.json').read_text()):
    request = {'function':'annotate','question':{'type':'inline','value':case['question']},'records':['alpha']}
    rejections.append(('SELECT * FROM ' + invocation(request), case['code'] + ': ' + case['message']))
for case in json.loads((root / 'conformance/public-contract/sql.json').read_text()):
    if 'message' in case:
        rejections.append(('SET thinkthen_settings = ' + quote(case['settings']), case['code'] + ': ' + case['message']))
for query, message in rejections:
    result = subprocess.run([str(cli), '-unsigned', '-json', '-init', '/dev/null', ':memory:', prefix + query],
                            env=env, text=True, capture_output=True)
    assert result.returncode != 0 and message in result.stderr, (query, result.stdout, result.stderr)
print('official DuckDB 1.4.3 command: all ten shared stub cases and native errors pass')

with tempfile.TemporaryDirectory() as temporary:
    venv = Path(temporary) / 'venv'
    subprocess.run([sys.executable, '-m', 'venv', '--without-pip', str(venv)], env=env, check=True)
    python = venv / 'bin/python3'
    site = Path(subprocess.check_output([str(python), '-c', 'import sysconfig; print(sysconfig.get_path("purelib"))'],
                                        env=env, text=True).strip())
    abi = f'cp{sys.version_info.major}{sys.version_info.minor}'
    wheel, = deps.glob(f'duckdb-1.4.3-{abi}-{abi}-*.whl')
    # Install the verified wheel into an empty environment without an index or network.
    with zipfile.ZipFile(wheel) as archive:
        archive.extractall(site)
    script = '''import duckdb, json, sys
assert duckdb.__version__ == '1.4.3'
connection = duckdb.connect(config={'allow_unsigned_extensions': 'true'})
connection.execute(sys.argv[1])
for query in json.loads(sys.argv[2]):
    result = connection.execute(query)
    names = [column[0] for column in result.description]
    print(json.dumps([dict(zip(names, row)) for row in result.fetchall()]))
for query, message in json.loads(sys.argv[3]):
    try:
        connection.execute(query)
    except Exception as error:
        assert message in str(error), str(error)
    else:
        raise AssertionError('invalid SQL unexpectedly succeeded: ' + query)
connection.close()
'''
    output = subprocess.check_output([str(python), '-I', '-c', script, prefix, json.dumps(queries), json.dumps(rejections)], env=env, text=True)
    results = output.splitlines()
    assert len(results) == len(cases)
    for case, result in zip(cases, results):
        actual = rows(json.loads(result))
        assert actual == expected(case), ('official Python wheel', case['request']['function'], actual, expected(case))
print('official DuckDB 1.4.3 Python wheel: all ten shared stub cases and native errors pass')
