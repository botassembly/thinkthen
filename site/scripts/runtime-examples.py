"""Execute existing examples using the copied, verified runtime closure only."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys


ROOT = Path(__file__).resolve().parents[2]


def require(condition, message):
    if not condition:
        raise ValueError(message)


def inventory(root, allowed):
    files, links = {}, {}
    for directory, folders, names in os.walk(root, followlinks=False):
        for name in folders[:] + names:
            path = Path(directory) / name
            relative = path.relative_to(root).as_posix()
            if path.is_symlink():
                links[relative] = os.readlink(path)
                require(path.exists(), 'Copied closure has an unresolved link: ' + relative)
                require(any(path.resolve().is_relative_to(base) for base in allowed),
                        'Copied closure link leaves its owned roots: ' + relative)
                if name in folders:
                    folders.remove(name)
            elif path.is_file() and relative != 'manifest.json':
                with path.open('rb') as stream:
                    files[relative] = hashlib.file_digest(stream, 'sha256').hexdigest()
    return files, links


def execute():
    mode, *ids = sys.argv[1:]
    require(mode in {'routine', 'capture', 'replay'}, 'Use a runtime execution mode')
    bundle = Path(os.environ['SITE_RUNTIME_BUNDLE'])
    require(bundle.is_absolute() and not bundle.resolve().is_relative_to(ROOT),
            'Use an absolute runtime bundle outside the checkout')
    manifest = json.loads((bundle / 'manifest.json').read_text())
    packages, prepared = Path(manifest['packages']), Path(manifest['prepared'])
    for path in (packages, prepared):
        require(path.is_absolute() and not path.resolve().is_relative_to(ROOT),
                'Use copied packages and prepared examples outside the checkout')
    allowed = tuple(path.resolve() for path in (bundle, packages, prepared))
    require(inventory(bundle, allowed) == (manifest['files'], manifest['links']),
            'Copied runtime bytes or links differ')
    require(inventory(packages, allowed) == (manifest['packageFiles'], manifest['packageLinks']),
            'Copied package bytes or links differ')
    require((packages / 'source-commit').read_text().strip() == manifest['packageSource'],
            'Copied package source differs')
    require((prepared / 'source-commit').read_text().strip() == manifest['preparedSource'],
            'Copied prepared source differs')
    unchanged = subprocess.run(['git', 'diff', '--quiet', manifest['source'], '--', '.',
        *[':(exclude)site/examples/**/' + name for name in
          ('output.txt', 'stderr.txt', 'exit', 'ran', 'capture.json', 'unverified')]], cwd=ROOT)
    require(unchanged.returncode == 0, 'Runtime bundle source inputs differ')
    environment = json.loads((bundle / 'environment.json').read_text())
    allowed = {'SITE_PACKAGES', 'SITE_PREPARED_EXAMPLES', 'SITE_BIN_DIR',
               'THINKTHEN_BUILD_TOOLS', 'DOTNET_ROOT', 'RUBYLIB', 'PATH', 'LD_LIBRARY_PATH'}
    require(set(environment) == allowed and all(isinstance(value, str) for value in environment.values()),
            'Unexpected recorded runtime environment')
    require(environment['SITE_PACKAGES'] == str(packages)
            and environment['SITE_PREPARED_EXAMPLES'] == str(prepared),
            'Recorded closure paths differ')
    # Credentials and capture budgets stay inherited; the bundle contains neither.
    runtime = manifest['roots']['node']
    node = Path(manifest['mapped']['node']) / Path(runtime['executable']).relative_to(runtime['root'])
    arguments = [str(node), 'scripts/run-examples.mjs']
    if mode != 'routine':
        arguments.append('--' + mode)
    arguments.extend(ids)
    os.chdir(ROOT / 'site')
    os.execve(node, arguments, {**os.environ, **environment})


if __name__ == '__main__':
    try:
        execute()
    except (ValueError, KeyError, OSError) as error:
        print(str(error), file=sys.stderr)
        sys.exit(1)
