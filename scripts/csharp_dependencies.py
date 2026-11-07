"""Derive the pinned .NET SDK and the offline preparation project from their sources."""
import json
import os
from pathlib import Path
import platform
import xml.etree.ElementTree as ET

from release_downloads import PINS

ROOT = Path(__file__).resolve().parent.parent


def sdk_archive():
    system = 'osx' if platform.system() == 'Darwin' else 'linux'
    arch = 'arm64' if platform.machine() in ('arm64', 'aarch64') else 'x64'
    url, = [url for url in json.loads(PINS.read_text()) if '/Sdk/' in url and url.endswith(f'-{system}-{arch}.tar.gz')]
    return url


def sdk_version():
    return sdk_archive().split('/Sdk/')[1].split('/')[0]


def package_id():
    return ET.parse(ROOT / 'sdk/csharp/ThinkThen.csproj').findtext('.//PackageId')


def prepare_project(aot=False, advisories_stop=True):
    library = ET.parse(ROOT / 'sdk/csharp/ThinkThen.csproj')
    identity = package_id()
    project = ET.Element('Project', Sdk='Microsoft.NET.Sdk')
    properties = ET.SubElement(project, 'PropertyGroup')
    values = {'OutputType': 'Exe', 'TargetFramework': library.findtext('.//TargetFramework'),
              'PublishSingleFile': 'true', 'SelfContained': 'false', 'IsAotCompatible': 'true'}
    if advisories_stop:
        # Only a restore from nuget.org can see advisories, so this one must stop on them,
        # and on NU1900 and NU1905, which otherwise pass unaudited without advisory data.
        values['WarningsAsErrors'] = 'NU1900;NU1901;NU1902;NU1903;NU1904;NU1905'
    if aot:
        values['PublishAot'] = 'true'
        values['SelfContained'] = 'true'
    for name, value in values.items():
        ET.SubElement(properties, name).text = value
    references = {}
    for source in (library, ET.parse(ROOT / 'sdk/csharp/tests/Consumer.csproj')):
        for reference in source.findall('.//PackageReference'):
            if reference.get('Include') != identity:
                references[reference.get('Include')] = reference.attrib
    packages = ET.SubElement(project, 'ItemGroup')
    for name in sorted(references):
        ET.SubElement(packages, 'PackageReference', references[name])
    return ET.tostring(project, encoding='unicode')


def prepared_feed():
    if os.environ.get('THINKTHEN_WINDOWS_SINGLEFILE_FEED'):
        return Path(os.environ['THINKTHEN_WINDOWS_SINGLEFILE_FEED'])
    return Path(os.environ.get('THINKTHEN_DOTNET_HOME',
                              Path.home() / '.local/share/thinkthen-build-tools/dotnet10')) / 'singlefile-packages'
