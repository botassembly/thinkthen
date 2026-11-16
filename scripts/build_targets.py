"""Keep Linux baseline Cargo files separate from host compiler output."""
import os
from pathlib import Path


def floor_target(root, target=None):
    selected = target or os.environ.get('CARGO_TARGET_DIR')
    directory = Path(selected or Path(root) / 'target').resolve()
    if selected and os.environ.get('THINKTHEN_FLOOR_BUILD') == '1':
        return directory
    return directory / 'runtime-floor'


def container_labels(_root, *, image_build=False):
    label_file = os.environ.get('JOB_LABEL_FILE')
    if label_file:
        path = Path(label_file).resolve(strict=True)
        if not image_build:
            return ['--label-file', str(path)]
        # Docker image builds accept individual labels, not --label-file.
        labels = [line.strip() for line in path.read_text().splitlines()
                  if line.strip() and not line.lstrip().startswith('#')]
        return [argument for label in labels for argument in ('--label', label)]
    return []
