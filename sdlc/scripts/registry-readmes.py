#!/usr/bin/env python3
"""Check packed public text against its authoritative source without contacting registries."""
import argparse
import email
import gzip
import io
import json
import os
from pathlib import Path
import re
import shutil
import signal
import subprocess
import sys
import tempfile
import tarfile
import time
import tomllib
import xml.etree.ElementTree as ET
import zipfile

REPO = Path(__file__).resolve().parents[2]


class Refusal(ValueError):
    pass


def version(root=REPO):
    return tomllib.loads((root / 'crates/thinkthen/Cargo.toml').read_text())['package']['version']


def source(family, root=REPO):
    if family in ('command', 'packagist'):
        return root / 'README.md'
    if family == 'crate':
        return root / 'crates/thinkthen/README.md'
    if family in ('typescript', 'go', 'ruby'):
        return root / 'libraries' / family / 'README.md'
    if family in ('sqlite', 'postgresql', 'duckdb'):
        return root / 'databases' / family / 'README-PUBLIC.md'
    if family == 'flutter':
        return root / 'libraries/dart/flutter/README-PUBLIC.md'
    return root / 'libraries' / family / 'README-PUBLIC.md'


def check_text(data, release):
    text = data.decode('utf-8')
    rules = {
        'adr': r'\bADR[\s#:-]*\d+\b',
        'ticket': r'\bticket[\s#:-]*\d+\b',
        'internal-path': r'\bsdlc[/\\]',
        'internal-project': r'\b(?:thinkthen[-_]core|thinkthen[-_]native|thinkthen[-_]conformance)\b',
        'internal-crate': r'\bthinkthen-(?:python|node|ruby|c|postgresql|sqlite|duckdb)\b(?!-\d)',
    }
    for reason, pattern in rules.items():
        if re.search(pattern, text, re.I):
            raise Refusal(reason)
    names = Path.home() / '.config/thinkthen/private-names.txt'
    if names.is_file():
        for name in names.read_text().splitlines():
            if name.strip() and re.search(r'(?<!\w)' + re.escape(name.strip()) + r'(?!\w)', text, re.I):
                raise Refusal('private-name')
    # Versioned ThinkThen artifacts, release URLs, install pins and explicit product claims.
    # Other packages' compiler, Rust, DuckDB and language-runtime versions are unrestricted.
    check_versions(text, release)


def check_versions(text, release):
    patterns = [
        r'\bThinkThen(?:\s+(?:version|release))?\s+v?(\d+\.\d+(?:\.\d+)?)',
        r'thinkthen(?:[-_][\w.]+)?(?:[-_@= ]+|\s+-v\s+)v?(\d+\.\d+\.\d+)',
        r'releases/download/v(\d+\.\d+\.\d+)',
        r'botassembly:thinkthen-jvm:(\d+\.\d+\.\d+)',
    ]
    for pattern in patterns:
        for match in re.finditer(pattern, text, re.I):
            found = match.group(1)
            if found != release and found != '.'.join(release.split('.')[:2]):
                raise Refusal('stale-version')


def check_readme(family, data, root=REPO):
    check_text(data, version(root))
    if len(data.decode().splitlines()) >= 40:
        raise Refusal('readme-needs-under-40-lines: ' + family)
    expected = source(family, root).read_bytes()
    if data != expected:
        raise Refusal('readme-source-mismatch: ' + family)
    if family in ('command', 'packagist') and opening(data.decode()) != OPENING:
        raise Refusal('shared-opening: ' + family)
    if opening(data.decode()) != opening((root / 'README.md').read_text()):
        raise Refusal('shared-opening: ' + family)


def opening(text):
    for paragraph in text.split('\n\n'):
        if paragraph.strip() and not paragraph.startswith(('#', '<', '```')):
            return paragraph
    raise Refusal('missing-opening')


OPENING = opening((REPO / 'README.md').read_text())


def members(data, zipped=False):
    if zipped:
        with zipfile.ZipFile(io.BytesIO(data)) as archive:
            return {name: archive.read(name) for name in archive.namelist() if not name.endswith('/')}
    with tarfile.open(fileobj=io.BytesIO(data), mode='r:*') as archive:
        return {entry.name.removeprefix('./'): archive.extractfile(entry).read()
                for entry in archive if entry.isfile()}


def unique_suffix(files, suffix):
    matches = [value for name, value in files.items() if name == suffix or name.endswith('/' + suffix)]
    if len(matches) != 1:
        raise Refusal('missing-or-repeated-member: ' + suffix)
    return matches[0]


