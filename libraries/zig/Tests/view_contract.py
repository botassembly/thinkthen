"""Derive the installed view oracle's enum labels from the C generator graph."""
import importlib.util
import json
from pathlib import Path
import sys
ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'sdlc/generators/results'))
import generate
spec = importlib.util.spec_from_file_location('views', ROOT / 'sdlc/generators/results/templates/c.py')
views = importlib.util.module_from_spec(spec)
spec.loader.exec_module(views)
def render():
    definitions = generate.prepare(generate.graph(json.loads(generate.SCHEMA.read_text()), ('completesessionPacket',)))
    target = views.Target(definitions)
    for name, source in definitions.items():
        target.definition(name, source)
    lines = ['// Derived from the canonical C view generator; test oracle only.', 'const c = @import("thinkthen").c;', 'pub fn enumLabel(comptime T: type, kind: u32) ?[]const u8 {']
    for name, source in sorted(target.schemas.items()):
        if 'enum' in source:
            lines.append('    if (T == c.' + name + ') return switch (kind) { ' + ', '.join(str(i) + ' => ' + json.dumps(v) for i, v in enumerate(source['enum'], 1)) + ', else => null };')
    return '\n'.join(lines + ['    return null;', '}']) + '\n'
if __name__ == '__main__':
    print(render(), end='')
