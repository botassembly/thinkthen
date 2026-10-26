#!/usr/bin/env python3
"""Prepare one pinned official EventPipe decoder without a global installation."""
import base64
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import urllib.request
import xml.etree.ElementTree as ET
import zipfile

VERSION = '10.0.750501'
BASE = 'https://api.nuget.org/v3-flatcontainer/dotnet-trace/' + VERSION


def fetch(url):
    with urllib.request.urlopen(url, timeout=120) as response:
        return response.read()


def main():
    output = Path(sys.argv[1]).resolve()
    output.mkdir(parents=True, exist_ok=False)
    registration = json.loads(fetch('https://api.nuget.org/v3/registration5-semver1/dotnet-trace/' + VERSION + '.json'))
    catalog = json.loads(fetch(registration['catalogEntry']))
    archive = fetch(BASE + '/dotnet-trace.' + VERSION + '.nupkg')
    digest = base64.b64encode(hashlib.sha512(archive).digest()).decode()
    if catalog['packageHashAlgorithm'] != 'SHA512' or digest != catalog['packageHash']:
        raise ValueError('official catalog package hash did not match')
    package = output / ('dotnet-trace.' + VERSION + '.nupkg')
    package.write_bytes(archive)
    (output / 'catalog.json').write_text(json.dumps(catalog, indent=2))
    extracted = output / 'package'
    with zipfile.ZipFile(package) as source:
        source.extractall(extracted)
    nuspec, = extracted.glob('*.nuspec')
    metadata = ET.parse(nuspec).getroot().find('{*}metadata')
    if metadata.findtext('{*}id') != 'dotnet-trace' or metadata.findtext('{*}version') != VERSION:
        raise ValueError('unexpected tool package identity')
    if 'Microsoft' not in metadata.findtext('{*}authors'):
        raise ValueError('unexpected tool package author')
    trace, = extracted.rglob('Microsoft.Diagnostics.Tracing.TraceEvent.dll')
    tool = trace.parent
    dotnet = str(Path(os.environ['DOTNET_ROOT']) / 'dotnet')
    env = dict(os.environ, DOTNET_CLI_HOME=str(output / 'home'), NUGET_PACKAGES=str(output / 'nuget'))
    with (output / 'signature.log').open('w') as log:
        signed = subprocess.run([dotnet, 'nuget', 'verify', str(package), '--all'], env=env, stdout=log, stderr=subprocess.STDOUT)
    signed.check_returncode()
    project = output / 'probe'
    project.mkdir()
    shutil.copyfile(Path(__file__).with_name('csharp-trace-probe.cs'), project / 'Program.cs')
    root = ET.Element('Project', Sdk='Microsoft.NET.Sdk')
    props = ET.SubElement(root, 'PropertyGroup')
    for key,value in dict(OutputType='Exe',TargetFramework='net10.0',ImplicitUsings='enable',Nullable='enable',TreatWarningsAsErrors='true').items():
        ET.SubElement(props,key).text=value
    references = ET.SubElement(root, 'ItemGroup')
    for library in sorted(tool.glob('*.dll')):
        if library.name != 'dotnet-trace.dll':
            reference = ET.SubElement(references, 'Reference', Include=library.stem)
            ET.SubElement(reference,'HintPath').text=str(library)
    ET.ElementTree(root).write(project / 'Probe.csproj')
    config = project / 'NuGet.Config'
    config.write_text('<configuration><packageSources><clear /></packageSources></configuration>')
    with (output / 'build.log').open('w') as log:
        subprocess.run([dotnet,'build',str(project / 'Probe.csproj'),'--configfile',str(config),'-o',str(output / 'bin'),'--disable-build-servers'],env=env,stdout=log,stderr=subprocess.STDOUT,check=True)
    schema = subprocess.check_output([dotnet,str(output / 'bin/Probe.dll')],env=env,text=True)
    (output / 'schema.json').write_text(schema)
    provenance = dict(version=VERSION,url=BASE,sha256=hashlib.sha256(archive).hexdigest(),sha512=digest,
                      catalog_url=registration['catalogEntry'],tool_directory=str(tool),signature_exit=signed.returncode)
    (output / 'provenance.json').write_text(json.dumps(provenance,indent=2)+'\n')
    print(json.dumps(provenance),flush=True)


if __name__ == '__main__':
    main()
