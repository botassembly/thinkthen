"""Generate the flat COBOL preview from all canonical Plan fields."""
import json


def render(root):
    definitions = json.loads((root / 'specification/result.schema.json').read_text())['$defs']
    plan = definitions['plan']
    header = ['typedef struct thinkthen_cobol_AnswerTokenBand {uint64_t lower; uint64_t upper;} thinkthen_cobol_AnswerTokenBand;', 'typedef struct thinkthen_cobol_AnswerPlan {']
    body = ['thinkthen_cobol_AnswerPlan value={0};', 'int code=0;']
    for member, schema in plan['properties'].items():
        access = f'field_value(owner,"{member}",{len(member)})'
        kind = schema.get('type')
        if kind == 'integer':
            header.append('uint64_t ' + member + ';')
            body.append(f'if(!code) code=plan_unsigned({access},&value.{member});')
        elif kind == 'boolean':
            header.append('uint64_t ' + member + ';')
            body += [f'if(!code) {{const thinkthen_complete_json_v1 *field={access};', 'if(!field || field->kind!=THINKTHEN_COMPLETE_JSON_BOOLEAN_V1) code=THINKTHEN_COBOL_REPRESENTATION;', f'else value.{member}=field->data.boolean;}}']
        elif kind == 'string':
            header.append('thinkthen_cobol_AnswerText ' + member + ';')
            body.append(f'if(!code) code=plan_text({access},&value.{member});')
        elif kind == ['string', 'null']:
            header += ['uint64_t ' + member + '_presence;', 'thinkthen_cobol_AnswerText ' + member + ';']
            body += [f'if(!code) {{const thinkthen_complete_json_v1 *field={access};', f'value.{member}_presence=!field ? THINKTHEN_COMPLETE_PRESENCE_MISSING_V1 : field->kind==THINKTHEN_COMPLETE_JSON_NULL_V1 ? THINKTHEN_COMPLETE_PRESENCE_NULL_V1 : THINKTHEN_COMPLETE_PRESENCE_VALUE_V1;', f'if(value.{member}_presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) code=plan_text(field,&value.{member});}}']
        elif schema.get('$ref') == '#/$defs/tokenBand':
            header.append('thinkthen_cobol_AnswerTokenBand ' + member + ';')
            body += [f'if(!code) {{const thinkthen_complete_json_v1 *field={access};', 'tt_value view={.value=(thinkthen_complete_json_v1 *)field};', 'if(!field || field->kind!=THINKTHEN_COMPLETE_JSON_OBJECT_V1) code=THINKTHEN_COBOL_REPRESENTATION;']
            for key in definitions['tokenBand']['properties']:
                body.append(f'if(!code) code=plan_unsigned(field_value(&view,"{key}",{len(key)}),&value.{member}.{key});')
            body.append('}')
        else:
            raise ValueError('unsupported Plan field: ' + member)
    header += ['} thinkthen_cobol_AnswerPlan;']
    signature = 'int TT_PREVIEW(const void *engine,const char *function,const uint64_t *length,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerPlan *out,thinkthen_cobol_AnswerStatus *status_out)'
    code = SUPPORT + signature + ''' {
    if(!length || !out || !status_out) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_RequestCall call={.m_question=question,.m_input=input,.m_options=options};
    tt_value *owner=NULL;
    int result=TT_PLAN(engine,function,*length,&call,&owner);
    if(result) return capture_immediate_error(result,status_out);
''' + '\n'.join(body) + '''
    TT_VALUE_FREE(owner);
    *status_out=(thinkthen_cobol_AnswerStatus){.code=code};
    if(!code) *out=value;
    return code;
}
'''
    return '\n'.join(header), signature + ';', code


SUPPORT = r'''
static int plan_unsigned(const thinkthen_complete_json_v1 *value,uint64_t *out) {
    if(!value || value->kind!=THINKTHEN_COMPLETE_JSON_NUMBER_V1 || !value->data.number.len || !value->data.number.data) return THINKTHEN_COBOL_REPRESENTATION;
    uint64_t number=0;
    for(size_t i=0;i<value->data.number.len;i++) {
        unsigned char digit=(unsigned char)value->data.number.data[i];
        if(digit<'0' || digit>'9') return THINKTHEN_COBOL_REPRESENTATION;
        uint64_t add=digit-'0';
        if(number>(UINT64_MAX-add)/10) return THINKTHEN_COBOL_OVERFLOW;
        number=number*10+add;
    }
    *out=number; return 0;
}
static int plan_text(const thinkthen_complete_json_v1 *value,thinkthen_cobol_AnswerText *out) {
    if(!value || value->kind!=THINKTHEN_COMPLETE_JSON_STRING_V1) return THINKTHEN_COBOL_REPRESENTATION;
    return answer_text(value->data.string,out);
}
'''
