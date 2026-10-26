"""Keep Linux baseline Cargo files separate from host compiler output."""
import os
from pathlib import Path
import subprocess


def floor_target(root, target=None):
    selected = target or os.environ.get('CARGO_TARGET_DIR')
    directory = Path(selected or Path(root) / 'target').resolve()
    if selected and os.environ.get('THINKTHEN_FLOOR_BUILD') == '1':
        return directory
    return directory / 'runtime-floor'


def container_labels(root):
    try:
        common = subprocess.check_output(
            ['git', '-C', str(root), 'rev-parse', '--git-common-dir'],
            text=True, stderr=subprocess.DEVNULL).strip()
        repo = Path(os.path.abspath(Path(root) / common)).parent.name
    except (OSError, subprocess.CalledProcessError):
        repo = Path(root).resolve().name
    ticket = (os.environ.get('PM_JOB', '').split() or ['local'])[0]
    return ['--label', 'pm.repo=' + repo, '--label', 'pm.ticket=' + ticket]


if __name__ == '__main__':
    # Shell callers keep their arguments and quote this single repo label.
    print(container_labels(Path(__file__).resolve().parent.parent)[1])
