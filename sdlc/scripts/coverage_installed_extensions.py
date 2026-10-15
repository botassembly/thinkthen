"""Measure routine callers of packaged native extensions and Rust consumers."""
import os
from pathlib import Path
import shutil
import subprocess
import tarfile

COLLECTIONS = ('installed-javascript', 'installed-typescript', 'installed-ruby',
               'installed-r', 'installed-rust', 'installed-polars')


def prepare(context):
    root, run = context['root'], context['run']
    command, missing = context['command'], context['missing_tool']
    env = {**context['env'], 'CARGO_BUILD_JOBS': '2', 'THINKTHEN_FULL_COMPILER': '1',
           'THINKTHEN_TEST_PROFILE': 'routine'}
    tools = Path(env.get('THINKTHEN_TOOLCHAINS', str(Path(env['HOME']) / '.cache/thinkthen-toolchains')))
    node = tools / 'node-v22.22.3-linux-x64/bin'
    ruby = tools / 'ruby' / selected_ruby_version(root, env)
    if not (node / 'node').is_file() or not (ruby / 'thinkthen-toolchain.stamp').is_file():
        raise missing('installed extensions need the cached pinned Node and stamped Ruby toolchains')
    env['PATH'] = str(node) + os.pathsep + env['PATH']
    for tool in ('node', 'npm', 'R', 'Rscript', 'cargo', 'rustc', 'cc', 'nm'):
        if not shutil.which(tool, path=env['PATH']):
            raise missing('installed extensions need ' + tool)
    clang = sorted(Path('/usr/lib').glob('llvm-*/lib/libclang.so.1'))
    if not clang:
        raise missing('installed Ruby needs cached host libclang')
    stage = run / 'stage-extensions'
    tree = stage / 'source'
    tree.mkdir(parents=True)
    archive = stage / 'source.tar'
    command(['git', 'archive', 'HEAD', '--output', str(archive)], root, env, stage / 'archive.log')
    with tarfile.open(archive) as source:
        source.extractall(tree, filter='data')
    archive.unlink()
    for source in (tree / 'crates/thinkthen/src').rglob('*'):
        if source.is_file() and source.read_bytes() != (root / source.relative_to(tree)).read_bytes():
            raise RuntimeError('coverage source copy differs: ' + str(source.relative_to(tree)))
    flags = env.get('CARGO_ENCODED_RUSTFLAGS', '')
    env['CARGO_ENCODED_RUSTFLAGS'] = flags + '\x1f' + f'--remap-path-prefix={tree}=/build/source'
    node_env = {**env, 'CARGO_TARGET_DIR': str(run / 'target-node')}
    command(['sh', 'build-addon.sh'], tree / 'libraries/typescript', node_env, stage / 'node-build.log', timeout=1800)
    command(['npm', 'pack', '--offline', '--pack-destination', str(stage)],
            tree / 'libraries/typescript', node_env, stage / 'node-pack.log')
    node_artifact = next(stage.glob('thinkthen-*.tgz'))
    collections = []
    for language in ('javascript', 'typescript'):
        name = 'installed-' + language
        def runner(folder, language=language):
            command(['sh', './check.sh'], root / 'libraries/typescript',
                    {**env, 'CARGO_TARGET_DIR': str(run / 'target-tools'),
                     'THINKTHEN_ARTIFACT': str(node_artifact), 'THINKTHEN_PARITY_CONSUMER': language,
                     'THINKTHEN_COVERAGE_OBJECT': str(folder / 'installed.node')}, folder / 'check.log', timeout=1800)
        collections.append(dict(name=name, objects=[run / name / 'installed.node'], runner=runner))
    ruby_env = {**env, 'RUBY': str(ruby / 'bin/ruby'), 'PATH': str(ruby / 'bin') + os.pathsep + env['PATH'],
                'LIBCLANG_PATH': str(clang[-1].parent), 'LD_LIBRARY_PATH': str(ruby / 'lib'),
                'CARGO_TARGET_DIR': str(run / 'target-ruby')}
    command(['sh', 'build.sh'], tree / 'libraries/ruby', ruby_env, stage / 'ruby-build.log', timeout=1800)
    gems = [path for path in (tree / 'libraries/ruby').glob('thinkthen-*.gem') if path.name.count('-') > 1]
    if len(gems) != 1:
        raise RuntimeError('expected one platform Ruby gem')
    def ruby_runner(folder):
        command(['sh', './check.sh'], root / 'libraries/ruby',
                {**ruby_env, 'CARGO_TARGET_DIR': str(run / 'target-tools'), 'THINKTHEN_ARTIFACT': str(gems[0]),
                 'THINKTHEN_COVERAGE_OBJECT': str(folder / 'installed-ruby.so')}, folder / 'check.log', timeout=1800)
    collections.append(dict(name='installed-ruby', objects=[run / 'installed-ruby/installed-ruby.so'], runner=ruby_runner))
    package_env = {**env, 'CARGO_TARGET_DIR': str(run / 'target-package')}
    command(['cargo', 'package', '--locked', '--offline', '--no-verify', '-p', 'thinkthen'],
            tree, package_env, stage / 'crate-package.log', timeout=300)
    crate = next((run / 'target-package/package').glob('thinkthen-*.crate'))
    verify_packaged_core(root, crate, crate.stem + '/src/')
    command(['bash', 'libraries/r/tools/make-tarball.sh', str(stage)], tree,
            package_env, stage / 'r-package.log', timeout=300)
    r_artifact = next(stage.glob('thinkthen_*.tar.gz'))
    verify_packaged_core(root, r_artifact, 'thinkthen/src/rust/vendor/thinkthen/src/')
    sysroot = command(['rustc', '--print', 'sysroot'], root, env, stage / 'sysroot.log', capture=True).decode().strip()
    host = command(['rustc', '-vV'], root, env, stage / 'rustc.log', capture=True).decode()
    host = next(line.split(': ', 1)[1] for line in host.splitlines() if line.startswith('host: '))
    runtime = next((Path(sysroot) / 'lib/rustlib' / host / 'lib').glob('libprofiler_builtins-*.rlib'))
    r_env = {**env, 'CARGO_TARGET_DIR': str(run / 'target-r'), 'THINKTHEN_ARTIFACT': str(r_artifact),
             'THINKTHEN_COVERAGE_STAGE': str(stage / 'r'), 'THINKTHEN_COVERAGE_ROOT': '/build/source',
             'THINKTHEN_COVERAGE_LIBS': f'{context["initializer"]} -Wl,-u,__llvm_profile_runtime {runtime}'}
    command(['bash', './check.sh'], root / 'libraries/r', {**r_env, 'THINKTHEN_COVERAGE_PHASE': 'prepare'},
            stage / 'r-install.log', timeout=1800)
    def r_runner(folder):
        command(['bash', './check.sh'], root / 'libraries/r',
                {**r_env, 'CARGO_TARGET_DIR': str(run / 'target-tools'), 'THINKTHEN_COVERAGE_PHASE': 'run'},
                folder / 'check.log', timeout=1800)
    collections.append(dict(name='installed-r', objects=[stage / 'r/lib/thinkthen/libs/thinkthen.so'], runner=r_runner))
    for language, binary in (('rust', 'thinkthen-rust-consumer'), ('polars', 'thinkthen-polars-consumer')):
        target = run / ('target-' + language)
        def runner(folder, language=language, target=target):
            command(['sh', './check.sh'], root / 'libraries' / language,
                    {**env, 'CARGO_TARGET_DIR': str(target), 'THINKTHEN_ARTIFACT': str(crate),
                     'THINKTHEN_TEST_BACKEND': str(run / 'target-tools/debug/conformance-backend'),
                     'THINKTHEN_COVERAGE_ROOT': '/build/source'}, folder / 'check.log', timeout=3600)
        objects = [target / ('polars/debug' if language == 'polars' else 'debug') / binary]
        collections.append(dict(name='installed-' + language, objects=objects, runner=runner))
    return collections


def verify_packaged_core(root, artifact, prefix):
    """Canonical remaps require unchanged source bytes in each installed input."""
    compared = 0
    with tarfile.open(artifact) as archive:
        for member in archive.getmembers():
            if not member.isfile() or not member.name.startswith(prefix):
                continue
            canonical = root / 'crates/thinkthen/src' / member.name.removeprefix(prefix)
            with archive.extractfile(member) as source:
                if source.read() != canonical.read_bytes():
                    raise RuntimeError('coverage packaged core differs: ' + member.name)
            compared += 1
    if not compared:
        raise RuntimeError('coverage package contains no core source: ' + str(artifact))


def selected_ruby_version(root, env):
    """Read the same pinned selector as setup, build and installed checks."""
    return subprocess.check_output(
        ['sh', '-c', '. "$1"; printf "%s" "$RUBY_VERSION"', 'ruby-toolchain',
         str(root / 'libraries/ruby/toolchain.env')], env=env, text=True).strip()
