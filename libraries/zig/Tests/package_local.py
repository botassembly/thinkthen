"""Prepare disposable source and matching native archives from this checkout."""
from pathlib import Path
import tarfile

package = Path(__file__).resolve().parent.parent
artifacts = package / "target/artifacts"
artifacts.mkdir(parents=True, exist_ok=True)
members = ["LICENSE", "README.md", "build.zig", "build.zig.zon", "examples/decide.zig", "src/thinkthen.zig"]
with tarfile.open(artifacts / "thinkthen-zig-0.0.1-src.tar.gz", "w:gz") as archive:
    for name in members:
        assert (package / name).is_file(), name
        archive.add(package / name, name)
with tarfile.open(artifacts / "thinkthen-c-0.0.1-x86_64-linux-gnu.tar.gz", "w:gz") as archive:
    archive.add(package / "target/native/include/thinkthen.h", "include/thinkthen.h")
    for name in ("libthinkthen.so", "libthinkthen.so.0", "libthinkthen.a"):
        archive.add(package / "target/native/lib" / name, "lib/" + name)
print("Zig local source/native archives prepared")
