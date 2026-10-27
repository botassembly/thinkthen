"""Build the Windows x86-64 distribution with a prepared MSVC cross toolchain."""
import hashlib
import fcntl
import importlib.util
import inspect
from importlib.machinery import SourceFileLoader
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile
import tomllib
import zipfile

from release_windows_check import smoke
from release_wheel import python_wheel
from release_native import source_inputs, complete, stage_outputs, publish

ROOT = Path(__file__).resolve().parent.parent
TARGET = 'x86_64-pc-windows-msvc'
TOOLS = Path.home() / '.local/share/thinkthen-build-tools'


def run(*args, **kwargs):
    kwargs.setdefault('capture_output', True)
    kwargs.setdefault('text', True)
    try:
        return subprocess.run(list(map(str, args)), check=True, cwd=kwargs.pop('cwd', ROOT), **kwargs)
    except subprocess.CalledProcessError as error:
        output = (error.stdout or '') + (error.stderr or '')
        print('\n'.join(output.splitlines()[-30:]), file=sys.stderr)
        raise


def copy(source, destination):
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(source, destination)


def zip_package(source, destination):
    destination.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(destination, 'w', zipfile.ZIP_DEFLATED) as archive:
        for path in sorted(source.rglob('*')):
            if path.is_file(): archive.write(path, str(path.relative_to(source)))


def source_package(stage, out, name, version, native, import_library):
    package = stage / name
    source = ROOT / 'sdk' / name
    if name == 'cpp':
        shutil.copytree(source / 'include', package / 'include')
        shutil.copytree(TOOLS / 'nlohmann-json/3.11.3/include/nlohmann', package / 'include/nlohmann')
        copy(source / 'README.md', package / 'README.md')
        copy(source / 'JSON-LICENSE', package / 'JSON-LICENSE')
    else:
        shutil.copytree(source, package, ignore=shutil.ignore_patterns('tests', 'examples', 'target', '.build'))
    if name == 'go':
        copy(native, package / 'native/library')
        copy(ROOT / 'crates/thinkthen-ffi/include/thinkthen.h', package / 'thinkthen.h')
    elif name == 'swift':
        (package / 'generate-codes.py').unlink()
        subprocess.run([sys.executable, str(ROOT / 'sdk/swift/generate-codes.py'),
                        str(ROOT / 'schemas/contract.json'),
                        str(package / 'Sources/ThinkThen/ErrorCode.swift')], check=True)
        copy(native, package / 'Sources/ThinkThen/Native/win-x64/thinkthen_ffi.dll')
        copy(ROOT / 'crates/thinkthen-ffi/include/thinkthen.h', package / 'Sources/CThinkThen/include/thinkthen.h')
    elif name != 'cpp':
        copy(native, package / 'native/win-x64/thinkthen_ffi.dll')
        copy(import_library, package / 'native/win-x64/thinkthen_ffi.lib')
        copy(ROOT / 'crates/thinkthen-ffi/include/thinkthen.h', package / 'include/thinkthen.h')
    if name == 'swift': (package / 'VERSION').write_text(version + '\n')
    copy(ROOT / 'LICENSE', package / 'LICENSE')
    if name == 'cpp': cmake_config(package, 'Cpp', 'native/win-x64/thinkthen_ffi.dll', 'native/win-x64/thinkthen_ffi.lib', version)
    result = out / f'{name}/thinkthen-{name}-{version}-windows-x86_64.zip'
    zip_package(package, result)
    return result