def description(family, root=REPO):
    if family in ('python', 'crate'):
        path = 'libraries/python/pyproject.toml' if family == 'python' else 'crates/thinkthen/Cargo.toml'
        key = 'project' if family == 'python' else 'package'
        return tomllib.loads((root / path).read_text())[key]['description']
    if family == 'packagist':
        return json.loads((root / 'composer.json').read_text())['description']
    if family in ('typescript', 'php'):
        path = 'libraries/typescript/package.json' if family == 'typescript' else 'libraries/php/composer.json'
        return json.loads((root / path).read_text())['description']
    if family in ('jvm', 'csharp'):
        path = 'libraries/jvm/pom.xml' if family == 'jvm' else 'libraries/csharp/Botassembly.ThinkThen.nuspec'
        document = ET.parse(root / path)
        return next(node.text.strip() for node in document.iter() if node.tag.rsplit('}', 1)[-1] == 'description')
    if family == 'dart':
        return re.search(r'(?m)^description: (.+)$', (root / 'libraries/dart/pubspec.yaml').read_text()).group(1)
    if family == 'ruby':
        return re.search(r'spec.description = "([^"]+)"', (root / 'libraries/ruby/thinkthen.gemspec').read_text()).group(1)
    if family == 'r':
        text = (root / 'libraries/r/thinkthen/DESCRIPTION').read_text()
        return re.search(r'(?m)^Description: (.*(?:\n[ \t]+[^\n]*)*)', text).group(1)
    return None


def check_description(family, text, root=REPO):
    check_text(text.encode(), version(root))
    if text != description(family, root):
        raise Refusal('description-source-mismatch: ' + family)


def check_package(family, path, root=REPO):
    files = members(path.read_bytes(), path.suffix in ('.whl', '.nupkg', '.zip'))
    if family == 'python':
        metadata = email.message_from_bytes(unique_suffix(files, 'METADATA') if path.suffix == '.whl' else unique_suffix(files, 'PKG-INFO'))
        check_readme(family, metadata.get_payload(decode=True), root)
        check_description(family, metadata['Summary'], root)
    elif family == 'ruby':
        package = members(files['data.tar.gz'])
        check_readme(family, unique_suffix(package, 'README.md'), root)
        metadata = gzip.decompress(files['metadata.gz']).decode()
        # Gem emits description as a YAML scalar, potentially folded by its serializer.
        text = re.search(r'(?m)^description: (.*(?:\n  [^\n]*)*)', metadata).group(1)
        text = ' '.join(text.split())
        if text.startswith("'") and text.endswith("'"):
            text = text[1:-1].replace("''", "'")
        elif text.startswith('"'):
            text = json.loads(text)
        check_description(family, text, root)
    elif family == 'csharp' and path.suffix != '.nupkg':
        check_readme(family, unique_suffix(files, 'README.md'), root)
        nested = [name for name in files if name.endswith('.nupkg')]
        if len(nested) != 1:
            raise Refusal('missing-or-repeated-nupkg')
        check_nupkg(files[nested[0]], root)
    else:
        prefix = {'crate': 'thinkthen-' + version(root) + '/', 'r': 'thinkthen/', 'typescript': 'package/', 'flutter': 'flutter/'}.get(family, '')
        name = prefix + 'README.md'
        if name not in files:
            raise Refusal('missing-or-repeated-member: ' + name)
        check_readme(family, files[name], root)
        if family == 'typescript':
            check_description(family, json.loads(unique_suffix(files, 'package.json'))['description'], root)
        elif family == 'crate':
            check_description(family, tomllib.loads(files[prefix + 'Cargo.toml'].decode())['package']['description'], root)
        elif family == 'php':
            check_description(family, json.loads(unique_suffix(files, 'composer.json'))['description'], root)
        elif family == 'jvm':
            document = ET.fromstring(unique_suffix(files, 'pom.xml'))
            check_description(family, next(node.text.strip() for node in document.iter() if node.tag.rsplit('}', 1)[-1] == 'description'), root)
        elif family == 'dart':
            text = unique_suffix(files, 'pubspec.yaml').decode()
            check_description(family, re.search(r'(?m)^description: (.+)$', text).group(1), root)
        elif family == 'r':
            text = files[prefix + 'DESCRIPTION'].decode()
            check_description(family, re.search(r'(?m)^Description: (.*(?:\n[ \t]+[^\n]*)*)', text).group(1), root)
        elif family == 'csharp':
            check_nupkg(path.read_bytes(), root)


