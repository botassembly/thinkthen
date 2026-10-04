"""Channel installers for a clean, explicitly dispatched consumer check."""
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
from install_check import Failure, check_index, check_result, validate_version
from install_check_consumers import write_consumer
from install_check_postgresql import stop_postgres

TARGET = 'x86_64-unknown-linux-gnu'
HOMEBREW_TAP = 'https://github.com/botassembly/homebrew-thinkthen.git'


def selected_formula(history, version):
    """Return a formula from public history without substituting today's version."""
    validate_version(version)
    for formula in history:
        if re.search(r'^\s*version\s+[\"\']' + re.escape(version) + r'[\"\']\s*$', formula, re.M):
            return formula
    raise Failure(f'Homebrew tap history does not contain thinkthen {version}')


def dcf_packages(text):
    packages = {}
    for paragraph in re.split(r'\n\s*\n', text.strip()):
        fields = {}
        for line in paragraph.splitlines():
            if line.startswith((' ', '\t')):
                continue
            key, sep, value = line.partition(':')
            if sep:
                fields[key] = value.strip()
        if 'Package' in fields:
            packages[fields['Package']] = fields
    return packages


def sql_quote(text):
    return "'" + str(text).replace("'", "''") + "'"


def native_library(check):
    return check.c_archive() / 'lib/libthinkthen.so'


def native_call(check, command):
    return check.response(*command, check.root / 'settings.json', check.root / 'request.json')


def local_tap(check, formula):
    selected = check.root / 'selected-tap'
    (selected / 'Formula').mkdir(parents=True)
    (selected / 'Formula/thinkthen.rb').write_text(formula)
    check.run('git', 'init', selected)
    check.run('git', '-C', selected, 'add', 'Formula/thinkthen.rb')
    check.run('git', '-C', selected, '-c', 'user.name=Install check',
              '-c', 'user.email=install-check@localhost', '-c', 'commit.gpgsign=false',
              'commit', '-m', 'Select requested formula')
    return selected


def homebrew(check):
    tap_name = 'installcheck/selected'
    formula_name = tap_name + '/thinkthen'
    if tap_name in check.run('brew', 'tap').splitlines():
        raise Failure('Homebrew check tap already exists; refusing to replace it')
    previous = check.run('brew', 'list', '--formula', '--full-name').splitlines()
    if any(name.rsplit('/', 1)[-1] == 'thinkthen' for name in previous):
        raise Failure('Homebrew thinkthen formula already exists; refusing to replace it')
    tap = check.root / 'public-tap'
    check.run('git', 'clone', HOMEBREW_TAP, tap)
    revisions = check.run('git', '-C', tap, 'log', '--format=%H', '--diff-filter=AM', '--all', '--', 'Formula/thinkthen.rb').splitlines()
    history = (check.run('git', '-C', tap, 'show', f'{revision}:Formula/thinkthen.rb') for revision in revisions)
    formula = selected_formula(history, check.version)
    selected = local_tap(check, formula)
    # Homebrew clones this committed local repository. No tap is published.
    check.run('brew', 'tap', tap_name, selected)
    primary = None
    try:
        check.run('brew', 'install', formula_name)
        command = Path(check.run('brew', '--prefix', formula_name)) / 'bin/thinkthen'
        installed, reply = check.command_replay(command)
        check_result(check.version, installed, reply)
        return installed, reply, 'installed binary'
    except (Failure, OSError, subprocess.TimeoutExpired) as error:
        primary = error
        raise
    finally:
        try:
            # Both the tap and formula were absent before this run. Remove only
            # the exact formula from our tap, including a partially failed install.
            present = check.run('brew', 'list', '--formula', '--full-name').splitlines()
            if formula_name in present:
                check.run('brew', 'uninstall', '--formula', formula_name)
            check.run('brew', 'untap', tap_name)
        except (Failure, OSError, subprocess.TimeoutExpired) as cleanup:
            if primary is None:
                raise Failure(f'Homebrew cleanup failed: {cleanup}') from cleanup
            print(f'install-check: Homebrew cleanup also failed: {cleanup}', file=sys.stderr)


def python(check, uv=False):
    source = write_consumer(check.project, 'python', 'consumer.py')
    if uv:
        check.run('uv', 'init', '--bare', '--python', '3.13', check.project)
        check.run('uv', 'add', '--no-build', f'thinkthen=={check.version}')
        command = ['uv', 'run', '--no-sync', 'python']
    else:
        venv = check.root / 'venv'
        check.run(sys.executable, '-m', 'venv', venv)
        executable = venv / 'bin/python'
        check.run(executable, '-m', 'pip', 'install', '--only-binary=:all:', f'thinkthen=={check.version}')
        command = [executable]
    installed = check.run(*command, '-c', 'from importlib.metadata import version; print(version("thinkthen"))')
    return installed, check.response(*command, source, check.sample), 'installed package metadata'


