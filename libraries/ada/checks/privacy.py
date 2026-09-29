"""Reject private bytes in source and compressed shipping members."""
import gzip
from pathlib import Path
import tarfile
import tempfile
import zipfile

ROOT = Path(__file__).resolve().parents[1]
BAD = (b"/home/", b"/Users/", b"-----BEGIN PRIVATE KEY-----", b"auth.json", b"tt-canary-private")

def inspect(name, data):
    if any(marker in data for marker in BAD):
        raise ValueError(f"private byte pattern in {name}")

def check(path):
    inspect(str(path), path.read_bytes())
    if zipfile.is_zipfile(path):
        with zipfile.ZipFile(path) as archive:
            for name in archive.namelist():
                inspect(name, archive.read(name))
    elif tarfile.is_tarfile(path):
        with tarfile.open(path) as archive:
            for member in archive:
                inspect(member.name, member.name.encode())
                if member.isfile():
                    inspect(member.name, archive.extractfile(member).read())
    elif path.suffix == ".gz":
        inspect(str(path), gzip.decompress(path.read_bytes()))

sources = [ROOT / "README.md", ROOT / "LICENSE", *ROOT.glob("src/*"),
           *ROOT.glob("Src/*"), *ROOT.glob("Sources/*"), *ROOT.glob("examples/*"),
           *ROOT.glob("Examples/*")]
for path in sources:
    if path.is_file():
        check(path)
with tempfile.TemporaryDirectory() as name:
    folder = Path(name)
    for index, marker in enumerate((BAD[0], BAD[1])):
        path = folder / f"plant-{index}"
        path.write_bytes(marker)
        try:
            check(path)
        except ValueError:
            pass
        else:
            raise AssertionError(f"accepted planted marker {index}")
    zipped = folder / "plant.zip"
    with zipfile.ZipFile(zipped, "w") as archive:
        archive.writestr("marker.txt", BAD[1])
    try:
        check(zipped)
    except ValueError:
        pass
    else:
        raise AssertionError("accepted compressed marker")
print("source privacy and path/key/compressed plants passed")
