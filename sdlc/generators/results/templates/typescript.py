"""Owned JavaScript values and declarations from the shared Rust graph."""
import json
ROOTS = ('completesessionPacket',)

def name(key):
    return 'Native' + ''.join(p[:1].upper()+p[1:] for p in key.removeprefix('complete').split('_'))

def typ(source):
    if isinstance(source,bool): return 'JsonValue'
    if '$ref' in source: return name(source['$ref'].removeprefix('#/$defs/'))
    if 'const' in source: return json.dumps(source['const'])
    if 'enum' in source: return ' | '.join(json.dumps(v) for v in source['enum'])
    if 'variants' in source: return ' | '.join(name(k) for k,_ in source['variants'])
    if 'primitive_schemas' in source: return ' | '.join(typ(v) for v in source['primitive_schemas'].values())
    for union in ('anyOf','oneOf'):
        if union in source: return ' | '.join(typ(v) for v in source[union])
    kind=source.get('type')
    if isinstance(kind,list): return ' | '.join(typ({**source,'type':v}) for v in kind)
    if kind=='array': return 'readonly ('+typ(source.get('items',{}))+')[]'
    if kind=='object': return 'Readonly<Record<string, '+typ(source.get('additionalProperties',{}))+'>>'
    return {'string':'string','integer':'number | NativeNumber','number':'number | NativeNumber','boolean':'boolean','null':'null'}.get(kind,'JsonValue')

def declarations(definitions):
    lines=['// Generated from the shared Rust result graph; do not edit.', 'export type JsonValue = null | boolean | number | NativeNumber | string | readonly JsonValue[] | { readonly [key: string]: JsonValue };']
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
    lines += ['export function packet(value: JsonValue): NativeSessionPacket;', 'export function parse(text: string): JsonValue;', 'export const REQUEST_VERSION: string;']
    return '\n'.join(lines)+'\n'

def runtime(definitions,version):
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
function packet(value) { return convert(graph.completesessionPacket,value,'completesessionPacket'); }
''', 'module.exports = { packet, parse, NativeNumber, REQUEST_VERSION: '+json.dumps(version)+', '+', '.join(name(k) for k,s in definitions.items() if 'properties' in s)+' };']
    return '\n'.join(lines)+'\n'
