#!/usr/bin/env python3
"""Check packed public text against its authoritative source without contacting registries."""
import argparse
import email
import gzip
import hashlib
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
    text = data.decode('utf-8')
    if family in ('command', 'packagist'):
        # Composer renders the preserved docs README; only its pitch and versions
        # belong to the public package-page boundary.
        if opening(text) != OPENING:
            raise Refusal('shared-opening: ' + family)
        check_versions(text, version(root))
        return
    check_text(data, version(root))
    if len(text.splitlines()) >= 40:
        raise Refusal('readme-needs-under-40-lines: ' + family)
    if data != source(family, root).read_bytes():
        raise Refusal('readme-source-mismatch: ' + family)
    if opening(text) != opening((root / 'README.md').read_text()):
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
        body = metadata.get_payload(decode=True)
        # Maturin terminates the metadata description with one extra newline.
        # Accept that framing only when the remaining bytes are the source.
        if body.removesuffix(b'\n') == source(family, root).read_bytes():
            body = body.removesuffix(b'\n')
        check_readme(family, body, root)
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
    failures = []
    for family in PUBLIC_FAMILIES:
        for check in (lambda: check_readme(family, source(family, root).read_bytes(), root),
                      lambda: check_description(family, text, root) if (text := description(family, root)) is not None else None):
            try:
                check()
            except (Refusal, OSError, KeyError, AttributeError) as error:
                failures.append(family + ': ' + str(error))
    descriptor = (root / 'sdlc/records/0577-description-draft.yml').read_text()
    descriptor_opening = re.search(r'  extended_description: \|\n    (.*)', descriptor)
    pr_opening = (root / 'sdlc/records/0577-pr-description-draft.md').read_text().split('\n\n')[1:2]
    expected = opening((root / 'README.md').read_text())
    if descriptor_opening is None or descriptor_opening.group(1) != expected or pr_opening != [expected]:
        failures.append('shared-opening: duckdb-submission')
    if failures:
        raise Refusal('\n'.join(failures))