def rust(check, library=False):
    if not library:
        destination = check.root / 'cargo-bin'
        check.run('cargo', '+1.95.0', 'install', 'thinkthen', '--version', check.version, '--locked', '--root', destination)
        installed, reply = check.command_replay(destination / 'bin/thinkthen')
        return installed, reply, 'installed binary'
    check.run('cargo', '+1.95.0', 'init', '--name', 'installcheck', '--bin', check.project)
    check.run('cargo', '+1.95.0', 'add', f'thinkthen@={check.version}')
    write_consumer(check.project / 'src', 'rust', 'main.rs')
    metadata = check.response('cargo', '+1.95.0', 'metadata', '--format-version', '1')
    installed = next(p['version'] for p in metadata['packages'] if p['name'] == 'thinkthen')
    reply = check.response('cargo', '+1.95.0', 'run', '--quiet', '--locked', '--', check.sample)
    return installed, reply, 'resolved crate metadata'


def npm(check):
    (check.project / 'package.json').write_text('{"private":true}')
    check.run('npm', 'install', '--save-exact', f'thinkthen@{check.version}')
    installed = json.loads((check.project / 'node_modules/thinkthen/package.json').read_text())['version']
    source = write_consumer(check.project, 'node', 'consumer.cjs')
    return installed, check.response('node', source, check.sample), 'installed package metadata'


def rubygems(check):
    check.run('gem', 'install', 'thinkthen', '--version', check.version, '--no-document')
    installed = check.run('ruby', '-e', 'require "thinkthen"; puts Gem.loaded_specs.fetch("thinkthen").version')
    source = write_consumer(check.project, 'ruby', 'consumer.rb')
    return installed, check.response('ruby', source, check.sample), 'installed gem metadata'


def nuget(check):
    native_library(check)
    check.run('dotnet', 'new', 'console', '--name', 'InstallCheck', '--framework', 'net8.0')
    check.run('dotnet', 'add', 'package', 'Botassembly.ThinkThen', '--version', check.version)
    source = write_consumer(check.project, 'csharp', 'Program.cs')
    assets = json.loads((check.project / 'obj/project.assets.json').read_text())
    installed = next(name.split('/')[1] for name in assets['libraries'] if name.startswith('Botassembly.ThinkThen/'))
    check.run('dotnet', 'build', '--no-restore', '--nologo', '--verbosity', 'quiet')
    return installed, native_call(check, ['dotnet', check.project / 'bin/Debug/net8.0/InstallCheck.dll']), 'resolved NuGet metadata'


def maven(check):
    library = native_library(check)
    dependencies = ''.join(f'<dependency><groupId>io.github.botassembly</groupId><artifactId>thinkthen-jvm</artifactId><version>{check.version}</version>{classifier}</dependency>'
                           for classifier in ('', '<classifier>kotlin</classifier>', '<classifier>scala</classifier>'))
    (check.project / 'pom.xml').write_text(f'<project><modelVersion>4.0.0</modelVersion><groupId>installcheck</groupId><artifactId>consumer</artifactId><version>1</version><dependencies>{dependencies}</dependencies></project>')
    check.run('mvn', f'-Dmaven.repo.local={check.root / "maven-cache"}', '-q', 'dependency:copy-dependencies', '-DoutputDirectory=dependencies')
    jar = check.project / f'dependencies/thinkthen-jvm-{check.version}.jar'
    import xml.etree.ElementTree as ET
    pom = check.root / f'maven-cache/io/github/botassembly/thinkthen-jvm/{check.version}/thinkthen-jvm-{check.version}.pom'
    installed = ET.parse(pom).getroot().findtext('{http://maven.apache.org/POM/4.0.0}version')
    source = write_consumer(check.project, 'java', 'InstallCheck.java')
    check.run('javac', '--enable-preview', '--release', '21', '-cp', str(jar), source)
    return installed, native_call(check, ['java', '--enable-preview', '--enable-native-access=ALL-UNNAMED', f'-Dthinkthen.library={library}', '-cp', f'{check.project}:{jar}', 'InstallCheck']), 'resolved Maven POM metadata'


def pub(check):
    library = native_library(check)
    (check.project / 'pubspec.yaml').write_text("name: installcheck\nenvironment:\n  sdk: '>=3.13.0 <4.0.0'\n")
    check.run('dart', 'pub', 'add', f'thinkthen_dart:{check.version}')
    lock = (check.project / 'pubspec.lock').read_text()
    match = re.search(r'(?ms)^  thinkthen_dart:\n(.*?)(?=^  \w|^sdks:|\Z)', lock)
    installed = re.search(r'^    version: "?([^"\n]+)', match.group(1), re.M).group(1)
    source = write_consumer(check.project, 'dart', 'consumer.dart')
    return installed, native_call(check, ['dart', 'run', source, library]), 'resolved pub metadata'


