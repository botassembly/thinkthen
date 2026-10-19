"""Generate owned receipt and numeric projections for the COBOL caller."""
import re

HEADER = '''typedef struct thinkthen_cobol_AnswerNumber {uint64_t present; double value;} thinkthen_cobol_AnswerNumber;
typedef struct thinkthen_cobol_AnswerProbabilityList {const thinkthen_cobol_AnswerNumber *data; uint64_t count;} thinkthen_cobol_AnswerProbabilityList;
typedef struct thinkthen_cobol_AnswerProbability {uint64_t scalar; thinkthen_cobol_AnswerNumber first; const thinkthen_cobol_AnswerNumber *data; uint64_t count; const thinkthen_cobol_AnswerProbabilityList *lists; uint64_t list_count;} thinkthen_cobol_AnswerProbability;
'''


def render(facts, root, functions):
    from templates.cobol_facts import render as record_render
    from generate import graph, prepare
    import json
    from templates.host_answers import call_results
    schema = json.loads((root / 'crates/thinkthen/src/public/results/complete.schema.json').read_text())
    contracts = call_results(prepare(graph(schema, ('completesessionPacket',))))
    records = facts['records']
    def base(kind):
        return re.sub(r'\b(const|struct|union)\b|\*', '', kind).strip()
    def owned(kind):
        return 'thinkthen_cobol_Receipt' + ''.join(word.title() for word in kind.removeprefix('thinkthen_complete_').removesuffix('_v1').split('_'))
    payloads, roots = {}, []
    for function in functions:
        contract = contracts[function]
        if contract['probability_mode'] not in ('yes', 'picked', 'items', 'null'):
            raise ValueError('unknown canonical probability projection')
        aggregate = 'thinkthen_complete_session_packet_' + function + '_aggregate_v1'
        payload = base(records[aggregate]['fields']['value']['type'])
        roots.append(payload)
        is_list = 'data' in records[payload]['fields'] and 'len' in records[payload]['fields']
        item = base(records[payload]['fields']['data']['type']) if is_list else payload
        payloads[function] = (payload, item, is_list)
    header, source = record_render(facts, roots, 'Receipt')
    header = HEADER + header
    declarations = {}
    for function, (payload, item, is_list) in payloads.items():
        name = owned(payload) if is_list else owned(payload) + 'List'
        if not is_list:
            header += f'\ntypedef struct {name} {{const {owned(item)} *data; uint64_t len;}} {name};\n'
        declarations[function] = name
    source += '\n' + SUPPORT
    source += '\nstatic int collect_receipts(tt_answer_owner *owner,const thinkthen_complete_session_packet_v1 *packet,uint32_t function) {\narena *a=&owner->storage; int code=0;\nswitch(function) {\n'
    for function, (_, item, is_list) in payloads.items():
        constant = 'THINKTHEN_COMPLETE_REQUEST_FUNCTION_' + function.upper() + '_V1'
        agg = 'THINKTHEN_COMPLETE_SESSION_PACKET_' + function.upper() + '_AGGREGATE_V1'
        member = function + '_aggregate'
        source += f'case {constant}: {{\n'
        if is_list:
            source += f'const {item} *const *items=NULL; size_t count=0;\n'
            if 'thinkthen_complete_session_packet_' + function + '_row_v1' in records:
                source += f'const {item} *single=NULL;\n'
            source += f'if(packet->kind=={agg}) {{items=packet->data.{member}->value.data; count=packet->data.{member}->value.len;}}\n'
            row_packet = 'thinkthen_complete_session_packet_' + function + '_row_v1'
            if row_packet in records:
                source += f'else if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_{function.upper()}_ROW_V1) {{single=packet->data.{function}_row->value; items=&single; count=1;}}\n'
            source += 'else return 0;\n'
        else:
            source += f'if(packet->kind!={agg}) return 0;\nconst {item} *single=packet->data.{member}->value; const {item} *const *items=&single; size_t count=1;\n'
        source += f'{owned(item)} *rows=allocate(a,count,sizeof(*rows));\nif(!rows) return a->code;\n'
        source += f'for(size_t i=0;i<count && !code;i++) code=copy_{owned(item)}(a,items[i],rows+i);\n'
        source += 'if(code) return code;\n'
        source += 'code=append_receipts(owner,rows,count,sizeof(*rows)); if(code) return code;\n'
        mode = contracts[function]['probability_mode']
        if mode in ('yes', 'picked') or function == 'rank':
            source += 'thinkthen_cobol_AnswerNumber *numbers=allocate(a,count,sizeof(*numbers)); if(!numbers) return a->code;\nfor(size_t i=0;i<count;i++) {\n'
            if function == 'find':
                source += 'if(items[i]->index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) numbers[i]=picked_number(items[i]->answer->probabilities,items[i]->answer->pick);\n'
            else:
                source += 'numbers[i]=answer_number(items[i]->answer);\n'
                if function in ('decide', 'choose'):
                    source += 'uint64_t index=items[i]->index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? items[i]->index.value : owner->receipt_count-count+i;\nif(index>=owner->resolved_count || !owner->resolved[index]) numbers[i].present=0;\n'
            source += '}\ncode=append_numbers(owner,numbers,count);\n'
        elif function == 'recognize':
            source += 'for(size_t i=0;i<count && !code;i++) {\nconst thinkthen_complete_recognize_v1 *value=items[i]->value;\nsize_t length=value->kind==THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES_V1 ? value->data.entities->entities.len : value->data.mode_proposals->proposals.len;\nthinkthen_cobol_AnswerNumber *numbers=allocate(a,length,sizeof(*numbers)); if(!numbers) return a->code;\nfor(size_t j=0;j<length;j++) {\nif(value->kind==THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES_V1) numbers[j]=recognition_number(items[i],value->data.entities->entities.data[j]);\nelse numbers[j]=(thinkthen_cobol_AnswerNumber){1,value->data.mode_proposals->proposals.data[j]->probability};\n}\ncode=append_probability_list(owner,numbers,length);\n}\n'
        elif function == 'relate':
            source += 'size_t length=single->value.len; thinkthen_cobol_AnswerNumber *numbers=allocate(a,length,sizeof(*numbers)); if(!numbers) return a->code;\nfor(size_t i=0;i<length;i++) numbers[i]=(thinkthen_cobol_AnswerNumber){1,single->value.data[i]->probability};\ncode=append_numbers(owner,numbers,length);\n'
        source += 'break;\n}\n'
    source += 'default: return THINKTHEN_COBOL_REPRESENTATION;\n}\nreturn code ? code : a->code;\n}\n'
    return header, source, declarations, payloads


