"""Adapt existing COBOL carriers to the canonical Rust request ABI."""
from templates.native_request import render as native_render, name
from templates.csharp import enum_values


def render(definitions, old_nodes):
    metadata = {}
    native_render(definitions, metadata)
    nodes = metadata['nodes']
    pending, bodies, done = [('RequestCall', 'RequestArguments'), ('RequestSessionDescriptor', 'RequestSessionDescriptor')], [], set()
    def converter(old, native):
        identity = old, native
        if identity not in done and identity not in pending:
            pending.append(identity)
        return 'convert_' + old + '_' + native
    while pending:
        old, native = pending.pop(0)
        if (old, native) in done:
            continue
        done.add((old, native))
        source = old_nodes[old]
        typ = name(native)
        body = []
        def child(value, path, old_path):
            old_key = value['$ref'].removeprefix('#/$defs/') if isinstance(value, dict) and '$ref' in value else old_path
            return converter(old_key, metadata['children'][(native, path)])
        if '$ref' in source:
            target = source['$ref'].removeprefix('#/$defs/')
            body = ['return ' + converter(target, target) + '(a,p->value);']
        elif 'properties' in source:
            for member, value in source['properties'].items():
                if old == 'RequestCall' and member == 'function':
                    continue
                resolved = old_nodes[value['$ref'].removeprefix('#/$defs/')] if isinstance(value, dict) and '$ref' in value else value
                if isinstance(resolved, dict) and 'const' in resolved:
                    continue
                call = child(value, native + '_' + member, old + '_member_' + member)
                body.append(f'if(p->m_{member}) out->m_{member}={call}(a,p->m_{member});')
        elif source.get('variants') or source.get('primitive_variants') or source.get('oneOf') or source.get('anyOf') or isinstance(source.get('type'), list):
            alternatives = source.get('oneOf', source.get('anyOf', []))
            if source.get('variants'):
                alternatives = [{'$ref':'#/$defs/' + target} for target, _ in source['variants']]
            elif source.get('primitive_variants'):
                alternatives = [source['primitive_schemas'][kind] for kind in source['primitive_variants']]
            elif isinstance(source.get('type'), list):
                alternatives = [{**source, 'type':kind} for kind in source['type']]
            body = ['out->kind=(uint32_t)p->kind;', 'switch(p->kind) {']
            for index, alternative in enumerate(alternatives, 1):
                call = child(alternative, native + '_arm_' + str(index), old + '_arm_' + str(index))
                body.append(f'case {index}: out->data.arm_{index}={call}(a,p->value); break;')
            body += ['default: a->code=THINKTHEN_COBOL_REPRESENTATION; break;', '}']
        elif source.get('type') == 'array':
            call = child(source.get('items', {}), native + '_item', old + '_item')
            body = ['if(p->len && !p->data) {a->code=THINKTHEN_COBOL_REPRESENTATION; return out;}',
                    'void **items=allocate(a,p->len,sizeof(*items));', 'out->data=(const void *)items; out->len=p->len;',
                    f'if(items) for(uint64_t i=0;i<p->len && !a->code;i++) items[i]={call}(a,p->data[i]);']
        elif source.get('type') == 'object':
            call = child(source.get('additionalProperties') if isinstance(source.get('additionalProperties'), dict) else {}, native + '_value', old + '_entry')
            entry = typ.removesuffix('_v1') + '_entry_v1'
            body = ['if(p->len && (!p->keys || !p->values)) {a->code=THINKTHEN_COBOL_REPRESENTATION; return out;}',
                    f'{entry} *items=allocate(a,p->len,sizeof(*items));', 'out->data=items; out->len=p->len;',
                    f'if(items) for(uint64_t i=0;i<p->len && !a->code;i++) {{items[i].key=(thinkthen_complete_utf8_v1){{p->keys[i].data,p->keys[i].len}}; items[i].value={call}(a,p->values[i]);}}']
        elif 'const' in source or source.get('type') == 'null':
            body = ['(void)p;']
        elif source.get('type') in ('integer','number','boolean'):
            if source['type'] == 'boolean':
                body = ['if(p->value>1) a->code=THINKTHEN_COBOL_REPRESENTATION;', 'out->value=(uint32_t)p->value;']
            else:
                body = ['out->value=p->value;']
        else:
            body = ['out->text=(thinkthen_complete_utf8_v1){p->data,p->len};']
        const = 'const' in source or source.get('type') == 'null'
        function = 'convert_' + old + '_' + native
        prefix = [f'static void *{function}(arena *a,const void *input) {{', f'const thinkthen_cobol_{old} *p=input;', f'{typ} *out=allocate(a,1,sizeof(*out));', 'if(!out) return NULL;']
        if not const:
            prefix += ['if(!p) {a->code=THINKTHEN_COBOL_REPRESENTATION; return out;}']
        bodies.append((function, '\n'.join(prefix + body + ['return out;', '}'])))
    code = [SUPPORT]
    code.extend('static void *' + function + '(arena *,const void *);' for function, _ in bodies)
    code.extend(body for _, body in bodies)
    code += ['static int start(const void *engine,const thinkthen_cobol_RequestCall *input,void **out,const char *function,size_t length) {',
             'if(!input) return THINKTHEN_COBOL_REPRESENTATION;', 'arena a={0};',
             'const thinkthen_request_arguments_v1 *arguments=convert_RequestCall_RequestArguments(&a,input);',
             'int code=a.code ? a.code : thinkthen_session_new_typed_with_surface(engine,(thinkthen_complete_utf8_v1){function,length},arguments,(thinkthen_complete_utf8_v1){"cobol",5},(thinkthen_session **)out);',
             'release(&a); return code;', '}',
             'int TT_SESSION_REQUEST(const void *engine,const thinkthen_cobol_Request *request,void **out) {',
             'if(!request || !request->m_call) return THINKTHEN_COBOL_REPRESENTATION;',
             'const thinkthen_cobol_RequestCall *call=request->m_call;',
             'const thinkthen_cobol_RequestFunction *function=call->m_function;',
             'if(!function) return THINKTHEN_COBOL_REPRESENTATION;', 'switch(function->kind) {']
    functions = enum_values(definitions['RequestFunction'])
    for index, function in enumerate(functions, 1):
        code.append(f'case {index}: return start(engine,call,out,"{function}",{len(function)});')
    code += ['default: return THINKTHEN_COBOL_REPRESENTATION;', '}', '}',
             'int TT_SESSION_PUSH(const void *session,const thinkthen_cobol_RequestSessionDescriptor *input,uint32_t *status) {',
             'arena a={0}; const thinkthen_request_session_descriptor_v1 *descriptor=convert_RequestSessionDescriptor_RequestSessionDescriptor(&a,input);',
             'int code=a.code ? a.code : thinkthen_session_push_typed(session,descriptor,status);',
             'release(&a); return code;', '}']
    for function in functions:
        code += [f'int TT_SESSION_{function.upper()}(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {{', f'return start(engine,input,out,"{function}",{len(function)});', '}']
    code += [PLAN]
    code += [NEXT_PACKET, USAGE_STATUS]
    return '\n'.join(code) + '\n'


