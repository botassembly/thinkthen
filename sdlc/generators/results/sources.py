"""Pure ownership and source classification shared by size and policy gates."""
from pathlib import Path as FilePath, PurePosixPath as Path
import os
import subprocess
from registry import OUTPUTS, generated_paths

EXTENSIONS = {'.rs', '.py', '.pyi', '.js', '.mjs', '.cjs', '.ts', '.mts', '.cts', '.rb', '.R', '.c', '.h', '.cpp',
              '.hpp', '.cc', '.cs', '.go', '.java', '.kt', '.scala', '.swift', '.zig',
              '.php', '.dart', '.ads', '.adb', '.cob', '.cbl', '.cpy', '.m'}
# These declarations come from real tool invocations rather than the schema generator.
TOOL_GENERATED = {'libraries/r/thinkthen/R/extendr-wrappers.R',
                  'libraries/r/thinkthen/src/entrypoint.c',
                  'libraries/ruby/fallback/thinkthen.h',
                  'libraries/dart/flutter/example/linux/flutter/generated_plugin_registrant.cc',
                  'libraries/dart/flutter/example/linux/flutter/generated_plugin_registrant.h',
                  'libraries/dart/flutter/example/windows/flutter/generated_plugin_registrant.cc',
                  'libraries/dart/flutter/example/windows/flutter/generated_plugin_registrant.h'}

# Artifact qualification scripts exercise installed packages rather than ship with them.
TEST_FILES = {'libraries/python/check-windows.py'}


def source_owners(manifest):
    return {**manifest['libraries'], **manifest.get('databases', {})}


def owner(path, manifest, outputs):
    owners = source_owners(manifest)
    for name, config in owners.items():
        if path.startswith(config['root'] + '/'):
            return name
    target = outputs.get(path)
    if target in owners:
        return target
    return None


def classification(path, name, manifest, generated):
    if path in generated or path in TOOL_GENERATED:
        return 'generated'
    if Path(path).suffix not in EXTENSIONS:
        return None
    config = source_owners(manifest)[name]
    parts = Path(path).parts
    relative = str(Path(path).relative_to(config['root']))
    if (path in TEST_FILES or relative in config.get('test_files', []) or Path(path).name == 'tests.rs'
            or any(folder in parts for folder in config['test_folders'])):
        return 'test'
    return 'handwritten'


def output_owners():
    """Include generated side outputs outside each library's source root."""
    return {output.path: output.target for output in OUTPUTS}


def production(path, manifest):
    """Identify handwritten production source using the shared conventions."""
    name = owner(path, manifest, output_owners())
    return bool(name and classification(path, name, manifest, generated_paths()) == 'handwritten')


def tracked_files(root):
    """List indexed files under a checkout directory, including staged additions."""
    root = FilePath(root).resolve()
    top = FilePath(subprocess.check_output(
        ['git', '-C', str(root), 'rev-parse', '--show-toplevel'], text=True).strip())
    names = subprocess.check_output(
        ['git', '-C', str(root), 'ls-files', '-z', '--full-name', '--', '.'])
    return sorted({top / os.fsdecode(name) for name in names.split(b'\0') if name})
