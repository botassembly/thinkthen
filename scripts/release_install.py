"""Finish a distribution with ordinary R archives and user install instructions."""
import hashlib
import json
from pathlib import Path
import tarfile

from release_readme import package_readmes
from release_python_dependencies import wheelhouse
from native_platform import HOSTS, FIELDS
from duckdb_release import VERSION as DUCKDB_VERSION


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
    from release_r_dependencies import directories, refresh
    for folder in directories(out):
        refresh(folder)


def install_note(out):
    build = json.loads((out / 'build.json').read_text())
    requirements = json.loads((out / 'requirements.json').read_text())
    version, host = build['version'], build['platform']
    mac = host == 'macos-arm64'
    aliases = dict(zip(FIELDS, next(row for row in HOSTS.values() if row[0] == host)))
    rid, suffix = aliases['rid'], aliases['suffix']
    python_dependencies = wheelhouse(out).relative_to(out).as_posix() if "python" in requirements else ""
    commands = {
        'command': f'tar -xzf thinkthen-{version}-{host}.tar.gz && printf "evidence 5" | ./thinkthen decide --backend stub "Ready?" --json',
        'python': f'python3 -m venv .smoke-python && .smoke-python/bin/python -m pip install --no-index --find-links {python_dependencies} python/*.whl && .smoke-python/bin/python checks/python.py checks/stub.json',
        'typescript': 'npm install --offline --ignore-scripts --no-audit --no-fund ./typescript/*.tgz && cp checks/node.mjs .smoke-node.mjs && node .smoke-node.mjs checks/stub.json',
        'go': 'THINKTHEN_STUB_CASES="$PWD/checks/stub.json" THINKTHEN_CASES="$PWD/checks" ./checks/go-consumer -test.run=^TestSharedStub$ -test.v',
        'c': 'tar -xzf c/*.tar.gz -C c && python3 checks/native_cases.py checks/c-consumer checks/stub.json checks/contract.json',
        'cpp': 'tar -xzf c/*.tar.gz -C c && tar -xzf cpp/*.tar.gz -C cpp && python3 checks/native_cases.py checks/cpp-consumer checks/stub.json checks/contract.json utf8',
        'java': 'java --enable-native-access=ALL-UNNAMED -cp "checks/jvm/consumer.jar:java/*:java/dependencies/*" ConsumerCases checks/stub.json --stub-only && java --enable-native-access=ALL-UNNAMED -cp "checks/jvm/consumer.jar:java/*:java/dependencies/*" JavaApiCases --stub-only',
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
CREATE SCHEMA IF NOT EXISTS thinkthen;
CREATE EXTENSION IF NOT EXISTS thinkthen SCHEMA thinkthen;
SELECT format('ALTER DATABASE %I SET thinkthen.allowed_backends = %L', current_database(), 'stub:*')
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
             'Install the command on Linux or macOS with `curl -fsSL https://thinkthen.dev/install.sh | sh`. On Windows PowerShell, run `irm https://thinkthen.dev/install.ps1 | iex`.', '',
             'The installers add their command folder to your user PATH for new terminals. Set `THINKTHEN_NO_MODIFY_PATH=1` for print-only PATH instructions. The receipt lists any shell profile or registry changes.', '',
             'Download the command, C, C++ and Zig archives from GitHub Releases. Verify SHA256SUMS before extracting an archive. The platform package archives also contain offline installation commands in INSTALL.md and package READMEs.', '',
             'Package-site installation follows publication of 0.2.0 on each site. Use PyPI `thinkthen` for Python, npm `thinkthen` for TypeScript, RubyGems `thinkthen` for Ruby, NuGet `Botassembly.ThinkThen` for C#, and Maven Central `io.github.botassembly:thinkthen-java` or `io.github.botassembly:thinkthen-kotlin` for Java or Kotlin. Install the command with `brew install botassembly/thinkthen/thinkthen`. Install R `thinkthen` from `https://botassembly.r-universe.dev`. Use the Go module `github.com/botassembly/thinkthen/sdk/go/thinkthen` at `v' + version + '`. Select version `' + version + '` with Swift Package Manager. Rust users add the `thinkthen` library from crates.io with `cargo add thinkthen --no-default-features` and install its command with `cargo install thinkthen`.', '',
             'Download the DuckDB and SQLite extensions from GitHub Releases. PostgreSQL ships in the platform package archives for Linux x86-64 and arm64 servers running PostgreSQL 18 with glibc 2.28 or later. PGXN installation and the DuckDB community listing are outside this release.', '',
             '## For 0.1 users', '',
             'Version 0.2.0 changes the command and SDK APIs and provides no compatibility shims. Update scripts and callers against the current surface guides. It does not include the Ada, COBOL, Dart, Objective-C, PHP or Scala bindings.', '',
             'The commands `status`, `check`, `cache`, `transform`, `audit`, `diff`, `prune`, `unused` and `convert` are removed, together with their command-specific options, including audit’s `--seed`. Rewrite workflows that used them with the current commands and ordinary file-processing tools.', '',
             'Use `--json` for detailed JSON answers; the output schema changes, so update consumers of the old detailed result objects. Current `--details` adds status, probability, error, detail and source columns to table answers. Input paths are positional instead of `--input`. Replace `--lines`, `--jsonl`, `--csv` and `--tsv` with `--from lines`, `--from jsonl`, `--from csv` and `--from tsv`. Use `--field` with a JSON Pointer such as `/text` to select a value within each record.', '',
             'For decision bands, `--threshold LOW:HIGH` becomes `--low LOW --high HIGH`. Replace `--batch max` with a numeric batch size; `--batch` now takes a record count.', '',
             'The options `--on-error`, `--plan`, `--dry-run`, `--timeout`, `--profile`, `--max-requests-total`, `--max-estimated-input-tokens-total`, `--facts`, `--context`, `--jobs`, `--max-retries`, `--max-request-bytes` and `--max-text-bytes` are removed. Remove them from scripts and review request limits and error handling against the current API.', '',
             'The old answer and question options `--raw`, `--option`, `--options`, `--relation`, `--relation-threshold`, `--kind-field` and `--none` are removed. For decide and filter, `--true TEXT` and `--false TEXT` describe what yes and no mean. Supply choices with `--choice` and scale levels with `--level`; `--label` and `--kind` take names rather than the old NAME=DESCRIPTION syntax. Use `--rule` for relations. `--quiet` now hides notices and progress; use `--check` when a supported command should return its answer as an exit code. Rewrite saved question files and `@FILE` callers as current toolboxes and `thinkthen run` entries.', '',
             f'Intel macOS and static musl Linux builds are no longer supplied. Use Apple Silicon on macOS and glibc Linux builds with glibc {FLOORS["linux"]["glibc"]} or newer; Swift has the separate floor listed below.', '',
             'The built-in backend names are now `systemone`, `openai` and `stub`. The 0.1.2 names `typesafe`, `liquid` and `ollama` are no longer built in. Select `systemone` for the default TypeSafe service and configure your client again for the selected backend.', '',
             'Maven users replace `io.github.botassembly:thinkthen-jvm` with `io.github.botassembly:thinkthen-java` or `io.github.botassembly:thinkthen-kotlin`. Rust users use the single `thinkthen` crate for both the library and command. The Rust features `polars`, `bundled-sqlite` and `host-sqlite` are removed. Its default `command` feature replaces the old `cli` feature; the command installs with `cargo install thinkthen`.', '',
             'Go users replace imports of `github.com/botassembly/thinkthen/libraries/go` with `github.com/botassembly/thinkthen/sdk/go/thinkthen`. Language packages now carry their matching native engine; do not mix them with the separate native library from 0.1.2.', '',
             'The answer cache and the `--record`, `--replay`, `--cache`, `--no-cache` and `--refresh-cache` options from 0.1.2 are absent. Model-backed calls send their questions again. Use the offline `stub` backend for installation checks; it returns synthetic answers.', '',
             'Use the current surface guides when updating callers. The command selects a backend with `--backend` and structured output with `--json`. SDKs expose typed single-record results and collection APIs; compile callers against the current API and its matching native library.', '',
             'For the command, `THINKTHEN_URL` sets the address for either backend. System One also reads `THINKTHEN_BASE_URL`. An explicit `--url` takes precedence over address variables. `OPENAI_BASE_URL` is not read.', '',
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
                database_runtimes.setdefault(name, requirements[name]['runtime'])
    language_requirements = []
    for name, value in FLOORS['runtimes'].items():
        # Only Swift needs a separate runtime floor after its build requirement.
        requirement = value.split(',')[0] if name == 'swift' else value.split(';')[0]
        language_requirements.append(('Kotlin: ' if name == 'kotlin' else '') + requirement)
    lines += ['', 'Language requirements: ' + '; '.join(language_requirements) + '.', '',
              f'The optional C++ JSON adapter headers and license are in nlohmann-json-{version}.tar.gz on this release.', '',
              f'R source packages require R >=4.2.0. Supplied R binary packages require the {FLOORS["r"]["Linux"]} series on Linux and the {FLOORS["r"]["Darwin"]} series on macOS; Windows requires ' + platforms.get('windows-x86_64', {}).get('r', {}).get('runtime', 'the R version in its archive requirements.json').split(';')[0] + '. C needs a C compiler to build, and C++ needs C++17. See each archive’s requirements.json for its package requirements and supplied runtime dependencies.', '',
              'Database requirements: ' + '; '.join(database_runtimes.values()) + '.', '',
              'ThinkThen uses 0.x APIs. APIs may change between releases.', '']
    limits = r"""## Known limits

Backend requests can incur charges even when a query returns fewer rows or is cancelled. Read these limits before making paid calls.

| Surface or operation | Limit and action |
| --- | --- |
| SQLite joins and correlated subqueries | Putting a ThinkThen function on the inner side of a join or in a correlated subquery can execute it repeatedly and repeat paid backend calls. Materialize one standalone ThinkThen query into a temporary table before joining or querying its answers. Reexecuting any database query can make another paid call. |
| DuckDB loading | The extension requires exactly DuckDB {duckdb} and unsigned extension loading. Start the CLI with `duckdb -unsigned` before `LOAD`. Other DuckDB versions are not supported by this binary. |
| Unsigned macOS binaries | The binaries are unsigned and not notarized. After verifying the release checksum, remove quarantine from the extracted directory with `xattr -dr com.apple.quarantine /path/to/thinkthen` if macOS blocks loading or execution. Apply this only to the verified download. |
| Unsigned Windows binaries | The binaries are unsigned. After verifying the release checksum, run `Unblock-File .\thinkthen-0.2.0-windows-x86_64.zip` in PowerShell before extraction. Windows policy may still block unsigned code. |
| Database cancellation | Cancelling a DuckDB, SQLite or PostgreSQL run discards answers already obtained by that run. Backend charges for completed requests remain. Materialize completed results before later analysis; cancellation does not save a partial answer set for reuse. |
| Cancellation on every surface | Cancellation stops future sends and closes the transport. Answers from paid requests already in flight can be lost. Refunds are not promised. |
| PostgreSQL LIMIT | PostgreSQL materializes the function result before applying SQL LIMIT. LIMIT 1 can therefore pay for every input row; it is not a backend cost cap. |
| DuckDB short LIMIT and input buffering | An outer LIMIT may wait for other requests or their timeout and may pay for rows it never returns. Put LIMIT inside the input subquery to bound input work. While a request is held, DuckDB can buffer one 2048-row input vector separately from the engine iterator's read-ahead bound. |
| PostgreSQL correlated LATERAL | A correlated function invocation runs for each outer row and repeats paid backend work. The measured three-row LATERAL query sent three requests. Materialize one standalone ThinkThen query before joining its answers when the input and question can be reused. |
"""
    lines[lines.index('## Platforms and requirements'):lines.index('## Platforms and requirements')] = [limits.replace('{duckdb}', DUCKDB_VERSION), '']
    lines += [f'Resource observations are bounded measurements, not proof of leak-free operation. See the [resource observations](https://github.com/botassembly/thinkthen/blob/v{version}/docs/resource-observations.md) for completed soak intervals, measured rates and remaining uncertainty.', '']
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
