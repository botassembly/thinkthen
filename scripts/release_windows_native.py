"""Link Windows consumers and database adapters using the prepared target SDK."""
import os
import hashlib
import re
from pathlib import Path
import subprocess
import shutil
import zipfile
import tempfile

ROOT = Path(__file__).resolve().parent.parent
TOOLS = Path.home() / '.local/share/thinkthen-build-tools'


def run(*args, **kwargs):
    kwargs.setdefault('capture_output', True)
    kwargs.setdefault('text', True)
    try:
        return subprocess.run(list(map(str, args)), check=True, cwd=kwargs.pop('cwd', ROOT), **kwargs)
    except subprocess.CalledProcessError as error:
        import sys
        output = (error.stdout or '') + (error.stderr or '')
        print('\n'.join(output.splitlines()[-30:]), file=sys.stderr)
        raise


def environment():
    roots = list((TOOLS / 'windows-msvc').rglob('crt/include'))
    if len(roots) != 1: raise SystemExit('Prepare one Windows MSVC SDK before linking')
    root = roots[0].parent.parent
    env = dict(os.environ)
    env['INCLUDE'] = ';'.join(str(root / part) for part in ('crt/include', 'sdk/include/ucrt', 'sdk/include/shared', 'sdk/include/um'))
    env['LIB'] = ';'.join(str(root / part) for part in ('crt/lib/x86_64', 'sdk/lib/ucrt/x86_64', 'sdk/lib/um/x86_64'))
    return env


def compile_windows(output, sources, *, includes=(), libraries=(), exports=(), cxx=False, definitions=()):
    output.parent.mkdir(parents=True, exist_ok=True)
    arguments = ['clang-cl', '--target=x86_64-pc-windows-msvc', '-fuse-ld=lld', '/nologo', '/O2', '/MT',
                 '/EHsc' if cxx else '/TC', '/W4', '/WX', '/Brepro']
    if cxx: arguments += ['/std:c++17', '/DWIN32_LEAN_AND_MEAN', '/DNOMINMAX', '/wd4100']
    else: arguments += ['/std:c11']
    if output.suffix != '.exe': arguments += ['/LD']
    arguments += [f'/D{name}' for name in definitions]
    arguments += [f'/I{path}' for path in includes] + list(sources)
    arguments += [f'/Fe:{output}', '/link', '/Brepro', *libraries]
    if output.suffix != '.exe': arguments += ['/implib:' + str(output.with_suffix('.lib'))]
    arguments += [f'/export:{name}' for name in exports]
    with tempfile.TemporaryDirectory(prefix='thinkthen-windows-link-') as temporary:
        run(*arguments, env=environment(), cwd=temporary)


def c_consumer(out, native):
    compile_windows(out / 'checks/c-consumer.exe', [ROOT / 'sdk/c/tests/conformance.c'],
                    includes=[ROOT / 'crates/thinkthen-ffi/include'],
                    libraries=[native / 'thinkthen_ffi.dll.lib'])


def databases_and_r(stage, out, version, native, packages):
    from release_windows import copy, zip_package
    # These adapters have separate native dependencies. Their target builds are
    # prepared once beside the shared Rust target, then copied into each format.
    binary = native / 'thinkthen_sqlite.dll'
    if not binary.is_file(): raise SystemExit('Build the Windows SQLite adapter before packaging')
    sqlite = stage / 'sqlite'
    copy(binary, sqlite / binary.name)
    copy(ROOT / 'extensions/sqlite/README.md', sqlite / 'README.md')
    copy(ROOT / 'LICENSE', sqlite / 'LICENSE')
    packages['sqlite'] = out / f'sqlite/thinkthen-sqlite-{version}-windows-x86_64.zip'
    zip_package(sqlite, packages['sqlite'])
    duckdb = stage / 'duckdb'
    from package_inventory import INPUTS, DUCKDB_TERMS
    for name in ['thinkthen.duckdb_extension', *DUCKDB_TERMS, *(destination for _, destination in INPUTS['duckdb'])]:
        copy(native / 'duckdb' / name, duckdb / name)
    packages['duckdb'] = out / f'duckdb/thinkthen-duckdb-{version}-windows-x86_64.zip'
    zip_package(duckdb, packages['duckdb'])
    r = stage / f'thinkthen_{version}.zip'
    if not r.is_file(): raise SystemExit('Build the Windows R binary package before packaging')
    packages['r'] = out / 'r' / r.name
    copy(r, packages['r'])



def system_libraries():
    return [name + '.lib' for name in ('advapi32', 'bcrypt', 'crypt32', 'dbghelp',
            'iphlpapi', 'kernel32', 'ncrypt', 'ntdll', 'ole32', 'rstrtmgr', 'secur32', 'shell32',
            'synchronization', 'userenv', 'ws2_32', 'legacy_stdio_definitions')]


def cmake_toolchain(destination):
    env = environment()
    includes = ' '.join('/imsvc' + value for value in env['INCLUDE'].split(';'))
    libraries = ' '.join('/libpath:' + value for value in env['LIB'].split(';'))
    destination.write_text('\n'.join([
        'set(CMAKE_SYSTEM_NAME Windows)',
        'set(CMAKE_SYSTEM_PROCESSOR AMD64)',
        'set(CMAKE_TRY_COMPILE_CONFIGURATION Release)',
        'set(CMAKE_C_COMPILER clang-cl)',
        'set(CMAKE_CXX_COMPILER clang-cl)',
        'set(CMAKE_LINKER lld-link)',
        'set(CMAKE_AR llvm-lib-21)',
        'set(CMAKE_RC_COMPILER llvm-rc-21)',
        'set(CMAKE_C_COMPILER_TARGET x86_64-pc-windows-msvc)',
        'set(CMAKE_CXX_COMPILER_TARGET x86_64-pc-windows-msvc)',
        f'set(CMAKE_C_FLAGS_INIT "{includes}")',
        f'set(CMAKE_CXX_FLAGS_INIT "{includes}")',
        f'set(CMAKE_EXE_LINKER_FLAGS_INIT "{libraries}")',
        f'set(CMAKE_SHARED_LINKER_FLAGS_INIT "{libraries}")',
        'set(CMAKE_POLICY_DEFAULT_CMP0091 NEW)',
        'set(CMAKE_MSVC_RUNTIME_LIBRARY MultiThreaded)',
    ]) + '\n')


