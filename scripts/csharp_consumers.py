"""Check installed C# compile boundaries and trimming-safe public consumers."""
import json
import os
from pathlib import Path
import re
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
        if result.returncode == 0 or 'error CS' not in result.stdout or 'error NU' in result.stdout:
            raise RuntimeError(f'negative compilation did not reject {case.name}:\n{result.stdout}')
        print('C# rejected invalid consumer:', case.name, flush=True)


def stub_example(example):
    return example.replace(
        "new ThinkThenClient()",
        "new ThinkThenClient(new ThinkThenOptions { Backend = Backend.Stub })")


def readme(stage):
    examples = re.findall(r'```csharp\n(.*?)\n```', (ROOT / 'sdk/csharp/README.md').read_text(), re.S)
    if not examples:
        raise RuntimeError('C# README contains no examples')
    for number, example in enumerate(examples, 1):
        folder = stage / ('readme-' + str(number))
        reference = project(folder, 'Example')
        example = stub_example(example)
        (folder / 'Program.cs').write_text(example + '\n')
        configuration = config(stage, folder)
        run('dotnet', 'restore', reference, '--configfile', configuration)
        run('dotnet', 'run', '--no-restore', '--project', reference)
        print('C# README example passed:', number, flush=True)


def guide(stage):
    folder = stage / 'guide'
    reference = project(folder, 'Example')
    helper = ROOT / 'scripts/surface-example'
    run(sys.executable, helper, 'prepare', 'csharp', folder)
    configuration = config(stage, folder)
    run('dotnet', 'restore', reference, '--configfile', configuration)
    subprocess.run([sys.executable, str(helper), 'check', 'csharp', 'dotnet', 'run',
                    '--no-restore', '--project', str(reference)], cwd=folder, check=True)


def metadata(stage):
    folder = stage / 'aot'
    reference = project(folder, 'AotConsumer', aot=True)
    shutil.copyfile(ROOT / 'sdk/csharp/tests/aot/Program.cs.txt', folder / 'Program.cs')
    configuration = config(stage, folder)
    run('dotnet', 'publish', reference, '--configfile', configuration, '-o', stage / 'aot-bin')
    executable = 'AotConsumer.exe' if os.name == 'nt' else 'AotConsumer'
    run(stage / 'aot-bin' / executable)
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
    if sys.argv[1] == 'readme':
        readme(stage)
    elif sys.argv[1] == 'abi':
        abi(stage)
    else:
        negative(stage)
        readme(stage)
        guide(stage)
        metadata(stage)
