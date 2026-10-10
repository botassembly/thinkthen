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
    return '\n'.join(lines)+'\n'+inputs()


def usage(header):
    members = re.findall(r'^#define (THINKTHEN_COMPLETE_USAGE_PERSISTENCE_(\w+)_V1) (\d+)\s*$', header, re.M)
    lines = ['// Generated from the compiler-derived C header; do not edit.', 'package thinkthen', 'type UsagePersistenceState uint32', 'const (']
    lines += ['Usage' + word.title() + ' UsagePersistenceState = ' + value for _, word, value in members]
    return '\n'.join(lines + [')']) + '\n'


def inputs():
    """Generate Go input values from the Rust-owned request grammar."""
    from pathlib import Path
    definitions = json.loads((Path(__file__).resolve().parents[4] / 'specification/request.schema.json').read_text())['$defs']
    pending = ['EngineSettings', 'RequestOptions', 'RequestQuestion', 'RequestItem', 'RequestSource', 'RequestInput']
    generated = {}
    lines = []
    def ident(key):
        if key == 'SourceUnit': return 'RequestSourceUnit'
        return ''.join(p[:1].upper()+p[1:] for p in re.findall(r'[A-Z]?[a-z]+|[A-Z]+(?![a-z])|[0-9]+', key))
    def resolve(s):
        return definitions[s['$ref'].split('/')[-1]] if isinstance(s, dict) and '$ref' in s else s
    def field_type(s, key):
        if s is True or not s: return 'any'
        if '$ref' in s:
            ref = s['$ref'].split('/')[-1]; pending.append(ref); return ident(ref)
        t = s.get('type')
        if t is None and 'const' in s:
            t = 'string' if isinstance(s['const'],str) else 'boolean' if isinstance(s['const'],bool) else 'integer'
        if isinstance(t, list): t = next((x for x in t if x != 'null'), 'null')
        if t == 'array': return '[]' + field_type(s.get('items', {}), key+'Item')
        if t == 'object' and 'properties' not in s:
            values = s.get('additionalProperties', True)
            return 'map[string]' + field_type(values, key+'Value')
        if t == 'object' or 'anyOf' in s or 'oneOf' in s:
            definitions[key] = s; pending.append(key); return ident(key)
        return {'string':'string', 'boolean':'bool', 'integer':('uint64' if str(s.get('format','')).startswith('uint') else 'int64'), 'number':'float64'}.get(t, 'any')
    while pending:
        key = pending.pop()
        if key in generated: continue
        s = definitions[key]; generated[key] = True; n = ident(key)
        if isinstance(s,dict) and isinstance(s.get('type'),list) and len([t for t in s['type'] if t != 'null']) > 1:
            s = {**s, 'anyOf':[{'type':t} for t in s['type']]}; s.pop('type')
        if s is True or not s:
            lines.append(f'type {n} = any'); continue
        alternatives = s.get('oneOf', s.get('anyOf', []))
        if alternatives and not ('properties' in s):
            # Literal string enums retain their schema spelling and typed constants.
            if all(isinstance(a,dict) and isinstance(a.get('const'),str) for a in alternatives):
                lines += [f'type {n} string', 'const (']
                lines += [f'{n}{ident(a["const"])} {n} = {json.dumps(a["const"])}' for a in alternatives]
                lines += [')']; continue
            # Keep explicit null alternatives as generated null values.
            lines.append(f'type {n} interface {{ is{n}() }}')
            for a in alternatives:
                r=resolve(a)
                if '$ref' in a:
                    child=a['$ref'].split('/')[-1]; pending.append(child)
                else:
                    literals=[v['const'] for v in r.get('properties',{}).values() if isinstance(v,dict) and isinstance(v.get('const'),str)]
                    required=[v for v in r.get('required',[]) if v not in ('kind','version')]
                    suffix=ident(literals[0]) if literals else ident(required[0]) if required else ident(str(r.get('type','Value')))
                    child=key+suffix; definitions[child]=r; pending.append(child)
                cn=ident(child)
                lines.append(f'func ({cn}) is{n}() {{}}')
            continue
        if 'properties' in s:
            lines += [f'type {n} struct {{']
            constants={}
            for member,field in s['properties'].items():
                if isinstance(field,dict) and 'const' in field:
                    constants[member]=field['const']; continue
                ft=field_type(field,key+ident(member)); required=member in s.get('required',[])
                # Optional pointers retain explicit false, zero, empty and null values.
                if not required: ft='*'+ft
                lines.append(f'{ident(member)} {ft} `json:"{member}{"" if required else ",omitempty"}"`')
            lines += ['}']
            if constants:
                lines += [f'func (v {n}) MarshalJSON() ([]byte,error) {{', f'type plain {n}', 'data,err:=json.Marshal(plain(v)); if err!=nil { return nil,err }; var object map[string]json.RawMessage; if err=json.Unmarshal(data,&object);err!=nil {return nil,err}']
                lines += [f'object[{json.dumps(k)}]=json.RawMessage({json.dumps(json.dumps(v))})' for k,v in constants.items()]
                lines += ['return json.Marshal(object)', '}']
            continue
        if s.get('type') == 'null':
            lines += [f'type {n} struct {{}}', f'func ({n}) MarshalJSON() ([]byte,error) {{ return []byte("null"),nil }}']; continue
        ft=field_type(s,key+'Value')
        nullable = isinstance(s.get('type'),list) and 'null' in s['type']
        lines.append(f'type {n} = *{ft}' if nullable else f'type {n} {ft}')
    return '\n'.join(lines)+'\n'
