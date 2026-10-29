"""Install universal packages using the ordinary package managers and shared cases."""
from csharp_dependencies import prepared_feed
from jvm_repository import consumer as stage_jvm_consumer
import os
import json
import hashlib
from xml.sax.saxutils import escape
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
import shutil
import sys
import threading

from release_workflow import ROOT, run as command, version


def run(*args, **options):
    # CreateProcess does not search PATHEXT for Maven/npm batch launchers.
    if os.name == 'nt':
        executable = shutil.which(str(args[0]))
        if executable is None:
            raise RuntimeError('Missing package consumer tool: ' + str(args[0]))
        args = (executable, *args[1:])
    options.setdefault('timeout', 600)
    return command(*args, **options)


def java_arguments(path, *arguments):
    """Keep complete JVM arguments outside the operating system command line."""
    path.write_text('\n'.join(json.dumps(str(argument), ensure_ascii=False) for argument in arguments) + '\n',
                    encoding='utf-8')
    return '@' + str(path)


def node_install(packages, node, scratch, isolated):
    """Serve the staged tarball through npm's normal by-name registry contract."""
    archive, = (packages / 'npm').glob('*.tgz')
    payload = archive.read_bytes()

    class Registry(BaseHTTPRequestHandler):
        def log_message(self, *_):
            pass

        def do_GET(self):
            if self.path == '/thinkthen':
                artifact = {'name': 'thinkthen', 'version': version(), 'dist': {
                    'tarball': f'http://127.0.0.1:{self.server.server_port}/package.tgz',
                    'shasum': hashlib.sha1(payload).hexdigest()}}
                data = json.dumps({'name': 'thinkthen', 'versions': {version(): artifact},
                                   'dist-tags': {'latest': version()}}).encode()
            elif self.path == '/package.tgz':
                data = payload
            else:
                self.send_error(404)
                return
            self.send_response(200)
            self.send_header('Content-Length', str(len(data)))
            self.end_headers()
            self.wfile.write(data)

    service = ThreadingHTTPServer(('127.0.0.1', 0), Registry)
    thread = threading.Thread(target=service.serve_forever)
    thread.start()
    try:
        run('npm', 'install', '--offline=false', '--registry', f'http://127.0.0.1:{service.server_port}',
            '--ignore-scripts', '--no-audit', '--no-fund', '--cache', scratch / 'npm-cache',
            '--prefix', node, 'thinkthen@' + version(), env=isolated)
    finally:
        service.shutdown()
        service.server_close()
        thread.join()


def environment(scratch):
    environment = dict(os.environ, HOME=str(scratch / 'home'), DOTNET_CLI_HOME=str(scratch / 'home'),
                       XDG_CACHE_HOME=str(scratch / 'cache'), XDG_CONFIG_HOME=str(scratch / 'config'),
                       NUGET_PACKAGES=str(scratch / 'nuget-cache'), DOTNET_CLI_TELEMETRY_OPTOUT='1',
                       DOTNET_SKIP_FIRST_TIME_EXPERIENCE='1', DOTNET_GENERATE_ASPNET_CERTIFICATE='false')
    environment.update(USERPROFILE=str(scratch / 'home'), APPDATA=str(scratch / 'config'),
                       NPM_CONFIG_USERCONFIG=str(scratch / 'config/npmrc'), PYTHONNOUSERSITE='1')
    for name in ('THINKTHEN_API_KEY', 'OPENAI_API_KEY', 'THINKTHEN_BASE_URL', 'THINKTHEN_BATCH',
                 'PYTHONPATH', 'NODE_PATH', 'NODE_OPTIONS', 'CLASSPATH', 'JAVA_TOOL_OPTIONS',
                 '_JAVA_OPTIONS', 'JDK_JAVA_OPTIONS', 'MAVEN_ARGS', 'MAVEN_OPTS',
                 'DOTNET_ADDITIONAL_DEPS', 'DOTNET_SHARED_STORE'):
        environment.pop(name, None)
    if os.name == 'nt':
        from release_windows_cache import prepare_cache
        cache = scratch / 'cache'
        environment.update(LOCALAPPDATA=str(cache), LocalAppData=str(cache))
        prepare_cache(cache, environment)
    return environment


