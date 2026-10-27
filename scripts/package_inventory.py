"""Share staging inputs and check delivered files against package declarations."""
import csv
import fnmatch
import hashlib
import io
import json
from pathlib import Path, PurePosixPath
import re
import shutil
import subprocess
import sys
import xml.etree.ElementTree as ET

from release_notice_check import contents

# Each pair names a source glob and its destination glob. Stagers use the same
# declarations as the source-free checker; generated/native files stay separate.
INPUTS = {
    'duckdb': [('databases/duckdb/README.md', 'README.md'), ('LICENSE', 'LICENSE'),
               ('LICENSE', 'THINKTHEN-LICENSE')],
    'command': [('LICENSE', 'LICENSE'), ('README.md', 'README.md')],
    'go': [('sdk/go/thinkthen/*' + suffix, '*' + suffix) for suffix in ('.go', '.c', '.h')]
          + [('sdk/go/thinkthen/go.mod', 'go.mod'), ('sdk/go/README.md', 'README.md'),
             ('crates/thinkthen-ffi/include/thinkthen.h', 'thinkthen.h'), ('LICENSE', 'LICENSE')],
    'sqlite': [('extensions/sqlite/README.md', 'README.md'), ('LICENSE', 'LICENSE')],
    'postgres': [('extensions/postgres/README.md', 'README.md'), ('LICENSE', 'LICENSE'),
                 ('extensions/postgres/*.sql', 'share/extension/*.sql'),
                 ('extensions/postgres/*.control', 'share/extension/*.control')],
    'swift': [('sdk/swift/Package.swift', 'Package.swift'), ('sdk/swift/README.md', 'README.md'),
              ('LICENSE', 'LICENSE'), ('sdk/swift/Sources/CThinkThen/anchor.c', 'Sources/CThinkThen/anchor.c'),
              ('crates/thinkthen-ffi/include/thinkthen.h', 'Sources/CThinkThen/include/thinkthen.h'),
              ('sdk/swift/Sources/ThinkThen/*.swift', 'Sources/ThinkThen/*.swift')],
}
DUCKDB_TERMS = {'DUCKDB-LICENSE': None, 'YYJSON-LICENSE': 'third_party/yyjson/LICENSE'}
GENERATED = {'command': 'thinkthen.1', 'go': 'native/dependencies.txt', 'swift': 'VERSION'}


def stage(family, destination, root, built=None):
    for source, pattern in INPUTS[family]:
        paths = list(root.glob(source))
        if not paths: raise ValueError('missing package input: ' + source)
        for path in paths:
            if built is not None and path.suffix == '.sql': path = built / path.name
            target = destination / (str(PurePosixPath(pattern).parent / path.name) if '*' in pattern else pattern)
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(path, target)


def duckdb_terms(source, destination):
    for name, relative in DUCKDB_TERMS.items():
        if relative is not None:
            shutil.copy2(source / relative, destination / name)
            continue
        notices = [source / 'LICENSE']
        for directory in ('third_party', 'extension'):
            notices.extend(sorted(path for path in (source / directory).rglob('*')
                                  if path.is_file() and path.name.upper().startswith(('LICENSE', 'COPYING', 'NOTICE'))))
        (destination / name).write_text(''.join(f'\n{path.relative_to(source)}\n\n' + path.read_text() for path in notices))


def r_files(files):
    tables = [name for name in files if name.endswith(('/MD5', '/SHA256'))]
    assert tables, 'missing R binary checksum inventory'
    expected = set(tables)
    for table in tables:
        algorithm = PurePosixPath(table).name.lower()
        prefix = table.rsplit('/', 1)[0] + '/'
        for row in files[table].decode().splitlines():
            fields = row.split(maxsplit=2 if algorithm == 'sha256' else 1)
            name = prefix + fields[-1].lstrip('*')
            expected.add(name)
            assert hashlib.new(algorithm, files[name]).hexdigest() == fields[0], (name, 'R checksum differs')
            if algorithm == 'sha256': assert len(files[name]) == int(fields[1]), (name, 'R size differs')
    return expected


def matches(path, pattern):
    parts, expected = PurePosixPath(path).parts, PurePosixPath(pattern).parts
    return len(parts) == len(expected) and all(fnmatch.fnmatchcase(p, e) for p, e in zip(parts, expected))


def ruby(path):
    declared = json.loads(subprocess.check_output([
        'ruby', '-rjson', '-rrubygems/package', '-e',
        'puts JSON.generate(Gem::Package.new(ARGV.fetch(0)).spec.files)', str(path)], text=True))
    actual = contents(path)
    assert set(actual) == set(declared), (path.name, 'files differ from gemspec', set(actual) - set(declared), set(declared) - set(actual))


def zig_paths(text):
    declaration = re.search(r'\.paths\s*=\s*\.\{([^}]+)\}', text)[1]
    return re.findall(r'"([^"\\]+)"', declaration)