def check_nupkg(data, root):
    files = members(data, True)
    check_readme('csharp', unique_suffix(files, 'README.md'), root)
    document = ET.fromstring(unique_suffix(files, 'Botassembly.ThinkThen.nuspec'))
    text = next(node.text.strip() for node in document.iter() if node.tag.rsplit('}', 1)[-1] == 'description')
    check_description('csharp', text, root)


PUBLIC_FAMILIES = ('command', 'crate', 'python', 'typescript', 'ruby', 'go', 'cpp',
                   'c', 'csharp', 'jvm', 'swift', 'zig', 'php', 'dart', 'flutter',
                   'ada', 'objective-c', 'cobol', 'r', 'sqlite', 'postgresql', 'duckdb', 'packagist')


def check_sources(root=REPO):
    for family in PUBLIC_FAMILIES:
        check_readme(family, source(family, root).read_bytes(), root)
        text = description(family, root)
        if text is not None:
            check_description(family, text, root)
    descriptor = (root / 'sdlc/records/0577-description-draft.yml').read_text()
    descriptor_opening = re.search(r'  extended_description: \|\n    (.*)', descriptor)
    pr_opening = (root / 'sdlc/records/0577-pr-description-draft.md').read_text().split('\n\n')[1:2]
    expected = opening((root / 'README.md').read_text())
    if descriptor_opening is None or descriptor_opening.group(1) != expected or pr_opening != [expected]:
        raise Refusal('shared-opening: duckdb-submission')


SAMPLES = {
    'command': ('sh', 'sample.sh'), 'packagist': ('php', 'sample.php'),
    'crate': ('rust', 'sample.rs'), 'python': ('python', 'sample.py'),
    'typescript': ('typescript', 'sample.mjs'), 'ruby': ('ruby', 'sample.rb'),
    'go': ('go', 'main.go'), 'cpp': ('cpp', 'sample.cpp'), 'c': ('c', 'sample.c'),
    'csharp': ('csharp', 'Program.cs'), 'jvm': ('java', 'RegistryExample.java'),
    'swift': ('swift', 'sample.swift'), 'zig': ('zig', 'sample.zig'),
    'php': ('php', 'sample.php'), 'dart': ('dart', 'sample.dart'),
    'flutter': ('dart', 'sample.dart'), 'ada': ('ada', 'registry_example.adb'),
    'objective-c': ('objc', 'sample.m'), 'cobol': ('cobol', 'sample.cob'),
    'r': ('r', 'sample.R'), 'sqlite': ('sql', 'sample.sql'),
    'postgresql': ('sql', 'sample.sql'), 'duckdb': ('sql', 'sample.sql'),
}


FUNCTION = r'(?<![\w])(?:thinkthen_|tt_)?(decide|choose|tag|score|filter|rank|find|annotate|recognize|relate)(?:Answer(?:Async)?|ValueAsync|_Answer)?(?![\w])'


def examples(family, root=REPO):
    language, filename = SAMPLES[family]
    text = source(family, root).read_text()
    blocks = list(re.finditer(r'```' + language + r'\n(.*?)\n```', text, re.S))
    samples = []
    functions = set()
    for block in blocks:
        code = block.group(1) + '\n'
        # Installation belongs to package assembly, not offline sample replay.
        if not re.search(FUNCTION, code, re.I) and re.fullmatch(
                r'(?:(?:install\.packages\([^\n]*\)|CREATE EXTENSION thinkthen;|'
                r'INSTALL thinkthen FROM community;|LOAD (?:thinkthen|\x27[^\n]+\x27);|\.load [^\n]*|'
                r'curl [^\n]*install\.sh[^\n]*|brew install [^\n]*)\n)+', code):
            continue
        functions.update(name.lower() for name in re.findall(FUNCTION, code, re.I))
        output = re.match(r'\s*```text\n(.*?)\n```', text[block.end():], re.S)
        samples.append((filename, code, output.group(1) + '\n' if output else None))
    if not samples:
        raise Refusal('missing-example: ' + family)
    if len(functions) < 2:
        raise Refusal('example-needs-two-functions: ' + family)
    return samples


def example(family, root=REPO):
    return examples(family, root)[0]


