"""Install every Windows SDK from its candidate and exercise the shared stub cases.

Run only on native Windows with the ordinary runtimes from requirements.json.
Missing packages, compilers or runtimes fail the check; no surface is skipped.
"""
from contextlib import contextmanager
import json
import os
from pathlib import Path
from package_inventory import python_wheelhouse as wheelhouse
from long_path_scratch import long_path_scratch
import shutil
import subprocess
import sys
import zipfile
import xml.etree.ElementTree as ET


def run(*arguments, cwd, env, timeout=600, resolve_path=None, stream_output=False):
    command = list(map(str, arguments))
    # Invoke the runtime behind known Windows launchers directly.
    lookup = resolve_path if resolve_path is not None else env['PATH']
    executable = shutil.which(command[0], path=lookup)
    if executable is None:
        raise RuntimeError('Required Windows runtime is missing: ' + command[0])
    command[0] = executable
    if executable.lower().endswith(('.cmd', '.bat')):
        if Path(executable).stem.lower() == 'npm':
            script = Path(executable).parent / 'node_modules/npm/bin/npm-cli.js'
            runtime = shutil.which('node', path=lookup)
        elif Path(executable).stem.lower() == 'gem':
            script = Path(executable).with_suffix('')
            runtime = shutil.which('ruby', path=lookup)
        else:
            raise RuntimeError('Unsupported Windows runtime launcher: ' + executable)
        if runtime is None or not script.is_file():
            raise RuntimeError('Missing installed runtime behind ' + executable)
        command = [runtime, str(script), *command[1:]]
    if stream_output:
        from release_windows_cache import run_consumer
        status, output = run_consumer(command, env, cwd=cwd)
        result = subprocess.CompletedProcess(command, status, stdout=output, stderr='')
    else:
        result = subprocess.run(command, cwd=cwd, env=env, capture_output=True,
                                text=True, encoding='utf-8', errors='replace', timeout=timeout)
    if result.returncode:
        output = '\n'.join((result.stdout + result.stderr).splitlines()[-30:])
        raise RuntimeError(f'{Path(executable).name} exited {result.returncode}:\n{output}')
    return result.stdout


def unpack(path, destination):
    destination.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(path) as archive:
        archive.extractall(destination)
    return destination


def environment(scratch):
    # cli's Windows exit finalizer reads the OS architecture before cancelling its thread.
    keep = {'PATH', 'SYSTEMROOT', 'SYSTEMDRIVE', 'WINDIR', 'COMSPEC', 'PROGRAMFILES',
            'PROCESSOR_ARCHITECTURE',
            'PROGRAMFILES(X86)', 'PROGRAMDATA', 'INCLUDE', 'LIB', 'LIBPATH',
            'VCINSTALLDIR', 'VCTOOLSINSTALLDIR', 'WINDOWSSDKDIR', 'WINDOWSSDKVERSION',
            'UCRTVERSION', 'UCRTCONTENTROOT', 'SDKROOT', 'SWIFT_DEVELOPER_DIR', 'CC', 'CXX',
            'DOTNET_ROOT', 'DOTNET_ROOT_X64', 'THINKTHEN_LOOPBACK'}
    env = {key: value for key, value in os.environ.items() if key.upper() in keep}
    for name, folder in [('HOME', 'home'), ('USERPROFILE', 'home'), ('APPDATA', 'roaming'),
                         ('LOCALAPPDATA', 'local'), ('TEMP', 'tmp'), ('TMP', 'tmp')]:
        path = scratch / folder
        path.mkdir(exist_ok=True)
        env[name] = str(path)
    env.update(PYTHONUTF8='1', PYTHONDONTWRITEBYTECODE='1', PIP_NO_INDEX='1', PIP_DISABLE_PIP_VERSION_CHECK='1',
               npm_config_offline='true', npm_config_audit='false', npm_config_fund='false',
               GOPROXY='off', GOSUMDB='off', GOTOOLCHAIN='local', GOWORK='off', CGO_ENABLED='1',
               DOTNET_CLI_TELEMETRY_OPTOUT='1', DOTNET_SKIP_FIRST_TIME_EXPERIENCE='1',
               DOTNET_GENERATE_ASPNET_CERTIFICATE='false', R_LIBS_SITE='', R_ENVIRON_USER='NUL',
               R_PROFILE_USER='NUL', LC_ALL='C', LANG='C')
    from release_windows_cache import cache_environment, prepare_cache
    env = cache_environment(scratch / 'local', env)
    prepare_cache(scratch / 'local', env)
    return env