SUPPORT = '''/* Generated counted-carrier adaptation; Rust owns request admission. */
#include "tt_session.h"
#include "thinkthen.h"
#include <stdlib.h>
#include <stdint.h>
typedef struct allocation {struct allocation *next; void *memory;} allocation;
typedef struct {allocation *head; int code;} arena;
static void *allocate(arena *a,size_t count,size_t width) {
    if(a->code) return NULL;
    if(width && count>SIZE_MAX/width) {a->code=THINKTHEN_COBOL_REPRESENTATION; return NULL;}
    allocation *item=malloc(sizeof(*item));
    void *memory=calloc(count ? count : 1,width);
    if(!item || !memory) {free(item); free(memory); a->code=THINKTHEN_ELOCAL; return NULL;}
    *item=(allocation){a->head,memory}; a->head=item; return memory;
}
static void release(arena *a) {
    while(a->head) {allocation *item=a->head; a->head=item->next; free(item->memory); free(item);}
}
'''

NEXT_PACKET = 'int TT_SESSION_NEXT(thinkthen_session *session,uint32_t *status,thinkthen_session_result **out) {\n    return thinkthen_session_next_packet(session,THINKTHEN_SESSION_WAIT_SETTLEMENT_V1,status,out);\n}\n'

USAGE_STATUS = 'static int usage_status(const thinkthen_engine *engine,thinkthen_complete_usage_persistence_v1 *state,\n                        char *advice,uint64_t *written,int finish) {\n    if(!state || !advice || !written) return THINKTHEN_COBOL_REPRESENTATION;\n    thinkthen_complete_usage_persistence_v1 observed;\n    thinkthen_complete_utf8_v1 text;\n    int code=finish ? thinkthen_engine_finish_usage_status_v1(engine,&observed,&text)\n                    : thinkthen_engine_usage_persistence_v1(engine,&observed,&text);\n    if(code) return code;\n    code=TT_SESSION_TEXT(&text,advice,written);\n    if(code) return code;\n    *state=observed;\n    return 0;\n}\nint TT_ENGINE_USAGE_PERSISTENCE(const thinkthen_engine *engine,thinkthen_complete_usage_persistence_v1 *state,\n                              char *advice,uint64_t *written) {\n    return usage_status(engine,state,advice,written,0);\n}\nint TT_ENGINE_FINISH_USAGE_STATUS(const thinkthen_engine *engine,thinkthen_complete_usage_persistence_v1 *state,\n                                char *advice,uint64_t *written) {\n    return usage_status(engine,state,advice,written,1);\n}\n'


PLAN = r'''int TT_PLAN(const void *engine,const char *function,uint64_t length,
            const thinkthen_cobol_RequestCall *input,char **out,uint64_t *written) {
    arena a={0};
    const thinkthen_request_arguments_v1 *arguments=convert_RequestCall_RequestArguments(&a,input);
    int code=a.code ? a.code : thinkthen_request_plan_typed_json(engine,
        (thinkthen_complete_utf8_v1){function,length},arguments,
        (thinkthen_complete_utf8_v1){"cobol",5},out,written);
    release(&a); return code;
}
'''


