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

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--native', type=Path, required=True)
    parser.add_argument('--target', required=True)
    args = parser.parse_args()
    out = args.out.resolve()
    if out.exists() and any(out.iterdir()):
        raise ValueError("Session package output must be empty")
    out.mkdir(parents=True, exist_ok=True)
    jdk = Path(os.environ['THINKTHEN_JDK_HOME'])
    kotlin = Path(os.environ['THINKTHEN_KOTLIN_HOME'])
    scala = Path(os.environ['THINKTHEN_SCALA_HOME'])
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
            command = [str(jdk / 'bin/javac'), '-J-Xmx1g', '--release', '22', '-d', str(classes), *map(str,sources)]
        elif kind == 'kotlin':
            command = [str(kotlin / 'bin/kotlinc'), '-J-Xmx1g', '-J-XX:ActiveProcessorCount=2', '-jvm-target', '22', '-classpath', str(jars / inventory['jars']['door']) + os.pathsep + str(kotlin / 'lib/kotlinx-coroutines-core-jvm.jar'), str(ROOT / 'libraries/jvm/session/kotlin/KotlinEngine.kt'), '-d', str(classes)]
        else:
            command = [str(scala / 'bin/scalac'), '-J-Xmx1g', '-J-XX:ActiveProcessorCount=2', '-classpath', str(jars / inventory['jars']['door']), '-d', str(classes), str(ROOT / 'libraries/jvm/session/scala/ScalaEngine.scala')]
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
    print('Stable JVM session JARs and inventory-selected native classifier built')


if __name__ == '__main__':
    main()