def runtime_environment(scratch, prepared):
    """Expose OS commands and runtime DLLs without the consumer compilers."""
    binary = scratch / 'runtime-bin'
    binary.mkdir()
    system = Path(prepared['SYSTEMROOT'])
    env = {key: value for key, value in prepared.items() if key not in {
        'INCLUDE', 'LIB', 'LIBPATH', 'VCINSTALLDIR', 'VCTOOLSINSTALLDIR',
        'WINDOWSSDKDIR', 'WINDOWSSDKVERSION', 'UCRTVERSION', 'UCRTCONTENTROOT',
        'SDKROOT', 'SWIFT_DEVELOPER_DIR', 'CC', 'CXX'}}
    # Launch each runtime by its resolved absolute path. Its original directory
    # may also contain javac, swiftc or another compiler, so it never enters PATH.
    # Swift and cgo consumers need their ordinary runtime DLLs for lookup.
    folders = set()
    executable = shutil.which('gcc', path=prepared['PATH'])
    if executable: folders.add(Path(executable).parent)
    # Swift's compiler bin has different host dispatch/Blocks DLLs. Only its
    # runtime component contains swiftCore and belongs in consumer lookup.
    # https://github.com/swiftlang/swift/blob/swift-6.3.2-RELEASE/docs/WindowsToolchain.md
    for entry in prepared['PATH'].split(os.pathsep):
        folder = Path(entry)
        if entry and (folder / 'swiftCore.dll').is_file(): folders.add(folder)
    for folder in sorted(folders):
        for library in folder.glob('*.dll'):
            target = binary / library.name
            if target.exists() and target.read_bytes() != library.read_bytes():
                raise RuntimeError('Conflicting Windows runtime DLL: ' + library.name)
            if not target.exists(): shutil.copy2(library, target)
    env['PATH'] = os.pathsep.join(map(str, [binary, system / 'System32', system]))
    for name in ('cc', 'c++', 'gcc', 'clang', 'clang-cl', 'cl', 'cargo', 'rustc',
                 'go', 'swift', 'swiftc', 'zig', 'javac', 'cmake', 'make'):
        if shutil.which(name, path=env['PATH']):
            raise RuntimeError('Compiler entered the Windows runtime check: ' + name)
    return env


def go_load_failure(execute, binary):
    execute(binary, '-test.run=^TestNativeLoadFailure$', '-test.count=1', '-test.v',
            '-test.timeout=10m', stream_output=True)


def go_temporary_cache(execute, binary):
    execute(binary, '-test.run=^TestNativeTemporaryCache$', '-test.count=1', '-test.v',
            '-test.timeout=10m', stream_output=True)


def lookup_check(execute, checks, engine, *command, embedded=False, extra=None):
    execute(sys.executable, checks / 'native-lookup-check', *(['--embedded'] if embedded else []),
            engine, '--', *command, extra=extra)


def sdk_scratch():
    return long_path_scratch('thinkthen-windows-sdks-', 'Windows SDK')


