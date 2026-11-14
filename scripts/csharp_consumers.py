"""Check installed C# compile boundaries and trimming-safe public consumers."""
import json
import os
from pathlib import Path
import re
import runpy
import shutil
import subprocess
import sys
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parent.parent


def run(*arguments):
    subprocess.run(list(map(str, arguments)), check=True)


def project(folder, name, output='Exe', aot=False):
    folder.mkdir()
    source = ET.parse(ROOT / 'sdk/csharp/tests/Consumer.csproj')
    properties = source.find('PropertyGroup')
    properties.find('OutputType').text = output
    if aot:
        ET.SubElement(properties, 'PublishAot').text = 'true'
        ET.SubElement(properties, 'IsAotCompatible').text = 'true'
    destination = folder / (name + '.csproj')
    source.write(destination, encoding='unicode')
    return destination


def package_config(candidate, folder):
    from csharp_dependencies import prepared_feed
    configuration = ET.Element('configuration')
    sources = ET.SubElement(configuration, 'packageSources')
    ET.SubElement(sources, 'clear')
    for name, path in [('candidate', candidate), ('prepared', prepared_feed())]:
        ET.SubElement(sources, 'add', key=name, value=str(path))
    destination = folder / 'NuGet.Config'
    ET.ElementTree(configuration).write(destination, encoding='unicode')
    return destination


def config(stage, folder):
    return package_config(stage / 'feed', folder)


def negative(stage):
    folder = stage / 'negative'
    reference = project(folder, 'Negative', 'Library')
    configuration = config(stage, folder)
    run('dotnet', 'restore', reference, '--configfile', configuration)
    for case in sorted((ROOT / 'sdk/csharp/tests/negative').glob('*.cs.txt')):
        shutil.copyfile(case, folder / 'Invalid.cs')
        result = subprocess.run(['dotnet', 'build', str(reference), '--no-restore', '--nologo'],
                                stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
        expected=re.search(r'^// Expected: (.+)$',case.read_text(),re.M)
        if result.returncode == 0 or 'error CS' not in result.stdout or 'error NU' in result.stdout or (expected and not re.search(expected[1],result.stdout)):
            raise RuntimeError(f'negative compilation did not reject {case.name} at its intended boundary:\n{result.stdout}')
        print('C# rejected invalid consumer:', case.name, flush=True)


def example_environment():
    environment = dict(os.environ, THINKTHEN_BACKEND='stub')
    for name in ('THINKTHEN_API_KEY', 'OPENAI_API_KEY', 'THINKTHEN_BASE_URL',
                 'THINKTHEN_URL', 'THINKTHEN_BATCH'):
        environment.pop(name, None)
    return environment


def example(stage, folder, code, toolbox=None):
    reference = project(folder, 'Example')
    (folder / 'Program.cs').write_text(code + '\n')
    configuration = config(stage, folder)
    if toolbox is not None:
        (folder / 'toolbox.json').write_text(toolbox + '\n')
    run('dotnet', 'restore', reference, '--configfile', configuration)
    result = subprocess.run(['dotnet', 'run', '--no-restore', '--project', str(reference)],
                            env=example_environment(),
                            cwd=folder, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    if result.returncode:
        raise RuntimeError(result.stdout)
    if result.stdout:
        print(result.stdout, end='', flush=True)


def readme(stage):
    text = (ROOT / 'sdk/csharp/README.md').read_text()
    examples = list(re.finditer(r'```csharp\n(.*?)\n```', text, re.S))
    if not examples:
        raise RuntimeError('C# README contains no examples')
    for number, block in enumerate(examples, 1):
        example(stage, stage / ('readme-' + str(number)), block[1])
        print('C# README example passed:', number, flush=True)


def guide(stage):
    from guide_csharp import run_csharp
    run_csharp(stage)


def build_consumer(stage):
    folder=stage / 'consumer'
    reference=project(folder,'Consumer')
    for source in (ROOT / 'sdk/csharp/tests').glob('*.cs'):
        shutil.copyfile(source,folder / source.name)
    configuration=config(stage,folder)
    run('dotnet','build',reference,'--configfile',configuration,'--nologo','-o',stage / 'consumer-bin')
    return stage / 'consumer-bin/Consumer.dll'


def consumer(stage):
    run('dotnet',build_consumer(stage),ROOT / 'construction/stub.json',ROOT / 'conformance',os.environ['THINKTHEN_LOOPBACK'])


def projection(stage):
    from guide_examples import scenario
    assembly=build_consumer(stage)
    for name in ('yes','unsure','failed','relations'):
        with scenario(name) as (work, environment):
            subprocess.run(['dotnet',str(assembly),'--ideal-host',name],cwd=work,env=environment,check=True)
    producer=runpy.run_path(str(ROOT / 'scripts/abi-minor-check'))
    filename='libthinkthen_ffi.dylib' if sys.platform=='darwin' else 'libthinkthen_ffi.so'
    engine,=assembly.parent.rglob(filename)
    original=stage / 'projection-original' / filename
    original.parent.mkdir()
    shutil.copy2(engine,original)
    fixtures=stage / 'malformed'
    for kind in ('invalid-json','unanswered-answer','unknown-function'):
        producer['malformed_fixture'](original,fixtures / kind,kind)
    run(sys.executable,ROOT / 'sdk/csharp/tests/malformed.py',assembly.parent,fixtures)


def metadata(stage):
    folder = stage / 'aot'
    reference = project(folder, 'AotConsumer', aot=True)
    shutil.copyfile(ROOT / 'sdk/csharp/tests/aot/Program.cs.txt', folder / 'Program.cs')
    configuration = config(stage, folder)
    run('dotnet', 'publish', reference, '--configfile', configuration, '-o', stage / 'aot-bin')
    executable = 'AotConsumer.exe' if os.name == 'nt' else 'AotConsumer'
    from guide_examples import scenario
    with scenario('yes') as (_, environment):
        subprocess.run([str(stage / 'aot-bin' / executable)], env=environment, check=True)
    print('C# Native AOT metadata consumer passed', flush=True)


def abi(stage):
    from native_platform import current
    host = current()
    fixtures = Path(os.environ['THINKTHEN_NATIVE_FIXTURES'])
    manifest = json.loads((fixtures / 'manifest.json').read_text())
    filename = 'libthinkthen_ffi.' + host['suffix']
    for case in manifest['cases']:
        folder = stage / ('abi-' + case['name'])
        shutil.copytree(stage / 'bin', folder)
        native = folder / 'runtimes' / host['rid'] / 'native' / filename
        if not native.exists():
            native = folder / filename
        shutil.copyfile(case['library'], native)
        arguments = [ROOT / 'construction/stub.json', '--stub-only'] if case['accept'] else ['--abi-refusal']
        run('dotnet', folder / 'Consumer.dll', *arguments)
        print('Installed C# ABI fixture passed:', case['name'], flush=True)


if __name__ == '__main__':
    stage = Path(sys.argv[2]).resolve()
    if sys.argv[1] == 'examples':
        readme(stage)
        guide(stage)
    elif sys.argv[1] == 'readme':
        readme(stage)
    elif sys.argv[1] == 'consumer':
        consumer(stage)
    elif sys.argv[1] == 'projection':
        projection(stage)
    elif sys.argv[1] == 'metadata':
        metadata(stage)
    elif sys.argv[1] == 'abi':
        abi(stage)
    elif sys.argv[1] == 'interface':
        negative(stage)
    else:
        raise SystemExit('usage: csharp_consumers.py examples|readme|consumer|projection|metadata|abi|interface STAGE')
