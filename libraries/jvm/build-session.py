#!/usr/bin/env python3
"""Build the bounded stable-JDK session package from the actual product inventory."""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import zipfile
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env
sys.path.insert(0, str(ROOT / 'libraries/jvm/tests'))
from toolchains import JDK, KOTLIN, SCALA, stable, JDK_FLOOR


def maven_package(jars, inventory, classifier):
    """Assemble the locally supplied family with its ordinary Maven coordinates."""
    project = ET.parse(jars / 'pom.xml').getroot()
    group, artifact, version = (project.find('{*}' + field).text for field in ('groupId', 'artifactId', 'version'))
    dependencies = project.findall('{*}dependencies/{*}dependency')
    native = [dep for dep in dependencies if dep.find('{*}classifier') is not None]
    for dep in native:
        if [dep.find('{*}' + field).text for field in ('groupId', 'artifactId', 'version')] != ['${project.groupId}', '${project.artifactId}', '${project.version}']:
            raise ValueError('Maven native dependencies must use this package version')
    repository = jars.parent / 'maven' / Path(*group.split('.')) / artifact / version
    repository.mkdir(parents=True)
    prefix = artifact + '-' + version
    shutil.copyfile(jars / 'pom.xml', repository / (prefix + '.pom'))
    for kind, filename in inventory['jars'].items():
        suffix = '' if kind == 'door' else '-' + kind
        shutil.copyfile(jars / filename, repository / (prefix + suffix + '.jar'))
    shutil.copyfile(jars / ('thinkthen-' + classifier + '.jar'), repository / (prefix + '-' + classifier + '.jar'))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--native', type=Path, required=True)
    parser.add_argument('--target', required=True)
    args = parser.parse_args()
    stable()
    if not args.native.is_file():
        raise ValueError('JVM package needs a matching native library')
    if not args.out.is_absolute() or args.out.is_symlink():
        raise ValueError("JVM package output must be an absolute real directory")
    out = args.out.resolve()
    if out.exists() and any(out.iterdir()):
        raise ValueError("Session package output must be empty")
    out.mkdir(parents=True, exist_ok=True)
    jdk, kotlin, scala = JDK, KOTLIN, SCALA
    env = child_env(keep=('LANG','LC_ALL'), JAVA_HOME=str(jdk), JAVACMD=str(jdk / 'bin/java'), JAVA_OPTS='-Xmx1g -XX:ActiveProcessorCount=2')
    inventory = json.loads(subprocess.check_output([sys.executable, str(ROOT / 'sdlc/scripts/package-inventory.py'), 'jvm'], env=env))
    subprocess.run([sys.executable, str(ROOT / 'sdlc/generators/results/generate.py'), '--target', 'jvm', '--check'], check=True, env=env)
    jars = out / 'jars'
    jars.mkdir(exist_ok=True)
    for kind, filename in inventory['jars'].items():
        classes = out / 'classes' / kind
        classes.mkdir(parents=True, exist_ok=True)
        if kind == 'door':
            sources = sorted((ROOT / 'libraries/jvm/session/thinkthen').glob('*.java'))
            sources.append(ROOT / 'libraries/jvm/door/thinkthen/Json.java')
            command = [str(jdk / 'bin/javac'), '-J-Xmx1g', '--release', str(JDK_FLOOR), '-d', str(classes), *map(str,sources)]
        elif kind == 'kotlin':
            command = [str(kotlin / 'bin/kotlinc'), '-J-Xmx1g', '-J-XX:ActiveProcessorCount=2', '-jvm-target', str(JDK_FLOOR), '-classpath', str(jars / inventory['jars']['door']) + os.pathsep + str(kotlin / 'lib/kotlinx-coroutines-core-jvm.jar'), *map(str, sorted((ROOT / 'libraries/jvm/session/kotlin').glob('*.kt'))), '-d', str(classes)]
        else:
            command = [str(scala / 'bin/scalac'), '-J-Xmx1g', '-J-XX:ActiveProcessorCount=2', '-classpath', str(jars / inventory['jars']['door']), '-d', str(classes), *map(str, sorted((ROOT / 'libraries/jvm/session/scala').glob('*.scala')))]
        subprocess.run(command, check=True, env=env)
        with zipfile.ZipFile(jars / filename, 'w', zipfile.ZIP_DEFLATED) as jar:
            for file in sorted(classes.rglob('*')):
                if file.is_file(): jar.write(file, file.relative_to(classes))
            if kind == 'door': jar.writestr('META-INF/thinkthen/product-inventory.json', json.dumps(inventory))
    classifier, selected = next((name,asset) for name,asset in inventory['native'].items() if asset['target'] == args.target)
    with zipfile.ZipFile(jars / ('thinkthen-' + classifier + '.jar'), 'w', zipfile.ZIP_DEFLATED) as jar:
        for resource in selected['files']: jar.write(args.native, resource)
    for name in ('pom.xml','LICENSE','README.md'):
        shutil.copyfile(ROOT / 'libraries/jvm' / name,jars / name)
    actual = json.loads(subprocess.check_output([sys.executable, str(ROOT / 'sdlc/scripts/package-inventory.py'), 'jvm', '--out', str(out)], env=env))
    (jars / 'product-inventory.json').write_text(json.dumps(actual,indent=2) + '\n')
    maven_package(jars, inventory, classifier)
    print('Stable JVM session JARs, selected native classifier and local Maven coordinates built')


if __name__ == '__main__':
    main()