def build_native():
    """Reuse one target build when only distribution scripts or docs change."""
    inputs = source_inputs(ROOT, run)
    for name, function in [('producer', build_native), ('source_selection', source_inputs)]:
        inputs[name] = hashlib.sha256(inspect.getsource(function).encode()).hexdigest()
    channel = tomllib.loads((ROOT / 'rust-toolchain.toml').read_text())['toolchain']['channel']
    compiler = run('rustup', 'which', '--toolchain', channel, 'rustc').stdout.strip()
    inputs['compiler'] = run(compiler, '-vV').stdout
    inputs['build_settings'] = Path(run('scripts/native-prepare', '--path').stdout.strip()).name
    inputs['linker'] = run('clang-cl', '--version').stdout
    inputs['cross'] = subprocess.check_output(['cargo', 'xwin', '--version'], text=True)
    pins = json.loads((TOOLS / 'windows-msvc/versions.json').read_text())
    inputs['windows_sdk'] = pins
    inputs['runtime'] = 'static-crt'
    inputs['flags'] = {name: value for name, value in os.environ.items()
                       if name.startswith(('RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'CARGO_PROFILE_', 'XWIN_'))}
    digest = hashlib.sha256(json.dumps(inputs, sort_keys=True).encode()).hexdigest()
    cache = TOOLS / 'windows-native' / digest
    cache.mkdir(parents=True, exist_ok=True)
    artifacts = ['thinkthen.exe', 'thinkthen_ffi.dll', 'thinkthen_ffi.dll.lib',
                 'thinkthen_ffi.lib', '_native.dll', 'thinkthen_node.dll']
    with (cache / 'build.lock').open('w') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX)
        payload = cache / 'rust-artifacts'
        if not complete(payload, inputs, artifacts):
            env = dict(os.environ, RUSTUP_TOOLCHAIN=channel, RUSTC=compiler,
                       CARGO_NET_OFFLINE='true', CARGO_BUILD_JOBS='4', RUSTFLAGS='-C target-feature=+crt-static',
                       XWIN_CACHE_DIR=str(TOOLS / 'windows-msvc'), XWIN_ARCH='x86_64',
                       XWIN_VERSION='17', XWIN_SDK_VERSION=pins['sdk'], XWIN_CRT_VERSION=pins['crt'])
            env.pop('CARGO_ENCODED_RUSTFLAGS', None)
            result = run('cargo', 'xwin', 'build', '--offline', '--locked', '--release', '--target', TARGET,
                '--message-format=json-render-diagnostics',
                '-p', 'thinkthen', '-p', 'thinkthen-ffi', '-p', 'thinkthen-python',
                '-p', 'thinkthen-node', env=env)
            (cache / 'artifacts.json').write_text(result.stdout)
            target = Path(os.environ.get('CARGO_TARGET_DIR', ROOT / 'target')).resolve() / TARGET / 'release'
            stage_outputs(target, payload, inputs, artifacts)
        for name in artifacts:
            publish(payload / name, cache / name)
    return cache


def candidate(destination):
    if not os.environ.get('PM_JOB'):
        raise SystemExit('Run Windows candidate builds through pm job')
    if os.name == 'nt': raise SystemExit('Windows candidate cross builds run on Linux')
    os.environ['PATH'] = os.pathsep.join([str(Path.home() / '.local/bin'), str(TOOLS / 'dotnet10'), os.environ['PATH']])
    version = tomllib.loads((ROOT / 'Cargo.toml').read_text())['workspace']['package']['version']
    out = destination / version / 'windows-x86_64'
    out.mkdir(parents=True)
    (out / 'build.json').write_text(json.dumps({'version': version, 'platform': 'windows-x86_64'}))
    from release_sites import crates
    crates(out)
    windows_requirements(out)
    from release_windows_r import verified
    r = verified()
    native = build_native()
    from release_notices import compiled_names
    with tempfile.NamedTemporaryFile(mode='w+', suffix='.json') as messages:
        messages.write((native / 'artifacts.json').read_text().rstrip() + '\n' + (r / 'artifacts.json').read_text())
        messages.flush()
        names = compiled_names('windows-x86_64', messages.name, ROOT)
    (out / 'compiled-crates.json').write_text(json.dumps({'target': 'windows-x86_64', 'crates': names}, indent=2) + '\n')
    # Protect adapter writes and every package copy from another recipe writer.
    with (native / 'distribution.lock').open('w') as lock, tempfile.TemporaryDirectory(prefix='thinkthen-windows-pack-') as temporary:
        fcntl.flock(lock, fcntl.LOCK_EX)
        stage = Path(temporary)
        copy(r / f'thinkthen_{version}.zip', stage / f'thinkthen_{version}.zip')
        command = stage / 'command'
        copy(native / 'thinkthen.exe', command / 'thinkthen.exe')
        manual = run('cargo', 'run', '--release', '--offline', '--locked', '-p', 'thinkthen', '--bin', 'thinkthen', '--', 'man').stdout
        (command / 'thinkthen.1').write_text(manual)
        copy(ROOT / 'README.md', command / 'README.md')
        command_path = out / f'thinkthen-{version}-windows-x86_64.zip'
        # The command manual comes from the same argument grammar.
        with zipfile.ZipFile(command_path, 'w', zipfile.ZIP_DEFLATED) as archive:
            for name in ('thinkthen.exe', 'thinkthen.1', 'README.md'): archive.write(command / name, name)
        from release_windows_native import build_databases
        build_databases(native)
        c = stage / 'c'
        c.mkdir()
        copy(native / 'thinkthen_ffi.dll', c / 'bin/thinkthen_ffi.dll')
        copy(native / 'thinkthen_ffi.dll.lib', c / 'lib/thinkthen_ffi.lib')
        copy(ROOT / 'crates/thinkthen-ffi/include/thinkthen.h', c / 'include/thinkthen.h')
        copy(ROOT / 'sdk/c/README.md', c / 'README.md')
        copy(ROOT / 'LICENSE', c / 'LICENSE')
        for leftover in [*list((c / 'bin').glob('*.lib')), *list((c / 'bin').glob('*.exp'))]: leftover.unlink()
        cmake_config(c, 'C', 'bin/thinkthen_ffi.dll', 'lib/thinkthen_ffi.lib', version)
        packages = {'command': command_path, 'c': out / f'c/thinkthen-c-{version}-windows-x86_64.zip'}
        assert not any('jansson' in p.name.lower() for directory in ('bin', 'lib', 'include')
                       for p in (c / directory).rglob('*') if p.is_file())
        zip_package(c, packages['c'])
        packages['python'] = python_wheel(stage, out, version, native / '_native.dll')
        for name in ('go', 'cpp', 'swift', 'zig'):
            packages[name] = source_package(stage, out, name, version,
                                            native / 'thinkthen_ffi.dll', native / 'thinkthen_ffi.dll.lib')
        package_more(stage, out, version, native, packages, r)
    copy(ROOT / 'construction/stub.json', out / 'checks/stub.json')
    copy(ROOT / 'schemas/contract.json', out / 'checks/contract.json')
    copy(ROOT / 'scripts/release-check', out / 'check')
    copy(ROOT / 'scripts/release_notice_check.py', out / 'release_notice_check.py')
    copy(ROOT / 'scripts/release_images.py', out / 'release_images.py')
    copy(ROOT / 'scripts/release_windows_check.py', out / 'release_windows_check.py')
    copy(ROOT / 'scripts/production_native.py', out / 'production_native.py')
    copy(ROOT / 'scripts/release_windows_sdks.py', out / 'release_windows_sdks.py')
    copy(ROOT / 'scripts/release_windows_cache.py', out / 'release_windows_cache.py')
    copy(ROOT / 'scripts/windows_caller_process.py', out / 'windows_caller_process.py')
    check_sources(out)
    (out / 'build.json').write_text(json.dumps({'version': version, 'platform': 'windows-x86_64',
        'target': TARGET, 'toolchain': json.loads((TOOLS / 'windows-msvc/versions.json').read_text()),
        'rust': run('rustc', '--version').stdout.strip(), 'c': run('clang-cl', '--version').stdout.splitlines()[0],
        'dotnet': run('dotnet', '--version').stdout.strip(), 'dependency_inventory': 'compiled-crates.json',
        'revision': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
        'packages': {name: str(path.relative_to(out)) for name, path in packages.items()}}, indent=2) + '\n')
    from release_notice_packages import distribute
    distribute(out, 'windows-x86_64', ROOT)
    finish_readmes(out, packages)
    from release_install import checksums
    checksums(out)
    smoke(out)
    print(out)


def package_more(stage, out, version, native, packages, r):
    run(sys.executable, ROOT / 'scripts/typescript-prepare')
    npm = stage / 'typescript'
    npm.mkdir()
    manifest = json.loads((ROOT / 'sdk/typescript/package.json').read_text())
    manifest['version'] = version
    (npm / 'package.json').write_text(json.dumps(manifest, indent=2) + '\n')
    for name in manifest['files']:
        if name == 'thinkthen.node': source = native / 'thinkthen_node.dll'
        elif name == 'LICENSE': source = ROOT / name
        else: source = ROOT / 'sdk/typescript' / name
        copy(source, npm / name)
    target = out / 'typescript'
    target.mkdir()
    run('npm', 'pack', '--offline', '--ignore-scripts', '--quiet', '--pack-destination', target, cwd=npm)
    packages['typescript'] = next(target.glob('*.tgz'))
    ruby = stage / 'ruby'
    shutil.copytree(ROOT / 'sdk/ruby', ruby, ignore=shutil.ignore_patterns('tests'))
    copy(native / 'thinkthen_ffi.dll', ruby / 'lib/thinkthen/thinkthen_ffi.dll')
    copy(ROOT / 'LICENSE', ruby / 'LICENSE')
    (ruby / 'VERSION').write_text(version + '\n')
    (ruby / 'PLATFORM').write_text('x64-mingw-ucrt\n')
    run('gem', 'build', '--strict', 'thinkthen.gemspec', cwd=ruby)
    gem = next(ruby.glob('*.gem'))
    packages['ruby'] = out / 'ruby' / gem.name
    copy(gem, packages['ruby'])
    dependency, = (TOOLS / 'windows').glob('ffi-*-x64-mingw-ucrt.gem')
    copy(dependency, out / 'ruby/dependencies' / dependency.name)
    from release_windows_r import bundle_dependencies
    bundle_dependencies(r, out / 'prerequisites/r')
    # Reuse the language tools' own JAR and NuGet writers in an isolated source.
    loader = SourceFileLoader('candidate_host', str(ROOT / 'scripts/release-candidate'))
    spec = importlib.util.spec_from_loader(loader.name, loader)
    host = importlib.util.module_from_spec(spec)
    loader.exec_module(host)
    checkout = stage / 'managed'
    run('git', 'clone', '--quiet', '--local', '--no-hardlinks', ROOT, checkout)
    host.ROOT = checkout
    host.metadata(version)
    checks = out / 'checks'
    checks.mkdir(exist_ok=True)
    host.jvm(out, checks, version, 'windows-amd64', 'thinkthen_ffi.dll', native / 'thinkthen_ffi.dll')
    host.csharp(out, checks, 'win-x64', native / 'thinkthen_ffi.dll')
    for name in ('java', 'kotlin'): packages[name] = next((out / name).glob('thinkthen-*.jar'))
    packages['csharp'] = next((out / 'csharp').glob('*.nupkg'))
    from release_windows_native import databases_and_r, c_consumer
    databases_and_r(stage, out, version, native, packages)
    c_consumer(out, native)



def windows_requirements(out):
    config = json.loads((ROOT / 'scripts/runtime-floors.json').read_text())
    runtime = dict(config['runtimes'])
    runtime.update(command='Windows x86-64', r='R 4.6; bundled matching R dependency binaries',
                   c='C11 compiler and Windows x64 linker to build; operating-system runtime thereafter',
                   cpp='C++17 compiler; bundled nlohmann_json headers; standard C++ runtime thereafter',
                   swift='Swift >=6.3 Windows toolchain',
                   sqlite='SQLite >=3.41 with extension loading enabled',
                   duckdb='Official DuckDB 1.4.3 Windows x86-64')
    (out / 'requirements.json').write_text(json.dumps({name: {'runtime': value, 'platform': 'windows-x86_64'}
        for name, value in runtime.items()}, indent=2) + '\n')


def finish_readmes(out, packages):
    from release_readme import readme, update_zip, update_gem, tar_bytes
    from runtime_floors import FLOORS
    for name, artifact in packages.items():
        document = readme(name, out, artifact)
        if artifact.suffix == '.gem':
            update_gem(artifact, document)
        elif artifact.suffix == '.tgz':
            artifact.write_bytes(tar_bytes(artifact.read_bytes(), {'package/README.md': document}))
        else:
            member = ('META-INF/README.md' if name in ('java', 'kotlin') else
                      'thinkthen/README.md' if name == 'r' else 'README.md')
            update_zip(artifact, member, document)
        if name != 'command': (artifact.parent / 'README.md').write_bytes(document)
    commands = [
        '# Install ThinkThen on Windows x86-64', '',
        FLOORS['windows']['support'], '',
        'This candidate is unpublished. Use the bundled package files. Put each ordinary runtime from requirements.json on PATH before installing its package. PostgreSQL is Linux only.', '',
        'Extract the command ZIP and run `thinkthen.exe decide --backend stub "Ready?"` with text on standard input. The stub needs no key or network.', '',
        'Install the Python wheel with `py -m pip install --no-index python\\thinkthen-*.whl` after selecting the concrete bundled filename. Install the npm archive with `npm install --offline --ignore-scripts .\\typescript\\thinkthen-*.tgz` after selecting its concrete filename.', '',
        'Install the R binary with `install.packages("r/thinkthen_VERSION.zip", repos=NULL, type="win.binary")`, replacing VERSION with the bundled version. Install the bundled prerequisite ZIPs in prerequisites/r/install-order.txt order first. The optional tibble, dplyr and data.table packages use their ordinary R installers.', '',
        'Install the Ruby gems with `gem install --local --no-document` using the bundled ffi dependency and ThinkThen gem. Use the NuGet file as a local source for dotnet. Java and Kotlin include their runtime JAR dependencies beside the package.', '',
        'The C package supplies include/thinkthen.h, lib/thinkthen_ffi.lib and bin/thinkthen_ffi.dll. Link the import library and keep the DLL beside the consuming executable. C++ additionally needs the supplied nlohmann_json headers. Go, Zig and Swift packages include the Windows DLL for their ordinary build tools.', '',
        'Extract each database ZIP before loading its extension by absolute path. SQLite uses thinkthen_sqlite.dll. DuckDB uses thinkthen.duckdb_extension and its official unsigned-extension setting.', '',
        'The local Wine check verifies the installed command version and the shared decide stub case. The C ABI runs all ten shared stub cases. The release workflow runs every package on a Windows runner before publication.', '',
    ]
    (out / 'INSTALL.md').write_text('\n'.join(commands))


def cmake_config(package, language, dll, library, version):
    folder = package / f'lib/cmake/ThinkThen{language}'
    folder.mkdir(parents=True, exist_ok=True)
    target = 'C'
    lines = [
        'get_filename_component(_thinkthen_prefix "${CMAKE_CURRENT_LIST_DIR}/../../.." ABSOLUTE)',
        f'if(NOT TARGET ThinkThen::{target})',
        f'  add_library(ThinkThen::{target} SHARED IMPORTED)',
        f'  set_target_properties(ThinkThen::{target} PROPERTIES',
        f'    IMPORTED_LOCATION "${{_thinkthen_prefix}}/{dll}"',
        f'    IMPORTED_IMPLIB "${{_thinkthen_prefix}}/{library}"',
        '    INTERFACE_INCLUDE_DIRECTORIES "${_thinkthen_prefix}/include")',
        'endif()',
    ]
    if language != 'C':
        lines = ['include(CMakeFindDependencyMacro)', f'find_dependency(ThinkThenC {version} CONFIG REQUIRED)',
                 'get_filename_component(_thinkthen_prefix "${CMAKE_CURRENT_LIST_DIR}/../../.." ABSOLUTE)']
        lines += ['add_library(ThinkThen::Cpp INTERFACE IMPORTED)',
                  'set_target_properties(ThinkThen::Cpp PROPERTIES',
                  '  INTERFACE_INCLUDE_DIRECTORIES "${_thinkthen_prefix}/include"',
                  '  INTERFACE_LINK_LIBRARIES ThinkThen::C INTERFACE_COMPILE_FEATURES cxx_std_17)']
    (folder / f'ThinkThen{language}Config.cmake').write_text('\n'.join(lines) + '\n')
    with tempfile.NamedTemporaryFile(mode='w', suffix='.cmake') as recipe:
        recipe.write('include(CMakePackageConfigHelpers)\n'
                     'write_basic_package_version_file("${OUTPUT}" VERSION "${VERSION}" '
                     'COMPATIBILITY SameMinorVersion ARCH_INDEPENDENT)\n')
        recipe.flush()
        run('cmake', f'-DVERSION={version}',
            f'-DOUTPUT={folder / f"ThinkThen{language}ConfigVersion.cmake"}', '-P', recipe.name)


def check_sources(out):
    """Keep portable installed consumers with the candidate, away from the checkout."""
    for path in (ROOT / 'scripts/release').iterdir():
        if path.is_file(): copy(path, out / 'checks' / path.name)
    for family, patterns in {'go': ['*.go', 'go.mod'], 'c': ['conformance.c', 'rows.h'],
                              'cpp': ['consumer.cpp', '*.hpp'], 'swift': ['*.swift'], 'zig': ['*.zig', '*.zon']}.items():
        for pattern in patterns:
            for path in (ROOT / 'sdk' / family / 'tests').glob(pattern):
                copy(path, out / 'checks' / family / path.name)
    run(sys.executable, ROOT / 'sdk/swift/tests/readme.py', ROOT / 'sdk/swift/README.md',
        out / 'checks/swift/Readme')
    run(sys.executable, ROOT / 'scripts/surface-example', 'prepare', 'swift',
        out / 'checks/swift/Guide')
    copy(ROOT / 'sdk/r/tests/run.R', out / 'checks/run.R')
    copy(ROOT / 'sdk/r/tests/support.R', out / 'checks/support.R')
    copy(ROOT / 'sdk/r/tests/release-cases.R', out / 'checks/r-cases.R')
    for pattern in ('*.cs', 'Consumer.csproj'):
        for path in (ROOT / 'sdk/csharp/tests').glob(pattern):
            copy(path, out / 'checks/csharp-source' / path.name)
