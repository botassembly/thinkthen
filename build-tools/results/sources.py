"""Pure ownership and source classification for native generation and policy gates."""
from pathlib import Path as FilePath, PurePosixPath as Path
import json
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


def load_manifest(path):
    """Load shared surface policy and each host's own migration entry."""
    path = FilePath(path)
    manifest = json.loads(path.read_text())
    for section in ('libraries', 'databases'):
        if section in manifest:
            raise ValueError('surface entries belong in ' + str(path.with_suffix('')))
        manifest[section] = {}
    folder = path.with_suffix('').resolve()
    if not folder.is_dir():
        raise ValueError('missing library surface entries: ' + str(folder))
    # Indexed entries remain required even if a local file disappears. Include
    # new entries too, so a local addition cannot bypass the same policy.
    entries = {entry for entry in tracked_files(folder) if entry.suffix == '.json'}
    entries.update(folder.glob('*.json'))
    for entry in sorted(entries):
        data = json.loads(entry.read_text())
        if (not isinstance(data, dict) or len(data) != 1
                or next(iter(data)) not in ('libraries', 'databases')):
            raise ValueError('invalid library surface entry: ' + str(entry))
        section = next(iter(data))
        if not isinstance(data[section], dict) or list(data[section]) != [entry.stem]:
            raise ValueError('surface entry must own its filename: ' + str(entry))
        if any(entry.stem in manifest[name] for name in ('libraries', 'databases')):
            raise ValueError('duplicate library surface entry: ' + str(entry))
        manifest[section].update(data[section])
    if not manifest['libraries']:
        raise ValueError('no library surface entries: ' + str(path.with_suffix('')))
    return manifest


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
    # Callers name a checkout directory, never an inherited Git location or index.
    env = {name: value for name, value in os.environ.items()
           if name not in {'GIT_DIR', 'GIT_WORK_TREE', 'GIT_COMMON_DIR', 'GIT_INDEX_FILE'}}
    top = FilePath(subprocess.check_output(
        ['git', '-C', str(root), 'rev-parse', '--show-toplevel'], env=env
    ).decode(errors='surrogateescape').removesuffix('\n'))
    root.relative_to(top)
    names = subprocess.check_output(
        ['git', '-C', str(root), 'ls-files', '-z', '--full-name', '--', '.'], env=env)
    return sorted({top / os.fsdecode(name) for name in names.split(b'\0') if name})
