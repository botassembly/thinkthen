"""Go accessors from the shared prepared result graph."""
import re
import json
from csharp import shape
from host_answers import named_object_fields, usage_operations, call_results, scalar_input_selectors
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
    lines=['// Generated from the shared Rust result graph; do not edit.','package thinkthen','import ("encoding/json"; "bytes"; "reflect"; "context"; "strconv")']
    for key, raw in definitions.items():
        n=name(key); s=shape(raw)
        lines += [f'type {n} struct {{ ownedJSON }}']
        if s.get('type') in ('string','boolean','integer','number'):
            lines += [f'func (v {n}) Value() ({typ(s)}, error) {{ return ownedDecode[{typ(s)}](v.raw) }}']
        if not s.get('properties') and not s.get('variants') and s.get('type') not in ('string', 'boolean', 'integer', 'number'):
            alternatives = s.get('oneOf', s.get('anyOf', []))
            if alternatives and all(a.get('type') == 'string' for a in alternatives):
                lines += [f'func (v {n}) Value() (string,error) {{ return ownedDecode[string](v.raw) }}']
            elif s.get('type') == 'object':
                lines += [f'func (v {n}) Value() ({typ(s)},error) {{ return ownedDecode[{typ(s)}](v.raw) }}']
            else:
                lines += [f'func (v {n}) Value() (any,error) {{ return v.jsonValue() }}']
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
    return '\n'.join(lines)+'\n'+inputs(input_definitions)+named_calls(input_definitions)+call_helpers(definitions, input_definitions)



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
    def encoded(value, ft):
        if ft in ('[]any', 'map[string]any'): return value
        if ft.startswith('[]'): return f'transportSequence({value})'
        if ft.startswith('map[string]'): return f'transportDictionary({value})'
        return value
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
            fields=[]
            for member,field in s['properties'].items():
                resolved=resolve(field)
                literals=resolved.get('oneOf',[]) if isinstance(resolved,dict) else []
                if isinstance(resolved,dict) and 'const' in resolved:
                    constants[member]=resolved['const']; continue
                if len(literals)==1 and 'const' in literals[0]:
                    constants[member]=literals[0]['const']; continue
                ft=field_type(field,key+ident(member)); required=member in s.get('required',[])
                # Optional pointers retain explicit false, zero, empty and null values.
                fields.append((member, ft, required))
                if not required: ft='*'+ft
                lines.append(f'{ident(member)} {ft} `json:"{member}{"" if required else ",omitempty"}"`')
            lines += ['}']
            lines += [f'func (v {n}) marshalJSON() ([]byte,error) {{', 'object:=map[string]json.RawMessage{}']
            lines += [f'object[{json.dumps(k)}]=json.RawMessage({json.dumps(json.dumps(v))})' for k,v in constants.items()]
            for member, ft, required in fields:
                value='v.'+ident(member)
                if not required: lines.append(f'if {value}!=nil {{'); value='*'+value
                if ft == 'any':
                    lines.append(f'raw,err:=json.Marshal({value}); if err!=nil {{ return nil,err }}; object[{json.dumps(member)}]=raw')
                else:
                    lines.append(f'if err:=transportMember(object,{json.dumps(member)},{encoded(value,ft)});err!=nil {{ return nil,err }}')
                if not required: lines.append('}')
            lines += ['return json.Marshal(object)', '}']
            continue
        if s.get('type') == 'null':
            lines += [f'type {n} struct {{}}', f'func ({n}) marshalJSON() ([]byte,error) {{ return []byte("null"),nil }}']; continue
        ft=field_type(s,key+'Value')
        nullable = isinstance(s.get('type'),list) and 'null' in s['type']
        lines.append(f'type {n} = *{ft}' if nullable else f'type {n} {ft}')
        if not nullable and (ft.startswith('[]') and not s.get('x-preserve-order') or ft.startswith('map[string]')):
            lines += [f'func (v {n}) marshalJSON() ([]byte,error) {{ return json.Marshal({encoded(ft + "(v)", ft)}) }}']
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


