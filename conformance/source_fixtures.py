"""Stage the real source checks needed by thin installed-script fixtures."""
import shutil

from sdlc.generators.results.registry import OUTPUTS


def stage_files(source, root, paths):
    for relative in paths:
        destination = root / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source / relative, destination)


def stage_generated_source(source, root, target):
    generator = 'sdlc/generators/results/'
    stage_files(source, root, (
        *(generator + name + '.py' for name in
          ('generate', 'registry', 'native', 'layouts', 'formatting')),
        generator + 'templates/' + target + '.py',
        *((generator + 'templates/host_answers.py',) if target in ('python', 'typescript') else ()),
        'conformance/children/children.py',
        'crates/thinkthen/src/public/results/complete.schema.json',
        'specification/request.schema.json',
        'rust-toolchain.toml',
        *(row.path for row in OUTPUTS if row.target == target),
    ))
