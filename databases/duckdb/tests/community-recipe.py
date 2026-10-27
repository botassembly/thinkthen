"""Exercise the official recipe without building DuckDB or the Rust engine."""
import json
import os
from pathlib import Path
import shlex
import subprocess
import tempfile
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from testing import child_environment
env = child_environment()

root = Path(__file__).resolve().parents[3]
source = Path(os.environ.get('THINKTHEN_DUCKDB_SOURCE', root / 'duckdb')).resolve()
tools = Path(os.environ.get('EXTENSION_CI_TOOLS', root / 'extension-ci-tools')).resolve()
versions = json.loads((root / 'databases/duckdb/community-versions.json').read_text())
version = os.environ.get('THINKTHEN_DUCKDB_VERSION', '1.4.3')
for project, pin in versions['1.4.3'].items():
    entry = subprocess.check_output(['git', 'ls-files', '--stage', '--', project],
                                    cwd=root, env=env, text=True).split()
    assert entry[:2] == ['160000', pin], (project, entry)
assert version in versions
revision = subprocess.check_output(['git', '-C', str(tools), 'rev-parse', 'HEAD'], text=True, env=env).strip()
assert revision == versions[version]['extension-ci-tools'], revision
# Feature: the public repo builds through DuckDB's own extension makefile.
commands = subprocess.check_output([
    'make', '-n', 'release', f'DUCKDB_SRCDIR={source}', f'EXTENSION_CI_TOOLS={tools}',
    f'OVERRIDE_GIT_DESCRIBE=v{version}',
    'EXT_FLAGS=-DCMAKE_EXPORT_COMPILE_COMMANDS=ON -DBUILD_UNITTESTS=OFF -DBUILD_SHELL=OFF'
], cwd=root, text=True, env=env)
configure = next(shlex.split(line) for line in commands.splitlines()
                 if line.startswith('cmake ') and '--build' not in line)
with tempfile.TemporaryDirectory(prefix='thinkthen-community-') as temporary:
    configure[configure.index('-B') + 1] = temporary
    subprocess.run(configure, cwd=root, check=True, env=env)
    build = Path(temporary)
    compilation = json.loads((build / 'compile_commands.json').read_text())
    adapter = next(item for item in compilation
                   if item['file'] == str(root / 'databases/duckdb/src/extension.cpp')
                   and 'loadable_extension.dir' in item['command'])
    command = shlex.split(adapter['command'])
    output = command.index('-o')
    del command[output:output + 2]
    command.remove('-c')
    # Compile the real adapter against this version's headers; link nothing.
    subprocess.run([*command, '-fsyntax-only'], cwd=adapter['directory'], check=True, env=env)
    assert (build / 'extension/thinkthen/contract_types.hpp').is_file()
    targets = subprocess.check_output(['cmake', '--build', temporary, '--target', 'help'], text=True, env=env)
    assert 'thinkthen_rust' in targets and 'thinkthen_loadable_extension' in targets
print(f'Community recipe configures and adapter compiles against DuckDB {version}; no engine build or load claimed')
