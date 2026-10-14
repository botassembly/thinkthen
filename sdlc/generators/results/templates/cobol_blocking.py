"""Generate blocking COBOL projections from canonical native packet views."""

HEADER = r"""
typedef struct tt_answer_owner tt_answer_owner;
typedef struct thinkthen_cobol_AnswerText {uint64_t length; char text[8192];} thinkthen_cobol_AnswerText;
typedef struct thinkthen_cobol_AnswerDecision {uint64_t present; const tt_value *value;} thinkthen_cobol_AnswerDecision;
typedef struct thinkthen_cobol_AnswerChoice {uint64_t present; thinkthen_cobol_AnswerText value;} thinkthen_cobol_AnswerChoice;
typedef struct thinkthen_cobol_AnswerScore {double value;} thinkthen_cobol_AnswerScore;
typedef struct thinkthen_cobol_AnswerRankIndex {uint64_t value;} thinkthen_cobol_AnswerRankIndex;
typedef struct thinkthen_cobol_AnswerFilterOriginal {const tt_value *original;} thinkthen_cobol_AnswerFilterOriginal;
typedef struct thinkthen_cobol_AnswerFindSelection {uint64_t present; uint64_t index; const tt_value *original;} thinkthen_cobol_AnswerFindSelection;
typedef struct thinkthen_cobol_AnswerAnnotationField {
 uint64_t record; thinkthen_cobol_AnswerText name; uint64_t kind;
 uint64_t boolean; double number; thinkthen_cobol_AnswerText text;
 const thinkthen_cobol_AnswerText *tags; uint64_t tag_count; uint64_t failure_kind; uint64_t failure_cause;
} thinkthen_cobol_AnswerAnnotationField;
typedef struct thinkthen_cobol_AnswerRecognition {
 uint64_t record; uint64_t proposal; thinkthen_cobol_AnswerText kind; thinkthen_cobol_AnswerText text;
 uint64_t start; uint64_t end; uint64_t length; double strength;
 uint64_t mode; uint64_t file_present; thinkthen_cobol_AnswerText file;
 uint64_t first_line_present; uint64_t first_line; uint64_t last_line_present; uint64_t last_line;
 const struct thinkthen_cobol_AnswerExtensions *extensions;
} thinkthen_cobol_AnswerRecognition;
typedef struct thinkthen_cobol_AnswerRecognitionEdge {
 uint64_t record; uint64_t either_present; uint64_t either; double probability;
 thinkthen_cobol_AnswerText relation; thinkthen_cobol_AnswerRecognition source; thinkthen_cobol_AnswerRecognition target;
 const struct thinkthen_cobol_AnswerExtensions *extensions;
} thinkthen_cobol_AnswerRecognitionEdge;
typedef struct thinkthen_cobol_AnswerRelation {
 uint64_t record; thinkthen_cobol_AnswerText relation; thinkthen_cobol_AnswerText source;
 thinkthen_cobol_AnswerText source_kind; thinkthen_cobol_AnswerText target;
 thinkthen_cobol_AnswerText target_kind; double probability; uint64_t either_present; uint64_t either;
} thinkthen_cobol_AnswerRelation;
typedef struct thinkthen_cobol_AnswerStatus {
 int64_t code; uint64_t retryable; uint64_t failure_kind; thinkthen_cobol_AnswerText message;
 uint64_t facts_present; thinkthen_cobol_AnswerFacts facts;
 const thinkthen_cobol_AnswerImmediateFacts *immediate_facts; const thinkthen_cobol_AnswerError *details;
 tt_answer_owner *owner;
} thinkthen_cobol_AnswerStatus;
typedef struct thinkthen_cobol_AnswerTable {tt_answer_owner *owner; const void *rows; uint64_t count; const thinkthen_cobol_AnswerRecognitionEdge *edges; uint64_t edge_count;} thinkthen_cobol_AnswerTable;
void TT_ANSWER_FREE(tt_answer_owner *);
"""