def full(folder, build, *, caller_owned=False):
    if os.name != 'nt':
        raise RuntimeError('All installed Windows SDK checks require a native Windows runner')
    folder = Path(folder).resolve()
    if not caller_owned:
        from windows_caller_process import launch
        launch([sys.executable, str(Path(__file__).resolve()), '--caller-owned', folder],
               cwd=folder, env=dict(os.environ))
        return
    for stream in (sys.stdout, sys.stderr):
        stream.reconfigure(errors='backslashreplace')
    from release_windows_cache import check_cache
    languages = ('Python', 'Python frames', 'TypeScript', 'Go', 'java', 'kotlin', 'C#', 'Ruby', 'R',
                 'C', 'C++', 'Swift', 'Zig', 'sqlite', 'duckdb')
    failures = {}

    @contextmanager
    def check(name):
        try:
            yield
        except Exception as error:  # noqa: BLE001 - report every SDK before failing the step
            failures[name] = str(error)
            print('candidate Windows ' + name + ': FAIL: ' + str(error), flush=True)
        else:
            print('candidate Windows ' + name + ': installed shared stub cases pass', flush=True)

    with sdk_scratch() as scratch:
        try:
            packages = {name: folder / path for name, path in build['packages'].items()}
            cases = json.loads((folder / 'checks/stub.json').read_text(encoding='utf-8'))
            env = environment(scratch)
            runtime = runtime_environment(scratch, env)
        except Exception as error:
            for name in languages:
                print('candidate Windows ' + name + ': NOT RUN: shared setup failed: ' + str(error), flush=True)
            raise RuntimeError('Windows SDK setup failed') from error
        checks = folder / 'checks'
        fixture = checks / 'stub.json'
        def build_consumer(*args, cwd=scratch, extra=None):
            return run(*args, cwd=cwd, env=env | (extra or {}))
        def execute(*args, cwd=scratch, extra=None, stream_output=False):
            return run(*args, cwd=cwd, env=runtime | (extra or {}), resolve_path=env['PATH'],
                       stream_output=stream_output)
        def cancel(surface, *adapter, native=False, interrupt=False, extra=None):
            """Each adapter is an absolute path, because the driver starts it with no compiler on PATH."""
            flags = ['--native'] * native + ['--interrupt'] * interrupt
            print(execute(sys.executable, checks / 'cancel.py', '--surface', surface, *flags, '--',
                          *adapter, extra=extra), end='', flush=True)
        def tool(name):
            return shutil.which(name, path=env['PATH'])
        python = Path(sys.executable)
        installed_python = scratch / 'python/Scripts/python.exe'
        with check('Python'):
            execute(python, '-I', '-m', 'venv', scratch / 'python')
            execute(installed_python, '-I', '-m', 'pip', 'install', '--no-index', '--no-cache-dir', '--find-links', wheelhouse(folder), packages['python'])
            execute(installed_python, '-I', checks / 'python.py', fixture)
            cancel('Python', installed_python, '-I', checks / 'python_cancel.py')
        with check('Python frames'):
            optional = folder / 'prerequisites/python-frames-windows'
            loopback = Path(os.environ['THINKTHEN_LOOPBACK']).resolve(strict=True)
            execute(installed_python, '-I', '-m', 'pip', 'install', '--no-index',
                    '--require-hashes', '--find-links', optional / 'wheels',
                    '-r', optional / 'requirements.txt')
            execute(installed_python, checks / 'python-frames.py', '--cases',
                    checks / 'conformance', extra={'THINKTHEN_LOOPBACK': str(loopback)})
        with check('TypeScript'):
            node = scratch / 'node'
            node.mkdir()
            execute('npm', 'install', '--offline', '--ignore-scripts', '--no-audit', '--no-fund',
                    packages['typescript'], cwd=node)
            shutil.copy2(checks / 'node.mjs', node / 'consumer.mjs')
            execute('node', node / 'consumer.mjs', fixture, cwd=node)
            execute('node', '--test', checks / 'typescript/stream-concurrency.test.js', cwd=node, stream_output=True,
                    extra={'THINKTHEN_NODE_PACKAGE': str(node / 'node_modules/thinkthen/runtime.js')})
            cancel('TypeScript', tool('node'), checks / 'typescript/stress.test.js',
                   extra={'THINKTHEN_NODE_PACKAGE': str(node / 'node_modules/thinkthen/runtime.js')})
        with check('Go'):
            go = scratch / 'go'
            unpack(packages['go'], go / 'package')
            shutil.copytree(checks / 'go', go / 'consumer')
            build_consumer('go', 'mod', 'edit', '-replace=github.com/botassembly/thinkthen/sdk/go/thinkthen=' + str(go / 'package'), cwd=go / 'consumer')
            go_binary = go / 'consumer.exe'
            build_consumer('go', 'test', '-mod=mod', '-c', '-o', go_binary, cwd=go / 'consumer')
            go_arguments = ['-test.run=^TestSharedStub$', '-test.count=1', '-test.v', '-test.timeout=10m']
            go_env = {'THINKTHEN_STUB_CASES': str(fixture), 'THINKTHEN_CASES': str(checks)}
            execute(go_binary, *go_arguments, stream_output=True,
                    extra=go_env)
            check_cache([go_binary, *go_arguments],
                        scratch / 'go-cache-proof', runtime | go_env, 'library.dll')
            go_temporary_cache(execute, go_binary)
            go_load_failure(execute, go_binary)
            lookup_check(execute, checks, go / 'package/native/library', go_binary, *go_arguments,
                         embedded=True, extra=go_env)
            cancel('Go', go_binary, '-test.run=^TestSharedStress$', '-test.count=1', '-test.timeout=5m')
        for name, main in [('java', 'ConsumerCases'), ('kotlin', 'KotlinCasesKt')]:
            with check(name):
                classpath = os.pathsep.join(map(str, [checks / 'jvm/consumer.jar', folder / name / '*',
                                                       folder / name / 'dependencies/*']))
                execute('java', '--enable-native-access=ALL-UNNAMED', '-cp', classpath,
                        main, fixture, '--stub-only')
                if name == 'java':
                    execute('java', '--enable-native-access=ALL-UNNAMED', '-cp', classpath,
                            'JavaApiCases', '--stub-only')
                    check_cache([shutil.which('java', path=env['PATH']), '--enable-native-access=ALL-UNNAMED',
                                 '-cp', classpath, main, fixture, '--stub-only'],
                                scratch / 'java-cache-proof', runtime, 'thinkthen_ffi.dll')
                    lookup_check(execute, checks, scratch / 'go/package/native/library', shutil.which('java', path=env['PATH']),
                                 '--enable-native-access=ALL-UNNAMED', '-cp', classpath, main, fixture, '--stub-only', embedded=True)
                cancel(name, tool('java'), '--enable-native-access=ALL-UNNAMED', '-cp', classpath,
                       {'java': 'StressCases', 'kotlin': 'consumer.StressCasesKt'}[name])
        with check('C#'):
            consumer = scratch / 'dotnet'
            shutil.copytree(checks / 'csharp-source', consumer)
            project = consumer / 'Consumer.csproj'
            project.write_text(project.read_text(encoding='utf-8').replace('Version="0.0.0"', 'Version="' + build['version'] + '"'), encoding='utf-8')
            config = ET.Element('configuration')
            sources = ET.SubElement(config, 'packageSources')
            ET.SubElement(sources, 'clear')
            for name, value in [('candidate', str(packages['csharp'].parent)),
                                ('prepared', os.environ['THINKTHEN_WINDOWS_SINGLEFILE_FEED'])]:
                ET.SubElement(sources, 'add', key=name, value=value)
            ET.ElementTree(config).write(consumer / 'NuGet.Config', encoding='unicode')
            single = scratch / 'single-file'
            build_consumer('dotnet', 'publish', project, '--disable-build-servers',
                           '--configfile', consumer / 'NuGet.Config',
                           '-r', 'win-x64', '-p:PublishSingleFile=true', '-p:SelfContained=false',
                           '--artifacts-path', scratch / 'dotnet-artifacts', '-o', single,
                           extra={'NUGET_PACKAGES': str(scratch / 'nuget-packages')})
            assert (single / 'thinkthen_ffi.dll').is_file(), 'single-file publish lost its side-by-side native DLL'
            execute(single / 'Consumer.exe', fixture, '--stub-only')
            lookup_check(execute, checks, single / 'thinkthen_ffi.dll', single / 'Consumer.exe', fixture, '--stub-only')
            cancel('C#', single / 'Consumer.exe', '--stress-adapter')
        with check('Ruby'):
            gems = scratch / 'gems'
            ruby_env = {'GEM_HOME': str(gems), 'GEM_PATH': str(gems), 'GEM_SPEC_CACHE': str(scratch / 'gem-specs')}
            dependencies = sorted((folder / 'ruby/dependencies').glob('*.gem'))
            if not dependencies: raise RuntimeError('Missing bundled Windows ffi dependency')
            execute('gem', 'install', '--local', '--no-document', *dependencies, packages['ruby'], extra=ruby_env)
            signatures = execute('ruby', '-e', 'puts Gem::Specification.find_by_name("thinkthen").full_gem_path',
                                 extra=ruby_env).strip()
            build_consumer('ruby', '-S', 'rbs', '-I', Path(signatures) / 'sig', 'validate')
            execute('ruby', checks / 'ruby.rb', fixture, extra=ruby_env)
            lookup_check(execute, checks, Path(signatures) / 'lib/thinkthen/thinkthen_ffi.dll', shutil.which('ruby', path=env['PATH']),
                         '-e', 'require "thinkthen"; ThinkThen::Client.open(backend: :stub) {}', extra=ruby_env)
            cancel('Ruby', tool('ruby'), checks / 'ruby-stress.rb', extra=ruby_env)
        with check('R'):
            r_env = r_cases(execute, folder, scratch, packages['r'], cases)
            # Rscript only starts Rterm and dies on the interrupt that Rterm handles.
            cancel('R', tool('Rterm'), '--vanilla', '--no-echo', '--file=' + str(checks / 'r-stress.R'),
                   interrupt=True, extra=r_env)
        c_and_cpp(build_consumer, execute, packages, checks, fixture, cases, scratch, runtime, check, cancel)
        swift_and_zig(build_consumer, execute, packages, checks, fixture, scratch, check, cancel)
        for name in ('sqlite', 'duckdb'):
            with check(name):
                installed = unpack(packages[name], scratch / name)
                executable = 'sqlite3' if name == 'sqlite' else 'duckdb'
                extension = installed / ('thinkthen_sqlite.dll' if name == 'sqlite' else 'thinkthen.duckdb_extension')
                executable = shutil.which(executable, path=env['PATH'])
                if executable is None: raise RuntimeError('Missing installed database runtime: ' + name)
                execute(python, checks / 'sql.py', name, executable, extension.as_posix(), fixture)
    if failures:
        raise RuntimeError('Windows installed SDK checks failed: ' + ', '.join(failures))
    print('Windows candidate: every installed SDK passed', flush=True)


