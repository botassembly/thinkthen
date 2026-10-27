"""Exercise the actual registry merge with installed package files in platform tarballs."""
import io
from pathlib import Path
import sys
import tarfile
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / 'scripts'))
from release_registry import node

installed = Path(sys.argv[1])
files = {str(path.relative_to(installed)): path.read_bytes()
         for path in installed.rglob('*') if path.is_file()}
for name, alteration in [('bridge.js', 'change'), ('stream.js', 'change'),
                         ('contract.js', 'change'), ('errors.js', 'change'),
                         ('errors.d.ts', 'change'), ('unexpected.txt', 'add'),
                         ('bridge.js', 'remove')]:
    with tempfile.TemporaryDirectory(prefix='thinkthen-merge-') as temporary:
        root = Path(temporary)
        packages = {}
        for target in ('linux-x86_64', 'linux-arm64'):
            package = root / target
            (package / 'typescript').mkdir(parents=True)
            content = dict(files)
            if target == 'linux-arm64':
                if alteration == 'remove':
                    del content[name]
                else:
                    content[name] = content.get(name, b'') + b'\nchanged\n'
            with tarfile.open(package / 'typescript/thinkthen.tgz', 'w:gz') as archive:
                for filename, data in content.items():
                    info = tarfile.TarInfo('package/' + filename)
                    info.size = len(data)
                    archive.addfile(info, io.BytesIO(data))
            packages[target] = package
        scratch = root / 'scratch'
        scratch.mkdir()
        try:
            node(packages, root / 'destination', scratch)
        except ValueError as error:
            assert str(error) == f'npm {name} differs between platforms', str(error)
        else:
            raise AssertionError(f'merge accepted {alteration}: {name}')