def call_helpers(definitions, input_definitions):
    rules = call_results(definitions)
    lines = ['type Receipt struct { ownedJSON; function string }',
             'type Probability interface { isProbability() }',
             'type ProbabilityScalar float64', 'type ProbabilityList []*float64',
             'type ProbabilityRows []ProbabilityList',
             'func (ProbabilityScalar) isProbability() {}',
             'func (ProbabilityList) isProbability() {}',
             'func (ProbabilityRows) isProbability() {}']
    for function, rule in rules.items():
        payload = rule['receipt_payload'].get('items', rule['receipt_payload'])
        target = typ(payload)
        lines += [f'func (v Receipt) As{function.title()}() ({target},error) {{ if v.function!={json.dumps(function)} {{ return {target}{{}},errAlternative }}; return ownedDecode[{target}](v.raw) }}']
    lines += ['type callRule struct { projection, shape, probability string; path, pick, selection []string; collection, nested bool; resolution *resolutionRule }',
              'type resolutionGuard struct { path []string; value string }', 'type resolutionRule struct { path, index []string; guards []resolutionGuard }', 'var callRules = map[string]callRule{']
    for function, rule in rules.items():
        path = '[]string{' + ','.join(json.dumps(x) for x in rule.get('probability_path', ())) + '}'
        pick = '[]string{' + ','.join(json.dumps(x) for x in rule.get('pick_path', ())) + '}'
        collection = str(rule['receipt_payload'].get('type') == 'array').lower()
        nested = str(rule['call_shape'] == 'input' and rule['probability_mode'] == 'items').lower()
        selection = '[]string{' + ','.join(json.dumps(x) for x in rule.get('selection_path', ())) + '}'
        resolution = 'nil'
        if 'resolution' in rule:
            spec = rule['resolution']
            guards = '[]resolutionGuard{' + ','.join('{[]string{' + ','.join(json.dumps(x) for x in path) + '},' + json.dumps(value) + '}' for path, value in spec['guards']) + '}'
            resolution = '&resolutionRule{[]string{' + ','.join(json.dumps(x) for x in spec['path']) + '},[]string{' + ','.join(json.dumps(x) for x in spec['index_path']) + '},' + guards + '}'
        lines += [f'{json.dumps(function)}: {{{json.dumps(rule["value_projection"])},{json.dumps(rule["call_shape"])},{json.dumps(rule["probability_mode"])},{path},{pick},{selection},{collection},{nested},{resolution}}},']
    lines += ['}']
    # Input shape comes from the canonical selector grammar, independently of
    # a nested array in an authored answer.
    scalars = [name(child) for child in scalar_input_selectors(input_definitions)]
    lines += ['func scalarInput(input RequestInput) bool { switch input.(type) {',
              'case ' + ','.join(scalars) + ': return true', '}; return false }']
    return '\n'.join(lines) + '\n' + call_runtime()


