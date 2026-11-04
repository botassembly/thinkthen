#!/usr/bin/env python3
"""Stage shipped Linux qualification packages for the existing site examples.

This consumes an already verified candidate. It never builds the native engine.
Run outside the offline site build; Python/.NET consumers may prepare dependencies.
"""
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import zipfile

candidate = Path(sys.argv[1]).resolve()
root = Path(__file__).resolve().parents[2]
out = root / 'target/site-packages'
tools = Path.home() / '.local/share/thinkthen-build-tools'


def one(folder, pattern):
    items = list((candidate / folder).glob(pattern))
    if len(items) != 1:
        raise ValueError(f'{folder}/{pattern}: expected one candidate file, found {len(items)}')
    return items[0]


def extract(archive, destination):
    destination.mkdir(parents=True, exist_ok=True)
    if zipfile.is_zipfile(archive):
        with zipfile.ZipFile(archive) as handle:
            handle.extractall(destination)
    else:
        with tarfile.open(archive) as handle:
            handle.extractall(destination, filter='data')


def run(*args, **kwargs):
    subprocess.run(list(map(str, args)), check=True, **kwargs)


def prepare_pandas_arrow():
    python = subprocess.check_output(['uv', 'python', 'find', '--offline', '3.10'], text=True).strip()
    run('uv', 'pip', 'install', '--python', python, '--target', out / 'python', 'pyarrow==19.0.1')


if '--pandas-only' in sys.argv:
    prepare_pandas_arrow()
    raise SystemExit(0)
if out.exists():
    shutil.rmtree(out)
out.mkdir(parents=True)

for name in ('c', 'cpp', 'sqlite', 'duckdb', 'postgres', 'swift'):
    destination = out / ('c' if name == 'cpp' else 'swift/thinkthen' if name == 'swift' else name)
    extract(one(name, '*.tar.gz'), destination)
extract(one('', '*.tar.gz'), root / 'target/release')
extract(one('typescript', '*.tgz'), out / 'node')
(out / 'node/package').rename(out / 'node/thinkthen')
extract(one('python', '*.whl'), out / 'python')
# Install the frame dependencies outside the build, alongside the delivered wheel.
python = subprocess.check_output(['uv', 'python', 'find', '--offline', '3.10'], text=True).strip()
run('uv', 'pip', 'install', '--python', python, '--target', out / 'python', '--no-index',
    '--find-links', candidate / 'prerequisites/python-frames-cp310/wheels', 'pandas', 'polars')
prepare_pandas_arrow()
gem_env = os.environ | {'GEM_HOME': str(out / 'ruby-gems'), 'GEM_PATH': str(out / 'ruby-gems')}
for gem in sorted((candidate / 'ruby/dependencies').glob('*.gem')):
    run(tools / 'ruby/runtime/bin/gem', 'install', gem, '--local', '--no-document',
        '--install-dir', out / 'ruby-gems', env=gem_env)
run(tools / 'ruby/runtime/bin/ruby', '--version')
run(tools / 'ruby/runtime/bin/gem', '--version')
run(tools / 'ruby/runtime/bin/ruby', '-rrubygems', '-e',
    'p Gem.path; p Gem::Platform.local.to_s; p Gem::Specification.find_all_by_name("ffi").map { |s| [s.full_name, s.missing_extensions?, Gem::Platform.match_spec?(s)] }', env=gem_env)
# The fixture loads delivered library files directly, like prepare-packages.sh.
run(tools / 'ruby/runtime/bin/gem', 'unpack', one('ruby', '*.gem'), '--target', out / 'ruby-source')
ruby = next((out / 'ruby-source').glob('thinkthen-*'))
shutil.copytree(ruby / 'lib', out / 'ruby/lib', dirs_exist_ok=True)
for language in ('java', 'kotlin'):
    for jar in (candidate / language).rglob('*.jar'):
        target = out / 'jvm/lib' / jar.name
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(jar, target)
compiler = out / 'jvm/kotlinc'
compiler.mkdir(parents=True, exist_ok=True)
for jar in (tools / 'jvm/m2').rglob('*.jar'):
    shutil.copy2(jar, compiler / jar.name)
(out / 'csharp/feed').mkdir(parents=True, exist_ok=True)
for package in (candidate / 'csharp').rglob('*.nupkg'):
    shutil.copy2(package, out / 'csharp/feed' / package.name)
