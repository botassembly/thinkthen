"""Build typed authored fixture values from the canonical generated readers."""
from pathlib import Path
import re
import sys
ROOT = Path(__file__).resolve().parents[3]
sys.path[:0] = [str(ROOT / 'sdlc/generators/results'), str(ROOT / 'sdlc/generators/results/templates')]
import generate
import csharp
import json
source = csharp.render_inputs(json.loads((ROOT / 'specification/request.schema.json').read_text()), generate.prepare, generate.graph)
methods = []
for match in re.finditer(r'internal (?:new )?static (\w+) Read\(JsonElement value\) (.*)', source):
    kind, body = match.groups()
    if body.endswith(' } }'):
        body = body[:-2]
    body = re.sub(r'ParsedDocument = value.Clone\(\),?\s*', '', body)
    body = re.sub(r'(\w+)\.Read\(', r'Read\1(', body)
    methods.append('public static ' + kind + ' Read' + kind + '(JsonElement value) ' + body)
output = '''// Generated fixture adapter from the canonical request template; do not edit.
using System.Text.Json;
using ThinkThen.Inputs;
enum AuthoredQuestionKind { Atomic, DynamicChoose, Find, RankSet, Rank, Recognize, Relate, Set }
static class FixtureQuestions {
public static RequestQuestionDefinition Parse(AuthoredQuestionKind kind, string json) {
    using var document = JsonDocument.Parse(json, new JsonDocumentOptions { MaxDepth = int.MaxValue });
    return new RequestQuestionDefinition { Value = ReadRequestDefinition(document.RootElement) };
}
''' + '\n'.join(methods) + '\n}\n'
p = Path(__file__).parent / 'source/FixtureQuestions.g.cs'
if '--check' in sys.argv:
    assert p.read_text() == output, 'generated typed fixture adapter is stale'
else:
    p.write_text(output)