def unpack_example_package(family, packages, destination, selected=None):
    family = 'php' if family == 'packagist' else family
    candidates = []
    for path in packages.iterdir():
        if not path.is_file() or path.name.endswith('.sha256') or path.name.startswith('thinkthen-first-run'):
            continue
        try:
            if archive_family(path) == family:
                candidates.append(path)
        except Refusal:
            continue
    machine = {'arm64': 'aarch64'}.get(os.uname().machine, os.uname().machine)
    native = 'apple-darwin' if os.uname().sysname == 'Darwin' else 'linux'
    host = [path for path in candidates if machine in path.name and native in path.name]
    if host:
        candidates = host
    if selected is not None:
        candidates = [path for path in candidates if path == selected]
    if len(candidates) != 1:
        raise Refusal('missing-or-ambiguous-example-package: ' + family)
    package = candidates[0]
    check_package(family, package)
    files = members(package.read_bytes(), package.suffix in ('.whl', '.nupkg', '.zip'))
    executable = set()
    if package.suffix not in ('.whl', '.nupkg', '.zip') and family != 'ruby':
        with tarfile.open(package) as archive:
            executable = {item.name for item in archive if item.isfile() and item.mode & 0o111}
    if family == 'ruby':
        files = members(files['data.tar.gz'])
    for name, data in files.items():
        path = Path(name)
        if path.is_absolute() or '..' in path.parts:
            raise Refusal('unsafe-example-package-member')
        out = destination / path
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_bytes(data)
        if path.name == 'thinkthen' or name in executable:
            out.chmod(0o755)
    if family == 'c':
        for filename, alias in (('libthinkthen.so', 'libthinkthen.so.0'), ('libthinkthen.dylib', 'libthinkthen.0.dylib')):
            library = destination / 'lib' / filename
            if library.is_file() and not (library.parent / alias).exists():
                (library.parent / alias).symlink_to(library.name)
    if family == 'r':
        return destination / 'thinkthen'
    pages = list(destination.rglob('README.md'))
    return pages[0].parent if len(pages) == 1 else destination


def run_process(command, work, env, input, timeout):
    with subprocess.Popen(command, cwd=work, env=env, stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                          text=True, start_new_session=True) as process:
        try:
            stdout, stderr = process.communicate(input, timeout=timeout)
        except BaseException:
            # Reap the compiler before its owner releases the lock or removes scratch.
            # Let compilers reap their workers, then kill any unresponsive group.
            try:
                os.killpg(process.pid, signal.SIGTERM)
            except ProcessLookupError:
                pass
            try:
                process.communicate(timeout=1)
            except subprocess.TimeoutExpired:
                pass
            finally:
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                process.communicate()
            raise
        return subprocess.CompletedProcess(command, process.returncode, stdout, stderr)


def run_build(command, work, env, input):
    lock_path = os.environ.get('THINKTHEN_HEAVY_LOCK', '/tmp/thinkthen-heavy.lock')
    if sys.platform == 'linux':
        supervisor = REPO / 'sdlc/scripts/build-admission.py'
        owned = [sys.executable, str(supervisor), '--lock', lock_path,
                 '--priority', 'normal', '--timeout', '300', '--', *command]
        # The supervisor bounds admission and compilation, then reaps every
        # descendant before releasing its permit. Cleanup has no second deadline.
        with subprocess.Popen(owned, cwd=work, env=env, stdin=subprocess.PIPE,
                              stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                              text=True, start_new_session=True) as process:
            try:
                stdout, stderr = process.communicate(input)
            except BaseException:
                try:
                    process.terminate()
                except ProcessLookupError:
                    pass
                process.communicate()
                raise
            return subprocess.CompletedProcess(command, process.returncode, stdout, stderr)
    import fcntl
    # macOS has no Linux subreaper; keep its bounded process-group fallback.
    # Children never inherit the exclusive lock.
    with open(lock_path, 'a') as lock:
        deadline = time.monotonic() + 180
        while True:
            try:
                fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
                break
            except BlockingIOError:
                if time.monotonic() >= deadline:
                    raise subprocess.TimeoutExpired(command, 180)
                time.sleep(.05)
        return run_process(command, work, env, input, 300)


