"""Owned Go accessors from the shared prepared result graph."""
import re
import json
from csharp import shape
ROOTS = ('completesessionPacket',)
def name(key):
    return 'Owned' + ''.join(p[:1].upper()+p[1:] for p in re.findall(r'[A-Z]?[a-z]+|[A-Z]+(?![a-z])|[0-9]+', key.removeprefix('complete')))
def typ(raw):
    s=shape(raw)
    if '$ref' in s: return name(s['$ref'].removeprefix('#/$defs/'))
    t=s.get('type')
    if t=='array': return '[]'+typ(s['items'])
    if t=='object' and isinstance(s.get('additionalProperties'),dict): return 'map[string]'+typ(s['additionalProperties'])
    return {'string':'string','boolean':'bool','integer':'json.Number','number':'json.Number'}.get(t,'any')
def condition(tag):
    mode,field,value=tag
    if mode=='literal': return f'ownedLiteral(v.raw, {json.dumps(field)}, {json.dumps(value)})'
    if mode=='literals': return ' && '.join(f'ownedLiteral(v.raw, {json.dumps(k)}, {json.dumps(x)})' for k,x in value.items())
    if mode=='member': return f'ownedHas(v.raw, {json.dumps(field)})'
    return ' && '.join([f'ownedHas(v.raw, {json.dumps(k)})' for k in value['required']]+[f'!ownedHas(v.raw, {json.dumps(k)})' for k in value['excluded']])
def render(definitions):
    lines=['// Generated from the shared Rust result graph; do not edit.','package thinkthen','import "encoding/json"']
    for key, raw in definitions.items():
        n=name(key); s=shape(raw)
        lines += [f'type {n} struct {{ ownedJSON }}']
        if s.get('type') in ('string','boolean','integer','number'):
            lines += [f'func (v {n}) Value() ({typ(s)}, error) {{ return ownedDecode[{typ(s)}](v.raw) }}']
        if 'properties' in s:
            for member,field in s['properties'].items():
                method=name(member).removeprefix('Owned')
                lines += [f'func (v {n}) {method}() Presence[{typ(field)}] {{ return ownedMember[{typ(field)}](v.raw, "{member}") }}']
        for child,tag in s.get('variants',[]):
            cn=name(child)
            lines += [f'func (v {n}) As{cn.removeprefix("Owned")}() ({cn}, error) {{ if !({condition(tag)}) {{ return {cn}{{}}, errOwnedAlternative }}; return ownedDecode[{cn}](v.raw) }}']
    return '\n'.join(lines)+'\n'


def usage(header):
    members = re.findall(r'^#define (THINKTHEN_COMPLETE_USAGE_PERSISTENCE_(\w+)_V1) (\d+)\s*$', header, re.M)
    lines = ['// Generated from the compiler-derived C header; do not edit.', 'package thinkthen', 'type UsagePersistenceState uint32', 'const (']
    lines += ['Usage' + word.title() + ' UsagePersistenceState = ' + value for _, word, value in members]
    return '\n'.join(lines + [')']) + '\n'
