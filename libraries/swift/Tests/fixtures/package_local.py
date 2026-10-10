"""Build disposable source and native archives from the checked-out package."""
from pathlib import Path
import platform
import tarfile
import zipfile

package = Path(__file__).resolve().parents[2]
artifacts = package / "target/artifacts"
artifacts.mkdir(parents=True, exist_ok=True)
members = [package / name for name in ("LICENSE", "README.md", "Package.swift", "Examples/main.swift",
    "Sources/CThinkThen/include/thinkthen.h", "Sources/CThinkThen/include/loader.h", "Sources/CThinkThen/module.modulemap", "Sources/CThinkThen/bridge.c",
    "Sources/ThinkThen/ThinkThen.swift", "Sources/ThinkThen/Complete.swift", "Tests/TypeCase/main.swift")]
members.extend(sorted(path for path in (package / "Sources/ThinkThen").glob("*.swift") if path not in members))
triple = {'x86_64': 'x86_64-unknown-linux-gnu', 'aarch64': 'aarch64-unknown-linux-gnu'}[platform.machine()]
members.extend(sorted(path for path in (package / 'Sources/ThinkThen/Native').rglob('*') if path.is_file()))
assert all(path.is_file() for path in members)
with zipfile.ZipFile(artifacts / "thinkthen-swift-0.0.1.zip", "w") as archive:
    for path in members:
        archive.write(path, "thinkthen-swift-0.0.1/" + str(path.relative_to(package)))
with tarfile.open(artifacts / "thinkthen-c-0.0.1-x86_64-linux-gnu.tar.gz", "w:gz", dereference=True) as archive:
    archive.add(package / "Sources/CThinkThen/include/thinkthen.h", "include/thinkthen.h")
    for name in ("libthinkthen.so", "libthinkthen.so.0"):
        archive.add(package / "target/native/lib" / name, "lib/" + name)
print("Swift local source/native archives prepared")
