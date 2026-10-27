"""Keep secret and word checks on owned files when the index has gitlinks."""
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from testing import child_environment

env = child_environment()
root = Path(__file__).resolve().parents[3]
pins = json.loads((root / 'databases/duckdb/community-versions.json').read_text())['1.4.3']
# A generated synthetic credential exercises the pinned upstream detector.
credential = 'ghp_' + 'aB3cD4eF5gH6iJ7kL8mN9oP0qR1sT2uV3wX4'
with tempfile.TemporaryDirectory(prefix='gitlink-scans-') as temporary:
    repo = Path(temporary)
    def git(*args):
        subprocess.run(['git', *args], cwd=repo, env=env, check=True, capture_output=True)
    def scan(command, succeeds):
        result = subprocess.run([str(repo / 'scripts' / command)], cwd=repo, env=env,
                                capture_output=True, text=True)
        assert (result.returncode == 0) == succeeds, (command, result.returncode)
    git('init', '-q')
    (repo / 'scripts').mkdir()
    for name in ('words', 'secrets'):
        shutil.copy2(root / 'scripts' / name, repo / 'scripts' / name)
    shutil.copy2(root / '.gitleaks.toml', repo / '.gitleaks.toml')
    shutil.copy2(root / '.gitmodules', repo / '.gitmodules')
    (repo / 'ordinary.txt').write_text('safe text\n')
    git('add', '.')
    for name, revision in pins.items():
        git('update-index', '--add', '--cacheinfo', f'160000,{revision},{name}')
        (repo / name).mkdir()
        (repo / name / 'upstream.txt').write_text(credential)
    # Feature: upstream gitlinks do not enter the current owned-file scans.
    scan('secrets', True)
    scan('words', True)
    # Ordinary untracked files and .gitmodules still enter the secret scan.
    untracked = repo / 'untracked.txt'
    untracked.write_text(credential)
    scan('secrets', False)
    untracked.unlink()
    modules = repo / '.gitmodules'
    modules.write_text(modules.read_text() + '\n# ' + credential)
    scan('secrets', False)
    shutil.copy2(root / '.gitmodules', modules)
    # The same directory loses its exemption when its contents become owned.
    git('rm', '--cached', '--force', 'duckdb')
    git('add', 'duckdb/upstream.txt')
    scan('secrets', False)
    (repo / 'ordinary.txt').unlink()
    scan('words', False)
print('PASS: scans skip indexed gitlinks, detect owned/untracked secrets and reject missing owned files')
