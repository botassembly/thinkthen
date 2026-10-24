"""Finish a distribution with ordinary R archives and user install instructions."""
import hashlib
import json
from pathlib import Path
import tarfile

from release_readme import package_readmes
from native_platform import HOSTS, FIELDS


def r_archives(out):
    """Retain installed binary contents without Python's default pax headers."""
    for path in [*sorted((out / 'r').glob('*.tgz')),
                 *sorted((out / 'r').glob('*.tar.gz')),
                 *sorted((out / 'prerequisites/r-frames').glob('*.tar.gz'))]:
        temporary = path.with_suffix('.tmp')
        with tarfile.open(path) as source, tarfile.open(temporary, 'w:gz', format=tarfile.GNU_FORMAT) as target:
            for member in source:
                member.pax_headers = {}
                target.addfile(member, source.extractfile(member) if member.isfile() else None)
        temporary.replace(path)
    path = out / 'prerequisites/r-frames/inventory.json'
    inventory = json.loads(path.read_text())
    for package in inventory['packages'].values():
        package['sha256'] = hashlib.sha256((path.parent / package['file']).read_bytes()).hexdigest()
    path.write_text(json.dumps(inventory, indent=2) + '\n')


def install_note(out):
    build = json.loads((out / 'build.json').read_text())
    requirements = json.loads((out / 'requirements.json').read_text())
    version, host = build['version'], build['platform']
    mac = host == 'macos-arm64'
    aliases = dict(zip(FIELDS, next(row for row in HOSTS.values() if row[0] == host)))
    rid, suffix = aliases['rid'], aliases['suffix']
    commands = {
        'command': f'tar -xzf thinkthen-{version}-{host}.tar.gz && printf "evidence 5" | ./thinkthen decide --adapter stub "Ready?" --details',
        'python': 'python3 -m venv .smoke-python && .smoke-python/bin/python -m pip install --no-index python/*.whl && .smoke-python/bin/python checks/python.py checks/stub.json',
        'typescript': 'npm install --offline --ignore-scripts --no-audit --no-fund ./typescript/*.tgz && cp checks/node.mjs .smoke-node.mjs && node .smoke-node.mjs checks/stub.json',
        'go': 'THINKTHEN_STUB_CASES="$PWD/checks/stub.json" ./checks/go-consumer -test.run=^TestSharedStub$ -test.v',
        'c': 'tar -xzf c/*.tar.gz -C c && ./checks/c-consumer checks/stub.json',
        'cpp': 'tar -xzf cpp/*.tar.gz -C cpp && ./checks/cpp-consumer stub checks/stub.json',
        'java': 'java --enable-native-access=ALL-UNNAMED -cp "checks/jvm/consumer.jar:java/*:java/dependencies/*" ConsumerCases checks/stub.json --stub-only',
        'kotlin': 'java --enable-native-access=ALL-UNNAMED -cp "checks/jvm/consumer.jar:kotlin/*:kotlin/dependencies/*" KotlinCasesKt checks/stub.json --stub-only',
        'csharp': 'python3 -c \'import glob,zipfile; zipfile.ZipFile(glob.glob("csharp/*.nupkg")[0]).extractall(".smoke-dotnet")\' && cp checks/csharp/* .smoke-dotnet/lib/net10.0/ && cp -R .smoke-dotnet/runtimes .smoke-dotnet/lib/net10.0/ && dotnet .smoke-dotnet/lib/net10.0/Consumer.dll checks/stub.json --stub-only',
        'ruby': 'mkdir -p .smoke-gems && export GEM_HOME="$PWD/.smoke-gems" GEM_PATH="$PWD/.smoke-gems" && gem install --local --no-document ruby/dependencies/*.gem ruby/*.gem && ruby checks/ruby.rb checks/stub.json',
        'zig': f'tar -xzf zig/*.tar.gz -C checks/zig --strip-components 3 thinkthen/native/{rid}/libthinkthen_ffi.{suffix} && python3 checks/zig.py checks/zig/consumer checks/stub.json',
                    'swift': 'python3 checks/swift.py checks/swift/Consumer checks/stub.json',
        'r': 'R_LIBS_USER="$PWD/.smoke-r" R_LIBS_SITE= Rscript --vanilla checks/r.R "$PWD"',
        'sqlite': f'tar -xzf sqlite/*.tar.gz -C sqlite && python3 checks/sql.py sqlite sqlite3 "$PWD/sqlite/thinkthen_sqlite.{suffix}" checks/stub.json',
        'duckdb': 'tar -xzf duckdb/*.tar.gz -C duckdb && python3 checks/sql.py duckdb "$PWD/prerequisites/duckdb/duckdb" "$PWD/duckdb/thinkthen.duckdb_extension" checks/stub.json',
    }
    if not mac:
        commands['swift'] = 'LD_LIBRARY_PATH="$PWD/prerequisites/swift-runtime/linux${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" ' + commands['swift']
        commands['postgres'] = """tar -xzf postgres/*.tar.gz -C postgres
cp postgres/lib/* "$(pg_config --pkglibdir)/"
cp postgres/share/extension/* "$(pg_config --sharedir)/extension/"
psql -X -v ON_ERROR_STOP=1 <<'SQL'
CREATE EXTENSION IF NOT EXISTS thinkthen;
SELECT format('ALTER DATABASE %I SET thinkthen.allowed_backends = %L', current_database(), '{"stub":["stub"]}')
\\gexec
SQL
python3 checks/sql.py postgres "$(command -v psql)" unused checks/stub.json"""

    lines = [f'# Install ThinkThen {version} ({host})', '',
             'Run each command from this folder in a separate shell. These smoke tests use the offline stub and need no key. Keep the folder intact. Commands create local install directories.', '',
             'Stock Mac Python 3.9 and Ruby 2.6 do not meet these requirements. The stock Mac sqlite3 cannot load extensions; use an extension-enabled SQLite 3.41+ host. Compatible Homebrew Python, Ruby and SQLite versions work when their ordinary commands are on PATH.', '',
             'Put the ordinary runtimes below on PATH first. Python helpers need Python 3.10 or newer. SDK source builds need the compilers listed; the supplied compiled smoke consumers need only their standard runtimes.', '',
             f'Source: {build["revision"]}. Tested Node {build["node"]}; {build["java"]}; R {build["r"]}.', '']
    for name, requirement in requirements.items():
        support = f'macOS {requirement["macos"]}' if mac else f'Linux glibc >= {requirement["supported_glibc"]}'
        lines += [f'## {name}', '', f'{requirement["runtime"]}. {support}.', '']
        if name == 'postgres':
            lines += ['Linux only. Use an isolated PostgreSQL 18 instance with PGHOST/PGUSER/PGDATABASE set. Copying extension files needs permission to the PostgreSQL installation; CREATE EXTENSION needs its administrator. The test changes only the local test database.', '']
        if name == 'swift' and not mac:
            lines += ['The supplied runtime libraries need their normal Ubuntu 22.04 system dependencies (including libxml2, ICU 70, libcurl and libedit). The smoke command selects the supplied Swift runtime directory.', '']
        lines += ['```sh', commands[name], '```', '']
    lines += ['## pandas and Polars', '',
              'The optional wheels support CPython 3.10 only. Install Python 3.10 with pip/venv and expose it as python3.10. The base ThinkThen abi3 wheel supports Python 3.10 or newer independently. This command installs both optional libraries and checks both public namespaces.', '',
              '```sh', 'python3.10 -m venv .smoke-frames && .smoke-frames/bin/python -m pip install --no-index --require-hashes --find-links prerequisites/python-frames-cp310/wheels -r prerequisites/python-frames-cp310/requirements.txt && .smoke-frames/bin/python -m pip install --no-index --no-deps python/*.whl && .smoke-frames/bin/python checks/python-frames.py', '```', '',
              'The R command above also installs the supplied matching tibble, dplyr and data.table binaries and checks the R data frame helpers.', '',
              'For application development, install the package in its surface directory using that language\'s normal package tools. Runtime dependencies and licenses are beside the packages. See requirements.json for tested platform details and each package README for its API.', '',
              'Default-backend use is a separate human trial: set THINKTHEN_API_KEY in your shell, then use CLI, Python, TypeScript or DuckDB without the stub selection. Never put keys in this folder.', '']
    (out / 'INSTALL.md').write_text('\n'.join(lines))


