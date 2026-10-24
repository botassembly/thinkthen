"""The public runtime floor contract shared by builds and package metadata."""
import json
import re
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parent.parent
FLOORS = json.loads((ROOT / 'scripts/runtime-floors.json').read_text())


def requirements(runtime, system):
    """Keep measured provenance separate from the supported minimum."""
    if system not in ('Darwin', 'Linux'):
        return runtime
    for name, item in runtime.items():
        if name in FLOORS['runtimes']:
            item['runtime'] = FLOORS['runtimes'][name]
        if name == 'r':
            item['runtime'] = f'R {FLOORS["r"][system]} series; bundled matching jsonlite binary'
        if system == 'Darwin':
            item['macos'] = FLOORS['macos']
        else:
            item['supported_glibc'] = FLOORS['linux']['swift_glibc' if name == 'swift' else 'glibc']
            item['supported_glibcxx'] = FLOORS['linux']['swift_glibcxx' if name == 'swift' else 'glibcxx']
    return runtime


def check_sources():
    # Native manifests remain ordinary ecosystem files; compare their copies.
    for name in ('sdk/swift/Package.swift', 'sdk/swift/tests/Package.swift'):
        text = (ROOT / name).read_text()
        assert f'.macOS("{FLOORS["macos"]}")' in text, name
    declarations = {
        'python': ('sdk/python/pyproject.toml', r'requires-python = ">=([0-9.]+)"'),
        'typescript': ('sdk/typescript/package.json', r'"node": ">=([0-9.]+)"'),
        'ruby': ('sdk/ruby/thinkthen.gemspec', r'required_ruby_version = ">= ([0-9.]+)"'),
        'go': ('sdk/go/go.mod', r'(?m)^go ([0-9.]+)$'),
        'java': ('sdk/java/pom.xml', r'<maven.compiler.release>([0-9.]+)<'),
        'zig': ('sdk/zig/build.zig.zon', r'minimum_zig_version = "([0-9.]+)"'),
        'swift': ('sdk/swift/Package.swift', r'swift-tools-version: ([0-9.]+)'),
        'csharp': ('sdk/csharp/ThinkThen.csproj', r'<TargetFramework>net([0-9]+)\.0<'),
    }
    for name, (path, expression) in declarations.items():
        expected = re.search(r'[0-9]+(?:\.[0-9]+)*', FLOORS['runtimes'][name])[0]
        actual = re.search(expression, (ROOT / path).read_text())[1]
        assert actual == expected, (name, actual, expected)


if __name__ == '__main__':
    check_sources()
    if sys.argv[1:] == ['macos']:
        print(FLOORS['macos'])
    else:
        sys.exit('usage: runtime_floors.py macos')
