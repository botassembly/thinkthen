#!/usr/bin/env python3
"""Install final candidate npm, NuGet and Maven packages on Windows and count fake calls."""
import argparse
import http.server
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import threading
import xml.etree.ElementTree as ET

REPO = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO / 'conformance/children'))
from children import child_env
from install_check_consumers import SOURCES

SPEC = importlib.util.spec_from_file_location('release_language_tools', Path(__file__).with_name('release-language-tools.py'))
TOOLS = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(TOOLS)

KOTLIN = '''import thinkthen.kotlin.KotlinEngine
import thinkthen.kotlin.Results
import thinkthen.Inputs
import kotlinx.coroutines.runBlocking
fun main() = runBlocking {
  KotlinEngine(Inputs.EngineSettings().cache(Inputs.CacheDocument(false)).maxRetries(0)).use { engine ->
    val call = engine.decide(Inputs.RequestQuestionText().text("Is it?"), Inputs.RequestInputText().text("consumer-kotlin"))
    val row = call.packets.filterIsInstance<Results.SessionPacketDecideRow>().single()
    check(row.value.value.value == true && call.terminal.facts!!.requestsSent.intValueExact() == 1)
    println("{\\\"value\\\":true,\\\"requests_sent\\\":1}")
  }
}
'''
SCALA = '''import thinkthen.scala.{ScalaEngine, Results}
import thinkthen.Inputs
import scala.concurrent.Await
import scala.concurrent.duration.*
object InstallScala {
  def main(args: Array[String]): Unit = {
    val engine = new ScalaEngine(new Inputs.EngineSettings().cache(new Inputs.CacheDocument(false)).maxRetries(0))
    try {
      val call = Await.result(engine.decide(new Inputs.RequestQuestionText().text("Is it?"), new Inputs.RequestInputText().text("consumer-scala")).result, 10.seconds)
      val row = call.packets.collect { case value: Results.SessionPacketDecideRow => value }.head
      assert(row.value.value.value == true && call.terminal.facts.get.requestsSent.toInt == 1)
      println("{\\\"value\\\":true,\\\"requests_sent\\\":1}")
    } finally engine.close()
  }
}
'''


def run(command, project, env):
    result = subprocess.run(list(map(str, command)), cwd=project, env=env,
                            capture_output=True, text=True, timeout=600)
    if result.returncode:
        raise RuntimeError(f'{command[0]} failed ({result.returncode}): {result.stdout}\n{result.stderr}')
    return result.stdout.strip()


def live_source(language):
    source = SOURCES[language]
    return source.replace("replay: root + '/recording', ", '').replace(
        'Replay = Path.Combine(root, "recording"),\n    ', '').replace(
        '.replay(root.resolve("recording").toString())', '')


def dependency(group, artifact, version, classifier=''):
    suffix = f'<classifier>{classifier}</classifier>' if classifier else ''
    return f'<dependency><groupId>{group}</groupId><artifactId>{artifact}</artifactId><version>{version}</version>{suffix}</dependency>'


def sdk(name, work):
    url, algorithm, digest = TOOLS.ASSETS[name]
    archive = work / url.rsplit('/', 1)[1]
    TOOLS.fetch(url, archive)
    TOOLS.verify_digest(archive, algorithm, digest)
    output = work / name
    TOOLS.safe_extract(archive, output)
    return next(path.parent.parent for path in output.rglob('bin/' + ('kotlinc.bat' if name == 'kotlin' else 'scalac.bat')))


