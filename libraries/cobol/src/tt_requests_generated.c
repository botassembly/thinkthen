/* Generated counted-carrier adaptation; Rust owns request admission. */
#include "tt_session.h"
#include "thinkthen.h"
#include <stdlib.h>
#include <stdint.h>
typedef struct allocation {
    struct allocation *next;
    void *memory;
} allocation;
typedef struct {
    allocation *head;
    int code;
} arena;
static void *allocate(arena *a,size_t count,size_t width) {
    if(a->code) return NULL;
    if(width && count>SIZE_MAX/width) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return NULL;
    }
    allocation *item=malloc(sizeof(*item));
    void *memory=calloc(count ? count : 1,width);
    if(!item || !memory) {
        free(item);
        free(memory);
        a->code=THINKTHEN_ELOCAL;
        return NULL;
    }
    *item=(allocation){
        a->head,memory
    };
    a->head=item;
    return memory;
}
static void release(arena *a) {
    while(a->head) {
        allocation *item=a->head;
        a->head=item->next;
        free(item->memory);
        free(item);
    }
}
#include <string.h>
struct tt_value {
    arena storage;
    thinkthen_complete_json_v1 *value;
};
static thinkthen_complete_utf8_v1 copy_text(arena *storage,thinkthen_complete_utf8_v1 input) {
    if(input.len && !input.data) {
        storage->code=THINKTHEN_COBOL_REPRESENTATION;
        return (thinkthen_complete_utf8_v1){
            0
        };
    }
    char *bytes=allocate(storage,input.len,1);
    if(bytes && input.len) memcpy(bytes,input.data,input.len);
    return (thinkthen_complete_utf8_v1){
        bytes,input.len
    };
}
static thinkthen_complete_json_v1 *copy_value(arena *storage,const thinkthen_complete_json_v1 *input) {
    if(!input) {
        storage->code=THINKTHEN_COBOL_REPRESENTATION;
        return NULL;
    }
    thinkthen_complete_json_v1 *out=allocate(storage,1,sizeof(*out));
    if(!out) return NULL;
    out->kind=input->kind;
    switch(input->kind) {
        case THINKTHEN_COMPLETE_JSON_NULL_V1: break;
        case THINKTHEN_COMPLETE_JSON_BOOLEAN_V1: out->data.boolean=input->data.boolean;
        break;
        case THINKTHEN_COMPLETE_JSON_STRING_V1: out->data.string=copy_text(storage,input->data.string);
        break;
        case THINKTHEN_COMPLETE_JSON_NUMBER_V1: out->data.number=copy_text(storage,input->data.number);
        break;
        case THINKTHEN_COMPLETE_JSON_ARRAY_V1: {
            size_t count=input->data.array.len;
            if(count && !input->data.array.data) {
                storage->code=THINKTHEN_COBOL_REPRESENTATION;
                break;
            }
            const thinkthen_complete_json_v1 **items=allocate(storage,count,sizeof(*items));
            out->data.array=(thinkthen_complete_json_array_v1){
                items,count
            };
            if(items) for(size_t i=0;
            i<count && !storage->code;
            i++) items[i]=copy_value(storage,input->data.array.data[i]);
            break;
        }
        case THINKTHEN_COMPLETE_JSON_OBJECT_V1: {
            size_t count=input->data.object.len;
            if(count && !input->data.object.data) {
                storage->code=THINKTHEN_COBOL_REPRESENTATION;
                break;
            }
            thinkthen_complete_json_entry_v1 *items=allocate(storage,count,sizeof(*items));
            out->data.object=(thinkthen_complete_json_object_v1){
                items,count
            };
            if(items) for(size_t i=0;
            i<count && !storage->code;
            i++) {
                items[i].name=copy_text(storage,input->data.object.data[i].name);
                items[i].value=copy_value(storage,input->data.object.data[i].value);
            }
            break;
        }
        default: storage->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
void TT_VALUE_FREE(tt_value *value) {
    if(value) {
        release(&value->storage);
        free(value);
    }
}
static int own_value(const thinkthen_complete_json_v1 *input,tt_value **out) {
    if(!out) return THINKTHEN_COBOL_REPRESENTATION;
    tt_value *owner=calloc(1,sizeof(*owner));
    if(!owner) return THINKTHEN_ELOCAL;
    owner->value=copy_value(&owner->storage,input);
    if(owner->storage.code) {
        int code=owner->storage.code;
        TT_VALUE_FREE(owner);
        return code;
    }
    *out=owner;
    return 0;
}
int TT_VALUE_KIND(const tt_value *value,uint32_t *out) {
    if(!value || !out) return THINKTHEN_COBOL_REPRESENTATION;
    *out=value->value->kind;
    return 0;
}
int TT_VALUE_LENGTH(const tt_value *value,uint64_t *out) {
    if(!value || !out) return THINKTHEN_COBOL_REPRESENTATION;
    if(value->value->kind==THINKTHEN_COMPLETE_JSON_ARRAY_V1) *out=value->value->data.array.len;
    else if(value->value->kind==THINKTHEN_COMPLETE_JSON_OBJECT_V1) *out=value->value->data.object.len;
    else return THINKTHEN_COBOL_REPRESENTATION;
    return 0;
}
int TT_VALUE_BOOLEAN(const tt_value *value,uint32_t *out) {
    if(!value || !out || value->value->kind!=THINKTHEN_COMPLETE_JSON_BOOLEAN_V1) return THINKTHEN_COBOL_REPRESENTATION;
    *out=value->value->data.boolean;
    return 0;
}
int TT_VALUE_TEXT(const tt_value *value,char *out,uint64_t *written) {
    if(!value) return THINKTHEN_COBOL_REPRESENTATION;
    if(value->value->kind==THINKTHEN_COMPLETE_JSON_STRING_V1) return TT_SESSION_TEXT(&value->value->data.string,
    out,written);
    if(value->value->kind==THINKTHEN_COMPLETE_JSON_NUMBER_V1) return TT_SESSION_TEXT(&value->value->data.number,
    out,written);
    return THINKTHEN_COBOL_REPRESENTATION;
}
int TT_VALUE_NAME(const tt_value *value,uint64_t position,char *out,uint64_t *written) {
    if(!value || value->value->kind!=THINKTHEN_COMPLETE_JSON_OBJECT_V1 || !position || position>value->value->data.object.len) return THINKTHEN_COBOL_REPRESENTATION;
    return TT_SESSION_TEXT(&value->value->data.object.data[position-1].name,out,written);
}
int TT_VALUE_ELEMENT(const tt_value *value,uint64_t position,tt_value **out) {
    if(!value || !position) return THINKTHEN_COBOL_REPRESENTATION;
    if(value->value->kind==THINKTHEN_COMPLETE_JSON_ARRAY_V1 && position<=value->value->data.array.len) return own_value(value->value->data.array.data[position-1],
    out);
    if(value->value->kind==THINKTHEN_COMPLETE_JSON_OBJECT_V1 && position<=value->value->data.object.len) return own_value(value->value->data.object.data[position-1].value,
    out);
    return THINKTHEN_COBOL_REPRESENTATION;
}
static const thinkthen_complete_json_v1 *field_value(const tt_value *value,const char *name,
uint64_t length) {
    if(!value || (length && !name) || value->value->kind!=THINKTHEN_COMPLETE_JSON_OBJECT_V1) return NULL;
    for(size_t i=0;
    i<value->value->data.object.len;
    i++) {
        const thinkthen_complete_json_entry_v1 *field=&value->value->data.object.data[i];
        if(field->name.len==length && (!length || !memcmp(field->name.data,name,length))) return field->value;
    }
    return NULL;
}
int TT_VALUE_FIELD(const tt_value *value,const char *name,uint64_t length,tt_value **out) {
    const thinkthen_complete_json_v1 *field=field_value(value,name,length);
    return field ? own_value(field,out) : THINKTHEN_COBOL_REPRESENTATION;
}
int TT_VALUE_HAS_FIELD(const tt_value *value,const char *name,uint64_t length,uint32_t *out) {
    if(!value || !out || (length && !name) || value->value->kind!=THINKTHEN_COMPLETE_JSON_OBJECT_V1) return THINKTHEN_COBOL_REPRESENTATION;
    *out=field_value(value,name,length)!=NULL;
    return 0;
}
int TT_VALUE_NULL(tt_value **out) {
    return own_value(&(thinkthen_complete_json_v1){
        .kind=THINKTHEN_COMPLETE_JSON_NULL_V1
    },out);
}
int TT_VALUE_BOOL(uint32_t value,tt_value **out) {
    return own_value(&(thinkthen_complete_json_v1){
        .kind=THINKTHEN_COMPLETE_JSON_BOOLEAN_V1,.data.boolean=value
    },out);
}
int TT_VALUE_STRING(const char *value,uint64_t length,tt_value **out) {
    return own_value(&(thinkthen_complete_json_v1){
        .kind=THINKTHEN_COMPLETE_JSON_STRING_V1,.data.string={
            value,length
        }
    },out);
}
int TT_VALUE_NUMBER(const char *value,uint64_t length,tt_value **out) {
    return own_value(&(thinkthen_complete_json_v1){
        .kind=THINKTHEN_COMPLETE_JSON_NUMBER_V1,.data.number={
            value,length
        }
    },out);
}
int TT_VALUE_ARRAY(const tt_value *const *values,uint64_t count,tt_value **out) {
    if(count && !values) return THINKTHEN_COBOL_REPRESENTATION;
    arena temporary={
        0
    };
    const thinkthen_complete_json_v1 **items=allocate(&temporary,count,sizeof(*items));
    if(items) for(uint64_t i=0;
    i<count;
    i++) {
        if(!values[i]) {
            temporary.code=THINKTHEN_COBOL_REPRESENTATION;
            break;
        }
        items[i]=values[i]->value;
    }
    int code=temporary.code ? temporary.code : own_value(&(thinkthen_complete_json_v1){
        .kind=THINKTHEN_COMPLETE_JSON_ARRAY_V1,.data.array={
            items,count
        }
    },out);
    release(&temporary);
    return code;
}
int TT_VALUE_OBJECT(const thinkthen_cobol_text *names,const tt_value *const *values,uint64_t count,
tt_value **out) {
    if(count && (!values || !names)) return THINKTHEN_COBOL_REPRESENTATION;
    arena temporary={
        0
    };
    thinkthen_complete_json_entry_v1 *items=allocate(&temporary,count,sizeof(*items));
    if(items) for(uint64_t i=0;
    i<count;
    i++) {
        if(!values[i]) {
            temporary.code=THINKTHEN_COBOL_REPRESENTATION;
            break;
        }
        items[i]=(thinkthen_complete_json_entry_v1){
            {
                names[i].data,names[i].len
            },values[i]->value
        };
    }
    int code=temporary.code ? temporary.code : own_value(&(thinkthen_complete_json_v1){
        .kind=THINKTHEN_COMPLETE_JSON_OBJECT_V1,.data.object={
            items,count
        }
    },out);
    release(&temporary);
    return code;
}
static void *convert_RequestCall_RequestArguments(arena *,const void *);
static void *convert_RequestSessionDescriptor_RequestSessionDescriptor(arena *,const void *);
static void *convert_EngineSettings_EngineSettings(arena *,const void *);
static void *convert_RequestInput_RequestInput(arena *,const void *);
static void *convert_RequestOptions_RequestOptions(arena *,const void *);
static void *convert_RequestQuestion_RequestQuestion(arena *,const void *);
static void *convert_RequestItem_RequestItem(arena *,const void *);
static void *convert_SessionSourceLocation_SessionSourceLocation(arena *,const void *);
static void *convert_EngineSettings_member_backend_Text(arena *,const void *);
static void *convert_EngineSettings_member_base_url_Text(arena *,const void *);
static void *convert_RequestBatch_RequestBatch(arena *,const void *);
static void *convert_CacheDocument_CacheDocument(arena *,const void *);
static void *convert_EngineSettings_member_max_estimated_input_tokens_total_EngineSettings_max_estimated_input_tokens_total(arena *,
const void *);
static void *convert_EngineSettings_member_max_request_bytes_Unsigned(arena *,const void *);
static void *convert_EngineSettings_member_max_requests_EngineSettings_max_requests(arena *,
const void *);
static void *convert_EngineSettings_member_max_requests_total_EngineSettings_max_estimated_input_tokens_total(arena *,
const void *);
static void *convert_EngineSettings_member_max_retries_Unsigned(arena *,const void *);
static void *convert_EngineSettings_member_model_Text(arena *,const void *);
static void *convert_EngineSettings_member_profile_Text(arena *,const void *);
static void *convert_EngineSettings_member_proxy_JsonValue(arena *,const void *);
static void *convert_EngineSettings_member_record_Text(arena *,const void *);
static void *convert_EngineSettings_member_refresh_cache_Boolean(arena *,const void *);
static void *convert_EngineSettings_member_replay_Text(arena *,const void *);
static void *convert_EngineSettings_member_throttle_Unsigned(arena *,const void *);
static void *convert_EngineSettings_member_timeout_Unsigned(arena *,const void *);
static void *convert_EngineSettings_member_usd_per_million_input_Text(arena *,const void *);
static void *convert_EngineSettings_member_usd_per_million_output_Text(arena *,const void *);
static void *convert_RequestInput_text_RequestInput_text(arena *,const void *);
static void *convert_RequestInput_json_RequestInput_json(arena *,const void *);
static void *convert_RequestInput_records_RequestInput_records(arena *,const void *);
static void *convert_RequestInput_units_RequestInput_units(arena *,const void *);
static void *convert_RequestInput_entities_RequestInput_entities(arena *,const void *);
static void *convert_RequestInput_source_RequestInput_source(arena *,const void *);
static void *convert_RequestInput_feed_RequestInput_feed(arena *,const void *);
static void *convert_RequestOptions_member_attempts_Boolean(arena *,const void *);
static void *convert_RequestOptions_member_context_Text(arena *,const void *);
static void *convert_RequestOptions_member_context_field_Text(arena *,const void *);
static void *convert_RequestOptions_member_deadline_ms_Integer(arena *,const void *);
static void *convert_RequestOptions_member_examples_RecognitionExample_list(arena *,const void *);
static void *convert_RequestOptions_member_examples_field_Text(arena *,const void *);
static void *convert_RequestOptions_member_field_Text_list(arena *,const void *);
static void *convert_RequestOptions_member_files_only_Boolean(arena *,const void *);
static void *convert_RequestOptions_member_max_requests_total_Unsigned(arena *,const void *);
static void *convert_RecognitionMode_RecognitionMode(arena *,const void *);
static void *convert_RequestOptions_member_model_Text(arena *,const void *);
static void *convert_RequestOptions_member_none_Boolean(arena *,const void *);
static void *convert_RequestOptions_member_options_field_Text(arena *,const void *);
static void *convert_RequestThreshold_RequestThreshold(arena *,const void *);
static void *convert_RequestOptions_member_seed_spans_RecognitionSeedSpan_list(arena *,const void *);
static void *convert_RequestOptions_member_seed_spans_field_Text(arena *,const void *);
static void *convert_RequestOptions_member_snippet_pieces_Unsigned(arena *,const void *);
static void *convert_RecognitionStageContext_RecognitionStageContext(arena *,const void *);
static void *convert_RequestOptions_member_top_Unsigned(arena *,const void *);
static void *convert_RequestQuestion_text_RequestQuestion_text(arena *,const void *);
static void *convert_RequestQuestion_definition_RequestQuestion_definition(arena *,const void *);
static void *convert_RequestQuestion_file_RequestQuestion_file(arena *,const void *);
static void *convert_RequestQuestion_name_RequestQuestion_name(arena *,const void *);
static void *convert_RequestQuestion_reference_RequestQuestion_reference(arena *,const void *);
static void *convert_ContextSchema_ContextSchema(arena *,const void *);
static void *convert_RequestItem_member_examples_RecognitionExample_list(arena *,const void *);
static void *convert_RequestItem_member_images_RequestImage_list(arena *,const void *);
static void *convert_RequestItem_member_options_OptionSchema_list(arena *,const void *);
static void *convert_RequestOriginal_RequestOriginal(arena *,const void *);
static void *convert_RequestItem_member_seed_spans_RecognitionSeedSpan_list(arena *,const void *);
static void *convert_SessionSourceLocation_member_file_Text(arena *,const void *);
static void *convert_SessionSourceLocation_member_first_line_Unsigned(arena *,const void *);
static void *convert_SessionSourceLocation_member_last_line_Unsigned(arena *,const void *);
static void *convert_RequestBatch_Integer_Value_Unsigned(arena *,const void *);
static void *convert_RequestBatch_Text_Value_Text(arena *,const void *);
static void *convert_CacheDocument_Text_Value_Text(arena *,const void *);
static void *convert_DisabledCache_DisabledCache(arena *,const void *);
static void *convert_EngineSettings_member_max_estimated_input_tokens_total_Integer_Value_Unsigned(arena *,
const void *);
static void *convert_EngineSettings_member_max_estimated_input_tokens_total_Null_Value_Null(arena *,
const void *);
static void *convert_EngineSettings_member_max_requests_Integer_Value_Unsigned(arena *,const void *);
static void *convert_EngineSettings_member_max_requests_Null_Value_Null(arena *,const void *);
static void *convert_EngineSettings_member_max_requests_total_Integer_Value_Unsigned(arena *,
const void *);
static void *convert_EngineSettings_member_max_requests_total_Null_Value_Null(arena *,const void *);
static void *convert_RequestInput_text_member_images_RequestImage_list(arena *,const void *);
static void *convert_RequestInput_text_member_text_Text(arena *,const void *);
static void *convert_RequestInput_json_member_images_RequestImage_list(arena *,const void *);
static void *convert_RequestInput_json_member_value_RequestInput_json_value(arena *,const void *);
static void *convert_RequestInput_records_member_items_RequestItem_list(arena *,const void *);
static void *convert_RequestInput_units_member_items_RequestItem_list(arena *,const void *);
static void *convert_RequestInput_entities_member_items_RequestItem_list(arena *,const void *);
static void *convert_RequestSource_RequestSource(arena *,const void *);
static void *convert_RequestFraming_RequestFraming(arena *,const void *);
static void *convert_RequestInput_feed_member_images_RequestImage_list(arena *,const void *);
static void *convert_RequestInput_feed_member_name_Text(arena *,const void *);
static void *convert_RequestReader_RequestReader(arena *,const void *);
static void *convert_RecognitionExample_RecognitionExample(arena *,const void *);
static void *convert_RequestOptions_member_field_item_Text(arena *,const void *);
static void *convert_RecognitionMode_whole_Value_Literal_whole(arena *,const void *);
static void *convert_RecognitionMode_boundary_only_Value_Literal_boundary_only(arena *,const void *);
static void *convert_RequestThreshold_Number_Value_Number(arena *,const void *);
static void *convert_RequestThreshold_Text_Value_Text(arena *,const void *);
static void *convert_RecognitionSeedSpan_RecognitionSeedSpan(arena *,const void *);
static void *convert_RecognitionStageContext_member_boundary_Text(arena *,const void *);
static void *convert_RecognitionStageContext_member_kind_edge_Text(arena *,const void *);
static void *convert_RecognitionStageContext_member_relation_Text(arena *,const void *);
static void *convert_RequestQuestion_text_member_text_Text(arena *,const void *);
static void *convert_RequestDefinition_RequestDefinition(arena *,const void *);
static void *convert_RequestQuestion_file_member_path_Text(arena *,const void *);
static void *convert_RequestQuestion_name_member_name_Text(arena *,const void *);
static void *convert_RequestQuestion_reference_member_reference_Text(arena *,const void *);
static void *convert_ContextSchema_Text_Value_Text(arena *,const void *);
static void *convert_ContextSchema_Object_Value_JsonValue_map(arena *,const void *);
static void *convert_RequestImage_RequestImage(arena *,const void *);
static void *convert_OptionSchema_OptionSchema(arena *,const void *);
static void *convert_RequestOriginal_text_RequestOriginal_text(arena *,const void *);
static void *convert_RequestOriginal_json_RequestOriginal_json(arena *,const void *);
static void *convert_ReaderMedia_ReaderMedia(arena *,const void *);
static void *convert_RequestSource_member_paths_Text_list(arena *,const void *);
static void *convert_RequestFraming_document_Value_Literal_document(arena *,const void *);
static void *convert_RequestFraming_lines_Value_Literal_lines(arena *,const void *);
static void *convert_RequestFraming_jsonl_Value_Literal_jsonl(arena *,const void *);
static void *convert_RequestFraming_csv_Value_Literal_csv(arena *,const void *);
static void *convert_RequestFraming_tsv_Value_Literal_tsv(arena *,const void *);
static void *convert_SourceUnit_SourceUnit(arena *,const void *);
static void *convert_RequestReader_member_window_Unsigned(arena *,const void *);
static void *convert_RecognitionExample_Text_Value_Text(arena *,const void *);
static void *convert_RecognitionExampleText_RecognitionExampleText(arena *,const void *);
static void *convert_RecognitionSeedSpan_member_end_Unsigned(arena *,const void *);
static void *convert_RecognitionSeedSpan_member_kind_Text(arena *,const void *);
static void *convert_RecognitionSeedSpan_member_start_Unsigned(arena *,const void *);
static void *convert_RequestDefinition_Authored_decide_RequestDefinition_Authored_decide(arena *,
const void *);
static void *convert_RequestDefinition_Authored_choose_RequestDefinition_Authored_choose(arena *,
const void *);
static void *convert_RequestDefinition_Authored_tag_RequestDefinition_Authored_tag(arena *,
const void *);
static void *convert_RequestDefinition_Authored_score_RequestDefinition_Authored_score(arena *,
const void *);
static void *convert_RequestDefinition_Authored_relate_RequestDefinition_Authored_relate(arena *,
const void *);
static void *convert_RequestDefinition_Authored_find_RequestDefinition_Authored_find(arena *,
const void *);
static void *convert_RequestDefinition_RecognizeDefinition_RequestDefinition_RecognizeDefinition(arena *,
const void *);
static void *convert_RequestDefinition_AnnotateDefinition_RequestDefinition_AnnotateDefinition(arena *,
const void *);
static void *convert_ContextSchema_Object_Value_entry_JsonValue(arena *,const void *);
static void *convert_RequestImage_file_RequestImage_file(arena *,const void *);
static void *convert_RequestImage_bytes_RequestImage_bytes(arena *,const void *);
static void *convert_OptionSchema_member_description_JsonValue(arena *,const void *);
static void *convert_OptionSchema_member_name_Text(arena *,const void *);
static void *convert_RequestOriginal_text_member_text_Text(arena *,const void *);
static void *convert_RequestOriginal_json_member_value_RequestOriginal_json_value(arena *,const void *);
static void *convert_ReaderMedia_text_Value_Literal_text(arena *,const void *);
static void *convert_ReaderMedia_image_Value_Literal_image(arena *,const void *);
static void *convert_RequestSource_member_paths_item_Text(arena *,const void *);
static void *convert_SourceUnit_line_Value_Literal_line(arena *,const void *);
static void *convert_SourceUnit_window_Value_Literal_window(arena *,const void *);
static void *convert_SourceUnit_file_Value_Literal_file(arena *,const void *);
static void *convert_RecognitionExampleText_member_entities_RecognitionExampleEntity_list(arena *,
const void *);
static void *convert_RecognitionExampleText_member_kinds_Text_list(arena *,const void *);
static void *convert_RecognitionExampleText_member_text_Text(arena *,const void *);
static void *convert_Authored_questionBatch_Authored_questionBatch(arena *,const void *);
static void *convert_Authored_inputDeclaration_Authored_inputDeclaration(arena *,const void *);
static void *convert_Authored_questionText_Authored_questionText(arena *,const void *);
static void *convert_Authored_criterion_Authored_criterion(arena *,const void *);
static void *convert_Authored_name_Authored_name(arena *,const void *);
static void *convert_RequestDefinition_Authored_decide_member_name_Text(arena *,const void *);
static void *convert_Authored_pointers_Authored_pointers(arena *,const void *);
static void *convert_Authored_profile_Authored_profile(arena *,const void *);
static void *convert_Authored_threshold_Authored_threshold(arena *,const void *);
static void *convert_RequestDefinition_Authored_decide_member_wording_version_Integer(arena *,
const void *);
static void *convert_RequestDefinition_Authored_choose_member_name_Text(arena *,const void *);
static void *convert_Authored_options_Authored_options(arena *,const void *);
static void *convert_Authored_cut_Authored_cut(arena *,const void *);
static void *convert_RequestDefinition_Authored_choose_member_wording_version_Integer(arena *,
const void *);
static void *convert_Authored_labels_Authored_labels(arena *,const void *);
static void *convert_RequestDefinition_Authored_tag_member_name_Text(arena *,const void *);
static void *convert_RequestDefinition_Authored_tag_member_wording_version_Integer(arena *,
const void *);
static void *convert_Authored_levels_Authored_levels(arena *,const void *);
static void *convert_RequestDefinition_Authored_score_member_name_Text(arena *,const void *);
static void *convert_RequestDefinition_Authored_score_member_wording_version_Integer(arena *,
const void *);
static void *convert_RequestDefinition_Authored_relate_member_name_Text(arena *,const void *);
static void *convert_RelateQuestion_RelateQuestion(arena *,const void *);
static void *convert_RequestDefinition_Authored_relate_member_wording_version_Integer(arena *,
const void *);
static void *convert_RequestDefinition_Authored_find_member_name_Text(arena *,const void *);
static void *convert_RequestDefinition_Authored_find_member_wording_version_Integer(arena *,
const void *);
static void *convert_RequestDefinition_RecognizeDefinition_member_name_Text(arena *,const void *);
static void *convert_RecognizeQuestion_RecognizeQuestion(arena *,const void *);
static void *convert_RequestDefinition_RecognizeDefinition_member_wording_version_Integer(arena *,
const void *);
static void *convert_RequestDefinition_AnnotateDefinition_member_batch_JsonValue(arena *,const void *);
static void *convert_RequestDefinition_AnnotateDefinition_member_questions_AnnotateQuestion_map(arena *,
const void *);
static void *convert_ImageMedia_ImageMedia(arena *,const void *);
static void *convert_RequestImage_file_member_path_Text(arena *,const void *);
static void *convert_RequestImage_bytes_member_bytes_Text(arena *,const void *);
static void *convert_RecognitionExampleEntity_RecognitionExampleEntity(arena *,const void *);
static void *convert_RecognitionExampleText_member_kinds_item_Text(arena *,const void *);
static void *convert_Authored_questionBatch_max_Value_Literal_max(arena *,const void *);
static void *convert_Authored_questionBatch_Integer_Value_Integer(arena *,const void *);
static void *convert_Authored_inputDeclaration_string_Authored_inputDeclaration_string(arena *,
const void *);
static void *convert_Authored_inputDeclaration_object_Authored_inputDeclaration_object(arena *,
const void *);
static void *convert_Authored_questionText_Text_Value_Text(arena *,const void *);
static void *convert_Authored_questionText_Object_Value_JsonValue_map(arena *,const void *);
static void *convert_Authored_questionText_Array_Value_JsonValue_list(arena *,const void *);
static void *convert_Authored_criterion_Text_Value_Text(arena *,const void *);
static void *convert_Authored_criterion_Object_Value_JsonValue_map(arena *,const void *);
static void *convert_Authored_criterion_Array_Value_JsonValue_list(arena *,const void *);
static void *convert_Authored_criterion_Null_Value_Null(arena *,const void *);
static void *convert_Authored_pointers_Text_Value_Text(arena *,const void *);
static void *convert_Authored_pointers_Array_Value_Text_list(arena *,const void *);
static void *convert_Authored_threshold_Number_Value_Number(arena *,const void *);
static void *convert_Authored_threshold_Text_Value_Text(arena *,const void *);
static void *convert_Authored_options_Array_Value_Authored_name_list(arena *,const void *);
static void *convert_Authored_options_Object_Value_Authored_description_map(arena *,const void *);
static void *convert_Authored_cut_Number_Value_Number(arena *,const void *);
static void *convert_Authored_cut_Text_Value_Text(arena *,const void *);
static void *convert_Authored_labels_Array_Value_Authored_name_list(arena *,const void *);
static void *convert_Authored_labels_Object_Value_Authored_description_map(arena *,const void *);
static void *convert_Authored_levels_Array_Value_Authored_name_list(arena *,const void *);
static void *convert_Authored_levels_Object_Value_Authored_criterion_map(arena *,const void *);
static void *convert_RelateFields_RelateFields(arena *,const void *);
static void *convert_RelateQuestion_member_relations_Authored_relation_list(arena *,const void *);
static void *convert_RecognizeQuestion_member_kinds_Authored_description_map(arena *,const void *);
static void *convert_RecognizeQuestion_member_relations_Authored_relation_list(arena *,const void *);
static void *convert_RecognizeQuestion_member_snippet_pieces_Unsigned(arena *,const void *);
static void *convert_AnnotateQuestion_AnnotateQuestion(arena *,const void *);
static void *convert_ImageMedia_image_jpeg_Value_Literal_image_jpeg(arena *,const void *);
static void *convert_ImageMedia_image_png_Value_Literal_image_png(arena *,const void *);
static void *convert_RecognitionExampleEntity_member_end_Unsigned(arena *,const void *);
static void *convert_RecognitionExampleEntity_member_kind_Text(arena *,const void *);
static void *convert_RecognitionExampleEntity_member_start_Unsigned(arena *,const void *);
static void *convert_Authored_inputDeclaration_object_member_properties_Authored_inputProperty_map(arena *,
const void *);
static void *convert_Authored_inputDeclaration_object_member_required_Text_list(arena *,const void *);
static void *convert_Authored_questionText_Object_Value_entry_JsonValue(arena *,const void *);
static void *convert_Authored_questionText_Array_Value_item_JsonValue(arena *,const void *);
static void *convert_Authored_criterion_Object_Value_entry_JsonValue(arena *,const void *);
static void *convert_Authored_criterion_Array_Value_item_JsonValue(arena *,const void *);
static void *convert_Authored_pointers_Array_Value_item_Text(arena *,const void *);
static void *convert_Authored_description_Authored_description(arena *,const void *);
static void *convert_RelateFields_member_kind_Text(arena *,const void *);
static void *convert_RelateFields_member_name_Text(arena *,const void *);
static void *convert_Authored_relation_Authored_relation(arena *,const void *);
static void *convert_AnnotateQuestion_decide_AnnotateQuestion_decide(arena *,const void *);
static void *convert_AnnotateQuestion_choose_AnnotateQuestion_choose(arena *,const void *);
static void *convert_AnnotateQuestion_tag_AnnotateQuestion_tag(arena *,const void *);
static void *convert_AnnotateQuestion_score_AnnotateQuestion_score(arena *,const void *);
static void *convert_Authored_inputProperty_Authored_inputProperty(arena *,const void *);
static void *convert_Authored_inputDeclaration_object_member_required_item_Text(arena *,const void *);
static void *convert_Authored_description_Text_Value_Text(arena *,const void *);
static void *convert_Authored_description_Object_Value_JsonValue_map(arena *,const void *);
static void *convert_Authored_description_Array_Value_JsonValue_list(arena *,const void *);
static void *convert_Authored_description_Null_Value_Null(arena *,const void *);
static void *convert_Authored_relation_member_either_Boolean(arena *,const void *);
static void *convert_Authored_relation_member_single_Boolean(arena *,const void *);
static void *convert_AnnotateQuestion_decide_member_name_Text(arena *,const void *);
static void *convert_AnnotateQuestion_decide_member_wording_version_Integer(arena *,const void *);
static void *convert_AnnotateQuestion_choose_member_name_Text(arena *,const void *);
static void *convert_AnnotateQuestion_choose_member_wording_version_Integer(arena *,const void *);
static void *convert_AnnotateQuestion_tag_member_name_Text(arena *,const void *);
static void *convert_AnnotateQuestion_tag_member_wording_version_Integer(arena *,const void *);
static void *convert_AnnotateQuestion_score_member_name_Text(arena *,const void *);
static void *convert_AnnotateQuestion_score_member_wording_version_Integer(arena *,const void *);
static void *convert_Authored_inputProperty_string_Authored_inputProperty_string(arena *,const void *);
static void *convert_Authored_inputProperty_number_Authored_inputProperty_number(arena *,const void *);
static void *convert_Authored_inputProperty_boolean_Authored_inputProperty_boolean(arena *,
const void *);
static void *convert_Authored_inputProperty_array_Authored_inputProperty_array(arena *,const void *);
static void *convert_Authored_description_Object_Value_entry_JsonValue(arena *,const void *);
static void *convert_Authored_description_Array_Value_item_JsonValue(arena *,const void *);
static void *convert_Authored_inputProperty_array_member_items_Authored_inputProperty_array_items(arena *,
const void *);
static void *convert_RequestCall_RequestArguments(arena *a,const void *input) {
    const thinkthen_cobol_RequestCall *p=input;
    thinkthen_request_arguments_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_input) out->m_input=convert_RequestInput_RequestInput(a,p->m_input);
    if(p->m_options) out->m_options=convert_RequestOptions_RequestOptions(a,p->m_options);
    if(p->m_question) out->m_question=convert_RequestQuestion_RequestQuestion(a,p->m_question);
    return out;
}
static void *convert_RequestSessionDescriptor_RequestSessionDescriptor(arena *a,const void *input) {
    const thinkthen_cobol_RequestSessionDescriptor *p=input;
    thinkthen_request_session_descriptor_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_item) out->m_item=convert_RequestItem_RequestItem(a,p->m_item);
    if(p->m_location) out->m_location=convert_SessionSourceLocation_SessionSourceLocation(a,p->m_location);
    return out;
}
static void *convert_EngineSettings_EngineSettings(arena *a,const void *input) {
    const thinkthen_cobol_Settings *p=input;
    thinkthen_request_engine_settings_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_backend) out->m_backend=convert_EngineSettings_member_backend_Text(a,p->m_backend);
    if(p->m_base_url) out->m_base_url=convert_EngineSettings_member_base_url_Text(a,p->m_base_url);
    if(p->m_batch) out->m_batch=convert_RequestBatch_RequestBatch(a,p->m_batch);
    if(p->m_cache) out->m_cache=convert_CacheDocument_CacheDocument(a,p->m_cache);
    if(p->m_max_estimated_input_tokens_total) out->m_max_estimated_input_tokens_total=convert_EngineSettings_member_max_estimated_input_tokens_total_EngineSettings_max_estimated_input_tokens_total(a,
    p->m_max_estimated_input_tokens_total);
    if(p->m_max_request_bytes) out->m_max_request_bytes=convert_EngineSettings_member_max_request_bytes_Unsigned(a,
    p->m_max_request_bytes);
    if(p->m_max_requests) out->m_max_requests=convert_EngineSettings_member_max_requests_EngineSettings_max_requests(a,
    p->m_max_requests);
    if(p->m_max_requests_total) out->m_max_requests_total=convert_EngineSettings_member_max_requests_total_EngineSettings_max_estimated_input_tokens_total(a,
    p->m_max_requests_total);
    if(p->m_max_retries) out->m_max_retries=convert_EngineSettings_member_max_retries_Unsigned(a,
    p->m_max_retries);
    if(p->m_model) out->m_model=convert_EngineSettings_member_model_Text(a,p->m_model);
    if(p->m_profile) out->m_profile=convert_EngineSettings_member_profile_Text(a,p->m_profile);
    if(p->m_proxy) out->m_proxy=convert_EngineSettings_member_proxy_JsonValue(a,p->m_proxy);
    if(p->m_record) out->m_record=convert_EngineSettings_member_record_Text(a,p->m_record);
    if(p->m_refresh_cache) out->m_refresh_cache=convert_EngineSettings_member_refresh_cache_Boolean(a,
    p->m_refresh_cache);
    if(p->m_replay) out->m_replay=convert_EngineSettings_member_replay_Text(a,p->m_replay);
    if(p->m_throttle) out->m_throttle=convert_EngineSettings_member_throttle_Unsigned(a,p->m_throttle);
    if(p->m_timeout) out->m_timeout=convert_EngineSettings_member_timeout_Unsigned(a,p->m_timeout);
    if(p->m_usd_per_million_input) out->m_usd_per_million_input=convert_EngineSettings_member_usd_per_million_input_Text(a,
    p->m_usd_per_million_input);
    if(p->m_usd_per_million_output) out->m_usd_per_million_output=convert_EngineSettings_member_usd_per_million_output_Text(a,
    p->m_usd_per_million_output);
    return out;
}
static void *convert_RequestInput_RequestInput(arena *a,const void *input) {
    const thinkthen_cobol_RequestInput *p=input;
    thinkthen_request_input_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.text=convert_RequestInput_text_RequestInput_text(a,p->value);
        break;
        case 2: out->data.json=convert_RequestInput_json_RequestInput_json(a,p->value);
        break;
        case 3: out->data.records=convert_RequestInput_records_RequestInput_records(a,p->value);
        break;
        case 4: out->data.units=convert_RequestInput_units_RequestInput_units(a,p->value);
        break;
        case 5: out->data.entities=convert_RequestInput_entities_RequestInput_entities(a,p->value);
        break;
        case 6: out->data.source=convert_RequestInput_source_RequestInput_source(a,p->value);
        break;
        case 7: out->data.feed=convert_RequestInput_feed_RequestInput_feed(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_RequestOptions_RequestOptions(arena *a,const void *input) {
    const thinkthen_cobol_RequestOptions *p=input;
    thinkthen_request_call_options_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_attempts) out->m_attempts=convert_RequestOptions_member_attempts_Boolean(a,p->m_attempts);
    if(p->m_batch) out->m_batch=convert_RequestBatch_RequestBatch(a,p->m_batch);
    if(p->m_context) out->m_context=convert_RequestOptions_member_context_Text(a,p->m_context);
    if(p->m_context_field) out->m_context_field=convert_RequestOptions_member_context_field_Text(a,
    p->m_context_field);
    if(p->m_deadline_ms) out->m_deadline_ms=convert_RequestOptions_member_deadline_ms_Integer(a,
    p->m_deadline_ms);
    if(p->m_examples) out->m_examples=convert_RequestOptions_member_examples_RecognitionExample_list(a,
    p->m_examples);
    if(p->m_examples_field) out->m_examples_field=convert_RequestOptions_member_examples_field_Text(a,
    p->m_examples_field);
    if(p->m_field) out->m_field=convert_RequestOptions_member_field_Text_list(a,p->m_field);
    if(p->m_files_only) out->m_files_only=convert_RequestOptions_member_files_only_Boolean(a,p->m_files_only);
    if(p->m_max_requests_total) out->m_max_requests_total=convert_RequestOptions_member_max_requests_total_Unsigned(a,
    p->m_max_requests_total);
    if(p->m_mode) out->m_mode=convert_RecognitionMode_RecognitionMode(a,p->m_mode);
    if(p->m_model) out->m_model=convert_RequestOptions_member_model_Text(a,p->m_model);
    if(p->m_none) out->m_none=convert_RequestOptions_member_none_Boolean(a,p->m_none);
    if(p->m_options_field) out->m_options_field=convert_RequestOptions_member_options_field_Text(a,
    p->m_options_field);
    if(p->m_relation_threshold) out->m_relation_threshold=convert_RequestThreshold_RequestThreshold(a,
    p->m_relation_threshold);
    if(p->m_seed_spans) out->m_seed_spans=convert_RequestOptions_member_seed_spans_RecognitionSeedSpan_list(a,
    p->m_seed_spans);
    if(p->m_seed_spans_field) out->m_seed_spans_field=convert_RequestOptions_member_seed_spans_field_Text(a,
    p->m_seed_spans_field);
    if(p->m_snippet_pieces) out->m_snippet_pieces=convert_RequestOptions_member_snippet_pieces_Unsigned(a,
    p->m_snippet_pieces);
    if(p->m_stage_context) out->m_stage_context=convert_RecognitionStageContext_RecognitionStageContext(a,
    p->m_stage_context);
    if(p->m_threshold) out->m_threshold=convert_RequestThreshold_RequestThreshold(a,p->m_threshold);
    if(p->m_top) out->m_top=convert_RequestOptions_member_top_Unsigned(a,p->m_top);
    return out;
}
static void *convert_RequestQuestion_RequestQuestion(arena *a,const void *input) {
    const thinkthen_cobol_RequestQuestion *p=input;
    thinkthen_request_question_selector_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.text=convert_RequestQuestion_text_RequestQuestion_text(a,p->value);
        break;
        case 2: out->data.definition=convert_RequestQuestion_definition_RequestQuestion_definition(a,
        p->value);
        break;
        case 3: out->data.file=convert_RequestQuestion_file_RequestQuestion_file(a,p->value);
        break;
        case 4: out->data.name=convert_RequestQuestion_name_RequestQuestion_name(a,p->value);
        break;
        case 5: out->data.reference=convert_RequestQuestion_reference_RequestQuestion_reference(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_RequestItem_RequestItem(arena *a,const void *input) {
    const thinkthen_cobol_RequestItem *p=input;
    thinkthen_request_item_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_context) out->m_context=convert_ContextSchema_ContextSchema(a,p->m_context);
    if(p->m_examples) out->m_examples=convert_RequestItem_member_examples_RecognitionExample_list(a,
    p->m_examples);
    if(p->m_images) out->m_images=convert_RequestItem_member_images_RequestImage_list(a,p->m_images);
    if(p->m_options) out->m_options=convert_RequestItem_member_options_OptionSchema_list(a,p->m_options);
    if(p->m_original) out->m_original=convert_RequestOriginal_RequestOriginal(a,p->m_original);
    if(p->m_seed_spans) out->m_seed_spans=convert_RequestItem_member_seed_spans_RecognitionSeedSpan_list(a,
    p->m_seed_spans);
    return out;
}
static void *convert_SessionSourceLocation_SessionSourceLocation(arena *a,const void *input) {
    const thinkthen_cobol_SessionSourceLocation *p=input;
    thinkthen_request_session_source_location_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_file) out->m_file=convert_SessionSourceLocation_member_file_Text(a,p->m_file);
    if(p->m_first_line) out->m_first_line=convert_SessionSourceLocation_member_first_line_Unsigned(a,
    p->m_first_line);
    if(p->m_last_line) out->m_last_line=convert_SessionSourceLocation_member_last_line_Unsigned(a,
    p->m_last_line);
    return out;
}
static void *convert_EngineSettings_member_backend_Text(arena *a,const void *input) {
    const thinkthen_cobol_SettingsBackend *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_EngineSettings_member_base_url_Text(arena *a,const void *input) {
    const thinkthen_cobol_SettingsBaseUrl *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestBatch_RequestBatch(arena *a,const void *input) {
    const thinkthen_cobol_RequestBatch *p=input;
    thinkthen_request_batch_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.unsigned_value=convert_RequestBatch_Integer_Value_Unsigned(a,p->value);
        break;
        case 2: out->data.text=convert_RequestBatch_Text_Value_Text(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_CacheDocument_CacheDocument(arena *a,const void *input) {
    const thinkthen_cobol_CacheDocument *p=input;
    thinkthen_request_cache_document_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.text=convert_CacheDocument_Text_Value_Text(a,p->value);
        break;
        case 2: out->data.disabled_cache=convert_DisabledCache_DisabledCache(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_EngineSettings_member_max_estimated_input_tokens_total_EngineSettings_max_estimated_input_tokens_total(arena *a,
const void *input) {
    const thinkthen_cobol_SettingsInputTokenBudget *p=input;
    thinkthen_request_optional_total_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.unsigned_value=convert_EngineSettings_member_max_estimated_input_tokens_total_Integer_Value_Unsigned(a,
        p->value);
        break;
        case 2: out->data.null=convert_EngineSettings_member_max_estimated_input_tokens_total_Null_Value_Null(a,
        p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_EngineSettings_member_max_request_bytes_Unsigned(arena *a,const void *input) {
    const thinkthen_cobol_SettingsMaxRequestBytes *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_EngineSettings_member_max_requests_EngineSettings_max_requests(arena *a,
const void *input) {
    const thinkthen_cobol_SettingsMaxRequests *p=input;
    thinkthen_request_optional_count_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.unsigned_value=convert_EngineSettings_member_max_requests_Integer_Value_Unsigned(a,
        p->value);
        break;
        case 2: out->data.null=convert_EngineSettings_member_max_requests_Null_Value_Null(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_EngineSettings_member_max_requests_total_EngineSettings_max_estimated_input_tokens_total(arena *a,
const void *input) {
    const thinkthen_cobol_SettingsMaxRequestsTotal *p=input;
    thinkthen_request_optional_total_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.unsigned_value=convert_EngineSettings_member_max_requests_total_Integer_Value_Unsigned(a,
        p->value);
        break;
        case 2: out->data.null=convert_EngineSettings_member_max_requests_total_Null_Value_Null(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_EngineSettings_member_max_retries_Unsigned(arena *a,const void *input) {
    const thinkthen_cobol_SettingsMaxRetries *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_EngineSettings_member_model_Text(arena *a,const void *input) {
    const thinkthen_cobol_SettingsModel *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_EngineSettings_member_profile_Text(arena *a,const void *input) {
    const thinkthen_cobol_SettingsProfile *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_EngineSettings_member_proxy_JsonValue(arena *a,const void *input) {
    const thinkthen_cobol_SettingsProxy *p=input;
    thinkthen_request_json_value_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_EngineSettings_member_record_Text(arena *a,const void *input) {
    const thinkthen_cobol_SettingsRecord *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_EngineSettings_member_refresh_cache_Boolean(arena *a,const void *input) {
    const thinkthen_cobol_SettingsRefreshCache *p=input;
    thinkthen_request_boolean_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->value>1) a->code=THINKTHEN_COBOL_REPRESENTATION;
    out->value=(uint32_t)p->value;
    return out;
}
static void *convert_EngineSettings_member_replay_Text(arena *a,const void *input) {
    const thinkthen_cobol_SettingsReplay *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_EngineSettings_member_throttle_Unsigned(arena *a,const void *input) {
    const thinkthen_cobol_SettingsThrottle *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_EngineSettings_member_timeout_Unsigned(arena *a,const void *input) {
    const thinkthen_cobol_SettingsTimeout *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_EngineSettings_member_usd_per_million_input_Text(arena *a,const void *input) {
    const thinkthen_cobol_SettingsUsdPerMillionInput *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_EngineSettings_member_usd_per_million_output_Text(arena *a,const void *input) {
    const thinkthen_cobol_SettingsUsdPerMillionOutput *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestInput_text_RequestInput_text(arena *a,const void *input) {
    const thinkthen_cobol_RequestInputText *p=input;
    thinkthen_request_input_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_images) out->m_images=convert_RequestInput_text_member_images_RequestImage_list(a,p->m_images);
    if(p->m_text) out->m_text=convert_RequestInput_text_member_text_Text(a,p->m_text);
    return out;
}
static void *convert_RequestInput_json_RequestInput_json(arena *a,const void *input) {
    const thinkthen_cobol_RequestInputJson *p=input;
    thinkthen_request_input_json_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_images) out->m_images=convert_RequestInput_json_member_images_RequestImage_list(a,p->m_images);
    if(p->m_value) out->m_value=convert_RequestInput_json_member_value_RequestInput_json_value(a,
    p->m_value);
    return out;
}
static void *convert_RequestInput_records_RequestInput_records(arena *a,const void *input) {
    const thinkthen_cobol_RequestInputRecords *p=input;
    thinkthen_request_input_records_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_items) out->m_items=convert_RequestInput_records_member_items_RequestItem_list(a,p->m_items);
    return out;
}
static void *convert_RequestInput_units_RequestInput_units(arena *a,const void *input) {
    const thinkthen_cobol_RequestInputUnits *p=input;
    thinkthen_request_input_units_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_items) out->m_items=convert_RequestInput_units_member_items_RequestItem_list(a,p->m_items);
    return out;
}
static void *convert_RequestInput_entities_RequestInput_entities(arena *a,const void *input) {
    const thinkthen_cobol_RequestInputEntities *p=input;
    thinkthen_request_input_entities_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_items) out->m_items=convert_RequestInput_entities_member_items_RequestItem_list(a,p->m_items);
    return out;
}
static void *convert_RequestInput_source_RequestInput_source(arena *a,const void *input) {
    const thinkthen_cobol_RequestInputSource *p=input;
    thinkthen_request_input_source_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_source) out->m_source=convert_RequestSource_RequestSource(a,p->m_source);
    return out;
}
static void *convert_RequestInput_feed_RequestInput_feed(arena *a,const void *input) {
    const thinkthen_cobol_RequestInputFeed *p=input;
    thinkthen_request_input_feed_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_framing) out->m_framing=convert_RequestFraming_RequestFraming(a,p->m_framing);
    if(p->m_images) out->m_images=convert_RequestInput_feed_member_images_RequestImage_list(a,p->m_images);
    if(p->m_name) out->m_name=convert_RequestInput_feed_member_name_Text(a,p->m_name);
    if(p->m_reading) out->m_reading=convert_RequestReader_RequestReader(a,p->m_reading);
    return out;
}
static void *convert_RequestOptions_member_attempts_Boolean(arena *a,const void *input) {
    const thinkthen_cobol_RequestOptionsAttempts *p=input;
    thinkthen_request_boolean_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->value>1) a->code=THINKTHEN_COBOL_REPRESENTATION;
    out->value=(uint32_t)p->value;
    return out;
}
static void *convert_RequestOptions_member_context_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestOptionsContext *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestOptions_member_context_field_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestOptionsContextField *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestOptions_member_deadline_ms_Integer(arena *a,const void *input) {
    const thinkthen_cobol_RequestOptionsDeadlineMs *p=input;
    thinkthen_request_integer_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RequestOptions_member_examples_RecognitionExample_list(arena *a,const void *input) {
    const thinkthen_cobol_RequestOptionsExamples *p=input;
    thinkthen_request_recognition_example_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_RecognitionExample_RecognitionExample(a,p->data[i]);
    return out;
}
static void *convert_RequestOptions_member_examples_field_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestOptionsExamplesField *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestOptions_member_field_Text_list(arena *a,const void *input) {
    const thinkthen_cobol_RequestOptionsField *p=input;
    thinkthen_request_text_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_RequestOptions_member_field_item_Text(a,p->data[i]);
    return out;
}
static void *convert_RequestOptions_member_files_only_Boolean(arena *a,const void *input) {
    const thinkthen_cobol_RequestOptionsFilesOnly *p=input;
    thinkthen_request_boolean_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->value>1) a->code=THINKTHEN_COBOL_REPRESENTATION;
    out->value=(uint32_t)p->value;
    return out;
}
static void *convert_RequestOptions_member_max_requests_total_Unsigned(arena *a,const void *input) {
    const thinkthen_cobol_RequestOptionsMaxRequestsTotal *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RecognitionMode_RecognitionMode(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionMode *p=input;
    thinkthen_request_recognition_mode_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.literal_whole=convert_RecognitionMode_whole_Value_Literal_whole(a,p->value);
        break;
        case 2: out->data.literal_boundary_only=convert_RecognitionMode_boundary_only_Value_Literal_boundary_only(a,
        p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_RequestOptions_member_model_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestOptionsModel *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestOptions_member_none_Boolean(arena *a,const void *input) {
    const thinkthen_cobol_RequestOptionsNone *p=input;
    thinkthen_request_boolean_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->value>1) a->code=THINKTHEN_COBOL_REPRESENTATION;
    out->value=(uint32_t)p->value;
    return out;
}
static void *convert_RequestOptions_member_options_field_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestOptionsOptionsField *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestThreshold_RequestThreshold(arena *a,const void *input) {
    const thinkthen_cobol_RequestThreshold *p=input;
    thinkthen_request_call_threshold_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.number=convert_RequestThreshold_Number_Value_Number(a,p->value);
        break;
        case 2: out->data.text=convert_RequestThreshold_Text_Value_Text(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_RequestOptions_member_seed_spans_RecognitionSeedSpan_list(arena *a,const void *input) {
    const thinkthen_cobol_RequestOptionsSeedSpans *p=input;
    thinkthen_request_recognition_seed_span_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_RecognitionSeedSpan_RecognitionSeedSpan(a,p->data[i]);
    return out;
}
static void *convert_RequestOptions_member_seed_spans_field_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestOptionsSeedSpansField *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestOptions_member_snippet_pieces_Unsigned(arena *a,const void *input) {
    const thinkthen_cobol_RequestOptionsSnippetPieces *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RecognitionStageContext_RecognitionStageContext(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionStageContext *p=input;
    thinkthen_request_recognition_stage_context_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_boundary) out->m_boundary=convert_RecognitionStageContext_member_boundary_Text(a,p->m_boundary);
    if(p->m_kind_edge) out->m_kind_edge=convert_RecognitionStageContext_member_kind_edge_Text(a,
    p->m_kind_edge);
    if(p->m_relation) out->m_relation=convert_RecognitionStageContext_member_relation_Text(a,p->m_relation);
    return out;
}
static void *convert_RequestOptions_member_top_Unsigned(arena *a,const void *input) {
    const thinkthen_cobol_RequestOptionsTop *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RequestQuestion_text_RequestQuestion_text(arena *a,const void *input) {
    const thinkthen_cobol_RequestQuestionText *p=input;
    thinkthen_request_question_selector_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_text) out->m_text=convert_RequestQuestion_text_member_text_Text(a,p->m_text);
    return out;
}
static void *convert_RequestQuestion_definition_RequestQuestion_definition(arena *a,const void *input) {
    const thinkthen_cobol_RequestQuestionDefinition *p=input;
    thinkthen_request_question_selector_definition_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_value) out->m_value=convert_RequestDefinition_RequestDefinition(a,p->m_value);
    return out;
}
static void *convert_RequestQuestion_file_RequestQuestion_file(arena *a,const void *input) {
    const thinkthen_cobol_RequestQuestionFile *p=input;
    thinkthen_request_question_selector_file_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_path) out->m_path=convert_RequestQuestion_file_member_path_Text(a,p->m_path);
    return out;
}
static void *convert_RequestQuestion_name_RequestQuestion_name(arena *a,const void *input) {
    const thinkthen_cobol_RequestQuestionName *p=input;
    thinkthen_request_question_selector_name_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_name) out->m_name=convert_RequestQuestion_name_member_name_Text(a,p->m_name);
    return out;
}
static void *convert_RequestQuestion_reference_RequestQuestion_reference(arena *a,const void *input) {
    const thinkthen_cobol_RequestQuestionReference *p=input;
    thinkthen_request_question_selector_reference_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_reference) out->m_reference=convert_RequestQuestion_reference_member_reference_Text(a,
    p->m_reference);
    return out;
}
static void *convert_ContextSchema_ContextSchema(arena *a,const void *input) {
    const thinkthen_cobol_ContextSchema *p=input;
    thinkthen_request_context_schema_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.text=convert_ContextSchema_Text_Value_Text(a,p->value);
        break;
        case 2: out->data.json_value_map=convert_ContextSchema_Object_Value_JsonValue_map(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_RequestItem_member_examples_RecognitionExample_list(arena *a,const void *input) {
    const thinkthen_cobol_RequestItemExamples *p=input;
    thinkthen_request_recognition_example_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_RecognitionExample_RecognitionExample(a,p->data[i]);
    return out;
}
static void *convert_RequestItem_member_images_RequestImage_list(arena *a,const void *input) {
    const thinkthen_cobol_RequestItemImages *p=input;
    thinkthen_request_image_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_RequestImage_RequestImage(a,p->data[i]);
    return out;
}
static void *convert_RequestItem_member_options_OptionSchema_list(arena *a,const void *input) {
    const thinkthen_cobol_RequestItemOptions *p=input;
    thinkthen_request_option_schema_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_OptionSchema_OptionSchema(a,p->data[i]);
    return out;
}
static void *convert_RequestOriginal_RequestOriginal(arena *a,const void *input) {
    const thinkthen_cobol_RequestOriginal *p=input;
    thinkthen_request_original_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.text=convert_RequestOriginal_text_RequestOriginal_text(a,p->value);
        break;
        case 2: out->data.json=convert_RequestOriginal_json_RequestOriginal_json(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_RequestItem_member_seed_spans_RecognitionSeedSpan_list(arena *a,const void *input) {
    const thinkthen_cobol_RequestItemSeedSpans *p=input;
    thinkthen_request_recognition_seed_span_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_RecognitionSeedSpan_RecognitionSeedSpan(a,p->data[i]);
    return out;
}
static void *convert_SessionSourceLocation_member_file_Text(arena *a,const void *input) {
    const thinkthen_cobol_SessionSourceLocationFile *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_SessionSourceLocation_member_first_line_Unsigned(arena *a,const void *input) {
    const thinkthen_cobol_SessionSourceLocationFirstLine *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_SessionSourceLocation_member_last_line_Unsigned(arena *a,const void *input) {
    const thinkthen_cobol_SessionSourceLocationLastLine *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RequestBatch_Integer_Value_Unsigned(arena *a,const void *input) {
    const thinkthen_cobol_RequestBatchIntegerValue *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RequestBatch_Text_Value_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestBatchTextValue *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_CacheDocument_Text_Value_Text(arena *a,const void *input) {
    const thinkthen_cobol_CacheDocumentTextValue *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_DisabledCache_DisabledCache(arena *a,const void *input) {
    const thinkthen_cobol_DisabledCache *p=input;
    thinkthen_request_disabled_cache_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_EngineSettings_member_max_estimated_input_tokens_total_Integer_Value_Unsigned(arena *a,
const void *input) {
    const thinkthen_cobol_SettingsInputTokenBudgetIntegerValue *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_EngineSettings_member_max_estimated_input_tokens_total_Null_Value_Null(arena *a,
const void *input) {
    const thinkthen_cobol_SettingsInputTokenBudgetNullValue *p=input;
    thinkthen_request_null_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_EngineSettings_member_max_requests_Integer_Value_Unsigned(arena *a,const void *input) {
    const thinkthen_cobol_SettingsMaxRequestsIntegerValue *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_EngineSettings_member_max_requests_Null_Value_Null(arena *a,const void *input) {
    const thinkthen_cobol_SettingsMaxRequestsNullValue *p=input;
    thinkthen_request_null_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_EngineSettings_member_max_requests_total_Integer_Value_Unsigned(arena *a,
const void *input) {
    const thinkthen_cobol_SettingsMaxRequestsTotalIntegerValue *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_EngineSettings_member_max_requests_total_Null_Value_Null(arena *a,const void *input) {
    const thinkthen_cobol_SettingsMaxRequestsTotalNullValue *p=input;
    thinkthen_request_null_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RequestInput_text_member_images_RequestImage_list(arena *a,const void *input) {
    const thinkthen_cobol_RequestInputTextImages *p=input;
    thinkthen_request_image_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_RequestImage_RequestImage(a,p->data[i]);
    return out;
}
static void *convert_RequestInput_text_member_text_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestInputTextText *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestInput_json_member_images_RequestImage_list(arena *a,const void *input) {
    const thinkthen_cobol_RequestInputJsonImages *p=input;
    thinkthen_request_image_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_RequestImage_RequestImage(a,p->data[i]);
    return out;
}
static void *convert_RequestInput_json_member_value_RequestInput_json_value(arena *a,const void *input) {
    const thinkthen_cobol_RequestInputJsonValue *p=input;
    thinkthen_request_input_json_value_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RequestInput_records_member_items_RequestItem_list(arena *a,const void *input) {
    const thinkthen_cobol_RequestInputRecordsItems *p=input;
    thinkthen_request_item_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_RequestItem_RequestItem(a,p->data[i]);
    return out;
}
static void *convert_RequestInput_units_member_items_RequestItem_list(arena *a,const void *input) {
    const thinkthen_cobol_RequestInputUnitsItems *p=input;
    thinkthen_request_item_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_RequestItem_RequestItem(a,p->data[i]);
    return out;
}
static void *convert_RequestInput_entities_member_items_RequestItem_list(arena *a,const void *input) {
    const thinkthen_cobol_RequestInputEntitiesItems *p=input;
    thinkthen_request_item_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_RequestItem_RequestItem(a,p->data[i]);
    return out;
}
static void *convert_RequestSource_RequestSource(arena *a,const void *input) {
    const thinkthen_cobol_RequestSource *p=input;
    thinkthen_request_source_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_framing) out->m_framing=convert_RequestFraming_RequestFraming(a,p->m_framing);
    if(p->m_media) out->m_media=convert_ReaderMedia_ReaderMedia(a,p->m_media);
    if(p->m_paths) out->m_paths=convert_RequestSource_member_paths_Text_list(a,p->m_paths);
    if(p->m_reading) out->m_reading=convert_RequestReader_RequestReader(a,p->m_reading);
    return out;
}
static void *convert_RequestFraming_RequestFraming(arena *a,const void *input) {
    const thinkthen_cobol_RequestFraming *p=input;
    thinkthen_request_framing_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.literal_document=convert_RequestFraming_document_Value_Literal_document(a,
        p->value);
        break;
        case 2: out->data.literal_lines=convert_RequestFraming_lines_Value_Literal_lines(a,p->value);
        break;
        case 3: out->data.literal_jsonl=convert_RequestFraming_jsonl_Value_Literal_jsonl(a,p->value);
        break;
        case 4: out->data.literal_csv=convert_RequestFraming_csv_Value_Literal_csv(a,p->value);
        break;
        case 5: out->data.literal_tsv=convert_RequestFraming_tsv_Value_Literal_tsv(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_RequestInput_feed_member_images_RequestImage_list(arena *a,const void *input) {
    const thinkthen_cobol_RequestInputFeedImages *p=input;
    thinkthen_request_image_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_RequestImage_RequestImage(a,p->data[i]);
    return out;
}
static void *convert_RequestInput_feed_member_name_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestInputFeedName *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestReader_RequestReader(arena *a,const void *input) {
    const thinkthen_cobol_RequestReader *p=input;
    thinkthen_request_reader_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_unit) out->m_unit=convert_SourceUnit_SourceUnit(a,p->m_unit);
    if(p->m_window) out->m_window=convert_RequestReader_member_window_Unsigned(a,p->m_window);
    return out;
}
static void *convert_RecognitionExample_RecognitionExample(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionExample *p=input;
    thinkthen_request_recognition_example_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.text=convert_RecognitionExample_Text_Value_Text(a,p->value);
        break;
        case 2: out->data.example_text=convert_RecognitionExampleText_RecognitionExampleText(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_RequestOptions_member_field_item_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestOptionsFieldItem *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RecognitionMode_whole_Value_Literal_whole(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionModeWholeValue *p=input;
    thinkthen_request_literal_whole_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RecognitionMode_boundary_only_Value_Literal_boundary_only(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionModeBoundaryOnlyValue *p=input;
    thinkthen_request_literal_boundary_only_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RequestThreshold_Number_Value_Number(arena *a,const void *input) {
    const thinkthen_cobol_RequestThresholdNumberValue *p=input;
    thinkthen_request_number_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RequestThreshold_Text_Value_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestThresholdTextValue *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RecognitionSeedSpan_RecognitionSeedSpan(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionSeedSpan *p=input;
    thinkthen_request_recognition_seed_span_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_end) out->m_end=convert_RecognitionSeedSpan_member_end_Unsigned(a,p->m_end);
    if(p->m_kind) out->m_kind=convert_RecognitionSeedSpan_member_kind_Text(a,p->m_kind);
    if(p->m_start) out->m_start=convert_RecognitionSeedSpan_member_start_Unsigned(a,p->m_start);
    return out;
}
static void *convert_RecognitionStageContext_member_boundary_Text(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionStageContextBoundary *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RecognitionStageContext_member_kind_edge_Text(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionStageContextKindEdge *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RecognitionStageContext_member_relation_Text(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionStageContextRelation *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestQuestion_text_member_text_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestQuestionTextText *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestDefinition_RequestDefinition(arena *a,const void *input) {
    const thinkthen_cobol_RequestDefinition *p=input;
    thinkthen_request_definition_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.authored_decide=convert_RequestDefinition_Authored_decide_RequestDefinition_Authored_decide(a,
        p->value);
        break;
        case 2: out->data.authored_choose=convert_RequestDefinition_Authored_choose_RequestDefinition_Authored_choose(a,
        p->value);
        break;
        case 3: out->data.authored_tag=convert_RequestDefinition_Authored_tag_RequestDefinition_Authored_tag(a,
        p->value);
        break;
        case 4: out->data.authored_score=convert_RequestDefinition_Authored_score_RequestDefinition_Authored_score(a,
        p->value);
        break;
        case 5: out->data.authored_relate=convert_RequestDefinition_Authored_relate_RequestDefinition_Authored_relate(a,
        p->value);
        break;
        case 6: out->data.authored_find=convert_RequestDefinition_Authored_find_RequestDefinition_Authored_find(a,
        p->value);
        break;
        case 7: out->data.recognize_definition=convert_RequestDefinition_RecognizeDefinition_RequestDefinition_RecognizeDefinition(a,
        p->value);
        break;
        case 8: out->data.annotate_definition=convert_RequestDefinition_AnnotateDefinition_RequestDefinition_AnnotateDefinition(a,
        p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_RequestQuestion_file_member_path_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestQuestionFilePath *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestQuestion_name_member_name_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestQuestionNameName *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestQuestion_reference_member_reference_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestQuestionReferenceReference *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_ContextSchema_Text_Value_Text(arena *a,const void *input) {
    const thinkthen_cobol_ContextSchemaTextValue *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_ContextSchema_Object_Value_JsonValue_map(arena *a,const void *input) {
    const thinkthen_cobol_ContextSchemaObjectValue *p=input;
    thinkthen_request_json_value_map_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && (!p->keys || !p->values)) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    thinkthen_request_json_value_map_entry_v1 *items=allocate(a,p->len,sizeof(*items));
    out->data=items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) {
        items[i].key=(thinkthen_complete_utf8_v1){
            p->keys[i].data,p->keys[i].len
        };
        items[i].value=convert_ContextSchema_Object_Value_entry_JsonValue(a,p->values[i]);
    }
    return out;
}
static void *convert_RequestImage_RequestImage(arena *a,const void *input) {
    const thinkthen_cobol_RequestImage *p=input;
    thinkthen_request_image_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.file=convert_RequestImage_file_RequestImage_file(a,p->value);
        break;
        case 2: out->data.bytes=convert_RequestImage_bytes_RequestImage_bytes(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_OptionSchema_OptionSchema(arena *a,const void *input) {
    const thinkthen_cobol_OptionSchema *p=input;
    thinkthen_request_option_schema_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_description) out->m_description=convert_OptionSchema_member_description_JsonValue(a,
    p->m_description);
    if(p->m_name) out->m_name=convert_OptionSchema_member_name_Text(a,p->m_name);
    return out;
}
static void *convert_RequestOriginal_text_RequestOriginal_text(arena *a,const void *input) {
    const thinkthen_cobol_RequestOriginalText *p=input;
    thinkthen_request_original_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_text) out->m_text=convert_RequestOriginal_text_member_text_Text(a,p->m_text);
    return out;
}
static void *convert_RequestOriginal_json_RequestOriginal_json(arena *a,const void *input) {
    const thinkthen_cobol_RequestOriginalJson *p=input;
    thinkthen_request_original_json_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_value) out->m_value=convert_RequestOriginal_json_member_value_RequestOriginal_json_value(a,
    p->m_value);
    return out;
}
static void *convert_ReaderMedia_ReaderMedia(arena *a,const void *input) {
    const thinkthen_cobol_ReaderMedia *p=input;
    thinkthen_request_reader_media_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.literal_text=convert_ReaderMedia_text_Value_Literal_text(a,p->value);
        break;
        case 2: out->data.literal_image=convert_ReaderMedia_image_Value_Literal_image(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_RequestSource_member_paths_Text_list(arena *a,const void *input) {
    const thinkthen_cobol_RequestSourcePaths *p=input;
    thinkthen_request_text_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_RequestSource_member_paths_item_Text(a,p->data[i]);
    return out;
}
static void *convert_RequestFraming_document_Value_Literal_document(arena *a,const void *input) {
    const thinkthen_cobol_RequestFramingDocumentValue *p=input;
    thinkthen_request_literal_document_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RequestFraming_lines_Value_Literal_lines(arena *a,const void *input) {
    const thinkthen_cobol_RequestFramingLinesValue *p=input;
    thinkthen_request_literal_lines_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RequestFraming_jsonl_Value_Literal_jsonl(arena *a,const void *input) {
    const thinkthen_cobol_RequestFramingJsonlValue *p=input;
    thinkthen_request_literal_jsonl_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RequestFraming_csv_Value_Literal_csv(arena *a,const void *input) {
    const thinkthen_cobol_RequestFramingCsvValue *p=input;
    thinkthen_request_literal_csv_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RequestFraming_tsv_Value_Literal_tsv(arena *a,const void *input) {
    const thinkthen_cobol_RequestFramingTsvValue *p=input;
    thinkthen_request_literal_tsv_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_SourceUnit_SourceUnit(arena *a,const void *input) {
    const thinkthen_cobol_SourceUnit *p=input;
    thinkthen_request_source_unit_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.literal_line=convert_SourceUnit_line_Value_Literal_line(a,p->value);
        break;
        case 2: out->data.literal_window=convert_SourceUnit_window_Value_Literal_window(a,p->value);
        break;
        case 3: out->data.literal_file=convert_SourceUnit_file_Value_Literal_file(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_RequestReader_member_window_Unsigned(arena *a,const void *input) {
    const thinkthen_cobol_RequestReaderWindow *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RecognitionExample_Text_Value_Text(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionExampleTextValue *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RecognitionExampleText_RecognitionExampleText(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionExampleText *p=input;
    thinkthen_request_recognition_example_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_entities) out->m_entities=convert_RecognitionExampleText_member_entities_RecognitionExampleEntity_list(a,
    p->m_entities);
    if(p->m_kinds) out->m_kinds=convert_RecognitionExampleText_member_kinds_Text_list(a,p->m_kinds);
    if(p->m_text) out->m_text=convert_RecognitionExampleText_member_text_Text(a,p->m_text);
    return out;
}
static void *convert_RecognitionSeedSpan_member_end_Unsigned(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionSeedSpanEnd *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RecognitionSeedSpan_member_kind_Text(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionSeedSpanKind *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RecognitionSeedSpan_member_start_Unsigned(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionSeedSpanStart *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RequestDefinition_Authored_decide_RequestDefinition_Authored_decide(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionDecide *p=input;
    thinkthen_request_definition_authored_decide_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_batch) out->m_batch=convert_Authored_questionBatch_Authored_questionBatch(a,p->m_batch);
    if(p->m_context_schema) out->m_context_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_context_schema);
    if(p->m_decide) out->m_decide=convert_Authored_questionText_Authored_questionText(a,p->m_decide);
    if(p->m_false) out->m_false=convert_Authored_criterion_Authored_criterion(a,p->m_false);
    if(p->m_item_schema) out->m_item_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_item_schema);
    if(p->m_model) out->m_model=convert_Authored_name_Authored_name(a,p->m_model);
    if(p->m_name) out->m_name=convert_RequestDefinition_Authored_decide_member_name_Text(a,p->m_name);
    if(p->m_on) out->m_on=convert_Authored_pointers_Authored_pointers(a,p->m_on);
    if(p->m_profile) out->m_profile=convert_Authored_profile_Authored_profile(a,p->m_profile);
    if(p->m_threshold) out->m_threshold=convert_Authored_threshold_Authored_threshold(a,p->m_threshold);
    if(p->m_true) out->m_true=convert_Authored_criterion_Authored_criterion(a,p->m_true);
    if(p->m_wording_version) out->m_wording_version=convert_RequestDefinition_Authored_decide_member_wording_version_Integer(a,
    p->m_wording_version);
    return out;
}
static void *convert_RequestDefinition_Authored_choose_RequestDefinition_Authored_choose(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionChoose *p=input;
    thinkthen_request_definition_authored_choose_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_batch) out->m_batch=convert_Authored_questionBatch_Authored_questionBatch(a,p->m_batch);
    if(p->m_choose) out->m_choose=convert_Authored_questionText_Authored_questionText(a,p->m_choose);
    if(p->m_context_schema) out->m_context_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_context_schema);
    if(p->m_item_schema) out->m_item_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_item_schema);
    if(p->m_model) out->m_model=convert_Authored_name_Authored_name(a,p->m_model);
    if(p->m_name) out->m_name=convert_RequestDefinition_Authored_choose_member_name_Text(a,p->m_name);
    if(p->m_on) out->m_on=convert_Authored_pointers_Authored_pointers(a,p->m_on);
    if(p->m_options) out->m_options=convert_Authored_options_Authored_options(a,p->m_options);
    if(p->m_profile) out->m_profile=convert_Authored_profile_Authored_profile(a,p->m_profile);
    if(p->m_threshold) out->m_threshold=convert_Authored_cut_Authored_cut(a,p->m_threshold);
    if(p->m_wording_version) out->m_wording_version=convert_RequestDefinition_Authored_choose_member_wording_version_Integer(a,
    p->m_wording_version);
    return out;
}
static void *convert_RequestDefinition_Authored_tag_RequestDefinition_Authored_tag(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionTag *p=input;
    thinkthen_request_definition_authored_tag_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_batch) out->m_batch=convert_Authored_questionBatch_Authored_questionBatch(a,p->m_batch);
    if(p->m_context_schema) out->m_context_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_context_schema);
    if(p->m_item_schema) out->m_item_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_item_schema);
    if(p->m_labels) out->m_labels=convert_Authored_labels_Authored_labels(a,p->m_labels);
    if(p->m_model) out->m_model=convert_Authored_name_Authored_name(a,p->m_model);
    if(p->m_name) out->m_name=convert_RequestDefinition_Authored_tag_member_name_Text(a,p->m_name);
    if(p->m_on) out->m_on=convert_Authored_pointers_Authored_pointers(a,p->m_on);
    if(p->m_profile) out->m_profile=convert_Authored_profile_Authored_profile(a,p->m_profile);
    if(p->m_tag) out->m_tag=convert_Authored_questionText_Authored_questionText(a,p->m_tag);
    if(p->m_threshold) out->m_threshold=convert_Authored_cut_Authored_cut(a,p->m_threshold);
    if(p->m_wording_version) out->m_wording_version=convert_RequestDefinition_Authored_tag_member_wording_version_Integer(a,
    p->m_wording_version);
    return out;
}
static void *convert_RequestDefinition_Authored_score_RequestDefinition_Authored_score(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionScore *p=input;
    thinkthen_request_definition_authored_score_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_batch) out->m_batch=convert_Authored_questionBatch_Authored_questionBatch(a,p->m_batch);
    if(p->m_context_schema) out->m_context_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_context_schema);
    if(p->m_item_schema) out->m_item_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_item_schema);
    if(p->m_levels) out->m_levels=convert_Authored_levels_Authored_levels(a,p->m_levels);
    if(p->m_model) out->m_model=convert_Authored_name_Authored_name(a,p->m_model);
    if(p->m_name) out->m_name=convert_RequestDefinition_Authored_score_member_name_Text(a,p->m_name);
    if(p->m_on) out->m_on=convert_Authored_pointers_Authored_pointers(a,p->m_on);
    if(p->m_profile) out->m_profile=convert_Authored_profile_Authored_profile(a,p->m_profile);
    if(p->m_score) out->m_score=convert_Authored_questionText_Authored_questionText(a,p->m_score);
    if(p->m_wording_version) out->m_wording_version=convert_RequestDefinition_Authored_score_member_wording_version_Integer(a,
    p->m_wording_version);
    return out;
}
static void *convert_RequestDefinition_Authored_relate_RequestDefinition_Authored_relate(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionRelate *p=input;
    thinkthen_request_definition_authored_relate_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_context_schema) out->m_context_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_context_schema);
    if(p->m_item_schema) out->m_item_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_item_schema);
    if(p->m_model) out->m_model=convert_Authored_name_Authored_name(a,p->m_model);
    if(p->m_name) out->m_name=convert_RequestDefinition_Authored_relate_member_name_Text(a,p->m_name);
    if(p->m_profile) out->m_profile=convert_Authored_profile_Authored_profile(a,p->m_profile);
    if(p->m_relate) out->m_relate=convert_RelateQuestion_RelateQuestion(a,p->m_relate);
    if(p->m_threshold) out->m_threshold=convert_Authored_cut_Authored_cut(a,p->m_threshold);
    if(p->m_wording_version) out->m_wording_version=convert_RequestDefinition_Authored_relate_member_wording_version_Integer(a,
    p->m_wording_version);
    return out;
}
static void *convert_RequestDefinition_Authored_find_RequestDefinition_Authored_find(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionFind *p=input;
    thinkthen_request_definition_authored_find_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_context_schema) out->m_context_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_context_schema);
    if(p->m_find) out->m_find=convert_Authored_questionText_Authored_questionText(a,p->m_find);
    if(p->m_item_schema) out->m_item_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_item_schema);
    if(p->m_model) out->m_model=convert_Authored_name_Authored_name(a,p->m_model);
    if(p->m_name) out->m_name=convert_RequestDefinition_Authored_find_member_name_Text(a,p->m_name);
    if(p->m_on) out->m_on=convert_Authored_pointers_Authored_pointers(a,p->m_on);
    if(p->m_profile) out->m_profile=convert_Authored_profile_Authored_profile(a,p->m_profile);
    if(p->m_wording_version) out->m_wording_version=convert_RequestDefinition_Authored_find_member_wording_version_Integer(a,
    p->m_wording_version);
    return out;
}
static void *convert_RequestDefinition_RecognizeDefinition_RequestDefinition_RecognizeDefinition(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionRecognizeDefinition *p=input;
    thinkthen_request_definition_recognize_definition_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_context_schema) out->m_context_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_context_schema);
    if(p->m_item_schema) out->m_item_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_item_schema);
    if(p->m_model) out->m_model=convert_Authored_name_Authored_name(a,p->m_model);
    if(p->m_name) out->m_name=convert_RequestDefinition_RecognizeDefinition_member_name_Text(a,
    p->m_name);
    if(p->m_on) out->m_on=convert_Authored_pointers_Authored_pointers(a,p->m_on);
    if(p->m_profile) out->m_profile=convert_Authored_profile_Authored_profile(a,p->m_profile);
    if(p->m_recognize) out->m_recognize=convert_RecognizeQuestion_RecognizeQuestion(a,p->m_recognize);
    if(p->m_relation_threshold) out->m_relation_threshold=convert_Authored_cut_Authored_cut(a,p->m_relation_threshold);
    if(p->m_threshold) out->m_threshold=convert_Authored_cut_Authored_cut(a,p->m_threshold);
    if(p->m_wording_version) out->m_wording_version=convert_RequestDefinition_RecognizeDefinition_member_wording_version_Integer(a,
    p->m_wording_version);
    return out;
}
static void *convert_RequestDefinition_AnnotateDefinition_RequestDefinition_AnnotateDefinition(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionAnnotateDefinition *p=input;
    thinkthen_request_definition_annotate_definition_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_batch) out->m_batch=convert_RequestDefinition_AnnotateDefinition_member_batch_JsonValue(a,
    p->m_batch);
    if(p->m_profile) out->m_profile=convert_Authored_profile_Authored_profile(a,p->m_profile);
    if(p->m_questions) out->m_questions=convert_RequestDefinition_AnnotateDefinition_member_questions_AnnotateQuestion_map(a,
    p->m_questions);
    if(p->m_threshold) out->m_threshold=convert_Authored_threshold_Authored_threshold(a,p->m_threshold);
    return out;
}
static void *convert_ContextSchema_Object_Value_entry_JsonValue(arena *a,const void *input) {
    const thinkthen_cobol_ContextSchemaObjectValueEntry *p=input;
    thinkthen_request_json_value_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RequestImage_file_RequestImage_file(arena *a,const void *input) {
    const thinkthen_cobol_RequestImageFile *p=input;
    thinkthen_request_image_file_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_media) out->m_media=convert_ImageMedia_ImageMedia(a,p->m_media);
    if(p->m_path) out->m_path=convert_RequestImage_file_member_path_Text(a,p->m_path);
    return out;
}
static void *convert_RequestImage_bytes_RequestImage_bytes(arena *a,const void *input) {
    const thinkthen_cobol_RequestImageBytes *p=input;
    thinkthen_request_image_bytes_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_bytes) out->m_bytes=convert_RequestImage_bytes_member_bytes_Text(a,p->m_bytes);
    if(p->m_media) out->m_media=convert_ImageMedia_ImageMedia(a,p->m_media);
    return out;
}
static void *convert_OptionSchema_member_description_JsonValue(arena *a,const void *input) {
    const thinkthen_cobol_OptionSchemaDescription *p=input;
    thinkthen_request_json_value_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_OptionSchema_member_name_Text(arena *a,const void *input) {
    const thinkthen_cobol_OptionSchemaName *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestOriginal_text_member_text_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestOriginalTextText *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestOriginal_json_member_value_RequestOriginal_json_value(arena *a,
const void *input) {
    const thinkthen_cobol_RequestOriginalJsonValue *p=input;
    thinkthen_request_original_json_value_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_ReaderMedia_text_Value_Literal_text(arena *a,const void *input) {
    const thinkthen_cobol_ReaderMediaTextValue *p=input;
    thinkthen_request_literal_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_ReaderMedia_image_Value_Literal_image(arena *a,const void *input) {
    const thinkthen_cobol_ReaderMediaImageValue *p=input;
    thinkthen_request_literal_image_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RequestSource_member_paths_item_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestSourcePathsItem *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_SourceUnit_line_Value_Literal_line(arena *a,const void *input) {
    const thinkthen_cobol_SourceUnitLineValue *p=input;
    thinkthen_request_literal_line_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_SourceUnit_window_Value_Literal_window(arena *a,const void *input) {
    const thinkthen_cobol_SourceUnitWindowValue *p=input;
    thinkthen_request_literal_window_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_SourceUnit_file_Value_Literal_file(arena *a,const void *input) {
    const thinkthen_cobol_SourceUnitFileValue *p=input;
    thinkthen_request_literal_file_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RecognitionExampleText_member_entities_RecognitionExampleEntity_list(arena *a,
const void *input) {
    const thinkthen_cobol_RecognitionExampleTextEntities *p=input;
    thinkthen_request_recognition_example_entity_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_RecognitionExampleEntity_RecognitionExampleEntity(a,p->data[i]);
    return out;
}
static void *convert_RecognitionExampleText_member_kinds_Text_list(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionExampleTextKinds *p=input;
    thinkthen_request_text_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_RecognitionExampleText_member_kinds_item_Text(a,p->data[i]);
    return out;
}
static void *convert_RecognitionExampleText_member_text_Text(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionExampleTextText *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_Authored_questionBatch_Authored_questionBatch(arena *a,const void *input) {
    const thinkthen_cobol_QuestionBatch *p=input;
    thinkthen_request_question_batch_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.literal_max=convert_Authored_questionBatch_max_Value_Literal_max(a,p->value);
        break;
        case 2: out->data.integer=convert_Authored_questionBatch_Integer_Value_Integer(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_Authored_inputDeclaration_Authored_inputDeclaration(arena *a,const void *input) {
    const thinkthen_cobol_InputDeclaration *p=input;
    thinkthen_request_input_declaration_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.string=convert_Authored_inputDeclaration_string_Authored_inputDeclaration_string(a,
        p->value);
        break;
        case 2: out->data.object=convert_Authored_inputDeclaration_object_Authored_inputDeclaration_object(a,
        p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_Authored_questionText_Authored_questionText(arena *a,const void *input) {
    const thinkthen_cobol_QuestionText *p=input;
    thinkthen_request_question_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.text=convert_Authored_questionText_Text_Value_Text(a,p->value);
        break;
        case 2: out->data.json_value_map=convert_Authored_questionText_Object_Value_JsonValue_map(a,
        p->value);
        break;
        case 3: out->data.json_value_list=convert_Authored_questionText_Array_Value_JsonValue_list(a,
        p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_Authored_criterion_Authored_criterion(arena *a,const void *input) {
    const thinkthen_cobol_Criterion *p=input;
    thinkthen_request_criterion_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.text=convert_Authored_criterion_Text_Value_Text(a,p->value);
        break;
        case 2: out->data.json_value_map=convert_Authored_criterion_Object_Value_JsonValue_map(a,p->value);
        break;
        case 3: out->data.json_value_list=convert_Authored_criterion_Array_Value_JsonValue_list(a,p->value);
        break;
        case 4: out->data.null=convert_Authored_criterion_Null_Value_Null(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_Authored_name_Authored_name(arena *a,const void *input) {
    const thinkthen_cobol_Name *p=input;
    thinkthen_request_name_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestDefinition_Authored_decide_member_name_Text(arena *a,const void *input) {
    const thinkthen_cobol_DefinitionDecideName *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_Authored_pointers_Authored_pointers(arena *a,const void *input) {
    const thinkthen_cobol_Pointers *p=input;
    thinkthen_request_pointers_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.text=convert_Authored_pointers_Text_Value_Text(a,p->value);
        break;
        case 2: out->data.text_list=convert_Authored_pointers_Array_Value_Text_list(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_Authored_profile_Authored_profile(arena *a,const void *input) {
    const thinkthen_cobol_Profile *p=input;
    thinkthen_request_profile_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_Authored_threshold_Authored_threshold(arena *a,const void *input) {
    const thinkthen_cobol_Threshold *p=input;
    thinkthen_request_threshold_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.number=convert_Authored_threshold_Number_Value_Number(a,p->value);
        break;
        case 2: out->data.text=convert_Authored_threshold_Text_Value_Text(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_RequestDefinition_Authored_decide_member_wording_version_Integer(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionDecideWordingVersion *p=input;
    thinkthen_request_integer_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RequestDefinition_Authored_choose_member_name_Text(arena *a,const void *input) {
    const thinkthen_cobol_DefinitionChooseName *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_Authored_options_Authored_options(arena *a,const void *input) {
    const thinkthen_cobol_Options *p=input;
    thinkthen_request_options_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.name_list=convert_Authored_options_Array_Value_Authored_name_list(a,p->value);
        break;
        case 2: out->data.description_map=convert_Authored_options_Object_Value_Authored_description_map(a,
        p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_Authored_cut_Authored_cut(arena *a,const void *input) {
    const thinkthen_cobol_Cut *p=input;
    thinkthen_request_cut_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.number=convert_Authored_cut_Number_Value_Number(a,p->value);
        break;
        case 2: out->data.text=convert_Authored_cut_Text_Value_Text(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_RequestDefinition_Authored_choose_member_wording_version_Integer(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionChooseWordingVersion *p=input;
    thinkthen_request_integer_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_Authored_labels_Authored_labels(arena *a,const void *input) {
    const thinkthen_cobol_Labels *p=input;
    thinkthen_request_labels_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.name_list=convert_Authored_labels_Array_Value_Authored_name_list(a,p->value);
        break;
        case 2: out->data.description_map=convert_Authored_labels_Object_Value_Authored_description_map(a,
        p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_RequestDefinition_Authored_tag_member_name_Text(arena *a,const void *input) {
    const thinkthen_cobol_DefinitionTagName *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestDefinition_Authored_tag_member_wording_version_Integer(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionTagWordingVersion *p=input;
    thinkthen_request_integer_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_Authored_levels_Authored_levels(arena *a,const void *input) {
    const thinkthen_cobol_Levels *p=input;
    thinkthen_request_levels_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.name_list=convert_Authored_levels_Array_Value_Authored_name_list(a,p->value);
        break;
        case 2: out->data.criterion_map=convert_Authored_levels_Object_Value_Authored_criterion_map(a,
        p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_RequestDefinition_Authored_score_member_name_Text(arena *a,const void *input) {
    const thinkthen_cobol_DefinitionScoreName *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestDefinition_Authored_score_member_wording_version_Integer(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionScoreWordingVersion *p=input;
    thinkthen_request_integer_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RequestDefinition_Authored_relate_member_name_Text(arena *a,const void *input) {
    const thinkthen_cobol_DefinitionRelateName *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RelateQuestion_RelateQuestion(arena *a,const void *input) {
    const thinkthen_cobol_RelateQuestion *p=input;
    thinkthen_request_relate_question_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_fields) out->m_fields=convert_RelateFields_RelateFields(a,p->m_fields);
    if(p->m_relations) out->m_relations=convert_RelateQuestion_member_relations_Authored_relation_list(a,
    p->m_relations);
    return out;
}
static void *convert_RequestDefinition_Authored_relate_member_wording_version_Integer(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionRelateWordingVersion *p=input;
    thinkthen_request_integer_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RequestDefinition_Authored_find_member_name_Text(arena *a,const void *input) {
    const thinkthen_cobol_DefinitionFindName *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestDefinition_Authored_find_member_wording_version_Integer(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionFindWordingVersion *p=input;
    thinkthen_request_integer_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RequestDefinition_RecognizeDefinition_member_name_Text(arena *a,const void *input) {
    const thinkthen_cobol_DefinitionRecognizeDefinitionName *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RecognizeQuestion_RecognizeQuestion(arena *a,const void *input) {
    const thinkthen_cobol_RecognizeQuestion *p=input;
    thinkthen_request_recognize_question_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_entity_definition) out->m_entity_definition=convert_Authored_questionText_Authored_questionText(a,
    p->m_entity_definition);
    if(p->m_instructions) out->m_instructions=convert_Authored_questionText_Authored_questionText(a,
    p->m_instructions);
    if(p->m_kinds) out->m_kinds=convert_RecognizeQuestion_member_kinds_Authored_description_map(a,
    p->m_kinds);
    if(p->m_mode) out->m_mode=convert_RecognitionMode_RecognitionMode(a,p->m_mode);
    if(p->m_relations) out->m_relations=convert_RecognizeQuestion_member_relations_Authored_relation_list(a,
    p->m_relations);
    if(p->m_snippet_pieces) out->m_snippet_pieces=convert_RecognizeQuestion_member_snippet_pieces_Unsigned(a,
    p->m_snippet_pieces);
    if(p->m_stage_context) out->m_stage_context=convert_RecognitionStageContext_RecognitionStageContext(a,
    p->m_stage_context);
    return out;
}
static void *convert_RequestDefinition_RecognizeDefinition_member_wording_version_Integer(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionRecognizeDefinitionWordingVersion *p=input;
    thinkthen_request_integer_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RequestDefinition_AnnotateDefinition_member_batch_JsonValue(arena *a,const void *input) {
    const thinkthen_cobol_DefinitionAnnotateDefinitionBatch *p=input;
    thinkthen_request_json_value_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RequestDefinition_AnnotateDefinition_member_questions_AnnotateQuestion_map(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionAnnotateDefinitionQuestions *p=input;
    thinkthen_request_annotate_question_map_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && (!p->keys || !p->values)) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    thinkthen_request_annotate_question_map_entry_v1 *items=allocate(a,p->len,sizeof(*items));
    out->data=items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) {
        items[i].key=(thinkthen_complete_utf8_v1){
            p->keys[i].data,p->keys[i].len
        };
        items[i].value=convert_AnnotateQuestion_AnnotateQuestion(a,p->values[i]);
    }
    return out;
}
static void *convert_ImageMedia_ImageMedia(arena *a,const void *input) {
    const thinkthen_cobol_ImageMedia *p=input;
    thinkthen_request_image_media_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.literal_image_jpeg=convert_ImageMedia_image_jpeg_Value_Literal_image_jpeg(a,
        p->value);
        break;
        case 2: out->data.literal_image_png=convert_ImageMedia_image_png_Value_Literal_image_png(a,
        p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_RequestImage_file_member_path_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestImageFilePath *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RequestImage_bytes_member_bytes_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestImageBytesBytes *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RecognitionExampleEntity_RecognitionExampleEntity(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionExampleEntity *p=input;
    thinkthen_request_recognition_example_entity_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_end) out->m_end=convert_RecognitionExampleEntity_member_end_Unsigned(a,p->m_end);
    if(p->m_kind) out->m_kind=convert_RecognitionExampleEntity_member_kind_Text(a,p->m_kind);
    if(p->m_start) out->m_start=convert_RecognitionExampleEntity_member_start_Unsigned(a,p->m_start);
    return out;
}
static void *convert_RecognitionExampleText_member_kinds_item_Text(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionExampleTextKindsItem *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_Authored_questionBatch_max_Value_Literal_max(arena *a,const void *input) {
    const thinkthen_cobol_QuestionBatchMaxValue *p=input;
    thinkthen_request_literal_max_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_Authored_questionBatch_Integer_Value_Integer(arena *a,const void *input) {
    const thinkthen_cobol_QuestionBatchIntegerValue *p=input;
    thinkthen_request_integer_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_Authored_inputDeclaration_string_Authored_inputDeclaration_string(arena *a,
const void *input) {
    const thinkthen_cobol_InputDeclarationString *p=input;
    thinkthen_request_input_declaration_string_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    return out;
}
static void *convert_Authored_inputDeclaration_object_Authored_inputDeclaration_object(arena *a,
const void *input) {
    const thinkthen_cobol_InputDeclarationObject *p=input;
    thinkthen_request_input_declaration_object_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_properties) out->m_properties=convert_Authored_inputDeclaration_object_member_properties_Authored_inputProperty_map(a,
    p->m_properties);
    if(p->m_required) out->m_required=convert_Authored_inputDeclaration_object_member_required_Text_list(a,
    p->m_required);
    return out;
}
static void *convert_Authored_questionText_Text_Value_Text(arena *a,const void *input) {
    const thinkthen_cobol_QuestionTextTextValue *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_Authored_questionText_Object_Value_JsonValue_map(arena *a,const void *input) {
    const thinkthen_cobol_QuestionTextObjectValue *p=input;
    thinkthen_request_json_value_map_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && (!p->keys || !p->values)) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    thinkthen_request_json_value_map_entry_v1 *items=allocate(a,p->len,sizeof(*items));
    out->data=items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) {
        items[i].key=(thinkthen_complete_utf8_v1){
            p->keys[i].data,p->keys[i].len
        };
        items[i].value=convert_Authored_questionText_Object_Value_entry_JsonValue(a,p->values[i]);
    }
    return out;
}
static void *convert_Authored_questionText_Array_Value_JsonValue_list(arena *a,const void *input) {
    const thinkthen_cobol_QuestionTextArrayValue *p=input;
    thinkthen_request_json_value_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_Authored_questionText_Array_Value_item_JsonValue(a,p->data[i]);
    return out;
}
static void *convert_Authored_criterion_Text_Value_Text(arena *a,const void *input) {
    const thinkthen_cobol_CriterionTextValue *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_Authored_criterion_Object_Value_JsonValue_map(arena *a,const void *input) {
    const thinkthen_cobol_CriterionObjectValue *p=input;
    thinkthen_request_json_value_map_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && (!p->keys || !p->values)) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    thinkthen_request_json_value_map_entry_v1 *items=allocate(a,p->len,sizeof(*items));
    out->data=items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) {
        items[i].key=(thinkthen_complete_utf8_v1){
            p->keys[i].data,p->keys[i].len
        };
        items[i].value=convert_Authored_criterion_Object_Value_entry_JsonValue(a,p->values[i]);
    }
    return out;
}
static void *convert_Authored_criterion_Array_Value_JsonValue_list(arena *a,const void *input) {
    const thinkthen_cobol_CriterionArrayValue *p=input;
    thinkthen_request_json_value_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_Authored_criterion_Array_Value_item_JsonValue(a,p->data[i]);
    return out;
}
static void *convert_Authored_criterion_Null_Value_Null(arena *a,const void *input) {
    const thinkthen_cobol_CriterionNullValue *p=input;
    thinkthen_request_null_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_Authored_pointers_Text_Value_Text(arena *a,const void *input) {
    const thinkthen_cobol_PointersTextValue *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_Authored_pointers_Array_Value_Text_list(arena *a,const void *input) {
    const thinkthen_cobol_PointersArrayValue *p=input;
    thinkthen_request_text_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_Authored_pointers_Array_Value_item_Text(a,p->data[i]);
    return out;
}
static void *convert_Authored_threshold_Number_Value_Number(arena *a,const void *input) {
    const thinkthen_cobol_ThresholdNumberValue *p=input;
    thinkthen_request_number_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_Authored_threshold_Text_Value_Text(arena *a,const void *input) {
    const thinkthen_cobol_ThresholdTextValue *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_Authored_options_Array_Value_Authored_name_list(arena *a,const void *input) {
    const thinkthen_cobol_OptionsArrayValue *p=input;
    thinkthen_request_name_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_Authored_name_Authored_name(a,p->data[i]);
    return out;
}
static void *convert_Authored_options_Object_Value_Authored_description_map(arena *a,const void *input) {
    const thinkthen_cobol_OptionsObjectValue *p=input;
    thinkthen_request_description_map_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && (!p->keys || !p->values)) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    thinkthen_request_description_map_entry_v1 *items=allocate(a,p->len,sizeof(*items));
    out->data=items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) {
        items[i].key=(thinkthen_complete_utf8_v1){
            p->keys[i].data,p->keys[i].len
        };
        items[i].value=convert_Authored_description_Authored_description(a,p->values[i]);
    }
    return out;
}
static void *convert_Authored_cut_Number_Value_Number(arena *a,const void *input) {
    const thinkthen_cobol_CutNumberValue *p=input;
    thinkthen_request_number_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_Authored_cut_Text_Value_Text(arena *a,const void *input) {
    const thinkthen_cobol_CutTextValue *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_Authored_labels_Array_Value_Authored_name_list(arena *a,const void *input) {
    const thinkthen_cobol_LabelsArrayValue *p=input;
    thinkthen_request_name_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_Authored_name_Authored_name(a,p->data[i]);
    return out;
}
static void *convert_Authored_labels_Object_Value_Authored_description_map(arena *a,const void *input) {
    const thinkthen_cobol_LabelsObjectValue *p=input;
    thinkthen_request_description_map_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && (!p->keys || !p->values)) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    thinkthen_request_description_map_entry_v1 *items=allocate(a,p->len,sizeof(*items));
    out->data=items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) {
        items[i].key=(thinkthen_complete_utf8_v1){
            p->keys[i].data,p->keys[i].len
        };
        items[i].value=convert_Authored_description_Authored_description(a,p->values[i]);
    }
    return out;
}
static void *convert_Authored_levels_Array_Value_Authored_name_list(arena *a,const void *input) {
    const thinkthen_cobol_LevelsArrayValue *p=input;
    thinkthen_request_name_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_Authored_name_Authored_name(a,p->data[i]);
    return out;
}
static void *convert_Authored_levels_Object_Value_Authored_criterion_map(arena *a,const void *input) {
    const thinkthen_cobol_LevelsObjectValue *p=input;
    thinkthen_request_criterion_map_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && (!p->keys || !p->values)) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    thinkthen_request_criterion_map_entry_v1 *items=allocate(a,p->len,sizeof(*items));
    out->data=items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) {
        items[i].key=(thinkthen_complete_utf8_v1){
            p->keys[i].data,p->keys[i].len
        };
        items[i].value=convert_Authored_criterion_Authored_criterion(a,p->values[i]);
    }
    return out;
}
static void *convert_RelateFields_RelateFields(arena *a,const void *input) {
    const thinkthen_cobol_RelateFields *p=input;
    thinkthen_request_relate_fields_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_kind) out->m_kind=convert_RelateFields_member_kind_Text(a,p->m_kind);
    if(p->m_name) out->m_name=convert_RelateFields_member_name_Text(a,p->m_name);
    return out;
}
static void *convert_RelateQuestion_member_relations_Authored_relation_list(arena *a,const void *input) {
    const thinkthen_cobol_RelateQuestionRelations *p=input;
    thinkthen_request_relation_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_Authored_relation_Authored_relation(a,p->data[i]);
    return out;
}
static void *convert_RecognizeQuestion_member_kinds_Authored_description_map(arena *a,const void *input) {
    const thinkthen_cobol_RecognizeQuestionKinds *p=input;
    thinkthen_request_description_map_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && (!p->keys || !p->values)) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    thinkthen_request_description_map_entry_v1 *items=allocate(a,p->len,sizeof(*items));
    out->data=items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) {
        items[i].key=(thinkthen_complete_utf8_v1){
            p->keys[i].data,p->keys[i].len
        };
        items[i].value=convert_Authored_description_Authored_description(a,p->values[i]);
    }
    return out;
}
static void *convert_RecognizeQuestion_member_relations_Authored_relation_list(arena *a,const void *input) {
    const thinkthen_cobol_RecognizeQuestionRelations *p=input;
    thinkthen_request_relation_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_Authored_relation_Authored_relation(a,p->data[i]);
    return out;
}
static void *convert_RecognizeQuestion_member_snippet_pieces_Unsigned(arena *a,const void *input) {
    const thinkthen_cobol_RecognizeQuestionSnippetPieces *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_AnnotateQuestion_AnnotateQuestion(arena *a,const void *input) {
    const thinkthen_cobol_AnnotateQuestion *p=input;
    thinkthen_request_annotate_question_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.decide=convert_AnnotateQuestion_decide_AnnotateQuestion_decide(a,p->value);
        break;
        case 2: out->data.choose=convert_AnnotateQuestion_choose_AnnotateQuestion_choose(a,p->value);
        break;
        case 3: out->data.tag=convert_AnnotateQuestion_tag_AnnotateQuestion_tag(a,p->value);
        break;
        case 4: out->data.score=convert_AnnotateQuestion_score_AnnotateQuestion_score(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_ImageMedia_image_jpeg_Value_Literal_image_jpeg(arena *a,const void *input) {
    const thinkthen_cobol_ImageMediaImageJpegValue *p=input;
    thinkthen_request_literal_image_jpeg_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_ImageMedia_image_png_Value_Literal_image_png(arena *a,const void *input) {
    const thinkthen_cobol_ImageMediaImagePngValue *p=input;
    thinkthen_request_literal_image_png_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RecognitionExampleEntity_member_end_Unsigned(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionExampleEntityEnd *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RecognitionExampleEntity_member_kind_Text(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionExampleEntityKind *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RecognitionExampleEntity_member_start_Unsigned(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionExampleEntityStart *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_Authored_inputDeclaration_object_member_properties_Authored_inputProperty_map(arena *a,
const void *input) {
    const thinkthen_cobol_InputDeclarationObjectProperties *p=input;
    thinkthen_request_input_property_map_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && (!p->keys || !p->values)) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    thinkthen_request_input_property_map_entry_v1 *items=allocate(a,p->len,sizeof(*items));
    out->data=items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) {
        items[i].key=(thinkthen_complete_utf8_v1){
            p->keys[i].data,p->keys[i].len
        };
        items[i].value=convert_Authored_inputProperty_Authored_inputProperty(a,p->values[i]);
    }
    return out;
}
static void *convert_Authored_inputDeclaration_object_member_required_Text_list(arena *a,const void *input) {
    const thinkthen_cobol_InputDeclarationObjectRequired *p=input;
    thinkthen_request_text_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_Authored_inputDeclaration_object_member_required_item_Text(a,p->data[i]);
    return out;
}
static void *convert_Authored_questionText_Object_Value_entry_JsonValue(arena *a,const void *input) {
    const thinkthen_cobol_QuestionTextObjectValueEntry *p=input;
    thinkthen_request_json_value_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_Authored_questionText_Array_Value_item_JsonValue(arena *a,const void *input) {
    const thinkthen_cobol_QuestionTextArrayValueItem *p=input;
    thinkthen_request_json_value_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_Authored_criterion_Object_Value_entry_JsonValue(arena *a,const void *input) {
    const thinkthen_cobol_CriterionObjectValueEntry *p=input;
    thinkthen_request_json_value_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_Authored_criterion_Array_Value_item_JsonValue(arena *a,const void *input) {
    const thinkthen_cobol_CriterionArrayValueItem *p=input;
    thinkthen_request_json_value_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_Authored_pointers_Array_Value_item_Text(arena *a,const void *input) {
    const thinkthen_cobol_PointersArrayValueItem *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_Authored_description_Authored_description(arena *a,const void *input) {
    const thinkthen_cobol_Description *p=input;
    thinkthen_request_description_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.text=convert_Authored_description_Text_Value_Text(a,p->value);
        break;
        case 2: out->data.json_value_map=convert_Authored_description_Object_Value_JsonValue_map(a,
        p->value);
        break;
        case 3: out->data.json_value_list=convert_Authored_description_Array_Value_JsonValue_list(a,
        p->value);
        break;
        case 4: out->data.null=convert_Authored_description_Null_Value_Null(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_RelateFields_member_kind_Text(arena *a,const void *input) {
    const thinkthen_cobol_RelateFieldsKind *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_RelateFields_member_name_Text(arena *a,const void *input) {
    const thinkthen_cobol_RelateFieldsName *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_Authored_relation_Authored_relation(arena *a,const void *input) {
    const thinkthen_cobol_Relation *p=input;
    thinkthen_request_relation_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_either) out->m_either=convert_Authored_relation_member_either_Boolean(a,p->m_either);
    if(p->m_name) out->m_name=convert_Authored_name_Authored_name(a,p->m_name);
    if(p->m_reads) out->m_reads=convert_Authored_name_Authored_name(a,p->m_reads);
    if(p->m_single) out->m_single=convert_Authored_relation_member_single_Boolean(a,p->m_single);
    if(p->m_source) out->m_source=convert_Authored_name_Authored_name(a,p->m_source);
    if(p->m_target) out->m_target=convert_Authored_name_Authored_name(a,p->m_target);
    return out;
}
static void *convert_AnnotateQuestion_decide_AnnotateQuestion_decide(arena *a,const void *input) {
    const thinkthen_cobol_AnnotateQuestionDecide *p=input;
    thinkthen_request_annotate_question_decide_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_context_schema) out->m_context_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_context_schema);
    if(p->m_decide) out->m_decide=convert_Authored_questionText_Authored_questionText(a,p->m_decide);
    if(p->m_false) out->m_false=convert_Authored_criterion_Authored_criterion(a,p->m_false);
    if(p->m_item_schema) out->m_item_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_item_schema);
    if(p->m_name) out->m_name=convert_AnnotateQuestion_decide_member_name_Text(a,p->m_name);
    if(p->m_on) out->m_on=convert_Authored_pointers_Authored_pointers(a,p->m_on);
    if(p->m_threshold) out->m_threshold=convert_Authored_threshold_Authored_threshold(a,p->m_threshold);
    if(p->m_true) out->m_true=convert_Authored_criterion_Authored_criterion(a,p->m_true);
    if(p->m_wording_version) out->m_wording_version=convert_AnnotateQuestion_decide_member_wording_version_Integer(a,
    p->m_wording_version);
    return out;
}
static void *convert_AnnotateQuestion_choose_AnnotateQuestion_choose(arena *a,const void *input) {
    const thinkthen_cobol_AnnotateQuestionChoose *p=input;
    thinkthen_request_annotate_question_choose_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_choose) out->m_choose=convert_Authored_questionText_Authored_questionText(a,p->m_choose);
    if(p->m_context_schema) out->m_context_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_context_schema);
    if(p->m_item_schema) out->m_item_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_item_schema);
    if(p->m_name) out->m_name=convert_AnnotateQuestion_choose_member_name_Text(a,p->m_name);
    if(p->m_on) out->m_on=convert_Authored_pointers_Authored_pointers(a,p->m_on);
    if(p->m_options) out->m_options=convert_Authored_options_Authored_options(a,p->m_options);
    if(p->m_threshold) out->m_threshold=convert_Authored_cut_Authored_cut(a,p->m_threshold);
    if(p->m_wording_version) out->m_wording_version=convert_AnnotateQuestion_choose_member_wording_version_Integer(a,
    p->m_wording_version);
    return out;
}
static void *convert_AnnotateQuestion_tag_AnnotateQuestion_tag(arena *a,const void *input) {
    const thinkthen_cobol_AnnotateQuestionTag *p=input;
    thinkthen_request_annotate_question_tag_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_context_schema) out->m_context_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_context_schema);
    if(p->m_item_schema) out->m_item_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_item_schema);
    if(p->m_labels) out->m_labels=convert_Authored_labels_Authored_labels(a,p->m_labels);
    if(p->m_name) out->m_name=convert_AnnotateQuestion_tag_member_name_Text(a,p->m_name);
    if(p->m_on) out->m_on=convert_Authored_pointers_Authored_pointers(a,p->m_on);
    if(p->m_tag) out->m_tag=convert_Authored_questionText_Authored_questionText(a,p->m_tag);
    if(p->m_threshold) out->m_threshold=convert_Authored_cut_Authored_cut(a,p->m_threshold);
    if(p->m_wording_version) out->m_wording_version=convert_AnnotateQuestion_tag_member_wording_version_Integer(a,
    p->m_wording_version);
    return out;
}
static void *convert_AnnotateQuestion_score_AnnotateQuestion_score(arena *a,const void *input) {
    const thinkthen_cobol_AnnotateQuestionScore *p=input;
    thinkthen_request_annotate_question_score_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_context_schema) out->m_context_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_context_schema);
    if(p->m_item_schema) out->m_item_schema=convert_Authored_inputDeclaration_Authored_inputDeclaration(a,
    p->m_item_schema);
    if(p->m_levels) out->m_levels=convert_Authored_levels_Authored_levels(a,p->m_levels);
    if(p->m_name) out->m_name=convert_AnnotateQuestion_score_member_name_Text(a,p->m_name);
    if(p->m_on) out->m_on=convert_Authored_pointers_Authored_pointers(a,p->m_on);
    if(p->m_score) out->m_score=convert_Authored_questionText_Authored_questionText(a,p->m_score);
    if(p->m_wording_version) out->m_wording_version=convert_AnnotateQuestion_score_member_wording_version_Integer(a,
    p->m_wording_version);
    return out;
}
static void *convert_Authored_inputProperty_Authored_inputProperty(arena *a,const void *input) {
    const thinkthen_cobol_InputProperty *p=input;
    thinkthen_request_input_property_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.string=convert_Authored_inputProperty_string_Authored_inputProperty_string(a,
        p->value);
        break;
        case 2: out->data.number=convert_Authored_inputProperty_number_Authored_inputProperty_number(a,
        p->value);
        break;
        case 3: out->data.boolean=convert_Authored_inputProperty_boolean_Authored_inputProperty_boolean(a,
        p->value);
        break;
        case 4: out->data.array=convert_Authored_inputProperty_array_Authored_inputProperty_array(a,
        p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_Authored_inputDeclaration_object_member_required_item_Text(arena *a,const void *input) {
    const thinkthen_cobol_InputDeclarationObjectRequiredItem *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_Authored_description_Text_Value_Text(arena *a,const void *input) {
    const thinkthen_cobol_DescriptionTextValue *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_Authored_description_Object_Value_JsonValue_map(arena *a,const void *input) {
    const thinkthen_cobol_DescriptionObjectValue *p=input;
    thinkthen_request_json_value_map_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && (!p->keys || !p->values)) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    thinkthen_request_json_value_map_entry_v1 *items=allocate(a,p->len,sizeof(*items));
    out->data=items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) {
        items[i].key=(thinkthen_complete_utf8_v1){
            p->keys[i].data,p->keys[i].len
        };
        items[i].value=convert_Authored_description_Object_Value_entry_JsonValue(a,p->values[i]);
    }
    return out;
}
static void *convert_Authored_description_Array_Value_JsonValue_list(arena *a,const void *input) {
    const thinkthen_cobol_DescriptionArrayValue *p=input;
    thinkthen_request_json_value_list_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->len && !p->data) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    void **items=allocate(a,p->len,sizeof(*items));
    out->data=(const void *)items;
    out->len=p->len;
    if(items) for(uint64_t i=0;
    i<p->len && !a->code;
    i++) items[i]=convert_Authored_description_Array_Value_item_JsonValue(a,p->data[i]);
    return out;
}
static void *convert_Authored_description_Null_Value_Null(arena *a,const void *input) {
    const thinkthen_cobol_DescriptionNullValue *p=input;
    thinkthen_request_null_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_Authored_relation_member_either_Boolean(arena *a,const void *input) {
    const thinkthen_cobol_RelationEither *p=input;
    thinkthen_request_boolean_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->value>1) a->code=THINKTHEN_COBOL_REPRESENTATION;
    out->value=(uint32_t)p->value;
    return out;
}
static void *convert_Authored_relation_member_single_Boolean(arena *a,const void *input) {
    const thinkthen_cobol_RelationSingle *p=input;
    thinkthen_request_boolean_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->value>1) a->code=THINKTHEN_COBOL_REPRESENTATION;
    out->value=(uint32_t)p->value;
    return out;
}
static void *convert_AnnotateQuestion_decide_member_name_Text(arena *a,const void *input) {
    const thinkthen_cobol_AnnotateQuestionDecideName *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_AnnotateQuestion_decide_member_wording_version_Integer(arena *a,const void *input) {
    const thinkthen_cobol_AnnotateQuestionDecideWordingVersion *p=input;
    thinkthen_request_integer_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_AnnotateQuestion_choose_member_name_Text(arena *a,const void *input) {
    const thinkthen_cobol_AnnotateQuestionChooseName *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_AnnotateQuestion_choose_member_wording_version_Integer(arena *a,const void *input) {
    const thinkthen_cobol_AnnotateQuestionChooseWordingVersion *p=input;
    thinkthen_request_integer_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_AnnotateQuestion_tag_member_name_Text(arena *a,const void *input) {
    const thinkthen_cobol_AnnotateQuestionTagName *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_AnnotateQuestion_tag_member_wording_version_Integer(arena *a,const void *input) {
    const thinkthen_cobol_AnnotateQuestionTagWordingVersion *p=input;
    thinkthen_request_integer_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_AnnotateQuestion_score_member_name_Text(arena *a,const void *input) {
    const thinkthen_cobol_AnnotateQuestionScoreName *p=input;
    thinkthen_request_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_AnnotateQuestion_score_member_wording_version_Integer(arena *a,const void *input) {
    const thinkthen_cobol_AnnotateQuestionScoreWordingVersion *p=input;
    thinkthen_request_integer_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_Authored_inputProperty_string_Authored_inputProperty_string(arena *a,const void *input) {
    const thinkthen_cobol_InputPropertyString *p=input;
    thinkthen_request_input_property_string_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    return out;
}
static void *convert_Authored_inputProperty_number_Authored_inputProperty_number(arena *a,const void *input) {
    const thinkthen_cobol_InputPropertyNumber *p=input;
    thinkthen_request_input_property_number_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    return out;
}
static void *convert_Authored_inputProperty_boolean_Authored_inputProperty_boolean(arena *a,
const void *input) {
    const thinkthen_cobol_InputPropertyBoolean *p=input;
    thinkthen_request_input_property_boolean_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    return out;
}
static void *convert_Authored_inputProperty_array_Authored_inputProperty_array(arena *a,const void *input) {
    const thinkthen_cobol_InputPropertyArray *p=input;
    thinkthen_request_input_property_array_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    if(p->m_items) out->m_items=convert_Authored_inputProperty_array_member_items_Authored_inputProperty_array_items(a,
    p->m_items);
    return out;
}
static void *convert_Authored_description_Object_Value_entry_JsonValue(arena *a,const void *input) {
    const thinkthen_cobol_DescriptionObjectValueEntry *p=input;
    thinkthen_request_json_value_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_Authored_description_Array_Value_item_JsonValue(arena *a,const void *input) {
    const thinkthen_cobol_DescriptionArrayValueItem *p=input;
    thinkthen_request_json_value_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_Authored_inputProperty_array_member_items_Authored_inputProperty_array_items(arena *a,
const void *input) {
    const thinkthen_cobol_InputPropertyArrayItems *p=input;
    thinkthen_request_input_property_array_items_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    return out;
}
static int start(const void *engine,const thinkthen_cobol_RequestCall *input,void **out,const char *function,
size_t length) {
    if(!input) return THINKTHEN_COBOL_REPRESENTATION;
    arena a={
        0
    };
    const thinkthen_request_arguments_v1 *arguments=convert_RequestCall_RequestArguments(&a,input);
    int code=a.code ? a.code : thinkthen_session_new_typed_with_surface(engine,(thinkthen_complete_utf8_v1){
        function,length
    },arguments,(thinkthen_complete_utf8_v1){
        "cobol",5
    },(thinkthen_session **)out);
    release(&a);
    return code;
}
int TT_SESSION_REQUEST(const void *engine,const thinkthen_cobol_Request *request,void **out) {
    if(!request || !request->m_call) return THINKTHEN_COBOL_REPRESENTATION;
    const thinkthen_cobol_RequestCall *call=request->m_call;
    const thinkthen_cobol_RequestFunction *function=call->m_function;
    if(!function) return THINKTHEN_COBOL_REPRESENTATION;
    switch(function->kind) {
        case 1: return start(engine,call,out,"decide",6);
        case 2: return start(engine,call,out,"choose",6);
        case 3: return start(engine,call,out,"tag",3);
        case 4: return start(engine,call,out,"score",5);
        case 5: return start(engine,call,out,"filter",6);
        case 6: return start(engine,call,out,"rank",4);
        case 7: return start(engine,call,out,"find",4);
        case 8: return start(engine,call,out,"annotate",8);
        case 9: return start(engine,call,out,"recognize",9);
        case 10: return start(engine,call,out,"relate",6);
        default: return THINKTHEN_COBOL_REPRESENTATION;
    }
}
int TT_SESSION_PUSH(const void *session,const thinkthen_cobol_RequestSessionDescriptor *input,
uint32_t *status) {
    arena a={
        0
    };
    const thinkthen_request_session_descriptor_v1 *descriptor=convert_RequestSessionDescriptor_RequestSessionDescriptor(&a,
    input);
    int code=a.code ? a.code : thinkthen_session_push_typed(session,descriptor,status);
    release(&a);
    return code;
}
int TT_SESSION_DECIDE(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    return start(engine,input,out,"decide",6);
}
int TT_SESSION_CHOOSE(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    return start(engine,input,out,"choose",6);
}
int TT_SESSION_TAG(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    return start(engine,input,out,"tag",3);
}
int TT_SESSION_SCORE(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    return start(engine,input,out,"score",5);
}
int TT_SESSION_FILTER(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    return start(engine,input,out,"filter",6);
}
int TT_SESSION_RANK(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    return start(engine,input,out,"rank",4);
}
int TT_SESSION_FIND(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    return start(engine,input,out,"find",4);
}
int TT_SESSION_ANNOTATE(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    return start(engine,input,out,"annotate",8);
}
int TT_SESSION_RECOGNIZE(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    return start(engine,input,out,"recognize",9);
}
int TT_SESSION_RELATE(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    return start(engine,input,out,"relate",6);
}
int TT_ENGINE_NEW(const thinkthen_cobol_Settings *input,void **out) {
    if(!input || !out) return THINKTHEN_COBOL_REPRESENTATION;
    arena a={
        0
    };
    const thinkthen_request_engine_settings_v1 *settings=convert_EngineSettings_EngineSettings(&a,
    input);
    thinkthen_engine *engine=a.code ? NULL : thinkthen_engine_new_typed(settings);
    int code=a.code ? a.code : engine ? 0 : thinkthen_error_code(NULL);
    release(&a);
    if(!code) *out=engine;
    return code;
}
int TT_PLAN(const void *engine,const char *function,uint64_t length,
const thinkthen_cobol_RequestCall *input,tt_value **out) {
    if(!out) return THINKTHEN_COBOL_REPRESENTATION;
    arena a={
        0
    };
    const thinkthen_request_arguments_v1 *arguments=convert_RequestCall_RequestArguments(&a,
    input);
    thinkthen_value *owner=NULL;
    int code=a.code ? a.code : thinkthen_request_plan_typed_value(engine,
    (thinkthen_complete_utf8_v1){
        function,length
    },arguments,
    (thinkthen_complete_utf8_v1){
        "cobol",5
    },&owner);
    release(&a);
    if(!code) {
        const thinkthen_complete_json_v1 *view=NULL;
        code=thinkthen_value_view(owner,&view);
        if(!code) code=own_value(view,out);
    }
    thinkthen_value_free(owner);
    return code;
}
int TT_SESSION_NEXT(thinkthen_session *session,uint32_t *status,thinkthen_session_result **out) {
    return thinkthen_session_next_packet(session,THINKTHEN_SESSION_WAIT_SETTLEMENT_V1,status,
    out);
}
static int usage_status(const thinkthen_engine *engine,thinkthen_complete_usage_persistence_v1 *state,
char *advice,uint64_t *written,int finish) {
    if(!state || !advice || !written) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_complete_usage_persistence_v1 observed;
    thinkthen_complete_utf8_v1 text;
    int code=finish ? thinkthen_engine_finish_usage_status_v1(engine,&observed,&text)
    : thinkthen_engine_usage_persistence_v1(engine,&observed,&text);
    if(code) return code;
    code=TT_SESSION_TEXT(&text,advice,written);
    if(code) return code;
    *state=observed;
    return 0;
}
int TT_ENGINE_USAGE_PERSISTENCE(const thinkthen_engine *engine,thinkthen_complete_usage_persistence_v1 *state,
char *advice,uint64_t *written) {
    return usage_status(engine,state,advice,written,0);
}
int TT_ENGINE_FINISH_USAGE_STATUS(const thinkthen_engine *engine,thinkthen_complete_usage_persistence_v1 *state,
char *advice,uint64_t *written) {
    return usage_status(engine,state,advice,written,1);
}
int TT_QUESTION_TEXT(thinkthen_cobol_RequestQuestion *selector,thinkthen_cobol_RequestQuestionText *arm,
thinkthen_cobol_RequestQuestionTextText *scalar,const char *data,uint64_t length) {
    if(!selector || !arm || !scalar) return THINKTHEN_COBOL_REPRESENTATION;
    *scalar=(thinkthen_cobol_RequestQuestionTextText){
        data,length
    };
    *arm=(thinkthen_cobol_RequestQuestionText){
        .m_text=scalar
    };
    *selector=(thinkthen_cobol_RequestQuestion){
        .kind=1,.value=arm
    };
    return 0;
}
int TT_QUESTION_FILE(thinkthen_cobol_RequestQuestion *selector,thinkthen_cobol_RequestQuestionFile *arm,
thinkthen_cobol_RequestQuestionFilePath *scalar,const char *data,uint64_t length) {
    if(!selector || !arm || !scalar) return THINKTHEN_COBOL_REPRESENTATION;
    *scalar=(thinkthen_cobol_RequestQuestionFilePath){
        data,length
    };
    *arm=(thinkthen_cobol_RequestQuestionFile){
        .m_path=scalar
    };
    *selector=(thinkthen_cobol_RequestQuestion){
        .kind=3,.value=arm
    };
    return 0;
}
int TT_QUESTION_NAME(thinkthen_cobol_RequestQuestion *selector,thinkthen_cobol_RequestQuestionName *arm,
thinkthen_cobol_RequestQuestionNameName *scalar,const char *data,uint64_t length) {
    if(!selector || !arm || !scalar) return THINKTHEN_COBOL_REPRESENTATION;
    *scalar=(thinkthen_cobol_RequestQuestionNameName){
        data,length
    };
    *arm=(thinkthen_cobol_RequestQuestionName){
        .m_name=scalar
    };
    *selector=(thinkthen_cobol_RequestQuestion){
        .kind=4,.value=arm
    };
    return 0;
}
int TT_QUESTION_REFERENCE(thinkthen_cobol_RequestQuestion *selector,thinkthen_cobol_RequestQuestionReference *arm,
thinkthen_cobol_RequestQuestionReferenceReference *scalar,const char *data,uint64_t length) {
    if(!selector || !arm || !scalar) return THINKTHEN_COBOL_REPRESENTATION;
    *scalar=(thinkthen_cobol_RequestQuestionReferenceReference){
        data,length
    };
    *arm=(thinkthen_cobol_RequestQuestionReference){
        .m_reference=scalar
    };
    *selector=(thinkthen_cobol_RequestQuestion){
        .kind=5,.value=arm
    };
    return 0;
}
int TT_QUESTION_RELATE(thinkthen_cobol_RequestQuestion *selector,thinkthen_cobol_RequestQuestionDefinition *arm,
thinkthen_cobol_RequestDefinition *definition,const thinkthen_cobol_DefinitionRelate *value) {
    if(!selector || !arm || !definition || !value) return THINKTHEN_COBOL_REPRESENTATION;
    *definition=(thinkthen_cobol_RequestDefinition){
        .kind=5,.value=value
    };
    *arm=(thinkthen_cobol_RequestQuestionDefinition){
        .m_value=definition
    };
    *selector=(thinkthen_cobol_RequestQuestion){
        .kind=2,.value=arm
    };
    return 0;
}
int TT_QUESTION_RECOGNIZE(thinkthen_cobol_RequestQuestion *selector,thinkthen_cobol_RequestQuestionDefinition *arm,
thinkthen_cobol_RequestDefinition *definition,const thinkthen_cobol_DefinitionRecognizeDefinition *value) {
    if(!selector || !arm || !definition || !value) return THINKTHEN_COBOL_REPRESENTATION;
    *definition=(thinkthen_cobol_RequestDefinition){
        .kind=7,.value=value
    };
    *arm=(thinkthen_cobol_RequestQuestionDefinition){
        .m_value=definition
    };
    *selector=(thinkthen_cobol_RequestQuestion){
        .kind=2,.value=arm
    };
    return 0;
}
int TT_TEXT_ITEM(thinkthen_cobol_RequestItem *item,thinkthen_cobol_RequestOriginal *original,
thinkthen_cobol_RequestOriginalText *arm,thinkthen_cobol_RequestOriginalTextText *scalar,const char *data,
uint64_t length) {
    if(!item || !original || !arm || !scalar) return THINKTHEN_COBOL_REPRESENTATION;
    *scalar=(thinkthen_cobol_RequestOriginalTextText){
        data,length
    };
    *arm=(thinkthen_cobol_RequestOriginalText){
        .m_text=scalar
    };
    *original=(thinkthen_cobol_RequestOriginal){
        .kind=1,.value=arm
    };
    *item=(thinkthen_cobol_RequestItem){
        .m_original=original
    };
    return 0;
}
int TT_JSON_ITEM(thinkthen_cobol_RequestItem *item,thinkthen_cobol_RequestOriginal *original,
thinkthen_cobol_RequestOriginalJson *arm,thinkthen_cobol_RequestOriginalJsonValue *scalar,const tt_value *value) {
    if(!item || !original || !arm || !scalar || !value) return THINKTHEN_COBOL_REPRESENTATION;
    *scalar=(thinkthen_cobol_RequestOriginalJsonValue){
        .value=value->value
    };
    *arm=(thinkthen_cobol_RequestOriginalJson){
        .m_value=scalar
    };
    *original=(thinkthen_cobol_RequestOriginal){
        .kind=2,.value=arm
    };
    *item=(thinkthen_cobol_RequestItem){
        .m_original=original
    };
    return 0;
}
int TT_FILE_INPUT(thinkthen_cobol_RequestInput *selector,thinkthen_cobol_RequestInputSource *arm,
thinkthen_cobol_RequestSource *source,thinkthen_cobol_RequestSourcePaths *paths,thinkthen_cobol_RequestSourcePathsItem *path,
const void **slots,const char *data,uint64_t length) {
    if(!selector || !arm || !source || !paths || !path || !slots) return THINKTHEN_COBOL_REPRESENTATION;
    *path=(thinkthen_cobol_RequestSourcePathsItem){
        data,length
    };
    slots[0]=path;
    *paths=(thinkthen_cobol_RequestSourcePaths){
        .data=slots,.len=1
    };
    *source=(thinkthen_cobol_RequestSource){
        .m_paths=paths
    };
    *arm=(thinkthen_cobol_RequestInputSource){
        .m_source=source
    };
    *selector=(thinkthen_cobol_RequestInput){
        .kind=6,.value=arm
    };
    return 0;
}
struct tt_answer_owner {
    arena storage;
    void *rows;
    size_t count;
    size_t capacity;
    void *edges;
    size_t edge_count;
    size_t edge_capacity;
    void *receipts;
    size_t receipt_count;
    thinkthen_cobol_AnswerNumber *probabilities;
    size_t probability_count;
    thinkthen_cobol_AnswerProbabilityList *probability_lists;
    size_t probability_list_count;
    unsigned char *resolved;
    size_t resolved_count;
};
void TT_ANSWER_FREE(tt_answer_owner *owner) {
    if(owner) {
        release(&owner->storage);
        free(owner->rows);
        free(owner->edges);
        free(owner->receipts);
        free(owner->probabilities);
        free(owner->probability_lists);
        free(owner->resolved);
        free(owner);
    }
}
static int answer_text(thinkthen_complete_utf8_v1 input,thinkthen_cobol_AnswerText *out) {
    return TT_SESSION_TEXT(&input,out->text,&out->length);
}
static int original_limit(const thinkthen_complete_json_v1 *input) {
    if(!input) return THINKTHEN_COBOL_REPRESENTATION;
    if(input->kind==THINKTHEN_COMPLETE_JSON_STRING_V1 && input->data.string.len>THINKTHEN_COBOL_TEXT_CAPACITY) return THINKTHEN_COBOL_OVERFLOW;
    if(input->kind==THINKTHEN_COMPLETE_JSON_NUMBER_V1 && input->data.number.len>THINKTHEN_COBOL_TEXT_CAPACITY) return THINKTHEN_COBOL_OVERFLOW;
    if(input->kind==THINKTHEN_COMPLETE_JSON_ARRAY_V1) for(size_t i=0;
    i<input->data.array.len;
    i++) {
        int code=original_limit(input->data.array.data[i]);
        if(code) return code;
    }
    if(input->kind==THINKTHEN_COMPLETE_JSON_OBJECT_V1) for(size_t i=0;
    i<input->data.object.len;
    i++) {
        if(input->data.object.data[i].name.len>THINKTHEN_COBOL_TEXT_CAPACITY) return THINKTHEN_COBOL_OVERFLOW;
        int code=original_limit(input->data.object.data[i].value);
        if(code) return code;
    }
    return 0;
}
static const tt_value *answer_original(arena *storage,const thinkthen_complete_json_v1 *input) {
    int code=original_limit(input);
    if(code) {
        storage->code=code;
        return NULL;
    }
    tt_value *out=allocate(storage,1,sizeof(*out));
    if(out) out->value=copy_value(storage,input);
    return out;
}
static int answer_tags(arena *storage,thinkthen_complete_string_list_v1 input,const thinkthen_cobol_AnswerText **out) {
    thinkthen_cobol_AnswerText *rows=allocate(storage,input.len,sizeof(*rows));
    if(!rows) return storage->code;
    for(size_t i=0;
    i<input.len;
    i++) {
        int code=answer_text(input.data[i],rows+i);
        if(code) return code;
    }
    *out=rows;
    return 0;
}
static int copy_thinkthen_cobol_AnswerError(arena *,const thinkthen_complete_error_v1 *,thinkthen_cobol_AnswerError *);
static int copy_thinkthen_cobol_AnswerExtensions(arena *,const thinkthen_complete_extensions_v1 *,
thinkthen_cobol_AnswerExtensions *);
static int copy_thinkthen_cobol_AnswerExtension(arena *,const thinkthen_complete_extension_v1 *,
thinkthen_cobol_AnswerExtension *);
static int copy_thinkthen_cobol_AnswerStopped(arena *,const thinkthen_complete_stopped_v1 *,
thinkthen_cobol_AnswerStopped *);
static int copy_thinkthen_cobol_AnswerU64Presence(arena *,const thinkthen_complete_u64_presence_v1 *,
thinkthen_cobol_AnswerU64Presence *);
static int copy_thinkthen_cobol_AnswerStopCause(arena *,const thinkthen_complete_stop_cause_v1 *,
thinkthen_cobol_AnswerStopCause *);
static int copy_thinkthen_cobol_AnswerSendDenialPresence(arena *,const thinkthen_complete_send_denial_presence_v1 *,
thinkthen_cobol_AnswerSendDenialPresence *);
static int copy_thinkthen_cobol_AnswerSendDenial(arena *,const thinkthen_complete_send_denial_v1 *,
thinkthen_cobol_AnswerSendDenial *);
static int copy_thinkthen_cobol_AnswerSendDenialData(arena *,const thinkthen_complete_send_denial_data_v1 *,
thinkthen_cobol_AnswerSendDenialData *,uint32_t);
static int copy_thinkthen_cobol_AnswerSendDenialBeforeRetry(arena *,const thinkthen_complete_send_denial_before_retry_v1 *,
thinkthen_cobol_AnswerSendDenialBeforeRetry *);
static int copy_thinkthen_cobol_AnswerSendDenialBeforeAdditionalSend(arena *,const thinkthen_complete_send_denial_before_additional_send_v1 *,
thinkthen_cobol_AnswerSendDenialBeforeAdditionalSend *);
static int copy_thinkthen_cobol_AnswerSendDenialBeforeFirstSend(arena *,const thinkthen_complete_send_denial_before_first_send_v1 *,
thinkthen_cobol_AnswerSendDenialBeforeFirstSend *);
static int copy_thinkthen_cobol_AnswerFailureKind(arena *,const thinkthen_complete_failure_kind_v1 *,
thinkthen_cobol_AnswerFailureKind *);
static int copy_thinkthen_cobol_AnswerInputDenialPresence(arena *,const thinkthen_complete_input_denial_presence_v1 *,
thinkthen_cobol_AnswerInputDenialPresence *);
static int copy_thinkthen_cobol_AnswerInputDenial(arena *,const thinkthen_complete_input_denial_v1 *,
thinkthen_cobol_AnswerInputDenial *);
static int copy_thinkthen_cobol_AnswerInputDenialData(arena *,const thinkthen_complete_input_denial_data_v1 *,
thinkthen_cobol_AnswerInputDenialData *,uint32_t);
static int copy_thinkthen_cobol_AnswerInputDenialRetry(arena *,const thinkthen_complete_input_denial_retry_v1 *,
thinkthen_cobol_AnswerInputDenialRetry *);
static int copy_thinkthen_cobol_AnswerInputDenialAdditionalRequest(arena *,const thinkthen_complete_input_denial_additional_request_v1 *,
thinkthen_cobol_AnswerInputDenialAdditionalRequest *);
static int copy_thinkthen_cobol_AnswerInputDenialInitialRequest(arena *,const thinkthen_complete_input_denial_initial_request_v1 *,
thinkthen_cobol_AnswerInputDenialInitialRequest *);
static int copy_thinkthen_cobol_AnswerImmediateFacts(arena *,const thinkthen_complete_immediate_facts_v1 *,
thinkthen_cobol_AnswerImmediateFacts *);
static int copy_thinkthen_cobol_AnswerPersistenceObservationPresence(arena *,const thinkthen_complete_persistence_observation_presence_v1 *,
thinkthen_cobol_AnswerPersistenceObservationPresence *);
static int copy_thinkthen_cobol_AnswerPersistenceObservation(arena *,const thinkthen_complete_persistence_observation_v1 *,
thinkthen_cobol_AnswerPersistenceObservation *);
static int copy_thinkthen_cobol_AnswerUsagePersistence(arena *,const thinkthen_complete_usage_persistence_v1 *,
thinkthen_cobol_AnswerUsagePersistence *);
static int copy_thinkthen_cobol_AnswerUtf8Presence(arena *,const thinkthen_complete_utf8_presence_v1 *,
thinkthen_cobol_AnswerUtf8Presence *);
static int copy_thinkthen_cobol_AnswerU32Presence(arena *,const thinkthen_complete_u32_presence_v1 *,
thinkthen_cobol_AnswerU32Presence *);
static int copy_thinkthen_cobol_AnswerCallIdPresence(arena *,const thinkthen_complete_call_id_presence_v1 *,
thinkthen_cobol_AnswerCallIdPresence *);
static int copy_thinkthen_cobol_AnswerCallId(arena *,const thinkthen_complete_call_id_v1 *,
thinkthen_cobol_AnswerCallId *);
static int copy_thinkthen_cobol_AnswerAttemptListPresence(arena *,const thinkthen_complete_attempt_list_presence_v1 *,
thinkthen_cobol_AnswerAttemptListPresence *);
static int copy_thinkthen_cobol_AnswerAttemptList(arena *,const thinkthen_complete_attempt_list_v1 *,
thinkthen_cobol_AnswerAttemptList *);
static int copy_thinkthen_cobol_AnswerAttempt(arena *,const thinkthen_complete_attempt_v1 *,
thinkthen_cobol_AnswerAttempt *);
static int copy_thinkthen_cobol_AnswerSdkRequestId(arena *,const thinkthen_complete_sdk_request_id_v1 *,
thinkthen_cobol_AnswerSdkRequestId *);
static int copy_thinkthen_cobol_AnswerAttemptOutcome(arena *,const thinkthen_complete_attempt_outcome_v1 *,
thinkthen_cobol_AnswerAttemptOutcome *);
static int copy_thinkthen_cobol_AnswerFacts(arena *,const thinkthen_complete_facts_v1 *,thinkthen_cobol_AnswerFacts *);
static int copy_thinkthen_cobol_AnswerExtensions(arena *a,const thinkthen_complete_extensions_v1 *input,
thinkthen_cobol_AnswerExtensions *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_AnswerExtension *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_AnswerExtension(a,&input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerExtension(arena *a,const thinkthen_complete_extension_v1 *input,
thinkthen_cobol_AnswerExtension *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->name,&out->name);
    if(input->value) out->value=answer_original(a,input->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerU64Presence(arena *a,const thinkthen_complete_u64_presence_v1 *input,
thinkthen_cobol_AnswerU64Presence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    out->value=input->value;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerStopCause(arena *a,const thinkthen_complete_stop_cause_v1 *input,
thinkthen_cobol_AnswerStopCause *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerSendDenialPresence(arena *a,const thinkthen_complete_send_denial_presence_v1 *input,
thinkthen_cobol_AnswerSendDenialPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_AnswerSendDenial *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_AnswerSendDenial(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerSendDenialData(arena *a,const thinkthen_complete_send_denial_data_v1 *input,
thinkthen_cobol_AnswerSendDenialData *out,uint32_t kind) {
    int code=0;
    (void)a;
    if(kind==THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_FIRST_SEND_V1) {
        if(input->before_first_send) {
            thinkthen_cobol_AnswerSendDenialBeforeFirstSend *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_AnswerSendDenialBeforeFirstSend(a,input->before_first_send,
            value);
            out->before_first_send=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_ADDITIONAL_SEND_V1) {
        if(input->before_additional_send) {
            thinkthen_cobol_AnswerSendDenialBeforeAdditionalSend *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_AnswerSendDenialBeforeAdditionalSend(a,input->before_additional_send,
            value);
            out->before_additional_send=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_SEND_DENIAL_BEFORE_RETRY_V1) {
        if(input->before_retry) {
            thinkthen_cobol_AnswerSendDenialBeforeRetry *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_AnswerSendDenialBeforeRetry(a,input->before_retry,value);
            out->before_retry=value;
        }
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerSendDenialBeforeRetry(arena *a,const thinkthen_complete_send_denial_before_retry_v1 *input,
thinkthen_cobol_AnswerSendDenialBeforeRetry *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->kind,&out->kind);
    out->last_status=input->last_status;
    if(!code) code=copy_thinkthen_cobol_AnswerExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerSendDenialBeforeAdditionalSend(arena *a,const thinkthen_complete_send_denial_before_additional_send_v1 *input,
thinkthen_cobol_AnswerSendDenialBeforeAdditionalSend *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->kind,&out->kind);
    if(!code) code=copy_thinkthen_cobol_AnswerExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerSendDenialBeforeFirstSend(arena *a,const thinkthen_complete_send_denial_before_first_send_v1 *input,
thinkthen_cobol_AnswerSendDenialBeforeFirstSend *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->kind,&out->kind);
    if(!code) code=copy_thinkthen_cobol_AnswerExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerFailureKind(arena *a,const thinkthen_complete_failure_kind_v1 *input,
thinkthen_cobol_AnswerFailureKind *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerInputDenialPresence(arena *a,const thinkthen_complete_input_denial_presence_v1 *input,
thinkthen_cobol_AnswerInputDenialPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_AnswerInputDenial *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_AnswerInputDenial(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerInputDenialData(arena *a,const thinkthen_complete_input_denial_data_v1 *input,
thinkthen_cobol_AnswerInputDenialData *out,uint32_t kind) {
    int code=0;
    (void)a;
    if(kind==THINKTHEN_COMPLETE_INPUT_DENIAL_INITIAL_REQUEST_V1) {
        if(input->initial_request) {
            thinkthen_cobol_AnswerInputDenialInitialRequest *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_AnswerInputDenialInitialRequest(a,input->initial_request,
            value);
            out->initial_request=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_INPUT_DENIAL_ADDITIONAL_REQUEST_V1) {
        if(input->additional_request) {
            thinkthen_cobol_AnswerInputDenialAdditionalRequest *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_AnswerInputDenialAdditionalRequest(a,input->additional_request,
            value);
            out->additional_request=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_INPUT_DENIAL_RETRY_V1) {
        if(input->retry) {
            thinkthen_cobol_AnswerInputDenialRetry *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_AnswerInputDenialRetry(a,input->retry,value);
            out->retry=value;
        }
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerInputDenialRetry(arena *a,const thinkthen_complete_input_denial_retry_v1 *input,
thinkthen_cobol_AnswerInputDenialRetry *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->kind,&out->kind);
    out->last_status=input->last_status;
    out->limit=input->limit;
    if(!code) code=copy_thinkthen_cobol_AnswerExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerInputDenialAdditionalRequest(arena *a,const thinkthen_complete_input_denial_additional_request_v1 *input,
thinkthen_cobol_AnswerInputDenialAdditionalRequest *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->kind,&out->kind);
    out->limit=input->limit;
    if(!code) code=copy_thinkthen_cobol_AnswerExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerInputDenialInitialRequest(arena *a,const thinkthen_complete_input_denial_initial_request_v1 *input,
thinkthen_cobol_AnswerInputDenialInitialRequest *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->kind,&out->kind);
    out->limit=input->limit;
    if(!code) code=copy_thinkthen_cobol_AnswerExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerPersistenceObservationPresence(arena *a,const thinkthen_complete_persistence_observation_presence_v1 *input,
thinkthen_cobol_AnswerPersistenceObservationPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_AnswerPersistenceObservation *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_AnswerPersistenceObservation(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerUsagePersistence(arena *a,const thinkthen_complete_usage_persistence_v1 *input,
thinkthen_cobol_AnswerUsagePersistence *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerUtf8Presence(arena *a,const thinkthen_complete_utf8_presence_v1 *input,
thinkthen_cobol_AnswerUtf8Presence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(!code) code=answer_text(input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerU32Presence(arena *a,const thinkthen_complete_u32_presence_v1 *input,
thinkthen_cobol_AnswerU32Presence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    out->value=input->value;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerCallIdPresence(arena *a,const thinkthen_complete_call_id_presence_v1 *input,
thinkthen_cobol_AnswerCallIdPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_AnswerCallId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_AnswerCallId(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerCallId(arena *a,const thinkthen_complete_call_id_v1 *input,
thinkthen_cobol_AnswerCallId *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerAttemptList(arena *a,const thinkthen_complete_attempt_list_v1 *input,
thinkthen_cobol_AnswerAttemptList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_AnswerAttempt *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_AnswerAttempt(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerAttempt(arena *a,const thinkthen_complete_attempt_v1 *input,
thinkthen_cobol_AnswerAttempt *out) {
    int code=0;
    (void)a;
    out->ordinal=input->ordinal;
    if(input->outcome) {
        thinkthen_cobol_AnswerAttemptOutcome *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_AnswerAttemptOutcome(a,input->outcome,value);
        out->outcome=value;
    }
    if(!code) code=copy_thinkthen_cobol_AnswerUtf8Presence(a,&input->request_id,&out->request_id);
    if(!code) code=answer_text(input->request_sha256,&out->request_sha256);
    if(input->sdk_request_id) {
        thinkthen_cobol_AnswerSdkRequestId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_AnswerSdkRequestId(a,input->sdk_request_id,value);
        out->sdk_request_id=value;
    }
    if(!code) code=copy_thinkthen_cobol_AnswerU64Presence(a,&input->server_ms,&out->server_ms);
    if(!code) code=copy_thinkthen_cobol_AnswerU64Presence(a,&input->status,&out->status);
    out->wall_ms=input->wall_ms;
    if(!code) code=copy_thinkthen_cobol_AnswerExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerSdkRequestId(arena *a,const thinkthen_complete_sdk_request_id_v1 *input,
thinkthen_cobol_AnswerSdkRequestId *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerAttemptOutcome(arena *a,const thinkthen_complete_attempt_outcome_v1 *input,
thinkthen_cobol_AnswerAttemptOutcome *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerError(arena *a,const thinkthen_complete_error_v1 *input,
thinkthen_cobol_AnswerError *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_AnswerInputDenialPresence(a,&input->estimated_input_denial,
    &out->estimated_input_denial);
    if(input->kind) {
        thinkthen_cobol_AnswerFailureKind *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_AnswerFailureKind(a,input->kind,value);
        out->kind=value;
    }
    if(!code) code=answer_text(input->message,&out->message);
    out->retryable=input->retryable;
    if(!code) code=copy_thinkthen_cobol_AnswerSendDenialPresence(a,&input->send_budget_denial,&out->send_budget_denial);
    if(input->stopped) {
        thinkthen_cobol_AnswerStopped *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_AnswerStopped(a,input->stopped,value);
        out->stopped=value;
    }
    if(!code) code=copy_thinkthen_cobol_AnswerExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerStopped(arena *a,const thinkthen_complete_stopped_v1 *input,
thinkthen_cobol_AnswerStopped *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_AnswerU64Presence(a,&input->at,&out->at);
    if(input->cause) {
        thinkthen_cobol_AnswerStopCause *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_AnswerStopCause(a,input->cause,value);
        out->cause=value;
    }
    out->retryable=input->retryable;
    if(!code) code=copy_thinkthen_cobol_AnswerU64Presence(a,&input->status,&out->status);
    if(!code) code=copy_thinkthen_cobol_AnswerExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerSendDenial(arena *a,const thinkthen_complete_send_denial_v1 *input,
thinkthen_cobol_AnswerSendDenial *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    if(!code) code=copy_thinkthen_cobol_AnswerSendDenialData(a,&input->data,&out->data,input->kind);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerInputDenial(arena *a,const thinkthen_complete_input_denial_v1 *input,
thinkthen_cobol_AnswerInputDenial *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    if(!code) code=copy_thinkthen_cobol_AnswerInputDenialData(a,&input->data,&out->data,input->kind);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerPersistenceObservation(arena *a,const thinkthen_complete_persistence_observation_v1 *input,
thinkthen_cobol_AnswerPersistenceObservation *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_AnswerUtf8Presence(a,&input->advice,&out->advice);
    if(!code) code=answer_text(input->observed_at,&out->observed_at);
    if(input->state) {
        thinkthen_cobol_AnswerUsagePersistence *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_AnswerUsagePersistence(a,input->state,value);
        out->state=value;
    }
    if(!code) code=copy_thinkthen_cobol_AnswerExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerAttemptListPresence(arena *a,const thinkthen_complete_attempt_list_presence_v1 *input,
thinkthen_cobol_AnswerAttemptListPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(!code) code=copy_thinkthen_cobol_AnswerAttemptList(a,&input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerFacts(arena *a,const thinkthen_complete_facts_v1 *input,
thinkthen_cobol_AnswerFacts *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_AnswerAttemptListPresence(a,&input->attempts,&out->attempts);
    out->cache_answers=input->cache_answers;
    if(input->call_id) {
        thinkthen_cobol_AnswerCallId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_AnswerCallId(a,input->call_id,value);
        out->call_id=value;
    }
    if(!code) code=copy_thinkthen_cobol_AnswerUtf8Presence(a,&input->estimated_cost_usd,&out->estimated_cost_usd);
    if(!code) code=copy_thinkthen_cobol_AnswerU32Presence(a,&input->held_model_mismatch,&out->held_model_mismatch);
    if(!code) code=copy_thinkthen_cobol_AnswerU64Presence(a,&input->input_tokens,&out->input_tokens);
    out->largest_request_bytes=input->largest_request_bytes;
    if(!code) code=copy_thinkthen_cobol_AnswerU64Presence(a,&input->largest_request_estimated_input_tokens,
    &out->largest_request_estimated_input_tokens);
    if(!code) code=copy_thinkthen_cobol_AnswerUtf8Presence(a,&input->model,&out->model);
    if(!code) code=copy_thinkthen_cobol_AnswerU64Presence(a,&input->output_tokens,&out->output_tokens);
    out->records=input->records;
    out->requests_sent=input->requests_sent;
    out->seconds=input->seconds;
    if(!code) code=answer_text(input->token_estimate_method,&out->token_estimate_method);
    if(!code) code=copy_thinkthen_cobol_AnswerPersistenceObservationPresence(a,&input->usage_persistence,
    &out->usage_persistence);
    if(!code) code=copy_thinkthen_cobol_AnswerExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_AnswerImmediateFacts(arena *a,const thinkthen_complete_immediate_facts_v1 *input,
thinkthen_cobol_AnswerImmediateFacts *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_AnswerAttemptListPresence(a,&input->attempts,&out->attempts);
    out->cache_answers=input->cache_answers;
    if(!code) code=copy_thinkthen_cobol_AnswerCallIdPresence(a,&input->call_id,&out->call_id);
    if(!code) code=copy_thinkthen_cobol_AnswerUtf8Presence(a,&input->estimated_cost_usd,&out->estimated_cost_usd);
    if(!code) code=copy_thinkthen_cobol_AnswerU32Presence(a,&input->held_model_mismatch,&out->held_model_mismatch);
    if(!code) code=copy_thinkthen_cobol_AnswerU64Presence(a,&input->input_tokens,&out->input_tokens);
    out->largest_request_bytes=input->largest_request_bytes;
    if(!code) code=copy_thinkthen_cobol_AnswerU64Presence(a,&input->largest_request_estimated_input_tokens,
    &out->largest_request_estimated_input_tokens);
    if(!code) code=copy_thinkthen_cobol_AnswerUtf8Presence(a,&input->model,&out->model);
    if(!code) code=copy_thinkthen_cobol_AnswerU64Presence(a,&input->output_tokens,&out->output_tokens);
    out->records=input->records;
    out->requests_sent=input->requests_sent;
    out->seconds=input->seconds;
    if(!code) code=answer_text(input->token_estimate_method,&out->token_estimate_method);
    if(!code) code=copy_thinkthen_cobol_AnswerPersistenceObservationPresence(a,&input->usage_persistence,
    &out->usage_persistence);
    if(!code) code=copy_thinkthen_cobol_AnswerExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelation(arena *,const thinkthen_complete_relation_v1 *,
thinkthen_cobol_ReceiptRelation *);
static int copy_thinkthen_cobol_ReceiptExtensions(arena *,const thinkthen_complete_extensions_v1 *,
thinkthen_cobol_ReceiptExtensions *);
static int copy_thinkthen_cobol_ReceiptExtension(arena *,const thinkthen_complete_extension_v1 *,
thinkthen_cobol_ReceiptExtension *);
static int copy_thinkthen_cobol_ReceiptRelatedEntityEdgeList(arena *,const thinkthen_complete_related_entity_edge_list_v1 *,
thinkthen_cobol_ReceiptRelatedEntityEdgeList *);
static int copy_thinkthen_cobol_ReceiptRelatedEntityEdge(arena *,const thinkthen_complete_related_entity_edge_v1 *,
thinkthen_cobol_ReceiptRelatedEntityEdge *);
static int copy_thinkthen_cobol_ReceiptRelationEndpoint(arena *,const thinkthen_complete_relation_endpoint_v1 *,
thinkthen_cobol_ReceiptRelationEndpoint *);
static int copy_thinkthen_cobol_ReceiptRelationEndpointData(arena *,const thinkthen_complete_relation_endpoint_data_v1 *,
thinkthen_cobol_ReceiptRelationEndpointData *,uint32_t);
static int copy_thinkthen_cobol_ReceiptRelationEndpointSource(arena *,const thinkthen_complete_relation_endpoint_source_v1 *,
thinkthen_cobol_ReceiptRelationEndpointSource *);
static int copy_thinkthen_cobol_ReceiptJsonPresence(arena *,const thinkthen_complete_json_presence_v1 *,
thinkthen_cobol_ReceiptJsonPresence *);
static int copy_thinkthen_cobol_ReceiptU64Presence(arena *,const thinkthen_complete_u64_presence_v1 *,
thinkthen_cobol_ReceiptU64Presence *);
static int copy_thinkthen_cobol_ReceiptUtf8Presence(arena *,const thinkthen_complete_utf8_presence_v1 *,
thinkthen_cobol_ReceiptUtf8Presence *);
static int copy_thinkthen_cobol_ReceiptRelationEndpointRelatedEntity(arena *,const thinkthen_complete_relation_endpoint_related_entity_v1 *,
thinkthen_cobol_ReceiptRelationEndpointRelatedEntity *);
static int copy_thinkthen_cobol_ReceiptU32Presence(arena *,const thinkthen_complete_u32_presence_v1 *,
thinkthen_cobol_ReceiptU32Presence *);
static int copy_thinkthen_cobol_ReceiptVersion(arena *,const thinkthen_complete_version_v1 *,
thinkthen_cobol_ReceiptVersion *);
static int copy_thinkthen_cobol_ReceiptRelateQuestionDocument(arena *,const thinkthen_complete_relate_question_document_v1 *,
thinkthen_cobol_ReceiptRelateQuestionDocument *);
static int copy_thinkthen_cobol_ReceiptWordingVersionPresence(arena *,const thinkthen_complete_wording_version_presence_v1 *,
thinkthen_cobol_ReceiptWordingVersionPresence *);
static int copy_thinkthen_cobol_ReceiptWordingVersion(arena *,const thinkthen_complete_wording_version_v1 *,
thinkthen_cobol_ReceiptWordingVersion *);
static int copy_thinkthen_cobol_ReceiptThreshold(arena *,const thinkthen_complete_threshold_v1 *,
thinkthen_cobol_ReceiptThreshold *);
static int copy_thinkthen_cobol_ReceiptThresholdData(arena *,const thinkthen_complete_threshold_data_v1 *,
thinkthen_cobol_ReceiptThresholdData *,uint32_t);
static int copy_thinkthen_cobol_ReceiptRelationRuleList(arena *,const thinkthen_complete_relation_rule_list_v1 *,
thinkthen_cobol_ReceiptRelationRuleList *);
static int copy_thinkthen_cobol_ReceiptRelationRule(arena *,const thinkthen_complete_relation_rule_v1 *,
thinkthen_cobol_ReceiptRelationRule *);
static int copy_thinkthen_cobol_ReceiptStringListPresence(arena *,const thinkthen_complete_string_list_presence_v1 *,
thinkthen_cobol_ReceiptStringListPresence *);
static int copy_thinkthen_cobol_ReceiptStringList(arena *,const thinkthen_complete_string_list_v1 *,
thinkthen_cobol_ReceiptStringList *);
static int copy_thinkthen_cobol_ReceiptQuestionNamePresence(arena *,const thinkthen_complete_question_name_presence_v1 *,
thinkthen_cobol_ReceiptQuestionNamePresence *);
static int copy_thinkthen_cobol_ReceiptQuestionName(arena *,const thinkthen_complete_question_name_v1 *,
thinkthen_cobol_ReceiptQuestionName *);
static int copy_thinkthen_cobol_ReceiptLabelListPresence(arena *,const thinkthen_complete_label_list_presence_v1 *,
thinkthen_cobol_ReceiptLabelListPresence *);
static int copy_thinkthen_cobol_ReceiptLabelList(arena *,const thinkthen_complete_label_list_v1 *,
thinkthen_cobol_ReceiptLabelList *);
static int copy_thinkthen_cobol_ReceiptLabel(arena *,const thinkthen_complete_label_v1 *,thinkthen_cobol_ReceiptLabel *);
static int copy_thinkthen_cobol_ReceiptInputDeclarationPresence(arena *,const thinkthen_complete_input_declaration_presence_v1 *,
thinkthen_cobol_ReceiptInputDeclarationPresence *);
static int copy_thinkthen_cobol_ReceiptInputDeclaration(arena *,const thinkthen_complete_input_declaration_v1 *,
thinkthen_cobol_ReceiptInputDeclaration *);
static int copy_thinkthen_cobol_ReceiptInputDeclarationData(arena *,const thinkthen_complete_input_declaration_data_v1 *,
thinkthen_cobol_ReceiptInputDeclarationData *,uint32_t);
static int copy_thinkthen_cobol_ReceiptInputDeclarationObjectRoot(arena *,const thinkthen_complete_input_declaration_object_root_v1 *,
thinkthen_cobol_ReceiptInputDeclarationObjectRoot *);
static int copy_thinkthen_cobol_ReceiptObjectType(arena *,const thinkthen_complete_object_type_v1 *,
thinkthen_cobol_ReceiptObjectType *);
static int copy_thinkthen_cobol_ReceiptInputPropertyTypeMap(arena *,const thinkthen_complete_input_property_type_map_v1 *,
thinkthen_cobol_ReceiptInputPropertyTypeMap *);
static int copy_thinkthen_cobol_ReceiptInputPropertyTypeMapEntry(arena *,const thinkthen_complete_input_property_type_map_entry_v1 *,
thinkthen_cobol_ReceiptInputPropertyTypeMapEntry *);
static int copy_thinkthen_cobol_ReceiptInputPropertyType(arena *,const thinkthen_complete_input_property_type_v1 *,
thinkthen_cobol_ReceiptInputPropertyType *);
static int copy_thinkthen_cobol_ReceiptInputPropertyTypeData(arena *,const thinkthen_complete_input_property_type_data_v1 *,
thinkthen_cobol_ReceiptInputPropertyTypeData *,uint32_t);
static int copy_thinkthen_cobol_ReceiptInputPropertyTypeArray(arena *,const thinkthen_complete_input_property_type_array_v1 *,
thinkthen_cobol_ReceiptInputPropertyTypeArray *);
static int copy_thinkthen_cobol_ReceiptStringRoot(arena *,const thinkthen_complete_string_root_v1 *,
thinkthen_cobol_ReceiptStringRoot *);
static int copy_thinkthen_cobol_ReceiptStringType(arena *,const thinkthen_complete_string_type_v1 *,
thinkthen_cobol_ReceiptStringType *);
static int copy_thinkthen_cobol_ReceiptInputPropertyTypeBoolean(arena *,const thinkthen_complete_input_property_type_boolean_v1 *,
thinkthen_cobol_ReceiptInputPropertyTypeBoolean *);
static int copy_thinkthen_cobol_ReceiptInputPropertyTypeNumber(arena *,const thinkthen_complete_input_property_type_number_v1 *,
thinkthen_cobol_ReceiptInputPropertyTypeNumber *);
static int copy_thinkthen_cobol_ReceiptInputPropertyTypeString(arena *,const thinkthen_complete_input_property_type_string_v1 *,
thinkthen_cobol_ReceiptInputPropertyTypeString *);
static int copy_thinkthen_cobol_ReceiptInputDeclarationStringRoot(arena *,const thinkthen_complete_input_declaration_string_root_v1 *,
thinkthen_cobol_ReceiptInputDeclarationStringRoot *);
static int copy_thinkthen_cobol_ReceiptRelateFieldsPresence(arena *,const thinkthen_complete_relate_fields_presence_v1 *,
thinkthen_cobol_ReceiptRelateFieldsPresence *);
static int copy_thinkthen_cobol_ReceiptRelateFields(arena *,const thinkthen_complete_relate_fields_v1 *,
thinkthen_cobol_ReceiptRelateFields *);
static int copy_thinkthen_cobol_ReceiptBatchPresence(arena *,const thinkthen_complete_batch_presence_v1 *,
thinkthen_cobol_ReceiptBatchPresence *);
static int copy_thinkthen_cobol_ReceiptBatch(arena *,const thinkthen_complete_batch_v1 *,thinkthen_cobol_ReceiptBatch *);
static int copy_thinkthen_cobol_ReceiptBatchData(arena *,const thinkthen_complete_batch_data_v1 *,
thinkthen_cobol_ReceiptBatchData *,uint32_t);
static int copy_thinkthen_cobol_ReceiptBatchString(arena *,const thinkthen_complete_batch_string_v1 *,
thinkthen_cobol_ReceiptBatchString *);
static int copy_thinkthen_cobol_ReceiptPositionPresence(arena *,const thinkthen_complete_position_presence_v1 *,
thinkthen_cobol_ReceiptPositionPresence *);
static int copy_thinkthen_cobol_ReceiptPosition(arena *,const thinkthen_complete_position_v1 *,
thinkthen_cobol_ReceiptPosition *);
static int copy_thinkthen_cobol_ReceiptMeta(arena *,const thinkthen_complete_meta_v1 *,thinkthen_cobol_ReceiptMeta *);
static int copy_thinkthen_cobol_ReceiptUsagePresence(arena *,const thinkthen_complete_usage_presence_v1 *,
thinkthen_cobol_ReceiptUsagePresence *);
static int copy_thinkthen_cobol_ReceiptUsage(arena *,const thinkthen_complete_usage_v1 *,thinkthen_cobol_ReceiptUsage *);
static int copy_thinkthen_cobol_ReceiptQuestionSourceList(arena *,const thinkthen_complete_question_source_list_v1 *,
thinkthen_cobol_ReceiptQuestionSourceList *);
static int copy_thinkthen_cobol_ReceiptQuestionSource(arena *,const thinkthen_complete_question_source_v1 *,
thinkthen_cobol_ReceiptQuestionSource *);
static int copy_thinkthen_cobol_ReceiptOrigin(arena *,const thinkthen_complete_origin_v1 *,
thinkthen_cobol_ReceiptOrigin *);
static int copy_thinkthen_cobol_ReceiptProfileWarningPresence(arena *,const thinkthen_complete_profile_warning_presence_v1 *,
thinkthen_cobol_ReceiptProfileWarningPresence *);
static int copy_thinkthen_cobol_ReceiptProfileWarning(arena *,const thinkthen_complete_profile_warning_v1 *,
thinkthen_cobol_ReceiptProfileWarning *);
static int copy_thinkthen_cobol_ReceiptOriginPresence(arena *,const thinkthen_complete_origin_presence_v1 *,
thinkthen_cobol_ReceiptOriginPresence *);
static int copy_thinkthen_cobol_ReceiptObservationList(arena *,const thinkthen_complete_observation_list_v1 *,
thinkthen_cobol_ReceiptObservationList *);
static int copy_thinkthen_cobol_ReceiptObservation(arena *,const thinkthen_complete_observation_v1 *,
thinkthen_cobol_ReceiptObservation *);
static int copy_thinkthen_cobol_ReceiptObservationData(arena *,const thinkthen_complete_observation_data_v1 *,
thinkthen_cobol_ReceiptObservationData *,uint32_t);
static int copy_thinkthen_cobol_ReceiptObservationFailureId(arena *,const thinkthen_complete_observation_failure_id_v1 *,
thinkthen_cobol_ReceiptObservationFailureId *);
static int copy_thinkthen_cobol_ReceiptFailureId(arena *,const thinkthen_complete_failure_id_v1 *,
thinkthen_cobol_ReceiptFailureId *);
static int copy_thinkthen_cobol_ReceiptObservationObservationId(arena *,const thinkthen_complete_observation_observation_id_v1 *,
thinkthen_cobol_ReceiptObservationObservationId *);
static int copy_thinkthen_cobol_ReceiptObservationId(arena *,const thinkthen_complete_observation_id_v1 *,
thinkthen_cobol_ReceiptObservationId *);
static int copy_thinkthen_cobol_ReceiptBatchWarningPresence(arena *,const thinkthen_complete_batch_warning_presence_v1 *,
thinkthen_cobol_ReceiptBatchWarningPresence *);
static int copy_thinkthen_cobol_ReceiptBatchWarning(arena *,const thinkthen_complete_batch_warning_v1 *,
thinkthen_cobol_ReceiptBatchWarning *);
static int copy_thinkthen_cobol_ReceiptBatchSetting(arena *,const thinkthen_complete_batch_setting_v1 *,
thinkthen_cobol_ReceiptBatchSetting *);
static int copy_thinkthen_cobol_ReceiptBatchSettingData(arena *,const thinkthen_complete_batch_setting_data_v1 *,
thinkthen_cobol_ReceiptBatchSettingData *,uint32_t);
static int copy_thinkthen_cobol_ReceiptBatchSettingPresence(arena *,const thinkthen_complete_batch_setting_presence_v1 *,
thinkthen_cobol_ReceiptBatchSettingPresence *);
static int copy_thinkthen_cobol_ReceiptAttemptListPresence(arena *,const thinkthen_complete_attempt_list_presence_v1 *,
thinkthen_cobol_ReceiptAttemptListPresence *);
static int copy_thinkthen_cobol_ReceiptAttemptList(arena *,const thinkthen_complete_attempt_list_v1 *,
thinkthen_cobol_ReceiptAttemptList *);
static int copy_thinkthen_cobol_ReceiptAttempt(arena *,const thinkthen_complete_attempt_v1 *,
thinkthen_cobol_ReceiptAttempt *);
static int copy_thinkthen_cobol_ReceiptSdkRequestId(arena *,const thinkthen_complete_sdk_request_id_v1 *,
thinkthen_cobol_ReceiptSdkRequestId *);
static int copy_thinkthen_cobol_ReceiptAttemptOutcome(arena *,const thinkthen_complete_attempt_outcome_v1 *,
thinkthen_cobol_ReceiptAttemptOutcome *);
static int copy_thinkthen_cobol_ReceiptSessionInputSourceListPresence(arena *,const thinkthen_complete_session_input_source_list_presence_v1 *,
thinkthen_cobol_ReceiptSessionInputSourceListPresence *);
static int copy_thinkthen_cobol_ReceiptSessionInputSourceList(arena *,const thinkthen_complete_session_input_source_list_v1 *,
thinkthen_cobol_ReceiptSessionInputSourceList *);
static int copy_thinkthen_cobol_ReceiptSessionInputSource(arena *,const thinkthen_complete_session_input_source_v1 *,
thinkthen_cobol_ReceiptSessionInputSource *);
static int copy_thinkthen_cobol_ReceiptPhysicalSource(arena *,const thinkthen_complete_physical_source_v1 *,
thinkthen_cobol_ReceiptPhysicalSource *);
static int copy_thinkthen_cobol_ReceiptAnswerId(arena *,const thinkthen_complete_answer_id_v1 *,
thinkthen_cobol_ReceiptAnswerId *);
static int copy_thinkthen_cobol_ReceiptAnswers(arena *,const thinkthen_complete_answers_v1 *,
thinkthen_cobol_ReceiptAnswers *);
static int copy_thinkthen_cobol_ReceiptRelationMemberList(arena *,const thinkthen_complete_relation_member_list_v1 *,
thinkthen_cobol_ReceiptRelationMemberList *);
static int copy_thinkthen_cobol_ReceiptRelationMember(arena *,const thinkthen_complete_relation_member_v1 *,
thinkthen_cobol_ReceiptRelationMember *);
static int copy_thinkthen_cobol_ReceiptRelationMemberData(arena *,const thinkthen_complete_relation_member_data_v1 *,
thinkthen_cobol_ReceiptRelationMemberData *,uint32_t);
static int copy_thinkthen_cobol_ReceiptRelationMemberFailureId(arena *,const thinkthen_complete_relation_member_failure_id_v1 *,
thinkthen_cobol_ReceiptRelationMemberFailureId *);
static int copy_thinkthen_cobol_ReceiptFailure(arena *,const thinkthen_complete_failure_v1 *,
thinkthen_cobol_ReceiptFailure *);
static int copy_thinkthen_cobol_ReceiptFailureFieldKind(arena *,const thinkthen_complete_failure_field_kind_v1 *,
thinkthen_cobol_ReceiptFailureFieldKind *);
static int copy_thinkthen_cobol_ReceiptFailureCause(arena *,const thinkthen_complete_failure_cause_v1 *,
thinkthen_cobol_ReceiptFailureCause *);
static int copy_thinkthen_cobol_ReceiptRelatedEntityPresence(arena *,const thinkthen_complete_related_entity_presence_v1 *,
thinkthen_cobol_ReceiptRelatedEntityPresence *);
static int copy_thinkthen_cobol_ReceiptRelatedEntity(arena *,const thinkthen_complete_related_entity_v1 *,
thinkthen_cobol_ReceiptRelatedEntity *);
static int copy_thinkthen_cobol_ReceiptQuestionDocument(arena *,const thinkthen_complete_question_document_v1 *,
thinkthen_cobol_ReceiptQuestionDocument *);
static int copy_thinkthen_cobol_ReceiptQuestionDocumentData(arena *,const thinkthen_complete_question_document_data_v1 *,
thinkthen_cobol_ReceiptQuestionDocumentData *,uint32_t);
static int copy_thinkthen_cobol_ReceiptQuestionDocumentScore(arena *,const thinkthen_complete_question_document_score_v1 *,
thinkthen_cobol_ReceiptQuestionDocumentScore *);
static int copy_thinkthen_cobol_ReceiptQuestionDocumentTag(arena *,const thinkthen_complete_question_document_tag_v1 *,
thinkthen_cobol_ReceiptQuestionDocumentTag *);
static int copy_thinkthen_cobol_ReceiptQuestionDocumentChoose(arena *,const thinkthen_complete_question_document_choose_v1 *,
thinkthen_cobol_ReceiptQuestionDocumentChoose *);
static int copy_thinkthen_cobol_ReceiptQuestionDocumentDecide(arena *,const thinkthen_complete_question_document_decide_v1 *,
thinkthen_cobol_ReceiptQuestionDocumentDecide *);
static int copy_thinkthen_cobol_ReceiptRelationMethod(arena *,const thinkthen_complete_relation_method_v1 *,
thinkthen_cobol_ReceiptRelationMethod *);
static int copy_thinkthen_cobol_ReceiptRelationDirection(arena *,const thinkthen_complete_relation_direction_v1 *,
thinkthen_cobol_ReceiptRelationDirection *);
static int copy_thinkthen_cobol_ReceiptRelationMemberAnswerId(arena *,const thinkthen_complete_relation_member_answer_id_v1 *,
thinkthen_cobol_ReceiptRelationMemberAnswerId *);
static int copy_thinkthen_cobol_ReceiptAnswer(arena *,const thinkthen_complete_answer_v1 *,
thinkthen_cobol_ReceiptAnswer *);
static int copy_thinkthen_cobol_ReceiptAnswerData(arena *,const thinkthen_complete_answer_data_v1 *,
thinkthen_cobol_ReceiptAnswerData *,uint32_t);
static int copy_thinkthen_cobol_ReceiptAnswerScore(arena *,const thinkthen_complete_answer_score_v1 *,
thinkthen_cobol_ReceiptAnswerScore *);
static int copy_thinkthen_cobol_ReceiptNumberMap(arena *,const thinkthen_complete_number_map_v1 *,
thinkthen_cobol_ReceiptNumberMap *);
static int copy_thinkthen_cobol_ReceiptNumberMapEntry(arena *,const thinkthen_complete_number_map_entry_v1 *,
thinkthen_cobol_ReceiptNumberMapEntry *);
static int copy_thinkthen_cobol_ReceiptF64Presence(arena *,const thinkthen_complete_f64_presence_v1 *,
thinkthen_cobol_ReceiptF64Presence *);
static int copy_thinkthen_cobol_ReceiptAnswerTag(arena *,const thinkthen_complete_answer_tag_v1 *,
thinkthen_cobol_ReceiptAnswerTag *);
static int copy_thinkthen_cobol_ReceiptAnswerChoice(arena *,const thinkthen_complete_answer_choice_v1 *,
thinkthen_cobol_ReceiptAnswerChoice *);
static int copy_thinkthen_cobol_ReceiptAnswerYesNo(arena *,const thinkthen_complete_answer_yes_no_v1 *,
thinkthen_cobol_ReceiptAnswerYesNo *);
static int copy_thinkthen_cobol_ReceiptRecognitionList(arena *,const thinkthen_complete_recognition_list_v1 *,
thinkthen_cobol_ReceiptRecognitionList *);
static int copy_thinkthen_cobol_ReceiptRecognition(arena *,const thinkthen_complete_recognition_v1 *,
thinkthen_cobol_ReceiptRecognition *);
static int copy_thinkthen_cobol_ReceiptRecognize(arena *,const thinkthen_complete_recognize_v1 *,
thinkthen_cobol_ReceiptRecognize *);
static int copy_thinkthen_cobol_ReceiptRecognizeData(arena *,const thinkthen_complete_recognize_data_v1 *,
thinkthen_cobol_ReceiptRecognizeData *,uint32_t);
static int copy_thinkthen_cobol_ReceiptRecognizeModeProposals(arena *,const thinkthen_complete_recognize_mode_proposals_v1 *,
thinkthen_cobol_ReceiptRecognizeModeProposals *);
static int copy_thinkthen_cobol_ReceiptBoundaryProposalList(arena *,const thinkthen_complete_boundary_proposal_list_v1 *,
thinkthen_cobol_ReceiptBoundaryProposalList *);
static int copy_thinkthen_cobol_ReceiptBoundaryProposal(arena *,const thinkthen_complete_boundary_proposal_v1 *,
thinkthen_cobol_ReceiptBoundaryProposal *);
static int copy_thinkthen_cobol_ReceiptBoundaryMode(arena *,const thinkthen_complete_boundary_mode_v1 *,
thinkthen_cobol_ReceiptBoundaryMode *);
static int copy_thinkthen_cobol_ReceiptRecognizeEntities(arena *,const thinkthen_complete_recognize_entities_v1 *,
thinkthen_cobol_ReceiptRecognizeEntities *);
static int copy_thinkthen_cobol_ReceiptEntityEdgeListPresence(arena *,const thinkthen_complete_entity_edge_list_presence_v1 *,
thinkthen_cobol_ReceiptEntityEdgeListPresence *);
static int copy_thinkthen_cobol_ReceiptEntityEdgeList(arena *,const thinkthen_complete_entity_edge_list_v1 *,
thinkthen_cobol_ReceiptEntityEdgeList *);
static int copy_thinkthen_cobol_ReceiptEntityEdge(arena *,const thinkthen_complete_entity_edge_v1 *,
thinkthen_cobol_ReceiptEntityEdge *);
static int copy_thinkthen_cobol_ReceiptEntity(arena *,const thinkthen_complete_entity_v1 *,
thinkthen_cobol_ReceiptEntity *);
static int copy_thinkthen_cobol_ReceiptEntityList(arena *,const thinkthen_complete_entity_list_v1 *,
thinkthen_cobol_ReceiptEntityList *);
static int copy_thinkthen_cobol_ReceiptPhysicalSourcePresence(arena *,const thinkthen_complete_physical_source_presence_v1 *,
thinkthen_cobol_ReceiptPhysicalSourcePresence *);
static int copy_thinkthen_cobol_ReceiptRecognizeQuestionDocument(arena *,const thinkthen_complete_recognize_question_document_v1 *,
thinkthen_cobol_ReceiptRecognizeQuestionDocument *);
static int copy_thinkthen_cobol_ReceiptVerb(arena *,const thinkthen_complete_verb_v1 *,thinkthen_cobol_ReceiptVerb *);
static int copy_thinkthen_cobol_ReceiptRecognitionStageContextPresence(arena *,const thinkthen_complete_recognition_stage_context_presence_v1 *,
thinkthen_cobol_ReceiptRecognitionStageContextPresence *);
static int copy_thinkthen_cobol_ReceiptRecognitionStageContext(arena *,const thinkthen_complete_recognition_stage_context_v1 *,
thinkthen_cobol_ReceiptRecognitionStageContext *);
static int copy_thinkthen_cobol_ReceiptRelationRuleListPresence(arena *,const thinkthen_complete_relation_rule_list_presence_v1 *,
thinkthen_cobol_ReceiptRelationRuleListPresence *);
static int copy_thinkthen_cobol_ReceiptThresholdPresence(arena *,const thinkthen_complete_threshold_presence_v1 *,
thinkthen_cobol_ReceiptThresholdPresence *);
static int copy_thinkthen_cobol_ReceiptRecognitionModePresence(arena *,const thinkthen_complete_recognition_mode_presence_v1 *,
thinkthen_cobol_ReceiptRecognitionModePresence *);
static int copy_thinkthen_cobol_ReceiptRecognitionMode(arena *,const thinkthen_complete_recognition_mode_v1 *,
thinkthen_cobol_ReceiptRecognitionMode *);
static int copy_thinkthen_cobol_ReceiptJsonMap(arena *,const thinkthen_complete_json_map_v1 *,
thinkthen_cobol_ReceiptJsonMap *);
static int copy_thinkthen_cobol_ReceiptJsonMapEntry(arena *,const thinkthen_complete_json_map_entry_v1 *,
thinkthen_cobol_ReceiptJsonMapEntry *);
static int copy_thinkthen_cobol_ReceiptRecognitionOdds(arena *,const thinkthen_complete_recognition_odds_v1 *,
thinkthen_cobol_ReceiptRecognitionOdds *);
static int copy_thinkthen_cobol_ReceiptRecognitionOddsData(arena *,const thinkthen_complete_recognition_odds_data_v1 *,
thinkthen_cobol_ReceiptRecognitionOddsData *,uint32_t);
static int copy_thinkthen_cobol_ReceiptRecognitionOddsBoundaryOdds(arena *,const thinkthen_complete_recognition_odds_boundary_odds_v1 *,
thinkthen_cobol_ReceiptRecognitionOddsBoundaryOdds *);
static int copy_thinkthen_cobol_ReceiptPieceOddsList(arena *,const thinkthen_complete_piece_odds_list_v1 *,
thinkthen_cobol_ReceiptPieceOddsList *);
static int copy_thinkthen_cobol_ReceiptPieceOdds(arena *,const thinkthen_complete_piece_odds_v1 *,
thinkthen_cobol_ReceiptPieceOdds *);
static int copy_thinkthen_cobol_ReceiptRecognitionOddsRecognizeAnswer(arena *,const thinkthen_complete_recognition_odds_recognize_answer_v1 *,
thinkthen_cobol_ReceiptRecognitionOddsRecognizeAnswer *);
static int copy_thinkthen_cobol_ReceiptRecognitionProposalList(arena *,const thinkthen_complete_recognition_proposal_list_v1 *,
thinkthen_cobol_ReceiptRecognitionProposalList *);
static int copy_thinkthen_cobol_ReceiptRecognitionProposal(arena *,const thinkthen_complete_recognition_proposal_v1 *,
thinkthen_cobol_ReceiptRecognitionProposal *);
static int copy_thinkthen_cobol_ReceiptPlacePresence(arena *,const thinkthen_complete_place_presence_v1 *,
thinkthen_cobol_ReceiptPlacePresence *);
static int copy_thinkthen_cobol_ReceiptPlace(arena *,const thinkthen_complete_place_v1 *,thinkthen_cobol_ReceiptPlace *);
static int copy_thinkthen_cobol_ReceiptPairOddsList(arena *,const thinkthen_complete_pair_odds_list_v1 *,
thinkthen_cobol_ReceiptPairOddsList *);
static int copy_thinkthen_cobol_ReceiptPairOdds(arena *,const thinkthen_complete_pair_odds_v1 *,
thinkthen_cobol_ReceiptPairOdds *);
static int copy_thinkthen_cobol_ReceiptNameOddsList(arena *,const thinkthen_complete_name_odds_list_v1 *,
thinkthen_cobol_ReceiptNameOddsList *);
static int copy_thinkthen_cobol_ReceiptNameOdds(arena *,const thinkthen_complete_name_odds_v1 *,
thinkthen_cobol_ReceiptNameOdds *);
static int copy_thinkthen_cobol_ReceiptNumberMapPresence(arena *,const thinkthen_complete_number_map_presence_v1 *,
thinkthen_cobol_ReceiptNumberMapPresence *);
static int copy_thinkthen_cobol_ReceiptAnnotationList(arena *,const thinkthen_complete_annotation_list_v1 *,
thinkthen_cobol_ReceiptAnnotationList *);
static int copy_thinkthen_cobol_ReceiptAnnotation(arena *,const thinkthen_complete_annotation_v1 *,
thinkthen_cobol_ReceiptAnnotation *);
static int copy_thinkthen_cobol_ReceiptAnnotatedRow(arena *,const thinkthen_complete_annotated_row_v1 *,
thinkthen_cobol_ReceiptAnnotatedRow *);
static int copy_thinkthen_cobol_ReceiptAnnotatedFieldMap(arena *,const thinkthen_complete_annotated_field_map_v1 *,
thinkthen_cobol_ReceiptAnnotatedFieldMap *);
static int copy_thinkthen_cobol_ReceiptAnnotatedFieldMapEntry(arena *,const thinkthen_complete_annotated_field_map_entry_v1 *,
thinkthen_cobol_ReceiptAnnotatedFieldMapEntry *);
static int copy_thinkthen_cobol_ReceiptAnnotatedField(arena *,const thinkthen_complete_annotated_field_v1 *,
thinkthen_cobol_ReceiptAnnotatedField *);
static int copy_thinkthen_cobol_ReceiptAnnotatedFieldData(arena *,const thinkthen_complete_annotated_field_data_v1 *,
thinkthen_cobol_ReceiptAnnotatedFieldData *,uint32_t);
static int copy_thinkthen_cobol_ReceiptFailed(arena *,const thinkthen_complete_failed_v1 *,
thinkthen_cobol_ReceiptFailed *);
static int copy_thinkthen_cobol_ReceiptAnnotationMemberMap(arena *,const thinkthen_complete_annotation_member_map_v1 *,
thinkthen_cobol_ReceiptAnnotationMemberMap *);
static int copy_thinkthen_cobol_ReceiptAnnotationMemberMapEntry(arena *,const thinkthen_complete_annotation_member_map_entry_v1 *,
thinkthen_cobol_ReceiptAnnotationMemberMapEntry *);
static int copy_thinkthen_cobol_ReceiptAnnotationMember(arena *,const thinkthen_complete_annotation_member_v1 *,
thinkthen_cobol_ReceiptAnnotationMember *);
static int copy_thinkthen_cobol_ReceiptAnnotationMemberData(arena *,const thinkthen_complete_annotation_member_data_v1 *,
thinkthen_cobol_ReceiptAnnotationMemberData *,uint32_t);
static int copy_thinkthen_cobol_ReceiptAnnotationMemberFailureId(arena *,const thinkthen_complete_annotation_member_failure_id_v1 *,
thinkthen_cobol_ReceiptAnnotationMemberFailureId *);
static int copy_thinkthen_cobol_ReceiptAnnotationMemberAnswerId(arena *,const thinkthen_complete_annotation_member_answer_id_v1 *,
thinkthen_cobol_ReceiptAnnotationMemberAnswerId *);
static int copy_thinkthen_cobol_ReceiptValuePresence(arena *,const thinkthen_complete_value_presence_v1 *,
thinkthen_cobol_ReceiptValuePresence *);
static int copy_thinkthen_cobol_ReceiptValue(arena *,const thinkthen_complete_value_v1 *,thinkthen_cobol_ReceiptValue *);
static int copy_thinkthen_cobol_ReceiptValueData(arena *,const thinkthen_complete_value_data_v1 *,
thinkthen_cobol_ReceiptValueData *,uint32_t);
static int copy_thinkthen_cobol_ReceiptFind(arena *,const thinkthen_complete_find_v1 *,thinkthen_cobol_ReceiptFind *);
static int copy_thinkthen_cobol_ReceiptFindQuestionDocument(arena *,const thinkthen_complete_find_question_document_v1 *,
thinkthen_cobol_ReceiptFindQuestionDocument *);
static int copy_thinkthen_cobol_ReceiptFindCandidateListPresence(arena *,const thinkthen_complete_find_candidate_list_presence_v1 *,
thinkthen_cobol_ReceiptFindCandidateListPresence *);
static int copy_thinkthen_cobol_ReceiptFindCandidateList(arena *,const thinkthen_complete_find_candidate_list_v1 *,
thinkthen_cobol_ReceiptFindCandidateList *);
static int copy_thinkthen_cobol_ReceiptFindCandidate(arena *,const thinkthen_complete_find_candidate_v1 *,
thinkthen_cobol_ReceiptFindCandidate *);
static int copy_thinkthen_cobol_ReceiptFindAnswer(arena *,const thinkthen_complete_find_answer_v1 *,
thinkthen_cobol_ReceiptFindAnswer *);
static int copy_thinkthen_cobol_ReceiptAtomicNonZeroUsizeList(arena *,const thinkthen_complete_atomic_non_zero_usize_list_v1 *,
thinkthen_cobol_ReceiptAtomicNonZeroUsizeList *);
static int copy_thinkthen_cobol_ReceiptAtomicNonZeroUsize(arena *,const thinkthen_complete_atomic_non_zero_usize_v1 *,
thinkthen_cobol_ReceiptAtomicNonZeroUsize *);
static int copy_thinkthen_cobol_ReceiptRankMemberListPresence(arena *,const thinkthen_complete_rank_member_list_presence_v1 *,
thinkthen_cobol_ReceiptRankMemberListPresence *);
static int copy_thinkthen_cobol_ReceiptRankMemberList(arena *,const thinkthen_complete_rank_member_list_v1 *,
thinkthen_cobol_ReceiptRankMemberList *);
static int copy_thinkthen_cobol_ReceiptRankMember(arena *,const thinkthen_complete_rank_member_v1 *,
thinkthen_cobol_ReceiptRankMember *);
static int copy_thinkthen_cobol_ReceiptRankMemberResult(arena *,const thinkthen_complete_rank_member_result_v1 *,
thinkthen_cobol_ReceiptRankMemberResult *);
static int copy_thinkthen_cobol_ReceiptImageListPresence(arena *,const thinkthen_complete_image_list_presence_v1 *,
thinkthen_cobol_ReceiptImageListPresence *);
static int copy_thinkthen_cobol_ReceiptImageList(arena *,const thinkthen_complete_image_list_v1 *,
thinkthen_cobol_ReceiptImageList *);
static int copy_thinkthen_cobol_ReceiptImage(arena *,const thinkthen_complete_image_v1 *,thinkthen_cobol_ReceiptImage *);
static int copy_thinkthen_cobol_ReceiptImageMedia(arena *,const thinkthen_complete_image_media_v1 *,
thinkthen_cobol_ReceiptImageMedia *);
static int copy_thinkthen_cobol_ReceiptAtomicBooleanList(arena *,const thinkthen_complete_atomic_boolean_list_v1 *,
thinkthen_cobol_ReceiptAtomicBooleanList *);
static int copy_thinkthen_cobol_ReceiptAtomicBoolean(arena *,const thinkthen_complete_atomic_boolean_v1 *,
thinkthen_cobol_ReceiptAtomicBoolean *);
static int copy_thinkthen_cobol_ReceiptAtomicDoubleList(arena *,const thinkthen_complete_atomic_double_list_v1 *,
thinkthen_cobol_ReceiptAtomicDoubleList *);
static int copy_thinkthen_cobol_ReceiptAtomicDouble(arena *,const thinkthen_complete_atomic_double_v1 *,
thinkthen_cobol_ReceiptAtomicDouble *);
static int copy_thinkthen_cobol_ReceiptAtomicArrayOfStringList(arena *,const thinkthen_complete_atomic_array_of_string_list_v1 *,
thinkthen_cobol_ReceiptAtomicArrayOfStringList *);
static int copy_thinkthen_cobol_ReceiptAtomicArrayOfString(arena *,const thinkthen_complete_atomic_array_of_string_v1 *,
thinkthen_cobol_ReceiptAtomicArrayOfString *);
static int copy_thinkthen_cobol_ReceiptAtomicNullableStringList(arena *,const thinkthen_complete_atomic_nullable_string_list_v1 *,
thinkthen_cobol_ReceiptAtomicNullableStringList *);
static int copy_thinkthen_cobol_ReceiptAtomicNullableString(arena *,const thinkthen_complete_atomic_nullable_string_v1 *,
thinkthen_cobol_ReceiptAtomicNullableString *);
static int copy_thinkthen_cobol_ReceiptAtomicDecideValueList(arena *,const thinkthen_complete_atomic_decide_value_list_v1 *,
thinkthen_cobol_ReceiptAtomicDecideValueList *);
static int copy_thinkthen_cobol_ReceiptAtomicDecideValue(arena *,const thinkthen_complete_atomic_decide_value_v1 *,
thinkthen_cobol_ReceiptAtomicDecideValue *);
static int copy_thinkthen_cobol_ReceiptDecideValuePresence(arena *,const thinkthen_complete_decide_value_presence_v1 *,
thinkthen_cobol_ReceiptDecideValuePresence *);
static int copy_thinkthen_cobol_ReceiptDecideValue(arena *,const thinkthen_complete_decide_value_v1 *,
thinkthen_cobol_ReceiptDecideValue *);
static int copy_thinkthen_cobol_ReceiptExtensions(arena *a,const thinkthen_complete_extensions_v1 *input,
thinkthen_cobol_ReceiptExtensions *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptExtension *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptExtension(a,&input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptExtension(arena *a,const thinkthen_complete_extension_v1 *input,
thinkthen_cobol_ReceiptExtension *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->name,&out->name);
    if(input->value) out->value=answer_original(a,input->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelatedEntityEdgeList(arena *a,const thinkthen_complete_related_entity_edge_list_v1 *input,
thinkthen_cobol_ReceiptRelatedEntityEdgeList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptRelatedEntityEdge *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptRelatedEntityEdge(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelationEndpointData(arena *a,const thinkthen_complete_relation_endpoint_data_v1 *input,
thinkthen_cobol_ReceiptRelationEndpointData *out,uint32_t kind) {
    int code=0;
    (void)a;
    if(kind==THINKTHEN_COMPLETE_RELATION_ENDPOINT_RELATED_ENTITY_V1) {
        if(input->related_entity) {
            thinkthen_cobol_ReceiptRelationEndpointRelatedEntity *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptRelationEndpointRelatedEntity(a,input->related_entity,
            value);
            out->related_entity=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_RELATION_ENDPOINT_SOURCE_V1) {
        if(input->source) {
            thinkthen_cobol_ReceiptRelationEndpointSource *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptRelationEndpointSource(a,input->source,value);
            out->source=value;
        }
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptJsonPresence(arena *a,const thinkthen_complete_json_presence_v1 *input,
thinkthen_cobol_ReceiptJsonPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) out->value=answer_original(a,input->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptU64Presence(arena *a,const thinkthen_complete_u64_presence_v1 *input,
thinkthen_cobol_ReceiptU64Presence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    out->value=input->value;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptUtf8Presence(arena *a,const thinkthen_complete_utf8_presence_v1 *input,
thinkthen_cobol_ReceiptUtf8Presence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(!code) code=answer_text(input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelationEndpointRelatedEntity(arena *a,const thinkthen_complete_relation_endpoint_related_entity_v1 *input,
thinkthen_cobol_ReceiptRelationEndpointRelatedEntity *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->kind,&out->kind);
    if(!code) code=answer_text(input->name,&out->name);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptU32Presence(arena *a,const thinkthen_complete_u32_presence_v1 *input,
thinkthen_cobol_ReceiptU32Presence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    out->value=input->value;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptVersion(arena *a,const thinkthen_complete_version_v1 *input,
thinkthen_cobol_ReceiptVersion *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptWordingVersionPresence(arena *a,const thinkthen_complete_wording_version_presence_v1 *input,
thinkthen_cobol_ReceiptWordingVersionPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_ReceiptWordingVersion *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptWordingVersion(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptWordingVersion(arena *a,const thinkthen_complete_wording_version_v1 *input,
thinkthen_cobol_ReceiptWordingVersion *out) {
    int code=0;
    (void)a;
    out->value=input->value;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptThresholdData(arena *a,const thinkthen_complete_threshold_data_v1 *input,
thinkthen_cobol_ReceiptThresholdData *out,uint32_t kind) {
    int code=0;
    (void)a;
    if(kind==THINKTHEN_COMPLETE_THRESHOLD_NUMBER_V1) {
        out->number=input->number;
    }
    if(kind==THINKTHEN_COMPLETE_THRESHOLD_STRING_V1) {
        if(!code) code=answer_text(input->string,&out->string);
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelationRuleList(arena *a,const thinkthen_complete_relation_rule_list_v1 *input,
thinkthen_cobol_ReceiptRelationRuleList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptRelationRule *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptRelationRule(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelationRule(arena *a,const thinkthen_complete_relation_rule_v1 *input,
thinkthen_cobol_ReceiptRelationRule *out) {
    int code=0;
    (void)a;
    out->either=input->either;
    if(!code) code=answer_text(input->name,&out->name);
    if(!code) code=answer_text(input->reads,&out->reads);
    if(!code) code=copy_thinkthen_cobol_ReceiptU32Presence(a,&input->single,&out->single);
    if(!code) code=answer_text(input->source,&out->source);
    if(!code) code=answer_text(input->target,&out->target);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptStringList(arena *a,const thinkthen_complete_string_list_v1 *input,
thinkthen_cobol_ReceiptStringList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_AnswerText *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=answer_text(*(&input->data[i]),items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptQuestionNamePresence(arena *a,const thinkthen_complete_question_name_presence_v1 *input,
thinkthen_cobol_ReceiptQuestionNamePresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_ReceiptQuestionName *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptQuestionName(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptQuestionName(arena *a,const thinkthen_complete_question_name_v1 *input,
thinkthen_cobol_ReceiptQuestionName *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptLabelList(arena *a,const thinkthen_complete_label_list_v1 *input,
thinkthen_cobol_ReceiptLabelList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptLabel *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptLabel(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptLabel(arena *a,const thinkthen_complete_label_v1 *input,
thinkthen_cobol_ReceiptLabel *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->description,&out->description);
    if(!code) code=answer_text(input->name,&out->name);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptInputDeclarationPresence(arena *a,const thinkthen_complete_input_declaration_presence_v1 *input,
thinkthen_cobol_ReceiptInputDeclarationPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_ReceiptInputDeclaration *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptInputDeclaration(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptInputDeclarationData(arena *a,const thinkthen_complete_input_declaration_data_v1 *input,
thinkthen_cobol_ReceiptInputDeclarationData *out,uint32_t kind) {
    int code=0;
    (void)a;
    if(kind==THINKTHEN_COMPLETE_INPUT_DECLARATION_STRING_ROOT_V1) {
        if(input->string_root) {
            thinkthen_cobol_ReceiptInputDeclarationStringRoot *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptInputDeclarationStringRoot(a,input->string_root,
            value);
            out->string_root=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_INPUT_DECLARATION_OBJECT_ROOT_V1) {
        if(input->object_root) {
            thinkthen_cobol_ReceiptInputDeclarationObjectRoot *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptInputDeclarationObjectRoot(a,input->object_root,
            value);
            out->object_root=value;
        }
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptObjectType(arena *a,const thinkthen_complete_object_type_v1 *input,
thinkthen_cobol_ReceiptObjectType *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptInputPropertyTypeMap(arena *a,const thinkthen_complete_input_property_type_map_v1 *input,
thinkthen_cobol_ReceiptInputPropertyTypeMap *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptInputPropertyTypeMapEntry *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptInputPropertyTypeMapEntry(a,&input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptInputPropertyTypeMapEntry(arena *a,const thinkthen_complete_input_property_type_map_entry_v1 *input,
thinkthen_cobol_ReceiptInputPropertyTypeMapEntry *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->name,&out->name);
    if(input->value) {
        thinkthen_cobol_ReceiptInputPropertyType *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptInputPropertyType(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptInputPropertyTypeData(arena *a,const thinkthen_complete_input_property_type_data_v1 *input,
thinkthen_cobol_ReceiptInputPropertyTypeData *out,uint32_t kind) {
    int code=0;
    (void)a;
    if(kind==THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_STRING_V1) {
        if(input->string) {
            thinkthen_cobol_ReceiptInputPropertyTypeString *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptInputPropertyTypeString(a,input->string,value);
            out->string=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_NUMBER_V1) {
        if(input->number) {
            thinkthen_cobol_ReceiptInputPropertyTypeNumber *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptInputPropertyTypeNumber(a,input->number,value);
            out->number=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_BOOLEAN_V1) {
        if(input->boolean) {
            thinkthen_cobol_ReceiptInputPropertyTypeBoolean *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptInputPropertyTypeBoolean(a,input->boolean,value);
            out->boolean=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_INPUT_PROPERTY_TYPE_ARRAY_V1) {
        if(input->array) {
            thinkthen_cobol_ReceiptInputPropertyTypeArray *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptInputPropertyTypeArray(a,input->array,value);
            out->array=value;
        }
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptInputPropertyTypeArray(arena *a,const thinkthen_complete_input_property_type_array_v1 *input,
thinkthen_cobol_ReceiptInputPropertyTypeArray *out) {
    int code=0;
    (void)a;
    if(input->items) {
        thinkthen_cobol_ReceiptStringRoot *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptStringRoot(a,input->items,value);
        out->items=value;
    }
    if(!code) code=answer_text(input->type,&out->type);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptStringRoot(arena *a,const thinkthen_complete_string_root_v1 *input,
thinkthen_cobol_ReceiptStringRoot *out) {
    int code=0;
    (void)a;
    if(input->type) {
        thinkthen_cobol_ReceiptStringType *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptStringType(a,input->type,value);
        out->type=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptStringType(arena *a,const thinkthen_complete_string_type_v1 *input,
thinkthen_cobol_ReceiptStringType *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptInputPropertyTypeBoolean(arena *a,const thinkthen_complete_input_property_type_boolean_v1 *input,
thinkthen_cobol_ReceiptInputPropertyTypeBoolean *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->type,&out->type);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptInputPropertyTypeNumber(arena *a,const thinkthen_complete_input_property_type_number_v1 *input,
thinkthen_cobol_ReceiptInputPropertyTypeNumber *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->type,&out->type);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptInputPropertyTypeString(arena *a,const thinkthen_complete_input_property_type_string_v1 *input,
thinkthen_cobol_ReceiptInputPropertyTypeString *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->type,&out->type);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptInputDeclarationStringRoot(arena *a,const thinkthen_complete_input_declaration_string_root_v1 *input,
thinkthen_cobol_ReceiptInputDeclarationStringRoot *out) {
    int code=0;
    (void)a;
    if(input->type) {
        thinkthen_cobol_ReceiptStringType *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptStringType(a,input->type,value);
        out->type=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelateFieldsPresence(arena *a,const thinkthen_complete_relate_fields_presence_v1 *input,
thinkthen_cobol_ReceiptRelateFieldsPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_ReceiptRelateFields *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptRelateFields(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelateFields(arena *a,const thinkthen_complete_relate_fields_v1 *input,
thinkthen_cobol_ReceiptRelateFields *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->kind,&out->kind);
    if(!code) code=answer_text(input->name,&out->name);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptBatchPresence(arena *a,const thinkthen_complete_batch_presence_v1 *input,
thinkthen_cobol_ReceiptBatchPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_ReceiptBatch *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptBatch(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptBatchData(arena *a,const thinkthen_complete_batch_data_v1 *input,
thinkthen_cobol_ReceiptBatchData *out,uint32_t kind) {
    int code=0;
    (void)a;
    if(kind==THINKTHEN_COMPLETE_BATCH_INTEGER_V1) {
        out->integer=input->integer;
    }
    if(kind==THINKTHEN_COMPLETE_BATCH_STRING_V1) {
        if(input->string) {
            thinkthen_cobol_ReceiptBatchString *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptBatchString(a,input->string,value);
            out->string=value;
        }
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptBatchString(arena *a,const thinkthen_complete_batch_string_v1 *input,
thinkthen_cobol_ReceiptBatchString *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptPositionPresence(arena *a,const thinkthen_complete_position_presence_v1 *input,
thinkthen_cobol_ReceiptPositionPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_ReceiptPosition *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptPosition(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptUsagePresence(arena *a,const thinkthen_complete_usage_presence_v1 *input,
thinkthen_cobol_ReceiptUsagePresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_ReceiptUsage *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptUsage(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptUsage(arena *a,const thinkthen_complete_usage_v1 *input,
thinkthen_cobol_ReceiptUsage *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->input_tokens,&out->input_tokens);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->output_tokens,&out->output_tokens);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptQuestionSourceList(arena *a,const thinkthen_complete_question_source_list_v1 *input,
thinkthen_cobol_ReceiptQuestionSourceList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptQuestionSource *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptQuestionSource(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptQuestionSource(arena *a,const thinkthen_complete_question_source_v1 *input,
thinkthen_cobol_ReceiptQuestionSource *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->answered_by,&out->answered_by);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->batch_size,&out->batch_size);
    if(input->origin) {
        thinkthen_cobol_ReceiptOrigin *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptOrigin(a,input->origin,value);
        out->origin=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptOrigin(arena *a,const thinkthen_complete_origin_v1 *input,
thinkthen_cobol_ReceiptOrigin *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptProfileWarningPresence(arena *a,const thinkthen_complete_profile_warning_presence_v1 *input,
thinkthen_cobol_ReceiptProfileWarningPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_ReceiptProfileWarning *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptProfileWarning(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptProfileWarning(arena *a,const thinkthen_complete_profile_warning_v1 *input,
thinkthen_cobol_ReceiptProfileWarning *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->running,&out->running);
    if(!code) code=answer_text(input->tuned_for,&out->tuned_for);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptOriginPresence(arena *a,const thinkthen_complete_origin_presence_v1 *input,
thinkthen_cobol_ReceiptOriginPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_ReceiptOrigin *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptOrigin(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptObservationList(arena *a,const thinkthen_complete_observation_list_v1 *input,
thinkthen_cobol_ReceiptObservationList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptObservation *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptObservation(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptObservationData(arena *a,const thinkthen_complete_observation_data_v1 *input,
thinkthen_cobol_ReceiptObservationData *out,uint32_t kind) {
    int code=0;
    (void)a;
    if(kind==THINKTHEN_COMPLETE_OBSERVATION_OBSERVATION_ID_V1) {
        if(input->observation_id) {
            thinkthen_cobol_ReceiptObservationObservationId *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptObservationObservationId(a,input->observation_id,
            value);
            out->observation_id=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_OBSERVATION_FAILURE_ID_V1) {
        if(input->failure_id) {
            thinkthen_cobol_ReceiptObservationFailureId *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptObservationFailureId(a,input->failure_id,value);
            out->failure_id=value;
        }
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptObservationFailureId(arena *a,const thinkthen_complete_observation_failure_id_v1 *input,
thinkthen_cobol_ReceiptObservationFailureId *out) {
    int code=0;
    (void)a;
    if(input->failure_id) {
        thinkthen_cobol_ReceiptFailureId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptFailureId(a,input->failure_id,value);
        out->failure_id=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptFailureId(arena *a,const thinkthen_complete_failure_id_v1 *input,
thinkthen_cobol_ReceiptFailureId *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptObservationObservationId(arena *a,const thinkthen_complete_observation_observation_id_v1 *input,
thinkthen_cobol_ReceiptObservationObservationId *out) {
    int code=0;
    (void)a;
    if(input->observation_id) {
        thinkthen_cobol_ReceiptObservationId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptObservationId(a,input->observation_id,value);
        out->observation_id=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptObservationId(arena *a,const thinkthen_complete_observation_id_v1 *input,
thinkthen_cobol_ReceiptObservationId *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptBatchWarningPresence(arena *a,const thinkthen_complete_batch_warning_presence_v1 *input,
thinkthen_cobol_ReceiptBatchWarningPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_ReceiptBatchWarning *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptBatchWarning(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptBatchWarning(arena *a,const thinkthen_complete_batch_warning_v1 *input,
thinkthen_cobol_ReceiptBatchWarning *out) {
    int code=0;
    (void)a;
    if(input->running) {
        thinkthen_cobol_ReceiptBatchSetting *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptBatchSetting(a,input->running,value);
        out->running=value;
    }
    if(input->tuned_for) {
        thinkthen_cobol_ReceiptBatchSetting *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptBatchSetting(a,input->tuned_for,value);
        out->tuned_for=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptBatchSettingData(arena *a,const thinkthen_complete_batch_setting_data_v1 *input,
thinkthen_cobol_ReceiptBatchSettingData *out,uint32_t kind) {
    int code=0;
    (void)a;
    if(kind==THINKTHEN_COMPLETE_BATCH_SETTING_INTEGER_V1) {
        out->integer=input->integer;
    }
    if(kind==THINKTHEN_COMPLETE_BATCH_SETTING_STRING_V1) {
        if(!code) code=answer_text(input->string,&out->string);
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptBatchSettingPresence(arena *a,const thinkthen_complete_batch_setting_presence_v1 *input,
thinkthen_cobol_ReceiptBatchSettingPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_ReceiptBatchSetting *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptBatchSetting(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAttemptList(arena *a,const thinkthen_complete_attempt_list_v1 *input,
thinkthen_cobol_ReceiptAttemptList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptAttempt *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptAttempt(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAttempt(arena *a,const thinkthen_complete_attempt_v1 *input,
thinkthen_cobol_ReceiptAttempt *out) {
    int code=0;
    (void)a;
    out->ordinal=input->ordinal;
    if(input->outcome) {
        thinkthen_cobol_ReceiptAttemptOutcome *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAttemptOutcome(a,input->outcome,value);
        out->outcome=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->request_id,&out->request_id);
    if(!code) code=answer_text(input->request_sha256,&out->request_sha256);
    if(input->sdk_request_id) {
        thinkthen_cobol_ReceiptSdkRequestId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptSdkRequestId(a,input->sdk_request_id,value);
        out->sdk_request_id=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->server_ms,&out->server_ms);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->status,&out->status);
    out->wall_ms=input->wall_ms;
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptSdkRequestId(arena *a,const thinkthen_complete_sdk_request_id_v1 *input,
thinkthen_cobol_ReceiptSdkRequestId *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAttemptOutcome(arena *a,const thinkthen_complete_attempt_outcome_v1 *input,
thinkthen_cobol_ReceiptAttemptOutcome *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptSessionInputSourceList(arena *a,const thinkthen_complete_session_input_source_list_v1 *input,
thinkthen_cobol_ReceiptSessionInputSourceList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptSessionInputSource *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptSessionInputSource(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptSessionInputSource(arena *a,const thinkthen_complete_session_input_source_v1 *input,
thinkthen_cobol_ReceiptSessionInputSource *out) {
    int code=0;
    (void)a;
    out->index=input->index;
    if(input->source) {
        thinkthen_cobol_ReceiptPhysicalSource *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptPhysicalSource(a,input->source,value);
        out->source=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptPhysicalSource(arena *a,const thinkthen_complete_physical_source_v1 *input,
thinkthen_cobol_ReceiptPhysicalSource *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->file,&out->file);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->first_line,&out->first_line);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->last_line,&out->last_line);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnswerId(arena *a,const thinkthen_complete_answer_id_v1 *input,
thinkthen_cobol_ReceiptAnswerId *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelationMemberList(arena *a,const thinkthen_complete_relation_member_list_v1 *input,
thinkthen_cobol_ReceiptRelationMemberList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptRelationMember *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptRelationMember(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelationMemberData(arena *a,const thinkthen_complete_relation_member_data_v1 *input,
thinkthen_cobol_ReceiptRelationMemberData *out,uint32_t kind) {
    int code=0;
    (void)a;
    if(kind==THINKTHEN_COMPLETE_RELATION_MEMBER_ANSWER_ID_V1) {
        if(input->answer_id) {
            thinkthen_cobol_ReceiptRelationMemberAnswerId *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptRelationMemberAnswerId(a,input->answer_id,value);
            out->answer_id=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_RELATION_MEMBER_FAILURE_ID_V1) {
        if(input->failure_id) {
            thinkthen_cobol_ReceiptRelationMemberFailureId *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptRelationMemberFailureId(a,input->failure_id,value);
            out->failure_id=value;
        }
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptFailure(arena *a,const thinkthen_complete_failure_v1 *input,
thinkthen_cobol_ReceiptFailure *out) {
    int code=0;
    (void)a;
    if(input->cause) {
        thinkthen_cobol_ReceiptFailureCause *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptFailureCause(a,input->cause,value);
        out->cause=value;
    }
    if(input->kind) {
        thinkthen_cobol_ReceiptFailureFieldKind *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptFailureFieldKind(a,input->kind,value);
        out->kind=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptFailureFieldKind(arena *a,const thinkthen_complete_failure_field_kind_v1 *input,
thinkthen_cobol_ReceiptFailureFieldKind *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptFailureCause(arena *a,const thinkthen_complete_failure_cause_v1 *input,
thinkthen_cobol_ReceiptFailureCause *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelatedEntityPresence(arena *a,const thinkthen_complete_related_entity_presence_v1 *input,
thinkthen_cobol_ReceiptRelatedEntityPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_ReceiptRelatedEntity *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptRelatedEntity(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelatedEntity(arena *a,const thinkthen_complete_related_entity_v1 *input,
thinkthen_cobol_ReceiptRelatedEntity *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->kind,&out->kind);
    if(!code) code=answer_text(input->name,&out->name);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptQuestionDocumentData(arena *a,const thinkthen_complete_question_document_data_v1 *input,
thinkthen_cobol_ReceiptQuestionDocumentData *out,uint32_t kind) {
    int code=0;
    (void)a;
    if(kind==THINKTHEN_COMPLETE_QUESTION_DOCUMENT_DECIDE_V1) {
        if(input->decide) {
            thinkthen_cobol_ReceiptQuestionDocumentDecide *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptQuestionDocumentDecide(a,input->decide,value);
            out->decide=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_QUESTION_DOCUMENT_CHOOSE_V1) {
        if(input->choose) {
            thinkthen_cobol_ReceiptQuestionDocumentChoose *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptQuestionDocumentChoose(a,input->choose,value);
            out->choose=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_QUESTION_DOCUMENT_TAG_V1) {
        if(input->tag) {
            thinkthen_cobol_ReceiptQuestionDocumentTag *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptQuestionDocumentTag(a,input->tag,value);
            out->tag=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_QUESTION_DOCUMENT_SCORE_V1) {
        if(input->score) {
            thinkthen_cobol_ReceiptQuestionDocumentScore *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptQuestionDocumentScore(a,input->score,value);
            out->score=value;
        }
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelationMethod(arena *a,const thinkthen_complete_relation_method_v1 *input,
thinkthen_cobol_ReceiptRelationMethod *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelationDirection(arena *a,const thinkthen_complete_relation_direction_v1 *input,
thinkthen_cobol_ReceiptRelationDirection *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelationMemberAnswerId(arena *a,const thinkthen_complete_relation_member_answer_id_v1 *input,
thinkthen_cobol_ReceiptRelationMemberAnswerId *out) {
    int code=0;
    (void)a;
    if(input->direction) {
        thinkthen_cobol_ReceiptRelationDirection *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptRelationDirection(a,input->direction,value);
        out->direction=value;
    }
    if(input->method) {
        thinkthen_cobol_ReceiptRelationMethod *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptRelationMethod(a,input->method,value);
        out->method=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptObservationList(a,&input->observations,&out->observations);
    if(input->question) {
        thinkthen_cobol_ReceiptQuestionDocument *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptQuestionDocument(a,input->question,value);
        out->question=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptQuestionSourceList(a,&input->question_sources,&out->question_sources);
    if(!code) code=answer_text(input->reads,&out->reads);
    if(!code) code=answer_text(input->relation,&out->relation);
    if(!code) code=answer_text(input->request,&out->request);
    if(input->source) {
        thinkthen_cobol_ReceiptRelatedEntity *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptRelatedEntity(a,input->source,value);
        out->source=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptRelatedEntityPresence(a,&input->target,&out->target);
    if(input->threshold) {
        thinkthen_cobol_ReceiptThreshold *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptThreshold(a,input->threshold,value);
        out->threshold=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptUsagePresence(a,&input->usage,&out->usage);
    out->accepted=input->accepted;
    if(input->answer) {
        thinkthen_cobol_ReceiptAnswer *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswer(a,input->answer,value);
        out->answer=value;
    }
    if(input->answer_id) {
        thinkthen_cobol_ReceiptAnswerId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswerId(a,input->answer_id,value);
        out->answer_id=value;
    }
    out->probability=input->probability;
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnswerData(arena *a,const thinkthen_complete_answer_data_v1 *input,
thinkthen_cobol_ReceiptAnswerData *out,uint32_t kind) {
    int code=0;
    (void)a;
    if(kind==THINKTHEN_COMPLETE_ANSWER_YES_NO_V1) {
        if(input->yes_no) {
            thinkthen_cobol_ReceiptAnswerYesNo *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptAnswerYesNo(a,input->yes_no,value);
            out->yes_no=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_ANSWER_CHOICE_V1) {
        if(input->choice) {
            thinkthen_cobol_ReceiptAnswerChoice *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptAnswerChoice(a,input->choice,value);
            out->choice=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_ANSWER_TAG_V1) {
        if(input->tag) {
            thinkthen_cobol_ReceiptAnswerTag *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptAnswerTag(a,input->tag,value);
            out->tag=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_ANSWER_SCORE_V1) {
        if(input->score) {
            thinkthen_cobol_ReceiptAnswerScore *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptAnswerScore(a,input->score,value);
            out->score=value;
        }
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptNumberMap(arena *a,const thinkthen_complete_number_map_v1 *input,
thinkthen_cobol_ReceiptNumberMap *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptNumberMapEntry *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptNumberMapEntry(a,&input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptNumberMapEntry(arena *a,const thinkthen_complete_number_map_entry_v1 *input,
thinkthen_cobol_ReceiptNumberMapEntry *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->name,&out->name);
    out->value=input->value;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptF64Presence(arena *a,const thinkthen_complete_f64_presence_v1 *input,
thinkthen_cobol_ReceiptF64Presence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    out->value=input->value;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnswerTag(arena *a,const thinkthen_complete_answer_tag_v1 *input,
thinkthen_cobol_ReceiptAnswerTag *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->kind,&out->kind);
    if(!code) code=copy_thinkthen_cobol_ReceiptNumberMap(a,&input->probabilities,&out->probabilities);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnswerChoice(arena *a,const thinkthen_complete_answer_choice_v1 *input,
thinkthen_cobol_ReceiptAnswerChoice *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptF64Presence(a,&input->confidence,&out->confidence);
    if(!code) code=answer_text(input->kind,&out->kind);
    if(!code) code=answer_text(input->pick,&out->pick);
    if(!code) code=copy_thinkthen_cobol_ReceiptNumberMap(a,&input->probabilities,&out->probabilities);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnswerYesNo(arena *a,const thinkthen_complete_answer_yes_no_v1 *input,
thinkthen_cobol_ReceiptAnswerYesNo *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->kind,&out->kind);
    out->probability=input->probability;
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRecognitionList(arena *a,const thinkthen_complete_recognition_list_v1 *input,
thinkthen_cobol_ReceiptRecognitionList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptRecognition *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptRecognition(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRecognizeData(arena *a,const thinkthen_complete_recognize_data_v1 *input,
thinkthen_cobol_ReceiptRecognizeData *out,uint32_t kind) {
    int code=0;
    (void)a;
    if(kind==THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES_V1) {
        if(input->entities) {
            thinkthen_cobol_ReceiptRecognizeEntities *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptRecognizeEntities(a,input->entities,value);
            out->entities=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_RECOGNIZE_MODE_PROPOSALS_V1) {
        if(input->mode_proposals) {
            thinkthen_cobol_ReceiptRecognizeModeProposals *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptRecognizeModeProposals(a,input->mode_proposals,value);
            out->mode_proposals=value;
        }
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptBoundaryProposalList(arena *a,const thinkthen_complete_boundary_proposal_list_v1 *input,
thinkthen_cobol_ReceiptBoundaryProposalList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptBoundaryProposal *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptBoundaryProposal(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptBoundaryProposal(arena *a,const thinkthen_complete_boundary_proposal_v1 *input,
thinkthen_cobol_ReceiptBoundaryProposal *out) {
    int code=0;
    (void)a;
    out->end=input->end;
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->file,&out->file);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->first_line,&out->first_line);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->last_line,&out->last_line);
    out->length=input->length;
    out->probability=input->probability;
    out->start=input->start;
    if(!code) code=answer_text(input->text,&out->text);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptBoundaryMode(arena *a,const thinkthen_complete_boundary_mode_v1 *input,
thinkthen_cobol_ReceiptBoundaryMode *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptEntityEdgeList(arena *a,const thinkthen_complete_entity_edge_list_v1 *input,
thinkthen_cobol_ReceiptEntityEdgeList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptEntityEdge *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptEntityEdge(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptEntityEdge(arena *a,const thinkthen_complete_entity_edge_v1 *input,
thinkthen_cobol_ReceiptEntityEdge *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptU32Presence(a,&input->either,&out->either);
    out->probability=input->probability;
    if(!code) code=answer_text(input->relation,&out->relation);
    if(input->source) {
        thinkthen_cobol_ReceiptEntity *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptEntity(a,input->source,value);
        out->source=value;
    }
    if(input->target) {
        thinkthen_cobol_ReceiptEntity *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptEntity(a,input->target,value);
        out->target=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptEntity(arena *a,const thinkthen_complete_entity_v1 *input,
thinkthen_cobol_ReceiptEntity *out) {
    int code=0;
    (void)a;
    out->end=input->end;
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->file,&out->file);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->first_line,&out->first_line);
    if(!code) code=answer_text(input->kind,&out->kind);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->last_line,&out->last_line);
    out->length=input->length;
    out->start=input->start;
    out->strength=input->strength;
    if(!code) code=answer_text(input->text,&out->text);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptEntityList(arena *a,const thinkthen_complete_entity_list_v1 *input,
thinkthen_cobol_ReceiptEntityList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptEntity *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptEntity(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptPhysicalSourcePresence(arena *a,const thinkthen_complete_physical_source_presence_v1 *input,
thinkthen_cobol_ReceiptPhysicalSourcePresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_ReceiptPhysicalSource *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptPhysicalSource(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptVerb(arena *a,const thinkthen_complete_verb_v1 *input,
thinkthen_cobol_ReceiptVerb *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRecognitionStageContextPresence(arena *a,const thinkthen_complete_recognition_stage_context_presence_v1 *input,
thinkthen_cobol_ReceiptRecognitionStageContextPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_ReceiptRecognitionStageContext *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptRecognitionStageContext(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRecognitionStageContext(arena *a,const thinkthen_complete_recognition_stage_context_v1 *input,
thinkthen_cobol_ReceiptRecognitionStageContext *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->boundary,&out->boundary);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->kind_edge,&out->kind_edge);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->relation,&out->relation);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelationRuleListPresence(arena *a,const thinkthen_complete_relation_rule_list_presence_v1 *input,
thinkthen_cobol_ReceiptRelationRuleListPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(!code) code=copy_thinkthen_cobol_ReceiptRelationRuleList(a,&input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptThresholdPresence(arena *a,const thinkthen_complete_threshold_presence_v1 *input,
thinkthen_cobol_ReceiptThresholdPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_ReceiptThreshold *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptThreshold(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRecognitionModePresence(arena *a,const thinkthen_complete_recognition_mode_presence_v1 *input,
thinkthen_cobol_ReceiptRecognitionModePresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_ReceiptRecognitionMode *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptRecognitionMode(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRecognitionMode(arena *a,const thinkthen_complete_recognition_mode_v1 *input,
thinkthen_cobol_ReceiptRecognitionMode *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptJsonMap(arena *a,const thinkthen_complete_json_map_v1 *input,
thinkthen_cobol_ReceiptJsonMap *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptJsonMapEntry *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptJsonMapEntry(a,&input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptJsonMapEntry(arena *a,const thinkthen_complete_json_map_entry_v1 *input,
thinkthen_cobol_ReceiptJsonMapEntry *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->name,&out->name);
    if(input->value) out->value=answer_original(a,input->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRecognitionOddsData(arena *a,const thinkthen_complete_recognition_odds_data_v1 *input,
thinkthen_cobol_ReceiptRecognitionOddsData *out,uint32_t kind) {
    int code=0;
    (void)a;
    if(kind==THINKTHEN_COMPLETE_RECOGNITION_ODDS_RECOGNIZE_ANSWER_V1) {
        if(input->recognize_answer) {
            thinkthen_cobol_ReceiptRecognitionOddsRecognizeAnswer *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptRecognitionOddsRecognizeAnswer(a,input->recognize_answer,
            value);
            out->recognize_answer=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_RECOGNITION_ODDS_BOUNDARY_ODDS_V1) {
        if(input->boundary_odds) {
            thinkthen_cobol_ReceiptRecognitionOddsBoundaryOdds *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptRecognitionOddsBoundaryOdds(a,input->boundary_odds,
            value);
            out->boundary_odds=value;
        }
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptPieceOddsList(arena *a,const thinkthen_complete_piece_odds_list_v1 *input,
thinkthen_cobol_ReceiptPieceOddsList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptPieceOdds *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptPieceOdds(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptPieceOdds(arena *a,const thinkthen_complete_piece_odds_v1 *input,
thinkthen_cobol_ReceiptPieceOdds *out) {
    int code=0;
    (void)a;
    out->end=input->end;
    out->start=input->start;
    if(!code) code=copy_thinkthen_cobol_ReceiptNumberMap(a,&input->tags,&out->tags);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRecognitionProposalList(arena *a,const thinkthen_complete_recognition_proposal_list_v1 *input,
thinkthen_cobol_ReceiptRecognitionProposalList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptRecognitionProposal *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptRecognitionProposal(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptPlacePresence(arena *a,const thinkthen_complete_place_presence_v1 *input,
thinkthen_cobol_ReceiptPlacePresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_ReceiptPlace *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptPlace(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptPlace(arena *a,const thinkthen_complete_place_v1 *input,
thinkthen_cobol_ReceiptPlace *out) {
    int code=0;
    (void)a;
    out->end=input->end;
    out->start=input->start;
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptPairOddsList(arena *a,const thinkthen_complete_pair_odds_list_v1 *input,
thinkthen_cobol_ReceiptPairOddsList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptPairOdds *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptPairOdds(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptPairOdds(arena *a,const thinkthen_complete_pair_odds_v1 *input,
thinkthen_cobol_ReceiptPairOdds *out) {
    int code=0;
    (void)a;
    out->probability=input->probability;
    if(!code) code=answer_text(input->relation,&out->relation);
    if(input->source) {
        thinkthen_cobol_ReceiptPlace *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptPlace(a,input->source,value);
        out->source=value;
    }
    if(input->target) {
        thinkthen_cobol_ReceiptPlace *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptPlace(a,input->target,value);
        out->target=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptNameOddsList(arena *a,const thinkthen_complete_name_odds_list_v1 *input,
thinkthen_cobol_ReceiptNameOddsList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptNameOdds *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptNameOdds(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptNumberMapPresence(arena *a,const thinkthen_complete_number_map_presence_v1 *input,
thinkthen_cobol_ReceiptNumberMapPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(!code) code=copy_thinkthen_cobol_ReceiptNumberMap(a,&input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnnotationList(arena *a,const thinkthen_complete_annotation_list_v1 *input,
thinkthen_cobol_ReceiptAnnotationList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptAnnotation *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptAnnotation(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnnotatedFieldMap(arena *a,const thinkthen_complete_annotated_field_map_v1 *input,
thinkthen_cobol_ReceiptAnnotatedFieldMap *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptAnnotatedFieldMapEntry *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptAnnotatedFieldMapEntry(a,&input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnnotatedFieldMapEntry(arena *a,const thinkthen_complete_annotated_field_map_entry_v1 *input,
thinkthen_cobol_ReceiptAnnotatedFieldMapEntry *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->name,&out->name);
    if(input->value) {
        thinkthen_cobol_ReceiptAnnotatedField *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnnotatedField(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnnotatedFieldData(arena *a,const thinkthen_complete_annotated_field_data_v1 *input,
thinkthen_cobol_ReceiptAnnotatedFieldData *out,uint32_t kind) {
    int code=0;
    (void)a;
    if(kind==THINKTHEN_COMPLETE_ANNOTATED_FIELD_BOOLEAN_V1) {
        out->boolean=input->boolean;
    }
    if(kind==THINKTHEN_COMPLETE_ANNOTATED_FIELD_NULL_V1) {
        out->null=input->null;
    }
    if(kind==THINKTHEN_COMPLETE_ANNOTATED_FIELD_STRING_V1) {
        if(!code) code=answer_text(input->string,&out->string);
    }
    if(kind==THINKTHEN_COMPLETE_ANNOTATED_FIELD_ARRAY_V1) {
        if(!code) code=copy_thinkthen_cobol_ReceiptStringList(a,&input->array,&out->array);
    }
    if(kind==THINKTHEN_COMPLETE_ANNOTATED_FIELD_NUMBER_V1) {
        out->number=input->number;
    }
    if(kind==THINKTHEN_COMPLETE_ANNOTATED_FIELD_OBJECT_V1) {
        if(input->object) {
            thinkthen_cobol_ReceiptFailed *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptFailed(a,input->object,value);
            out->object=value;
        }
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptFailed(arena *a,const thinkthen_complete_failed_v1 *input,
thinkthen_cobol_ReceiptFailed *out) {
    int code=0;
    (void)a;
    if(input->failed) {
        thinkthen_cobol_ReceiptFailure *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptFailure(a,input->failed,value);
        out->failed=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnnotationMemberMap(arena *a,const thinkthen_complete_annotation_member_map_v1 *input,
thinkthen_cobol_ReceiptAnnotationMemberMap *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptAnnotationMemberMapEntry *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptAnnotationMemberMapEntry(a,&input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnnotationMemberMapEntry(arena *a,const thinkthen_complete_annotation_member_map_entry_v1 *input,
thinkthen_cobol_ReceiptAnnotationMemberMapEntry *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->name,&out->name);
    if(input->value) {
        thinkthen_cobol_ReceiptAnnotationMember *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnnotationMember(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnnotationMemberData(arena *a,const thinkthen_complete_annotation_member_data_v1 *input,
thinkthen_cobol_ReceiptAnnotationMemberData *out,uint32_t kind) {
    int code=0;
    (void)a;
    if(kind==THINKTHEN_COMPLETE_ANNOTATION_MEMBER_ANSWER_ID_V1) {
        if(input->answer_id) {
            thinkthen_cobol_ReceiptAnnotationMemberAnswerId *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptAnnotationMemberAnswerId(a,input->answer_id,value);
            out->answer_id=value;
        }
    }
    if(kind==THINKTHEN_COMPLETE_ANNOTATION_MEMBER_FAILURE_ID_V1) {
        if(input->failure_id) {
            thinkthen_cobol_ReceiptAnnotationMemberFailureId *value=allocate(a,1,sizeof(*value));
            if(!value) return a->code;
            if(!code) code=copy_thinkthen_cobol_ReceiptAnnotationMemberFailureId(a,input->failure_id,value);
            out->failure_id=value;
        }
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnnotationMemberFailureId(arena *a,const thinkthen_complete_annotation_member_failure_id_v1 *input,
thinkthen_cobol_ReceiptAnnotationMemberFailureId *out) {
    int code=0;
    (void)a;
    if(input->failure) {
        thinkthen_cobol_ReceiptFailure *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptFailure(a,input->failure,value);
        out->failure=value;
    }
    if(input->failure_id) {
        thinkthen_cobol_ReceiptFailureId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptFailureId(a,input->failure_id,value);
        out->failure_id=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptObservationList(a,&input->observations,&out->observations);
    if(input->question) {
        thinkthen_cobol_ReceiptQuestionDocument *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptQuestionDocument(a,input->question,value);
        out->question=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptQuestionSourceList(a,&input->question_sources,&out->question_sources);
    if(!code) code=answer_text(input->request,&out->request);
    if(!code) code=copy_thinkthen_cobol_ReceiptThresholdPresence(a,&input->threshold,&out->threshold);
    if(!code) code=copy_thinkthen_cobol_ReceiptUsagePresence(a,&input->usage,&out->usage);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptValuePresence(arena *a,const thinkthen_complete_value_presence_v1 *input,
thinkthen_cobol_ReceiptValuePresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_ReceiptValue *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptValue(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptValueData(arena *a,const thinkthen_complete_value_data_v1 *input,
thinkthen_cobol_ReceiptValueData *out,uint32_t kind) {
    int code=0;
    (void)a;
    if(kind==THINKTHEN_COMPLETE_VALUE_BOOLEAN_V1) {
        out->boolean=input->boolean;
    }
    if(kind==THINKTHEN_COMPLETE_VALUE_NULL_V1) {
        out->null=input->null;
    }
    if(kind==THINKTHEN_COMPLETE_VALUE_STRING_V1) {
        if(!code) code=answer_text(input->string,&out->string);
    }
    if(kind==THINKTHEN_COMPLETE_VALUE_ARRAY_V1) {
        if(!code) code=copy_thinkthen_cobol_ReceiptStringList(a,&input->array,&out->array);
    }
    if(kind==THINKTHEN_COMPLETE_VALUE_NUMBER_V1) {
        out->number=input->number;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptFindCandidateList(arena *a,const thinkthen_complete_find_candidate_list_v1 *input,
thinkthen_cobol_ReceiptFindCandidateList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptFindCandidate *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptFindCandidate(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptFindCandidate(arena *a,const thinkthen_complete_find_candidate_v1 *input,
thinkthen_cobol_ReceiptFindCandidate *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->index,&out->index);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->input,&out->input);
    out->probability=input->probability;
    if(!code) code=copy_thinkthen_cobol_ReceiptPhysicalSourcePresence(a,&input->source,&out->source);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptFindAnswer(arena *a,const thinkthen_complete_find_answer_v1 *input,
thinkthen_cobol_ReceiptFindAnswer *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptF64Presence(a,&input->confidence,&out->confidence);
    if(!code) code=answer_text(input->kind,&out->kind);
    if(!code) code=answer_text(input->pick,&out->pick);
    if(!code) code=copy_thinkthen_cobol_ReceiptNumberMap(a,&input->probabilities,&out->probabilities);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAtomicNonZeroUsizeList(arena *a,const thinkthen_complete_atomic_non_zero_usize_list_v1 *input,
thinkthen_cobol_ReceiptAtomicNonZeroUsizeList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptAtomicNonZeroUsize *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptAtomicNonZeroUsize(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRankMemberList(arena *a,const thinkthen_complete_rank_member_list_v1 *input,
thinkthen_cobol_ReceiptRankMemberList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptRankMember *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptRankMember(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRankMember(arena *a,const thinkthen_complete_rank_member_v1 *input,
thinkthen_cobol_ReceiptRankMember *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->name,&out->name);
    if(input->result) {
        thinkthen_cobol_ReceiptRankMemberResult *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptRankMemberResult(a,input->result,value);
        out->result=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptImageList(arena *a,const thinkthen_complete_image_list_v1 *input,
thinkthen_cobol_ReceiptImageList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptImage *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptImage(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptImage(arena *a,const thinkthen_complete_image_v1 *input,
thinkthen_cobol_ReceiptImage *out) {
    int code=0;
    (void)a;
    if(!code) code=answer_text(input->base64,&out->base64);
    out->height=input->height;
    if(input->media) {
        thinkthen_cobol_ReceiptImageMedia *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptImageMedia(a,input->media,value);
        out->media=value;
    }
    out->width=input->width;
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptImageMedia(arena *a,const thinkthen_complete_image_media_v1 *input,
thinkthen_cobol_ReceiptImageMedia *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAtomicBooleanList(arena *a,const thinkthen_complete_atomic_boolean_list_v1 *input,
thinkthen_cobol_ReceiptAtomicBooleanList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptAtomicBoolean *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptAtomicBoolean(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAtomicDoubleList(arena *a,const thinkthen_complete_atomic_double_list_v1 *input,
thinkthen_cobol_ReceiptAtomicDoubleList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptAtomicDouble *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptAtomicDouble(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAtomicArrayOfStringList(arena *a,const thinkthen_complete_atomic_array_of_string_list_v1 *input,
thinkthen_cobol_ReceiptAtomicArrayOfStringList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptAtomicArrayOfString *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptAtomicArrayOfString(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAtomicNullableStringList(arena *a,const thinkthen_complete_atomic_nullable_string_list_v1 *input,
thinkthen_cobol_ReceiptAtomicNullableStringList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptAtomicNullableString *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptAtomicNullableString(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAtomicDecideValueList(arena *a,const thinkthen_complete_atomic_decide_value_list_v1 *input,
thinkthen_cobol_ReceiptAtomicDecideValueList *out) {
    int code=0;
    (void)a;
    thinkthen_cobol_ReceiptAtomicDecideValue *items=allocate(a,input->len,sizeof(*items));
    out->data=items;
    out->len=input->len;
    if(!items) return a->code;
    for(size_t i=0;
    i<input->len && !code;
    i++) {
        code=copy_thinkthen_cobol_ReceiptAtomicDecideValue(a,input->data[i],items+i);
    }
    out->len=input->len;
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptDecideValuePresence(arena *a,const thinkthen_complete_decide_value_presence_v1 *input,
thinkthen_cobol_ReceiptDecideValuePresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(input->value) {
        thinkthen_cobol_ReceiptDecideValue *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptDecideValue(a,input->value,value);
        out->value=value;
    }
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptDecideValue(arena *a,const thinkthen_complete_decide_value_v1 *input,
thinkthen_cobol_ReceiptDecideValue *out) {
    int code=0;
    (void)a;
    if(input->value) out->value=answer_original(a,input->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelatedEntityEdge(arena *a,const thinkthen_complete_related_entity_edge_v1 *input,
thinkthen_cobol_ReceiptRelatedEntityEdge *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptU32Presence(a,&input->either,&out->either);
    out->probability=input->probability;
    if(!code) code=answer_text(input->relation,&out->relation);
    if(input->source) {
        thinkthen_cobol_ReceiptRelationEndpoint *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptRelationEndpoint(a,input->source,value);
        out->source=value;
    }
    if(input->target) {
        thinkthen_cobol_ReceiptRelationEndpoint *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptRelationEndpoint(a,input->target,value);
        out->target=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelationEndpoint(arena *a,const thinkthen_complete_relation_endpoint_v1 *input,
thinkthen_cobol_ReceiptRelationEndpoint *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    if(!code) code=copy_thinkthen_cobol_ReceiptRelationEndpointData(a,&input->data,&out->data,input->kind);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelationEndpointSource(arena *a,const thinkthen_complete_relation_endpoint_source_v1 *input,
thinkthen_cobol_ReceiptRelationEndpointSource *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->file,&out->file);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->first_line,&out->first_line);
    if(!code) code=answer_text(input->kind,&out->kind);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->last_line,&out->last_line);
    if(!code) code=answer_text(input->name,&out->name);
    out->ordinal=input->ordinal;
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->record,&out->record);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptThreshold(arena *a,const thinkthen_complete_threshold_v1 *input,
thinkthen_cobol_ReceiptThreshold *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    if(!code) code=copy_thinkthen_cobol_ReceiptThresholdData(a,&input->data,&out->data,input->kind);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptStringListPresence(arena *a,const thinkthen_complete_string_list_presence_v1 *input,
thinkthen_cobol_ReceiptStringListPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(!code) code=copy_thinkthen_cobol_ReceiptStringList(a,&input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptLabelListPresence(arena *a,const thinkthen_complete_label_list_presence_v1 *input,
thinkthen_cobol_ReceiptLabelListPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(!code) code=copy_thinkthen_cobol_ReceiptLabelList(a,&input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptInputDeclaration(arena *a,const thinkthen_complete_input_declaration_v1 *input,
thinkthen_cobol_ReceiptInputDeclaration *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    if(!code) code=copy_thinkthen_cobol_ReceiptInputDeclarationData(a,&input->data,&out->data,input->kind);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptInputDeclarationObjectRoot(arena *a,const thinkthen_complete_input_declaration_object_root_v1 *input,
thinkthen_cobol_ReceiptInputDeclarationObjectRoot *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptInputPropertyTypeMap(a,&input->properties,&out->properties);
    if(!code) code=copy_thinkthen_cobol_ReceiptStringListPresence(a,&input->required,&out->required);
    if(input->type) {
        thinkthen_cobol_ReceiptObjectType *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptObjectType(a,input->type,value);
        out->type=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptInputPropertyType(arena *a,const thinkthen_complete_input_property_type_v1 *input,
thinkthen_cobol_ReceiptInputPropertyType *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    if(!code) code=copy_thinkthen_cobol_ReceiptInputPropertyTypeData(a,&input->data,&out->data,
    input->kind);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptBatch(arena *a,const thinkthen_complete_batch_v1 *input,
thinkthen_cobol_ReceiptBatch *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    if(!code) code=copy_thinkthen_cobol_ReceiptBatchData(a,&input->data,&out->data,input->kind);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptPosition(arena *a,const thinkthen_complete_position_v1 *input,
thinkthen_cobol_ReceiptPosition *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->file,&out->file);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->first,&out->first);
    if(!code) code=copy_thinkthen_cobol_ReceiptStringListPresence(a,&input->images,&out->images);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->last,&out->last);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptObservation(arena *a,const thinkthen_complete_observation_v1 *input,
thinkthen_cobol_ReceiptObservation *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    if(!code) code=copy_thinkthen_cobol_ReceiptObservationData(a,&input->data,&out->data,input->kind);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptBatchSetting(arena *a,const thinkthen_complete_batch_setting_v1 *input,
thinkthen_cobol_ReceiptBatchSetting *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    if(!code) code=copy_thinkthen_cobol_ReceiptBatchSettingData(a,&input->data,&out->data,input->kind);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAttemptListPresence(arena *a,const thinkthen_complete_attempt_list_presence_v1 *input,
thinkthen_cobol_ReceiptAttemptListPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(!code) code=copy_thinkthen_cobol_ReceiptAttemptList(a,&input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptSessionInputSourceListPresence(arena *a,const thinkthen_complete_session_input_source_list_presence_v1 *input,
thinkthen_cobol_ReceiptSessionInputSourceListPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(!code) code=copy_thinkthen_cobol_ReceiptSessionInputSourceList(a,&input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnswers(arena *a,const thinkthen_complete_answers_v1 *input,
thinkthen_cobol_ReceiptAnswers *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptRelationMemberList(a,&input->questions,&out->questions);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelationMember(arena *a,const thinkthen_complete_relation_member_v1 *input,
thinkthen_cobol_ReceiptRelationMember *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    if(!code) code=copy_thinkthen_cobol_ReceiptRelationMemberData(a,&input->data,&out->data,input->kind);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelationMemberFailureId(arena *a,const thinkthen_complete_relation_member_failure_id_v1 *input,
thinkthen_cobol_ReceiptRelationMemberFailureId *out) {
    int code=0;
    (void)a;
    if(input->direction) {
        thinkthen_cobol_ReceiptRelationDirection *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptRelationDirection(a,input->direction,value);
        out->direction=value;
    }
    if(input->method) {
        thinkthen_cobol_ReceiptRelationMethod *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptRelationMethod(a,input->method,value);
        out->method=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptObservationList(a,&input->observations,&out->observations);
    if(input->question) {
        thinkthen_cobol_ReceiptQuestionDocument *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptQuestionDocument(a,input->question,value);
        out->question=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptQuestionSourceList(a,&input->question_sources,&out->question_sources);
    if(!code) code=answer_text(input->reads,&out->reads);
    if(!code) code=answer_text(input->relation,&out->relation);
    if(!code) code=answer_text(input->request,&out->request);
    if(input->source) {
        thinkthen_cobol_ReceiptRelatedEntity *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptRelatedEntity(a,input->source,value);
        out->source=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptRelatedEntityPresence(a,&input->target,&out->target);
    if(input->threshold) {
        thinkthen_cobol_ReceiptThreshold *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptThreshold(a,input->threshold,value);
        out->threshold=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptUsagePresence(a,&input->usage,&out->usage);
    if(input->failure) {
        thinkthen_cobol_ReceiptFailure *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptFailure(a,input->failure,value);
        out->failure=value;
    }
    if(input->failure_id) {
        thinkthen_cobol_ReceiptFailureId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptFailureId(a,input->failure_id,value);
        out->failure_id=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptQuestionDocument(arena *a,const thinkthen_complete_question_document_v1 *input,
thinkthen_cobol_ReceiptQuestionDocument *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    if(!code) code=copy_thinkthen_cobol_ReceiptQuestionDocumentData(a,&input->data,&out->data,input->kind);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptQuestionDocumentScore(arena *a,const thinkthen_complete_question_document_score_v1 *input,
thinkthen_cobol_ReceiptQuestionDocumentScore *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptBatchPresence(a,&input->batch,&out->batch);
    if(!code) code=copy_thinkthen_cobol_ReceiptInputDeclarationPresence(a,&input->context_schema,
    &out->context_schema);
    if(!code) code=copy_thinkthen_cobol_ReceiptInputDeclarationPresence(a,&input->item_schema,&out->item_schema);
    if(!code) code=copy_thinkthen_cobol_ReceiptLabelListPresence(a,&input->label_details,&out->label_details);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->model,&out->model);
    if(!code) code=copy_thinkthen_cobol_ReceiptQuestionNamePresence(a,&input->name,&out->name);
    if(!code) code=copy_thinkthen_cobol_ReceiptStringListPresence(a,&input->on,&out->on);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->profile,&out->profile);
    if(!code) code=copy_thinkthen_cobol_ReceiptWordingVersionPresence(a,&input->wording_version,
    &out->wording_version);
    if(!code) code=copy_thinkthen_cobol_ReceiptStringList(a,&input->levels,&out->levels);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->text,&out->text);
    if(!code) code=answer_text(input->verb,&out->verb);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptQuestionDocumentTag(arena *a,const thinkthen_complete_question_document_tag_v1 *input,
thinkthen_cobol_ReceiptQuestionDocumentTag *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptBatchPresence(a,&input->batch,&out->batch);
    if(!code) code=copy_thinkthen_cobol_ReceiptInputDeclarationPresence(a,&input->context_schema,
    &out->context_schema);
    if(!code) code=copy_thinkthen_cobol_ReceiptInputDeclarationPresence(a,&input->item_schema,&out->item_schema);
    if(!code) code=copy_thinkthen_cobol_ReceiptLabelListPresence(a,&input->label_details,&out->label_details);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->model,&out->model);
    if(!code) code=copy_thinkthen_cobol_ReceiptQuestionNamePresence(a,&input->name,&out->name);
    if(!code) code=copy_thinkthen_cobol_ReceiptStringListPresence(a,&input->on,&out->on);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->profile,&out->profile);
    if(!code) code=copy_thinkthen_cobol_ReceiptWordingVersionPresence(a,&input->wording_version,
    &out->wording_version);
    if(!code) code=copy_thinkthen_cobol_ReceiptStringList(a,&input->labels,&out->labels);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->text,&out->text);
    if(!code) code=answer_text(input->verb,&out->verb);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptQuestionDocumentChoose(arena *a,const thinkthen_complete_question_document_choose_v1 *input,
thinkthen_cobol_ReceiptQuestionDocumentChoose *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptBatchPresence(a,&input->batch,&out->batch);
    if(!code) code=copy_thinkthen_cobol_ReceiptInputDeclarationPresence(a,&input->context_schema,
    &out->context_schema);
    if(!code) code=copy_thinkthen_cobol_ReceiptInputDeclarationPresence(a,&input->item_schema,&out->item_schema);
    if(!code) code=copy_thinkthen_cobol_ReceiptLabelListPresence(a,&input->label_details,&out->label_details);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->model,&out->model);
    if(!code) code=copy_thinkthen_cobol_ReceiptQuestionNamePresence(a,&input->name,&out->name);
    if(!code) code=copy_thinkthen_cobol_ReceiptStringListPresence(a,&input->on,&out->on);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->profile,&out->profile);
    if(!code) code=copy_thinkthen_cobol_ReceiptWordingVersionPresence(a,&input->wording_version,
    &out->wording_version);
    if(!code) code=copy_thinkthen_cobol_ReceiptStringList(a,&input->options,&out->options);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->text,&out->text);
    if(!code) code=answer_text(input->verb,&out->verb);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptQuestionDocumentDecide(arena *a,const thinkthen_complete_question_document_decide_v1 *input,
thinkthen_cobol_ReceiptQuestionDocumentDecide *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptBatchPresence(a,&input->batch,&out->batch);
    if(!code) code=copy_thinkthen_cobol_ReceiptInputDeclarationPresence(a,&input->context_schema,
    &out->context_schema);
    if(!code) code=copy_thinkthen_cobol_ReceiptInputDeclarationPresence(a,&input->item_schema,&out->item_schema);
    if(!code) code=copy_thinkthen_cobol_ReceiptLabelListPresence(a,&input->label_details,&out->label_details);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->model,&out->model);
    if(!code) code=copy_thinkthen_cobol_ReceiptQuestionNamePresence(a,&input->name,&out->name);
    if(!code) code=copy_thinkthen_cobol_ReceiptStringListPresence(a,&input->on,&out->on);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->profile,&out->profile);
    if(!code) code=copy_thinkthen_cobol_ReceiptWordingVersionPresence(a,&input->wording_version,
    &out->wording_version);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->false_,&out->false_);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->text,&out->text);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->true_,&out->true_);
    if(!code) code=answer_text(input->verb,&out->verb);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnswer(arena *a,const thinkthen_complete_answer_v1 *input,
thinkthen_cobol_ReceiptAnswer *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    if(!code) code=copy_thinkthen_cobol_ReceiptAnswerData(a,&input->data,&out->data,input->kind);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnswerScore(arena *a,const thinkthen_complete_answer_score_v1 *input,
thinkthen_cobol_ReceiptAnswerScore *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptF64Presence(a,&input->confidence,&out->confidence);
    if(!code) code=answer_text(input->kind,&out->kind);
    if(!code) code=answer_text(input->level,&out->level);
    if(!code) code=copy_thinkthen_cobol_ReceiptNumberMap(a,&input->probabilities,&out->probabilities);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRecognition(arena *a,const thinkthen_complete_recognition_v1 *input,
thinkthen_cobol_ReceiptRecognition *out) {
    int code=0;
    (void)a;
    if(input->answer) {
        thinkthen_cobol_ReceiptRecognitionOdds *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptRecognitionOdds(a,input->answer,value);
        out->answer=value;
    }
    if(input->answer_id) {
        thinkthen_cobol_ReceiptAnswerId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswerId(a,input->answer_id,value);
        out->answer_id=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->file,&out->file);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->first_line,&out->first_line);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->index,&out->index);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->input,&out->input);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->last_line,&out->last_line);
    if(input->meta) {
        thinkthen_cobol_ReceiptMeta *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptMeta(a,input->meta,value);
        out->meta=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptPositionPresence(a,&input->position,&out->position);
    if(input->question) {
        thinkthen_cobol_ReceiptRecognizeQuestionDocument *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptRecognizeQuestionDocument(a,input->question,value);
        out->question=value;
    }
    if(input->schema) {
        thinkthen_cobol_ReceiptVersion *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptVersion(a,input->schema,value);
        out->schema=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptPhysicalSourcePresence(a,&input->source,&out->source);
    if(input->value) {
        thinkthen_cobol_ReceiptRecognize *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptRecognize(a,input->value,value);
        out->value=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRecognize(arena *a,const thinkthen_complete_recognize_v1 *input,
thinkthen_cobol_ReceiptRecognize *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    if(!code) code=copy_thinkthen_cobol_ReceiptRecognizeData(a,&input->data,&out->data,input->kind);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRecognizeModeProposals(arena *a,const thinkthen_complete_recognize_mode_proposals_v1 *input,
thinkthen_cobol_ReceiptRecognizeModeProposals *out) {
    int code=0;
    (void)a;
    if(input->mode) {
        thinkthen_cobol_ReceiptBoundaryMode *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptBoundaryMode(a,input->mode,value);
        out->mode=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptBoundaryProposalList(a,&input->proposals,&out->proposals);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptEntityEdgeListPresence(arena *a,const thinkthen_complete_entity_edge_list_presence_v1 *input,
thinkthen_cobol_ReceiptEntityEdgeListPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(!code) code=copy_thinkthen_cobol_ReceiptEntityEdgeList(a,&input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRecognizeQuestionDocument(arena *a,const thinkthen_complete_recognize_question_document_v1 *input,
thinkthen_cobol_ReceiptRecognizeQuestionDocument *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptBatchPresence(a,&input->batch,&out->batch);
    if(!code) code=copy_thinkthen_cobol_ReceiptInputDeclarationPresence(a,&input->context_schema,
    &out->context_schema);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->entity_definition,&out->entity_definition);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->instructions,&out->instructions);
    if(!code) code=copy_thinkthen_cobol_ReceiptInputDeclarationPresence(a,&input->item_schema,&out->item_schema);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonMap(a,&input->kinds,&out->kinds);
    if(!code) code=copy_thinkthen_cobol_ReceiptLabelListPresence(a,&input->label_details,&out->label_details);
    if(!code) code=copy_thinkthen_cobol_ReceiptRecognitionModePresence(a,&input->mode,&out->mode);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->model,&out->model);
    if(!code) code=copy_thinkthen_cobol_ReceiptQuestionNamePresence(a,&input->name,&out->name);
    if(!code) code=copy_thinkthen_cobol_ReceiptStringListPresence(a,&input->on,&out->on);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->profile,&out->profile);
    if(!code) code=copy_thinkthen_cobol_ReceiptThresholdPresence(a,&input->relation_threshold,&out->relation_threshold);
    if(!code) code=copy_thinkthen_cobol_ReceiptRelationRuleListPresence(a,&input->relations,&out->relations);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->snippet_pieces,&out->snippet_pieces);
    if(!code) code=copy_thinkthen_cobol_ReceiptRecognitionStageContextPresence(a,&input->stage_context,
    &out->stage_context);
    if(input->threshold) {
        thinkthen_cobol_ReceiptThreshold *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptThreshold(a,input->threshold,value);
        out->threshold=value;
    }
    if(input->verb) {
        thinkthen_cobol_ReceiptVerb *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptVerb(a,input->verb,value);
        out->verb=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptWordingVersionPresence(a,&input->wording_version,
    &out->wording_version);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRecognitionOdds(arena *a,const thinkthen_complete_recognition_odds_v1 *input,
thinkthen_cobol_ReceiptRecognitionOdds *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    if(!code) code=copy_thinkthen_cobol_ReceiptRecognitionOddsData(a,&input->data,&out->data,input->kind);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRecognitionOddsBoundaryOdds(arena *a,const thinkthen_complete_recognition_odds_boundary_odds_v1 *input,
thinkthen_cobol_ReceiptRecognitionOddsBoundaryOdds *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptPieceOddsList(a,&input->pieces,&out->pieces);
    if(!code) code=copy_thinkthen_cobol_ReceiptBoundaryProposalList(a,&input->proposals,&out->proposals);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRecognitionOddsRecognizeAnswer(arena *a,const thinkthen_complete_recognition_odds_recognize_answer_v1 *input,
thinkthen_cobol_ReceiptRecognitionOddsRecognizeAnswer *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptNameOddsList(a,&input->names,&out->names);
    if(!code) code=copy_thinkthen_cobol_ReceiptPairOddsList(a,&input->pairs,&out->pairs);
    if(!code) code=copy_thinkthen_cobol_ReceiptPieceOddsList(a,&input->pieces,&out->pieces);
    if(!code) code=copy_thinkthen_cobol_ReceiptRecognitionProposalList(a,&input->proposals,&out->proposals);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRecognitionProposal(arena *a,const thinkthen_complete_recognition_proposal_v1 *input,
thinkthen_cobol_ReceiptRecognitionProposal *out) {
    int code=0;
    (void)a;
    out->end=input->end;
    out->kept=input->kept;
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->kind,&out->kind);
    if(!code) code=copy_thinkthen_cobol_ReceiptPlacePresence(a,&input->selected,&out->selected);
    out->span_probability=input->span_probability;
    out->start=input->start;
    if(!code) code=copy_thinkthen_cobol_ReceiptF64Presence(a,&input->strength,&out->strength);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptNameOdds(arena *a,const thinkthen_complete_name_odds_v1 *input,
thinkthen_cobol_ReceiptNameOdds *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptNumberMapPresence(a,&input->edges,&out->edges);
    out->end=input->end;
    if(!code) code=copy_thinkthen_cobol_ReceiptNumberMapPresence(a,&input->kinds,&out->kinds);
    out->start=input->start;
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnnotation(arena *a,const thinkthen_complete_annotation_v1 *input,
thinkthen_cobol_ReceiptAnnotation *out) {
    int code=0;
    (void)a;
    if(input->answer_id) {
        thinkthen_cobol_ReceiptAnswerId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswerId(a,input->answer_id,value);
        out->answer_id=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptAnnotationMemberMap(a,&input->answers,&out->answers);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->file,&out->file);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->first_line,&out->first_line);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->index,&out->index);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->input,&out->input);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->last_line,&out->last_line);
    if(input->meta) {
        thinkthen_cobol_ReceiptMeta *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptMeta(a,input->meta,value);
        out->meta=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptPositionPresence(a,&input->position,&out->position);
    if(input->schema) {
        thinkthen_cobol_ReceiptVersion *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptVersion(a,input->schema,value);
        out->schema=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptPhysicalSourcePresence(a,&input->source,&out->source);
    if(input->value) {
        thinkthen_cobol_ReceiptAnnotatedRow *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnnotatedRow(a,input->value,value);
        out->value=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnnotatedRow(arena *a,const thinkthen_complete_annotated_row_v1 *input,
thinkthen_cobol_ReceiptAnnotatedRow *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptAnnotatedFieldMap(a,&input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnnotatedField(arena *a,const thinkthen_complete_annotated_field_v1 *input,
thinkthen_cobol_ReceiptAnnotatedField *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    if(!code) code=copy_thinkthen_cobol_ReceiptAnnotatedFieldData(a,&input->data,&out->data,input->kind);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnnotationMember(arena *a,const thinkthen_complete_annotation_member_v1 *input,
thinkthen_cobol_ReceiptAnnotationMember *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    if(!code) code=copy_thinkthen_cobol_ReceiptAnnotationMemberData(a,&input->data,&out->data,input->kind);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAnnotationMemberAnswerId(arena *a,const thinkthen_complete_annotation_member_answer_id_v1 *input,
thinkthen_cobol_ReceiptAnnotationMemberAnswerId *out) {
    int code=0;
    (void)a;
    if(input->answer) {
        thinkthen_cobol_ReceiptAnswer *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswer(a,input->answer,value);
        out->answer=value;
    }
    if(input->answer_id) {
        thinkthen_cobol_ReceiptAnswerId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswerId(a,input->answer_id,value);
        out->answer_id=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptObservationList(a,&input->observations,&out->observations);
    if(input->question) {
        thinkthen_cobol_ReceiptQuestionDocument *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptQuestionDocument(a,input->question,value);
        out->question=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptQuestionSourceList(a,&input->question_sources,&out->question_sources);
    if(!code) code=answer_text(input->request,&out->request);
    if(!code) code=copy_thinkthen_cobol_ReceiptThresholdPresence(a,&input->threshold,&out->threshold);
    if(!code) code=copy_thinkthen_cobol_ReceiptUsagePresence(a,&input->usage,&out->usage);
    if(!code) code=copy_thinkthen_cobol_ReceiptValuePresence(a,&input->value,&out->value);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptValue(arena *a,const thinkthen_complete_value_v1 *input,
thinkthen_cobol_ReceiptValue *out) {
    int code=0;
    (void)a;
    out->kind=input->kind;
    if(!code) code=copy_thinkthen_cobol_ReceiptValueData(a,&input->data,&out->data,input->kind);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptFindQuestionDocument(arena *a,const thinkthen_complete_find_question_document_v1 *input,
thinkthen_cobol_ReceiptFindQuestionDocument *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptBatchPresence(a,&input->batch,&out->batch);
    if(!code) code=copy_thinkthen_cobol_ReceiptInputDeclarationPresence(a,&input->context_schema,
    &out->context_schema);
    if(!code) code=copy_thinkthen_cobol_ReceiptInputDeclarationPresence(a,&input->item_schema,&out->item_schema);
    if(!code) code=copy_thinkthen_cobol_ReceiptLabelListPresence(a,&input->label_details,&out->label_details);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->model,&out->model);
    if(!code) code=copy_thinkthen_cobol_ReceiptQuestionNamePresence(a,&input->name,&out->name);
    out->none=input->none;
    if(!code) code=copy_thinkthen_cobol_ReceiptStringListPresence(a,&input->on,&out->on);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->profile,&out->profile);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->text,&out->text);
    if(!code) code=answer_text(input->verb,&out->verb);
    if(!code) code=copy_thinkthen_cobol_ReceiptWordingVersionPresence(a,&input->wording_version,
    &out->wording_version);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptFindCandidateListPresence(arena *a,const thinkthen_complete_find_candidate_list_presence_v1 *input,
thinkthen_cobol_ReceiptFindCandidateListPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(!code) code=copy_thinkthen_cobol_ReceiptFindCandidateList(a,&input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRankMemberListPresence(arena *a,const thinkthen_complete_rank_member_list_presence_v1 *input,
thinkthen_cobol_ReceiptRankMemberListPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(!code) code=copy_thinkthen_cobol_ReceiptRankMemberList(a,&input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptImageListPresence(arena *a,const thinkthen_complete_image_list_presence_v1 *input,
thinkthen_cobol_ReceiptImageListPresence *out) {
    int code=0;
    (void)a;
    out->presence=input->presence;
    if(!code) code=copy_thinkthen_cobol_ReceiptImageList(a,&input->value,&out->value);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAtomicBoolean(arena *a,const thinkthen_complete_atomic_boolean_v1 *input,
thinkthen_cobol_ReceiptAtomicBoolean *out) {
    int code=0;
    (void)a;
    if(input->answer) {
        thinkthen_cobol_ReceiptAnswer *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswer(a,input->answer,value);
        out->answer=value;
    }
    if(input->answer_id) {
        thinkthen_cobol_ReceiptAnswerId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswerId(a,input->answer_id,value);
        out->answer_id=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptImageListPresence(a,&input->images,&out->images);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->index,&out->index);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->input,&out->input);
    if(!code) code=copy_thinkthen_cobol_ReceiptRankMemberListPresence(a,&input->members,&out->members);
    if(input->meta) {
        thinkthen_cobol_ReceiptMeta *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptMeta(a,input->meta,value);
        out->meta=value;
    }
    if(input->question) {
        thinkthen_cobol_ReceiptQuestionDocument *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptQuestionDocument(a,input->question,value);
        out->question=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->question_name,&out->question_name);
    if(input->schema) {
        thinkthen_cobol_ReceiptVersion *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptVersion(a,input->schema,value);
        out->schema=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptPhysicalSourcePresence(a,&input->source,&out->source);
    if(!code) code=copy_thinkthen_cobol_ReceiptThresholdPresence(a,&input->threshold,&out->threshold);
    out->value=input->value;
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAtomicDouble(arena *a,const thinkthen_complete_atomic_double_v1 *input,
thinkthen_cobol_ReceiptAtomicDouble *out) {
    int code=0;
    (void)a;
    if(input->answer) {
        thinkthen_cobol_ReceiptAnswer *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswer(a,input->answer,value);
        out->answer=value;
    }
    if(input->answer_id) {
        thinkthen_cobol_ReceiptAnswerId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswerId(a,input->answer_id,value);
        out->answer_id=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptImageListPresence(a,&input->images,&out->images);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->index,&out->index);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->input,&out->input);
    if(!code) code=copy_thinkthen_cobol_ReceiptRankMemberListPresence(a,&input->members,&out->members);
    if(input->meta) {
        thinkthen_cobol_ReceiptMeta *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptMeta(a,input->meta,value);
        out->meta=value;
    }
    if(input->question) {
        thinkthen_cobol_ReceiptQuestionDocument *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptQuestionDocument(a,input->question,value);
        out->question=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->question_name,&out->question_name);
    if(input->schema) {
        thinkthen_cobol_ReceiptVersion *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptVersion(a,input->schema,value);
        out->schema=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptPhysicalSourcePresence(a,&input->source,&out->source);
    if(!code) code=copy_thinkthen_cobol_ReceiptThresholdPresence(a,&input->threshold,&out->threshold);
    out->value=input->value;
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAtomicArrayOfString(arena *a,const thinkthen_complete_atomic_array_of_string_v1 *input,
thinkthen_cobol_ReceiptAtomicArrayOfString *out) {
    int code=0;
    (void)a;
    if(input->answer) {
        thinkthen_cobol_ReceiptAnswer *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswer(a,input->answer,value);
        out->answer=value;
    }
    if(input->answer_id) {
        thinkthen_cobol_ReceiptAnswerId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswerId(a,input->answer_id,value);
        out->answer_id=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptImageListPresence(a,&input->images,&out->images);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->index,&out->index);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->input,&out->input);
    if(!code) code=copy_thinkthen_cobol_ReceiptRankMemberListPresence(a,&input->members,&out->members);
    if(input->meta) {
        thinkthen_cobol_ReceiptMeta *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptMeta(a,input->meta,value);
        out->meta=value;
    }
    if(input->question) {
        thinkthen_cobol_ReceiptQuestionDocument *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptQuestionDocument(a,input->question,value);
        out->question=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->question_name,&out->question_name);
    if(input->schema) {
        thinkthen_cobol_ReceiptVersion *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptVersion(a,input->schema,value);
        out->schema=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptPhysicalSourcePresence(a,&input->source,&out->source);
    if(!code) code=copy_thinkthen_cobol_ReceiptThresholdPresence(a,&input->threshold,&out->threshold);
    if(!code) code=copy_thinkthen_cobol_ReceiptStringList(a,&input->value,&out->value);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAtomicNullableString(arena *a,const thinkthen_complete_atomic_nullable_string_v1 *input,
thinkthen_cobol_ReceiptAtomicNullableString *out) {
    int code=0;
    (void)a;
    if(input->answer) {
        thinkthen_cobol_ReceiptAnswer *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswer(a,input->answer,value);
        out->answer=value;
    }
    if(input->answer_id) {
        thinkthen_cobol_ReceiptAnswerId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswerId(a,input->answer_id,value);
        out->answer_id=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptImageListPresence(a,&input->images,&out->images);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->index,&out->index);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->input,&out->input);
    if(!code) code=copy_thinkthen_cobol_ReceiptRankMemberListPresence(a,&input->members,&out->members);
    if(input->meta) {
        thinkthen_cobol_ReceiptMeta *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptMeta(a,input->meta,value);
        out->meta=value;
    }
    if(input->question) {
        thinkthen_cobol_ReceiptQuestionDocument *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptQuestionDocument(a,input->question,value);
        out->question=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->question_name,&out->question_name);
    if(input->schema) {
        thinkthen_cobol_ReceiptVersion *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptVersion(a,input->schema,value);
        out->schema=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptPhysicalSourcePresence(a,&input->source,&out->source);
    if(!code) code=copy_thinkthen_cobol_ReceiptThresholdPresence(a,&input->threshold,&out->threshold);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->value,&out->value);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAtomicDecideValue(arena *a,const thinkthen_complete_atomic_decide_value_v1 *input,
thinkthen_cobol_ReceiptAtomicDecideValue *out) {
    int code=0;
    (void)a;
    if(input->answer) {
        thinkthen_cobol_ReceiptAnswer *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswer(a,input->answer,value);
        out->answer=value;
    }
    if(input->answer_id) {
        thinkthen_cobol_ReceiptAnswerId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswerId(a,input->answer_id,value);
        out->answer_id=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptImageListPresence(a,&input->images,&out->images);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->index,&out->index);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->input,&out->input);
    if(!code) code=copy_thinkthen_cobol_ReceiptRankMemberListPresence(a,&input->members,&out->members);
    if(input->meta) {
        thinkthen_cobol_ReceiptMeta *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptMeta(a,input->meta,value);
        out->meta=value;
    }
    if(input->question) {
        thinkthen_cobol_ReceiptQuestionDocument *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptQuestionDocument(a,input->question,value);
        out->question=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->question_name,&out->question_name);
    if(input->schema) {
        thinkthen_cobol_ReceiptVersion *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptVersion(a,input->schema,value);
        out->schema=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptPhysicalSourcePresence(a,&input->source,&out->source);
    if(!code) code=copy_thinkthen_cobol_ReceiptThresholdPresence(a,&input->threshold,&out->threshold);
    if(!code) code=copy_thinkthen_cobol_ReceiptDecideValuePresence(a,&input->value,&out->value);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelation(arena *a,const thinkthen_complete_relation_v1 *input,
thinkthen_cobol_ReceiptRelation *out) {
    int code=0;
    (void)a;
    if(input->answer) {
        thinkthen_cobol_ReceiptAnswers *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswers(a,input->answer,value);
        out->answer=value;
    }
    if(input->answer_id) {
        thinkthen_cobol_ReceiptAnswerId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswerId(a,input->answer_id,value);
        out->answer_id=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->file,&out->file);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->first_line,&out->first_line);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->index,&out->index);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->input,&out->input);
    if(!code) code=copy_thinkthen_cobol_ReceiptSessionInputSourceListPresence(a,&input->input_sources,
    &out->input_sources);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->last_line,&out->last_line);
    if(input->meta) {
        thinkthen_cobol_ReceiptMeta *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptMeta(a,input->meta,value);
        out->meta=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptPositionPresence(a,&input->position,&out->position);
    if(input->question) {
        thinkthen_cobol_ReceiptRelateQuestionDocument *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptRelateQuestionDocument(a,input->question,value);
        out->question=value;
    }
    if(input->schema) {
        thinkthen_cobol_ReceiptVersion *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptVersion(a,input->schema,value);
        out->schema=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptRelatedEntityEdgeList(a,&input->value,&out->value);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRelateQuestionDocument(arena *a,const thinkthen_complete_relate_question_document_v1 *input,
thinkthen_cobol_ReceiptRelateQuestionDocument *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptBatchPresence(a,&input->batch,&out->batch);
    if(!code) code=copy_thinkthen_cobol_ReceiptInputDeclarationPresence(a,&input->context_schema,
    &out->context_schema);
    if(!code) code=copy_thinkthen_cobol_ReceiptRelateFieldsPresence(a,&input->fields,&out->fields);
    if(!code) code=copy_thinkthen_cobol_ReceiptInputDeclarationPresence(a,&input->item_schema,&out->item_schema);
    if(!code) code=copy_thinkthen_cobol_ReceiptLabelListPresence(a,&input->label_details,&out->label_details);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->model,&out->model);
    if(!code) code=copy_thinkthen_cobol_ReceiptQuestionNamePresence(a,&input->name,&out->name);
    if(!code) code=copy_thinkthen_cobol_ReceiptStringListPresence(a,&input->on,&out->on);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->profile,&out->profile);
    if(!code) code=copy_thinkthen_cobol_ReceiptRelationRuleList(a,&input->relations,&out->relations);
    if(input->threshold) {
        thinkthen_cobol_ReceiptThreshold *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptThreshold(a,input->threshold,value);
        out->threshold=value;
    }
    if(!code) code=answer_text(input->verb,&out->verb);
    if(!code) code=copy_thinkthen_cobol_ReceiptWordingVersionPresence(a,&input->wording_version,
    &out->wording_version);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptMeta(arena *a,const thinkthen_complete_meta_v1 *input,
thinkthen_cobol_ReceiptMeta *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->answered_by,&out->answered_by);
    if(!code) code=copy_thinkthen_cobol_ReceiptAttemptListPresence(a,&input->attempts,&out->attempts);
    if(!code) code=copy_thinkthen_cobol_ReceiptBatchSettingPresence(a,&input->batch_setting,&out->batch_setting);
    if(!code) code=copy_thinkthen_cobol_ReceiptBatchWarningPresence(a,&input->batch_warning,&out->batch_warning);
    out->cached=input->cached;
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->context_sha256,&out->context_sha256);
    out->failed_questions=input->failed_questions;
    if(!code) code=answer_text(input->model,&out->model);
    if(!code) code=copy_thinkthen_cobol_ReceiptObservationList(a,&input->observations,&out->observations);
    if(!code) code=copy_thinkthen_cobol_ReceiptOriginPresence(a,&input->origin,&out->origin);
    if(!code) code=copy_thinkthen_cobol_ReceiptProfileWarningPresence(a,&input->profile_warning,
    &out->profile_warning);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->question_sha256,&out->question_sha256);
    if(!code) code=copy_thinkthen_cobol_ReceiptQuestionSourceList(a,&input->question_sources,&out->question_sources);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->questions_sha256,&out->questions_sha256);
    if(!code) code=copy_thinkthen_cobol_ReceiptStringList(a,&input->requests,&out->requests);
    out->requests_sent=input->requests_sent;
    if(!code) code=answer_text(input->tool,&out->tool);
    if(!code) code=answer_text(input->url,&out->url);
    if(!code) code=copy_thinkthen_cobol_ReceiptUsagePresence(a,&input->usage,&out->usage);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRecognizeEntities(arena *a,const thinkthen_complete_recognize_entities_v1 *input,
thinkthen_cobol_ReceiptRecognizeEntities *out) {
    int code=0;
    (void)a;
    if(!code) code=copy_thinkthen_cobol_ReceiptEntityList(a,&input->entities,&out->entities);
    if(!code) code=copy_thinkthen_cobol_ReceiptEntityEdgeListPresence(a,&input->relations,&out->relations);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptFind(arena *a,const thinkthen_complete_find_v1 *input,
thinkthen_cobol_ReceiptFind *out) {
    int code=0;
    (void)a;
    if(input->answer) {
        thinkthen_cobol_ReceiptFindAnswer *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptFindAnswer(a,input->answer,value);
        out->answer=value;
    }
    if(input->answer_id) {
        thinkthen_cobol_ReceiptAnswerId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswerId(a,input->answer_id,value);
        out->answer_id=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptFindCandidateListPresence(a,&input->candidates,&out->candidates);
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->file,&out->file);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->first_line,&out->first_line);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->index,&out->index);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->last_line,&out->last_line);
    if(input->meta) {
        thinkthen_cobol_ReceiptMeta *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptMeta(a,input->meta,value);
        out->meta=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptPositionPresence(a,&input->position,&out->position);
    if(input->question) {
        thinkthen_cobol_ReceiptFindQuestionDocument *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptFindQuestionDocument(a,input->question,value);
        out->question=value;
    }
    if(input->schema) {
        thinkthen_cobol_ReceiptVersion *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptVersion(a,input->schema,value);
        out->schema=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->threshold,&out->threshold);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->value,&out->value);
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptAtomicNonZeroUsize(arena *a,const thinkthen_complete_atomic_non_zero_usize_v1 *input,
thinkthen_cobol_ReceiptAtomicNonZeroUsize *out) {
    int code=0;
    (void)a;
    if(input->answer) {
        thinkthen_cobol_ReceiptAnswer *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswer(a,input->answer,value);
        out->answer=value;
    }
    if(input->answer_id) {
        thinkthen_cobol_ReceiptAnswerId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswerId(a,input->answer_id,value);
        out->answer_id=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptImageListPresence(a,&input->images,&out->images);
    if(!code) code=copy_thinkthen_cobol_ReceiptU64Presence(a,&input->index,&out->index);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->input,&out->input);
    if(!code) code=copy_thinkthen_cobol_ReceiptRankMemberListPresence(a,&input->members,&out->members);
    if(input->meta) {
        thinkthen_cobol_ReceiptMeta *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptMeta(a,input->meta,value);
        out->meta=value;
    }
    if(input->question) {
        thinkthen_cobol_ReceiptQuestionDocument *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptQuestionDocument(a,input->question,value);
        out->question=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptUtf8Presence(a,&input->question_name,&out->question_name);
    if(input->schema) {
        thinkthen_cobol_ReceiptVersion *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptVersion(a,input->schema,value);
        out->schema=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptPhysicalSourcePresence(a,&input->source,&out->source);
    if(!code) code=copy_thinkthen_cobol_ReceiptThresholdPresence(a,&input->threshold,&out->threshold);
    out->value=input->value;
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int copy_thinkthen_cobol_ReceiptRankMemberResult(arena *a,const thinkthen_complete_rank_member_result_v1 *input,
thinkthen_cobol_ReceiptRankMemberResult *out) {
    int code=0;
    (void)a;
    if(input->answer) {
        thinkthen_cobol_ReceiptAnswer *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswer(a,input->answer,value);
        out->answer=value;
    }
    if(input->answer_id) {
        thinkthen_cobol_ReceiptAnswerId *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptAnswerId(a,input->answer_id,value);
        out->answer_id=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptImageListPresence(a,&input->images,&out->images);
    if(input->meta) {
        thinkthen_cobol_ReceiptMeta *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptMeta(a,input->meta,value);
        out->meta=value;
    }
    if(input->question) {
        thinkthen_cobol_ReceiptQuestionDocument *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptQuestionDocument(a,input->question,value);
        out->question=value;
    }
    if(input->schema) {
        thinkthen_cobol_ReceiptVersion *value=allocate(a,1,sizeof(*value));
        if(!value) return a->code;
        if(!code) code=copy_thinkthen_cobol_ReceiptVersion(a,input->schema,value);
        out->schema=value;
    }
    if(!code) code=copy_thinkthen_cobol_ReceiptPhysicalSourcePresence(a,&input->source,&out->source);
    if(!code) code=copy_thinkthen_cobol_ReceiptJsonPresence(a,&input->threshold,&out->threshold);
    out->value=input->value;
    if(!code) code=copy_thinkthen_cobol_ReceiptExtensions(a,&input->extensions,&out->extensions);
    return code ? code : a->code;
}
static int append_receipts(tt_answer_owner *owner,const void *rows,size_t count,size_t width) {
    if(count>SIZE_MAX/width-owner->receipt_count) return THINKTHEN_COBOL_OVERFLOW;
    size_t total=owner->receipt_count+count;
    void *data=realloc(owner->receipts,total*width);
    if(total && !data) return THINKTHEN_ELOCAL;
    owner->receipts=data;
    if(count) memcpy((char *)data+owner->receipt_count*width,rows,count*width);
    owner->receipt_count=total;
    return 0;
}
static int append_numbers(tt_answer_owner *owner,const thinkthen_cobol_AnswerNumber *rows,size_t count) {
    if(count>SIZE_MAX/sizeof(*rows)-owner->probability_count) return THINKTHEN_COBOL_OVERFLOW;
    size_t total=owner->probability_count+count;
    void *data=realloc(owner->probabilities,total*sizeof(*rows));
    if(total && !data) return THINKTHEN_ELOCAL;
    owner->probabilities=data;
    if(count) memcpy(owner->probabilities+owner->probability_count,rows,count*sizeof(*rows));
    owner->probability_count=total;
    return 0;
}
static int append_probability_list(tt_answer_owner *owner,const thinkthen_cobol_AnswerNumber *rows,
size_t count) {
    if(owner->probability_list_count==SIZE_MAX/sizeof(*owner->probability_lists)) return THINKTHEN_COBOL_OVERFLOW;
    size_t total=owner->probability_list_count+1;
    void *data=realloc(owner->probability_lists,total*sizeof(*owner->probability_lists));
    if(!data) return THINKTHEN_ELOCAL;
    owner->probability_lists=data;
    owner->probability_lists[owner->probability_list_count++]=(thinkthen_cobol_AnswerProbabilityList){
        rows,count
    };
    return 0;
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
        owner->resolved=data;
        memset(owner->resolved+owner->resolved_count,0,count-owner->resolved_count);
        owner->resolved_count=count;
    }
    owner->resolved[row->index]=(unsigned char)present;
    return 0;
}
static thinkthen_cobol_AnswerNumber picked_number(thinkthen_complete_number_map_v1 probabilities,
thinkthen_complete_utf8_v1 pick) {
    for(size_t i=0;
    i<probabilities.len;
    i++) if(probabilities.data[i].name.len==pick.len && (!pick.len || !memcmp(probabilities.data[i].name.data,
    pick.data,pick.len))) return (thinkthen_cobol_AnswerNumber){
        1,probabilities.data[i].value
    };
    return (thinkthen_cobol_AnswerNumber){
        0
    };
}
static thinkthen_cobol_AnswerNumber answer_number(const thinkthen_complete_answer_v1 *answer) {
    if(answer->kind==THINKTHEN_COMPLETE_ANSWER_YES_NO_V1) return (thinkthen_cobol_AnswerNumber){
        1,answer->data.yes_no->probability
    };
    if(answer->kind==THINKTHEN_COMPLETE_ANSWER_CHOICE_V1) return picked_number(answer->data.choice->probabilities,
    answer->data.choice->pick);
    return (thinkthen_cobol_AnswerNumber){
        0
    };
}
static thinkthen_cobol_AnswerNumber recognition_number(const thinkthen_complete_recognition_v1 *receipt,
const thinkthen_complete_entity_v1 *entity) {
    if(receipt->answer->kind!=THINKTHEN_COMPLETE_RECOGNITION_ODDS_RECOGNIZE_ANSWER_V1) return (thinkthen_cobol_AnswerNumber){
        0
    };
    const thinkthen_complete_recognition_proposal_list_v1 proposals=receipt->answer->data.recognize_answer->proposals;
    for(size_t i=0;
    i<proposals.len;
    i++) {
        const thinkthen_complete_recognition_proposal_v1 *proposal=proposals.data[i];
        if(proposal->kept && proposal->selected.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 && proposal->selected.value->start==entity->start && proposal->selected.value->end==entity->end && proposal->kind.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 && proposal->kind.value.len==entity->kind.len && (!entity->kind.len || !memcmp(proposal->kind.value.data,
        entity->kind.data,entity->kind.len))) return (thinkthen_cobol_AnswerNumber){
            1,proposal->span_probability
        };
    }
    return (thinkthen_cobol_AnswerNumber){
        0
    };
}
static int collect_receipts(tt_answer_owner *owner,const thinkthen_complete_session_packet_v1 *packet,
uint32_t function) {
    arena *a=&owner->storage;
    int code=0;
    switch(function) {
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_DECIDE_V1: {
            const thinkthen_complete_atomic_decide_value_v1 *const *items=NULL;
            size_t count=0;
            const thinkthen_complete_atomic_decide_value_v1 *single=NULL;
            if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_AGGREGATE_V1) {
                items=packet->data.decide_aggregate->value.data;
                count=packet->data.decide_aggregate->value.len;
            }
            else if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1) {
                single=packet->data.decide_row->value;
                items=&single;
                count=1;
            }
            else return 0;
            thinkthen_cobol_ReceiptAtomicDecideValue *rows=allocate(a,count,sizeof(*rows));
            if(!rows) return a->code;
            for(size_t i=0;
            i<count && !code;
            i++) code=copy_thinkthen_cobol_ReceiptAtomicDecideValue(a,items[i],rows+i);
            if(code) return code;
            code=append_receipts(owner,rows,count,sizeof(*rows));
            if(code) return code;
            thinkthen_cobol_AnswerNumber *numbers=allocate(a,count,sizeof(*numbers));
            if(!numbers) return a->code;
            for(size_t i=0;
            i<count;
            i++) {
                numbers[i]=answer_number(items[i]->answer);
                uint64_t index=items[i]->index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? items[i]->index.value : owner->receipt_count-count+i;
                if(index>=owner->resolved_count || !owner->resolved[index]) numbers[i].present=0;
            }
            code=append_numbers(owner,numbers,count);
            break;
        }
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_CHOOSE_V1: {
            const thinkthen_complete_atomic_nullable_string_v1 *const *items=NULL;
            size_t count=0;
            const thinkthen_complete_atomic_nullable_string_v1 *single=NULL;
            if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_AGGREGATE_V1) {
                items=packet->data.choose_aggregate->value.data;
                count=packet->data.choose_aggregate->value.len;
            }
            else if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_ROW_V1) {
                single=packet->data.choose_row->value;
                items=&single;
                count=1;
            }
            else return 0;
            thinkthen_cobol_ReceiptAtomicNullableString *rows=allocate(a,count,sizeof(*rows));
            if(!rows) return a->code;
            for(size_t i=0;
            i<count && !code;
            i++) code=copy_thinkthen_cobol_ReceiptAtomicNullableString(a,items[i],rows+i);
            if(code) return code;
            code=append_receipts(owner,rows,count,sizeof(*rows));
            if(code) return code;
            thinkthen_cobol_AnswerNumber *numbers=allocate(a,count,sizeof(*numbers));
            if(!numbers) return a->code;
            for(size_t i=0;
            i<count;
            i++) {
                numbers[i]=answer_number(items[i]->answer);
                uint64_t index=items[i]->index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? items[i]->index.value : owner->receipt_count-count+i;
                if(index>=owner->resolved_count || !owner->resolved[index]) numbers[i].present=0;
            }
            code=append_numbers(owner,numbers,count);
            break;
        }
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_TAG_V1: {
            const thinkthen_complete_atomic_array_of_string_v1 *const *items=NULL;
            size_t count=0;
            const thinkthen_complete_atomic_array_of_string_v1 *single=NULL;
            if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_TAG_AGGREGATE_V1) {
                items=packet->data.tag_aggregate->value.data;
                count=packet->data.tag_aggregate->value.len;
            }
            else if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_TAG_ROW_V1) {
                single=packet->data.tag_row->value;
                items=&single;
                count=1;
            }
            else return 0;
            thinkthen_cobol_ReceiptAtomicArrayOfString *rows=allocate(a,count,sizeof(*rows));
            if(!rows) return a->code;
            for(size_t i=0;
            i<count && !code;
            i++) code=copy_thinkthen_cobol_ReceiptAtomicArrayOfString(a,items[i],rows+i);
            if(code) return code;
            code=append_receipts(owner,rows,count,sizeof(*rows));
            if(code) return code;
            break;
        }
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_SCORE_V1: {
            const thinkthen_complete_atomic_double_v1 *const *items=NULL;
            size_t count=0;
            const thinkthen_complete_atomic_double_v1 *single=NULL;
            if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_AGGREGATE_V1) {
                items=packet->data.score_aggregate->value.data;
                count=packet->data.score_aggregate->value.len;
            }
            else if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_ROW_V1) {
                single=packet->data.score_row->value;
                items=&single;
                count=1;
            }
            else return 0;
            thinkthen_cobol_ReceiptAtomicDouble *rows=allocate(a,count,sizeof(*rows));
            if(!rows) return a->code;
            for(size_t i=0;
            i<count && !code;
            i++) code=copy_thinkthen_cobol_ReceiptAtomicDouble(a,items[i],rows+i);
            if(code) return code;
            code=append_receipts(owner,rows,count,sizeof(*rows));
            if(code) return code;
            break;
        }
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_FILTER_V1: {
            const thinkthen_complete_atomic_boolean_v1 *const *items=NULL;
            size_t count=0;
            const thinkthen_complete_atomic_boolean_v1 *single=NULL;
            if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_AGGREGATE_V1) {
                items=packet->data.filter_aggregate->value.data;
                count=packet->data.filter_aggregate->value.len;
            }
            else if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_ROW_V1) {
                single=packet->data.filter_row->value;
                items=&single;
                count=1;
            }
            else return 0;
            thinkthen_cobol_ReceiptAtomicBoolean *rows=allocate(a,count,sizeof(*rows));
            if(!rows) return a->code;
            for(size_t i=0;
            i<count && !code;
            i++) code=copy_thinkthen_cobol_ReceiptAtomicBoolean(a,items[i],rows+i);
            if(code) return code;
            code=append_receipts(owner,rows,count,sizeof(*rows));
            if(code) return code;
            thinkthen_cobol_AnswerNumber *numbers=allocate(a,count,sizeof(*numbers));
            if(!numbers) return a->code;
            for(size_t i=0;
            i<count;
            i++) {
                numbers[i]=answer_number(items[i]->answer);
            }
            code=append_numbers(owner,numbers,count);
            break;
        }
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_RANK_V1: {
            const thinkthen_complete_atomic_non_zero_usize_v1 *const *items=NULL;
            size_t count=0;
            if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_RANK_AGGREGATE_V1) {
                items=packet->data.rank_aggregate->value.data;
                count=packet->data.rank_aggregate->value.len;
            }
            else return 0;
            thinkthen_cobol_ReceiptAtomicNonZeroUsize *rows=allocate(a,count,sizeof(*rows));
            if(!rows) return a->code;
            for(size_t i=0;
            i<count && !code;
            i++) code=copy_thinkthen_cobol_ReceiptAtomicNonZeroUsize(a,items[i],rows+i);
            if(code) return code;
            code=append_receipts(owner,rows,count,sizeof(*rows));
            if(code) return code;
            thinkthen_cobol_AnswerNumber *numbers=allocate(a,count,sizeof(*numbers));
            if(!numbers) return a->code;
            for(size_t i=0;
            i<count;
            i++) {
                numbers[i]=answer_number(items[i]->answer);
            }
            code=append_numbers(owner,numbers,count);
            break;
        }
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_FIND_V1: {
            if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_FIND_AGGREGATE_V1) return 0;
            const thinkthen_complete_find_v1 *single=packet->data.find_aggregate->value;
            const thinkthen_complete_find_v1 *const *items=&single;
            size_t count=1;
            thinkthen_cobol_ReceiptFind *rows=allocate(a,count,sizeof(*rows));
            if(!rows) return a->code;
            for(size_t i=0;
            i<count && !code;
            i++) code=copy_thinkthen_cobol_ReceiptFind(a,items[i],rows+i);
            if(code) return code;
            code=append_receipts(owner,rows,count,sizeof(*rows));
            if(code) return code;
            thinkthen_cobol_AnswerNumber *numbers=allocate(a,count,sizeof(*numbers));
            if(!numbers) return a->code;
            for(size_t i=0;
            i<count;
            i++) {
                if(items[i]->index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) numbers[i]=picked_number(items[i]->answer->probabilities,
                items[i]->answer->pick);
            }
            code=append_numbers(owner,numbers,count);
            break;
        }
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_ANNOTATE_V1: {
            const thinkthen_complete_annotation_v1 *const *items=NULL;
            size_t count=0;
            const thinkthen_complete_annotation_v1 *single=NULL;
            if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_AGGREGATE_V1) {
                items=packet->data.annotate_aggregate->value.data;
                count=packet->data.annotate_aggregate->value.len;
            }
            else if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_ROW_V1) {
                single=packet->data.annotate_row->value;
                items=&single;
                count=1;
            }
            else return 0;
            thinkthen_cobol_ReceiptAnnotation *rows=allocate(a,count,sizeof(*rows));
            if(!rows) return a->code;
            for(size_t i=0;
            i<count && !code;
            i++) code=copy_thinkthen_cobol_ReceiptAnnotation(a,items[i],rows+i);
            if(code) return code;
            code=append_receipts(owner,rows,count,sizeof(*rows));
            if(code) return code;
            break;
        }
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_RECOGNIZE_V1: {
            const thinkthen_complete_recognition_v1 *const *items=NULL;
            size_t count=0;
            if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_RECOGNIZE_AGGREGATE_V1) {
                items=packet->data.recognize_aggregate->value.data;
                count=packet->data.recognize_aggregate->value.len;
            }
            else return 0;
            thinkthen_cobol_ReceiptRecognition *rows=allocate(a,count,sizeof(*rows));
            if(!rows) return a->code;
            for(size_t i=0;
            i<count && !code;
            i++) code=copy_thinkthen_cobol_ReceiptRecognition(a,items[i],rows+i);
            if(code) return code;
            code=append_receipts(owner,rows,count,sizeof(*rows));
            if(code) return code;
            for(size_t i=0;
            i<count && !code;
            i++) {
                const thinkthen_complete_recognize_v1 *value=items[i]->value;
                size_t length=value->kind==THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES_V1 ? value->data.entities->entities.len : value->data.mode_proposals->proposals.len;
                thinkthen_cobol_AnswerNumber *numbers=allocate(a,length,sizeof(*numbers));
                if(!numbers) return a->code;
                for(size_t j=0;
                j<length;
                j++) {
                    if(value->kind==THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES_V1) numbers[j]=recognition_number(items[i],
                    value->data.entities->entities.data[j]);
                    else numbers[j]=(thinkthen_cobol_AnswerNumber){
                        1,value->data.mode_proposals->proposals.data[j]->probability
                    };
                }
                code=append_probability_list(owner,numbers,length);
            }
            break;
        }
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_RELATE_V1: {
            if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_RELATE_AGGREGATE_V1) return 0;
            const thinkthen_complete_relation_v1 *single=packet->data.relate_aggregate->value;
            const thinkthen_complete_relation_v1 *const *items=&single;
            size_t count=1;
            thinkthen_cobol_ReceiptRelation *rows=allocate(a,count,sizeof(*rows));
            if(!rows) return a->code;
            for(size_t i=0;
            i<count && !code;
            i++) code=copy_thinkthen_cobol_ReceiptRelation(a,items[i],rows+i);
            if(code) return code;
            code=append_receipts(owner,rows,count,sizeof(*rows));
            if(code) return code;
            size_t length=single->value.len;
            thinkthen_cobol_AnswerNumber *numbers=allocate(a,length,sizeof(*numbers));
            if(!numbers) return a->code;
            for(size_t i=0;
            i<length;
            i++) numbers[i]=(thinkthen_cobol_AnswerNumber){
                1,single->value.data[i]->probability
            };
            code=append_numbers(owner,numbers,length);
            break;
        }
        default: return THINKTHEN_COBOL_REPRESENTATION;
    }
    return code ? code : a->code;
}
static int copy_recognition_entity(arena *a,const thinkthen_complete_entity_v1 *entity,thinkthen_cobol_AnswerRecognition *row) {
    row->start=entity->start;
    row->end=entity->end;
    row->length=entity->length;
    row->strength=entity->strength;
    row->file_present=entity->file.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1;
    row->first_line_present=entity->first_line.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1;
    row->first_line=entity->first_line.value;
    row->last_line_present=entity->last_line.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1;
    row->last_line=entity->last_line.value;
    int code=answer_text(entity->kind,&row->kind);
    if(!code) code=answer_text(entity->text,&row->text);
    if(!code && row->file_present) code=answer_text(entity->file.value,&row->file);
    thinkthen_cobol_AnswerExtensions *extensions=allocate(a,1,sizeof(*extensions));
    if(extensions && !code) code=copy_thinkthen_cobol_AnswerExtensions(a,&entity->extensions,
    extensions);
    row->extensions=extensions;
    return code ? code : a->code;
}
static int project_answer(arena *a,const thinkthen_complete_session_packet_v1 *packet,uint32_t function,
thinkthen_cobol_AnswerTable *out) {
    if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_ROW_V1) {
        const thinkthen_complete_atomic_decide_value_v1 *items[1]={
            packet->data.decide_row->value
        };
        const thinkthen_complete_session_packet_decide_aggregate_v1 aggregate={
            .value={
                items,1
            }
        };
        const thinkthen_complete_session_packet_v1 synthetic={
            .kind=THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_AGGREGATE_V1,.data.decide_aggregate=&aggregate
        };
        return project_answer(a,&synthetic,function,out);
    }
    if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_ROW_V1) {
        const thinkthen_complete_atomic_nullable_string_v1 *items[1]={
            packet->data.choose_row->value
        };
        const thinkthen_complete_session_packet_choose_aggregate_v1 aggregate={
            .value={
                items,1
            }
        };
        const thinkthen_complete_session_packet_v1 synthetic={
            .kind=THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_AGGREGATE_V1,.data.choose_aggregate=&aggregate
        };
        return project_answer(a,&synthetic,function,out);
    }
    if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_TAG_ROW_V1) {
        const thinkthen_complete_atomic_array_of_string_v1 *items[1]={
            packet->data.tag_row->value
        };
        const thinkthen_complete_session_packet_tag_aggregate_v1 aggregate={
            .value={
                items,1
            }
        };
        const thinkthen_complete_session_packet_v1 synthetic={
            .kind=THINKTHEN_COMPLETE_SESSION_PACKET_TAG_AGGREGATE_V1,.data.tag_aggregate=&aggregate
        };
        return project_answer(a,&synthetic,function,out);
    }
    if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_ROW_V1) {
        const thinkthen_complete_atomic_double_v1 *items[1]={
            packet->data.score_row->value
        };
        const thinkthen_complete_session_packet_score_aggregate_v1 aggregate={
            .value={
                items,1
            }
        };
        const thinkthen_complete_session_packet_v1 synthetic={
            .kind=THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_AGGREGATE_V1,.data.score_aggregate=&aggregate
        };
        return project_answer(a,&synthetic,function,out);
    }
    if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_ROW_V1) {
        const thinkthen_complete_atomic_boolean_v1 *items[1]={
            packet->data.filter_row->value
        };
        const thinkthen_complete_session_packet_filter_aggregate_v1 aggregate={
            .value={
                items,1
            }
        };
        const thinkthen_complete_session_packet_v1 synthetic={
            .kind=THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_AGGREGATE_V1,.data.filter_aggregate=&aggregate
        };
        return project_answer(a,&synthetic,function,out);
    }
    if(packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_ROW_V1) {
        const thinkthen_complete_annotation_v1 *items[1]={
            packet->data.annotate_row->value
        };
        const thinkthen_complete_session_packet_annotate_aggregate_v1 aggregate={
            .value={
                items,1
            }
        };
        const thinkthen_complete_session_packet_v1 synthetic={
            .kind=THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_AGGREGATE_V1,.data.annotate_aggregate=&aggregate
        };
        return project_answer(a,&synthetic,function,out);
    }
    switch(function) {
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_DECIDE_V1: {
            if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_DECIDE_AGGREGATE_V1) return 0;
            const thinkthen_complete_atomic_decide_value_list_v1 list=packet->data.decide_aggregate->value;
            thinkthen_cobol_AnswerDecision *rows=allocate(a,list.len,sizeof(*rows));
            out->rows=rows;
            out->count=list.len;
            if(!rows) return a->code;
            for(size_t i=0;
            i<list.len;
            i++) {
                const thinkthen_complete_decide_value_presence_v1 value=list.data[i]->value;
                rows[i].present=value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1;
                if(rows[i].present) {
                    if(!value.value || !value.value->value) return THINKTHEN_COBOL_REPRESENTATION;
                    rows[i].value=answer_original(a,value.value->value);
                } else rows[i].value=answer_original(a,&(thinkthen_complete_json_v1){
                    .kind=THINKTHEN_COMPLETE_JSON_NULL_V1
                });
            }
            break;
        }
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_CHOOSE_V1: {
            if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_CHOOSE_AGGREGATE_V1) return 0;
            const thinkthen_complete_atomic_nullable_string_list_v1 list=packet->data.choose_aggregate->value;
            thinkthen_cobol_AnswerChoice *rows=allocate(a,list.len,sizeof(*rows));
            out->rows=rows;
            out->count=list.len;
            if(!rows) return a->code;
            for(size_t i=0;
            i<list.len;
            i++) {
                rows[i].present=list.data[i]->value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1;
                if(rows[i].present) {
                    int code=answer_text(list.data[i]->value.value,&rows[i].value);
                    if(code) return code;
                }
            }
            break;
        }
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_TAG_V1: {
            if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_TAG_AGGREGATE_V1) return 0;
            const thinkthen_complete_atomic_array_of_string_list_v1 list=packet->data.tag_aggregate->value;
            thinkthen_cobol_AnswerTable *rows=allocate(a,list.len,sizeof(*rows));
            out->rows=rows;
            out->count=list.len;
            if(!rows) return a->code;
            for(size_t i=0;
            i<list.len;
            i++) {
                const thinkthen_cobol_AnswerText *tags=NULL;
                int code=answer_tags(a,list.data[i]->value,&tags);
                if(code) return code;
                rows[i].rows=tags;
                rows[i].count=list.data[i]->value.len;
            }
            break;
        }
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_SCORE_V1: {
            if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_SCORE_AGGREGATE_V1) return 0;
            const thinkthen_complete_atomic_double_list_v1 list=packet->data.score_aggregate->value;
            thinkthen_cobol_AnswerScore *rows=allocate(a,list.len,sizeof(*rows));
            out->rows=rows;
            out->count=list.len;
            if(!rows) return a->code;
            for(size_t i=0;
            i<list.len;
            i++) rows[i].value=list.data[i]->value;
            break;
        }
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_FILTER_V1: {
            if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_FILTER_AGGREGATE_V1) return 0;
            const thinkthen_complete_atomic_boolean_list_v1 list=packet->data.filter_aggregate->value;
            thinkthen_cobol_AnswerFilterOriginal *rows=allocate(a,list.len,sizeof(*rows));
            out->rows=rows;
            if(!rows) return a->code;
            for(size_t i=0;
            i<list.len;
            i++) if(list.data[i]->value) {
                if(list.data[i]->input.presence==THINKTHEN_COMPLETE_PRESENCE_MISSING_V1) return THINKTHEN_COBOL_REPRESENTATION;
                const thinkthen_complete_json_v1 null_value={
                    .kind=THINKTHEN_COMPLETE_JSON_NULL_V1
                };
                rows[out->count++].original=answer_original(a,list.data[i]->input.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? list.data[i]->input.value : &null_value);
            }
            break;
        }
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_RANK_V1: {
            if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_RANK_AGGREGATE_V1) return 0;
            const thinkthen_complete_atomic_non_zero_usize_list_v1 list=packet->data.rank_aggregate->value;
            thinkthen_cobol_AnswerRankIndex *rows=allocate(a,list.len,sizeof(*rows));
            out->rows=rows;
            out->count=list.len;
            if(!rows) return a->code;
            for(size_t i=0;
            i<list.len;
            i++) {
                if(list.data[i]->index.presence!=THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) return THINKTHEN_COBOL_REPRESENTATION;
                rows[i].value=list.data[i]->index.value;
            }
            break;
        }
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_ANNOTATE_V1: {
            if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_ANNOTATE_AGGREGATE_V1) return 0;
            const thinkthen_complete_annotation_list_v1 list=packet->data.annotate_aggregate->value;
            size_t count=0;
            for(size_t i=0;
            i<list.len;
            i++) count+=list.data[i]->value->value.len;
            thinkthen_cobol_AnswerAnnotationField *rows=allocate(a,count,sizeof(*rows));
            out->rows=rows;
            out->count=count;
            if(!rows) return a->code;
            size_t position=0;
            for(size_t i=0;
            i<list.len;
            i++) {
                const thinkthen_complete_annotated_field_map_v1 fields=list.data[i]->value->value;
                for(size_t j=0;
                j<fields.len;
                j++) {
                    thinkthen_cobol_AnswerAnnotationField *row=rows+position++;
                    const thinkthen_complete_annotated_field_v1 *value=fields.data[j].value;
                    row->record=list.data[i]->index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1 ? list.data[i]->index.value : i+1;
                    row->kind=value->kind;
                    int code=answer_text(fields.data[j].name,&row->name);
                    if(code) return code;
                    switch(value->kind) {
                        case THINKTHEN_COMPLETE_ANNOTATED_FIELD_BOOLEAN_V1: row->boolean=value->data.boolean;
                        break;
                        case THINKTHEN_COMPLETE_ANNOTATED_FIELD_NULL_V1: break;
                        case THINKTHEN_COMPLETE_ANNOTATED_FIELD_STRING_V1: code=answer_text(value->data.string,
                        &row->text);
                        break;
                        case THINKTHEN_COMPLETE_ANNOTATED_FIELD_NUMBER_V1: row->number=value->data.number;
                        break;
                        case THINKTHEN_COMPLETE_ANNOTATED_FIELD_ARRAY_V1: code=answer_tags(a,value->data.array,
                        &row->tags);
                        row->tag_count=value->data.array.len;
                        break;
                        case THINKTHEN_COMPLETE_ANNOTATED_FIELD_OBJECT_V1:
                        row->failure_kind=value->data.object->failed->kind->kind;
                        row->failure_cause=value->data.object->failed->cause->kind;
                        break;
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
            thinkthen_cobol_AnswerFindSelection *row=allocate(a,1,sizeof(*row));
            out->rows=row;
            out->count=1;
            if(!row) return a->code;
            row->present=value->index.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1;
            if(row->present) row->index=value->index.value;
            if(value->value.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) row->original=answer_original(a,
            value->value.value);
            else if(row->present) row->original=answer_original(a,&(thinkthen_complete_json_v1){
                .kind=THINKTHEN_COMPLETE_JSON_NULL_V1
            });
            break;
        }
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_RECOGNIZE_V1: {
            if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_RECOGNIZE_AGGREGATE_V1) return 0;
            const thinkthen_complete_recognition_list_v1 list=packet->data.recognize_aggregate->value;
            size_t count=0;
            for(size_t i=0;
            i<list.len;
            i++) {
                const thinkthen_complete_recognize_v1 *value=list.data[i]->value;
                count+=value->kind==THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES_V1 ? value->data.entities->entities.len : value->data.mode_proposals->proposals.len;
            }
            thinkthen_cobol_AnswerRecognition *rows=allocate(a,count,sizeof(*rows));
            out->rows=rows;
            out->count=count;
            if(!rows) return a->code;
            size_t edges_count=0;
            for(size_t i=0;
            i<list.len;
            i++) {
                const thinkthen_complete_recognize_v1 *value=list.data[i]->value;
                if(value->kind==THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES_V1 && value->data.entities->relations.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) edges_count+=value->data.entities->relations.value.len;
            }
            thinkthen_cobol_AnswerRecognitionEdge *edges=allocate(a,edges_count,sizeof(*edges));
            if(!edges) return a->code;
            out->edges=edges;
            out->edge_count=edges_count;
            size_t position=0,edge_position=0;
            for(size_t i=0;
            i<list.len;
            i++) {
                const thinkthen_complete_recognize_v1 *value=list.data[i]->value;
                if(value->kind==THINKTHEN_COMPLETE_RECOGNIZE_ENTITIES_V1) {
                    for(size_t j=0;
                    j<value->data.entities->entities.len;
                    j++) {
                        const thinkthen_complete_entity_v1 *entity=value->data.entities->entities.data[j];
                        thinkthen_cobol_AnswerRecognition *row=rows+position++;
                        row->record=i+1;
                        int code=copy_recognition_entity(a,entity,row);
                        if(code) return code;
                    }
                    if(value->data.entities->relations.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) {
                        const thinkthen_complete_entity_edge_list_v1 input=value->data.entities->relations.value;
                        for(size_t j=0;
                        j<input.len;
                        j++) {
                            const thinkthen_complete_entity_edge_v1 *edge=input.data[j];
                            thinkthen_cobol_AnswerRecognitionEdge *row=edges+edge_position++;
                            row->record=i+1;
                            row->probability=edge->probability;
                            row->either_present=edge->either.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1;
                            row->either=edge->either.value;
                            int code=answer_text(edge->relation,&row->relation);
                            if(!code) code=copy_recognition_entity(a,edge->source,&row->source);
                            if(!code) code=copy_recognition_entity(a,edge->target,&row->target);
                            thinkthen_cobol_AnswerExtensions *extensions=allocate(a,1,sizeof(*extensions));
                            if(extensions && !code) code=copy_thinkthen_cobol_AnswerExtensions(a,
                            &edge->extensions,extensions);
                            row->extensions=extensions;
                            if(code) return code;
                        }
                    }
                } else if(value->kind==THINKTHEN_COMPLETE_RECOGNIZE_MODE_PROPOSALS_V1) {
                    for(size_t j=0;
                    j<value->data.mode_proposals->proposals.len;
                    j++) {
                        const thinkthen_complete_boundary_proposal_v1 *proposal=value->data.mode_proposals->proposals.data[j];
                        thinkthen_cobol_AnswerRecognition *row=rows+position++;
                        row->record=i+1;
                        row->proposal=1;
                        row->mode=value->data.mode_proposals->mode->kind;
                        row->start=proposal->start;
                        row->end=proposal->end;
                        row->length=proposal->length;
                        row->strength=0;
                        int code=answer_text(proposal->text,&row->text);
                        thinkthen_cobol_AnswerExtensions *extensions=allocate(a,1,sizeof(*extensions));
                        if(extensions && !code) code=copy_thinkthen_cobol_AnswerExtensions(a,&proposal->extensions,
                        extensions);
                        row->extensions=extensions;
                        if(code) return code;
                    }
                } else return THINKTHEN_COBOL_REPRESENTATION;
            }
            break;
        }
        case THINKTHEN_COMPLETE_REQUEST_FUNCTION_RELATE_V1: {
            if(packet->kind!=THINKTHEN_COMPLETE_SESSION_PACKET_RELATE_AGGREGATE_V1) return 0;
            const thinkthen_complete_related_entity_edge_list_v1 list=packet->data.relate_aggregate->value->value;
            size_t count=list.len;
            thinkthen_cobol_AnswerRelation *rows=allocate(a,count,sizeof(*rows));
            out->rows=rows;
            out->count=count;
            if(!rows) return a->code;
            size_t position=0;
            for(size_t i=0;
            i<list.len;
            i++) {
                const thinkthen_complete_related_entity_edge_v1 *edge=list.data[i];
                thinkthen_cobol_AnswerRelation *row=rows+position++;
                row->record=i+1;
                row->probability=edge->probability;
                row->either_present=edge->either.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1;
                row->either=edge->either.value;
                int code=answer_text(edge->relation,&row->relation);
                if(code) return code;
                const thinkthen_complete_relation_endpoint_v1 *endpoints[2]={
                    edge->source,edge->target
                };
                thinkthen_cobol_AnswerText *names[2]={
                    &row->source,&row->target
                },*kinds[2]={
                    &row->source_kind,&row->target_kind
                };
                for(size_t k=0;
                k<2;
                k++) {
                    const thinkthen_complete_relation_endpoint_v1 *endpoint=endpoints[k];
                    if(endpoint->kind==THINKTHEN_COMPLETE_RELATION_ENDPOINT_RELATED_ENTITY_V1) {
                        code=answer_text(endpoint->data.related_entity->name,names[k]);
                        if(!code) code=answer_text(endpoint->data.related_entity->kind,kinds[k]);
                    } else if(endpoint->kind==THINKTHEN_COMPLETE_RELATION_ENDPOINT_SOURCE_V1) {
                        code=answer_text(endpoint->data.source->name,names[k]);
                        if(!code) code=answer_text(endpoint->data.source->kind,kinds[k]);
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
        owner->edges=edges;
        owner->edge_capacity=count;
    }
    if(part.edge_count) memcpy((char *)owner->edges+owner->edge_count*width,part.edges,part.edge_count*width);
    owner->edge_count=count;
    return 0;
}
static int append_answer(tt_answer_owner *owner,thinkthen_cobol_AnswerTable part,uint32_t kind) {
    const size_t widths[]={
        [THINKTHEN_COMPLETE_REQUEST_FUNCTION_DECIDE_V1]=sizeof(thinkthen_cobol_AnswerDecision),[THINKTHEN_COMPLETE_REQUEST_FUNCTION_CHOOSE_V1]=sizeof(thinkthen_cobol_AnswerChoice),
        [THINKTHEN_COMPLETE_REQUEST_FUNCTION_TAG_V1]=sizeof(thinkthen_cobol_AnswerTable),[THINKTHEN_COMPLETE_REQUEST_FUNCTION_SCORE_V1]=sizeof(thinkthen_cobol_AnswerScore),
        [THINKTHEN_COMPLETE_REQUEST_FUNCTION_FILTER_V1]=sizeof(thinkthen_cobol_AnswerFilterOriginal),
        [THINKTHEN_COMPLETE_REQUEST_FUNCTION_RANK_V1]=sizeof(thinkthen_cobol_AnswerRankIndex),[THINKTHEN_COMPLETE_REQUEST_FUNCTION_ANNOTATE_V1]=sizeof(thinkthen_cobol_AnswerAnnotationField),
        [THINKTHEN_COMPLETE_REQUEST_FUNCTION_FIND_V1]=sizeof(thinkthen_cobol_AnswerFindSelection),[THINKTHEN_COMPLETE_REQUEST_FUNCTION_RECOGNIZE_V1]=sizeof(thinkthen_cobol_AnswerRecognition),
        [THINKTHEN_COMPLETE_REQUEST_FUNCTION_RELATE_V1]=sizeof(thinkthen_cobol_AnswerRelation)
    };
    size_t width=widths[kind];
    if(part.count>SIZE_MAX/width-owner->count) return THINKTHEN_COBOL_OVERFLOW;
    size_t count=owner->count+part.count;
    if(count>owner->capacity) {
        size_t capacity=count<=SIZE_MAX/width/2 ? count*2 : count;
        void *rows=realloc(owner->rows,capacity*width);
        if(!rows) return THINKTHEN_ELOCAL;
        owner->rows=rows;
        owner->capacity=capacity;
    }
    if(part.count) memcpy((char *)owner->rows+owner->count*width,part.rows,part.count*width);
    owner->count=count;
    return 0;
}
static int capture_immediate_error(int code,thinkthen_cobol_AnswerStatus *out) {
    thinkthen_cobol_AnswerStatus observed={
        .code=code
    };
    if(code>=THINKTHEN_COBOL_REPRESENTATION) {
        *out=observed;
        return code;
    }
    tt_answer_owner *owner=calloc(1,sizeof(*owner));
    if(!owner) {
        observed.code=THINKTHEN_ELOCAL;
        *out=observed;
        return THINKTHEN_ELOCAL;
    }
    observed.owner=owner;
    const thinkthen_error_view_v1 *error=thinkthen_session_error_view();
    if(error) {
        observed.retryable=error->retryable;
        int copied=answer_text(error->message,&observed.message);
        if(error->facts) {
            thinkthen_cobol_AnswerImmediateFacts *facts=allocate(&owner->storage,1,sizeof(*facts));
            if(facts && !copied) copied=copy_thinkthen_cobol_AnswerImmediateFacts(&owner->storage,
            error->facts,facts);
            if(!copied) observed.immediate_facts=facts;
        }
        if(copied || owner->storage.code) observed.code=copied ? copied : owner->storage.code;
    }
    *out=observed;
    return (int)observed.code;
}
static int blocking_answer(const void *engine,const thinkthen_cobol_RequestQuestion *question,
const thinkthen_cobol_RequestInput *input,const thinkthen_cobol_RequestOptions *options,const char *function,
size_t length,uint32_t kind,thinkthen_cobol_AnswerTable *out,thinkthen_cobol_AnswerStatus *status_out) {
    if(!out || !status_out) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_AnswerStatus observed={
        0
    };
    const thinkthen_cobol_RequestCall arguments={
        .m_question=question,.m_input=input,.m_options=options
    };
    tt_answer_owner *owner=calloc(1,sizeof(*owner));
    if(!owner) return THINKTHEN_ELOCAL;
    observed.owner=owner;
    thinkthen_session *session=NULL;
    int code=start(engine,&arguments,(void **)&session,function,length);
    if(code) {
        TT_ANSWER_FREE(owner);
        return capture_immediate_error(code,status_out);
    }
    if(input && input->kind==THINKTHEN_COBOL_REQUESTINPUTFEED_KIND) code=thinkthen_session_finish_typed(session,
    NULL);
    thinkthen_cobol_AnswerTable answer={
        .owner=owner
    };
    uint32_t status=0;
    while(!code && status!=THINKTHEN_SESSION_END_V1) {
        thinkthen_session_result *packet_owner=NULL;
        code=thinkthen_session_next_packet(session,THINKTHEN_SESSION_WAIT_RESULT_V1,&status,
        &packet_owner);
        if(code || status!=THINKTHEN_SESSION_RESULT_V1) break;
        const thinkthen_complete_session_packet_v1 *packet=NULL;
        code=thinkthen_session_result_view(packet_owner,&packet);
        if(!code && packet->kind==THINKTHEN_COMPLETE_SESSION_PACKET_TERMINAL_V1) {
            const thinkthen_complete_session_packet_terminal_v1 *terminal=packet->data.terminal;
            if(terminal->facts.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) {
                code=copy_thinkthen_cobol_AnswerFacts(&owner->storage,terminal->facts.value,
                &observed.facts);
                if(!code) observed.facts_present=1;
            }
            if(terminal->failure.presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) {
                const thinkthen_complete_error_v1 *error=terminal->failure.value->error;
                thinkthen_cobol_AnswerError *details=allocate(&owner->storage,1,sizeof(*details));
                if(details && !code) code=copy_thinkthen_cobol_AnswerError(&owner->storage,
                error,details);
                if(!code) observed.details=details;
                observed.failure_kind=error->kind->kind;
                observed.retryable=error->retryable;
                if(!code) code=answer_text(error->message,&observed.message);
                const int codes[]={
                    0,THINKTHEN_EUSAGE,THINKTHEN_EBACKEND,THINKTHEN_ELOCAL,THINKTHEN_ECANCELLED,THINKTHEN_EDEADLINE,
                    THINKTHEN_ELOCAL
                };
                if(!code) code=owner->storage.code;
                if(!code) code=observed.failure_kind<sizeof(codes)/sizeof(codes[0]) ? codes[observed.failure_kind] : THINKTHEN_ELOCAL;
            }
        } else if(!code) {
            code=observe_resolution(owner,packet);
            if(!code) code=collect_receipts(owner,packet,kind);
            thinkthen_cobol_AnswerTable part={
                0
            };
            if(!code) code=project_answer(&owner->storage,packet,kind,&part);
            if(!code && part.count) code=append_answer(owner,part,kind);
            if(!code && part.edge_count) code=append_edges(owner,part);
        }
        thinkthen_session_result_free(packet_owner);
    }
    thinkthen_session_free(session);
    observed.code=code;
    *status_out=observed;
    if(code) return code;
    answer.rows=owner->rows;
    answer.count=owner->count;
    answer.edges=owner->edges;
    answer.edge_count=owner->edge_count;
    *out=answer;
    return 0;
}
int TT_DECIDE(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerDecideResult *out,thinkthen_cobol_AnswerStatus *status_out) {
    if(!out) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_AnswerTable table={
        0
    };
    int code=blocking_answer(engine,question,input,options,"decide",6,THINKTHEN_COMPLETE_REQUEST_FUNCTION_DECIDE_V1,
    &table,status_out);
    if(code) return code;
    for(size_t i=0;
    i<table.count;
    i++) ((thinkthen_cobol_AnswerDecision *)table.rows)[i].present=i<table.owner->probability_count && table.owner->probabilities[i].present;
    thinkthen_cobol_AnswerDecideResult result={
        .owner=table.owner,.value={
            .rows=table.rows,.count=table.count
        },.results={
            table.owner->receipts,table.owner->receipt_count
        },.facts_present=status_out->facts_present,.facts=status_out->facts
    };
    result.value.scalar=input && (input->kind==THINKTHEN_COBOL_REQUESTINPUTTEXT_KIND || input->kind==THINKTHEN_COBOL_REQUESTINPUTJSON_KIND);
    if(table.count) result.value.first=result.value.rows[0];
    result.probability=(thinkthen_cobol_AnswerProbability){
        .scalar=result.value.scalar,.data=table.owner->probabilities,.count=table.owner->probability_count,
        .lists=table.owner->probability_lists,.list_count=table.owner->probability_list_count
    };
    if(result.probability.count) result.probability.first=result.probability.data[0];
    *out=result;
    return 0;
}
int TT_CHOOSE(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerChooseResult *out,thinkthen_cobol_AnswerStatus *status_out) {
    if(!out) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_AnswerTable table={
        0
    };
    int code=blocking_answer(engine,question,input,options,"choose",6,THINKTHEN_COMPLETE_REQUEST_FUNCTION_CHOOSE_V1,
    &table,status_out);
    if(code) return code;
    thinkthen_cobol_AnswerChooseResult result={
        .owner=table.owner,.value={
            .rows=table.rows,.count=table.count
        },.results={
            table.owner->receipts,table.owner->receipt_count
        },.facts_present=status_out->facts_present,.facts=status_out->facts
    };
    result.value.scalar=input && (input->kind==THINKTHEN_COBOL_REQUESTINPUTTEXT_KIND || input->kind==THINKTHEN_COBOL_REQUESTINPUTJSON_KIND);
    if(table.count) result.value.first=result.value.rows[0];
    result.probability=(thinkthen_cobol_AnswerProbability){
        .scalar=result.value.scalar,.data=table.owner->probabilities,.count=table.owner->probability_count,
        .lists=table.owner->probability_lists,.list_count=table.owner->probability_list_count
    };
    if(result.probability.count) result.probability.first=result.probability.data[0];
    *out=result;
    return 0;
}
int TT_TAG(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerTagResult *out,thinkthen_cobol_AnswerStatus *status_out) {
    if(!out) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_AnswerTable table={
        0
    };
    int code=blocking_answer(engine,question,input,options,"tag",3,THINKTHEN_COMPLETE_REQUEST_FUNCTION_TAG_V1,
    &table,status_out);
    if(code) return code;
    thinkthen_cobol_AnswerTagResult result={
        .owner=table.owner,.value={
            .rows=table.rows,.count=table.count
        },.results={
            table.owner->receipts,table.owner->receipt_count
        },.facts_present=status_out->facts_present,.facts=status_out->facts
    };
    result.value.scalar=input && (input->kind==THINKTHEN_COBOL_REQUESTINPUTTEXT_KIND || input->kind==THINKTHEN_COBOL_REQUESTINPUTJSON_KIND);
    if(table.count) result.value.first=result.value.rows[0];
    result.probability=(thinkthen_cobol_AnswerProbability){
        .scalar=result.value.scalar,.data=table.owner->probabilities,.count=table.owner->probability_count,
        .lists=table.owner->probability_lists,.list_count=table.owner->probability_list_count
    };
    if(result.probability.count) result.probability.first=result.probability.data[0];
    *out=result;
    return 0;
}
int TT_SCORE(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerScoreResult *out,thinkthen_cobol_AnswerStatus *status_out) {
    if(!out) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_AnswerTable table={
        0
    };
    int code=blocking_answer(engine,question,input,options,"score",5,THINKTHEN_COMPLETE_REQUEST_FUNCTION_SCORE_V1,
    &table,status_out);
    if(code) return code;
    thinkthen_cobol_AnswerScoreResult result={
        .owner=table.owner,.value={
            .rows=table.rows,.count=table.count
        },.results={
            table.owner->receipts,table.owner->receipt_count
        },.facts_present=status_out->facts_present,.facts=status_out->facts
    };
    result.value.scalar=input && (input->kind==THINKTHEN_COBOL_REQUESTINPUTTEXT_KIND || input->kind==THINKTHEN_COBOL_REQUESTINPUTJSON_KIND);
    if(table.count) result.value.first=result.value.rows[0];
    result.probability=(thinkthen_cobol_AnswerProbability){
        .scalar=result.value.scalar,.data=table.owner->probabilities,.count=table.owner->probability_count,
        .lists=table.owner->probability_lists,.list_count=table.owner->probability_list_count
    };
    if(result.probability.count) result.probability.first=result.probability.data[0];
    *out=result;
    return 0;
}
int TT_FILTER(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerFilterResult *out,thinkthen_cobol_AnswerStatus *status_out) {
    if(!out) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_AnswerTable table={
        0
    };
    int code=blocking_answer(engine,question,input,options,"filter",6,THINKTHEN_COMPLETE_REQUEST_FUNCTION_FILTER_V1,
    &table,status_out);
    if(code) return code;
    thinkthen_cobol_AnswerFilterResult result={
        .owner=table.owner,.value={
            .rows=table.rows,.count=table.count
        },.results={
            table.owner->receipts,table.owner->receipt_count
        },.facts_present=status_out->facts_present,.facts=status_out->facts
    };
    result.value.scalar=input && (input->kind==THINKTHEN_COBOL_REQUESTINPUTTEXT_KIND || input->kind==THINKTHEN_COBOL_REQUESTINPUTJSON_KIND);
    result.value.scalar=0;
    if(table.count) result.value.first=result.value.rows[0];
    result.probability=(thinkthen_cobol_AnswerProbability){
        .scalar=result.value.scalar,.data=table.owner->probabilities,.count=table.owner->probability_count,
        .lists=table.owner->probability_lists,.list_count=table.owner->probability_list_count
    };
    if(result.probability.count) result.probability.first=result.probability.data[0];
    *out=result;
    return 0;
}
int TT_RANK(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerRankResult *out,thinkthen_cobol_AnswerStatus *status_out) {
    if(!out) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_AnswerTable table={
        0
    };
    int code=blocking_answer(engine,question,input,options,"rank",4,THINKTHEN_COMPLETE_REQUEST_FUNCTION_RANK_V1,
    &table,status_out);
    if(code) return code;
    thinkthen_cobol_AnswerRankResult result={
        .owner=table.owner,.value={
            .rows=table.rows,.count=table.count
        },.results={
            table.owner->receipts,table.owner->receipt_count
        },.facts_present=status_out->facts_present,.facts=status_out->facts
    };
    result.value.scalar=input && (input->kind==THINKTHEN_COBOL_REQUESTINPUTTEXT_KIND || input->kind==THINKTHEN_COBOL_REQUESTINPUTJSON_KIND);
    result.value.scalar=0;
    if(table.count) result.value.first=result.value.rows[0];
    result.probability=(thinkthen_cobol_AnswerProbability){
        .scalar=result.value.scalar,.data=table.owner->probabilities,.count=table.owner->probability_count,
        .lists=table.owner->probability_lists,.list_count=table.owner->probability_list_count
    };
    if(result.probability.count) result.probability.first=result.probability.data[0];
    *out=result;
    return 0;
}
int TT_FIND(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerFindResult *out,thinkthen_cobol_AnswerStatus *status_out) {
    if(!out) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_AnswerTable table={
        0
    };
    int code=blocking_answer(engine,question,input,options,"find",4,THINKTHEN_COMPLETE_REQUEST_FUNCTION_FIND_V1,
    &table,status_out);
    if(code) return code;
    thinkthen_cobol_AnswerFindResult result={
        .owner=table.owner,.value={
            .rows=table.rows,.count=table.count
        },.results={
            table.owner->receipts,table.owner->receipt_count
        },.facts_present=status_out->facts_present,.facts=status_out->facts
    };
    result.value.scalar=input && (input->kind==THINKTHEN_COBOL_REQUESTINPUTTEXT_KIND || input->kind==THINKTHEN_COBOL_REQUESTINPUTJSON_KIND);
    if(table.count) result.value.first=result.value.rows[0];
    result.probability=(thinkthen_cobol_AnswerProbability){
        .scalar=result.value.scalar,.data=table.owner->probabilities,.count=table.owner->probability_count,
        .lists=table.owner->probability_lists,.list_count=table.owner->probability_list_count
    };
    result.value.scalar=1;
    result.probability.scalar=1;
    if(result.probability.count) result.probability.first=result.probability.data[0];
    *out=result;
    return 0;
}
int TT_ANNOTATE(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerAnnotateResult *out,thinkthen_cobol_AnswerStatus *status_out) {
    if(!out) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_AnswerTable table={
        0
    };
    int code=blocking_answer(engine,question,input,options,"annotate",8,THINKTHEN_COMPLETE_REQUEST_FUNCTION_ANNOTATE_V1,
    &table,status_out);
    if(code) return code;
    thinkthen_cobol_AnswerAnnotateResult result={
        .owner=table.owner,.value={
            .rows=table.rows,.count=table.count
        },.results={
            table.owner->receipts,table.owner->receipt_count
        },.facts_present=status_out->facts_present,.facts=status_out->facts
    };
    result.value.scalar=input && (input->kind==THINKTHEN_COBOL_REQUESTINPUTTEXT_KIND || input->kind==THINKTHEN_COBOL_REQUESTINPUTJSON_KIND);
    if(table.count) result.value.first=result.value.rows[0];
    result.probability=(thinkthen_cobol_AnswerProbability){
        .scalar=result.value.scalar,.data=table.owner->probabilities,.count=table.owner->probability_count,
        .lists=table.owner->probability_lists,.list_count=table.owner->probability_list_count
    };
    if(result.probability.count) result.probability.first=result.probability.data[0];
    *out=result;
    return 0;
}
int TT_RECOGNIZE(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerRecognizeResult *out,thinkthen_cobol_AnswerStatus *status_out) {
    if(!out) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_AnswerTable table={
        0
    };
    int code=blocking_answer(engine,question,input,options,"recognize",9,THINKTHEN_COMPLETE_REQUEST_FUNCTION_RECOGNIZE_V1,
    &table,status_out);
    if(code) return code;
    thinkthen_cobol_AnswerRecognizeResult result={
        .owner=table.owner,.value={
            .rows=table.rows,.count=table.count
        },.results={
            table.owner->receipts,table.owner->receipt_count
        },.facts_present=status_out->facts_present,.facts=status_out->facts
    };
    result.value.scalar=input && (input->kind==THINKTHEN_COBOL_REQUESTINPUTTEXT_KIND || input->kind==THINKTHEN_COBOL_REQUESTINPUTJSON_KIND);
    if(table.count) result.value.first=result.value.rows[0];
    result.probability=(thinkthen_cobol_AnswerProbability){
        .scalar=result.value.scalar,.data=table.owner->probabilities,.count=table.owner->probability_count,
        .lists=table.owner->probability_lists,.list_count=table.owner->probability_list_count
    };
    if(result.value.scalar && result.probability.list_count==1) {
        result.probability.data=result.probability.lists[0].data;
        result.probability.count=result.probability.lists[0].count;
        result.probability.list_count=0;
    } result.probability.scalar=0;
    result.value.edges=table.edges;
    result.value.edge_count=table.edge_count;
    if(result.probability.count) result.probability.first=result.probability.data[0];
    *out=result;
    return 0;
}
int TT_RELATE(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerRelateResult *out,thinkthen_cobol_AnswerStatus *status_out) {
    if(!out) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_AnswerTable table={
        0
    };
    int code=blocking_answer(engine,question,input,options,"relate",6,THINKTHEN_COMPLETE_REQUEST_FUNCTION_RELATE_V1,
    &table,status_out);
    if(code) return code;
    thinkthen_cobol_AnswerRelateResult result={
        .owner=table.owner,.value={
            .rows=table.rows,.count=table.count
        },.results={
            table.owner->receipts,table.owner->receipt_count
        },.facts_present=status_out->facts_present,.facts=status_out->facts
    };
    result.value.scalar=input && (input->kind==THINKTHEN_COBOL_REQUESTINPUTTEXT_KIND || input->kind==THINKTHEN_COBOL_REQUESTINPUTJSON_KIND);
    result.value.scalar=0;
    if(table.count) result.value.first=result.value.rows[0];
    result.probability=(thinkthen_cobol_AnswerProbability){
        .scalar=result.value.scalar,.data=table.owner->probabilities,.count=table.owner->probability_count,
        .lists=table.owner->probability_lists,.list_count=table.owner->probability_list_count
    };
    if(result.probability.count) result.probability.first=result.probability.data[0];
    *out=result;
    return 0;
}
static int plan_unsigned(const thinkthen_complete_json_v1 *value,uint64_t *out) {
    if(!value || value->kind!=THINKTHEN_COMPLETE_JSON_NUMBER_V1 || !value->data.number.len || !value->data.number.data) return THINKTHEN_COBOL_REPRESENTATION;
    uint64_t number=0;
    for(size_t i=0;
    i<value->data.number.len;
    i++) {
        unsigned char digit=(unsigned char)value->data.number.data[i];
        if(digit<'0' || digit>'9') return THINKTHEN_COBOL_REPRESENTATION;
        uint64_t add=digit-'0';
        if(number>(UINT64_MAX-add)/10) return THINKTHEN_COBOL_OVERFLOW;
        number=number*10+add;
    }
    *out=number;
    return 0;
}
static int plan_text(const thinkthen_complete_json_v1 *value,thinkthen_cobol_AnswerText *out) {
    if(!value || value->kind!=THINKTHEN_COMPLETE_JSON_STRING_V1) return THINKTHEN_COBOL_REPRESENTATION;
    return answer_text(value->data.string,out);
}
int TT_PREVIEW(const void *engine,const char *function,const uint64_t *length,const thinkthen_cobol_RequestQuestion *question,
const thinkthen_cobol_RequestInput *input,const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerPlan *out,
thinkthen_cobol_AnswerStatus *status_out) {
    if(!length || !out || !status_out) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_RequestCall call={
        .m_question=question,.m_input=input,.m_options=options
    };
    tt_value *owner=NULL;
    int result=TT_PLAN(engine,function,*length,&call,&owner);
    if(result) return capture_immediate_error(result,status_out);
    thinkthen_cobol_AnswerPlan value={
        0
    };
    int code=0;
    if(!code) code=plan_unsigned(field_value(owner,"estimated_bytes",15),&value.estimated_bytes);
    if(!code) {
        const thinkthen_complete_json_v1 *field=field_value(owner,"estimated_input_tokens",22);
        tt_value view={
            .value=(thinkthen_complete_json_v1 *)field
        };
        if(!field || field->kind!=THINKTHEN_COMPLETE_JSON_OBJECT_V1) code=THINKTHEN_COBOL_REPRESENTATION;
        if(!code) code=plan_unsigned(field_value(&view,"lower",5),&value.estimated_input_tokens.lower);
        if(!code) code=plan_unsigned(field_value(&view,"upper",5),&value.estimated_input_tokens.upper);
    }
    if(!code) {
        const thinkthen_complete_json_v1 *field=field_value(owner,"first_body_utf8",15);
        value.first_body_utf8_presence=!field ? THINKTHEN_COMPLETE_PRESENCE_MISSING_V1 : field->kind==THINKTHEN_COMPLETE_JSON_NULL_V1 ? THINKTHEN_COMPLETE_PRESENCE_NULL_V1 : THINKTHEN_COMPLETE_PRESENCE_VALUE_V1;
        if(value.first_body_utf8_presence==THINKTHEN_COMPLETE_PRESENCE_VALUE_V1) code=plan_text(field,
        &value.first_body_utf8);
    }
    if(!code) code=plan_unsigned(field_value(owner,"largest_request_bytes",21),&value.largest_request_bytes);
    if(!code) code=plan_unsigned(field_value(owner,"largest_request_estimated_input_tokens",38),
    &value.largest_request_estimated_input_tokens);
    if(!code) code=plan_unsigned(field_value(owner,"records",7),&value.records);
    if(!code) code=plan_unsigned(field_value(owner,"requests",8),&value.requests);
    if(!code) code=plan_text(field_value(owner,"token_estimate_method",21),&value.token_estimate_method);
    if(!code) {
        const thinkthen_complete_json_v1 *field=field_value(owner,"upper_bound",11);
        if(!field || field->kind!=THINKTHEN_COMPLETE_JSON_BOOLEAN_V1) code=THINKTHEN_COBOL_REPRESENTATION;
        else value.upper_bound=field->data.boolean;
    }
    TT_VALUE_FREE(owner);
    *status_out=(thinkthen_cobol_AnswerStatus){
        .code=code
    };
    if(!code) *out=value;
    return code;
}