def cmake(files):
    """Read installed include directories from CMake exports and pkg-config."""
    include_dirs = set()
    for name, data in files.items():
        if name.endswith('.cmake'):
            include_dirs.update(re.findall(r'INTERFACE_INCLUDE_DIRECTORIES\s+"\$\{(?:_IMPORT_PREFIX|PACKAGE_PREFIX_DIR)\}/([^";]+)', data.decode()))
        if name.endswith('.pc'):
            values = dict(re.findall(r'(?m)^(\w+)=(.*)$', data.decode()))
            # Resolve prefix relative to the delivered pkg-config file, not the host.
            values['pcfiledir'] = str(PurePosixPath(name).parent)
            for _ in range(len(values)):
                values = {key: re.sub(r'\$\{(\w+)\}', lambda m: values.get(m[1], m[0]), value)
                          for key, value in values.items()}
            import posixpath
            flags = re.search(r'(?m)^Cflags:\s*(.*)$', data.decode())[1]
            flags = re.sub(r'\$\{(\w+)\}', lambda m: values[m[1]], flags)
            include_dirs.update(posixpath.normpath(directory) for directory in re.findall(r'(?:^|\s)-I(\S+)', flags))
    assert include_dirs, 'missing installed include-directory metadata'
    metadata_dirs = {str(PurePosixPath(name).parent) for name in files if name.endswith(('Config.cmake', '.pc'))}
    return {name for name in files if any(name.startswith(directory + '/') for directory in include_dirs)
            or (str(PurePosixPath(name).parent) in metadata_dirs and name.endswith(('.cmake', '.pc')))
            or PurePosixPath(name).name in ('LICENSE', 'README.md')}


def check(family, path, native):
    files = contents(path)
    actual = {p for p in files if 'THIRD-PARTY-NOTICES' not in p.split('/')}
    if family == 'ruby':
        ruby(path)
        return
    if family == 'r':
        expected = r_files(files)
        assert set(files) == expected, (path.name, 'files differ from R checksum inventory', set(files) ^ expected)
        return
    if family == 'python':
        record, = [name for name in files if name.endswith('.dist-info/RECORD')]
        expected = {row[0] for row in csv.reader(io.StringIO(files[record].decode()))}
        assert set(files) == expected, (path.name, 'files differ from wheel RECORD', set(files) ^ expected)
        return
    if family in INPUTS:
        patterns = [destination for _, destination in INPUTS[family]]
        expected = {name for name in actual if any(matches(name, pattern) for pattern in patterns)}
        expected.update(pattern for pattern in patterns if '*' not in pattern)
        expected.update(native)
        if family in GENERATED: expected.add(GENERATED[family])
        if family == 'duckdb': expected.update(DUCKDB_TERMS)
    elif family in ('c', 'cpp'):
        expected = cmake({name: files[name] for name in actual}) | set(native)
    elif family == 'zig':
        manifest, = [name for name in actual if name.endswith('/build.zig.zon')]
        prefix = manifest.removesuffix('build.zig.zon')
        declared = zig_paths(files[manifest].decode())
        expected = {name for name in actual if any(name == prefix + entry or name.startswith(prefix + entry + '/') for entry in declared)}
    elif family in ('java', 'kotlin'):
        # A module descriptor is standard JAR metadata, not an application class.
        expected = {name for name in actual if name.startswith(('META-INF/', 'io/thinkthen/'))
                    or (family == 'java' and name == 'module-info.class')}
    elif family == 'typescript':
        manifest = json.loads(files['package/package.json'])
        expected = {'package/' + name for name in manifest['files']} | {'package/package.json'}
    elif family == 'csharp':
        types = ET.fromstring(files['[Content_Types].xml'])
        extensions = {node.attrib['Extension'] for node in types if node.tag.endswith('}Default')}
        parts = {node.attrib['PartName'].lstrip('/') for node in types if node.tag.endswith('}Override')}
        expected = {name for name in actual if PurePosixPath(name).name.rsplit('.', 1)[-1] in extensions
                    or name in parts or name == 'LICENSE'}
        expected.add('[Content_Types].xml')
    else:
        raise ValueError('no inventory declaration: ' + family)
    assert actual == expected, (path.name, 'undeclared or missing files', actual - expected, expected - actual)


if __name__ == '__main__':
    if sys.argv[1] == 'stage':
        stage(sys.argv[2], Path(sys.argv[3]), Path(__file__).resolve().parent.parent,
              Path(sys.argv[4]) if len(sys.argv) == 5 else None)
    elif sys.argv[1] == 'duckdb-terms':
        duckdb_terms(Path(sys.argv[2]), Path(sys.argv[3]))
    elif sys.argv[1] == 'generated':
        print(GENERATED[sys.argv[2]])
    elif sys.argv[1] == 'ruby':
        ruby(Path(sys.argv[2]))
    else:
        raise SystemExit('usage: package_inventory.py stage FAMILY DESTINATION | generated FAMILY | ruby GEM')
