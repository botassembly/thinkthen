"""Prepare and select the pinned Kotlin compiler independently of Maven cache contents."""
import json
import os
from pathlib import Path
import subprocess
import shutil
import tempfile
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parent.parent
MANIFEST = '.thinkthen-kotlin-compiler.json'


def version():
    namespace = {'m': 'http://maven.apache.org/POM/4.0.0'}
    return ET.parse(ROOT / 'sdk/java/pom.xml').find('m:properties/m:kotlin.version', namespace).text


def repository():
    cache = Path(os.environ.get('THINKTHEN_JVM_CACHE', Path.home() / '.local/share/thinkthen-build-tools/jvm'))
    return cache / 'm2'


def classpath(folder=None):
    folder = Path(folder) if folder is not None else repository()
    closure = json.loads((folder / MANIFEST).read_text())
    if closure['version'] != version():
        raise ValueError('prepare the pinned Kotlin compiler with scripts/jvm-prepare')
    jars = [folder / name for name in closure['jars']]
    if not jars or any(not jar.is_file() for jar in jars):
        raise ValueError('prepared Kotlin compiler closure is incomplete')
    return os.pathsep.join(str(jar.resolve()) for jar in jars)


def prepare(offline=False):
    folder = repository().resolve()
    with tempfile.TemporaryDirectory(prefix='kotlin-compiler-') as temporary:
        scratch = Path(temporary)
        pom = scratch / 'pom.xml'
        pom.write_text('<project xmlns="http://maven.apache.org/POM/4.0.0">'
                       '<modelVersion>4.0.0</modelVersion><groupId>dev.thinkthen.check</groupId>'
                       '<artifactId>kotlin-compiler</artifactId><version>1</version><dependencies>'
                       '<dependency><groupId>org.jetbrains.kotlin</groupId>'
                       '<artifactId>kotlin-compiler-embeddable</artifactId><version>' + version() +
                       '</version></dependency></dependencies></project>')
        output = scratch / 'classpath'
        namespace = {'m': 'http://maven.apache.org/POM/4.0.0'}
        parent = ET.parse(ROOT / 'sdk/java/pom.xml')
        plugins = parent.findall('m:build/m:pluginManagement/m:plugins/m:plugin', namespace)
        plugin_version, = [plugin.find('m:version', namespace).text for plugin in plugins
                           if plugin.find('m:artifactId', namespace).text == 'maven-dependency-plugin']
        goal = 'org.apache.maven.plugins:maven-dependency-plugin:' + plugin_version + ':build-classpath'
        subprocess.run([shutil.which('mvn') or 'mvn', '-B', '-q', '-f', str(pom), '-Dmaven.repo.local=' + str(folder),
                        goal, '-Dmdep.outputFile=' + str(output), *(['-o'] if offline else [])], check=True)
        jars = [str(Path(name).relative_to(folder)) for name in output.read_text().strip().split(os.pathsep)]
        (folder / MANIFEST).write_text(json.dumps({'version': version(), 'jars': jars}) + '\n')
    classpath(folder)


if __name__ == '__main__':
    import sys
    prepare(offline='--offline' in sys.argv[1:])
