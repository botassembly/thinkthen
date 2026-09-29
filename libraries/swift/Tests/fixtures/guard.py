"""Inspect shipping bytes, including compressed members, for planted secrets/paths."""
import pathlib,sys,tarfile,zipfile,gzip
BAD=[b'tt-canary-294',b'/home/', b'/Users/',b'auth.json',b'-----BEGIN PRIVATE KEY-----']
def inspect(label,data):
    for token in BAD:
        if token in data: raise ValueError('rejected private byte pattern in '+label)
def check(path):
    inspect(str(path),path.read_bytes())
    if tarfile.is_tarfile(path):
        with tarfile.open(path) as a:
            for m in a:
                inspect(m.name,m.name.encode())
                if m.isfile(): inspect(m.name,a.extractfile(m).read())
    elif zipfile.is_zipfile(path):
        with zipfile.ZipFile(path) as a:
            for n in a.namelist(): inspect(n,a.read(n))
    elif path.suffix=='.gz': inspect(str(path),gzip.decompress(path.read_bytes()))
try:
    for item in sys.argv[1:]: check(pathlib.Path(item))
except ValueError as exc: print(exc);sys.exit(1)
print('distribution bytes free of private patterns')
