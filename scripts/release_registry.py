"""Stage registry packages from the release workflow's checked platform archives."""
import json
import os
import html
from csharp_dependencies import prepared_feed
import hashlib
from pathlib import Path
import shutil
import tarfile
import tempfile
import zipfile

from release_workflow import ROOT, TARGETS, run, unpack, version
from release_notices import bundle, combine, NOTICE
from release_notice_packages import apply
from release_notice_check import package_notices

NODE_TARGETS = {
    'linux-x86_64': 'linux-x64', 'linux-arm64': 'linux-arm64',
    'macos-arm64': 'darwin-arm64', 'windows-x86_64': 'win32-x64',
}
NATIVE_PROPERTIES = {
    'linux-x86_64': 'LinuxNative', 'linux-arm64': 'LinuxArm64Native',
    'macos-arm64': 'MacNative', 'windows-x86_64': 'WindowsNative',
}


def one(folder, pattern):
    paths = list(folder.glob(pattern))
    if len(paths) != 1:
        raise ValueError(f'expected one {pattern} in {folder}')
    return paths[0]


def copy_unique(source, destination):
    destination.parent.mkdir(parents=True, exist_ok=True)
    if destination.exists() and destination.read_bytes() != source.read_bytes():
        raise ValueError(f'conflicting registry artifact: {destination.name}')
    shutil.copy2(source, destination)


def node(packages, destination, scratch):
    """Keep one public npm name with all supported native bindings."""
    base = scratch / 'node'
    base.mkdir()
    for target, package in packages.items():
        expanded = scratch / ('node-' + target)
        with tarfile.open(one(package / 'typescript', '*.tgz')) as archive:
            archive.extractall(expanded, filter='data')
        source = expanded / 'package'
        if target == next(iter(packages)):
            shutil.copytree(source, base / 'package')
            (base / 'package/thinkthen.node').unlink()
        else:
            for name in ('runtime.js', 'index.d.ts', 'package.json'):
                if (source / name).read_bytes() != (base / 'package' / name).read_bytes():
                    raise ValueError(f'npm {name} differs between platforms')
        native = base / 'package/thinkthen.node'
        native.mkdir(exist_ok=True)
        shutil.copy2(source / 'thinkthen.node', native / (NODE_TARGETS[target] + '.node'))
    shutil.rmtree(base / 'package' / NOTICE, ignore_errors=True)
    for name, data in combine({target: package_notices(one(package / 'typescript', '*.tgz')) for target, package in packages.items()}).items():
        path = base / 'package' / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
    metadata = json.loads((base / 'package/package.json').read_text())
    metadata['files'] = sorted(set(metadata['files']) | {NOTICE})
    metadata['repository'] = json.loads((ROOT / 'sdk/typescript/package.json').read_text())['repository']
    (base / 'package/package.json').write_text(json.dumps(metadata, indent=2) + '\n')
    # A directory retains the binding's existing require("./thinkthen.node") entry.
    (native / 'package.json').write_text('{"main":"index.cjs","type":"commonjs"}\n')
    (native / 'index.cjs').write_text(
        'module.exports = require("./" + process.platform + "-" + process.arch + ".node");\n')
    run('npm', 'pack', '--ignore-scripts', '--offline', '--pack-destination', destination,
        cwd=base / 'package')


def python(packages, destination, scratch):
    """Package current Python source with each qualified native binding."""
    from release_wheel import python_wheel
    for target, package in packages.items():
        wheel = one(package / 'python', '*.whl')
        stage = scratch / ('python-' + target)
        with zipfile.ZipFile(wheel) as archive:
            natives = [name for name in archive.namelist() if name.startswith('thinkthen/_native.') and name.endswith(('.so', '.pyd'))]
            if len(natives) != 1:
                raise ValueError('each wheel must contain one native Python binding')
            archive.extract(natives[0], stage)
        tag = '-'.join(wheel.stem.split('-')[-3:])
        built = python_wheel(stage, scratch / 'python-packages', version(), stage / natives[0], tag, package_notices(wheel))
        copy_unique(built, destination / built.name)


