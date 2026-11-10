"""Finish a distribution with ordinary R archives and user install instructions."""
import hashlib
import json
from pathlib import Path
import tarfile

ROOT = Path(__file__).resolve().parent.parent

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


def known_limits():
    """The release notes page's known-limits table, the one list both notes share."""
    page = (ROOT / 'site/src/content/docs/reference/release-notes.mdx').read_text(encoding='utf-8')
    table = page.split('\n## Known limits\n', 1)[1].split('\n## ', 1)[0].strip()
    assert table.startswith('| Limit |') and f'DuckDB {DUCKDB_VERSION}' in table, 'release notes: known limits'
    return table.replace('](/', '](https://thinkthen.dev/') + '\n'


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
             'The commands `status`, `check`, `cache` and `transform` are removed. This includes the old `cache prune`, `cache unused` and `cache convert` subcommands and their `--max-size`, `--older-than`, `--answered-by-other-than`, `--used` and `--quote` options. Rewrite those workflows with the current commands and ordinary file-processing tools.', '',
             'Use `--json` for detailed JSON answers; the output schema changes, so update consumers of the old detailed result objects. Current `--details` adds status, probability, error, detail and source columns to table answers. Input paths are positional instead of `--input`. Replace `--lines`, `--jsonl`, `--csv` and `--tsv` with `--from lines`, `--from jsonl`, `--from csv` and `--from tsv`. Use `--field` with a JSON Pointer such as `/text` to select a value within each record.', '',
             'For decision bands, `--threshold LOW:HIGH` becomes `--low LOW --high HIGH`. Ordinary commands exit 0 for answered or unsure results; add `--check` to decide, filter, choose or find when a script needs verdict exit codes. Replace `--batch max` with a numeric batch size; `--batch` now takes a record count. Score now returns a number from 0 to 1 rather than from 0 to the number of levels minus one.', '',
             'The options `--on-error`, `--plan`, `--dry-run`, `--timeout`, `--profile`, `--max-requests-total`, `--max-estimated-input-tokens-total`, `--facts`, `--context`, `--jobs`, `--max-retries`, `--max-request-bytes` and `--max-text-bytes` are removed. Remove them from scripts and review request limits and error handling against the current API.', '',
             'The old answer and question options `--raw`, `--option`, `--options`, `--relation`, `--relation-threshold`, `--kind-field` and `--none` are removed. For decide and filter, `--true TEXT` and `--false TEXT` describe what yes and no mean. Supply choices with `--choice` and scale levels with `--level`; `--label` and `--kind` take names rather than the old NAME=DESCRIPTION syntax. For choose and tag, add `--describe LABEL=TEXT` after supplying that label, for example `--choice billing --choice shipping --describe "billing=About charges"`. The per-record `--options` pointer has no direct replacement; construct a current question for each required option set. Use `--rule NAME SOURCE TARGET` for relations. Rewrite saved question files and `@FILE` callers as current toolboxes and `thinkthen run` entries.', '',
             '`--quiet` suppresses answers, JSON rows, notices and progress; failure diagnostics still reach standard error. Remove it when capturing answers. Use `--check --quiet` when a supported command should communicate only through its exit code, and handle false/none (1), unsure (3) and failures separately.', '',
             'Find changes its input unit: 0.1 selected a candidate record from the whole input; 0.2 selects a line within each record. Pack each candidate set into one document and retain a Unicode-scalar span-to-ID map, including duplicate display text. Multiline or blank candidates need an explicit single-line encoding; blindly joining their original text changes the choices. None is always available and must reach the same threshold as a line. An answered null means none; unsure and failed do not. Old thresholds and model-quality results do not establish parity for the new presentation. See the [candidate-set migration example](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/bash.md#migrating-candidate-set-find-calls).', '',
             'Audit and diff are available as offline tools with new 0.2 formats and syntax. Use `thinkthen audit ROWS --manifest RUN.json --key GOLD.jsonl` for saved decide/filter, choose and find rows. Compare actual runs with `thinkthen diff LEFT --left-manifest L.json --right RIGHT --right-manifest R.json`; add `--key GOLD.jsonl` for correctness. Threshold-only diff uses `thinkthen diff ROWS --manifest RUN.json --threshold P` (or `--low L --high H`) and supports decide/filter only. Create manifests and labels from current typed rows; old detailed rows, keys and recordings are not accepted as current data. The old analysis options `--by`, `--id`, `--seed`, `--target`, `--optimize`, `--write`, `--write-to`, `--match`, `--curve`, `--pooled`, `--table`, `--cases` and `--compare-threshold` are gone; audit no longer accepts `--threshold`. These tools preserve missing/unsure/failed cases, make no model calls, recommend no threshold and change no settings. See the [offline audit and comparison guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/bash.md#audit-saved-answers-offline).', '',
             'Use `--usage` for observed request/token accounting on stderr and `--max-requests N` to cap actual send attempts for one command invocation, including retries, splits and saved function steps. SDK allowances belong to an explicit client; SQL does not inherit them. Unknown tokens remain unknown, and the allowance is not a token or invoice cap. File recovery retains completed typed rows: `thinkthen recover FILE` exports them without a key or model call. Recovery is not automatic resume, and an ordinary rerun without explicit response reuse can spend again. See the [usage and recovery guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/bash.md#observe-request-and-token-usage).', '',
             f'Intel macOS and static musl Linux builds are no longer supplied. Use Apple Silicon on macOS and glibc Linux builds with glibc {FLOORS["linux"]["glibc"]} or newer; Swift has the separate floor listed below.', '',
             'The built-in backend names are now `systemone`, `openai` and `stub`. The 0.1.2 names `typesafe`, `liquid` and `ollama` are no longer built in. Select `systemone` for the default System One service and configure your client again for the selected backend.', '',
             'Maven users replace `io.github.botassembly:thinkthen-jvm` with `io.github.botassembly:thinkthen-java` or `io.github.botassembly:thinkthen-kotlin`. Rust users use the single `thinkthen` crate for both the library and command. The Rust features `polars`, `bundled-sqlite` and `host-sqlite` are removed. Its default `command` feature replaces the old `cli` feature; the command installs with `cargo install thinkthen`.', '',
             'Go users replace imports of `github.com/botassembly/thinkthen/libraries/go` with `github.com/botassembly/thinkthen/sdk/go/thinkthen`. Language packages now carry their matching native engine; do not mix them with the separate native library from 0.1.2.', '',
             'The 0.1.2 default answer cache and old `--record`, `--replay`, `--cache`, `--no-cache` and `--refresh-cache` spellings are removed. Response storage is now opt-in: supply `--response-store DIR --response-mode cache|record|replay --response-namespace NAME --model-generation NAME` together. No environment variable enables retention. Optional positive-integer limits are `--response-max-bytes`, `--response-max-entries` and `--response-max-age`. The store retains input and output text; use a private local directory. Strict replay needs no provider key and never falls back to a live call; a missing ask fails with `local.replay-miss`. The new store does not import old cache folders. See the [record-then-replay example](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/bash.md#record-cache-and-replay-responses). Use the offline `stub` backend for installation checks; its answers are synthetic and it does not support response storage.', '',
             'Use the current surface guides when updating callers. The command selects a backend with `--backend` and structured output with `--json`. SDKs expose typed single-record results and collection APIs; compile callers against the current API and its matching native library.', '',
             '`THINKTHEN_URL` sets only the System One address, taking precedence over `THINKTHEN_BASE_URL`. OpenAI reads `OPENAI_BASE_URL`. An explicit `--url` or SDK `Settings.url` overrides the address for the chosen backend, including when `THINKTHEN_BACKEND` chooses it. `THINKTHEN_MODEL` applies only to commands; in an SDK, set the model on your client.', '',
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

"""
    lines[lines.index('## Platforms and requirements'):lines.index('## Platforms and requirements')] = [limits + known_limits(), '']
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
