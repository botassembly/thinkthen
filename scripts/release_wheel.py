"""Assemble an abi3 wheel around the already-built native Python module."""
import base64
import hashlib
import importlib.util
from pathlib import Path
import shutil
import zipfile
import tomllib


def python_wheel(stage, out, version, native, tag="cp310-abi3-win_amd64", notices=None):
    root = Path(__file__).resolve().parent.parent
    wheel = out / f'python/thinkthen-{version}-{tag}.whl'
    package = stage / 'wheel'
    shutil.copytree(root / 'sdk/python/thinkthen', package / 'thinkthen')
    spec = importlib.util.spec_from_file_location('python_build', root / 'sdk/python/relation_type.py')
    builder = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(builder)
    builder.relation_type(package / 'thinkthen')
    shutil.copy2(native, package / ('thinkthen/_native.pyd' if tag.endswith('win_amd64') else 'thinkthen/_native.abi3.so'))
    metadata = package / f'thinkthen-{version}.dist-info'
    metadata.mkdir()
    project = tomllib.loads((root / 'sdk/python/pyproject.toml').read_text())['project']
    license = project['license']
    fields = [('Metadata-Version', '2.4'), ('Name', project['name']), ('Version', version),
              ('Summary', project['description']), ('Requires-Python', project['requires-python']),
              ('License-Expression', license['text'] if isinstance(license, dict) else license),
              ('License-File', 'LICENSE'), ('Description-Content-Type', 'text/markdown')]
    fields.extend(('Project-URL', name + ', ' + url) for name, url in project.get('urls', {}).items())
    fields.extend(('Requires-Dist', dependency) for dependency in project.get('dependencies', []))
    for extra, dependencies in project.get('optional-dependencies', {}).items():
        fields.append(('Provides-Extra', extra))
        for dependency in dependencies:
            requirement, separator, marker = dependency.partition(';')
            condition = f'({marker.strip()}) and ' if separator else ''
            fields.append(('Requires-Dist', requirement.strip() + f'; {condition}extra == "{extra}"'))
    (metadata / 'METADATA').write_text(''.join(f'{key}: {value}\n' for key, value in fields) +
                                      '\n' + (root / 'sdk/python/README.md').read_text())
    (metadata / 'WHEEL').write_text('Wheel-Version: 1.0\nGenerator: ThinkThen\nRoot-Is-Purelib: false\nTag: ' + tag + '\n')
    (metadata / 'licenses').mkdir()
    shutil.copy2(root / 'LICENSE', metadata / 'licenses/LICENSE')
    shutil.copy2(root / 'sdk/python/README.md', (package if tag.endswith('win_amd64') else metadata) / 'README.md')
    rows = []
    for path in sorted(package.rglob('*')):
        if path.is_file():
            digest = base64.urlsafe_b64encode(hashlib.sha256(path.read_bytes()).digest()).decode().rstrip('=')
            rows.append(f'{path.relative_to(package)},sha256={digest},{path.stat().st_size}')
    rows.append(f'{metadata.name}/RECORD,,')
    (metadata / 'RECORD').write_text('\n'.join(rows) + '\n')
    wheel.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(wheel, 'w', zipfile.ZIP_DEFLATED) as archive:
        for path in sorted(package.rglob('*')):
            if path.is_file(): archive.write(path, path.relative_to(package))
    from release_notices import bundle
    from release_notice_packages import wheel_notices
    target = ('windows-x86_64' if tag.endswith('win_amd64') else 'macos-arm64' if 'macosx' in tag
              else 'linux-arm64' if tag.endswith('aarch64') else 'linux-x86_64')
    wheel_notices(wheel, notices if notices is not None else bundle(target, 'python', root))
    return wheel