def builders(definitions):
    """Fill existing caller-owned typed records without admission or allocation."""
    declarations, bodies = [], []
    for index, (branch, _) in enumerate(definitions['RequestQuestion']['variants'], 1):
        kind = branch.removeprefix('RequestQuestion_')
        if kind == 'definition':
            continue
        member = next(key for key in definitions[branch]['properties'] if key != 'kind')
        scalar = branch + '_member_' + member
        signature = f'int TT_QUESTION_{kind.upper()}(thinkthen_cobol_RequestQuestion *selector,thinkthen_cobol_{branch} *arm,thinkthen_cobol_{scalar} *scalar,const char *data,uint64_t length)'
        declarations.append(signature + ';')
        bodies += [signature + ' {', 'if(!selector || !arm || !scalar) return THINKTHEN_COBOL_REPRESENTATION;',
                   f'*scalar=(thinkthen_cobol_{scalar}){{data,length}};',
                   f'*arm=(thinkthen_cobol_{branch}){{.m_{member}=scalar}};',
                   f'*selector=(thinkthen_cobol_RequestQuestion){{.kind={index},.value=arm}};', 'return 0;', '}']
    for index, (branch, _) in enumerate(definitions['RequestDefinition']['variants'], 1):
        for function in ('recognize','relate'):
            if function not in definitions[branch].get('properties', {}):
                continue
            definition_index = next(i for i, (key, _) in enumerate(definitions['RequestQuestion']['variants'], 1) if key == 'RequestQuestion_definition')
            signature = f'int TT_QUESTION_{function.upper()}(thinkthen_cobol_RequestQuestion *selector,thinkthen_cobol_RequestQuestion_definition *arm,thinkthen_cobol_RequestDefinition *definition,const thinkthen_cobol_{branch} *value)'
            declarations.append(signature + ';')
            bodies += [signature + ' {', 'if(!selector || !arm || !definition || !value) return THINKTHEN_COBOL_REPRESENTATION;',
                       f'*definition=(thinkthen_cobol_RequestDefinition){{.kind={index},.value=value}};',
                       '*arm=(thinkthen_cobol_RequestQuestion_definition){.m_value=definition};',
                       f'*selector=(thinkthen_cobol_RequestQuestion){{.kind={definition_index},.value=arm}};', 'return 0;', '}']
    for original_index, (branch, _) in enumerate(definitions['RequestOriginal']['variants'], 1):
        kind = branch.removeprefix('RequestOriginal_')
        member = 'text' if kind == 'text' else 'value'
        scalar_key = branch + '_member_' + member
        signature = f'int TT_{kind.upper()}_ITEM(thinkthen_cobol_RequestItem *item,thinkthen_cobol_RequestOriginal *original,thinkthen_cobol_{branch} *arm,thinkthen_cobol_{scalar_key} *scalar,const char *data,uint64_t length)'
        declarations.append(signature + ';')
        bodies += [signature + ' {', 'if(!item || !original || !arm || !scalar) return THINKTHEN_COBOL_REPRESENTATION;',
                   f'*scalar=(thinkthen_cobol_{scalar_key}){{data,length}};',
                   f'*arm=(thinkthen_cobol_{branch}){{.m_{member}=scalar}};',
                   f'*original=(thinkthen_cobol_RequestOriginal){{.kind={original_index},.value=arm}};',
                   '*item=(thinkthen_cobol_RequestItem){.m_original=original};', 'return 0;', '}']
    source_index = next(i for i, (key, _) in enumerate(definitions['RequestInput']['variants'], 1) if key == 'RequestInput_source')
    source_key = definitions['RequestInput_source']['properties']['source']['$ref'].removeprefix('#/$defs/')
    list_key = source_key + '_member_paths'
    scalar_key = list_key + '_item'
    signature = f'int TT_FILE_INPUT(thinkthen_cobol_RequestInput *selector,thinkthen_cobol_RequestInput_source *arm,thinkthen_cobol_{source_key} *source,thinkthen_cobol_{list_key} *paths,thinkthen_cobol_{scalar_key} *path,const void **slots,const char *data,uint64_t length)'
    declarations.append(signature + ';')
    bodies += [signature + ' {', 'if(!selector || !arm || !source || !paths || !path || !slots) return THINKTHEN_COBOL_REPRESENTATION;',
               f'*path=(thinkthen_cobol_{scalar_key}){{data,length}}; slots[0]=path;',
               f'*paths=(thinkthen_cobol_{list_key}){{.data=slots,.len=1}};',
               f'*source=(thinkthen_cobol_{source_key}){{.m_paths=paths}};',
               '*arm=(thinkthen_cobol_RequestInput_source){.m_source=source};',
               f'*selector=(thinkthen_cobol_RequestInput){{.kind={source_index},.value=arm}};', 'return 0;', '}']
    return declarations, '\n'.join(bodies)