def install_node(packages, scratch):
    isolated = environment(scratch)
    node = scratch / 'node-consumer'
    node.mkdir()
    node_install(packages, node, scratch, isolated)
    shutil.copy2(ROOT / 'scripts/release/node.mjs', node / 'check.mjs')
    run('node', node / 'check.mjs', ROOT / 'construction/stub.json', env=isolated)


def install_dotnet(packages, scratch):
    isolated = environment(scratch)
    consumer = scratch / 'dotnet-consumer'
    shutil.copytree(ROOT / 'sdk/csharp/tests', consumer)
    project = consumer / 'Consumer.csproj'
    config = consumer / 'NuGet.Config'
    config.write_text(f'<configuration><packageSources><clear/><add key="local" value="{escape(str(packages / "nuget"), {chr(34): "&quot;"})}"/><add key="prepared" value="{escape(str(prepared_feed()), {chr(34): "&quot;"})}"/></packageSources></configuration>', encoding='utf-8')
    run('dotnet', 'build', project, '--configfile', config, '--artifacts-path', scratch / 'dotnet-artifacts',
        '-o', scratch / 'dotnet-bin', env=isolated)
    run('dotnet', scratch / 'dotnet-bin/Consumer.dll', ROOT / 'construction/stub.json', '--stub-only', env=isolated)


def install_jvm(packages, scratch):
    isolated = environment(scratch)
    # Maven install has compiled the current Java/Kotlin libraries. Use the public
    # consumer with a private local repository containing the packaged coordinates.
    consumer = scratch / 'java-consumer'
    stage_jvm_consumer(ROOT, consumer)
    pom = consumer / 'pom.xml'
    repository = scratch / 'm2'
    shutil.copytree(packages / 'maven', repository, dirs_exist_ok=True)
    classpath = scratch / 'jvm-classpath'
    run('mvn', '-o', '-B', '-q', '-Dmaven.repo.local=' + str(repository), '-f', pom,
        'package', 'dependency:build-classpath', '-Dmdep.outputFile=' + str(classpath), env=isolated)
    jars = [consumer / 'target/classes', classpath.read_text(encoding='utf-8').strip()]
    for main in ('ConsumerCases', 'KotlinCasesKt'):
        run('java', java_arguments(scratch / 'jvm-consumer.args', '--enable-native-access=ALL-UNNAMED',
            '-cp', os.pathsep.join(map(str, jars)), main, ROOT / 'construction/stub.json', '--stub-only'), env=isolated)
    run('java', java_arguments(scratch / 'jvm-consumer.args', '--enable-native-access=ALL-UNNAMED',
        '-cp', os.pathsep.join(map(str, jars)), 'JavaApiCases', '--stub-only'), env=isolated)



def install_python(packages, scratch):
    isolated = environment(scratch)
    home = scratch / 'python-consumer'
    run(sys.executable, '-m', 'venv', home, env=isolated)
    python = home / ('Scripts/python.exe' if os.name == 'nt' else 'bin/python')
    run(python, '-m', 'pip', 'install', '--no-index', '--no-cache-dir', '--find-links', packages / 'pypi',
        'thinkthen==' + version(), env=isolated)
    run(python, ROOT / 'scripts/release/python.py', ROOT / 'construction/stub.json', env=isolated)


def install(packages, scratch, report=None):
    """Report every aggregate package failure before refusing the release."""
    failures = []
    for name, check in [('npm', install_node), ('nuget', install_dotnet), ('maven', install_jvm)]:
        try:
            check(packages, scratch)
        except Exception as error:
            failures.append(name)
            print(f'registry {name}: FAIL: {error}', flush=True)
            if report:
                report(name, {'outcome': 'failure', 'error': str(error)})
        else:
            print(f'registry {name}: PASS', flush=True)
            if report:
                report(name, {'outcome': 'success'})
    if failures:
        raise RuntimeError('Failed registry consumers: ' + ', '.join(failures))