def r_cases(execute, folder, scratch, package, cases):
    library = scratch / 'r-library'
    library.mkdir()
    directory = folder / 'prerequisites/r'
    inventory = json.loads((directory / 'inventory.json').read_text(encoding='utf-8'))
    dependencies = [directory / name for name in inventory['install_order']]
    for dependency in dependencies:
        import hashlib
        record = next(record for record in inventory['packages'].values() if record['file'] == dependency.name)
        if hashlib.sha256(dependency.read_bytes()).hexdigest() != record['sha256']:
            raise RuntimeError('Windows R dependency checksum differs: ' + dependency.name)
    expression = 'a <- commandArgs(TRUE); stopifnot(as.character(getRversion()) == a[[2]]); .libPaths(c(a[[1]], .Library)); install.packages(a[-c(1,2)], lib=a[[1]], repos=NULL, type="win.binary")'
    r_env = {'R_LIBS_USER': str(library), 'R_LIBS_SITE': ''}
    execute('Rscript', '--vanilla', '-e', expression, library, inventory['r'], *dependencies, package, extra=r_env)
    # This installed-package probe covers every function for list and frame input.
    execute('Rscript', '--vanilla', folder / 'checks/r-cases.R',
            folder / 'checks/stub.json', extra=r_env)
    return r_env


