"""Run small installed-host lifetime comparisons inside one owning job."""
import hashlib
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile

from build_targets import container_labels

BEFORE = 'c0749a3929a75b49ec701df2e80ecb8158345e26'
FLOOR = 'sha256:25ddf271b9f898c913ed8069733d64e16ac40b0186c3ccd6372841d78921a88f'
POSTGRES = 'sha256:b47d1f7d2097b61520993a1dfcac8cbc4fac54dba8ae176b5b85660b58ebced0'


def expect_hash(path, expected):
    actual = hashlib.sha256(path.read_bytes()).hexdigest()
    if actual != expected:
        raise RuntimeError('prepared dependency changed: ' + str(path))
    return actual


def lifetime_proof(root, tools, output, loopback, receipt):
    before = output / 'before'
    before.mkdir()
    archive = subprocess.check_output(['git', 'archive', BEFORE, 'databases', 'extensions'])
    with tarfile.open(fileobj=io.BytesIO(archive)) as source:
        source.extractall(before, filter='data')
    stage = tools / 'stress-campaign/0182-7890a5eba312-native'
    receipt['lifetime_host_inputs'] = dict(postgres_image=POSTGRES, floor_image=FLOOR,
        postgres_extension_sha256=hashlib.sha256((stage / 'postgres/package/lib/thinkthen_postgres.so').read_bytes()).hexdigest(),
        duckdb_original_sql_sha256=hashlib.sha256((stage / 'duckdb/fixture/sql').read_bytes()).hexdigest())
    floor = tools / 'floors/linux-glibc2.28'
    sqlite, jansson = floor / 'sqlite-3.41.0', floor / 'jansson-2.14.1'
    receipt['sqlite_floor_inputs'] = {str(path): expect_hash(path, expected) for path, expected in (
        (sqlite / 'libsqlite3.a', '089324869415725bffd368c0b7d5c83b022785a0b9f0bf13dac95643de1c9526'),
        (jansson / 'lib/libjansson.a', '8a9db389f5035b5a6527d8c44773e4aa940a34c9b709caac74f96943f6d9b7a0'))}
    extension = stage / 'sqlite/installed/thinkthen_sqlite.so'
    receipt['sqlite_extension'] = expect_hash(extension, 'aab6a3264ebd8dadc54975a8dd2244a29bbb8c6c2721cff47135a47921f2570d')
    cargo_home = output / 'cargo-floor'
    cargo_home.mkdir()
    configuration = stage / 'cargo-floor/config.toml'
    expect_hash(configuration, 'a8520207aa913d93757f0114d6ceccec18d8e8b71a29a76935ce4b3fbebf6b15')
    shutil.copy2(configuration, cargo_home / 'config.toml')
    for name in ('registry', 'git', 'bin'):
        (cargo_home / name).symlink_to(Path.home() / '.cargo' / name)
    consumer = output / 'sqlite-consumer'
    compile_command = [sys.executable, str(root / 'scripts/runtime-floor-build'), 'cc', '-std=c11',
        '-Wall', '-Wextra', '-Werror', '-pthread', '-I', str(sqlite), '-I', str(jansson / 'include'),
        str(root / 'extensions/sqlite/tests/consumer.c'), str(sqlite / 'libsqlite3.a'),
        str(jansson / 'lib/libjansson.a'), '-ldl', '-lm', '-o', str(consumer)]
    env = dict(os.environ, CARGO_HOME=str(cargo_home), TMPDIR=str(output), THINKTHEN_FLOOR_IMAGE=FLOOR)
    try:
        with (output / 'sqlite-compile.log').open('w') as log:
            subprocess.run(compile_command, env=env, stdout=log, stderr=subprocess.STDOUT, check=True)
    finally:
        for name in ('registry', 'git', 'bin'):
            (cargo_home / name).unlink()
    receipt['sqlite_compile_command'] = compile_command
    receipt['sqlite_consumer_sha256'] = hashlib.sha256(consumer.read_bytes()).hexdigest()
    receipt['lifetime_checks'] = []
    for surface in ('duckdb', 'sqlite', 'postgres'):
        for version, concurrency in (('before', 1), ('after', 1), ('after', 2)):
            selected = before if version == 'before' else root
            test = [sys.executable, str(root / 'scripts/sql-stress-lifetime-test'), str(selected), surface, str(loopback)]
            if surface == 'duckdb':
                test.append(str(stage / 'duckdb/fixture'))
            elif surface == 'sqlite':
                test.extend([str(consumer), str(extension)])
            else:
                test = postgres_command(root, output, stage, loopback, version, concurrency)
            label = f'{surface}-{version}-{concurrency}'
            result = subprocess.run(test, env=dict(os.environ, THINKTHEN_LIFETIME_CONCURRENCY=str(concurrency)),
                capture_output=True, text=True)
            (output / (label + '.stdout')).write_text(result.stdout)
            (output / (label + '.stderr')).write_text(result.stderr)
            receipt['lifetime_checks'].append(dict(surface=surface, source=version,
                concurrency=concurrency, exit=result.returncode))
            if version == 'before':
                assert result.returncode != 0 and 'completed payload outlived checkpoint' in result.stderr, result.stderr
            else:
                assert result.returncode == 0, result.stderr
                assert json.loads(result.stdout)['ownership'] == 'released', result.stdout


def postgres_command(root, output, stage, loopback, version, concurrency):
    # The image and extension are reused read-only. Installation and data are private.
    script = '''
mkdir -p /work/pg-home
cp /package/lib/* "$(pg_config --pkglibdir)/"
cp /package/share/extension/* "$(pg_config --sharedir)/extension/"
if ! getent passwd "$1" >/dev/null; then
  printf 'lifetime:x:%s:%s::/work/pg-home:/bin/sh\\n' "$1" "$2" >> /etc/passwd
fi
chown "$1:$2" /work/pg-home
exec setpriv --reuid "$1" --regid "$2" --clear-groups env -i HOME=/work/pg-home \
  PATH=/opt/thinkthen-pg18/bin:/usr/bin:/bin LANG=C.UTF-8 THINKTHEN_LIFETIME_CONCURRENCY="$4" \
  python3 /src/scripts/sql-stress-lifetime-test "$3" postgres /loopback
'''
    return ['docker', 'run', '--rm', '--network', 'none', *container_labels(root),
        '-v', str(root) + ':/src:ro', '-v', str(output) + ':/work',
        '-v', str(loopback) + ':/loopback:ro', '-v', str(stage / 'postgres/package') + ':/package:ro',
        POSTGRES, 'sh', '-eu', '-c', script, 'lifetime', str(os.getuid()), str(os.getgid()),
        '/work/before' if version == 'before' else '/src', str(concurrency)]
