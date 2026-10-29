"""Prepare only the dependencies used by final registry package consumers."""
import os
import shutil
import tarfile

from csharp_dependencies import prepare_project
from jvm_repository import consumer
from release_downloads import fetch
from release_registry_check import environment, run
from release_workflow import ROOT


def dotnet(scratch):
    feed = scratch / 'prepared-nuget'
    source = scratch / 'dependency-project'
    source.mkdir()
    (source / 'Consumer.csproj').write_text(prepare_project(), encoding='utf-8')
    (source / 'Program.cs').write_text('System.Console.WriteLine("dependencies");\n', encoding='utf-8')
    (source / 'NuGet.Config').write_text('<configuration><packageSources><clear/></packageSources></configuration>', encoding='utf-8')
    os.environ['THINKTHEN_WINDOWS_SINGLEFILE_FEED'] = str(feed)
    run('dotnet', 'restore', source / 'Consumer.csproj', '--packages', feed,
        '--source', 'https://api.nuget.org/v3/index.json', env=environment(scratch))


def jvm(packages, scratch):
    url = 'https://archive.apache.org/dist/maven/maven-3/3.9.11/binaries/apache-maven-3.9.11-bin.tar.gz'
    archive = fetch(url, scratch / 'maven.tar.gz')
    with tarfile.open(archive) as packed:
        packed.extractall(scratch / 'tools', filter='data')
    os.environ['PATH'] = str(scratch / 'tools/apache-maven-3.9.11/bin') + os.pathsep + os.environ['PATH']
    repository = scratch / 'm2'
    shutil.copytree(packages / 'maven', repository)
    staged = consumer(ROOT, scratch / 'dependency-jvm')
    # Compile only the consumer to resolve plugin dependencies that go-offline
    # omits. The proof later stages a fresh consumer and permits no network.
    run('mvn', '-B', '-q', '-Dmaven.repo.local=' + str(repository), '-f', staged / 'pom.xml',
        'package', 'dependency:build-classpath', '-Dmdep.outputFile=' + str(scratch / 'prepared-classpath'),
        env=environment(scratch))