SUPPORT = r'''
static int append_receipts(tt_answer_owner *owner,const void *rows,size_t count,size_t width) {
    if(count>SIZE_MAX/width-owner->receipt_count) return THINKTHEN_COBOL_OVERFLOW;
    size_t total=owner->receipt_count+count;
    void *data=realloc(owner->receipts,total*width);
    if(total && !data) return THINKTHEN_ELOCAL;
    owner->receipts=data;
    if(count) memcpy((char *)data+owner->receipt_count*width,rows,count*width);
    owner->receipt_count=total; return 0;
}
static int append_numbers(tt_answer_owner *owner,const thinkthen_cobol_AnswerNumber *rows,size_t count) {
    if(count>SIZE_MAX/sizeof(*rows)-owner->probability_count) return THINKTHEN_COBOL_OVERFLOW;
    size_t total=owner->probability_count+count;
    void *data=realloc(owner->probabilities,total*sizeof(*rows));
    if(total && !data) return THINKTHEN_ELOCAL;
    owner->probabilities=data;
    if(count) memcpy(owner->probabilities+owner->probability_count,rows,count*sizeof(*rows));
    owner->probability_count=total; return 0;
}
static int append_probability_list(tt_answer_owner *owner,const thinkthen_cobol_AnswerNumber *rows,size_t count) {
    if(owner->probability_list_count==SIZE_MAX/sizeof(*owner->probability_lists)) return THINKTHEN_COBOL_OVERFLOW;
    size_t total=owner->probability_list_count+1;
    void *data=realloc(owner->probability_lists,total*sizeof(*owner->probability_lists));
    if(!data) return THINKTHEN_ELOCAL;
    owner->probability_lists=data;
    owner->probability_lists[owner->probability_list_count++]=(thinkthen_cobol_AnswerProbabilityList){rows,count}; return 0;
}
static int observe_resolution(tt_answer_owner *owner,const thinkthen_complete_session_packet_v1 *packet) {
    if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_OBSERVATION_V1) return 0;
    const thinkthen_complete_session_observation_v1 *observation=packet->data.observation->value;
    if(observation->kind!=THINKTHEN_COMPLETE_SESSION_OBSERVATION_ROW_V1) return 0;
    const thinkthen_complete_session_observation_row_v1 *row=observation->data.row;
    if(row->value->kind!=THINKTHEN_COMPLETE_SESSION_OBSERVED_ROW_JUDGMENT_V1) return 0;
    const thinkthen_complete_session_judgment_v1 *judgment=row->value->data.judgment->value;
    uint32_t present=0;
    if(judgment->kind==THINKTHEN_COMPLETE_SESSION_JUDGMENT_DECISION_V1) present=judgment->data.decision->value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1;
    else if(judgment->kind==THINKTHEN_COMPLETE_SESSION_JUDGMENT_CHOICE_V1) present=judgment->data.choice->value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1;
    else return 0;
    if(row->index==SIZE_MAX) return THINKTHEN_COBOL_OVERFLOW;
    size_t count=row->index+1;
    if(count>owner->resolved_count) {
        void *data=realloc(owner->resolved,count);
        if(!data) return THINKTHEN_ELOCAL;
        owner->resolved=data; memset(owner->resolved+owner->resolved_count,0,count-owner->resolved_count); owner->resolved_count=count;
    }
    owner->resolved[row->index]=(unsigned char)present; return 0;
}
static thinkthen_cobol_AnswerNumber picked_number(thinkthen_complete_number_map_v1 probabilities,thinkthen_complete_utf8_v1 pick) {
    for(size_t i=0;i<probabilities.len;i++) if(probabilities.data[i].name.len==pick.len && (!pick.len || !memcmp(probabilities.data[i].name.data,pick.data,pick.len))) return (thinkthen_cobol_AnswerNumber){1,probabilities.data[i].value};
    return (thinkthen_cobol_AnswerNumber){0};
}
static thinkthen_cobol_AnswerNumber answer_number(const thinkthen_complete_answer_v1 *answer) {
    if(answer->kind==THINKTHEN_COMPLETE_ANSWER_YES_NO_V1) return (thinkthen_cobol_AnswerNumber){1,answer->data.yes_no->probability};
    if(answer->kind==THINKTHEN_COMPLETE_ANSWER_CHOICE_V1) return picked_number(answer->data.choice->probabilities,answer->data.choice->pick);
    return (thinkthen_cobol_AnswerNumber){0};
}
static thinkthen_cobol_AnswerNumber recognition_number(const thinkthen_complete_recognition_v1 *receipt,const thinkthen_complete_entity_v1 *entity) {
    if(receipt->answer->kind!=THINKTHEN_COMPLETE_RECOGNITION_ODDS_RECOGNIZE_ANSWER_V1) return (thinkthen_cobol_AnswerNumber){0};
    const thinkthen_complete_recognition_proposal_list_v1 proposals=receipt->answer->data.recognize_answer->proposals;
    for(size_t i=0;i<proposals.len;i++) {
        const thinkthen_complete_recognition_proposal_v1 *proposal=proposals.data[i];
        if(proposal->kept && proposal->selected.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 && proposal->selected.value->start==entity->start && proposal->selected.value->end==entity->end && proposal->kind.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 && proposal->kind.value.len==entity->kind.len && (!entity->kind.len || !memcmp(proposal->kind.value.data,entity->kind.data,entity->kind.len))) return (thinkthen_cobol_AnswerNumber){1,proposal->span_probability};
    }
    return (thinkthen_cobol_AnswerNumber){0};
}
'''
