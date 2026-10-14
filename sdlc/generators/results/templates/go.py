"""Go accessors from the shared prepared result graph."""
import re
import json
from csharp import shape
from host_answers import aggregate_answers, named_object_fields, usage_operations
ROOTS = ('completesessionPacket', 'completeplan')
def name(key):
    if key.startswith('complete') and key.removeprefix('complete').lower() == 'error': return 'FailureDetail'
    if key.startswith('complete') and key.removeprefix('complete').lower() == 'imagemedia': return 'ResultImageMedia'
    if key.startswith('complete') and key.removeprefix('complete').lower() == 'requestfunction': return 'ResultFunction'
    if key.startswith('complete') and key.removeprefix('complete').lower() == 'recognitionstagecontext': return 'RecognitionContext'
    if key.startswith('complete') and key.removeprefix('complete').lower() == 'recognitionmode': return 'ResultRecognitionMode'
    if key.startswith('complete') and key.removeprefix('complete').lower() == 'relatefields': return 'RelationFields'
    return ''.join(p[:1].upper()+p[1:] for p in re.findall(r'[A-Z]?[a-z]+|[A-Z]+(?![a-z])|[0-9]+', key.removeprefix('complete')))
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
def render(definitions, input_definitions):
    lines=['// Generated from the shared Rust result graph; do not edit.','package thinkthen','import ("encoding/json"; "bytes"; "reflect"; "context")']
    for key, raw in definitions.items():
        n=name(key); s=shape(raw)
        lines += [f'type {n} struct {{ ownedJSON }}']
        if s.get('type') in ('string','boolean','integer','number'):
            lines += [f'func (v {n}) Value() ({typ(s)}, error) {{ return ownedDecode[{typ(s)}](v.raw) }}']
        if 'properties' in s:
            for member,field in s['properties'].items():
                method=name(member)
                lines += [f'func (v {n}) {method}() Presence[{typ(field)}] {{ return ownedMember[{typ(field)}](v.raw, "{member}") }}']
            for member in named_object_fields(s, definitions):
                method = name(member) + 'Name'
                lines += [f'func (v {n}) {method}() Presence[string] {{ nested:=ownedMember[json.RawMessage](v.raw,{json.dumps(member)}); if nested.Err!=nil || !nested.Present || nested.Null {{ return Presence[string]{{Present:nested.Present,Null:nested.Null,Err:nested.Err}} }}; return ownedMember[string](nested.Value,"name") }}']
        for child,tag in s.get('variants',[]):
            cn=name(child)
            lines += [f'func (v {n}) As{cn}() ({cn}, error) {{ if !({condition(tag)}) {{ return {cn}{{}}, errAlternative }}; return ownedDecode[{cn}](v.raw) }}']
    for function, _, payload in aggregate_answers(definitions):
        result = typ(payload)
        lines += [f'func (v SessionPacket) {function.title()}Answer() Presence[{result}] {{ if !ownedLiteral(v.raw, "kind", "aggregate") || !ownedLiteral(v.raw, "function", {json.dumps(function)}) {{ return Presence[{result}]{{Err:errAlternative}} }}; return ownedMember[{result}](v.raw, "value") }}']
    return '\n'.join(lines)+'\n'+inputs(input_definitions)+named_calls(input_definitions)+answer_helpers(definitions)