def release_note(version, platforms):
    """Describe the release once; platform archives retain their install guides."""
    from runtime_floors import FLOORS

    lines = [f'# ThinkThen {version}', '',
             'Ask typed questions about records from Python, TypeScript, R, Go, Java, Kotlin, C#, C, C++, Swift, Zig and Ruby. Use the `thinkthen` command or query DuckDB, SQLite and PostgreSQL. Work with pandas, Polars and R data frames. Read and write CSV, TSV and Excel files. Run saved questions and steps from toolboxes with `thinkthen run`.', '',
             '## Install', '',
             'Download the command, C, C++ and Zig archives from GitHub Releases. Verify SHA256SUMS before extracting an archive. The platform package archives also contain offline installation commands in INSTALL.md and package READMEs.', '',
             'Package-site installation follows publication of 0.2.0 on each site. Use PyPI `thinkthen` for Python, npm `thinkthen` for TypeScript, RubyGems `thinkthen` for Ruby, NuGet `Botassembly.ThinkThen` for C#, and Maven Central `io.github.botassembly:thinkthen-java` or `io.github.botassembly:thinkthen-kotlin` for Java or Kotlin. Install the command through the Homebrew tap `botassembly/homebrew-thinkthen`. Install R `thinkthen` from `https://botassembly.r-universe.dev`. Use the Go module `github.com/botassembly/thinkthen/sdk/go` at `v' + version + '`. Select version `' + version + '` with Swift Package Manager. Rust users add the `thinkthen` library from crates.io with `cargo add thinkthen` and install its command with `cargo install thinkthen`.', '',
             'Download the DuckDB and SQLite extensions from GitHub Releases. PostgreSQL ships for Linux in the platform package archives. PGXN installation and the DuckDB community listing are outside this release.', '',
             '## For 0.1 users', '',
             'Version 0.2.0 does not include the Ada, COBOL, Dart, Objective-C or PHP bindings.', '',
             'The built-in backend names are now `systemone`, `openai` and `stub`. The 0.1.2 names `typesafe`, `liquid` and `ollama` are no longer built in. Select `systemone` for the default TypeSafe service and configure your client again for the selected backend.', '',
             'Maven users replace `io.github.botassembly:thinkthen-jvm` with `io.github.botassembly:thinkthen-java` or `io.github.botassembly:thinkthen-kotlin`. The NuGet package remains `Botassembly.ThinkThen`, the name already used by 0.1.2. Rust users use the single `thinkthen` crate for both the library and command. Its default `command` feature replaces the old `cli` feature; the command installs with `cargo install thinkthen`.', '',
             '## Platforms and requirements', '',
             '| Platform | Minimum system libraries or operating system |',
             '| --- | --- |']
    for host, requirements in platforms.items():
        if host.startswith('linux-'):
            base, swift = requirements['command'], requirements['swift']
            floor = (f'glibc {base["supported_glibc"]}, GLIBCXX {base["supported_glibcxx"]}; '
                     f'Swift: glibc {swift["supported_glibc"]}, GLIBCXX {swift["supported_glibcxx"]}')
        elif host == 'macos-arm64':
            floor = f'macOS {requirements["command"]["macos"]} or newer (Apple Silicon)'
        else:
            floor = FLOORS['windows']['support']
        lines.append(f'| {host} | {floor} |')
    database_runtimes = {}
    for requirements in platforms.values():
        for name in ('sqlite', 'duckdb', 'postgres'):
            if name in requirements:
                runtime = requirements[name]['runtime']
                values = database_runtimes.setdefault(name, [])
                if runtime not in values:
                    values.append(runtime)
    language_requirements = []
    for name, value in FLOORS['runtimes'].items():
        # Only Swift needs a separate runtime floor after its build requirement.
        requirement = value.split(',')[0] if name == 'swift' else value.split(';')[0]
        language_requirements.append(('Kotlin: ' if name == 'kotlin' else '') + requirement)
    lines += ['', 'Language requirements: ' + '; '.join(language_requirements) + '.', '',
              f'R requires the {FLOORS["r"]["Linux"]} series on Linux and the {FLOORS["r"]["Darwin"]} series on macOS; Windows requires ' + platforms.get('windows-x86_64', {}).get('r', {}).get('runtime', 'the R version in its archive requirements.json').split(';')[0] + '. C needs a C compiler to build, and C++ needs C++17. See each archive’s requirements.json for its package requirements and supplied runtime dependencies.', '',
              'Database requirements: ' + '; '.join(', '.join(values) for values in database_runtimes.values()) + '.', '',
              'ThinkThen uses 0.x APIs. APIs may change between releases.', '']
    return '\n'.join(lines)


def checksums(out):
    (out / 'SHA256SUMS').write_text(''.join(
        f'{hashlib.sha256(path.read_bytes()).hexdigest()}  {path.relative_to(out)}\n'
        for path in sorted(out.rglob('*')) if path.is_file() and path != out / 'SHA256SUMS'))


def finish(out):
    out = Path(out)
    r_archives(out)
    package_readmes(out)
    install_note(out)
    checksums(out)
