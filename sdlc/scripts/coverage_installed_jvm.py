"""Prepare installed JVM routine consumers of the instrumented C package."""
import json
import os
from pathlib import Path
import shutil
from tools import commands
import sys

COLLECTIONS = tuple(
    "installed-" + language + suffix
    for language in ("java", "kotlin", "scala") for suffix in ("", "-usage")
)


def prepare(context):
    root, run = context['root'], context['run']
    command, missing = context['command'], context['missing_tool']
    env = dict(context['env'])
    for tool in commands(['jvm']):
        if not shutil.which(tool, path=env.get('PATH')):
            raise missing(f'installed JVM collection needs {tool}')
    jdk = os.environ.get('THINKTHEN_SESSION_JDK_HOME')
    if jdk:
        env['THINKTHEN_JDK_HOME'] = jdk
    for name in ('THINKTHEN_JDK_HOME', 'THINKTHEN_KOTLIN_HOME', 'THINKTHEN_SCALA_HOME'):
        if name in os.environ and name not in env:
            env[name] = os.environ[name]
    owned = run / 'installed-jvm'
    owned.mkdir()
    resolve = (
        'import json,sys; sys.path.insert(0,sys.argv[1]); '
        'from toolchains import JDK,KOTLIN,SCALA,stable; stable(); '
        'print(json.dumps(dict(THINKTHEN_JDK_HOME=str(JDK),'
        'THINKTHEN_KOTLIN_HOME=str(KOTLIN),THINKTHEN_SCALA_HOME=str(SCALA))))'
    )
    try:
        homes = command([sys.executable, '-c', resolve, str(root / 'libraries/jvm/tests')],
                        root, env, owned / 'toolchains.log', capture=True)
    except RuntimeError as error:
        raise missing(f'installed JVM toolchains unavailable; see {owned / "toolchains.log"}') from error
    env.update(json.loads(homes))
    env.update(THINKTHEN_JVM_NATIVE=str(context['c_shared']),
               THINKTHEN_JVM_OUT=str(owned / 'package'))
    command(['sh', str(root / 'libraries/jvm/build.sh')], root, env, owned / 'build.log', timeout=300)
    jars = owned / 'package/jars'
    collections = []
    for name in COLLECTIONS:
        language = name.removeprefix('installed-').removesuffix('-usage')
        module = 'usage_installed' if name.endswith('-usage') else 'session_installed'
        def runner(folder, *, language=language, module=module):
            # Runtime children set the profile explicitly after clearing their environment.
            source = (
                'import sys; from pathlib import Path; sys.path.insert(0,sys.argv[1]); '
                f'from {module} import run; '
                'run(Path(sys.argv[2]),Path(sys.argv[3]),language=sys.argv[4],profiles=Path(sys.argv[5]))'
            )
            command([sys.executable, '-c', source, str(root / 'libraries/jvm/tests'),
                     str(jars), str(folder / 'consumer'), language, str(run / 'active')],
                    root, env, folder / 'runner.log', timeout=180)
        collections.append({'name': name, 'objects': [context['c_shared']], 'runner': runner})
    return collections