def usage(header):
    members = re.findall(r'^#define (THINKTHEN_COMPLETE_USAGE_PERSISTENCE_(\w+)_V1) (\d+)\s*$', header, re.M)
    lines = ['// Generated from the compiler-derived C header; do not edit.', 'package thinkthen', 'type UsagePersistenceState uint32', 'const (']
    lines += ['Usage' + word.title() + ' UsagePersistenceState = ' + value for _, word, value in members]
    lines += [')', 'type ErrorKind int', 'const (']
    errors = re.findall(r'^#define THINKTHEN_E([A-Z]+) (\d+)\s*$', header, re.M)
    lines += ['Kind' + word.title() + ' ErrorKind = ' + value for word, value in errors]
    lines += [')', 'func (k ErrorKind) String() string { switch k {']
    lines += ['case Kind' + word.title() + ': return ' + json.dumps(word.lower()) for word, _ in errors]
    lines += ['default: return "unknown"', '} }']
    functions = {symbol: {'arguments': arguments.split(',')}
                 for symbol, arguments in re.findall(r'\bint\s+(thinkthen_engine_\w+)\s*\(([^;]+)\);', header, re.S)}
    for _, public in usage_operations(functions):
        method = name(public)
        finish = 'true' if public.startswith('finish_') else 'false'
        lines += [f'func (c *Client) {method}() (UsageStatus,error) {{ return c.engine.{method}() }}',
                  f'func (e *nativeEngine) {method}() (UsageStatus,error) {{ return e.usageStatus({finish}) }}']
    return '\n'.join(lines) + '\n'


def inputs(definitions):
    """Generate Go input values from the Rust-owned request grammar."""
    definitions = dict(definitions)
    pending = ['RequestSessionDescriptor', 'Request', 'EngineSettings', 'RequestOptions', 'RequestQuestion', 'RequestItem', 'RequestSource', 'RequestInput']
    generated = {}
    lines = [ordered_maps()]
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
            return ('OrderedMap[' + field_type(values, key+'Value') + ']' if s.get('x-preserve-order') else 'map[string]' + field_type(values, key+'Value'))
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
        if s.get('variants'):
            s = {'oneOf': [{'$ref': '#/$defs/' + child} for child, _ in s['variants']]}
        elif s.get('primitive_variants'):
            s = {'anyOf': [s['primitive_schemas'][kind] for kind in s['primitive_variants']]}
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
                resolved=resolve(field)
                literals=resolved.get('oneOf',[]) if isinstance(resolved,dict) else []
                if isinstance(resolved,dict) and 'const' in resolved:
                    constants[member]=resolved['const']; continue
                if len(literals)==1 and 'const' in literals[0]:
                    constants[member]=literals[0]['const']; continue
                ft=field_type(field,key+ident(member)); required=member in s.get('required',[])
                # Optional pointers retain explicit false, zero, empty and null values.
                if not required: ft='*'+ft
                lines.append(f'{ident(member)} {ft} `json:"{member}{"" if required else ",omitempty"}"`')
            lines += ['}']
            if constants:
                lines += [f'func (v {n}) marshalJSON() ([]byte,error) {{', f'type plain {n}', 'data,err:=transportMarshal(plain(v)); if err!=nil { return nil,err }; var object map[string]json.RawMessage; if err=json.Unmarshal(data,&object);err!=nil {return nil,err}']
                lines += [f'object[{json.dumps(k)}]=json.RawMessage({json.dumps(json.dumps(v))})' for k,v in constants.items()]
                lines += ['return transportMarshal(object)', '}']
            continue
        if s.get('type') == 'null':
            lines += [f'type {n} struct {{}}', f'func ({n}) marshalJSON() ([]byte,error) {{ return []byte("null"),nil }}']; continue
        ft=field_type(s,key+'Value')
        nullable = isinstance(s.get('type'),list) and 'null' in s['type']
        lines.append(f'type {n} = *{ft}' if nullable else f'type {n} {ft}')
        if s.get('x-preserve-order'):
            lines += [f'func (v {n}) marshalJSON() ([]byte,error) {{ return {ft}(v).marshalJSON() }}', f'func (v *{n}) UnmarshalJSON(data []byte) error {{ return (*{ft})(v).UnmarshalJSON(data) }}']
    return '\n'.join(lines)+'\n'


