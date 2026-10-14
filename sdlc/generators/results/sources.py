"""Pure ownership and source classification shared by size and policy gates."""
from pathlib import PurePosixPath as Path
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
    if (relative in config.get('test_files', []) or Path(path).name == 'tests.rs'
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
