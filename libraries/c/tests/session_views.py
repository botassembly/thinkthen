"""Render a C consumer serializer from the existing complete-view schema graph."""
import importlib.util
import json
from pathlib import Path
import sys
ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'sdlc/generators/results'))
import generate
spec = importlib.util.spec_from_file_location('c_views', ROOT / 'sdlc/generators/results/templates/c.py')
c = importlib.util.module_from_spec(spec)
spec.loader.exec_module(c)

def render():
    definitions = generate.prepare(generate.graph(json.loads(generate.SCHEMA.read_text()), ('completesessionPacket',)))
    output, functions = [], {}
    def value(source, path, expression):
        source = c.normalized(source)
        if '$ref' in source:
            key = source['$ref'].removeprefix('#/$defs/')
            emit(key, definitions[key])
            return 'emit_' + c.ident(key) + '(*(' + expression + '));'
        if not source or source.get('type') == 'null':
            return 'emit_json(' + expression + ');'
        if any(key in source for key in ('enum','variants','primitive_variants','properties')):
            emit(path, source)
            return 'emit_' + c.ident(path) + '(*(' + expression + '));'
        kind = source.get('type')
        if kind == 'string' or isinstance(source.get('const'), str):return 'quoted(' + expression + ');'
        if kind == 'boolean':return 'fputs((' + expression + ') ? "true" : "false", stdout);'
        if kind == 'number':return 'printf("%.17g", (double)(' + expression + '));'
        if kind == 'integer':
            unsigned = source.get('minimum', -1) >= 0 or source.get('format', '').startswith('uint')
            return 'printf("%" PRI' + ('u64, (uint64_t)(' if unsigned else 'd64, (int64_t)(') + expression + '));'
        emit(path, source, direct=True)
        return 'emit_' + c.ident(path) + '(' + expression + ');'
    def emit(path, source, direct=False):
        name = c.ident(path)
        if name in functions:return
        functions[name] = None
        source = c.normalized(source)
        body = []
        if 'enum' in source:
            body += ['switch(v.kind) {'] + ['case %d: fputs(%s, stdout); break;' % (i,json.dumps(json.dumps(label))) for i,label in enumerate(source['enum'],1)] + ['default: abort(); }']
        elif 'variants' in source:
            body += ['switch(v.kind) {']
            for i,(child,_) in enumerate(source['variants'],1):
                arm=child.removeprefix(path+'_')
                body.append('case %d: %s break;' % (i,value({'$ref':'#/$defs/'+child},child,'v.data.'+arm)))
            body += ['default: abort(); }']
        elif 'primitive_variants' in source:
            body += ['switch(v.kind) {']
            for i,kind in enumerate(source['primitive_variants'],1):
                code = 'fputs("null", stdout);' if kind == 'null' else value(source['primitive_schemas'][kind],path+'_'+kind,'v.data.'+kind)
                body.append('case %d: %s break;' % (i,code))
            body += ['default: abort(); }']
        elif 'properties' in source:
            body += ['putchar(123); int comma=0;']
            for member,shape in source['properties'].items():
                access='v.'+('true_' if member=='true' else 'false_' if member=='false' else member)
                optional=member not in source.get('required',[]) or c.nullable(shape,definitions)
                if optional:body.append('if (%s.presence != THINKTHEN_COMPLETE_PRESENCE_MISSING_V1) {' % access)
                body += ['if(comma++) { putchar(44); } fputs(%s, stdout);' % json.dumps(json.dumps(member)+':')]
                if optional:
                    body += ['if(%s.presence == THINKTHEN_COMPLETE_PRESENCE_NULL_V1) fputs("null", stdout); else {' % access,value(shape,path+'_field_'+member,access+'.value'),'}','}']
                else:body.append(value(shape,path+'_field_'+member,access))
            body += ['for(size_t i=0;i<v.extensions.len;++i) { if(comma++) { putchar(44); } quoted(v.extensions.data[i].name); putchar(58); fwrite(v.extensions.data[i].json.data,1,v.extensions.data[i].json.len,stdout); }','putchar(125);']
        elif not direct:
            body.append(value(source,path+'_value','v.value'))
        elif source.get('type')=='array':
            body += ['putchar(91); for(size_t i=0;i<v.len;++i) { if(i) { putchar(44); }',value(source['items'],path+'_item','v.data[i]'),'} putchar(93);']
        elif source.get('type')=='object' and 'additionalProperties' in source:
            body += ['putchar(123); for(size_t i=0;i<v.len;++i) { if(i) { putchar(44); } quoted(v.data[i].name); putchar(58);',value(source['additionalProperties'],path+'_entry_value','v.data[i].value'),'} putchar(125);']
        else:body.append(value(source,path+'_value','v.value'))
        functions[name] = 'static void emit_' + name + '(' + name + ' v) {\n' + '\n'.join(body) + '\n}'
    emit('completesessionPacket', definitions['completesessionPacket'])
    output += ['#include <thinkthen.h>', '#include <stdio.h>', '#include <stdlib.h>', '#include <inttypes.h>',
               'static void quoted(thinkthen_complete_utf8_v1 s) { putchar(34); for(size_t i=0;i<s.len;++i) { unsigned char b=s.data[i]; if(b==34||b==92) {putchar(92);putchar(b);} else if(b<32) printf("\\\\u%04x",b); else putchar(b); } putchar(34); }',
               'static void emit_json(const thinkthen_complete_json_v1 *v) { switch(v->kind) { case 1:fputs("null",stdout);break;case 2:fputs(v->data.boolean?"true":"false",stdout);break;case 3:fwrite(v->data.number.data,1,v->data.number.len,stdout);break;case 4:quoted(v->data.string);break;case 5:putchar(91);for(size_t i=0;i<v->data.array.len;++i){if(i)putchar(44);emit_json(v->data.array.data[i]);}putchar(93);break;case 6:putchar(123);for(size_t i=0;i<v->data.object.len;++i){if(i)putchar(44);quoted(v->data.object.data[i].name);putchar(58);emit_json(v->data.object.data[i].value);}putchar(125);break;default:abort();} }']
    text='\n'.join(functions.values())
    return '\n'.join(output + ['static void emit_'+name+'('+name+' v);' for name in functions] + [text])+'\n'
if __name__ == '__main__':print(render(),end='')
