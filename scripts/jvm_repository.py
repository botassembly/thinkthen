"""Own Maven's writable artifacts and check the engine selected by consumers."""
import hashlib
from pathlib import Path
import shutil
import sys
import zipfile


def seed(prepared, destination):
    """Copy dependencies, never links or artifacts installed by another tree."""
    artifacts = {'thinkthen-jvm-parent', 'thinkthen-java', 'thinkthen-kotlin', 'thinkthen-consumer'}
    def ignore(directory, names):
        if Path(directory).relative_to(prepared).as_posix() == 'io/github/botassembly':
            return artifacts.intersection(names)
        return ()
    shutil.copytree(prepared, destination, ignore=ignore)
    return destination


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def verify(root, jars, resources, native_platform, native_input):
    """Require consumer JAR bytes and embedded native bytes from this build."""
    root, resources = Path(root), Path(resources)
    for name, folder in [('java', 'sdk/java/library'), ('kotlin', 'sdk/kotlin')]:
        built, = (root / folder / 'target').glob(f'thinkthen-{name}-*.jar')
        selected, = [Path(p) for p in jars if Path(p).name == built.name]
        assert digest(selected) == digest(built), f'JVM artifact differs from this build: {selected}'
        if name == 'java':
            with zipfile.ZipFile(selected) as jar:
                expected = {p.relative_to(resources).as_posix(): digest(p) for p in resources.rglob('*') if p.is_file()}
                actual = {n.removeprefix('io/thinkthen/native/'): hashlib.sha256(jar.read(n)).hexdigest()
                          for n in jar.namelist() if n.startswith('io/thinkthen/native/') and not n.endswith('/')}
                expected[native_platform + '/' + Path(native_input).name] = digest(native_input)
                assert expected and actual == expected, f'JVM native resources differ from selected inputs: {selected}'
                for asset, value in sorted(actual.items()):
                    print(f'JVM native verified: {asset} {value}')
        print(f'JVM artifact verified: {selected.name} {digest(selected)}')


if __name__ == '__main__':
    if sys.argv[1] == 'seed':
        seed(Path(sys.argv[2]), Path(sys.argv[3]))
    elif sys.argv[1] == 'verify':
        verify(sys.argv[2], Path(sys.argv[3]).read_text().strip().split(':'), sys.argv[4], sys.argv[5], sys.argv[6])
    else:
        raise SystemExit('Usage: jvm_repository.py seed PREPARED DESTINATION | verify ROOT CLASSPATH RESOURCES PLATFORM NATIVE_INPUT')
