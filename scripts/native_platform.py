"""The native build aliases for the supported release hosts."""
from pathlib import Path
import platform
import sys

HOSTS = {
    ('Darwin', 'arm64'): ('macos-arm64', 'osx-arm64', 'macos-arm64', 'aarch64-apple-darwin', 'arm64-darwin', 'dylib'),
    ('Linux', 'x86_64'): ('linux-x86_64', 'linux-x64', 'linux-amd64', 'x86_64-unknown-linux-gnu', 'x86_64-linux', 'so'),
    ('Linux', 'aarch64'): ('linux-arm64', 'linux-arm64', 'linux-aarch64', 'aarch64-unknown-linux-gnu', 'aarch64-linux', 'so'),
}
FIELDS = ('platform', 'rid', 'jvm', 'rust', 'ruby', 'suffix')
ARM_BASE = 'quay.io/pypa/manylinux_2_28_aarch64@sha256:270a5e59af09f02dfc07fe4bb2cb2c5625c76d085cfa47f87dd93d9026eacc75'


def publish_jvm_aliases(cache):
    """Publish JVM names only for available assets, refusing conflicting links."""
    cache = Path(cache)
    for _, rid, jvm, _, _, suffix in HOSTS.values():
        if not (cache / rid / ('libthinkthen_ffi.' + suffix)).is_file():
            continue
        alias = cache / 'jvm' / jvm
        alias.parent.mkdir(parents=True, exist_ok=True)
        expected = Path('..') / rid
        try:
            alias.symlink_to(expected, target_is_directory=True)
        except FileExistsError:
            # A concurrent publisher may have installed the same link.
            if not alias.is_symlink() or alias.readlink() != expected:
                raise ValueError('JVM native alias conflicts with its platform: ' + str(alias))


def current():
    return dict(zip(FIELDS, HOSTS[(platform.system(), platform.machine())]))


if __name__ == '__main__':
    if len(sys.argv) != 2 or sys.argv[1] not in FIELDS:
        sys.exit('usage: native_platform.py ' + '|'.join(FIELDS))
    print(current()[sys.argv[1]])