def native_contract(checks):
    import importlib.util
    spec = importlib.util.spec_from_file_location('native_cases', checks / 'native_cases.py')
    native_cases = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(native_cases)
    registry = json.loads((checks / 'contract.json').read_text(encoding='utf-8'))
    return native_cases, registry


def c_and_cpp(build_consumer, execute, packages, checks, fixture, cases, scratch, env, check, cancel):
    with check('C'):
        c = unpack(packages['c'], scratch / 'c')
        c_output = scratch / 'c-consumer.exe'
        build_consumer('clang-cl', '/nologo', '/TC', '/std:c11', '/MD', '/W4', '/WX', '/UNDEBUG',
                '/D_CRT_SECURE_NO_WARNINGS', '/I' + str(c / 'include'),
                '/I' + str(checks / 'c'), checks / 'c/conformance.c',
                '/Fe:' + str(c_output), '/link', c / 'lib/thinkthen_ffi.lib')
        c_env = {'PATH': os.pathsep.join((str(c / 'bin'), env['PATH']))}
        native_cases, registry = native_contract(checks)
        for case in cases:
            rows = execute(c_output, json.dumps(case['request']), json.dumps(case['client']), extra=c_env)
            native_cases.compare(rows, case, registry)
        cancel('C', c_output, native=True, extra=c_env)
    with check('C++'):
        c = unpack(packages['c'], scratch / 'cpp-c')
        cpp = unpack(packages['cpp'], scratch / 'cpp')
        binary = scratch / 'cpp-consumer.exe'
        build_consumer('clang-cl', '/nologo', '/std:c++17', '/EHsc', '/MD', '/W4', '/WX', '/UNDEBUG',
                '/I' + str(cpp / 'include'), '/I' + str(c / 'include'), checks / 'cpp/consumer.cpp', '/Fe:' + str(binary), '/link',
                c / 'lib/thinkthen_ffi.lib')
        native_cases, registry = native_contract(checks)
        for case in cases:
            output = execute(binary, json.dumps(case['request']), json.dumps(case['client']), extra={'PATH': str(c / 'bin') + os.pathsep + env['PATH']})
            native_cases.compare(output, case, registry, 'utf8')
        cancel('C++', binary, native=True, extra={'PATH': str(c / 'bin') + os.pathsep + env['PATH']})


