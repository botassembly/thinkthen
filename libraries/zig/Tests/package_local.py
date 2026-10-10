"""Package Zig sources with an unchanged selected prebuilt native C archive."""
import os
import re
from pathlib import Path
import tarfile

package = Path(__file__).resolve().parent.parent
artifacts = package / 'target/artifacts'
artifacts.mkdir(parents=True, exist_ok=True)
selected = Path(os.environ['THINKTHEN_C_ARTIFACT']).resolve(strict=True)
version = re.search(r'\.version = "([^"]+)"', (package / 'build.zig.zon').read_text())[1]
members = ['LICENSE', 'README.md', 'build.zig', 'build.zig.zon', 'examples/decide.zig',
           'src/thinkthen.zig', 'src/session.zig', 'src/authored.zig', 'src/request_generated.zig', 'src/plan_generated.zig']
with tarfile.open(artifacts / f'thinkthen-zig-{version}-x86_64-unknown-linux-gnu.tar.gz', 'w:gz', compresslevel=1) as archive:
    for name in members:
        archive.add(package / name, './' + name)
    with tarfile.open(selected) as native:
        for name in ('./include/thinkthen.h', './lib/libthinkthen.a'):
            info = native.getmember(name)
            content = native.extractfile(info)
            info.name = './native/x86_64-unknown-linux-gnu/' + name.removeprefix('./')
            archive.addfile(info, content)
print('Zig local archive prepared from selected prebuilt native engine')
