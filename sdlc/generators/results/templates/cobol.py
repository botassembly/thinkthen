"""COBOL records and mechanical serialization from Rust schemas and measured C ABI."""
import hashlib
import importlib.util
import json
import re
from pathlib import Path


def identifier(value):
    text = value.removeprefix('thinkthen_').replace('_', '-')
    if len(text) > 52:
        text = text[:43] + '-' + hashlib.sha256(text.encode()).hexdigest()[:8]
    return 'tt-n-' + text


def abi_module(root):
    spec = importlib.util.spec_from_file_location('cobol_abi', root / 'sdlc/scripts/check-c-exports.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def copybook(facts):
    lines = ['      *> Generated from measured canonical C declarations; do not edit.']
    def members(record, level, path=""):
        def label(key):
            text = "v-" + (path + key).replace("_", "-").rstrip("-")
            return text if len(text) <= 60 else text[:51] + "-" + hashlib.sha256(text.encode()).hexdigest()[:8]
        output, cursor = [], 0
        direct = [(key, field) for key, field in record['fields'].items() if '.' not in key]
        union = len(direct) > 1 and all(field['offset'] == 0 for _, field in direct)
        if union:
            output.append(f' {level:02} {label("union_storage")} pic x({record["size"]}).')
        for key, field in direct:
            offset, width, kind = field['offset'], field['width'], field['type']
            if not union and offset > cursor:
                output.append(f' {level:02} filler pic x({offset-cursor}).')
            field_label = label(key)
            if union:
                field_label += ' redefines ' + label('union_storage')
            nested = kind.removeprefix('struct ').removeprefix('union ')
            if kind == 'union':
                candidates = [name for name, owners in facts['union_owners'].items()
                              if any(facts['records'][parent] is record and fieldname == key
                                     for parent, fieldname in owners)]
                if len(candidates) != 1:
                    raise ValueError(f'unsupported anonymous union: {key}')
                nested = candidates[0]
            if '*' in kind:
                output.append(f' {level:02} {field_label} usage pointer.')
            elif nested in facts['records']:
                output.append(f' {level:02} {field_label}.')
                output.extend(members(facts['records'][nested], level + 1, path + key + '_'))
            elif kind == 'double':
                output.append(f' {level:02} {field_label} usage float-long.')
            elif kind == 'float':
                output.append(f' {level:02} {field_label} usage float-short.')
            elif width in (1, 2, 4, 8):
                usage = {1:'binary-char', 2:'binary-short', 4:'binary-long', 8:'binary-double'}[width]
                signed = 'unsigned' if kind.startswith('uint') or kind in ('size_t', 'bool') else 'signed'
                output.append(f' {level:02} {field_label} usage {usage} {signed}.')
            else:
                raise ValueError(f'unrepresented field {key}: {field}')
            cursor = max(cursor, offset + width)
        if not union and cursor < record['size']:
            output.append(f' {level:02} filler pic x({record["size"]-cursor}).')
        return output
    for name, record in facts['records'].items():
        if not name.startswith(('thinkthen_complete_', 'thinkthen_cobol_')) or name.endswith('_data_v1'):
            # Union roots are represented in their owning structs.
            continue
        lines.append('01 ' + identifier(name) + ' based.')
        lines.extend(members(record, 2))
    for key, value in facts['constants'].items():
        lines.append('78 ' + identifier(key.lower() + '_constant') + ' value ' + str(value) + '.')
    return '\n'.join(lines) + '\n'


def requests(definitions):
    nodes = {key: (value["oneOf"][0] if isinstance(value, dict) and len(value.get("oneOf", [])) == 1 and "const" in value["oneOf"][0] else value) for key, value in definitions.items()}
    def type_for(source, path):
        if isinstance(source, dict) and '$ref' in source:
            return source['$ref'].removeprefix('#/$defs/')
        if path not in nodes:
            nodes[path] = source
        return path
    layouts, bodies = {}, {}
    pending = list(nodes)
    while pending:
        key = pending.pop(0)
        source = nodes[key]
        if isinstance(source, bool):
            source = {}
            nodes[key] = source
        name = 'thinkthen_cobol_' + key
        fields, body = [], []
        def child(field, path):
            previous = set(nodes)
            target = type_for(field, path)
            pending.extend(set(nodes) - previous)
            return target
        if isinstance(source, dict) and '$ref' in source:
            target = source['$ref'].removeprefix('#/$defs/')
            fields = [('const void *', 'value')]
            body = [f'emit_{target}(w, p->value);']
        elif source.get('variants'):
            fields = [('uint64_t', 'kind'), ('const void *', 'value')]
            body = ['switch(p->kind) {']
            for index, (target, tag) in enumerate(source['variants'], 1):
                body.append(f'case {index}: emit_{target}(w, p->value); break;')
            body += ['default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;', '}']
        elif 'properties' in source:
            fields = [('uint64_t', 'reserved')]
            body = ['int comma=0; raw(w,"{");']
            for member, field in source['properties'].items():
                target = child(field, key + '_member_' + member)
                fields.append(('const void *', 'm_' + member))
                resolved = nodes[target]
                # Literal members come from Rust, and require no caller field.
                if isinstance(resolved, dict) and 'const' in resolved:
                    condition = '1'
                else:
                    condition = 'p->m_' + member
                body += [f'if({condition}) {{ if(comma++) raw(w,","); raw(w,{json.dumps(json.dumps(member)+":")}); emit_{target}(w,p->m_{member}); }}']
            body += ['raw(w,"}");']
        elif source.get('type') == 'array':
            target = child(source.get('items', {}), key + '_item')
            fields = [('const void *const *', 'data'), ('uint64_t', 'len')]
            body = ['raw(w,"[");', 'if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}', f'for(uint64_t i=0;i<p->len && !w->code;i++) {{ if(i) raw(w,","); emit_{target}(w,p->data[i]); }}', 'raw(w,"]");']
        elif source.get('type') == 'object' and isinstance(source.get('additionalProperties'), dict):
            target = child(source['additionalProperties'], key + '_entry')
            fields = [('const thinkthen_cobol_text *', 'keys'), ('const void *const *', 'values'), ('uint64_t', 'len')]
            body = ['raw(w,"{");', 'if(p->len && (!p->keys || !p->values)) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}', f'for(uint64_t i=0;i<p->len && !w->code;i++) {{ if(i) raw(w,","); text(w,p->keys[i].data,p->keys[i].len,1); raw(w,":"); emit_{target}(w,p->values[i]); }}', 'raw(w,"}");']
        elif 'const' in source:
            fields = [('uint64_t', 'reserved')]
            body = ['(void)p;', 'raw(w,' + json.dumps(json.dumps(source['const'])) + ');']
        elif source.get('primitive_variants') or source.get('oneOf') or source.get('anyOf') or isinstance(source.get('type'), list):
            alternatives = source.get('oneOf', source.get('anyOf', []))
            if 'primitive_variants' in source:
                alternatives = [source['primitive_schemas'][k] for k in source['primitive_variants']]
            if isinstance(source.get('type'), list):
                alternatives = [{**source, 'type': k} for k in source['type']]
            fields = [('uint64_t', 'kind'), ('const void *', 'value')]
            body = ['switch(p->kind) {']
            for index, alternative in enumerate(alternatives, 1):
                target = child(alternative, key + '_arm_' + str(index))
                body.append(f'case {index}: emit_{target}(w,p->value); break;')
            body += ['default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;', '}']
        elif source.get('type') == 'string':
            fields = [('const char *', 'data'), ('uint64_t', 'len')]
            body = ['text(w,p->data,p->len,1);']
        elif source.get('type') in ('integer', 'number', 'boolean'):
            kind = source['type']
            native = 'double' if kind == 'number' else 'uint64_t' if kind == 'boolean' or source.get('format','').startswith('u') else 'int64_t'
            fields = [(native, 'value')]
            body = ['char bytes[64];'] if kind != 'boolean' else []
            if kind == 'number':
                body += ['if(!isfinite(p->value)) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}', 'snprintf(bytes,sizeof(bytes),"%.17g",p->value); raw(w,bytes);']
            elif kind == 'boolean':
                body += ['raw(w,p->value ? "true" : "false");']
            else:
                fmt, cast = ('%llu', 'unsigned long long') if native == 'uint64_t' else ('%lld', 'long long')
                body += [f'snprintf(bytes,sizeof(bytes),"{fmt}",({cast})p->value); raw(w,bytes);']
        elif source.get('type') == 'null':
            fields = [('uint64_t', 'reserved')]
            body = ['(void)p; raw(w,"null");']
        else:
            # Arbitrary JSON evidence is data. Rust alone decodes/admit it.
            fields = [('const char *', 'data'), ('uint64_t', 'len')]
            body = ['text(w,p->data,p->len,0);']
        layouts[name] = fields
        bodies[key] = body
    header = ['/* Generated from the canonical Request graph; do not edit. */', '#ifndef TT_COBOL_REQUESTS_GENERATED_H', '#define TT_COBOL_REQUESTS_GENERATED_H', '#include <stdint.h>', '#define THINKTHEN_COBOL_REPRESENTATION 1001', '#define THINKTHEN_COBOL_OVERFLOW 1002', '#define THINKTHEN_COBOL_TEXT_CAPACITY 8192', 'typedef struct thinkthen_cobol_text {const char *data; uint64_t len;} thinkthen_cobol_text;']
    for key, source in nodes.items():
        if source.get('variants'):
            for index, (target, tag) in enumerate(source['variants'], 1):
                header.append(f'#define THINKTHEN_COBOL_{target.upper()}_KIND {index}')
        elif source.get('primitive_variants') or source.get('oneOf') or source.get('anyOf') or isinstance(source.get('type'), list):
            count = len(source.get('primitive_variants', source.get('oneOf', source.get('anyOf', source.get('type', [])))))
            for index in range(1, count + 1):
                header.append(f'#define THINKTHEN_COBOL_{key.upper()}_ARM_{index}_KIND {index}')
    for name, fields in layouts.items():
        header += ['typedef struct ' + name + ' {'] + [' '+kind+' '+member+';' for kind, member in fields] + ['} '+name+';']
    named = definitions['RequestCall']['variants']
    for index, (target, (_, _, function)) in enumerate(named, 1):
        header.append(f'int TT_SESSION_{function.upper()}(const void *,const thinkthen_cobol_{target} *,void **);')
    header += ['int TT_SESSION_PUSH(const void *,const thinkthen_cobol_RequestSessionDescriptor *,uint32_t *);', 'int TT_SESSION_REQUEST(const void *engine,const thinkthen_cobol_Request *request,void **out);', '#endif']
    code = [SUPPORT]
    for key in bodies:
        code.append('static inline void emit_' + key + '(writer *,const void *);')
    for key, body in bodies.items():
        name = 'thinkthen_cobol_' + key
        code.append(f'static inline void emit_{key}(writer *w,const void *input) {{ const {name} *p=input;')
        if not ('const' in nodes[key] or nodes[key].get('type') == 'null'):
            code.append('if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}')
        code.extend(body)
        code.append('}')
    code += ['int TT_SESSION_REQUEST(const void *engine,const thinkthen_cobol_Request *request,void **out) {', 'writer w={0}; int encoded=encode(&w,request,emit_Request);', 'int code=encoded ? encoded : thinkthen_session_new_with_surface(engine,w.data,w.len,"cobol",5,(thinkthen_session **)out);', 'free(w.data); return code;', '}']
    code += ['int TT_SESSION_PUSH(const void *session,const thinkthen_cobol_RequestSessionDescriptor *descriptor,uint32_t *status) {', 'writer w={0}; int encoded=encode(&w,descriptor,emit_RequestSessionDescriptor);', 'int code=encoded ? encoded : thinkthen_session_try_push((thinkthen_session *)session,w.data,w.len,status);', 'free(w.data); return code;', '}']
    for index, (target, (_, _, function)) in enumerate(named, 1):
        code += [f'int TT_SESSION_{function.upper()}(const void *engine,const thinkthen_cobol_{target} *input,void **out) {{', f'thinkthen_cobol_RequestCall call={{.kind={index},.value=input}};', 'thinkthen_cobol_Request request={.m_call=&call};', 'return TT_SESSION_REQUEST(engine,&request,out);', '}']
    return '\n'.join(header)+'\n', '\n'.join(code)+'\n'


def outputs(root, definitions):
    header, serializer = requests(definitions)
    package = root / 'libraries/cobol'
    # Measure generated Request declarations with the same compiler as native ABI.
    import tempfile
    with tempfile.TemporaryDirectory(prefix='cobol-layout-') as folder:
        path = Path(folder) / 'requests.h'
        # The ABI probe requires public functions and constants as well as records.
        path.write_text('#include ' + json.dumps(str(root / 'libraries/c/include/thinkthen.h')) + '\n' + header)
        facts = abi_module(root).header_abi(path)
    all_copybooks = copybook(facts)
    return {'src/tt_requests_generated.h': header, 'src/tt_requests_generated.c': serializer,
            'copybooks/tt-native-generated.cpy': all_copybooks}


SUPPORT = r'''/* Generated mechanical Request conversion; Rust owns admission. */
#define _POSIX_C_SOURCE 200809L
#include "tt_requests_generated.h"
#include "thinkthen.h"
#include <locale.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
typedef struct {char *data; size_t len, capacity; int code;} writer;
static void append(writer *w,const char *data,size_t len) {
 if(w->code) return;
 if(len>SIZE_MAX-w->len-1) {w->code=THINKTHEN_COBOL_OVERFLOW; return;}
 size_t needed=w->len+len+1;
 if(needed>w->capacity) {char *next=realloc(w->data,needed); if(!next) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;} w->data=next; w->capacity=needed;}
 if(len) memcpy(w->data+w->len,data,len);
 w->len+=len; w->data[w->len]=0;
}
static void raw(writer *w,const char *value) {append(w,value,strlen(value));}
static void text(writer *w,const char *data,uint64_t len,int quote) {
 if(len>THINKTHEN_COBOL_TEXT_CAPACITY) {w->code=THINKTHEN_COBOL_OVERFLOW; return;}
 if(len && !data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
 if(!quote) {append(w,data,(size_t)len); return;}
 raw(w,"\"");
 for(uint64_t i=0;i<len && !w->code;i++) {
  unsigned char c=(unsigned char)data[i];
  if(c=='"' || c=='\\') {char escaped[2]={'\\',(char)c}; append(w,escaped,2);}
  else if(c<32) {char escaped[7]; snprintf(escaped,sizeof(escaped),"\\u%04x",c); raw(w,escaped);}
  else append(w,data+i,1);
 }
 raw(w,"\"");
}
static int encode(writer *w,const void *input,void (*convert)(writer *,const void *)) {
 locale_t locale=newlocale(LC_NUMERIC_MASK,"C",(locale_t)0);
 if(!locale) return THINKTHEN_COBOL_REPRESENTATION;
 locale_t previous=uselocale(locale);
 if(!previous) {freelocale(locale); return THINKTHEN_COBOL_REPRESENTATION;}
 convert(w,input);
 uselocale(previous); freelocale(locale);
 return w->code;
}
'''