def render(native_facts, root, definitions):
    from templates.csharp import enum_values
    functions = enum_values(definitions['RequestFunction'])
    from templates.cobol_facts import render as facts_render
    facts_header, facts_source = facts_render(native_facts)
    from templates.cobol_results import render as results_render
    result_header, result_source, receipt_types, payloads = results_render(native_facts, root, functions)
    from templates.cobol_plan import render as plan_render
    plan_header, plan_declaration, plan_source = plan_render(root)
    declarations = HEADER.replace('typedef struct thinkthen_cobol_AnswerStatus {', facts_header + '\ntypedef struct thinkthen_cobol_AnswerStatus {')
    declarations += result_header + '\n'
    declarations += plan_header + '\n' + plan_declaration + '\n'
    source = SUPPORT.replace('static int copy_recognition_entity(', facts_source + '\n' + result_source + '\nstatic int copy_recognition_entity(')
    rows = {'decide': 'Decision', 'choose': 'Choice', 'tag': 'Table', 'score': 'Score', 'filter': 'FilterOriginal', 'rank': 'RankIndex', 'annotate': 'AnnotationField', 'find': 'FindSelection', 'recognize': 'Recognition', 'relate': 'Relation'}
    for function in functions:
        typ = 'thinkthen_cobol_Answer' + function.title() + 'Result'
        row = 'thinkthen_cobol_Answer' + rows[function]
        value_type = 'thinkthen_cobol_Answer' + function.title() + 'Value'
        fields = ['uint64_t scalar;', f'const {row} *rows;', 'uint64_t count;', f'{row} first;']
        if function == 'recognize':
            fields += ['const thinkthen_cobol_AnswerRecognitionEdge *edges;', 'uint64_t edge_count;']
        declarations += f'typedef struct {value_type} {{' + ''.join(fields) + f'}} {value_type};\n'
        declarations += f'typedef struct {typ} {{tt_answer_owner *owner; {value_type} value; thinkthen_cobol_AnswerProbability probability; {receipt_types[function]} results; uint64_t facts_present; thinkthen_cobol_AnswerFacts facts;}} {typ};\n'
        signature = f'int TT_{function.upper()}(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,const thinkthen_cobol_RequestOptions *options,{typ} *out,thinkthen_cobol_AnswerStatus *status_out)'
        declarations += signature + ';\n'
        source += signature + ' {\nif(!out) return THINKTHEN_COBOL_REPRESENTATION;\nthinkthen_cobol_AnswerTable table={0};\nint code=blocking_answer(engine,question,input,options,"' + function + '",' + str(len(function)) + ',THINKTHEN_COMPLETE_REQUEST_FUNCTION_' + function.upper() + '_V1,&table,status_out);\nif(code) return code;\n'
        if function == 'decide':
            source += 'for(size_t i=0;i<table.count;i++) ((thinkthen_cobol_AnswerDecision *)table.rows)[i].present=i<table.owner->probability_count && table.owner->probabilities[i].present;\n'
        source += typ + ' result={.owner=table.owner,.value={.rows=table.rows,.count=table.count},.results={table.owner->receipts,table.owner->receipt_count},.facts_present=status_out->facts_present,.facts=status_out->facts};\n'
        source += 'result.value.scalar=input && (input->kind==THINKTHEN_COBOL_REQUESTINPUTTEXT_KIND || input->kind==THINKTHEN_COBOL_REQUESTINPUTJSON_KIND);\n'
        if function in ('filter', 'rank', 'find', 'relate'):
            source += 'result.value.scalar=0;\n'
        source += 'if(table.count) result.value.first=result.value.rows[0];\n'
        source += 'result.probability=(thinkthen_cobol_AnswerProbability){.scalar=result.value.scalar,.data=table.owner->probabilities,.count=table.owner->probability_count,.lists=table.owner->probability_lists,.list_count=table.owner->probability_list_count};\nif(result.probability.count) result.probability.first=result.probability.data[0];\n'
        if function == 'find':
            source += 'result.probability.scalar=1;\n'
        if function == 'recognize':
            source += 'if(result.value.scalar && result.probability.list_count==1) {result.probability.data=result.probability.lists[0].data; result.probability.count=result.probability.lists[0].count; result.probability.list_count=0;} result.probability.scalar=0;\n'
            source += 'result.value.edges=table.edges; result.value.edge_count=table.edge_count;\n'
        source += '*out=result; return 0;\n}\n'
    return declarations, source + plan_source


