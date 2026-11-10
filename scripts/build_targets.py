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


def repository_label(root):
    try:
        common = subprocess.check_output(
            ['git', '-C', str(root), 'rev-parse', '--git-common-dir'],
            text=True, stderr=subprocess.DEVNULL).strip()
        repo = Path(os.path.abspath(Path(root) / common)).parent.name
    except (OSError, subprocess.CalledProcessError):
        repo = Path(root).resolve().name
    return 'pm.repo=' + repo


def container_labels(root, *, image_build=False):
    label_file = os.environ.get('JOB_LABEL_FILE')
    if label_file:
        path = Path(label_file).resolve(strict=True)
        if not image_build:
            return ['--label-file', str(path)]
        # Docker image builds accept individual labels, not --label-file.
        labels = [line.strip() for line in path.read_text().splitlines()
                  if line.strip() and not line.lstrip().startswith('#')]
        return [argument for label in labels for argument in ('--label', label)]
    ticket = (os.environ.get('PM_JOB', '').split() or ['local'])[0]
    return ['--label', repository_label(root), '--label', 'pm.ticket=' + ticket]


if __name__ == '__main__':
    # Shell callers keep their arguments and quote this single repo label.
    print(repository_label(Path(__file__).resolve().parent.parent))