for package in (candidate / 'prerequisites').rglob('*.nupkg'):
    shutil.copy2(package, out / 'csharp/feed' / package.name)
(out / 'bin').mkdir(exist_ok=True)
(out / 'bin/python3').symlink_to(python)
(out / 'bin/sqlite3').symlink_to(tools / 'floors/linux-glibc2.28/bin/sqlite3')
shutil.copy2(candidate / 'prerequisites/duckdb/duckdb', out / 'bin/duckdb')
(out / 'zig').mkdir(exist_ok=True)
shutil.copy2(one('zig', '*.tar.gz'), out / 'zig/thinkthen-zig-linux-x64.tar.gz')
# R and PostgreSQL use the same already prepared Linux runtimes as qualification.
labels = ['--label-file', os.environ['JOB_LABEL_FILE']] if os.environ.get('JOB_LABEL_FILE') else ['--label', 'pm.repo=thinkthen']
container = subprocess.check_output(['docker', 'create', *labels,
    'thinkthen-floor-build:glibc2.28'], text=True).strip()
try:
    run('docker', 'cp', container + ':/opt/go', out / 'go-runtime')
    (out / 'bin/go').symlink_to(out / 'go-runtime/bin/go')
    run('docker', 'cp', container + ':/opt/R', out / 'R-runtime')
    rlib = out / 'R-runtime/lib64/R/lib'
    for library in ('libreadline.so.7', 'libicuuc.so.60', 'libicui18n.so.60', 'libicudata.so.60'):
        run('docker', 'cp', '-L', container + ':/usr/lib64/' + library, rlib / library)
    run('docker', 'cp', container + ':/opt/thinkthen-pg18', out / 'postgres/runtime')
finally:
    primary_failed = sys.exc_info()[0] is not None
    try:
        run('docker', 'rm', container, stdout=subprocess.DEVNULL)
    except subprocess.CalledProcessError as cleanup_error:
        if not primary_failed:
            raise
        print(f'Container cleanup also failed: {cleanup_error}', file=sys.stderr)
extract(one('r', '*.tar.gz'), out / 'r')
for archive in (candidate / 'prerequisites/r-frames').glob('*.tar.gz'):
    extract(archive, out / 'r')
(out / 'postgres/bindir').write_text(str(out / 'postgres/runtime/bin') + '\n')
print('Prepared shipped docs packages at ' + str(out), flush=True)
print('R archive: ' + str(one('r', '*.tar.gz')), flush=True)
run('ldd', next((out / 'R-runtime').rglob('libR.so')))
run('ldd', out / 'postgres/runtime/bin/postgres')
r_home = out / 'R-runtime/lib64/R'
wrapper = out / 'bin/Rscript'
wrapper.write_text('#!/bin/sh\nexport R_HOME="' + str(r_home) + '"\nexport LD_LIBRARY_PATH="' + str(r_home / 'lib') + '${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"\nscript=$1\nshift\nexec "' + str(r_home / 'bin/exec/R') + '" --no-echo --no-restore --file="$script" --args "$@"\n')
wrapper.chmod(0o755)

for package in (tools / 'dotnet10/singlefile-packages').rglob('*.nupkg'):
    shutil.copy2(package, out / 'csharp/feed' / package.name)
swift_libs = out / 'swift-libs'
swift_libs.mkdir()
container = subprocess.check_output(['docker', 'create', *labels,
    'thinkthen-swift-build:6.3.2-ubuntu22.04'], text=True).strip()
try:
    for library in ('libxml2.so.2', 'libicuuc.so.70', 'libicui18n.so.70', 'libicudata.so.70'):
        run('docker', 'cp', '-L', container + ':/lib/x86_64-linux-gnu/' + library, swift_libs / library)
finally:
    primary_failed = sys.exc_info()[0] is not None
    try:
        run('docker', 'rm', container, stdout=subprocess.DEVNULL)
    except subprocess.CalledProcessError as cleanup_error:
        if not primary_failed:
            raise
        print(f'Container cleanup also failed: {cleanup_error}', file=sys.stderr)

# Prepare the delivered Zig consumer outside the unchanged timed example run.
print('Prepare Zig example with two compiler jobs', flush=True)
run('bash', 'run.sh', '--prepare', cwd=root / 'site/examples/lang-zig',
    env=os.environ | {'THINKTHEN_BUILD_TOOLS': str(tools)})