def run_example(family, packages, work, root=REPO, selected=None, sample_index=0):
    filename, code, expected = examples(family, root)[sample_index]
    package = unpack_example_package(family, packages, work / 'package', selected)
    home = work / 'home'; home.mkdir()
    cache = work / 'cache'; cache.mkdir(mode=0o700)
    # The site uses this same recording as a fresh cache, without shell settings or a key.
    shutil.copyfile(root / 'site/recordings/thinkthen.jsonl', cache / 'thinkthen.jsonl')
    sample = work / filename; sample.write_text(code)
    env = {'PATH': os.environ.get('PATH', '/usr/bin:/bin'), 'HOME': str(home),
           'LANG': 'C.UTF-8', 'LC_ALL': 'C.UTF-8', 'XDG_CACHE_HOME': str(home / '.cache'),
           'XDG_CONFIG_HOME': str(home / '.config'), 'XDG_STATE_HOME': str(home / '.local/state'),
           'THINKTHEN_CACHE': str(cache), 'RUSTC_WRAPPER': '', 'CARGO_BUILD_RUSTC_WRAPPER': '',
           'CARGO_HOME': os.environ.get('CARGO_HOME', str(Path.home() / '.cargo')),
           'RUSTUP_HOME': os.environ.get('RUSTUP_HOME', str(Path.home() / '.rustup')),
           'CARGO_NET_OFFLINE': 'true', 'CARGO_BUILD_JOBS': '2',
           'CARGO_TARGET_DIR': str(root / 'target')}
    env['RUSTUP_TOOLCHAIN'] = tomllib.loads((root / 'rust-toolchain.toml').read_text())['toolchain']['channel']
    for name in ('HTTPS_PROXY', 'ALL_PROXY'):
        if os.environ.get(name, '').startswith('http://127.0.0.1:'):
            env[name] = os.environ[name]

    def run(command, build=False, input=None):
        done = run_build(command, work, env, input) if build else run_process(command, work, env, input, 30)
        if done.returncode:
            raise Refusal('example-did-not-run: ' + family + '\n' + (done.stdout + done.stderr)[-4000:])
        return done.stdout

    if family == 'command':
        sample_archive = packages / 'thinkthen-first-run.tar.gz'
        fixture = members(sample_archive.read_bytes()) if 'thinkthen-first-run' in code else {}
        for name, data in fixture.items():
            path = work / name
            if Path(name).is_absolute() or '..' in Path(name).parts:
                raise Refusal('unsafe-example-package-member')
            path.parent.mkdir(parents=True, exist_ok=True); path.write_bytes(data)
        sample.write_text(re.sub(r'^curl .*$', 'cd thinkthen-first-run', code, flags=re.M))
        env['PATH'] = str(package) + os.pathsep + env['PATH']
        output = run(['bash', '-euo', 'pipefail', str(sample)])
    elif family == 'python':
        run(['uv', 'venv', '--offline', '--python', '3.12', str(work / 'venv')])
        wheel = next(path for path in packages.iterdir() if path.suffix == '.whl')
        run(['uv', 'pip', 'install', '--offline', '--no-deps', '--python', str(work / 'venv/bin/python'), str(wheel)])
        output = run([str(work / 'venv/bin/python'), str(sample)])
    elif family == 'ruby':
        env['RUBYLIB'] = str(package / 'lib')
        output = run(['ruby', str(sample)])
    elif family in ('php', 'packagist'):
        (work / 'vendor').mkdir()
        (work / 'vendor/autoload.php').symlink_to(package / 'autoload.php')
        output = run([os.environ.get('THINKTHEN_PHP_BIN', 'php'), '-n', '-d', 'extension=ffi', '-d', 'ffi.enable=1', str(sample)])
    elif family == 'typescript':
        (work / 'node_modules').mkdir()
        (work / 'node_modules/thinkthen').symlink_to(package)
        output = run(['node', str(sample)])
    elif family == 'go':
        (work / 'go.mod').write_text('module example.org/registry\n\ngo 1.22\nrequire github.com/botassembly/thinkthen/libraries/go v0.0.0\nreplace github.com/botassembly/thinkthen/libraries/go => ' + str(package) + '\n')
        env.update(GOPROXY='off', GOSUMDB='off', GOTOOLCHAIN='local', CGO_ENABLED='1',
                   GOCACHE=str(root / 'target/readme-go-cache'), GOMODCACHE=str(work / 'modcache'))
        run(['go', 'build', '-p', '1', '-buildvcs=false', '-o', str(work / 'sample'), '.'], True)
        output = run([str(work / 'sample')])
    elif family == 'crate':
        (work / 'src').mkdir(); sample.rename(work / 'src/main.rs')
        (work / 'Cargo.toml').write_text('[package]\nname="registry-example"\nversion="0.0.0"\nedition="2024"\n[dependencies]\nthinkthen={path=' + json.dumps(str(package)) + '}\n')
        shutil.copyfile(root / 'Cargo.lock', work / 'Cargo.lock')
        run(['cargo', 'build', '--offline', '-j2'], True)
        output = run([str(root / 'target/debug/registry-example')])
    elif family == 'jvm':
        classpath = os.pathsep.join(str(path) for path in package.rglob('*.jar'))
        run(['javac', '--release', '22', '-cp', classpath, str(sample)], True)
        output = run(['java', '--enable-native-access=ALL-UNNAMED', '-cp', str(work) + os.pathsep + classpath, 'RegistryExample'])
    elif family in ('c', 'cpp', 'ada', 'cobol', 'zig'):
        native_package = unpack_example_package('c', packages, work / 'native')
        library = native_package / 'lib'
        if family == 'cpp':
            (work / 'include/thinkthen').mkdir(parents=True)
            shutil.copyfile(native_package / 'include/thinkthen.h', work / 'include/thinkthen/thinkthen.h')
            command = ['c++', '-std=c++17', '-Wall', '-Wextra', '-Werror', '-I' + str(package / 'include'), '-I' + str(work / 'include'), str(sample), '-L' + str(library), '-lthinkthen', '-Wl,-rpath,' + str(library), '-o', str(work / 'sample')]
        elif family == 'ada':
            command = ['gnatmake', '-q', '-gnat2022', '-I' + str(package / 'src'), str(sample), '-o', str(work / 'sample'), '-largs', '-L' + str(library), '-lthinkthen', '-Wl,-rpath,' + str(library)]
        elif family == 'c':
            command = ['cc', '-std=c11', '-Wall', '-Wextra', '-Werror', '-I' + str(package / 'include'), str(sample), '-L' + str(library), '-lthinkthen', '-pthread', '-Wl,-rpath,' + str(library), '-o', str(work / 'sample')]
        elif family == 'cobol':
            shutil.copyfile(native_package / 'include/thinkthen.h', package / 'src/thinkthen.h')
            command = ['cobc', '-x', '-free', '-fstatic-call', '-fno-gen-c-decl-static-call', '-I', str(package / 'copybooks'), '-A', '-include ' + str(package / 'src/tt_session.h') + ' -I' + str(package / 'src') + ' -Wno-incompatible-pointer-types', '-o', str(work / 'sample'), str(sample), str(package / 'src/tt_session.c'), str(package / 'src/tt_requests_generated.c'), '-Q', str(library / 'libthinkthen.so.0') + ' -Wl,-rpath,' + str(library)]
        else:
            env['ZIG_GLOBAL_CACHE_DIR'] = str(root / 'target/readme-zig-cache')
            command = ['zig', 'build-exe', '-j2', '-fllvm', '-flld', str(library / 'libthinkthen.a'), '--dep', 'thinkthen', '-Mroot=' + str(sample), '-I', str(native_package / 'include'), '-Mthinkthen=' + str(package / 'src/thinkthen.zig'), '-lc', '-lgcc_s', '-lutil', '-lrt', '-lpthread', '-lm', '-ldl', '-femit-bin=' + str(work / 'sample')]
        run(command, True)
        output = run([str(work / 'sample')])
    elif family in ('swift', 'objective-c'):
        product = 'ThinkThen' if family == 'swift' else 'ThinkThenFoundation'
        settings = '' if family == 'swift' else ', cSettings: [.unsafeFlags(["-fobjc-arc", "-fblocks"])], linkerSettings: [.linkedFramework("Foundation")]'
        (work / 'Sources/RegistryExample').mkdir(parents=True)
        sample.rename(work / 'Sources/RegistryExample' / ('main.swift' if family == 'swift' else 'main.m'))
        (work / 'Package.swift').write_text('// swift-tools-version: 6.0\nimport PackageDescription\nlet package = Package(name: "RegistryExample", platforms: [.macOS(.v15)], dependencies: [.package(path: ' + json.dumps(str(package)) + ')], targets: [.executableTarget(name: "RegistryExample", dependencies: [.product(name: ' + json.dumps(product) + ', package: ' + json.dumps(package.name) + ')]' + settings + ')])\n')
        run(['swift', 'build', '--disable-sandbox', '--jobs', '2'], True)
        output = run([str(work / '.build/debug/RegistryExample')])
    elif family == 'csharp':
        feed = work / 'feed'; feed.mkdir()
        nupkg = list(package.rglob('*.nupkg')) or list(packages.glob('*.nupkg'))
        if len(nupkg) != 1:
            raise Refusal('missing-or-ambiguous-example-nupkg')
        shutil.copyfile(nupkg[0], feed / nupkg[0].name)
        (work / 'example.csproj').write_text('<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net8.0</TargetFramework><ImplicitUsings>enable</ImplicitUsings></PropertyGroup><ItemGroup><PackageReference Include="Botassembly.ThinkThen" Version="' + version(root) + '" /></ItemGroup></Project>')
        env.update(DOTNET_CLI_HOME=str(home), NUGET_PACKAGES=str(work / 'nuget'),
                   DOTNET_CLI_TELEMETRY_OPTOUT='1', DOTNET_SKIP_FIRST_TIME_EXPERIENCE='1')
        dotnet = os.environ.get('THINKTHEN_DOTNET', 'dotnet')
        run([dotnet, 'restore', 'example.csproj', '--source', str(feed)], True)
        run([dotnet, 'build', 'example.csproj', '--no-restore', '-o', str(work / 'bin')], True)
        output = run([dotnet, str(work / 'bin/example.dll')])
    elif family == 'dart':
        (work / 'pubspec.yaml').write_text('name: registry_example\nenvironment:\n  sdk: ">=3.0.0 <4.0.0"\ndependencies:\n  thinkthen_dart:\n    path: ' + json.dumps(str(package)) + '\n')
        env['PUB_CACHE'] = os.environ.get('PUB_CACHE', str(Path.home() / '.pub-cache'))
        dart = os.environ.get('TT_DART', 'dart')
        run([dart, 'pub', 'get', '--offline'], True)
        run([dart, 'compile', 'exe', str(sample), '-o', str(work / 'sample')], True)
        output = run([str(work / 'sample')])
    elif family == 'r':
        (work / 'rlib').mkdir()
        env['R_LIBS_USER'] = str(work / 'rlib') + os.pathsep + os.environ.get('R_LIBS', str(Path.home() / '.cache/thinkthen-toolchains/r-library'))
        run(['R', 'CMD', 'INSTALL', '-l', str(work / 'rlib'), str(package)], True)
        output = run(['Rscript', '--vanilla', str(sample)])
    elif family == 'sqlite':
        library = next(package.rglob('libthinkthen0.so'))
        output = run(['sqlite3', '-batch', ':memory:'], input='.bail on\n.load ' + str(library) + '\n' + code)
    elif family == 'duckdb':
        source_id = run(['duckdb', '-csv', '-noheader', '-init', '/dev/null', '-c',
                         'SELECT source_id FROM pragma_version();']).strip()
        pins = dict(re.findall(r'(?m)^([A-Z0-9_]+)=(.*)$', (root / 'databases/duckdb/tools/version.env').read_text()))
        versions = [pins['DUCKDB_ROUTINE_RELEASE'] if kind == 'RELEASE' else pins['DUCKDB_ROUTINE_PREVIEW_SOURCE'][:10]
                    for kind in ('RELEASE', 'PREVIEW') if pins['DUCKDB_ROUTINE_' + kind + '_SOURCE'].startswith(source_id)]
        libraries = [path for path in package.rglob('*.duckdb_extension') if any(pin in path.parts for pin in versions)]
        if len(libraries) != 1:
            raise Refusal('duckdb-example-host-member-mismatch')
        library = libraries[0]
        output = run(['duckdb', '-unsigned', '-list', '-init', '/dev/null'], input="LOAD '" + str(library) + "';\n" + code)
    elif family == 'postgresql' and os.uname().sysname == 'Linux':
        pin = re.search(r'(?m)^VERSION=(.*)$', (root / 'databases/postgresql/runtime.sh').read_text()).group(1)
        tree = work / 'server'
        shutil.copytree(Path(os.environ.get('THINKTHEN_TOOLCHAINS', str(Path.home() / '.cache/thinkthen-toolchains'))) / 'postgresql' / pin, tree, symlinks=True)
        for suffix in ('usr/lib/postgresql/16/lib', 'usr/share/postgresql/16/extension'):
            shutil.copytree(package / suffix, tree / suffix, dirs_exist_ok=True)
        binaries = tree / 'usr/lib/postgresql/16/bin'
        data = work / 'data'; socket = work / 'socket'; socket.mkdir()
        run([str(binaries / 'initdb'), '-D', str(data), '--auth=trust', '-U', 'postgres'])
        with (data / 'postgresql.conf').open('a') as config:
            config.write("\nlisten_addresses = ''\nunix_socket_directories = '" + str(socket) + "'\n")
        try:
            run([str(binaries / 'pg_ctl'), '-D', str(data), '-l', str(work / 'server.log'), '-w', '-t', '20', 'start'])
            output = run(['psql', '-X', '-v', 'ON_ERROR_STOP=1', '-h', str(socket), '-U', 'postgres', '-d', 'postgres'], input='CREATE EXTENSION thinkthen;\n' + code)
        finally:
            if (data / 'postmaster.pid').is_file():
                run([str(binaries / 'pg_ctl'), '-D', str(data), '-m', 'immediate', '-w', '-t', '20', 'stop'])
    else:
        raise Refusal('example-preparation-unavailable: ' + family)
    if expected is not None and output != expected:
        raise Refusal('example-output-mismatch: ' + family)