def packagist(check):
    index = json.loads(check.text('https://repo.packagist.org/p2/botassembly/thinkthen.json', 'packagist.json'))
    listed = [entry.get('version', '').removeprefix('v') for entry in index['packages']['botassembly/thinkthen']]
    check_index('Packagist', check.version, listed)
    check.run('composer', 'require', '--no-interaction', '--no-plugins', '--no-scripts', f'botassembly/thinkthen:{check.version}')
    entries = json.loads((check.project / 'vendor/composer/installed.json').read_text())
    installed = next(p['version'].removeprefix('v') for p in entries['packages'] if p['name'] == 'botassembly/thinkthen')
    library = native_library(check)
    source = write_consumer(check.project, 'php', 'consumer.php')
    return installed, native_call(check, ['php', '-d', 'ffi.enable=true', source, library]), 'installed Composer metadata'


def go(check):
    module = 'github.com/botassembly/thinkthen/libraries/go'
    listed = check.text(f'https://proxy.golang.org/{module}/@v/list', 'go-index').splitlines()
    check_index('Go proxy', check.version, [v.removeprefix('v') for v in listed])
    native_library(check)
    check.run('go', 'mod', 'init', 'installcheck')
    check.run('go', 'get', f'{module}@v{check.version}')
    installed = check.response('go', 'list', '-m', '-json', module)['Version'].removeprefix('v')
    source = write_consumer(check.project, 'go', 'main.go')
    check.run('go', 'build', '-o', 'consumer', source)
    return installed, native_call(check, [check.project / 'consumer']), 'resolved Go module metadata'


def r_universe(check):
    base = 'https://botassembly.r-universe.dev/bin/linux/resolute-x86_64/4.6/src/contrib'
    packages = dcf_packages(check.text(base + '/PACKAGES', 'r-binary-index'))
    entry = packages.get('thinkthen')
    if entry is None:
        source = dcf_packages(check.text('https://botassembly.r-universe.dev/src/contrib/PACKAGES', 'r-source-index')).get('thinkthen')
        if source and source.get('Version') == check.version:
            check_index('R-universe', check.version, check.version, binary=False)
        check_index('R-universe', check.version, None)
    check_index('R-universe', check.version, entry['Version'])
    filename = entry.get('File', f'thinkthen_{check.version}.tar.gz')
    if filename != f'thinkthen_{check.version}.tar.gz':
        raise Failure('R-universe binary index names an unexpected file')
    archive = check.fetch(base + '/' + filename, check.root / filename)
    Path(check.env['R_LIBS_USER']).mkdir()
    import tarfile
    with tarfile.open(archive) as packed:
        description = packed.extractfile('thinkthen/DESCRIPTION').read().decode()
        packed_names = packed.getnames()
    fields = dcf_packages('Package: thinkthen\n' + description).get('thinkthen', {})
    if not re.search(r'^R 4\.6\.[0-9]+;.*linux', fields.get('Built', '')) or 'thinkthen/libs/thinkthen.so' not in packed_names:
        raise Failure('R-universe binary address returned a source archive; a Linux binary is required')
    check.run('R', 'CMD', 'INSTALL', '--no-test-load', '--library=' + check.env['R_LIBS_USER'], archive)
    installed = check.run('Rscript', '-e', 'cat(as.character(packageVersion("thinkthen")))')
    source = write_consumer(check.project, 'r', 'consumer.R')
    return installed, check.response('Rscript', source, check.sample), 'installed R package metadata'


def c(check):
    native = check.c_archive()
    source = write_consumer(check.project, 'c', 'consumer.c')
    flags = check.run('pkg-config', '--cflags', '--libs', 'thinkthen').split()
    check.run('gcc', source, '-o', 'consumer', *flags)
    command = check.project / 'consumer'
    installed = check.run(command, '--version')
    return installed, native_call(check, [command]), 'installed C header version'


def sqlite(check):
    archive = f'thinkthen-sqlite-{check.version}-{TARGET}.tar.gz'
    folder = check.unpack(check.release(archive), check.root / 'sqlite')
    extension = folder / 'libthinkthen0.so'
    if not extension.is_file():
        raise Failure('SQLite release asset is missing its extension')
    host = check.run('sqlite3', '--version').split()[0]
    if validate_version(host) < (3, 50, 4):
        raise Failure(f'SQLite host {host} is older than 3.50.4')
    settings = (check.root / 'settings.json').read_text()
    evidence = (check.sample / 'report.txt').read_text()
    question = (check.sample / 'question.txt').read_text()
    sql = f'.load {extension}\nSELECT thinkthen_configure({sql_quote(settings)});\nSELECT thinkthen_details({sql_quote(question)},{sql_quote(evidence)});\n'
    lines = check.run('sqlite3', '-bail', ':memory:', input=sql).splitlines()
    details = json.loads(lines[-1])
    return check.version, {'value': details.get('value'), 'requests_sent': details.get('meta', {}).get('requests_sent')}, 'release-asset identity'


