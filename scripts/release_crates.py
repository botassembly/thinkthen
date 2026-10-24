"""Upload approved crate bytes using Cargo's documented registry web API."""
import json
import os
import struct
import tarfile
import time
import tomllib
import urllib.error
import urllib.request

# https://doc.rust-lang.org/cargo/reference/registry-web-api.html#publish

def metadata(archive):
    with tarfile.open(archive) as bundle:
        prefix = archive.stem
        manifest = tomllib.loads(bundle.extractfile(prefix + '/Cargo.toml').read().decode())
        package = manifest['package']
        result = {key.replace('-', '_'): package.get(key) for key in (
            'description', 'documentation', 'homepage', 'license', 'license-file', 'repository', 'links', 'rust-version')}
        result.update(name=package['name'], vers=package['version'],
                      authors=package.get('authors', []), keywords=package.get('keywords', []),
                      categories=package.get('categories', []), features=manifest.get('features', {}), deps=[])
        readme = package.get('readme')
        if isinstance(readme, str):
            result.update(readme=bundle.extractfile(prefix + '/' + readme).read().decode(), readme_file=readme)
        for target, group in [(None, manifest), *manifest.get('target', {}).items()]:
            for section, kind in (('dependencies', 'normal'), ('dev-dependencies', 'dev'), ('build-dependencies', 'build')):
                for alias, dependency in group.get(section, {}).items():
                    if isinstance(dependency, str):
                        dependency = {'version': dependency}
                    result['deps'].append(dict(name=dependency.get('package', alias), version_req=dependency['version'],
                        features=dependency.get('features', []), optional=dependency.get('optional', False),
                        default_features=dependency.get('default-features', True), target=target, kind=kind,
                        registry=dependency.get('registry-index'),
                        explicit_name_in_toml=alias if 'package' in dependency else None))
        return result


def upload(archive):
    info = archive.with_suffix('.json').read_bytes()
    package = archive.read_bytes()
    payload = struct.pack('<I', len(info)) + info + struct.pack('<I', len(package)) + package
    data = json.loads(info)
    download = f'https://crates.io/api/v1/crates/{data["name"]}/{data["vers"]}/download'
    from release_upload import existing
    present = existing(download, package)
    request = urllib.request.Request('https://crates.io/api/v1/crates/new', payload,
        {'Authorization': os.environ['CARGO_REGISTRY_TOKEN'], 'Accept': 'application/json',
         'Content-Type': 'application/octet-stream', 'User-Agent': 'ThinkThen release'}, method='PUT')
    if not present:
        try:
            with urllib.request.urlopen(request) as response:
                if json.load(response).get('errors'):
                    raise ValueError('crates.io rejected the package')
        except urllib.error.HTTPError:
            if not existing(download, package):
                raise
    name = data['name']
    path = f'{name[:2]}/{name[2:4]}/{name}'
    for attempt in range(60):
        try:
            with urllib.request.urlopen('https://index.crates.io/' + path) as response:
                if any(json.loads(line)['vers'] == data['vers'] for line in response):
                    print('crates.io: published', name)
                    return
        except urllib.error.HTTPError as error:
            if error.code != 404:
                raise
        time.sleep(1)
    raise ValueError('crates.io index did not expose the accepted version')
