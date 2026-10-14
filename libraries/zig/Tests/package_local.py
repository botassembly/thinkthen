"""Package Zig sources with an unchanged selected prebuilt native C archive."""
import os
import re
from pathlib import Path, PurePosixPath
import tarfile

package = Path(__file__).resolve().parent.parent
artifacts = package / 'target/artifacts'
artifacts.mkdir(parents=True, exist_ok=True)
selected = Path(os.environ['THINKTHEN_C_ARTIFACT']).resolve(strict=True)
version = re.search(r'\.version = "([^"]+)"', (package / 'build.zig.zon').read_text())[1]
members = ['LICENSE', 'README.md', 'build.zig', 'build.zig.zon', 'examples/decide.zig',
           'src/blocking.zig', 'src/native.zig', 'src/results_generated.zig', 'src/calls_generated.zig', 'src/thinkthen.zig', 'src/session.zig', 'src/authored.zig', 'src/request_generated.zig', 'src/plan_generated.zig']
with tarfile.open(artifacts / f'thinkthen-zig-{version}-x86_64-unknown-linux-gnu.tar.gz', 'w:gz', compresslevel=1) as archive:
    for name in members:
        archive.add(package / name, './' + name)
    with tarfile.open(selected) as native:
        names = [info.name.removeprefix('./') for info in native.getmembers()]
        if len(names) != len(set(names)) or any(PurePosixPath(name).is_absolute() or '..' in PurePosixPath(name).parts for name in names):
            raise ValueError('native archive requires unique relative member paths')
        for name in ('./include/thinkthen.h', './lib/libthinkthen.a'):
            info = native.getmember(name if name in native.getnames() else name.removeprefix("./"))
            if not info.isfile() or names.count(name.removeprefix('./')) != 1:
                raise ValueError('native archive requires exactly one regular header and static library')
            content = native.extractfile(info)
            info.name = './native/x86_64-unknown-linux-gnu/' + name.removeprefix('./')
            archive.addfile(info, content)
print('Zig local archive prepared from selected prebuilt native engine')