def swift_and_zig(build_consumer, execute, packages, checks, fixture, scratch, check, cancel):
    with check('Swift'):
        swift = scratch / 'swift'
        unpack(packages['swift'], swift / 'ThinkThen')
        shutil.copytree(checks / 'swift', swift / 'Consumer')
        build_consumer('swift', 'build', '--package-path', swift / 'Consumer', '-c', 'release', '--jobs', '2',
                '--disable-sandbox', '-Xswiftc', '-warnings-as-errors')
        binary = build_consumer('swift', 'build', '--package-path', swift / 'Consumer', '-c', 'release', '--show-bin-path').strip()
        execute(sys.executable, checks / 'swift.py', Path(binary) / 'Consumer.exe', fixture)
        lookup_check(execute, checks, Path(binary) / 'ThinkThen_ThinkThen.resources/Native/win-x64/thinkthen_ffi.dll',
                     Path(binary) / 'Consumer.exe', fixture)
        cancel('Swift', Path(binary) / 'Consumer.exe', native=True)
    with check('Zig'):
        zig = scratch / 'zig'
        unpack(packages['zig'], zig / 'package')
        shutil.copytree(checks / 'zig', zig / 'consumer')
        build_consumer('zig', 'build', '-j2', '-Dtarget=x86_64-windows-msvc', '-Doptimize=ReleaseSafe', cwd=zig / 'consumer')
        execute(sys.executable, checks / 'zig.py', zig / 'consumer/zig-out/bin/consumer.exe', fixture)
        execute(zig / 'consumer/zig-out/bin/public-api.exe', stream_output=True)
        lookup_check(execute, checks, zig / 'consumer/zig-out/bin/thinkthen_ffi.dll', zig / 'consumer/zig-out/bin/consumer.exe', 'load')
        cancel('Zig', zig / 'consumer/zig-out/bin/consumer.exe', native=True)


if __name__ == '__main__':
    if len(sys.argv) != 3 or sys.argv[1] != '--caller-owned':
        raise SystemExit('Use the installed candidate checker')
    folder = Path(sys.argv[2]).resolve()
    full(folder, json.loads((folder / 'build.json').read_text(encoding='utf-8')), caller_owned=True)