SUPPORT = r"""
struct tt_answer_owner {arena storage; void *rows; size_t count; size_t capacity; void *edges; size_t edge_count; size_t edge_capacity; void *receipts; size_t receipt_count; thinkthen_cobol_AnswerNumber *probabilities; size_t probability_count; thinkthen_cobol_AnswerProbabilityList *probability_lists; size_t probability_list_count; unsigned char *resolved; size_t resolved_count;};
void TT_ANSWER_FREE(tt_answer_owner *owner) {
    if(owner) {release(&owner->storage); free(owner->rows); free(owner->edges); free(owner->receipts); free(owner->probabilities); free(owner->probability_lists); free(owner->resolved); free(owner);}
}
static int answer_text(thinkthen_complete_utf8_v1 input,thinkthen_cobol_AnswerText *out) {
    return TT_SESSION_TEXT(&input,out->text,&out->length);
}
static int original_limit(const thinkthen_complete_json_v1 *input) {
    if(!input) return THINKTHEN_COBOL_REPRESENTATION;
    if(input->kind==THINKTHEN_COMPLETE_JSON_STRING_V1 && input->data.string.len>THINKTHEN_COBOL_TEXT_CAPACITY) return THINKTHEN_COBOL_OVERFLOW;
    if(input->kind==THINKTHEN_COMPLETE_JSON_NUMBER_V1 && input->data.number.len>THINKTHEN_COBOL_TEXT_CAPACITY) return THINKTHEN_COBOL_OVERFLOW;
    if(input->kind==THINKTHEN_COMPLETE_JSON_ARRAY_V1) for(size_t i=0;i<input->data.array.len;i++) {int code=original_limit(input->data.array.data[i]); if(code) return code;}
    if(input->kind==THINKTHEN_COMPLETE_JSON_OBJECT_V1) for(size_t i=0;i<input->data.object.len;i++) {
        if(input->data.object.data[i].name.len>THINKTHEN_COBOL_TEXT_CAPACITY) return THINKTHEN_COBOL_OVERFLOW;
        int code=original_limit(input->data.object.data[i].value); if(code) return code;
    }
    return 0;
}
static const tt_value *answer_original(arena *storage,const thinkthen_complete_json_v1 *input) {
    int code=original_limit(input); if(code) {storage->code=code; return NULL;}
    tt_value *out=allocate(storage,1,sizeof(*out));
    if(out) out->value=copy_value(storage,input);
    return out;
}
static int answer_tags(arena *storage,thinkthen_complete_string_list_v1 input,const thinkthen_cobol_AnswerText **out) {
    thinkthen_cobol_AnswerText *rows=allocate(storage,input.len,sizeof(*rows));
    if(!rows) return storage->code;
    for(size_t i=0;i<input.len;i++) {
        int code=answer_text(input.data[i],rows+i); if(code) return code;
    }
    *out=rows; return 0;
}
static int copy_recognition_entity(arena *a,const thinkthen_complete_entity_v1 *entity,thinkthen_cobol_AnswerRecognition *row) {
    row->start=entity->start; row->end=entity->end; row->length=entity->length; row->strength=entity->strength;
    row->file_present=entity->file.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1;
    row->first_line_present=entity->first_line.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1; row->first_line=entity->first_line.value;
    row->last_line_present=entity->last_line.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1; row->last_line=entity->last_line.value;
    int code=answer_text(entity->kind,&row->kind); if(!code) code=answer_text(entity->text,&row->text);
    if(!code && row->file_present) code=answer_text(entity->file.value,&row->file);
    thinkthen_cobol_AnswerExtensions *extensions=allocate(a,1,sizeof(*extensions));
    if(extensions && !code) code=copy_thinkthen_cobol_AnswerExtensions(a,&entity->extensions,extensions);
    row->extensions=extensions; return code ? code : a->code;
}
static int project_answer(arena *a,const thinkthen_complete_session_packet_v1 *packet,uint32_t function,thinkthen_cobol_AnswerTable *out) {
    if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1) {
        const thinkthen_complete_atomic_decide_value_v1 *items[1]={packet->data.decide_row->value};
        const thinkthen_complete_session_packet_decide_aggregate_v1 aggregate={.value={items,1}};
        const thinkthen_complete_session_packet_v1 synthetic={.kind=THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_AGGREGATE_V1,.data.decide_aggregate=&aggregate};
        return project_answer(a,&synthetic,function,out);
    }
    if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_ROW_V1) {
        const thinkthen_complete_atomic_nullable_string_v1 *items[1]={packet->data.choose_row->value};
        const thinkthen_complete_session_packet_choose_aggregate_v1 aggregate={.value={items,1}};
        const thinkthen_complete_session_packet_v1 synthetic={.kind=THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_AGGREGATE_V1,.data.choose_aggregate=&aggregate};
        return project_answer(a,&synthetic,function,out);
    }
    if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_TAG_ROW_V1) {
        const thinkthen_complete_atomic_array_of_string_v1 *items[1]={packet->data.tag_row->value};
        const thinkthen_complete_session_packet_tag_aggregate_v1 aggregate={.value={items,1}};
        const thinkthen_complete_session_packet_v1 synthetic={.kind=THINKTHEN_COMPLETE_SESSION_PACKET_TAG_AGGREGATE_V1,.data.tag_aggregate=&aggregate};
        return project_answer(a,&synthetic,function,out);
    }
    if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_ROW_V1) {
        const thinkthen_complete_atomic_double_v1 *items[1]={packet->data.score_row->value};
        const thinkthen_complete_session_packet_score_aggregate_v1 aggregate={.value={items,1}};
        const thinkthen_complete_session_packet_v1 synthetic={.kind=THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_AGGREGATE_V1,.data.score_aggregate=&aggregate};
        return project_answer(a,&synthetic,function,out);
    }
    if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_ROW_V1) {
        const thinkthen_complete_atomic_boolean_v1 *items[1]={packet->data.filter_row->value};
        const thinkthen_complete_session_packet_filter_aggregate_v1 aggregate={.value={items,1}};
        const thinkthen_complete_session_packet_v1 synthetic={.kind=THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_AGGREGATE_V1,.data.filter_aggregate=&aggregate};
        return project_answer(a,&synthetic,function,out);
    }
    if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_ROW_V1) {
        const thinkthen_complete_annotation_v1 *items[1]={packet->data.annotate_row->value};
        const thinkthen_complete_session_packet_annotate_aggregate_v1 aggregate={.value={items,1}};
        const thinkthen_complete_session_packet_v1 synthetic={.kind=THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_AGGREGATE_V1,.data.annotate_aggregate=&aggregate};
        return project_answer(a,&synthetic,function,out);
    }
    switch(function) {
    case THINKTHEN_COMPLETE_REQUEST_FUNCTION_DECIDE_V1: {
        if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_AGGREGATE_V1) return 0;
        const thinkthen_complete_atomic_decide_value_list_v1 list=packet->data.decide_aggregate->value;
        thinkthen_cobol_AnswerDecision *rows=allocate(a,list.len,sizeof(*rows)); out->rows=rows; out->count=list.len;
        if(!rows) return a->code;
        for(size_t i=0;i<list.len;i++) {
            const thinkthen_complete_decide_value_presence_v1 value=list.data[i]->value;
            rows[i].present=value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1;
            if(rows[i].present) {
                if(!value.value || !value.value->value) return THINKTHEN_COBOL_REPRESENTATION;
                rows[i].value=answer_original(a,value.value->value);
            } else rows[i].value=answer_original(a,&(thinkthen_complete_json_v1){.kind=THINKTHEN_COMPLETE_JSON_NULL_V1});
        }
        break;
    }
    case THINKTHEN_COMPLETE_REQUEST_FUNCTION_CHOOSE_V1: {
        if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_AGGREGATE_V1) return 0;
        const thinkthen_complete_atomic_nullable_string_list_v1 list=packet->data.choose_aggregate->value;
        thinkthen_cobol_AnswerChoice *rows=allocate(a,list.len,sizeof(*rows)); out->rows=rows; out->count=list.len;
        if(!rows) return a->code;
        for(size_t i=0;i<list.len;i++) {
            rows[i].present=list.data[i]->value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1;
            if(rows[i].present) {int code=answer_text(list.data[i]->value.value,&rows[i].value); if(code) return code;}
        }
        break;
    }
    case THINKTHEN_COMPLETE_REQUEST_FUNCTION_TAG_V1: {
        if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_TAG_AGGREGATE_V1) return 0;
        const thinkthen_complete_atomic_array_of_string_list_v1 list=packet->data.tag_aggregate->value;
        thinkthen_cobol_AnswerTable *rows=allocate(a,list.len,sizeof(*rows)); out->rows=rows; out->count=list.len;
        if(!rows) return a->code;
        for(size_t i=0;i<list.len;i++) {
            const thinkthen_cobol_AnswerText *tags=NULL;
            int code=answer_tags(a,list.data[i]->value,&tags); if(code) return code;
            rows[i].rows=tags; rows[i].count=list.data[i]->value.len;
        }
        break;
    }
    case THINKTHEN_COMPLETE_REQUEST_FUNCTION_SCORE_V1: {
        if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_AGGREGATE_V1) return 0;
        const thinkthen_complete_atomic_double_list_v1 list=packet->data.score_aggregate->value;
        thinkthen_cobol_AnswerScore *rows=allocate(a,list.len,sizeof(*rows)); out->rows=rows; out->count=list.len;
        if(!rows) return a->code;
        for(size_t i=0;i<list.len;i++) rows[i].value=list.data[i]->value;
        break;
    }
    case THINKTHEN_COMPLETE_REQUEST_FUNCTION_FILTER_V1: {
        if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_AGGREGATE_V1) return 0;
        const thinkthen_complete_atomic_boolean_list_v1 list=packet->data.filter_aggregate->value;
        thinkthen_cobol_AnswerFilterOriginal *rows=allocate(a,list.len,sizeof(*rows)); out->rows=rows;
        if(!rows) return a->code;
        for(size_t i=0;i<list.len;i++) if(list.data[i]->value) {
            if(list.data[i]->input.presence==THINKTHEN_COMPLETE_PRESENCE_MISSING_V1) return THINKTHEN_COBOL_REPRESENTATION;
            const thinkthen_complete_json_v1 null_value={.kind=THINKTHEN_COMPLETE_JSON_NULL_V1};
            rows[out->count++].original=answer_original(a,list.data[i]->input.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? list.data[i]->input.value : &null_value);
        }
        break;
    }
    case THINKTHEN_COMPLETE_REQUEST_FUNCTION_RANK_V1: {
        if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_RANK_AGGREGATE_V1) return 0;
        const thinkthen_complete_atomic_non_zero_usize_list_v1 list=packet->data.rank_aggregate->value;
        thinkthen_cobol_AnswerRankIndex *rows=allocate(a,list.len,sizeof(*rows)); out->rows=rows; out->count=list.len;
        if(!rows) return a->code;
        for(size_t i=0;i<list.len;i++) {
            if(list.data[i]->index.presence!=THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) return THINKTHEN_COBOL_REPRESENTATION;
            rows[i].value=list.data[i]->index.value;
        }
        break;
    }
    case THINKTHEN_COMPLETE_REQUEST_FUNCTION_ANNOTATE_V1: {
        if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_AGGREGATE_V1) return 0;
        const thinkthen_complete_annotation_list_v1 list=packet->data.annotate_aggregate->value;
        size_t count=0; for(size_t i=0;i<list.len;i++) count+=list.data[i]->value->value.len;
        thinkthen_cobol_AnswerAnnotationField *rows=allocate(a,count,sizeof(*rows)); out->rows=rows; out->count=count;
        if(!rows) return a->code;
        size_t position=0;
        for(size_t i=0;i<list.len;i++) {
            const thinkthen_complete_annotated_field_map_v1 fields=list.data[i]->value->value;
            for(size_t j=0;j<fields.len;j++) {
                thinkthen_cobol_AnswerAnnotationField *row=rows+position++;
                const thinkthen_complete_annotated_field_v1 *value=fields.data[j].value;
                row->record=list.data[i]->index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? list.data[i]->index.value : i+1; row->kind=value->kind;
                int code=answer_text(fields.data[j].name,&row->name); if(code) return code;
                switch(value->kind) {
                case THINKTHEN_COMPLETE_ANNOTATED_FIELD_BOOLEAN_V1: row->boolean=value->data.boolean; break;
                case THINKTHEN_COMPLETE_ANNOTATED_FIELD_NULL_V1: break;
                case THINKTHEN_COMPLETE_ANNOTATED_FIELD_STRING_V1: code=answer_text(value->data.string,&row->text); break;
                case THINKTHEN_COMPLETE_ANNOTATED_FIELD_NUMBER_V1: row->number=value->data.number; break;
                case THINKTHEN_COMPLETE_ANNOTATED_FIELD_ARRAY_V1: code=answer_tags(a,value->data.array,&row->tags); row->tag_count=value->data.array.len; break;
                case THINKTHEN_COMPLETE_ANNOTATED_FIELD_OBJECT_V1:
                    row->failure_kind=value->data.object->failed->kind->kind;
                    row->failure_cause=value->data.object->failed->cause->kind; break;
                default: return THINKTHEN_COBOL_REPRESENTATION;
                }
                if(code) return code;
            }
        }
        break;
    }
    case THINKTHEN_COMPLETE_REQUEST_FUNCTION_FIND_V1: {
        if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_FIND_AGGREGATE_V1) return 0;
        const thinkthen_complete_find_v1 *value=packet->data.find_aggregate->value;
        thinkthen_cobol_AnswerFindSelection *row=allocate(a,1,sizeof(*row)); out->rows=row; out->count=1;
        if(!row) return a->code;
        row->present=value->index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1;
        if(row->present) row->index=value->index.value;
        if(value->value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) row->original=answer_original(a,value->value.value);
        else if(row->present) row->original=answer_original(a,&(thinkthen_complete_json_v1){.kind=THINKTHEN_COMPLETE_JSON_NULL_V1});
        break;
    }
    case THINKTHEN_COMPLETE_REQUEST_FUNCTION_RECOGNIZE_V1: {
        if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_RECOGNIZE_AGGREGATE_V1) return 0;
        const thinkthen_complete_recognition_list_v1 list=packet->data.recognize_aggregate->value;
        size_t count=0;
        for(size_t i=0;i<list.len;i++) {
            const thinkthen_complete_recognize_v1 *value=list.data[i]->value;
            count+=value->kind==THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES_V1 ? value->data.entities->entities.len : value->data.mode_proposals->proposals.len;
        }
        thinkthen_cobol_AnswerRecognition *rows=allocate(a,count,sizeof(*rows)); out->rows=rows; out->count=count;
        if(!rows) return a->code;
        size_t edges_count=0;
        for(size_t i=0;i<list.len;i++) {
            const thinkthen_complete_recognize_v1 *value=list.data[i]->value;
            if(value->kind==THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES_V1 && value->data.entities->relations.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) edges_count+=value->data.entities->relations.value.len;
        }
        thinkthen_cobol_AnswerRecognitionEdge *edges=allocate(a,edges_count,sizeof(*edges));
        if(!edges) return a->code;
        out->edges=edges; out->edge_count=edges_count;
        size_t position=0,edge_position=0;
        for(size_t i=0;i<list.len;i++) {
            const thinkthen_complete_recognize_v1 *value=list.data[i]->value;
            if(value->kind==THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES_V1) {
                for(size_t j=0;j<value->data.entities->entities.len;j++) {
                    const thinkthen_complete_entity_v1 *entity=value->data.entities->entities.data[j];
                    thinkthen_cobol_AnswerRecognition *row=rows+position++; row->record=i+1;
                    int code=copy_recognition_entity(a,entity,row); if(code) return code;
                }
                if(value->data.entities->relations.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) {
                    const thinkthen_complete_entity_edge_list_v1 input=value->data.entities->relations.value;
                    for(size_t j=0;j<input.len;j++) {
                        const thinkthen_complete_entity_edge_v1 *edge=input.data[j];
                        thinkthen_cobol_AnswerRecognitionEdge *row=edges+edge_position++;
                        row->record=i+1; row->probability=edge->probability;
                        row->either_present=edge->either.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1; row->either=edge->either.value;
                        int code=answer_text(edge->relation,&row->relation);
                        if(!code) code=copy_recognition_entity(a,edge->source,&row->source);
                        if(!code) code=copy_recognition_entity(a,edge->target,&row->target);
                        thinkthen_cobol_AnswerExtensions *extensions=allocate(a,1,sizeof(*extensions));
                        if(extensions && !code) code=copy_thinkthen_cobol_AnswerExtensions(a,&edge->extensions,extensions);
                        row->extensions=extensions; if(code) return code;
                    }
                }
            } else if(value->kind==THINKTHEN_COMPLETE_RECOGNIZE_MODE_PROPOSALS_V1) {
                for(size_t j=0;j<value->data.mode_proposals->proposals.len;j++) {
                    const thinkthen_complete_boundary_proposal_v1 *proposal=value->data.mode_proposals->proposals.data[j];
                    thinkthen_cobol_AnswerRecognition *row=rows+position++; row->record=i+1; row->proposal=1; row->mode=value->data.mode_proposals->mode->kind;
                    row->start=proposal->start; row->end=proposal->end; row->length=proposal->length; row->strength=0;
                    int code=answer_text(proposal->text,&row->text);
                    thinkthen_cobol_AnswerExtensions *extensions=allocate(a,1,sizeof(*extensions));
                    if(extensions && !code) code=copy_thinkthen_cobol_AnswerExtensions(a,&proposal->extensions,extensions);
                    row->extensions=extensions; if(code) return code;
                }
            } else return THINKTHEN_COBOL_REPRESENTATION;
        }
        break;
    }
    case THINKTHEN_COMPLETE_REQUEST_FUNCTION_RELATE_V1: {
        if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_RELATE_AGGREGATE_V1) return 0;
        const thinkthen_complete_related_entity_edge_list_v1 list=packet->data.relate_aggregate->value->value;
        size_t count=list.len;
        thinkthen_cobol_AnswerRelation *rows=allocate(a,count,sizeof(*rows)); out->rows=rows; out->count=count;
        if(!rows) return a->code;
        size_t position=0;
        for(size_t i=0;i<list.len;i++) {
            const thinkthen_complete_related_entity_edge_v1 *edge=list.data[i];
            thinkthen_cobol_AnswerRelation *row=rows+position++; row->record=i+1; row->probability=edge->probability;
            row->either_present=edge->either.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1; row->either=edge->either.value;
            int code=answer_text(edge->relation,&row->relation); if(code) return code;
            const thinkthen_complete_relation_endpoint_v1 *endpoints[2]={edge->source,edge->target};
            thinkthen_cobol_AnswerText *names[2]={&row->source,&row->target},*kinds[2]={&row->source_kind,&row->target_kind};
            for(size_t k=0;k<2;k++) {
                const thinkthen_complete_relation_endpoint_v1 *endpoint=endpoints[k];
                if(endpoint->kind==THINKTHEN_COMPLETE_RELATION_ENDPOINT_RELATED_ENTITY_V1) {
                    code=answer_text(endpoint->data.related_entity->name,names[k]); if(!code) code=answer_text(endpoint->data.related_entity->kind,kinds[k]);
                } else if(endpoint->kind==THINKTHEN_COMPLETE_RELATION_ENDPOINT_SOURCE_V1) {
                    code=answer_text(endpoint->data.source->name,names[k]); if(!code) code=answer_text(endpoint->data.source->kind,kinds[k]);
                } else return THINKTHEN_COBOL_REPRESENTATION;
                if(code) return code;
            }
        }
        break;
    }
    default: return THINKTHEN_COBOL_REPRESENTATION;
    }
    return a->code;
}
static int append_edges(tt_answer_owner *owner,thinkthen_cobol_AnswerTable part) {
    const size_t width=sizeof(thinkthen_cobol_AnswerRecognitionEdge);
    if(part.edge_count>SIZE_MAX/width-owner->edge_count) return THINKTHEN_COBOL_OVERFLOW;
    size_t count=owner->edge_count+part.edge_count;
    if(count>owner->edge_capacity) {
        void *edges=realloc(owner->edges,count*width);
        if(!edges) return THINKTHEN_ELOCAL;
        owner->edges=edges; owner->edge_capacity=count;
    }
    if(part.edge_count) memcpy((char *)owner->edges+owner->edge_count*width,part.edges,part.edge_count*width);
    owner->edge_count=count; return 0;
}
static int append_answer(tt_answer_owner *owner,thinkthen_cobol_AnswerTable part,uint32_t kind) {
    const size_t widths[]={[THINKTHEN_COMPLETE_REQUEST_FUNCTION_DECIDE_V1]=sizeof(thinkthen_cobol_AnswerDecision),[THINKTHEN_COMPLETE_REQUEST_FUNCTION_CHOOSE_V1]=sizeof(thinkthen_cobol_AnswerChoice),[THINKTHEN_COMPLETE_REQUEST_FUNCTION_TAG_V1]=sizeof(thinkthen_cobol_AnswerTable),[THINKTHEN_COMPLETE_REQUEST_FUNCTION_SCORE_V1]=sizeof(thinkthen_cobol_AnswerScore),[THINKTHEN_COMPLETE_REQUEST_FUNCTION_FILTER_V1]=sizeof(thinkthen_cobol_AnswerFilterOriginal),[THINKTHEN_COMPLETE_REQUEST_FUNCTION_RANK_V1]=sizeof(thinkthen_cobol_AnswerRankIndex),[THINKTHEN_COMPLETE_REQUEST_FUNCTION_ANNOTATE_V1]=sizeof(thinkthen_cobol_AnswerAnnotationField),[THINKTHEN_COMPLETE_REQUEST_FUNCTION_FIND_V1]=sizeof(thinkthen_cobol_AnswerFindSelection),[THINKTHEN_COMPLETE_REQUEST_FUNCTION_RECOGNIZE_V1]=sizeof(thinkthen_cobol_AnswerRecognition),[THINKTHEN_COMPLETE_REQUEST_FUNCTION_RELATE_V1]=sizeof(thinkthen_cobol_AnswerRelation)};
    size_t width=widths[kind];
    if(part.count>SIZE_MAX/width-owner->count) return THINKTHEN_COBOL_OVERFLOW;
    size_t count=owner->count+part.count;
    if(count>owner->capacity) {
        size_t capacity=count<=SIZE_MAX/width/2 ? count*2 : count;
        void *rows=realloc(owner->rows,capacity*width);
        if(!rows) return THINKTHEN_ELOCAL;
        owner->rows=rows; owner->capacity=capacity;
    }
    if(part.count) memcpy((char *)owner->rows+owner->count*width,part.rows,part.count*width);
    owner->count=count; return 0;
}
static int capture_immediate_error(int code,thinkthen_cobol_AnswerStatus *out) {
    thinkthen_cobol_AnswerStatus observed={.code=code};
    if(code>=THINKTHEN_COBOL_REPRESENTATION) {*out=observed; return code;}
    tt_answer_owner *owner=calloc(1,sizeof(*owner));
    if(!owner) {observed.code=THINKTHEN_ELOCAL; *out=observed; return THINKTHEN_ELOCAL;}
    observed.owner=owner;
    const thinkthen_error_view_v1 *error=thinkthen_session_error_view();
    if(error) {
        observed.retryable=error->retryable;
        int copied=answer_text(error->message,&observed.message);
        if(error->facts) {
            thinkthen_cobol_AnswerImmediateFacts *facts=allocate(&owner->storage,1,sizeof(*facts));
            if(facts && !copied) copied=copy_thinkthen_cobol_AnswerImmediateFacts(&owner->storage,error->facts,facts);
            if(!copied) observed.immediate_facts=facts;
        }
        if(copied || owner->storage.code) observed.code=copied ? copied : owner->storage.code;
    }
    *out=observed; return (int)observed.code;
}
static int blocking_answer(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,const thinkthen_cobol_RequestOptions *options,const char *function,size_t length,uint32_t kind,thinkthen_cobol_AnswerTable *out,thinkthen_cobol_AnswerStatus *status_out) {
    if(!out || !status_out) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_AnswerStatus observed={0};
    const thinkthen_cobol_RequestCall arguments={.m_question=question,.m_input=input,.m_options=options};
    tt_answer_owner *owner=calloc(1,sizeof(*owner));
    if(!owner) return THINKTHEN_ELOCAL;
    observed.owner=owner;
    thinkthen_session *session=NULL;
    int code=start(engine,&arguments,(void **)&session,function,length);
    if(code) {TT_ANSWER_FREE(owner); return capture_immediate_error(code,status_out);}
    if(input && input->kind==THINKTHEN_COBOL_REQUESTINPUTFEED_KIND) code=thinkthen_session_finish_typed(session,NULL);
    thinkthen_cobol_AnswerTable answer={.owner=owner};
    uint32_t status=0;
    while(!code && status!=THINKTHEN_SESSION_END_V1) {
        thinkthen_session_result *packet_owner=NULL;
        code=thinkthen_session_next_packet(session,THINKTHEN_SESSION_WAIT_RESULT_V1,&status,&packet_owner);
        if(code || status!=THINKTHEN_SESSION_RESULT_V1) break;
        const thinkthen_complete_session_packet_v1 *packet=NULL;
        code=thinkthen_session_result_view(packet_owner,&packet);
        if(!code && packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1) {
            const thinkthen_complete_session_packet_terminal_v1 *terminal=packet->data.terminal;
            if(terminal->facts.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) {
                code=copy_thinkthen_cobol_AnswerFacts(&owner->storage,terminal->facts.value,&observed.facts);
                if(!code) observed.facts_present=1;
            }
            if(terminal->failure.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) {
                const thinkthen_complete_error_v1 *error=terminal->failure.value->error;
                thinkthen_cobol_AnswerError *details=allocate(&owner->storage,1,sizeof(*details));
                if(details && !code) code=copy_thinkthen_cobol_AnswerError(&owner->storage,error,details);
                if(!code) observed.details=details;
                observed.failure_kind=error->kind->kind; observed.retryable=error->retryable;
                if(!code) code=answer_text(error->message,&observed.message);
                const int codes[]={0,THINKTHEN_EUSAGE,THINKTHEN_EBACKEND,THINKTHEN_ELOCAL,THINKTHEN_ECANCELLED,THINKTHEN_EDEADLINE,THINKTHEN_ELOCAL};
                if(!code) code=owner->storage.code;
                if(!code) code=observed.failure_kind<sizeof(codes)/sizeof(codes[0]) ? codes[observed.failure_kind] : THINKTHEN_ELOCAL;
            }
        } else if(!code) {
            code=observe_resolution(owner,packet);
            if(!code) code=collect_receipts(owner,packet,kind);
            thinkthen_cobol_AnswerTable part={0};
            if(!code) code=project_answer(&owner->storage,packet,kind,&part);
            if(!code && part.count) code=append_answer(owner,part,kind);
            if(!code && part.edge_count) code=append_edges(owner,part);
        }
        thinkthen_session_result_free(packet_owner);
    }
    thinkthen_session_free(session);
    observed.code=code; *status_out=observed;
    if(code) return code;
    answer.rows=owner->rows; answer.count=owner->count; answer.edges=owner->edges; answer.edge_count=owner->edge_count;
    *out=answer; return 0;
}
"""
