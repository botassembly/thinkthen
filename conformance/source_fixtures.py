"""Run shell fixtures against the complete tracked checkout."""
import json
import subprocess
import tarfile


def stage_checkout(source, root):
    """Extract HEAD so new sourced helpers need no fixture file list."""
    archive = subprocess.Popen(['git', 'archive', 'HEAD'], cwd=source, stdout=subprocess.PIPE)
    try:
        with tarfile.open(fileobj=archive.stdout, mode='r|') as packed:
            packed.extractall(root, filter='data')
    finally:
        archive.stdout.close()
        if archive.wait() != 0:
            raise RuntimeError('cannot archive tracked fixture checkout')


def fixture_tool_groups(source):
    """Let controlled tool stubs serve every declared library and rung."""
    manifest = json.loads((source / 'sdlc/tools.json').read_text())
    return ','.join(dict.fromkeys(('test', 'surfaces', *manifest['libraries'], *manifest['databases'])))