def duckdb(check):
    folder = check.unpack(check.release(f'thinkthen-duckdb-{check.version}-{TARGET}.tar.gz'), check.root / 'duckdb')
    extension = folder / 'thinkthen.duckdb_extension'
    sql = f'LOAD {sql_quote(extension)}; SET thinkthen_replay={sql_quote(check.sample / "recording")}; SET thinkthen_cache=\'off\';\n'
    sql += "SELECT extension_version FROM duckdb_extensions() WHERE extension_name='thinkthen';\n"
    sql += f'SELECT thinkthen_details({sql_quote((check.sample / "question.txt").read_text())},{sql_quote((check.sample / "report.txt").read_text())});\n'
    lines = check.run('duckdb', '-unsigned', '-noheader', '-list', ':memory:', input=sql).splitlines()
    details = json.loads(lines[-1])
    return lines[-2], {'value': details.get('value'), 'requests_sent': details.get('meta', {}).get('requests_sent')}, 'loaded extension version'


def postgresql(check):
    folder = check.unpack(check.release(f'thinkthen-postgresql16-{check.version}-{TARGET}.tar.gz'), check.root / 'postgresql')
    # Runtime setup supplies a private relocatable PGDG tree inside this run's scratch.
    runtime = check.root / 'pg-runtime'
    from install_check_tools import postgres_tools
    postgres_tools(check, runtime)
    binary = runtime / 'usr/lib/postgresql/16/bin'
    shared = runtime / 'usr/share/postgresql/16/extension'
    library = runtime / 'usr/lib/postgresql/16/lib'
    for source in (folder / 'extension').iterdir():
        shutil.copy2(source, shared / source.name)
    for source in (folder / 'lib').iterdir():
        shutil.copy2(source, library / source.name)
    data, socket = check.root / 'pg-data', check.root / 'pg-socket'
    socket.mkdir(mode=0o700)
    check.run(binary / 'initdb', '-D', data, '-A', 'trust', '--no-locale', '-E', 'UTF8')
    primary = None
    try:
        check.run(binary / 'pg_ctl', '-D', data, '-l', check.root / 'postgres.log', '-o', f"-k {socket} -c listen_addresses=''", 'start', '-w')
        sql = f'CREATE EXTENSION thinkthen; SET thinkthen.replay={sql_quote(check.sample / "recording")}; SET thinkthen.cache=\'off\';\n'
        sql += "SELECT extversion FROM pg_extension WHERE extname='thinkthen';\n"
        sql += f'SELECT thinkthen_details({sql_quote((check.sample / "question.txt").read_text())},{sql_quote((check.sample / "report.txt").read_text())});\n'
        lines = check.run(binary / 'psql', '-XAt', '-v', 'ON_ERROR_STOP=1', '-h', socket, '-d', 'postgres', input=sql).splitlines()
        details = json.loads(lines[-1])
        reply = {'value': details.get('value'), 'requests_sent': details.get('meta', {}).get('requests_sent')}
        check_result(check.version, lines[-2], reply)
        return lines[-2], reply, 'installed SQL extension version'
    except Exception as error:
        primary = error
        raise
    finally:
        try:
            stop_postgres(check, data, socket, binary)
        except Exception as cleanup:
            check.retain_scratch = True
            if primary is None:
                raise Failure(f'PostgreSQL cleanup failed: {cleanup}') from cleanup
            print(f'install-check: PostgreSQL cleanup also failed: {cleanup}', file=sys.stderr)


def download(check):
    script = check.fetch('https://raw.githubusercontent.com/botassembly/thinkthen/main/install.sh', check.root / 'install.sh')
    check.run('sh', script, '--version', check.version)
    installed, reply = check.command_replay(check.root / '.local/bin/thinkthen')
    return installed, reply, 'installed binary'


INSTALLERS = {'download': download, 'homebrew': homebrew, 'cargo-install': rust,
              'cargo-add': lambda check: rust(check, library=True), 'pip': python,
              'uv': lambda check: python(check, uv=True), 'npm': npm, 'rubygems': rubygems,
              'nuget': nuget, 'maven': maven, 'pub': pub, 'packagist': packagist,
              'go': go, 'r-universe': r_universe, 'c': c, 'sqlite': sqlite,
              'duckdb': duckdb, 'postgresql': postgresql}


def install(check):
    return INSTALLERS[check.channel](check)
