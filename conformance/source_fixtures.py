"""Stage the real source checks needed by thin installed-script fixtures."""
import json
import shutil
import subprocess
import tarfile
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'sdlc/generators/results'))
from sources import tracked_files

from sdlc.generators.results.registry import OUTPUTS
from conformance.children.children import child_env


def stage_files(source, root, paths):
    for relative in paths:
        destination = root / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source / relative, destination)


def stage_generated_source(source, root, target):
    generator = 'sdlc/generators/results/'
    stage_files(source, root, (
        # Stage the producer and its dependency graph from the canonical tree.
        # A new shared renderer import must not require another fixture copy list.
        *(path.relative_to(source).as_posix()
          for path in tracked_files(source / generator) if path.suffix == '.py'),
        'conformance/children/children.py',
        'crates/thinkthen/src/public/results/complete.schema.json',
        'specification/request.schema.json',
        'rust-toolchain.toml',
        *(row.path for row in OUTPUTS if row.target == target),
    ))


def stage_checkout(source, root):
    """Extract HEAD so new sourced helpers need no fixture file list."""
    env = child_env(home=root / 'fixture-git-home')
    archive = subprocess.Popen(['git', 'archive', 'HEAD'], cwd=source, env=env, stdout=subprocess.PIPE)
    try:
        with tarfile.open(fileobj=archive.stdout, mode='r|') as packed:
            packed.extractall(root, filter='data')
    finally:
        archive.stdout.close()
        if archive.wait() != 0:
            raise RuntimeError('cannot archive tracked fixture checkout')
    subprocess.run(['git', 'init', '-q', str(root)], env=env, check=True)
    subprocess.run(['git', '-C', str(root), 'add', '--force', '--all'], env=env, check=True)


def fixture_tool_groups(source):
    """Let controlled tool stubs serve every declared library and rung."""
    manifest = json.loads((source / 'sdlc/tools.json').read_text())
    return ','.join(dict.fromkeys(('test', 'surfaces', *manifest['libraries'], *manifest['databases'])))