SAMPLES = {
    'command': ('sh', 'sample.sh'), 'packagist': ('sh', 'sample.sh'),
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


FUNCTION_NAMES = '(' + '|'.join(variant['const'] for variant in json.loads(
    (REPO / 'specification/request.schema.json').read_text())['$defs']['RequestFunction']['oneOf']) + ')'
DATABASE_HELPERS = {'sqlite': set(), 'postgresql': {'thinkthen_record'},
                    'duckdb': {'thinkthen_record', 'thinkthen_location'}}
FUNCTION = r'(?<![\w])(?:thinkthen_|tt_)?' + FUNCTION_NAMES


def host_functions(code, family):
    # Canonical C requests and COBOL CALL operands carry function names in strings.
    # Shell commands use words rather than host invocation syntax.
    if family in ('command', 'packagist'):
        return {name.lower() for name in re.findall(FUNCTION + r'(?![\w])', code, re.I)}
    comments = r'//[^\n]*|/\*[\s\S]*?\*/'
    if family in ('python', 'ruby', 'r', 'php'):
        comments += r'|\#[^\n]*'
    if family == 'ada':
        comments = r'--[^\n]*'
    if family == 'cobol':
        comments = r'\*>[^\n]*'
    # Mask literals before seeking calls, including triple and verbatim strings.
    literals = r'"""[\s\S]*?"""|\'\'\'[\s\S]*?\'\'\'|@"(?:""|[^"])*"|"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|`[^`]*`'
    if family == 'ada':
        # Attributes use apostrophes; only one character is a character literal.
        # Ada strings escape a quotation mark by doubling it, not with a backslash.
        literals = r'"(?:""|[^"])*"|\'[^\n]\''
    elif family == 'crate':
        # Lifetimes remain code. Rust characters and raw strings remain literals.
        comments = r'//[^\n]*|/\*'
        literals = (r'(?:br|r)(?P<hashes>\#*)"[\s\S]*?"(?P=hashes)|'
                    r'"(?:\\[\s\S]|[^"\\])*"|\'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^\'\\\n])\'')
    functions = set()
    named_requests = {}
    if family == 'cobol':
        # Match CALL operands before masking other literals, including DISPLAY text.
        operands = r'\bCALL\s+["\'](?:thinkthen_|tt_)' + FUNCTION_NAMES + r'["\'](?=\s+USING\b)'
        def mask_cobol(token):
            if token.lastindex:
                functions.add(token[1].lower())
            return ' '
        re.sub(operands + '|' + comments + '|' + literals, mask_cobol, code, flags=re.I)
        return functions
    def mask(token):
        if family == 'c' and token[0].startswith('"'):
            try:
                request = json.loads(json.loads(token[0]))
                function = request['call']['function']
                if re.fullmatch(FUNCTION_NAMES, function):
                    functions.add(function)
            except (ValueError, TypeError, KeyError):
                pass
            # A blocking call's literal name belongs to its explicit request record.
            # Count it only when that exact record is passed to the exported call.
            declaration = re.search(r'\bthinkthen_named_request_v1\s+(\w+)\s*=\s*\{\s*\.function\s*=\s*\{\s*$',
                                    code[:token.start()])
            if declaration:
                name = json.loads(token[0])
                if isinstance(name, str) and re.fullmatch(FUNCTION_NAMES, name):
                    named_requests[declaration[1]] = name
        return ' '
    if family == 'crate':
        # Rust block comments nest, and their contents are never string tokens.
        tokens = re.compile(literals + '|' + comments)
        pieces, pos = [], 0
        while token := tokens.search(code, pos):
            pieces.append(code[pos:token.start()])
            end = token.end()
            if token[0] == '/*':
                depth = 1
                for delimiter in re.finditer(r'/\*|\*/', code[end:]):
                    depth += 1 if delimiter[0] == '/*' else -1
                    if not depth:
                        end += delimiter.end()
                        break
                else:
                    end = len(code)
            pieces.append(' ')
            pos = end
        pieces.append(code[pos:])
        code = ''.join(pieces)
    else:
        code = re.sub(literals + '|' + comments, mask, code)
    if family == 'c':
        for request in re.findall(r'\bthinkthen_run_call\s*\(\s*[^,;]+,\s*&\s*(\w+)\s*,', code):
            if request in named_requests:
                functions.add(named_requests[request])
    suffix = 'Async' if family == 'csharp' else ''
    # Rust turbofish and typed host arguments remain part of the same named call.
    generic = r'(?:\s*(?:::)?\s*<[\w\s.:,<>\[\]&?\']+>)?' if family in ('crate', 'cpp', 'typescript', 'csharp') else ''
    invocation = r'\s*:' if family == 'objective-c' else generic + r'\s*\('
    return functions | {name.lower() for name in re.findall(FUNCTION + suffix + invocation, code, re.I)}


def database_functions(code, family):
    # SQL strings and comments cannot stand in for executable named calls.
    tokens = re.compile(r"--[^\n]*|/\*|[eE]'(?:\\.|''|[^'\\])*'|'(?:''|[^'])*'|"
                        r'\$(\w*)\$[\s\S]*?\$\1\$|"(?:""|[^"])*"')
    if family == 'sqlite':
        tokens = re.compile(tokens.pattern + r'|`(?:``|[^`])*`|\[[^\]]*\]')
    pieces, pos = [], 0
    while token := tokens.search(code, pos):
        pieces.append(code[pos:token.start()])
        end = token.end()
        if token[0] == '/*':
            depth = 1
            for delimiter in re.finditer(r'/\*|\*/', code[end:]):
                depth += 1 if delimiter[0] == '/*' else -1
                if not depth:
                    end += delimiter.end()
                    break
            else:
                end = len(code)
        # Preserve only complete route identifiers; call syntax is checked after masking.
        quoted = token[0][1:-1] if token[0].startswith(('"', '`', '[')) else ''
        pieces.append(quoted if re.fullmatch(r'(?:thinkthen_|tt_)\w+', quoted, re.I) else ' ')
        pos = end
    pieces.append(code[pos:])
    sql = ''.join(pieces)
    functions = set()
    for route in re.findall(r'\b((?:thinkthen_|tt_)[A-Za-z_]\w*)"?\s*\(', sql, re.I):
        call = re.fullmatch(r'(?:thinkthen_|tt_)' + FUNCTION_NAMES + r'(?:_details)?', route, re.I)
        if not call:
            if route.lower() in DATABASE_HELPERS[family]:
                continue
            raise Refusal('unsupported-database-example-route: ' + route)
        functions.add(call[1].lower())
    return functions


def examples(family, root=REPO):
    language, filename = SAMPLES[family]
    text = source(family, root).read_text()
    blocks = list(re.finditer(r'```' + language + r'\n(.*?)\n```', text, re.S))
    samples = []
    functions = set()
    for block in blocks:
        code = block.group(1) + '\n'
        calls = (database_functions(code, family) if family in ('sqlite', 'postgresql', 'duckdb')
                 else host_functions(code, family))
        # Installation belongs to package assembly, not offline sample replay.
        if not calls and re.fullmatch(
                r'(?:(?:install\.packages\([^\n]*\)|CREATE EXTENSION thinkthen;|'
                r'INSTALL thinkthen FROM community;|LOAD (?:thinkthen|\x27[^\n]+\x27);|\.load [^\n]*|'
                r'curl [^\n]*install\.sh[^\n]*|brew install [^\n]*)\n)+', code):
            continue
        functions.update(calls)
        output = re.match(r'\s*```text\n(.*?)\n```', text[block.end():], re.S)
        samples.append((filename, code, output.group(1) + '\n' if output else None))
    if not samples:
        raise Refusal('missing-example: ' + family)
    if family not in ('command', 'packagist') and len(functions) < 2:
        raise Refusal('example-needs-two-functions: ' + family)
    return samples


def example(family, root=REPO):
    return examples(family, root)[0]


def unpack_example_package(family, packages, destination, selected=None, text_failures=None):
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
    try:
        check_package(family, package)
    except Refusal as error:
        if text_failures is None:
            raise
        text_failures.append(family + ' packed text: ' + str(error))
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
    if family == 'flutter':
        return destination / 'flutter'
    if family == 'r':
        return destination / 'thinkthen'
    if family == 'crate':
        return destination / ('thinkthen-' + version())
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


def run_example(family, packages, work, root=REPO, selected=None, sample_index=0, text_failures=None):
    filename, code, expected = examples(family, root)[sample_index]
    if family == 'packagist':
        family = 'command'
    package = unpack_example_package(family, packages, work / 'package', selected, text_failures)
    home = work / 'home'; home.mkdir()
    cache = work / 'cache'; cache.mkdir(mode=0o700)
    # The site uses this same recording as a fresh cache, without shell settings or a key.
    shutil.copyfile(root / 'site/recordings/thinkthen.jsonl', cache / 'thinkthen.jsonl')
    sample = work / filename; sample.write_text(code)
    env = {**{name: os.environ[name] for name in ('GITHUB_ACTIONS', 'RUNNER_ENVIRONMENT') if name in os.environ},
           'PATH': os.environ.get('PATH', '/usr/bin:/bin'), 'HOME': str(home),
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
            raise Refusal('example-did-not-run: ' + family + '\n' + done.stdout + done.stderr)
        return done.stdout

    if family == 'command':
        if '< message.txt' in code:
            demo = root / 'demos/01-refund-gate'
            shutil.copyfile(demo / 'message.txt', work / 'message.txt')
            shutil.copyfile(demo / 'recording/thinkthen.jsonl', cache / 'thinkthen.jsonl')
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
        output = run(['php', '-n', '-d', 'extension=ffi', '-d', 'ffi.enable=1', str(sample)])
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
        native_package = unpack_example_package('c', packages, work / 'native', text_failures=text_failures)
        library = native_package / 'lib'
        if family == 'cpp':
            installed = package
            if not (installed / 'lib/cmake/thinkthen-cpp').is_dir():
                installed = work / 'installed'
                run(['cmake', '-S', str(package), '-B', str(work / 'wrapper-build'),
                     '-DCMAKE_INSTALL_PREFIX=' + str(installed),
                     '-DTHINKTHEN_NATIVE_SHARED=' + str(library / 'libthinkthen.so'),
                     '-DTHINKTHEN_NATIVE_STATIC=' + str(library / 'libthinkthen.a'),
                     '-DTHINKTHEN_C_HEADER=' + str(native_package / 'include/thinkthen.h')], True)
                run(['cmake', '--build', str(work / 'wrapper-build'), '--target', 'install', '-j2'], True)
            (work / 'CMakeLists.txt').write_text('cmake_minimum_required(VERSION 3.18)\n'
                'project(RegistryExample LANGUAGES CXX)\n'
                'find_package(thinkthen-cpp CONFIG REQUIRED)\n'
                'add_executable(sample sample.cpp)\n'
                'target_link_libraries(sample PRIVATE thinkthen::thinkthen_cpp_shared)\n')
            run(['cmake', '-S', str(work), '-B', str(work / 'consumer-build'),
                 '-DCMAKE_PREFIX_PATH=' + str(installed)], True)
            run(['cmake', '--build', str(work / 'consumer-build'), '-j2'], True)
            output = run([str(work / 'consumer-build/sample')])
            if expected is not None and output != expected:
                raise Refusal('example-output-mismatch: ' + family)
            return
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
                   DOTNET_CLI_TELEMETRY_OPTOUT='1', DOTNET_SKIP_FIRST_TIME_EXPERIENCE='1',
                   DOTNET_CLI_WORKLOAD_UPDATE_NOTIFY_DISABLE='true')
        dotnet = os.environ.get('THINKTHEN_DOTNET', 'dotnet')
        (work / 'NuGet.Config').write_text('<configuration><packageSources><clear/><add key="local" value="' + str(feed) + '"/></packageSources><auditSources><clear/></auditSources></configuration>')
        run([dotnet, 'restore', 'example.csproj', '--configfile', str(work / 'NuGet.Config'), '-p:NuGetAudit=false'], True)
        run([dotnet, 'build', 'example.csproj', '--no-restore', '-o', str(work / 'bin')], True)
        output = run([dotnet, str(work / 'bin/example.dll')])
    elif family in ('dart', 'flutter'):
        core = unpack_example_package('dart', packages, work / 'dart', text_failures=text_failures) if family == 'flutter' else package
        native = unpack_example_package('c', packages, work / 'native', text_failures=text_failures)
        manifest = json.loads((core / 'native-assets.json').read_text())
        target = {'x86_64': 'x86_64', 'arm64': 'aarch64'}.get(os.uname().machine, os.uname().machine) + '-unknown-linux-gnu'
        assets = [asset for asset in manifest['assets'].values() if asset['target'] == target]
        if len(assets) != 1:
            raise Refusal('dart-example-native-target-mismatch')
        asset = assets[0]
        binary = packages / ('libthinkthen-dart.so' if asset['file'] == 'libthinkthen.so' else asset['file'])
        if not binary.is_file():
            binary = native / 'lib' / asset['file']
        if hashlib.sha256(binary.read_bytes()).hexdigest() != asset['sha256']:
            raise Refusal('dart-example-native-asset-mismatch')
        assets_cache = work / 'assets'; assets_cache.mkdir()
        (assets_cache / asset['sha256']).mkdir()
        shutil.copyfile(binary, assets_cache / asset['sha256'] / asset['file'])
        dependency = 'thinkthen_flutter' if family == 'flutter' else 'thinkthen_dart'
        override = '\ndependency_overrides:\n  thinkthen_dart:\n    path: ' + json.dumps(str(core)) if family == 'flutter' else ''
        (work / 'pubspec.yaml').write_text('name: registry_example\nenvironment:\n  sdk: ">=3.10.0 <4.0.0"\ndependencies:\n  ' + dependency + ':\n    path: ' + json.dumps(str(package)) + override + '\nhooks:\n  user_defines:\n    thinkthen_dart:\n      offline: true\n      asset_cache: ' + json.dumps(str(assets_cache) + '/') + '\n')
        env['PUB_CACHE'] = os.environ.get('PUB_CACHE', str(Path.home() / '.pub-cache'))
        dart = os.environ.get('TT_DART', 'dart')
        env['FLUTTER_SUPPRESS_ANALYTICS'] = 'true'
        pub = os.environ.get('TT_FLUTTER', 'flutter') if family == 'flutter' else dart
        run([pub, 'pub', 'get', '--offline'], True)
        run([dart, 'build', 'cli', '-t', str(sample), '-o', str(work / 'dart-build')], True)
        output = run([str(work / 'dart-build/bundle/bin/sample')])
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
        output = run(['duckdb', '-unsigned', '-bail', '-list', '-init', '/dev/null'], input="LOAD '" + str(library) + "';\n" + code)
        print('registry-readmes: DuckDB source ' + source_id + ' loaded ' + str(library.relative_to(package)))
    elif family == 'postgresql' and os.uname().sysname == 'Linux':
        major = re.match(r'thinkthen-postgresql(16|17|18)-', selected.name).group(1)
        pins = (root / 'databases/postgresql/runtime.sha256').read_text().splitlines()
        machine = {'x86_64': 'amd64', 'aarch64': 'arm64'}[os.uname().machine]
        server = next(line.split()[1] for line in pins if line.split()[1].startswith(f'postgresql-{major}_') and line.endswith(f'_{machine}.deb'))
        pin = server.split('_')[1]
        tree = work / 'server'
        shutil.copytree(Path(os.environ.get('THINKTHEN_TOOLCHAINS', str(Path.home() / '.cache/thinkthen-toolchains'))) / 'postgresql' / pin, tree, symlinks=True)
        for member, suffix in (('lib', f'usr/lib/postgresql/{major}/lib'),
                               ('extension', f'usr/share/postgresql/{major}/extension')):
            shutil.copytree(package / member, tree / suffix, dirs_exist_ok=True)
        binaries = tree / f'usr/lib/postgresql/{major}/bin'
        if major != '16':
            triplet = {'amd64': 'x86_64-linux-gnu', 'arm64': 'aarch64-linux-gnu'}[machine]
            env['LD_LIBRARY_PATH'] = str(tree / 'usr/lib' / triplet)
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
    if family == 'c':
        check_c_calls(output)
    if expected is not None and output != expected:
        raise Refusal('example-output-mismatch: ' + family)


def check_c_calls(output):
    """Read each complete owned Call; retain the README's six exact answers."""
    calls = [json.loads(line) for line in output.splitlines()]
    wanted = [('decide', [True, False, True]), ('choose', ['billing', 'shipping', 'account'])]
    if len(calls) != len(wanted):
        raise Refusal('c-example-call-count')
    for call, (function, values) in zip(calls, wanted):
        answer = call.get('answer', {})
        actual = answer.get('value')
        matches = (isinstance(actual, list) and len(actual) == len(values)
                   and all(type(got) is type(want) and got == want
                           for got, want in zip(actual, values)))
        if (answer.get('function') != function or not matches
                or call.get('facts', {}).get('records') != len(values)):
            raise Refusal('c-example-answer-or-facts: ' + json.dumps({
                'function': answer.get('function'), 'value': actual,
                'records': call.get('facts', {}).get('records'),
                'expected_function': function, 'expected_value': values}))


def check_examples(packages, root=REPO, families=None):
    failures = []
    with tempfile.TemporaryDirectory(prefix='thinkthen-registry-examples-') as folder:
        scratch = Path(folder)
        (scratch / 'OWNER').write_text('ThinkThen public package README replay\n')
        # Source-wrapper pages ship even when their registry publication is disabled.
        roster = PUBLIC_FAMILIES
        if families is not None and not set(families) <= set(roster):
            raise Refusal('unknown-public-example-family')
        for family in families if families is not None else roster:
            selected = [None]
            if family in ('duckdb', 'postgresql'):
                native = 'apple-darwin' if os.uname().sysname == 'Darwin' else 'linux'
                selected = [path for path in packages.glob('thinkthen-' + family + '*.tar.gz')
                            if {'arm64': 'aarch64'}.get(os.uname().machine, os.uname().machine) in path.name and native in path.name] or [None]
            for index, package in enumerate(selected):
                work = scratch / (family + '-' + str(index)); work.mkdir()
                try:
                    samples = examples(family, root)
                except (Refusal, OSError) as error:
                    failures.append(family + ': ' + str(error))
                    continue
                for sample_index in range(len(samples)):
                    sample_work = work / str(sample_index); sample_work.mkdir()
                    try:
                        run_example(family, packages, sample_work, root, package, sample_index, failures)
                        print('registry-readmes: replayed ' + family + ' block ' + str(sample_index + 1) + (' ' + package.name if package else ''))
                    except (Refusal, OSError, StopIteration, subprocess.SubprocessError) as error:
                        failures.append(family + ' block ' + str(sample_index + 1) + ': ' +
                                        (str(error) or 'example-did-not-run'))
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
        if name.startswith('thinkthen-' + family + '-') or (family == 'postgresql' and re.match(r'thinkthen-postgresql1[678]-', name)):
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