def consume(npm, registry, work):
    pom = next((registry / 'maven').rglob('thinkthen-jvm-*.pom'))
    metadata = ET.parse(pom).getroot()
    version = metadata.findtext('{*}version')
    floor = metadata.findtext('{*}properties/{*}thinkthen.session.jdk')
    coroutines = metadata.findtext('{*}properties/{*}thinkthen.kotlin.coroutines.version')
    env = child_env(keep=('JAVA_HOME',), home=work / 'home',
                    NUGET_PACKAGES=str(work / 'nuget-cache'), DOTNET_CLI_HOME=str(work / 'dotnet-home'),
                    DOTNET_CLI_TELEMETRY_OPTOUT='1', DOTNET_SKIP_FIRST_TIME_EXPERIENCE='1',
                    npm_config_cache=str(work / 'npm-cache'))
    arrivals = []

    class Backend(http.server.BaseHTTPRequestHandler):
        def do_POST(self):
            body = json.loads(self.rfile.read(int(self.headers['Content-Length'])))
            arrivals.append(body)
            reply = json.dumps({'model': 'jev-1.13.0', 'answers': {
                name: {'type': 'noul', 'noul': 0.9} for name in body['questions']}}).encode()
            self.send_response(200)
            self.send_header("Connection", "close")
            self.close_connection = True
            self.send_header('Content-Length', str(len(reply)))
            self.end_headers()
            self.wfile.write(reply)

        def log_message(self, *_args):
            pass

    with http.server.ThreadingHTTPServer(('127.0.0.1', 0), Backend) as server:
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        env.update(THINKTHEN_BASE_URL=f'http://127.0.0.1:{server.server_port}/generic/v1',
                   THINKTHEN_API_KEY='tt-windows-sdk-fake')
        expected = []

        def call(language, command, project):
            sample = project / 'sample'
            sample.mkdir(exist_ok=True)
            (sample / 'question.txt').write_text('Is it?', encoding='utf-8')
            (sample / 'report.txt').write_text('consumer-' + language, encoding='utf-8')
            reply = json.loads(run([*command, sample], project, env))
            assert reply == {'value': True, 'requests_sent': 1}, (language, reply)
            expected.append({'model': 'jev-1.13.0', 'questions': {'q1': {'type': 'noul',
                'instructions': f'The text is "consumer-{language}". Is it?'}},
                'state': 'Each question quotes the text it asks about.'})
            assert arrivals == expected, (language, arrivals, expected)
            print(f'Windows installed {language}: true, exactly one fake backend request')

        try:
            node = work / 'node'; node.mkdir()
            (node / 'package.json').write_text('{"private":true}')
            run([shutil.which('npm'), 'install', '--ignore-scripts', '--no-audit', '--no-fund', npm / f'thinkthen-{version}.tgz'], node, env)
            assert json.loads((node / 'node_modules/thinkthen/package.json').read_text())['version'] == version
            (node / 'consumer.cjs').write_text(live_source('node'), encoding='utf-8')
            call('node', ['node', node / 'consumer.cjs'], node)

            net = work / 'csharp'; net.mkdir()
            run(['dotnet', 'new', 'console', '--name', 'InstallCheck', '--framework', 'net8.0', '--no-restore'], net, env)
            (net / 'NuGet.Config').write_text(f'<configuration><packageSources><clear/><add key="candidate" value="{registry / "nuget"}"/></packageSources></configuration>')
            run(['dotnet', 'add', 'package', 'Botassembly.ThinkThen', '--version', version], net, env)
            (net / 'Program.cs').write_text(live_source('csharp'), encoding='utf-8')
            assets = json.loads((net / 'obj/project.assets.json').read_text())
            assert 'Botassembly.ThinkThen/' + version in assets['libraries']
            run(['dotnet', 'build', '--no-restore', '--nologo', '--verbosity', 'quiet'], net, env)
            call('csharp', ['dotnet', net / 'bin/Debug/net8.0/InstallCheck.dll'], net)

            jvm = work / 'jvm'; jvm.mkdir()
            cache = work / 'maven-cache'
            shutil.copytree(registry / 'maven', cache)
            kotlin, scala = sdk('kotlin', work), sdk('scala', work)
            kotlin_version = re.search(r'/v([^/]+)/', TOOLS.ASSETS['kotlin'][0])[1]
            scala_version = re.search(r'/download/([^/]+)/', TOOLS.ASSETS['scala'][0])[1]
            deps = ''.join(dependency('io.github.botassembly', 'thinkthen-jvm', version, classifier) for classifier in ('', 'kotlin', 'scala'))
            deps += dependency('org.jetbrains.kotlin', 'kotlin-stdlib', kotlin_version)
            deps += dependency('org.jetbrains.kotlinx', 'kotlinx-coroutines-core-jvm', coroutines)
            deps += dependency('org.scala-lang', 'scala3-library_3', scala_version)
            (jvm / 'pom.xml').write_text(f'<project><modelVersion>4.0.0</modelVersion><groupId>consumer</groupId><artifactId>windows</artifactId><version>1</version><dependencies>{deps}</dependencies></project>')
            run([shutil.which('mvn'), f'-Dmaven.repo.local={cache}', '-q', 'dependency:copy-dependencies', '-DoutputDirectory=dependencies'], jvm, env)
            cp = os.pathsep.join(str(path) for path in sorted((jvm / 'dependencies').glob('*.jar')))
            app = jvm / 'app'; app.mkdir()
            (jvm / 'InstallCheck.java').write_text(live_source('java'), encoding='utf-8')
            (jvm / 'InstallKotlin.kt').write_text(KOTLIN, encoding='utf-8')
            (jvm / 'InstallScala.scala').write_text(SCALA, encoding='utf-8')
            run(['javac', '--release', floor, '-cp', cp, '-d', app, jvm / 'InstallCheck.java'], jvm, env)
            run([kotlin / 'bin/kotlinc.bat', '-jvm-target', floor, '-classpath', cp, '-d', app, jvm / 'InstallKotlin.kt'], jvm, env)
            run([scala / 'bin/scalac.bat', '-classpath', cp, '-d', app, jvm / 'InstallScala.scala'], jvm, env)
            for language, main in (('java', 'InstallCheck'), ('kotlin', 'InstallKotlinKt'), ('scala', 'InstallScala')):
                call(language, ['java', '--enable-native-access=ALL-UNNAMED', '-cp', str(app) + os.pathsep + cp, main], jvm)
        finally:
            server.shutdown()
            thread.join(timeout=10)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('npm', type=Path)
    parser.add_argument('registry', type=Path)
    args = parser.parse_args()
    if os.name != 'nt':
        parser.error('installed Windows SDK execution requires Windows')
    with tempfile.TemporaryDirectory(prefix='thinkthen-windows-sdk-') as temporary:
        consume(args.npm.resolve(), args.registry.resolve(), Path(temporary))


if __name__ == '__main__':
    main()
