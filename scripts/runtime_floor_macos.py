"""Validate Mach-O deployment commands and their delivered dependency closure."""
from pathlib import PurePosixPath
import posixpath
import re


def versions(output):
    targets = re.findall(r'cmd LC_BUILD_VERSION\s+cmdsize \d+\s+platform \S+\s+minos ([0-9.]+)|cmd LC_VERSION_MIN_MACOSX\s+cmdsize \d+\s+version ([0-9.]+)', output)
    return [left or right for left, right in targets]


def dependencies(name, output, names):
    slices = re.split(r'(?m)^.*\(architecture [^)]+\):\n', output)
    if len(slices) > 1:
        for item in slices[1:]:
            dependencies(name, item, names)
        return
    imports = re.findall(r'cmd LC_(?:LOAD(?:_WEAK|_UPWARD)?|REEXPORT)_DYLIB\s+cmdsize \d+\s+name (.*?) \(offset', output)
    rpaths = re.findall(r'cmd LC_RPATH\s+cmdsize \d+\s+path (.*?) \(offset', output)
    # Archive paths also represent their ordinary extracted install layout.
    available = set(names)
    available.update(re.sub(r'[^/]+\.(?:tar\.gz|tgz|gem|whl|jar|nupkg)/', '', item) for item in names)
    # The published Zig install command places its packaged native library beside
    # the precompiled consumer before execution.
    available.update('checks/zig/' + PurePosixPath(item).name for item in available.copy()
                     if item.startswith('zig/thinkthen/native/osx-arm64/'))
    parent = str(PurePosixPath(name).parent)

    def system(path):
        return path.startswith(('/usr/lib/', '/System/Library/Frameworks/', '/Library/Frameworks/R.framework/'))

    def resolve(path):
        for prefix in ('@loader_path/', '@executable_path/'):
            if path.startswith(prefix):
                return posixpath.normpath(parent + '/' + path.removeprefix(prefix))
        return path

    for dependency in imports:
        if system(dependency):
            continue
        if dependency.startswith('@rpath/'):
            suffix = dependency.removeprefix('@rpath/')
            candidates = [resolve(path + '/' + suffix) for path in rpaths]
            assert any(system(path) or path in available for path in candidates), (name, dependency, rpaths)
        else:
            assert resolve(dependency) in available, (name, dependency)
