#!/usr/bin/env python3
"""Keep answer computation in Rust while existing host calls migrate."""
import json
from pathlib import Path
import re
import sys

from sources import EXTENSIONS, production, tracked_files

ROOT = Path(__file__).resolve().parents[2]

# Each language's phase 3 call ticket removes its entries. Ticket 0690 removes
# the shared helper and this entire exception list. Never add folder exceptions.
LEGACY_FILES = {
    'build-tools/results/templates/ada_answers.py',
    'build-tools/results/templates/ada_call_results.py',
    'build-tools/results/templates/ada_inputs.py',
    'build-tools/results/templates/cobol_results.py',
    'build-tools/results/templates/cpp.py',
    'build-tools/results/templates/cpp_results.py',
    'build-tools/results/templates/csharp.py',
    'build-tools/results/templates/dart.py',
    'build-tools/results/templates/go.py',
    'build-tools/results/templates/host_answers.py',
    'build-tools/results/templates/jvm.py',
    'build-tools/results/templates/jvm_host.py',
    'build-tools/results/templates/objc.py',
    'build-tools/results/templates/objc_calls.py',
    'build-tools/results/templates/php.py',
    'build-tools/results/templates/python.py',
    'build-tools/results/templates/r.py',
    'build-tools/results/templates/ruby.py',
    'build-tools/results/templates/ruby_rbs.py',
    'build-tools/results/templates/swift.py',
    'build-tools/results/templates/typescript.py',
    'build-tools/results/templates/zig.py',
    'libraries/dart/lib/src/session/values.dart',
    'libraries/go/session.go',
    'libraries/php/src/session/results.php',
    'libraries/python/thinkthen/_calls.py',
    'libraries/python/thinkthen/_pandas_calls.py',
    'libraries/ruby/lib/thinkthen/session.rb',
    'libraries/swift/Sources/ThinkThen/CallProjection.swift',
}

MARKERS = {
    'host answer rules': re.compile(r'\b(?:host_answers|callRules|probability_mode|value_projection)\b'),
    'call answer projection': re.compile(r'\b(?:callResult|receiptChance)\s*\('),
    # Settlement of a call's input/function computes a host answer. Ownership
    # settlement of a native Result is a mechanical error conversion instead.
    'call settlement': re.compile(r'\bsettle\s*\([^\n)]*\b(?:function|verb|input)\b'),
    'probability arithmetic': re.compile(
        r'\b(?:probability|probabilities|chance|odds)\s*'
        r'(?::\s*[^=\n;]+)?\s*(?:[+*/-]=|(?::=|=)\s*[^\n;]*[+*/-])'),
    # Reading a span-probability field for a declaration/codec is mechanical.
    # Selecting a proposal's odds joins observations into an answer.
    'span probability join': re.compile(
        r'(?:return|append|push|map|probabilit\w*\s*=|chance\s*=|out\.\*)'
        r'[^\n]*\b(?:proposal|proposals)\b[^\n]*\b(?:span_probability|spanProbability)\b', re.I),
}


def self_test():
    pattern = MARKERS['probability arithmetic']
    for source in ('probability = yes * no', 'probability=yes*no',
                   'probability: float = yes/no', 'chance: Double = yes-no',
                   'odds := yes+no', 'probability *= observation'):
        if not pattern.search(source):
            raise AssertionError(f'probability arithmetic escaped: {source}')
    for source in ('probability: float', 'probability = packet.probability',
                   'probabilities: list[float]', 'chance = nativeChance()'):
        if pattern.search(source):
            raise AssertionError(f'mechanical declaration refused: {source}')
    print('answer rules: arithmetic marker regressions pass')
    return 0


def main():
    if sys.argv[1:] == ['--self-test']:
        return self_test()
    manifest = json.loads((ROOT / 'conformance/library-surface.json').read_text())
    candidates = {path for path in tracked_files(ROOT / 'libraries')
                  if path.suffix in EXTENSIONS
                  and production(path.relative_to(ROOT).as_posix(), manifest)}
    # Include new, untracked templates too: a local negative plant must run
    # through the same gate without changing the index.
    candidates.update((ROOT / 'build-tools/results/templates').rglob('*.py'))
    found = set()
    failures = []
    for path in sorted(candidates):
        relative = path.relative_to(ROOT).as_posix()
        source = path.read_text()
        for name, pattern in MARKERS.items():
            for match in pattern.finditer(source):
                found.add(relative)
                if relative not in LEGACY_FILES:
                    line = source.count('\n', 0, match.start()) + 1
                    failures.append(f'{relative}:{line}: {name}')
    failures.extend(f'{path}: remove obsolete answer-rule exception'
                    for path in sorted(LEGACY_FILES - found))
    if failures:
        print('answer rules: Rust owns call answers\n' + '\n'.join(failures), file=sys.stderr)
        return 1
    print('answer rules: templates and handwritten libraries contain no new host answer rules')
    return 0


if __name__ == '__main__':
    sys.exit(main())