def published_families(root=REPO):
    workflow = (root / '.github/workflows/release.yml').read_text()
    inventory = (root / 'sdlc/scripts/package-inventory.py').read_text()
    declarations = []
    for name in ('release-pack', 'release-smoke', 'release-container'):
        text = (root / 'sdlc/scripts' / name).read_text()
        declarations += re.findall(r'(?:for family in|set --) ([\w -]+)(?:;|$)', text, re.M)
        declarations += re.findall(r'release-pack[^\n]+', text) if name == 'release-container' else []
    declarations += re.findall(r'release-pack[^\n]+', workflow)
    declared = set(re.findall(r'[a-z][a-z-]*', ' '.join(declarations)))
    declared.update(re.findall(r"choices=\['npm', '([^']+)', '([^']+)'\]", inventory)[0])
    if 'sdlc/scripts/package' in workflow:
        declared.add('crate')
    if (root / 'composer.json').is_file():
        declared.add('packagist')
    return tuple(family for family in PUBLIC_FAMILIES if family in declared)


def check_examples(packages, root=REPO, families=None):
    failures = []
    with tempfile.TemporaryDirectory(prefix='thinkthen-registry-examples-') as folder:
        scratch = Path(folder)
        (scratch / 'OWNER').write_text('ThinkThen public package README replay\n')
        roster = published_families(root)
        if families is not None and not set(families) <= set(roster):
            raise Refusal('unpublished-example-family')
        for family in families if families is not None else roster:
            selected = [None]
            if family == 'duckdb':
                native = 'apple-darwin' if os.uname().sysname == 'Darwin' else 'linux'
                selected = [path for path in packages.glob('thinkthen-duckdb-*.tar.gz')
                            if {'arm64': 'aarch64'}.get(os.uname().machine, os.uname().machine) in path.name and native in path.name] or [None]
            for index, package in enumerate(selected):
                work = scratch / (family + '-' + str(index)); work.mkdir()
                try:
                    for sample_index in range(len(examples(family, root))):
                        sample_work = work / str(sample_index); sample_work.mkdir()
                        run_example(family, packages, sample_work, root, package, sample_index)
                        print('registry-readmes: replayed ' + family + ' block ' + str(sample_index + 1) + (' ' + package.name if package else ''))
                except (Refusal, OSError, StopIteration, subprocess.SubprocessError) as error:
                    failures.append(str(error) or ('example-did-not-run: ' + family))
    if failures:
        raise Refusal('\n'.join(failures))


