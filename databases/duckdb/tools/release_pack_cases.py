"""Exercise the real reuse packer in an owned scratch source tree, never the lane."""
from __future__ import annotations

import os
import hashlib
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

from inputs import canonical, selected, versions

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
    names = ['Cargo.toml', 'Cargo.lock', 'sdlc/scripts/release-pack', 'sdlc/scripts/scratch.sh', 'sdlc/scripts/release-archive-tree.py',
             'crates/thinkthen/Cargo.toml', 'LICENSE', 'conformance/children/children.py']
    names += [str(p.relative_to(REPO)) for folder in ('databases/duckdb/tools', 'databases/duckdb/cpp')
              for p in (REPO / folder).iterdir() if p.is_file() and p.suffix in ('.py', '.sh', '.env', '.txt')]
    for folder in ('crates/thinkthen', 'conformance/backend', 'databases/duckdb/bridge'):
        for path in (REPO / folder).rglob('*'):
            if path.is_file() and 'target' not in path.relative_to(REPO / folder).parts:
                names.append(str(path.relative_to(REPO)))
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


def published_consumer(archive: Path, target: str, release: str, root: Path) -> None:
    import tarfile
    sys.path.insert(0, str(REPO / 'sdlc/scripts'))
    from install_check import Check, Failure, QUESTION, check_result
    from install_check_channels import duckdb
    from harness import Backend
    if target != 'x86_64-unknown-linux-gnu':
        return  # The published-install channel has only this native workflow cell.
    with Backend() as backend:
        for missing in (False, True):
            own = root / ('consumer-missing' if missing else 'consumer-good')
            own.mkdir()
            check = Check(own, 'duckdb', release)
            check.sample.mkdir(parents=True)
            sample = REPO / 'demos/27-test-with-no-network'
            shutil.copytree(sample / 'recording', check.sample / 'recording')
            shutil.copyfile(sample / 'report.txt', check.sample / 'report.txt')
            (check.sample / 'question.txt').write_text(QUESTION)
            tools = Path(os.environ['THINKTHEN_TOOLCHAINS']) / 'duckdb' / versions()[0]
            check.env['PATH'] = str(tools) + os.pathsep + check.env['PATH']
            check.env.update(THINKTHEN_BASE_URL=backend.base(), THINKTHEN_API_KEY='sk-loopback-package-consumer')
            recording = check.sample / 'recording/thinkthen.jsonl'
            entries = [json.loads(line) for line in recording.read_text().splitlines()]
            states = {item['sha256']: item['state'] for item in entries if 'sha256' in item}
            for item in entries:
                if 'key' not in item:
                    continue
                def key(url):
                    parts = ['systemone', url, json.dumps(item['model']), states[item['state']], item['question']]
                    return hashlib.sha256('\n'.join(parts).encode()).hexdigest()
                assert item['key'] == key(item['url']), 'saved recording identity differs'
                item['url'] = backend.base() + '/systemone'
                item['key'] = key(item['url'])
            recording.write_text('\n'.join(json.dumps(item, separators=(',', ':')) for item in entries) + '\n')
            local = archive
            if missing:
                local = own / 'missing.tar.gz'
                wanted = f'{versions()[0]}/{selected(versions()[0], target)["platform"]}/thinkthen.duckdb_extension'
                with tarfile.open(archive) as original, tarfile.open(local, 'w:gz') as output:
                    for entry in original:
                        if entry.name.removeprefix('./') != wanted:
                            output.addfile(entry, original.extractfile(entry) if entry.isfile() else None)
            check.release = lambda name: local
            before = backend.count()
            try:
                installed, reply, _ = duckdb(check)
            except Failure as error:
                assert missing and wanted in str(error), str(error)
            else:
                assert not missing, 'missing repository member was rescued'
                check_result(release, installed, reply)
            assert backend.count() == before, 'published replay sent a request'
    print('ok   published consumer loads real repository bytes and refuses a missing matching member')


def main(target: str) -> None:
    artifact = canonical(versions()[0], target)
    if not artifact.is_file():
        raise SystemExit(f'the built {target} canonical artifact is missing')
    originals = {version: canonical(version, target).read_bytes() for version in versions()}
    data = originals[versions()[0]]
    reject_lane_cleanup()
    with tempfile.TemporaryDirectory(prefix='thinkthen-reuse-') as folder:
        root = Path(folder)
        tree = root / 'source'
        copy_packer(tree)
        member = tree / artifact.relative_to(REPO)
        member.parent.mkdir(parents=True, exist_ok=True)
        for version, contents in originals.items():
            path = tree / canonical(version, target).relative_to(REPO)
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(contents)
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
        member.write_bytes(data)
        older = tree / canonical(versions()[1], target).relative_to(REPO)
        older.rename(root / 'older-backup')
        result = pack(tree, target, root / 'missing-older', env)
        assert result.returncode == 1 and f'{older.relative_to(tree)} is missing' in result.stderr, result.stderr[:500]
        assert not list((root / 'missing-older').glob('*.tar.gz*'))
        (root / 'older-backup').rename(older)
        at = len(originals[versions()[1]]) - 534 + 22 + 32 * 5
        older.write_bytes(originals[versions()[1]][:at] + versions()[0].encode().ljust(32, b'\0') + originals[versions()[1]][at + 32:])
        result = pack(tree, target, root / 'swapped-older', env)
        assert result.returncode == 1 and 'DuckDB footer DuckDB version is' in result.stderr, result.stderr[:500]
        assert not list((root / 'swapped-older').glob('*.tar.gz*'))
        older.write_bytes(originals[versions()[1]])
        good = root / 'good'
        result = pack(tree, target, good, env)
        assert result.returncode == 0, result.stderr[-1200:]
        archive, = good.glob('*.tar.gz')
        sys.path.insert(0, str(REPO / 'databases/duckdb/cpp'))
        from verify_repository import verify_repository
        release = next(line.split('"')[1] for line in (REPO / 'crates/thinkthen/Cargo.toml').read_text().splitlines() if line.startswith('version = '))
        verify_repository(archive, target, release)
        published_consumer(archive, target, release, root)
        print('ok   reuse packs both real versions and their legal inventories')
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
    assert all(canonical(v, target).read_bytes() == contents for v, contents in originals.items()), 'the lane artifact changed during reuse probes'


if __name__ == '__main__':
    if len(sys.argv) != 2 or sys.argv[1] not in WRONG_PLATFORM:
        raise SystemExit('usage: release_pack_cases.py RUST_TARGET')
    main(sys.argv[1])