def native_libraries(packages, scratch):
    libraries = {}
    for target, package in packages.items():
        expanded = scratch / ('native-' + target)
        with zipfile.ZipFile(one(package / 'csharp', '*.nupkg')) as archive:
            names = [n for n in archive.namelist() if '/native/' in n and not n.endswith('/')]
            if len(names) != 1:
                raise ValueError('each platform NuGet must contain one native library')
            archive.extract(names[0], expanded)
        libraries[target] = expanded / names[0]
    return libraries


def nuget(libraries, destination, scratch, notices):
    config = scratch / 'NuGet.Config'
    config.write_text('<configuration><packageSources><clear /><add key="prepared" value="' + str(prepared_feed()) + '" /></packageSources></configuration>')
    environment = dict(os.environ, HOME=str(scratch / 'dotnet-home'),
                       DOTNET_CLI_HOME=str(scratch / 'dotnet-home'),
                       NUGET_PACKAGES=str(scratch / 'nuget-cache'),
                       DOTNET_CLI_TELEMETRY_OPTOUT='1', DOTNET_SKIP_FIRST_TIME_EXPERIENCE='1',
                       DOTNET_GENERATE_ASPNET_CERTIFICATE='false')
    run('dotnet', 'pack', ROOT / 'sdk/csharp/ThinkThen.csproj', '-c', 'Release',
        '--configfile', config, '--artifacts-path', scratch / 'dotnet', '-o', destination,
        '-p:Version=' + version(), '-p:DebugType=None', '-p:DebugSymbols=false',
        *[f'-p:{NATIVE_PROPERTIES[t]}={p}' for t, p in libraries.items()], env=environment)
    apply(one(destination, '*.nupkg'), combine(notices))


def jvm(libraries, destination, scratch, notices):
    # Compile the current loader once; native engines come from checked packages.
    source = scratch / 'jvm-source'
    shutil.copytree(ROOT / 'sdk/java', source / 'java', ignore=shutil.ignore_patterns('target'))
    shutil.copytree(ROOT / 'sdk/kotlin', source / 'kotlin', ignore=shutil.ignore_patterns('target'))
    aliases = {'linux-x86_64': 'linux-amd64', 'linux-arm64': 'linux-aarch64',
               'macos-arm64': 'macos-arm64', 'windows-x86_64': 'windows-amd64'}
    for target, library in libraries.items():
        path = source / 'java/library/target/classes/io/thinkthen/native' / aliases[target]
        path.mkdir(parents=True)
        shutil.copy2(library, path / library.name)
    repository = scratch / 'm2'
    shutil.copytree(Path.home() / '.local/share/thinkthen-build-tools/jvm/m2', repository)
    run('python3', 'scripts/jvm-generated', source / 'java')
    run('mvn', '-o', '-B', '-q', '-Dmaven.repo.local=' + str(repository),
        '-f', source / 'java/pom.xml', 'install', '-DskipTests')
    parent = destination / 'io/github/botassembly/thinkthen-jvm-parent' / version()
    parent.mkdir(parents=True)
    shutil.copy2(source / 'java/pom.xml', parent / f'thinkthen-jvm-parent-{version()}.pom')
    for name, folder in [('java', source / 'java/library'), ('kotlin', source / 'kotlin')]:
        package = destination / 'io/github/botassembly' / ('thinkthen-' + name) / version()
        package.mkdir(parents=True)
        shutil.copy2(folder / 'pom.xml', package / f'thinkthen-{name}-{version()}.pom')
        jar = package / f'thinkthen-{name}-{version()}.jar'
        shutil.copy2(folder / 'target' / jar.name, jar)
        apply(jar, combine(notices), 'META-INF/')
        sources = folder / ('src/main/java' if name == 'java' else 'src/main/kotlin')
        with zipfile.ZipFile(package / f'thinkthen-{name}-{version()}-sources.jar', 'w') as archive:
            for path in sorted(sources.rglob('*')):
                if path.is_file():
                    archive.write(path, path.relative_to(sources))
        readme = ROOT / ('sdk/java' if name == 'java' else 'sdk/kotlin') / 'README.md'
        with zipfile.ZipFile(package / f'thinkthen-{name}-{version()}-javadoc.jar', 'w') as archive:
            archive.writestr('index.html', '<!doctype html><meta charset="utf-8"><pre>' +
                             html.escape(readme.read_text()) + '</pre>')


