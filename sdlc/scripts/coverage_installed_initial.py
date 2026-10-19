"""Prepare routine installed C and Python collections for the coverage driver."""
from pathlib import Path
import shutil
from tools import commands
import tarfile


COLLECTIONS = ('installed-c', 'installed-python-shared', 'installed-python',
               'installed-pandas', 'installed-python-polars')


def prepare(context):
    root, run = context['root'], context['run']
    env, command = context['env'], context['command']
    for tool in commands(['python']):
        if not shutil.which(tool):
            raise context['missing_tool'](tool)
    runtime_env = {**env, 'CARGO_TARGET_DIR': str(run / 'target-tools')}
    stage = run / 'stage-c'
    package = stage / 'package'
    (package / 'include').mkdir(parents=True)
    (package / 'lib/pkgconfig').mkdir(parents=True)
    shutil.copy2(root / 'libraries/c/include/thinkthen.h', package / 'include')
    shared = package / 'lib' / context['c_shared'].name.replace('thinkthen_c', 'thinkthen')
    shutil.copy2(context['c_shared'], shared)
    command(['sh', str(root / 'libraries/c/localize.sh'),
             str(context['c_target'] / 'debug/libthinkthen_c.a'), str(package / 'lib/libthinkthen.a')],
            root, {**env, 'THINKTHEN_COVERAGE_INITIALIZER': str(context['initializer'])}, run / 'localize-c.log')
    (package / 'lib/pkgconfig/thinkthen.pc').write_text(
        'prefix=${pcfiledir}/../..\nlibdir=${prefix}/lib\nincludedir=${prefix}/include\n'
        'Name: thinkthen\nDescription: ThinkThen C door\nVersion: 0.2.0\n'
        'Libs: -L${libdir} -lthinkthen\nLibs.private: -lgcc_s -lutil -lrt -lpthread -lm -ldl\n'
        'Cflags: -I${includedir}\n')
    artifact = stage / 'thinkthen-c.tar.gz'
    with tarfile.open(artifact, 'w:gz') as archive:
        for folder in ('include', 'lib'):
            archive.add(package / folder, arcname=folder)

    def c_runner(collection):
        command(['sh', './check.sh'], root / 'libraries/c',
                {**runtime_env, 'THINKTHEN_ARTIFACT': str(artifact), 'THINKTHEN_TEST_PROFILE': 'routine',
                 'THINKTHEN_COVERAGE_STAGE': str(collection / 'installed')}, collection / 'check.log', timeout=1800)

    # These are the final static consumers, retained by installed-session.py.
    c_objects = [run / 'installed-c/installed' / name for name in
                 ('owner-session', 'example-session', 'session-cases')]
    collections = [{'name': 'installed-c', 'objects': c_objects, 'runner': c_runner}]
    python_stage = run / 'stage-python'
    python_env = {**env, 'CARGO_TARGET_DIR': str(run / 'target-python'),
                  'THINKTHEN_COVERAGE_STAGE': str(python_stage), 'THINKTHEN_TEST_PROFILE': 'routine',
                  'THINKTHEN_FULL_COMPILER': '1'}
    command(['sh', './check.sh'], root / 'libraries/python',
            {**python_env, 'THINKTHEN_COVERAGE_PHASE': 'prepare'}, run / 'prepare-python.log', timeout=1800)
    extensions = list((python_stage / 'venv').glob('lib/python*/site-packages/thinkthen/_thinkthen*.so'))
    if len(extensions) != 1:
        raise RuntimeError('expected one native extension from the installed Python wheel')
    objects = [extensions[0]]
    for consumer in ('python-shared', 'python', 'pandas', 'python-polars'):
        def runner(collection, consumer=consumer):
            selected = {**python_env, 'CARGO_TARGET_DIR': str(run / 'target-tools'), 'THINKTHEN_COVERAGE_PHASE': 'shared' if consumer == 'python-shared' else 'consumer'}
            if consumer != 'python-shared':
                selected['THINKTHEN_PARITY_CONSUMER'] = consumer
            command(['sh', './check.sh'], root / 'libraries/python', selected, collection / 'check.log', timeout=1800)
        collections.append({'name': 'installed-' + consumer, 'objects': objects, 'runner': runner})
    return collections
