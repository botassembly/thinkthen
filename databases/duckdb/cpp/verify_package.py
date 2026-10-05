"""Load real selected C++ bytes in matching stock hosts and pin version refusals."""
from __future__ import annotations

import argparse
import hashlib
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'tools'))
from harness import child_env  # noqa: E402
from inputs import selected, versions  # noqa: E402
from verify_footer import verify  # noqa: E402


def python_load(path: Path, folder: Path, version: str, repository: bool = False) -> subprocess.CompletedProcess[str]:
    folder.mkdir(parents=True)
    code = (
        "import duckdb,sys; assert 'v'+duckdb.__version__ == sys.argv[2]; "
        "con=duckdb.connect(config={'allow_unsigned_extensions':'true', 'extension_directory':sys.argv[3]}); "
        + ("con.execute('INSTALL thinkthen FROM ' + chr(39) + sys.argv[1] + chr(39)); con.execute('LOAD thinkthen')"
           if repository else "con.execute('LOAD ' + chr(39) + sys.argv[1] + chr(39))")
    )
    return subprocess.run([sys.executable, '-c', code, str(path), version, str(folder / 'extensions')],
                          env=child_env('http://127.0.0.1:1/v1', folder), text=True,
                          capture_output=True, timeout=30, check=False)


def cli_load(cli: Path, path: Path, folder: Path, repository: bool = False) -> subprocess.CompletedProcess[str]:
    folder.mkdir(parents=True)
    sql = f"SET extension_directory='{folder / 'extensions'}'; "
    sql += f"INSTALL thinkthen FROM '{path}'; LOAD thinkthen;" if repository else f"LOAD '{path}';"
    return subprocess.run([str(cli), '-unsigned', '-noheader', '-list', '-c', sql],
                          env=child_env('http://127.0.0.1:1/v1', folder), text=True,
                          capture_output=True, timeout=30, check=False)


def refusal(result: subprocess.CompletedProcess[str], built: str, running: str) -> None:
    sentence = (f"The file was built specifically for DuckDB version '{built}' and can only be loaded with "
                f"that version of DuckDB. (this version of DuckDB is '{running}')")
    assert result.returncode == 1, f'expected version refusal exit 1, got {result.returncode}: {result.stderr[:800]}'
    assert sentence in result.stderr, result.stderr[:800]


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument('--extension', required=True, type=Path)
    parser.add_argument('--version', required=True)
    parser.add_argument('--target', required=True)
    parser.add_argument('--matching-host', required=True, type=Path)
    parser.add_argument('--different-host', required=True, type=Path)
    parser.add_argument('--different-version', required=True)
    parser.add_argument('--repository-extension', type=Path)
    args = parser.parse_args()
    identity = selected(args.version, args.target)
    other = selected(args.different_version, args.target)
    assert args.version != args.different_version, 'refusal needs a different supported host'
    source = args.extension.resolve(strict=True)
    for cli, item in ((args.matching_host, identity), (args.different_host, other)):
        assert hashlib.sha256(cli.read_bytes()).hexdigest() == item['cli_sha256'], 'the stock host differs from its pin'
    release = next(line.split('"')[1] for line in (Path(__file__).resolve().parents[3] / 'crates/thinkthen/Cargo.toml').read_text().splitlines() if line.startswith('version = '))
    verify(source, args.target, args.version, release)
    with tempfile.TemporaryDirectory(prefix='thinkthen-duckdb-package-') as scratch:
        root = Path(scratch)
        good = root / 'matching/thinkthen.duckdb_extension'
        good.parent.mkdir()
        data = source.read_bytes()
        good.write_bytes(data)
        for result in (python_load(good, root / 'matching-python', args.version),
                       cli_load(args.matching_host, good, root / 'matching-cli')):
            assert result.returncode == 0, result.stderr[:800]
        # Fixed footer field 5. Do not search or rewrite the binary's code or strings.
        at = len(data) - 534 + 22 + 32 * 5
        assert data[at:at + 32].rstrip(b'\0') == args.version.encode(), 'selected footer version is absent'
        wrong = root / 'wrong/thinkthen.duckdb_extension'
        wrong.parent.mkdir()
        wrong.write_bytes(data[:at] + args.different_version.encode().ljust(32, b'\0') + data[at + 32:])
        refusal(python_load(wrong, root / 'wrong-python', args.version), args.different_version, args.version)
        refusal(cli_load(args.different_host, good, root / 'different-cli'), args.version, args.different_version)
        if args.repository_extension:
            verify(args.repository_extension, args.target, args.different_version, release)
            repository = root / 'repository'
            for version, artifact in ((args.version, source), (args.different_version, args.repository_extension)):
                member = repository / version / identity['platform'] / 'thinkthen.duckdb_extension'
                member.parent.mkdir(parents=True)
                shutil.copyfile(artifact, member)
            for result in (python_load(repository, root / 'repository-python', args.version, True),
                           cli_load(args.matching_host, repository, root / 'repository-cli', True)):
                assert result.returncode == 0, result.stderr[:800]
            if args.version != versions()[0]:
                missing = root / 'missing-repository' / args.different_version / identity['platform']
                missing.mkdir(parents=True)
                shutil.copyfile(args.repository_extension, missing / 'thinkthen.duckdb_extension')
                result = cli_load(args.matching_host, root / 'missing-repository', root / 'missing-host', True)
                assert result.returncode == 1, result.stderr[:800]
                assert f"{args.version}/{identity['platform']}/thinkthen.duckdb_extension" in result.stderr, result.stderr[:800]
        print(f'C++ {args.version} package loads in matching stock hosts and refuses stock {args.different_version}')


if __name__ == '__main__':
    main()