def package_sites(packages, staged, scratch):
    """Create site formats from the admitted platform package directories."""
    for name in ('pypi', 'npm', 'rubygems', 'nuget', 'maven', 'github'):
        (staged / name).mkdir(parents=True)
    for package in packages.values():
        for folder, pattern, site in [('ruby', '*.gem', 'rubygems')]:
            copy_unique(one(package / folder, pattern), staged / site / one(package / folder, pattern).name)
        for family in ('c', 'cpp', 'zig', 'duckdb', 'postgres'):
            for artifact in (package / family).glob('*'):
                if not artifact.name.endswith(('.tar.gz', '.zip')):
                    continue
                copy_unique(artifact, staged / 'github' / artifact.name)
        for artifact in package.glob('thinkthen-*'):
            if not artifact.name.endswith(('.tar.gz', '.zip')):
                continue
            copy_unique(artifact, staged / 'github' / artifact.name)
    python(packages, staged / 'pypi', scratch)
    node(packages, staged / 'npm', scratch)
    libraries = native_libraries(packages, scratch)
    notices = {target: package_notices(one(package / 'csharp', '*.nupkg')) for target, package in packages.items()}
    nuget(libraries, staged / 'nuget', scratch, notices)
    jvm(libraries, staged / 'maven', scratch, notices)
    from release_source import stage as release_source
    release_source(libraries, scratch / 'tag-source', notices)
    from release_sites import sources, homebrew, validate
    sources(staged, scratch)
    (staged / 'crates').mkdir()
    for package in packages.values():
        for artifact in (package / 'crates').iterdir():
            copy_unique(artifact, staged / 'crates' / artifact.name)
    homebrew(staged)
    validate(staged)
    from release_registry_check import install, install_python
    install(staged, scratch)
    install_python(staged, scratch)


def stage(source, destination, sha):
    """Validate every platform before producing any upload directory."""
    if destination.exists():
        raise ValueError('registry destination must not exist')
    with tempfile.TemporaryDirectory(prefix='thinkthen-registry-') as temporary:
        scratch = Path(temporary)
        packages = {}
        for target in TARGETS:
            archive = one(source, f'**/thinkthen-{version()}-{target}-packages.tar.gz')
            packages[target] = unpack(archive, scratch / target, target, sha)
        staged = scratch / 'registry'
        from release_source import archive_source
        package_sites(packages, staged, scratch)
        (scratch / 'tag-source/release-source.json').write_text(json.dumps({'revision': sha, 'version': version()}) + '\n')
        archive_source(scratch / 'tag-source', staged / 'tag-source.tar.gz')
        (staged / 'source.json').write_text(json.dumps({'revision': sha, 'version': version()}) + '\n')
        shutil.copytree(staged, destination)
    print('release: staged registry packages from every checked platform')


def attach(registry, release, sha):
    """Include the upload inputs and ordinary archives in the reviewed draft."""
    info = json.loads((registry / 'source.json').read_text())
    if info != {'revision': sha, 'version': version()}:
        raise ValueError('registry source differs from the release')
    for path in (registry / 'github').iterdir():
        copy_unique(path, release / path.name)
    with tarfile.open(release / f'thinkthen-{version()}-registries.tar.gz', 'w:gz') as archive:
        for path in sorted(registry.iterdir()):
            archive.add(path, arcname=path.name)
    (release / 'SHA256SUMS').write_text(''.join(
        f'{hashlib.sha256(p.read_bytes()).hexdigest()}  {p.name}\n'
        for p in sorted(release.iterdir()) if p.name != 'SHA256SUMS'))