def call_runtime():
    return r'''func callPath(raw json.RawMessage, path []string) json.RawMessage {
    for _,field:=range path { member:=ownedMember[json.RawMessage](raw,field); if member.Err!=nil || !member.Present || member.Null { return nil }; raw=member.Value }; return raw
}
func callChance(raw json.RawMessage) Probability {
    if len(raw)==0 { return nil }; value,err:=ownedDecode[*float64](raw); if err!=nil || value==nil { return nil }; return ProbabilityScalar(*value)
}
func probabilityPointer(value Probability) *float64 { if scalar,ok:=value.(ProbabilityScalar); ok { result:=float64(scalar); return &result }; return nil }
func receiptChance(receipt json.RawMessage, rule callRule) (Probability,error) {
    if len(rule.selection)>0 && len(callPath(receipt,rule.selection))==0 { return nil,nil }
    switch rule.probability {
    case "null": return nil,nil
    case "yes": return callChance(callPath(receipt,rule.path)),nil
    case "picked":
        pickRaw:=callPath(receipt,rule.pick); if len(pickRaw)==0 { return nil,nil }; pick,err:=ownedDecode[*string](pickRaw); if err!=nil { return nil,err }; if pick==nil { return nil,nil }
        return callChance(callPath(callPath(receipt,rule.path),[]string{*pick})),nil
    }
    answer:=callPath(receipt,[]string{"answer"}); value:=callPath(receipt,[]string{"value"})
    if ownedHas(value,"entities") {
        entities,err:=ownedDecode[[]json.RawMessage](callPath(value,[]string{"entities"})); if err!=nil { return nil,err }
        proposals,err:=ownedDecode[[]json.RawMessage](callPath(answer,[]string{"proposals"})); if err!=nil { return nil,err }
        chances:=make(ProbabilityList,0,len(entities))
        for _,entity:=range entities {
            var chance *float64
            for _,proposal:=range proposals {
                kept:=ownedMember[bool](proposal,"kept"); if !kept.Value { continue }
                selected:=callPath(proposal,[]string{"selected"})
                if bytes.Equal(callPath(entity,[]string{"start"}),callPath(selected,[]string{"start"})) && bytes.Equal(callPath(entity,[]string{"end"}),callPath(selected,[]string{"end"})) && bytes.Equal(callPath(entity,[]string{"kind"}),callPath(proposal,[]string{"kind"})) { chance=probabilityPointer(callChance(callPath(proposal,[]string{"span_probability"}))); break }
            }; chances=append(chances,chance)
        }; return chances,nil
    }
    if ownedHas(value,"proposals") {
        proposals,err:=ownedDecode[[]json.RawMessage](callPath(value,[]string{"proposals"})); if err!=nil { return nil,err }
        chances:=make(ProbabilityList,0,len(proposals)); for _,proposal:=range proposals { chances=append(chances,probabilityPointer(callChance(callPath(proposal,[]string{"probability"})))) }; return chances,nil
    }
    if len(bytes.TrimSpace(value))>0 && bytes.TrimSpace(value)[0]=='[' {
        edges,err:=ownedDecode[[]json.RawMessage](value); if err!=nil { return nil,err }
        chances:=make(ProbabilityList,0,len(edges)); for _,edge:=range edges { chances=append(chances,probabilityPointer(callChance(callPath(edge,[]string{"probability"})))) }; return chances,nil
    }
    return callChance(callPath(answer,[]string{"probability"})),nil
}
func (v *Call) settle(function string, input RequestInput) error {
    rule:=callRules[function]; resolved:=map[string]bool{}
    if rule.resolution!=nil { for _,packet:=range v.Packets {
        if !ownedLiteral(packet.raw,"function",function) { continue }; matches:=true
        for _,guard:=range rule.resolution.guards { if !bytes.Equal(callPath(packet.raw,guard.path),json.RawMessage(strconv.Quote(guard.value))) { matches=false;break } }
        if matches { resolved[string(callPath(packet.raw,rule.resolution.index))]=len(callPath(packet.raw,rule.resolution.path))>0 }
    } }
    rows:=make([]json.RawMessage,0); aggregates:=make([]json.RawMessage,0); hasAggregate:=false
    for _,packet:=range v.Packets {
        if !ownedLiteral(packet.raw,"function",function) { continue }
        raw:=callPath(packet.raw,[]string{"value"})
        if ownedLiteral(packet.raw,"kind","row") { rows=append(rows,raw) }
        if ownedLiteral(packet.raw,"kind","aggregate") {
            hasAggregate=true
            if rule.collection { chunk,err:=ownedDecode[[]json.RawMessage](raw); if err!=nil { return err }; aggregates=append(aggregates,chunk...) } else { aggregates=append(aggregates,raw) }
        }
    }
    if hasAggregate { rows=aggregates }
    v.Results=make([]Receipt,0,len(rows)); values:=make([]any,0,len(rows)); chances:=make([]Probability,0,len(rows))
    for ordinal,raw:=range rows {
        projected:=ownedMember[json.RawMessage](raw,rule.projection); if projected.Err!=nil { return projected.Err }; if !projected.Present { return errAlternative }
        var value any; if !projected.Null { decoded,err:=ownedDecode[any](projected.Value); if err!=nil { return err }; value=decoded }
        chance,err:=receiptChance(raw,rule); if err!=nil { return err }
        if rule.resolution!=nil { index:=callPath(raw,[]string{"index"}); if len(index)==0 { index=json.RawMessage(strconv.Itoa(ordinal)) }; if !resolved[string(index)] { chance=nil } }
        v.Results=append(v.Results,Receipt{ownedJSON{raw:raw},function}); values=append(values,value); chances=append(chances,chance)
    }
    v.Value=values; if rule.probability!="null" {
        nested:=rule.nested
        if nested { grouped:=make(ProbabilityRows,0,len(chances)); for _,chance:=range chances { group,_:=chance.(ProbabilityList); grouped=append(grouped,group) }; v.Probability=grouped } else { aligned:=make(ProbabilityList,0,len(chances)); for _,chance:=range chances { aligned=append(aligned,probabilityPointer(chance)) }; v.Probability=aligned }
    }
    if rule.shape=="aggregate" || rule.shape=="input" && scalarInput(input) {
        v.Value=nil; v.Probability=nil
        if len(values)>0 { v.Value=values[0]; if rule.probability!="null" { v.Probability=chances[0] } }
    }
    if v.Terminal!=nil { facts:=v.Terminal.Facts(); if facts.Err!=nil { return facts.Err }; v.Facts=facts.Value }
    return nil
}
'''