def database_inputs():
    """Include every source copied into the cached database packages."""
    sources = [ROOT / 'crates/thinkthen-ffi/include/thinkthen.h',
               ROOT / 'crates/thinkthen-ffi/include/thinkthen_sql.h',
               *sorted((ROOT / 'extensions/sqlite/src').glob('*.[ch]')),
               *sorted((ROOT / 'databases/duckdb/src').glob('*')),
               ROOT / 'databases/duckdb/generate-types.py',
               ROOT / 'schemas/row.schema.json', ROOT / 'schemas/contract.json', ROOT / 'LICENSE']
    data = b''.join(path.read_bytes() for path in sources)
    data += Path(__file__).read_bytes()
    return hashlib.sha256(data).hexdigest()


def build_databases(native):
    """Build adapters from pinned dependency sources without network access."""
    from header_contract import check
    check(ROOT)
    digest = database_inputs()
    marker = native / 'database-inputs'
    if (marker.exists() and marker.read_text() == digest
            and (native / 'thinkthen_sqlite.dll').exists()
            and (native / 'duckdb/thinkthen.duckdb_extension').exists()):
        return
    with tempfile.TemporaryDirectory(prefix='thinkthen-windows-databases-') as temporary:
        stage = Path(temporary)
        toolchain = stage / 'windows.cmake'
        cmake_toolchain(toolchain)
        jansson = stage / 'jansson'
        run('cmake', '-S', TOOLS / 'windows/jansson-2.14.1', '-B', jansson,
            '-DCMAKE_TOOLCHAIN_FILE=' + str(toolchain), '-DCMAKE_BUILD_TYPE=Release',
            '-DCMAKE_POLICY_VERSION_MINIMUM=3.5', '-DCMAKE_POLICY_DEFAULT_CMP0091=NEW', '-DJANSSON_BUILD_SHARED_LIBS=OFF', '-DJANSSON_BUILD_DOCS=OFF', '-DJANSSON_WITHOUT_TESTS=ON')
        run('cmake', '--build', jansson, '--config', 'Release', '--parallel', '2')
        library = next(jansson.rglob('jansson.lib'))
        from release_windows import copy
        compile_windows(native / 'thinkthen_sqlite.dll',
            sorted((ROOT / 'extensions/sqlite/src').glob('*.c')),
            includes=[TOOLS / 'sqlite-3.41.0', TOOLS / 'windows/jansson-2.14.1/src',
                      jansson / 'include', ROOT / 'crates/thinkthen-ffi/include'],
            libraries=[native / 'thinkthen_ffi.lib', library, *system_libraries()],
            exports=['sqlite3_thinkthensqlite_init'])
        duckdb_build(stage, native, toolchain)
    marker.write_text(digest)


def duckdb_build(stage, native, toolchain):
    from release_windows import copy
    source = TOOLS / 'duckdb/1.4.3/duckdb-1.4.3'
    build = stage / 'duckdb'
    run('cmake', '-S', source, '-B', build, '-DCMAKE_TOOLCHAIN_FILE=' + str(toolchain),
        '-DCMAKE_BUILD_TYPE=Release', '-DBUILD_UNITTESTS=OFF', '-DBUILD_SHELL=OFF',
        '-DENABLE_EXTENSION_AUTOLOADING=OFF', '-DENABLE_EXTENSION_AUTOINSTALL=OFF',
        '-DDISABLE_BUILTIN_EXTENSIONS=ON', '-DOVERRIDE_GIT_DESCRIBE=v1.4.3')
    run('cmake', '--build', build, '--target', 'duckdb_static', '--parallel', '2')
    output = native / 'duckdb'
    output.mkdir(exist_ok=True)
    extension = output / 'thinkthen.duckdb_extension'
    import sys
    run(sys.executable, ROOT / 'databases/duckdb/generate-types.py', build / 'contract_types.hpp')
    compile_windows(extension, [ROOT / 'databases/duckdb/src/extension.cpp'],
        includes=[build, source / 'src/include', source / 'third_party/yyjson/include',
                  ROOT / 'crates/thinkthen-ffi/include'], cxx=True, definitions=['DUCKDB_STATIC_BUILD'],
        libraries=[native / 'thinkthen_ffi.lib', *sorted(build.rglob('*.lib')), *system_libraries()],
        exports=['thinkthen_duckdb_cpp_init'])
    metadata = stage / 'platform'
    metadata.write_text('windows_amd64')
    (stage / 'null').write_bytes(b'\0')
    run('cmake', '-DEXTENSION=' + str(extension), '-DNULL_FILE=' + str(stage / 'null'),
        '-DPLATFORM_FILE=' + str(metadata), '-DVERSION_FIELD=v1.4.3', '-DABI_TYPE=CPP',
        '-P', source / 'scripts/append_metadata.cmake')
    from package_inventory import stage as stage_inputs, duckdb_terms
    stage_inputs('duckdb', output, ROOT)
    duckdb_terms(source, output)