def ordered_maps():
    """JSON object transport keeps pair order, including duplicates for Rust refusal."""
    return r'''// OrderedEntry carries one authored object member.
type OrderedEntry[V any] struct { Key string; Value V }
type OrderedMap[V any] []OrderedEntry[V]
func (v OrderedMap[V]) marshalJSON() ([]byte,error) {
    if v == nil { return []byte("null"),nil }
    var out bytes.Buffer; out.WriteByte('{')
    for i,entry := range v {
        key,err:=transportMarshal(entry.Key); if err!=nil { return nil,err }
        value,err:=transportMarshal(entry.Value); if err!=nil { return nil,err }
        if i>0 { out.WriteByte(',') }; out.Write(key); out.WriteByte(':'); out.Write(value)
    }
    out.WriteByte('}'); return out.Bytes(),nil
}
func (v *OrderedMap[V]) UnmarshalJSON(data []byte) error {
    if bytes.Equal(bytes.TrimSpace(data),[]byte("null")) { *v=nil; return nil }
    decoder:=json.NewDecoder(bytes.NewReader(data)); token,err:=decoder.Token()
    if err!=nil { return err }; if token!=json.Delim('{') { return &json.UnmarshalTypeError{Value:"non-object",Type:reflect.TypeOf(*v)} }
    entries:=make(OrderedMap[V],0)
    for decoder.More() {
        token,err=decoder.Token(); if err!=nil { return err }; var value V
        if err=decoder.Decode(&value);err!=nil { return err }
        entries=append(entries,OrderedEntry[V]{Key:token.(string),Value:value})
    }
    if _,err=decoder.Token();err!=nil { return err }; *v=entries; return nil
}
'''


def named_calls(definitions):
    functions = definitions['RequestFunction'].get('oneOf', [])
    return '\n'.join(
        f'type {item["const"].title()}Input interface {{ RequestInput }}\nfunc (c *Client) {item["const"].title()}(ctx context.Context, question RequestQuestion, input {item["const"].title()}Input, options *RequestOptions) (Call,error) {{ return c.call(ctx,{json.dumps(item["const"])},question,input,options) }}'
        for item in functions) + '\n'


