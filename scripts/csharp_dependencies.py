"""Derive the offline .NET preparation project from package references."""
import os
from pathlib import Path
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parent.parent


def prepare_project(aot=False):
    library = ET.parse(ROOT / 'sdk/csharp/ThinkThen.csproj')
    identity = library.findtext('.//PackageId')
    project = ET.Element('Project', Sdk='Microsoft.NET.Sdk')
    properties = ET.SubElement(project, 'PropertyGroup')
    values = {'OutputType': 'Exe', 'TargetFramework': library.findtext('.//TargetFramework'),
              'PublishSingleFile': 'true', 'SelfContained': 'false', 'IsAotCompatible': 'true'}
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
