"""Run shell fixtures against the complete tracked checkout."""
import json
import subprocess
import tarfile

from conformance.children.children import child_env


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
    groups = [group for tool in manifest['tools'].values() for group in tool['groups']]
    return ','.join(dict.fromkeys(('test', 'surfaces', *groups, *manifest['libraries'], *manifest['databases'])))


def stub_other_surfaces(root, selected):
    """Keep the real registry while isolating unrelated consumer processes."""
    for line in (root / 'sdlc/surfaces.txt').read_text().splitlines():
        fields = line.split()
        if fields and fields[0].startswith('#'):
            continue
        if len(fields) > 1 and fields[1] in ('landed', 'feature') and fields[0] not in selected:
            (root / fields[0] / 'check.sh').write_text('if [ "${THINKTHEN_TEST_PROFILE:-}" = smoke ]; then printf "smoke: true\\n"; fi\nexit 0\n')
