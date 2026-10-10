#!/usr/bin/env python3
"""Run installed stable-JDK consumers with only JARs visible; count real sends."""
import argparse
import collections
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import zipfile
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT / 'conformance/children'))
from children import child_env
sys.path.insert(0, str(ROOT / 'libraries/jvm/tests'))
from toolchains import JDK_FLOOR


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--jars',type=Path,required=True)
    parser.add_argument('--out',type=Path,required=True)
    parser.add_argument('--maven',action='store_true',help='Consume the versioned local Maven files instead of development JAR names')
    args = parser.parse_args()
    version = ET.parse(args.jars / 'pom.xml').getroot().find('{http://maven.apache.org/POM/4.0.0}version').text
    logs = args.out.resolve()
    logs.mkdir(parents=True,exist_ok=True)
    work = Path(tempfile.mkdtemp(prefix="installed-",dir=logs))
    feed = work / 'feed'; feed.mkdir(exist_ok=True)
    if args.maven:
        project = ET.parse(args.jars / 'pom.xml').getroot()
        group, artifact = (project.find('{*}' + field).text for field in ('groupId','artifactId'))
        repository = args.jars.parent / 'maven' / Path(*group.split('.')) / artifact / version
        prefix = artifact + '-' + version
        assert (repository / (prefix + '.pom')).read_bytes() == (args.jars / 'pom.xml').read_bytes()
        for file in args.jars.glob('*.jar'):
            kind = file.stem.removeprefix('thinkthen-')
            suffix = '' if kind == 'door' else '-' + kind
            source = repository / (prefix + suffix + '.jar')
            assert source.read_bytes() == file.read_bytes(),source
            shutil.copyfile(source,feed / file.name)
    else:
        for file in args.jars.glob('*.jar'): shutil.copyfile(file,feed / file.name)
    for name in ('home','barrier','app'): (work / name).mkdir(exist_ok=True)
    source = ROOT / 'libraries/jvm/tests/SessionConsumer.java'
    shutil.copyfile(source,work / source.name)
    shutil.copyfile(ROOT / "libraries/jvm/tests/JsonTest.java", work / "JsonTest.java")
    jdk = Path(os.environ['THINKTHEN_JDK_HOME'])
    env = child_env(keep=('LANG','LC_ALL'),home=work / 'home', JAVA_HOME=str(jdk), JAVACMD=str(jdk / 'bin/java'))
    subprocess.run([str(jdk / 'bin/javac'),'--release',str(JDK_FLOOR),'-cp',str(feed / 'thinkthen-door.jar'),'-d',str(work / 'app'),str(work / source.name),str(work / "JsonTest.java")],check=True,env=env)
    kotlin = Path(os.environ['THINKTHEN_KOTLIN_HOME'])
    scala = Path(os.environ['THINKTHEN_SCALA_HOME'])
    for filename in ('SessionKotlin.kt','SessionScala.scala'):
        shutil.copyfile(ROOT / 'libraries/jvm/tests' / filename,work / filename)
    kotlin_cp = os.pathsep.join(map(str,(feed / 'thinkthen-door.jar',feed / 'thinkthen-kotlin.jar',kotlin / 'lib/kotlinx-coroutines-core-jvm.jar')))
    subprocess.run([str(kotlin / 'bin/kotlinc'),'-J-Xmx1g','-J-XX:ActiveProcessorCount=2','-jvm-target',str(JDK_FLOOR),'-classpath',kotlin_cp,str(work / 'SessionKotlin.kt'),'-d',str(work / 'app')],check=True,env=env)
    subprocess.run([str(scala / 'bin/scalac'),'-J-Xmx1g','-J-XX:ActiveProcessorCount=2','-classpath',str(feed / 'thinkthen-door.jar') + os.pathsep + str(feed / 'thinkthen-scala.jar'),'-d',str(work / 'app'),str(work / 'SessionScala.scala')],check=True,env=env)
    backend_source = ROOT / 'libraries/csharp/tests/backend.py'
    spec = importlib.util.spec_from_file_location('shared_package_backend',backend_source)
    backend = importlib.util.module_from_spec(spec); spec.loader.exec_module(backend)
    server = backend.Backend(work / 'barrier')
    env = child_env(home='/work/home',LANG='C.UTF-8',LC_ALL='C.UTF-8',THINKTHEN_BASE_URL=f'http://127.0.0.1:{server.server_port}/generic/v1',THINKTHEN_API_KEY='tt-canary-290')
    command = ['/usr/bin/bwrap','--unshare-all','--share-net','--die-with-parent','--ro-bind','/usr/lib','/usr/lib','--ro-bind','/lib','/lib','--ro-bind','/lib64','/lib64','--ro-bind',str(jdk),'/jdk','--bind',str(work),'/work','--proc','/proc','--dev','/dev','--tmpfs','/tmp','--chdir','/work','--','/jdk/bin/java','--enable-native-access=ALL-UNNAMED','-XX:ActiveProcessorCount=2','-Xmx1g','-cp','/work/feed/*:/work/app','SessionConsumer']
    try:
        parser_result = subprocess.run(command[:-1] + ["JsonTest"], env=env, capture_output=True, timeout=5)
        assert parser_result.returncode == 0 and b"JSON_READER_PASS" in parser_result.stdout, (parser_result.stdout, parser_result.stderr)
        print("Installed JSON interchange regressions PASS")
        result = subprocess.run(command,env=env,capture_output=True,timeout=50)
        (logs / 'java-consumer.log').write_bytes(result.stdout + result.stderr)
        print(result.stdout.decode(),end='')
        if result.returncode:
            print(result.stderr.decode(),file=sys.stderr)
            raise AssertionError(f'installed JVM exit {result.returncode}')
        assert b'INSTALLED_JVM_SESSION_PASS' in result.stdout
        expected = ['session-decide','session-choose','session-tag','session-score','session-filter','session-rank','[{\"id\":\"u001\",\"evidence\":\"session-find\"},{\"id\":\"u002\",\"evidence\":\"session-find-two\"}]','session-annotate','Maria Chen','Maria Chen',{'entities':[{'id':'i1','name':'First','kind':'alert'},{'id':'i2','name':'Second','kind':'alert'}]},'status-401','hold-jvm-java','session-independent']
        assert server.arrivals == expected,server.arrivals
        expected_agents = [f'thinkthen/{version} (java)'] * len(expected)
        assert server.attempts == server.connections == len(expected),server.arrivals
        assert all('malformed' not in str(item) and 'preview' not in str(item) for item in server.arrivals),server.arrivals
        print('Installed Java ten functions, counted zero-send refusals, held cancellation and owned results PASS')
        runtime = command[:command.index('--')]
        for language, sdk, mainclass, runtime_cp in (
            ('kotlin',kotlin,'SessionKotlinKt','/kotlin/lib/kotlin-stdlib.jar:/kotlin/lib/kotlinx-coroutines-core-jvm.jar'),
            ('scala',scala,'SessionScala','/scala/lib/scala.jar')):
            consumer = runtime + ['--ro-bind',str(sdk),'/' + language,'--','/jdk/bin/java','--enable-native-access=ALL-UNNAMED','-XX:ActiveProcessorCount=2','-Xmx1g','-cp','/work/feed/*:/work/app:' + runtime_cp,mainclass]
            actual = subprocess.run(consumer,env=env,capture_output=True,timeout=25)
            (logs / (language + '-consumer.log')).write_bytes(actual.stdout + actual.stderr)
            assert actual.returncode == 0,(actual.stdout,actual.stderr)
            assert ('INSTALLED_JVM_' + language.upper() + '_SESSION_PASS').encode() in actual.stdout,actual.stdout
            previous = len(expected)
            expected += ['hold-jvm-' + language,'session-' + language + '-independent']
            if language == 'scala': expected += ['session-scala-nested']
            expected += ['status-401']
            expected_agents += [f'thinkthen/{version} ({language})'] * (len(expected) - previous)
            assert server.arrivals == expected,server.arrivals
            print(actual.stdout.decode(),end='')
        assert server.attempts == server.connections == len(expected),server.arrivals
        assert server.user_agents == expected_agents,server.user_agents
        print('Installed Kotlin coroutine and Scala Future cancellation while held PASS')
    finally:
        for file in (work / 'barrier').glob('arrived-*'):
            file.with_name(file.name.replace('arrived-','release-',1)).touch()
        server.close()
        shutil.rmtree(work)


if __name__ == '__main__':
    main()