def answer_helpers(definitions):
    from host_answers import scalar_answers, judgment_route, aggregate_projection
    route, guards = judgment_route(definitions)
    checks = []
    for path, value in guards:
        checks.append('ownedLiteral(ownedAnswerPath(packet.raw, []string{' + ','.join(json.dumps(field) for field in path[:-1]) + '}).Value,' + json.dumps(path[-1]) + ',' + json.dumps(value) + ')')
    path = '[]string{' + ','.join(json.dumps(field) for field in route) + '}'
    collections = [function for function, _, payload in aggregate_answers(definitions) if payload.get('type') == 'array']
    collection_test = ' || '.join('function==' + json.dumps(function) for function in collections)
    lines = ['func ownedObject(raw json.RawMessage) bool { value:=bytes.TrimSpace(raw); return len(value)>0 && value[0]==123 }',
             'func ownedAnswerPath(raw json.RawMessage, path []string) Presence[json.RawMessage] { result:=Presence[json.RawMessage]{Present:true,Value:raw}; for _,field:=range path { result=ownedMember[json.RawMessage](result.Value,field); if result.Err!=nil || !result.Present || result.Null { return result } }; return result }',
             'func (v Call) answerTerminal() error { if v.Terminal==nil { return &AnswerCardinalityError{Call:v} }; kind:=v.Terminal.Kind(); if kind.Err!=nil { return kind.Err }; if !kind.Present || kind.Null || kind.Value!="terminal" { return &AnswerCardinalityError{Call:v} }; failure:=v.Terminal.Failure(); if failure.Err!=nil { return failure.Err }; if failure.Present { if failure.Null || !ownedObject(failure.Value.raw) { return &AnswerCardinalityError{Call:v} }; detail:=failure.Value.Error(); if detail.Err!=nil { return detail.Err }; if !detail.Present || detail.Null || !ownedObject(detail.Value.raw) { return &AnswerCardinalityError{Call:v} }; return &SessionError{Call:v,Detail:detail.Value} }; return nil }',
             'func (v Call) answerJSON(function, judgment string) (json.RawMessage,error) {',
             'if err:=v.answerTerminal();err!=nil { return nil,err }; values:=make([]json.RawMessage,0); collection:=' + collection_test,
             'for _,packet:=range v.Packets { if !ownedLiteral(packet.raw,"function",function) { continue }; var value Presence[json.RawMessage]; row:=false',
             'if judgment=="" { row=collection && ownedLiteral(packet.raw,"kind","row"); if !row && !ownedLiteral(packet.raw,"kind","aggregate") { continue }; value=ownedMember[json.RawMessage](packet.raw,"value") } else {',
             'if !(' + ' && '.join(checks) + ') { continue }; carrier:=ownedAnswerPath(packet.raw,' + path + '); if !ownedLiteral(carrier.Value,"kind",judgment) { continue }; value=ownedMember[json.RawMessage](carrier.Value,"value")',
             '}; if value.Err!=nil { return nil,value.Err }; if value.Present { raw:=value.Value; if value.Null { raw=json.RawMessage("null") }; if row { raw,err:=transportMarshal([]json.RawMessage{raw}); if err!=nil { return nil,err }; values=append(values,raw) } else { values=append(values,raw) } }',
             '}; if judgment=="" && collection { combined:=make([]json.RawMessage,0); for _,value:=range values { var items []json.RawMessage; if err:=json.Unmarshal(value,&items);err!=nil { return nil,err }; combined=append(combined,items...) }; return transportMarshal(combined) }; if len(values)!=1 { return nil,&AnswerCardinalityError{Function:function,Count:len(values),Call:v} }; return values[0],nil', '}']
    answers = {}
    for function, _, payload in aggregate_answers(definitions):
        projected, member = aggregate_projection(function, payload, definitions)
        target = typ(projected)
        if member and shape(projected['items']).get('format', '').startswith('uint'):
            target = '[]uint64'
        answers[function] = (target, '', member)
    lines += ['func ownedAnswerProjection[T any](raw json.RawMessage, member string) ([]T,error) {',
              'rows,err:=ownedDecode[[]json.RawMessage](raw); if err!=nil { return nil,err }; values:=make([]T,0,len(rows))',
              'for _,row:=range rows { field:=ownedMember[json.RawMessage](row,member); if field.Err!=nil { return nil,field.Err }; if !field.Present { return nil,errAlternative }; value:=field.Value; if field.Null { value=json.RawMessage("null") }; decoded,err:=ownedDecode[T](value); if err!=nil { return nil,err }; values=append(values,decoded) }; return values,nil', '}']
    for function, kind, payload in scalar_answers(definitions):
        source = dict(payload)
        kinds = source.get('type')
        nullable = isinstance(kinds, list) and 'null' in kinds
        if isinstance(kinds, list): source['type'] = next(value for value in kinds if value != 'null')
        target = 'float64' if source.get('type') == 'number' else typ(source)
        answers[function] = (('*' if nullable else '') + target, kind, None)
    for function, (target, kind, member) in answers.items():
        title = function.title()
        reader = ('ownedAnswerProjection[' + target.removeprefix('[]') + '](raw,' + json.dumps(member) + ')'
                  if member else 'ownedDecode[' + target + '](raw)')
        lines += [f'func (v Call) {title}Answer() ({target},error) {{ raw,err:=v.answerJSON({json.dumps(function)},{json.dumps(kind)}); if err!=nil {{ var zero {target}; return zero,err }}; return {reader} }}',
                  f'func (c *Client) {title}Answer(ctx context.Context, question RequestQuestion, input {title}Input, options *RequestOptions) ({target},error) {{ call,err:=c.{title}(ctx,question,input,options); if err!=nil {{ var zero {target}; return zero,err }}; return call.{title}Answer() }}']
    return '\n'.join(lines) + '\n'
