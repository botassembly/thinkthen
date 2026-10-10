"""C++ immutable typed accessors from the native semantic graph."""
import json
import re
from csharp import shape as common_shape, enum_values

def shape(raw):
    if isinstance(raw, dict) and "const" in raw and "type" not in raw:
        raw = {**raw, "type": "string" if isinstance(raw["const"],str) else "boolean" if isinstance(raw["const"],bool) else "integer"}
    try:
        result = common_shape(raw)
        if isinstance(result,dict) and enum_values(result): return {**result,"type":"string"}
        return result
    except ValueError:
        alternatives=raw.get("anyOf", raw.get("oneOf",[]))
        if not alternatives and isinstance(raw.get("type"),list): alternatives=[{**raw,"type":kind} for kind in raw["type"]]
        if not alternatives: raise
        return {"cpp_alternatives": alternatives}
ROOTS = ('completesessionPacket',)

def name(key):
    return ''.join(p[:1].upper()+p[1:] for p in re.findall(r'[A-Z]?[a-z]+|[A-Z]+(?![a-z])|[0-9]+', key.removeprefix('complete')))

def integer_type(s):
    if s.get("minimum", -1) >= 0 or s.get("format", "").startswith("uint"):
        return "uint8_t" if s.get("maximum")==255 else "uint32_t" if s.get("maximum")==4294967295 else "uint64_t"
    return "int64_t"

def typ(raw):
    s=shape(raw)
    if 'cpp_alternatives' in s: return 'std::variant<'+', '.join(dict.fromkeys(typ(child) for child in s['cpp_alternatives']))+'>'
    if '$ref' in s: return name(s['$ref'].removeprefix('#/$defs/'))
    kind=s.get('type')
    if kind=='array': return 'std::vector<'+typ(s.get('items',{}))+'>'
    if kind=='object' and isinstance(s.get('additionalProperties'),dict): return 'std::vector<std::pair<std::string, '+typ(s['additionalProperties'])+'>>'
    if 'primitive_variants' in s:
        schemas=s.get('primitive_schemas',{})
        return 'std::variant<'+', '.join(dict.fromkeys(typ(schemas.get(k,{'type':k})) for k in s['primitive_variants'] if k!='null'))+'>'
    return {'string':'std::string','boolean':'bool','integer':integer_type(s),'number':'double','null':'std::nullptr_t'}.get(kind,'Json') if isinstance(kind,str) else 'Json'

def condition(tag):
    mode,member,value=tag
    if mode=='literal': return f'literal(value_, {json.dumps(member)}, {json.dumps(value)})'
    if mode=='literals': return ' && '.join(f'literal(value_, {json.dumps(k)}, {json.dumps(v)})' for k,v in value.items())
    if mode=='member': return f'value_.contains({json.dumps(member)})'
    return ' && '.join([f'value_.contains({json.dumps(k)})' for k in value['required']]+[f'!value_.contains({json.dumps(k)})' for k in value['excluded']])

def render(definitions,version):
    lines=['// Generated from the shared Rust result graph; do not edit.','#pragma once','#include "result_value.hpp"','namespace tt::results {',f'inline constexpr const char* request_version = {json.dumps(version)};']
    for key in definitions: lines += [f'class {name(key)};']
    bodies=[]
    for key,raw in definitions.items():
        n=name(key); s=shape(raw)
        lines += [f'class {n} : public Node {{ public: using Node::Node;']
        if 'properties' in s:
            for member,field in s['properties'].items():
                t=typ(field); method=member+'_' if member in ('template','class','operator','namespace','default','true','false') else member
                lines += [f'    Presence<{t}> {method}() const;']
                bodies += [f'inline Presence<{t}> {n}::{method}() const {{ return member_value<{t}>(value_, {json.dumps(member)}); }}']
        for child,tag in s.get('variants',[]):
            cn=name(child)
            lines += [f'    std::optional<{cn}> as_{cn}() const;']
            bodies += [f'inline std::optional<{cn}> {n}::as_{cn}() const {{ if ({condition(tag)}) return {cn}(value_); return std::nullopt; }}']
        if 'properties' not in s and 'variants' not in s:
            t=typ(s)
            lines += [f'    {t} value() const;']
            bodies += [f'inline {t} {n}::value() const {{ return decode<{t}>(value_); }}']
        lines += ['};']
    return '\n'.join(lines+bodies+['}'])+'\n'

def input_render(definitions):
    lines=['// Generated from request.schema.json; do not edit.','#pragma once','#include "result_value.hpp"','namespace tt::inputs {','using results::Node;']
    for key in definitions: lines += [f'class {name(key)};']
    bodies=[]
    for key,raw in definitions.items():
        n=name(key);s=shape(raw)
        lines += [f'class {n} : public Node {{ public:']
        if 'variants' in s:
            for child,_ in s['variants']:
                cn=name(child); lines += [f'    {n}(const {cn}& value);']
                bodies += [f'inline {n}::{n}(const {cn}& value):Node(value.document()) {{}}']
        elif 'properties' in s:
            constants={k:v['const'] for k,v in s['properties'].items() if isinstance(v,dict) and 'const' in v}
            expr='Json::Object{'+','.join('{'+json.dumps(k)+',Json('+json.dumps(v)+')}' for k,v in constants.items())+'}'
            lines += [f'    {n}():Node(Json({expr})) {{}}']
            for member,field in s['properties'].items():
                if isinstance(field,dict) and 'const' in field: continue
                t=typ(field); lines += [f'    {n}& set_{member}(const {t}& value);']
                bodies += [f'inline {n}& {n}::set_{member}(const {t}& value) {{ set_member({json.dumps(member)},results::encode(value)); return *this; }}']
        else:
            t=typ(s)
            lines += [f'    explicit {n}(const {t}& value);']
            bodies += [f'inline {n}::{n}(const {t}& value):Node(results::encode(value)) {{}}']
        lines += ['};']
    return '\n'.join(lines+bodies+['}'])+'\n'
