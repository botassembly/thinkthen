"""Owned JavaScript values and declarations from the shared Rust graph."""
import json
from host_answers import call_results
ROOTS = ('completesessionPacket', 'completeplan')
INPUT_ROOTS = ('RequestOptions', 'EngineSettings', 'RequestDefinition')

def name(key):
    return 'Native' + ''.join(p[:1].upper()+p[1:] for p in key.removeprefix('complete').split('_'))

def input_name(key):
    if key.startswith('RequestDefinition_'):
        base = input_name(key.removeprefix('RequestDefinition_'))
        return base.removesuffix('Definition') + ('RequestDefinition' if base.endswith('Definition') else 'Definition')
    return ''.join(p[:1].upper()+p[1:] for p in key.removeprefix('Authored_').split('_'))

def typ(source, inputs=False):
    named = input_name if inputs else name
    if isinstance(source,bool): return 'JsonValue'
    if '$ref' in source: return named(source['$ref'].removeprefix('#/$defs/'))
    if 'const' in source: return json.dumps(source['const'])
    if 'enum' in source: return ' | '.join(json.dumps(v) for v in source['enum'])
    if 'variants' in source: return ' | '.join(named(k) for k,_ in source['variants'])
    if 'primitive_schemas' in source: return ' | '.join(typ(v, inputs) for v in source['primitive_schemas'].values())
    for union in ('anyOf','oneOf'):
        if union in source: return ' | '.join(typ(v, inputs) for v in source[union])
    kind=source.get('type')
    if isinstance(kind,list): return ' | '.join(typ({**source,'type':v}, inputs) for v in kind)
    if kind=='array': return 'readonly ('+typ(source.get('items',{}), inputs)+')[]'
    if kind=='object': return 'Readonly<Record<string, '+typ(source.get('additionalProperties',{}), inputs)+'>>'
    numeric = 'number' if inputs else 'number | NativeNumber'
    return {'string':'string','integer':numeric,'number':numeric,'boolean':'boolean','null':'null'}.get(kind,'JsonValue')

def declarations(definitions, input_definitions=None):
    lines=['// Generated from the shared Rust result graph; do not edit.', "import type {Question, Input, Controls, Result} from './native.js';", 'export type JsonValue = null | boolean | number | NativeNumber | string | readonly JsonValue[] | { readonly [key: string]: JsonValue };']
    lines += ['export class NativeNumber { readonly raw: string; private constructor(); }']
    for key,source in definitions.items():
        if 'properties' in source:
            lines += [f'export class {name(key)} {{']
            required=source.get('required',[])
            for member,field in source['properties'].items():
                lines += [' readonly '+json.dumps(member)+('' if member in required else '?')+': '+typ(field)+';']
            lines += [' has(key: string): boolean;', ' readonly [key: string]: unknown;', '}']
        else: lines += [f'export type {name(key)} = {typ(source)};']
    for verb in ('decide','choose','tag','score','filter','rank','find','annotate','recognize','relate'):
        values=[]
        for source in definitions.values():
            fields=source.get('properties',{})
            if fields.get('function',{}).get('const')==verb and 'value' in fields:
                value=fields['value']
                if value.get('type')=='array': value=value['items']
                values.append(typ(value))
        lines += ['export type Native'+verb.title()+'Result = '+' | '.join(dict.fromkeys(values))+';']
    lines += ['export interface NamedCalls {']
    for function, rule in call_results(definitions).items():
        value = typ(rule['aggregate_value'])
        scalar = typ(rule['aggregate_value']['items']) if function not in ('filter', 'rank', 'find', 'relate') else None
        if function == 'find': value += ' | null'
        mode = rule['probability_mode']
        probability = ('null' if mode == 'null' else
                       'readonly (number | null)[]' if function == 'rank' else
                       'readonly number[]' if function == 'relate' else
                       'readonly (readonly number[])[] | readonly number[]' if function == 'recognize' else
                       'number | null' if function == 'find' else
                       'readonly (number | null)[] | number | null')
        if scalar:
            scalar_probability = 'null' if mode == 'null' else 'readonly number[]' if function == 'recognize' else 'number | null'
            list_probability = 'null' if mode == 'null' else 'readonly (readonly number[])[]' if function == 'recognize' else 'readonly (number | null)[]'
            lines += [f' {function}(q:Question,i:readonly (JsonValue | import("./native.js").Item)[],c?:Controls):Promise<Result<Native{function.title()}Result, {value}, {list_probability}>>;',
                      f' {function}(q:Question,i:Exclude<JsonValue, readonly JsonValue[]> | import("./native.js").Item,c?:Controls):Promise<Result<Native{function.title()}Result, {scalar}, {scalar_probability}>>;']
            value += ' | ' + scalar
        lines += [f' {function}(q:Question,i:Input,c?:Controls):Promise<Result<Native{function.title()}Result, {value}, {probability}>>;']
    lines += ['}']
    lines += ['export const REQUEST_VERSION: string;']
    lines += input_declarations(input_definitions or {})
    return '\n'.join(lines)+'\n'

def input_declarations(definitions):
    lines = []
    excluded = {key: tag[2]['excluded'] for source in definitions.values()
                for key, tag in source.get('variants', []) if tag[0] == 'structure'}
    for key, source in definitions.items():
        if 'properties' not in source:
            lines += [f'export type {input_name(key)} = {typ(source, True)};']
            continue
        lines += [f'export interface {input_name(key)} {{']
        for member, field in source['properties'].items():
            optional = '' if member in source.get('required', []) else '?'
            lines += [f' readonly {json.dumps(member)}{optional}: {typ(field, True)};']
        lines += [f' readonly {json.dumps(member)}?: never;' for member in excluded.get(key, [])]
        lines += ['}']
    return lines

