"""Stage a Linux Go module with its supplied prebuilt static native artifact."""
from pathlib import Path
import shutil
import sys

source, native, output = map(Path, sys.argv[1:])
if output.exists():
    raise SystemExit('package output already exists')
output.mkdir(parents=True)
for path in source.iterdir():
    if path.suffix == '.go' and not path.name.endswith('_test.go') or path.name in ('go.mod', 'README.md', 'LICENSE'):
        shutil.copy2(path, output / path.name)
prefix = output / 'native/x86_64-unknown-linux-gnu'
(prefix / 'include').mkdir(parents=True)
(prefix / 'lib').mkdir()
shutil.copy2(native / 'include/thinkthen.h', prefix / 'include/thinkthen.h')
shutil.copy2(native / 'lib/libthinkthen.a', prefix / 'lib/libthinkthen.a')
