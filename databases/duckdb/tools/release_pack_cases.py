"""Exercise the real reuse packer in an owned scratch source tree, never the lane."""
from __future__ import annotations

import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

from inputs import canonical, versions

REPO = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(REPO / 'conformance/children'))
from children import CARGO, child_env  # noqa: E402

WRONG_PLATFORM = {
    'x86_64-unknown-linux-gnu': 'osx_amd64',
    'aarch64-unknown-linux-gnu': 'osx_arm64',
    'aarch64-apple-darwin': 'linux_arm64',
    'x86_64-apple-darwin': 'linux_amd64',
}


def copy_packer(destination: Path) -> None:
    names = ['sdlc/scripts/release-pack', 'sdlc/scripts/scratch.sh', 'sdlc/scripts/release-archive-tree.py',
             'crates/thinkthen/Cargo.toml', 'LICENSE', 'conformance/children/children.py']
    names += [str(p.relative_to(REPO)) for folder in ('databases/duckdb/tools', 'databases/duckdb/cpp')
              for p in (REPO / folder).iterdir() if p.is_file() and p.suffix in ('.py', '.sh', '.env', '.txt')]
    for name in names:
        target = destination / name
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(REPO / name, target)


def reject_lane_cleanup() -> None:
    result = subprocess.run(['/bin/sh', '-c', '. "$1"; scratch_remove "$2"', 'cleanup-plant',
                             str(REPO / 'sdlc/scripts/scratch.sh'), str(REPO)],
                            capture_output=True, text=True, check=False)
    assert result.returncode == 1 and 'which this run did not make with mktemp' in result.stderr


def pack(tree: Path, target: str, out: Path, env: dict[str, str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(['/bin/sh', str(tree / 'sdlc/scripts/release-pack'), '--reuse', target, str(out), 'duckdb'],
                          cwd=tree, capture_output=True, text=True, check=False, env=env, timeout=90)


def main(target: str) -> None:
    artifact = canonical(versions()[0], target)
    if not artifact.is_file():
        raise SystemExit(f'the built {target} canonical artifact is missing')
    data = artifact.read_bytes()
    reject_lane_cleanup()
    with tempfile.TemporaryDirectory(prefix='thinkthen-reuse-') as folder:
        root = Path(folder)
        tree = root / 'source'
        copy_packer(tree)
        member = tree / artifact.relative_to(REPO)
        member.parent.mkdir(parents=True)
        env = child_env(CARGO, RUSTC_WRAPPER='', CARGO_NET_OFFLINE='true',
                        THINKTHEN_TOOLCHAINS=os.environ.get('THINKTHEN_TOOLCHAINS', str(Path.home() / '.cache/thinkthen-toolchains')))
        for name, index, value in (('platform', 6, WRONG_PLATFORM[target]), ('DuckDB version', 5, versions()[1]),
                                    ('extension version', 4, '9.9.9'), ('ABI', 3, 'C'), ('ABI version', 7, '9')):
            at = len(data) - 534 + 22 + 32 * index
            member.write_bytes(data[:at] + value.encode().ljust(32, b'\0') + data[at + 32:])
            destination = root / name.replace(' ', '-')
            result = pack(tree, target, destination, env)
            assert result.returncode == 1, f'reuse wrong {name}: exit {result.returncode}: {result.stderr[:500]}'
            assert f'DuckDB footer {name} is' in result.stderr, result.stderr[:500]
            assert not list(destination.glob('*.tar.gz*')), f'reuse archived wrong {name}'
            print(f'ok   reuse refuses wrong {name}')
        obsolete = tree / 'databases/duckdb/build/artifacts/cpp' / target / 'thinkthen.duckdb_extension'
        obsolete.parent.mkdir(parents=True)
        obsolete.write_bytes(data)
        (tree / 'databases/duckdb/build/thinkthen.duckdb_extension').write_bytes(data)
        # Move the scratch-owned canonical member aside; retain both obsolete files.
        member.rename(root / 'canonical-backup')
        result = pack(tree, target, root / 'missing', env)
        assert result.returncode == 1 and f'{member.relative_to(tree)} is missing' in result.stderr, result.stderr[:500]
        assert not list((root / 'missing').glob('*.tar.gz*'))
        print('ok   reuse refuses missing canonical despite valid obsolete outputs')
    assert artifact.read_bytes() == data, 'the lane artifact changed during reuse probes'


if __name__ == '__main__':
    if len(sys.argv) != 2 or sys.argv[1] not in WRONG_PLATFORM:
        raise SystemExit('usage: release_pack_cases.py RUST_TARGET')
    main(sys.argv[1])