def runtime(definitions,version,input_definitions=None):
    lines=["'use strict';", '// Generated from the shared Rust result graph; do not edit.', 'const graph = {']
    for key,source in definitions.items():
        lines += [json.dumps(key)+': {']
        for member,value in source.items():
            if member=='properties':
                lines += ['"properties": {']
                lines += [json.dumps(k)+': '+json.dumps(v,separators=(',',':'))+',' for k,v in value.items()]
                lines += ['},']
            else: lines += [json.dumps(member)+': '+json.dumps(value,separators=(',',':'))+',']
        lines += ['},']
    lines += ['};', 'const classes = {};']
    for key,source in definitions.items():
        if 'properties' in source:
            lines += [f'class {name(key)} {{ has(key) {{ return Object.hasOwn(this,key); }} }}', f'classes[{json.dumps(key)}] = {name(key)};']
    lines += [r'''class NativeNumber {
  constructor(raw) {this.raw=raw;Object.freeze(this);}
  toJSON() {return JSON.rawJSON(this.raw);}
}
function parse(text) {
  return JSON.parse(text,(key,value,context)=> typeof value==='number' && Number.isInteger(value) && !Number.isSafeInteger(value) && /^-?\d+$/.test(context.source) ? new NativeNumber(context.source) : value);
}
function frozen(value) {
  if (value && typeof value === 'object') {
    for (const entry of Object.values(value)) frozen(entry);
    Object.freeze(value);
  }
  return value;
}
function selected(tag,value) {
  const [mode,member,expected]=tag;
  if(mode==='literal') return value[member]===expected;
  if(mode==='literals') return Object.entries(expected).every(([key,v])=>value[key]===v);
  if(mode==='member') return Object.hasOwn(value,member);
  return expected.required.every(key=>Object.hasOwn(value,key)) && expected.excluded.every(key=>!Object.hasOwn(value,key));
}
function convert(schema,value,key) {
  if(value instanceof NativeNumber) return value;
  if(typeof schema!=='object') return frozen(value);
  if ('$ref' in schema) { key=schema.$ref.slice('#/$defs/'.length); return convert(graph[key],value,key); }
  if (schema.variants) {
    const variant=schema.variants.find(([,tag])=>selected(tag,value));
    if(!variant) throw new TypeError('native result has no generated alternative');
    return convert(graph[variant[0]],value,variant[0]);
  }
  if (value===null || typeof value!=='object') return value;
  if (schema.primitive_schemas) return convert(schema.primitive_schemas[Array.isArray(value)?'array':'object'] || {},value);
  const union=schema.anyOf || schema.oneOf;
  if (union) {
    const kind=Array.isArray(value)?'array':'object';
    const alternative=union.find(v=>v.type===kind || v.$ref && graph[v.$ref.slice('#/$defs/'.length)].type===kind);
    return alternative ? convert(alternative,value) : frozen(value);
  }
  if (Array.isArray(value)) return Object.freeze(value.map(v=>convert(schema.items || {},v)));
  const held=key && classes[key] ? Object.create(classes[key].prototype) : {};
  for(const [member,v] of Object.entries(value)) {
    const field=schema.properties?.[member] || (typeof schema.additionalProperties==='object' ? schema.additionalProperties : {});
    Object.defineProperty(held,member,{value:convert(field,v),enumerable:true});
  }
  return Object.freeze(held);
}
function plan(value) { return convert(graph.completeplan,value,'completeplan'); }
function packet(value) { return convert(graph.completesessionPacket,value,'completesessionPacket'); }
''', result_runtime(definitions), 'module.exports = { callResult, plan, packet, parse, NativeNumber, REQUEST_VERSION: '+json.dumps(version)+', '+', '.join(name(k) for k,s in definitions.items() if 'properties' in s)+' };']
    return '\n'.join(lines)+'\n'


def result_runtime(definitions):
    rules = {function: {key: value for key, value in rule.items()
                       if key in ('value_projection', 'probability_mode', 'probability_path', 'pick_path')}
             for function, rule in call_results(definitions).items()}
    return 'const callRules = '+json.dumps(rules)+';'+r'''
function callResult(rows,functionName,scalar) {
  const rule=callRules[functionName],project=rule.value_projection;
  const values=rows.map(row=>row[project]);
  const single=functionName==='find' || functionName==='relate';
  const value=project!=='value' ? values : single ? values[0] ?? (functionName==='find'?null:[]) : scalar && values.length ? values[0] : values;
  if(rule.probability_mode==='null') return frozen({value,probability:null});
  const at=(row,path)=>path.reduce((value,key)=>value?.[key],row);
  const chances=rows.map(row=>{
    if(rule.probability_mode==='picked') return at(row,rule.probability_path)[at(row,rule.pick_path)] ?? null;
    if(functionName==='recognize') {
      if(!Object.hasOwn(row.value,'entities')) return row.value.proposals.map(p=>p.probability);
      const kept=new Map(row.answer.proposals.filter(p=>p.kept).map(p=>[JSON.stringify([p.selected.start,p.selected.end,p.kind]),p.span_probability]));
      return row.value.entities.map(e=>kept.get(JSON.stringify([e.start,e.end,e.kind])) ?? null);
    }
    if(functionName==='relate') return row.value.map(edge=>edge.probability);
    return rule.probability_path ? at(row,rule.probability_path) ?? null : row.answer?.probability ?? null;
  });
  const probability=single ? chances[0] ?? (functionName==='find'?null:[]) : functionName==='rank' ? chances : scalar && chances.length ? chances[0] : chances;
  return frozen({value,probability});
}
'''
