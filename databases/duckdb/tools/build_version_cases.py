"""Exercise build routing with counted fake Cargo/CMake; compile and load nothing."""
import json
import subprocess
from pathlib import Path

from release_pack_cases import copy_packer


def routing(case, encode_footer, write_script):
    tree = case.root / 'build-tree'
    copy_packer(tree)
    source = case.root / 'build-source'
    commit = case.source_fixture(source)
    pins = tree / 'databases/duckdb/tools/version.env'
    text = pins.read_text().replace('d8cdaa33fda8df955cc76ef58a280f68f4cd43fa', commit)
    pins.write_text(text.replace('08e34c447bae34eaee3723cac61f2878b6bdf787', commit))
    static = case.root / 'build-static'
    static.mkdir()
    import hashlib
    data = b'fixture archive'
    (static / 'libduckdb_static.a').write_bytes(data)
    manifest = hashlib.sha256(data).hexdigest() + ' libduckdb_static.a\n'
    for name in ('archive-sha256.txt', 'archive-sha256-v1.5.4-linux-amd64.txt'):
        (tree / 'databases/duckdb/cpp' / name).write_text(manifest)
    shim = case.root / 'build-shim'
    write_script(shim / 'rustc', 'echo "host: x86_64-unknown-linux-gnu"')
    write_script(shim / 'readelf', 'echo "Machine: Advanced Micro Devices X86-64"')
    write_script(shim / 'cargo', f'printf "%s\\n" "$*" >> "{case.root}/cargo-calls"')
    shim.mkdir(exist_ok=True)
    cmake = shim / 'cmake'
    cmake.write_text('''#!/usr/bin/env python3
import json,os,sys
from pathlib import Path
assert [os.environ.get(k) for k in ('GIT_CONFIG_NOSYSTEM','GIT_CONFIG_GLOBAL','GIT_NO_REPLACE_OBJECTS')] == ['1','/dev/null','1']
assert all(os.environ.get(k) is None for k in ('GIT_DIR','GIT_WORK_TREE','GIT_COMMON_DIR','GIT_INDEX_FILE','GIT_OBJECT_DIRECTORY','GIT_ALTERNATE_OBJECT_DIRECTORIES','GIT_CONFIG_COUNT','GIT_CONFIG_KEY_0','GIT_CONFIG_VALUE_0','GIT_CONFIG_PARAMETERS','GIT_CONFIG','GIT_CONFIG_SYSTEM','GIT_REPLACE_REF_BASE','GIT_CEILING_DIRECTORIES'))
args=sys.argv[1:]
base=Path(args[1] if args[0]=='--build' else args[args.index('-B')+1])
with open(os.environ['FIXTURE_CMAKE_LOG'],'a') as log:log.write(json.dumps(args)+'\\n')
if args[0]=='--build':
    version=os.environ.get('FIXTURE_BAD_FOOTER',base.parent.name)
    body=Path(os.environ['FIXTURE_FOOTERS'])/(version+'.bin')
    out=base/'extension/thinkthen/thinkthen.duckdb_extension'
    out.parent.mkdir(parents=True,exist_ok=True)
    out.write_bytes(body.read_bytes())
    if os.environ.get('FIXTURE_MUTATE_SOURCE'):Path(os.environ['THINKTHEN_DUCKDB_CPP_SOURCE'],'source.cc').write_text('mutated during build')
else:base.mkdir(parents=True,exist_ok=True)
''')
    cmake.chmod(0o755)
    footers = case.root / 'footers'
    footers.mkdir()
    for version in ('v1.5.5', 'v1.5.4', 'v9.9.9'):
        (footers / (version + '.bin')).write_bytes(encode_footer(version))
    cmake_log = case.root / 'cmake-calls'
    env = {**case.env, 'PATH': f'{shim}:{case.env["PATH"]}', 'FIXTURE_FOOTERS': str(footers),
           'FIXTURE_CMAKE_LOG': str(cmake_log), 'THINKTHEN_DUCKDB_CPP_SOURCE': str(source),
           'THINKTHEN_DUCKDB_CPP_STATIC_DIR': str(static), 'THINKTHEN_DUCKDB_CPP_BUILD': str(case.root / 'cmake-base')}
    build = tree / 'databases/duckdb/cpp/build.sh'
    alias = tree / 'databases/duckdb/build/thinkthen.duckdb_extension'
    from source_pin_cases import replacement_controls
    replacement_controls(case, source, commit, build, env, write_script)
    for version in ('v1.5.5', 'v1.5.4'):
        result = subprocess.run(['/bin/sh', str(build)], env={**env, 'THINKTHEN_DUCKDB_VERSION': version},
                                capture_output=True, text=True, timeout=30, check=False)
        case.assertEqual(result.returncode, 0, result.stderr)
        canonical = tree / 'databases/duckdb/build/artifacts/cpp' / version / 'x86_64-unknown-linux-gnu/thinkthen.duckdb_extension'
        case.assertEqual(canonical.read_bytes(), encode_footer(version))
        case.assertEqual(alias.read_bytes(), encode_footer('v1.5.5'))
    calls = [json.loads(line) for line in cmake_log.read_text().splitlines()]
    for version, args in zip(('v1.5.5', 'v1.5.4'), calls[::2], strict=True):
        case.assertEqual(args[args.index('-B') + 1], str(case.root / 'cmake-base' / version / 'x86_64-unknown-linux-gnu'))
        case.assertIn('-DTHINKTHEN_DUCKDB_SOURCE_COMMIT=' + commit, args)
    case.assertFalse((tree / 'databases/duckdb/build/artifacts/cpp/x86_64-unknown-linux-gnu').exists())
    result = subprocess.run(['/bin/sh', str(build)], env={**env, 'FIXTURE_BAD_FOOTER': 'v9.9.9'},
                            capture_output=True, text=True, timeout=30, check=False)
    case.assertEqual(result.returncode, 1, result.stderr)
    case.assertIn('DuckDB footer DuckDB version is', result.stderr)
    case.assertEqual(alias.read_bytes(), encode_footer('v1.5.5'))
    result = subprocess.run(['/bin/sh', str(build)], env={**env, 'FIXTURE_MUTATE_SOURCE': '1'},
                            capture_output=True, text=True, timeout=30, check=False)
    case.assertEqual(result.returncode, 1, result.stderr)
    case.assertIn('raw Git source file differs: source.cc', result.stderr)
    case.assertEqual(alias.read_bytes(), encode_footer('v1.5.5'))