def archive_family(path):
    name = path.name
    if name.endswith('.crate'): return 'crate'
    if name.endswith('.whl'): return 'python'
    if name.endswith('.gem'): return 'ruby'
    if name.endswith('.nupkg'): return 'csharp'
    if name.startswith('thinkthen_'): return 'r'
    for family in PUBLIC_FAMILIES:
        if name.startswith('thinkthen-' + family + '-') or (family == 'postgresql' and name.startswith('thinkthen-postgresql16-')):
            return family
    if name.endswith('.tgz'): return 'typescript'
    if re.match(r'thinkthen-\d', name): return 'command'
    raise Refusal('unknown-package-family')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('family', choices=PUBLIC_FAMILIES + ('auto', 'sources', 'examples'))
    parser.add_argument('packages', type=Path, nargs='*')
    parser.add_argument('--family', dest='example_families', choices=PUBLIC_FAMILIES, action='append')
    args = parser.parse_args()
    if args.family == 'examples':
        if len(args.packages) != 1 or not args.packages[0].is_dir():
            parser.error('examples requires one directory of release packages')
        check_examples(args.packages[0].resolve(), families=args.example_families)
        return
    if args.example_families:
        parser.error('--family is only for example replay')
    if args.family == 'sources':
        check_sources()
        print('registry-readmes: checked public sources (drafts; no approval implied)')
        return
    if not args.packages:
        parser.error('package paths are required')
    for package in args.packages:
        family = archive_family(package) if args.family == 'auto' else args.family
        check_package(family, package)
        print('registry-readmes: checked ' + package.name)


if __name__ == '__main__':
    try:
        main()
    except (Refusal, OSError, KeyError, StopIteration, AttributeError) as error:
        raise SystemExit('registry-readmes: ' + str(error))
