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
static void *convert_RequestCall_RequestArguments(arena *,const void *);
static void *convert_RequestSessionDescriptor_RequestSessionDescriptor(arena *,const void *);
static void *convert_RequestInput_RequestInput(arena *,const void *);
static void *convert_RequestOptions_RequestOptions(arena *,const void *);
static void *convert_RequestQuestion_RequestQuestion(arena *,const void *);
static void *convert_RequestItem_RequestItem(arena *,const void *);
static void *convert_SessionSourceLocation_SessionSourceLocation(arena *,const void *);
static void *convert_RequestInput_text_RequestInput_text(arena *,const void *);
static void *convert_RequestInput_json_RequestInput_json(arena *,const void *);
static void *convert_RequestInput_records_RequestInput_records(arena *,const void *);
static void *convert_RequestInput_units_RequestInput_units(arena *,const void *);
static void *convert_RequestInput_entities_RequestInput_entities(arena *,const void *);
static void *convert_RequestInput_source_RequestInput_source(arena *,const void *);
static void *convert_RequestInput_feed_RequestInput_feed(arena *,const void *);
static void *convert_RequestOptions_member_attempts_Boolean(arena *,const void *);
static void *convert_RequestBatch_RequestBatch(arena *,const void *);
static void *convert_RequestOptions_member_context_Text(arena *,const void *);
static void *convert_RequestOptions_member_context_field_Text(arena *,const void *);
static void *convert_RequestOptions_member_deadline_ms_Integer(arena *,const void *);
static void *convert_RequestOptions_member_details_Boolean(arena *,const void *);
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
static void *convert_RequestBatch_arm_1_Unsigned(arena *,const void *);
static void *convert_RequestBatch_arm_2_Text(arena *,const void *);
static void *convert_RecognitionExample_RecognitionExample(arena *,const void *);
static void *convert_RequestOptions_member_field_item_Text(arena *,const void *);
static void *convert_RecognitionMode_arm_1_Literal_whole(arena *,const void *);
static void *convert_RecognitionMode_arm_2_Literal_boundary_only(arena *,const void *);
static void *convert_RequestThreshold_arm_1_Number(arena *,const void *);
static void *convert_RequestThreshold_arm_2_Text(arena *,const void *);
static void *convert_RecognitionSeedSpan_RecognitionSeedSpan(arena *,const void *);
static void *convert_RecognitionStageContext_member_boundary_Text(arena *,const void *);
static void *convert_RecognitionStageContext_member_kind_edge_Text(arena *,const void *);
static void *convert_RecognitionStageContext_member_relation_Text(arena *,const void *);
static void *convert_RequestQuestion_text_member_text_Text(arena *,const void *);
static void *convert_RequestDefinition_RequestDefinition(arena *,const void *);
static void *convert_RequestQuestion_file_member_path_Text(arena *,const void *);
static void *convert_RequestQuestion_name_member_name_Text(arena *,const void *);
static void *convert_RequestQuestion_reference_member_reference_Text(arena *,const void *);
static void *convert_ContextSchema_arm_1_Text(arena *,const void *);
static void *convert_ContextSchema_arm_2_JsonValue_map(arena *,const void *);
static void *convert_RequestImage_RequestImage(arena *,const void *);
static void *convert_OptionSchema_OptionSchema(arena *,const void *);
static void *convert_RequestOriginal_text_RequestOriginal_text(arena *,const void *);
static void *convert_RequestOriginal_json_RequestOriginal_json(arena *,const void *);
static void *convert_ReaderMedia_ReaderMedia(arena *,const void *);
static void *convert_RequestSource_member_paths_Text_list(arena *,const void *);
static void *convert_RequestFraming_arm_1_Literal_document(arena *,const void *);
static void *convert_RequestFraming_arm_2_Literal_lines(arena *,const void *);
static void *convert_RequestFraming_arm_3_Literal_jsonl(arena *,const void *);
static void *convert_RequestFraming_arm_4_Literal_csv(arena *,const void *);
static void *convert_RequestFraming_arm_5_Literal_tsv(arena *,const void *);
static void *convert_SourceUnit_SourceUnit(arena *,const void *);
static void *convert_RequestReader_member_window_Unsigned(arena *,const void *);
static void *convert_RecognitionExample_arm_1_Text(arena *,const void *);
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
static void *convert_ContextSchema_arm_2_entry_JsonValue(arena *,const void *);
static void *convert_RequestImage_file_RequestImage_file(arena *,const void *);
static void *convert_RequestImage_bytes_RequestImage_bytes(arena *,const void *);
static void *convert_OptionSchema_member_description_JsonValue(arena *,const void *);
static void *convert_OptionSchema_member_name_Text(arena *,const void *);
static void *convert_RequestOriginal_text_member_text_Text(arena *,const void *);
static void *convert_RequestOriginal_json_member_value_RequestOriginal_json_value(arena *,const void *);
static void *convert_ReaderMedia_arm_1_Literal_text(arena *,const void *);
static void *convert_ReaderMedia_arm_2_Literal_image(arena *,const void *);
static void *convert_RequestSource_member_paths_item_Text(arena *,const void *);
static void *convert_SourceUnit_arm_1_Literal_line(arena *,const void *);
static void *convert_SourceUnit_arm_2_Literal_window(arena *,const void *);
static void *convert_SourceUnit_arm_3_Literal_file(arena *,const void *);
static void *convert_RecognitionExampleText_member_entities_RecognitionExampleEntity_list(arena *,
const void *);
static void *convert_RecognitionExampleText_member_kinds_Text_list(arena *,const void *);
static void *convert_RecognitionExampleText_member_text_Text(arena *,const void *);
static void *convert_RequestDefinition_Authored_decide_member_batch_RequestDefinition_Authored_score_batch(arena *,
const void *);
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
static void *convert_RequestDefinition_Authored_choose_member_batch_RequestDefinition_Authored_score_batch(arena *,
const void *);
static void *convert_RequestDefinition_Authored_choose_member_name_Text(arena *,const void *);
static void *convert_Authored_options_Authored_options(arena *,const void *);
static void *convert_Authored_cut_Authored_cut(arena *,const void *);
static void *convert_RequestDefinition_Authored_choose_member_wording_version_Integer(arena *,
const void *);
static void *convert_RequestDefinition_Authored_tag_member_batch_RequestDefinition_Authored_score_batch(arena *,
const void *);
static void *convert_Authored_labels_Authored_labels(arena *,const void *);
static void *convert_RequestDefinition_Authored_tag_member_name_Text(arena *,const void *);
static void *convert_RequestDefinition_Authored_tag_member_wording_version_Integer(arena *,
const void *);
static void *convert_RequestDefinition_Authored_score_member_batch_RequestDefinition_Authored_score_batch(arena *,
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
static void *convert_RequestDefinition_Authored_decide_member_batch_arm_1_Literal_max(arena *,
const void *);
static void *convert_RequestDefinition_Authored_decide_member_batch_arm_2_Integer(arena *,const void *);
static void *convert_Authored_inputDeclaration_string_Authored_inputDeclaration_string(arena *,
const void *);
static void *convert_Authored_inputDeclaration_object_Authored_inputDeclaration_object(arena *,
const void *);
static void *convert_Authored_questionText_arm_1_Text(arena *,const void *);
static void *convert_Authored_questionText_arm_2_JsonValue_map(arena *,const void *);
static void *convert_Authored_questionText_arm_3_JsonValue_list(arena *,const void *);
static void *convert_Authored_criterion_arm_1_Text(arena *,const void *);
static void *convert_Authored_criterion_arm_2_JsonValue_map(arena *,const void *);
static void *convert_Authored_criterion_arm_3_JsonValue_list(arena *,const void *);
static void *convert_Authored_criterion_arm_4_Null(arena *,const void *);
static void *convert_Authored_pointers_arm_1_Text(arena *,const void *);
static void *convert_Authored_pointers_arm_2_Text_list(arena *,const void *);
static void *convert_Authored_threshold_arm_1_Number(arena *,const void *);
static void *convert_Authored_threshold_arm_2_Text(arena *,const void *);
static void *convert_RequestDefinition_Authored_choose_member_batch_arm_1_Literal_max(arena *,
const void *);
static void *convert_RequestDefinition_Authored_choose_member_batch_arm_2_Integer(arena *,const void *);
static void *convert_Authored_options_arm_1_Authored_name_list(arena *,const void *);
static void *convert_Authored_options_arm_2_Authored_description_map(arena *,const void *);
static void *convert_Authored_cut_arm_1_Number(arena *,const void *);
static void *convert_Authored_cut_arm_2_Text(arena *,const void *);
static void *convert_RequestDefinition_Authored_tag_member_batch_arm_1_Literal_max(arena *,
const void *);
static void *convert_RequestDefinition_Authored_tag_member_batch_arm_2_Integer(arena *,const void *);
static void *convert_Authored_labels_arm_1_Authored_name_list(arena *,const void *);
static void *convert_Authored_labels_arm_2_Authored_description_map(arena *,const void *);
static void *convert_RequestDefinition_Authored_score_member_batch_arm_1_Literal_max(arena *,
const void *);
static void *convert_RequestDefinition_Authored_score_member_batch_arm_2_Integer(arena *,const void *);
static void *convert_Authored_levels_arm_1_Authored_name_list(arena *,const void *);
static void *convert_Authored_levels_arm_2_Authored_criterion_map(arena *,const void *);
static void *convert_RelateFields_RelateFields(arena *,const void *);
static void *convert_RelateQuestion_member_relations_Authored_relation_list(arena *,const void *);
static void *convert_RecognizeQuestion_member_kinds_Authored_description_map(arena *,const void *);
static void *convert_RecognizeQuestion_member_relations_Authored_relation_list(arena *,const void *);
static void *convert_RecognizeQuestion_member_snippet_pieces_Unsigned(arena *,const void *);
static void *convert_AnnotateQuestion_AnnotateQuestion(arena *,const void *);
static void *convert_ImageMedia_arm_1_Literal_image_jpeg(arena *,const void *);
static void *convert_ImageMedia_arm_2_Literal_image_png(arena *,const void *);
static void *convert_RecognitionExampleEntity_member_end_Unsigned(arena *,const void *);
static void *convert_RecognitionExampleEntity_member_kind_Text(arena *,const void *);
static void *convert_RecognitionExampleEntity_member_start_Unsigned(arena *,const void *);
static void *convert_Authored_inputDeclaration_object_member_properties_Authored_inputProperty_map(arena *,
const void *);
static void *convert_Authored_inputDeclaration_object_member_required_Text_list(arena *,const void *);
static void *convert_Authored_questionText_arm_2_entry_JsonValue(arena *,const void *);
static void *convert_Authored_questionText_arm_3_item_JsonValue(arena *,const void *);
static void *convert_Authored_criterion_arm_2_entry_JsonValue(arena *,const void *);
static void *convert_Authored_criterion_arm_3_item_JsonValue(arena *,const void *);
static void *convert_Authored_pointers_arm_2_item_Text(arena *,const void *);
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
static void *convert_Authored_description_arm_1_Text(arena *,const void *);
static void *convert_Authored_description_arm_2_JsonValue_map(arena *,const void *);
static void *convert_Authored_description_arm_3_JsonValue_list(arena *,const void *);
static void *convert_Authored_description_arm_4_Null(arena *,const void *);
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
static void *convert_Authored_description_arm_2_entry_JsonValue(arena *,const void *);
static void *convert_Authored_description_arm_3_item_JsonValue(arena *,const void *);
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
        case 1: out->data.arm_1=convert_RequestInput_text_RequestInput_text(a,p->value);
        break;
        case 2: out->data.arm_2=convert_RequestInput_json_RequestInput_json(a,p->value);
        break;
        case 3: out->data.arm_3=convert_RequestInput_records_RequestInput_records(a,p->value);
        break;
        case 4: out->data.arm_4=convert_RequestInput_units_RequestInput_units(a,p->value);
        break;
        case 5: out->data.arm_5=convert_RequestInput_entities_RequestInput_entities(a,p->value);
        break;
        case 6: out->data.arm_6=convert_RequestInput_source_RequestInput_source(a,p->value);
        break;
        case 7: out->data.arm_7=convert_RequestInput_feed_RequestInput_feed(a,p->value);
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
    if(p->m_details) out->m_details=convert_RequestOptions_member_details_Boolean(a,p->m_details);
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
        case 1: out->data.arm_1=convert_RequestQuestion_text_RequestQuestion_text(a,p->value);
        break;
        case 2: out->data.arm_2=convert_RequestQuestion_definition_RequestQuestion_definition(a,p->value);
        break;
        case 3: out->data.arm_3=convert_RequestQuestion_file_RequestQuestion_file(a,p->value);
        break;
        case 4: out->data.arm_4=convert_RequestQuestion_name_RequestQuestion_name(a,p->value);
        break;
        case 5: out->data.arm_5=convert_RequestQuestion_reference_RequestQuestion_reference(a,p->value);
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
        case 1: out->data.arm_1=convert_RequestBatch_arm_1_Unsigned(a,p->value);
        break;
        case 2: out->data.arm_2=convert_RequestBatch_arm_2_Text(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
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
static void *convert_RequestOptions_member_details_Boolean(arena *a,const void *input) {
    const thinkthen_cobol_RequestOptionsDetails *p=input;
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
        case 1: out->data.arm_1=convert_RecognitionMode_arm_1_Literal_whole(a,p->value);
        break;
        case 2: out->data.arm_2=convert_RecognitionMode_arm_2_Literal_boundary_only(a,p->value);
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
        case 1: out->data.arm_1=convert_RequestThreshold_arm_1_Number(a,p->value);
        break;
        case 2: out->data.arm_2=convert_RequestThreshold_arm_2_Text(a,p->value);
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
        case 1: out->data.arm_1=convert_ContextSchema_arm_1_Text(a,p->value);
        break;
        case 2: out->data.arm_2=convert_ContextSchema_arm_2_JsonValue_map(a,p->value);
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
        case 1: out->data.arm_1=convert_RequestOriginal_text_RequestOriginal_text(a,p->value);
        break;
        case 2: out->data.arm_2=convert_RequestOriginal_json_RequestOriginal_json(a,p->value);
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
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
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
        case 1: out->data.arm_1=convert_RequestFraming_arm_1_Literal_document(a,p->value);
        break;
        case 2: out->data.arm_2=convert_RequestFraming_arm_2_Literal_lines(a,p->value);
        break;
        case 3: out->data.arm_3=convert_RequestFraming_arm_3_Literal_jsonl(a,p->value);
        break;
        case 4: out->data.arm_4=convert_RequestFraming_arm_4_Literal_csv(a,p->value);
        break;
        case 5: out->data.arm_5=convert_RequestFraming_arm_5_Literal_tsv(a,p->value);
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
static void *convert_RequestBatch_arm_1_Unsigned(arena *a,const void *input) {
    const thinkthen_cobol_RequestBatchArm1 *p=input;
    thinkthen_request_unsigned_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RequestBatch_arm_2_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestBatchArm2 *p=input;
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
        case 1: out->data.arm_1=convert_RecognitionExample_arm_1_Text(a,p->value);
        break;
        case 2: out->data.arm_2=convert_RecognitionExampleText_RecognitionExampleText(a,p->value);
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
static void *convert_RecognitionMode_arm_1_Literal_whole(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionModeArm1 *p=input;
    thinkthen_request_literal_whole_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RecognitionMode_arm_2_Literal_boundary_only(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionModeArm2 *p=input;
    thinkthen_request_literal_boundary_only_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RequestThreshold_arm_1_Number(arena *a,const void *input) {
    const thinkthen_cobol_RequestThresholdArm1 *p=input;
    thinkthen_request_number_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_RequestThreshold_arm_2_Text(arena *a,const void *input) {
    const thinkthen_cobol_RequestThresholdArm2 *p=input;
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
        case 1: out->data.arm_1=convert_RequestDefinition_Authored_decide_RequestDefinition_Authored_decide(a,
        p->value);
        break;
        case 2: out->data.arm_2=convert_RequestDefinition_Authored_choose_RequestDefinition_Authored_choose(a,
        p->value);
        break;
        case 3: out->data.arm_3=convert_RequestDefinition_Authored_tag_RequestDefinition_Authored_tag(a,
        p->value);
        break;
        case 4: out->data.arm_4=convert_RequestDefinition_Authored_score_RequestDefinition_Authored_score(a,
        p->value);
        break;
        case 5: out->data.arm_5=convert_RequestDefinition_Authored_relate_RequestDefinition_Authored_relate(a,
        p->value);
        break;
        case 6: out->data.arm_6=convert_RequestDefinition_Authored_find_RequestDefinition_Authored_find(a,
        p->value);
        break;
        case 7: out->data.arm_7=convert_RequestDefinition_RecognizeDefinition_RequestDefinition_RecognizeDefinition(a,
        p->value);
        break;
        case 8: out->data.arm_8=convert_RequestDefinition_AnnotateDefinition_RequestDefinition_AnnotateDefinition(a,
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
static void *convert_ContextSchema_arm_1_Text(arena *a,const void *input) {
    const thinkthen_cobol_ContextSchemaArm1 *p=input;
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
static void *convert_ContextSchema_arm_2_JsonValue_map(arena *a,const void *input) {
    const thinkthen_cobol_ContextSchemaArm2 *p=input;
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
        items[i].value=convert_ContextSchema_arm_2_entry_JsonValue(a,p->values[i]);
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
        case 1: out->data.arm_1=convert_RequestImage_file_RequestImage_file(a,p->value);
        break;
        case 2: out->data.arm_2=convert_RequestImage_bytes_RequestImage_bytes(a,p->value);
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
        case 1: out->data.arm_1=convert_ReaderMedia_arm_1_Literal_text(a,p->value);
        break;
        case 2: out->data.arm_2=convert_ReaderMedia_arm_2_Literal_image(a,p->value);
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
static void *convert_RequestFraming_arm_1_Literal_document(arena *a,const void *input) {
    const thinkthen_cobol_RequestFramingArm1 *p=input;
    thinkthen_request_literal_document_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RequestFraming_arm_2_Literal_lines(arena *a,const void *input) {
    const thinkthen_cobol_RequestFramingArm2 *p=input;
    thinkthen_request_literal_lines_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RequestFraming_arm_3_Literal_jsonl(arena *a,const void *input) {
    const thinkthen_cobol_RequestFramingArm3 *p=input;
    thinkthen_request_literal_jsonl_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RequestFraming_arm_4_Literal_csv(arena *a,const void *input) {
    const thinkthen_cobol_RequestFramingArm4 *p=input;
    thinkthen_request_literal_csv_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RequestFraming_arm_5_Literal_tsv(arena *a,const void *input) {
    const thinkthen_cobol_RequestFramingArm5 *p=input;
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
        case 1: out->data.arm_1=convert_SourceUnit_arm_1_Literal_line(a,p->value);
        break;
        case 2: out->data.arm_2=convert_SourceUnit_arm_2_Literal_window(a,p->value);
        break;
        case 3: out->data.arm_3=convert_SourceUnit_arm_3_Literal_file(a,p->value);
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
static void *convert_RecognitionExample_arm_1_Text(arena *a,const void *input) {
    const thinkthen_cobol_RecognitionExampleArm1 *p=input;
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
    if(p->m_batch) out->m_batch=convert_RequestDefinition_Authored_decide_member_batch_RequestDefinition_Authored_score_batch(a,
    p->m_batch);
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
    if(p->m_batch) out->m_batch=convert_RequestDefinition_Authored_choose_member_batch_RequestDefinition_Authored_score_batch(a,
    p->m_batch);
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
    if(p->m_batch) out->m_batch=convert_RequestDefinition_Authored_tag_member_batch_RequestDefinition_Authored_score_batch(a,
    p->m_batch);
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
    if(p->m_batch) out->m_batch=convert_RequestDefinition_Authored_score_member_batch_RequestDefinition_Authored_score_batch(a,
    p->m_batch);
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
static void *convert_ContextSchema_arm_2_entry_JsonValue(arena *a,const void *input) {
    const thinkthen_cobol_ContextSchemaArm2Entry *p=input;
    thinkthen_request_json_value_v1 *out=allocate(a,1,sizeof(*out));
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
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
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
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
    return out;
}
static void *convert_ReaderMedia_arm_1_Literal_text(arena *a,const void *input) {
    const thinkthen_cobol_ReaderMediaArm1 *p=input;
    thinkthen_request_literal_text_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_ReaderMedia_arm_2_Literal_image(arena *a,const void *input) {
    const thinkthen_cobol_ReaderMediaArm2 *p=input;
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
static void *convert_SourceUnit_arm_1_Literal_line(arena *a,const void *input) {
    const thinkthen_cobol_SourceUnitArm1 *p=input;
    thinkthen_request_literal_line_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_SourceUnit_arm_2_Literal_window(arena *a,const void *input) {
    const thinkthen_cobol_SourceUnitArm2 *p=input;
    thinkthen_request_literal_window_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_SourceUnit_arm_3_Literal_file(arena *a,const void *input) {
    const thinkthen_cobol_SourceUnitArm3 *p=input;
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
static void *convert_RequestDefinition_Authored_decide_member_batch_RequestDefinition_Authored_score_batch(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionDecideBatch *p=input;
    thinkthen_request_definition_authored_score_batch_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.arm_1=convert_RequestDefinition_Authored_decide_member_batch_arm_1_Literal_max(a,
        p->value);
        break;
        case 2: out->data.arm_2=convert_RequestDefinition_Authored_decide_member_batch_arm_2_Integer(a,
        p->value);
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
        case 1: out->data.arm_1=convert_Authored_inputDeclaration_string_Authored_inputDeclaration_string(a,
        p->value);
        break;
        case 2: out->data.arm_2=convert_Authored_inputDeclaration_object_Authored_inputDeclaration_object(a,
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
        case 1: out->data.arm_1=convert_Authored_questionText_arm_1_Text(a,p->value);
        break;
        case 2: out->data.arm_2=convert_Authored_questionText_arm_2_JsonValue_map(a,p->value);
        break;
        case 3: out->data.arm_3=convert_Authored_questionText_arm_3_JsonValue_list(a,p->value);
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
        case 1: out->data.arm_1=convert_Authored_criterion_arm_1_Text(a,p->value);
        break;
        case 2: out->data.arm_2=convert_Authored_criterion_arm_2_JsonValue_map(a,p->value);
        break;
        case 3: out->data.arm_3=convert_Authored_criterion_arm_3_JsonValue_list(a,p->value);
        break;
        case 4: out->data.arm_4=convert_Authored_criterion_arm_4_Null(a,p->value);
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
        case 1: out->data.arm_1=convert_Authored_pointers_arm_1_Text(a,p->value);
        break;
        case 2: out->data.arm_2=convert_Authored_pointers_arm_2_Text_list(a,p->value);
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
        case 1: out->data.arm_1=convert_Authored_threshold_arm_1_Number(a,p->value);
        break;
        case 2: out->data.arm_2=convert_Authored_threshold_arm_2_Text(a,p->value);
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
static void *convert_RequestDefinition_Authored_choose_member_batch_RequestDefinition_Authored_score_batch(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionChooseBatch *p=input;
    thinkthen_request_definition_authored_score_batch_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.arm_1=convert_RequestDefinition_Authored_choose_member_batch_arm_1_Literal_max(a,
        p->value);
        break;
        case 2: out->data.arm_2=convert_RequestDefinition_Authored_choose_member_batch_arm_2_Integer(a,
        p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
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
        case 1: out->data.arm_1=convert_Authored_options_arm_1_Authored_name_list(a,p->value);
        break;
        case 2: out->data.arm_2=convert_Authored_options_arm_2_Authored_description_map(a,p->value);
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
        case 1: out->data.arm_1=convert_Authored_cut_arm_1_Number(a,p->value);
        break;
        case 2: out->data.arm_2=convert_Authored_cut_arm_2_Text(a,p->value);
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
static void *convert_RequestDefinition_Authored_tag_member_batch_RequestDefinition_Authored_score_batch(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionTagBatch *p=input;
    thinkthen_request_definition_authored_score_batch_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.arm_1=convert_RequestDefinition_Authored_tag_member_batch_arm_1_Literal_max(a,
        p->value);
        break;
        case 2: out->data.arm_2=convert_RequestDefinition_Authored_tag_member_batch_arm_2_Integer(a,
        p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
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
        case 1: out->data.arm_1=convert_Authored_labels_arm_1_Authored_name_list(a,p->value);
        break;
        case 2: out->data.arm_2=convert_Authored_labels_arm_2_Authored_description_map(a,p->value);
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
static void *convert_RequestDefinition_Authored_score_member_batch_RequestDefinition_Authored_score_batch(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionScoreBatch *p=input;
    thinkthen_request_definition_authored_score_batch_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->kind=(uint32_t)p->kind;
    switch(p->kind) {
        case 1: out->data.arm_1=convert_RequestDefinition_Authored_score_member_batch_arm_1_Literal_max(a,
        p->value);
        break;
        case 2: out->data.arm_2=convert_RequestDefinition_Authored_score_member_batch_arm_2_Integer(a,
        p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
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
        case 1: out->data.arm_1=convert_Authored_levels_arm_1_Authored_name_list(a,p->value);
        break;
        case 2: out->data.arm_2=convert_Authored_levels_arm_2_Authored_criterion_map(a,p->value);
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
    out->text=(thinkthen_complete_utf8_v1){
        p->data,p->len
    };
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
        case 1: out->data.arm_1=convert_ImageMedia_arm_1_Literal_image_jpeg(a,p->value);
        break;
        case 2: out->data.arm_2=convert_ImageMedia_arm_2_Literal_image_png(a,p->value);
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
static void *convert_RequestDefinition_Authored_decide_member_batch_arm_1_Literal_max(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionDecideBatchArm1 *p=input;
    thinkthen_request_literal_max_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RequestDefinition_Authored_decide_member_batch_arm_2_Integer(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionDecideBatchArm2 *p=input;
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
static void *convert_Authored_questionText_arm_1_Text(arena *a,const void *input) {
    const thinkthen_cobol_QuestionTextArm1 *p=input;
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
static void *convert_Authored_questionText_arm_2_JsonValue_map(arena *a,const void *input) {
    const thinkthen_cobol_QuestionTextArm2 *p=input;
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
        items[i].value=convert_Authored_questionText_arm_2_entry_JsonValue(a,p->values[i]);
    }
    return out;
}
static void *convert_Authored_questionText_arm_3_JsonValue_list(arena *a,const void *input) {
    const thinkthen_cobol_QuestionTextArm3 *p=input;
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
    i++) items[i]=convert_Authored_questionText_arm_3_item_JsonValue(a,p->data[i]);
    return out;
}
static void *convert_Authored_criterion_arm_1_Text(arena *a,const void *input) {
    const thinkthen_cobol_CriterionArm1 *p=input;
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
static void *convert_Authored_criterion_arm_2_JsonValue_map(arena *a,const void *input) {
    const thinkthen_cobol_CriterionArm2 *p=input;
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
        items[i].value=convert_Authored_criterion_arm_2_entry_JsonValue(a,p->values[i]);
    }
    return out;
}
static void *convert_Authored_criterion_arm_3_JsonValue_list(arena *a,const void *input) {
    const thinkthen_cobol_CriterionArm3 *p=input;
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
    i++) items[i]=convert_Authored_criterion_arm_3_item_JsonValue(a,p->data[i]);
    return out;
}
static void *convert_Authored_criterion_arm_4_Null(arena *a,const void *input) {
    const thinkthen_cobol_CriterionArm4 *p=input;
    thinkthen_request_null_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_Authored_pointers_arm_1_Text(arena *a,const void *input) {
    const thinkthen_cobol_PointersArm1 *p=input;
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
static void *convert_Authored_pointers_arm_2_Text_list(arena *a,const void *input) {
    const thinkthen_cobol_PointersArm2 *p=input;
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
    i++) items[i]=convert_Authored_pointers_arm_2_item_Text(a,p->data[i]);
    return out;
}
static void *convert_Authored_threshold_arm_1_Number(arena *a,const void *input) {
    const thinkthen_cobol_ThresholdArm1 *p=input;
    thinkthen_request_number_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_Authored_threshold_arm_2_Text(arena *a,const void *input) {
    const thinkthen_cobol_ThresholdArm2 *p=input;
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
static void *convert_RequestDefinition_Authored_choose_member_batch_arm_1_Literal_max(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionChooseBatchArm1 *p=input;
    thinkthen_request_literal_max_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RequestDefinition_Authored_choose_member_batch_arm_2_Integer(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionChooseBatchArm2 *p=input;
    thinkthen_request_integer_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_Authored_options_arm_1_Authored_name_list(arena *a,const void *input) {
    const thinkthen_cobol_OptionsArm1 *p=input;
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
static void *convert_Authored_options_arm_2_Authored_description_map(arena *a,const void *input) {
    const thinkthen_cobol_OptionsArm2 *p=input;
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
static void *convert_Authored_cut_arm_1_Number(arena *a,const void *input) {
    const thinkthen_cobol_CutArm1 *p=input;
    thinkthen_request_number_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_Authored_cut_arm_2_Text(arena *a,const void *input) {
    const thinkthen_cobol_CutArm2 *p=input;
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
static void *convert_RequestDefinition_Authored_tag_member_batch_arm_1_Literal_max(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionTagBatchArm1 *p=input;
    thinkthen_request_literal_max_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RequestDefinition_Authored_tag_member_batch_arm_2_Integer(arena *a,const void *input) {
    const thinkthen_cobol_DefinitionTagBatchArm2 *p=input;
    thinkthen_request_integer_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_Authored_labels_arm_1_Authored_name_list(arena *a,const void *input) {
    const thinkthen_cobol_LabelsArm1 *p=input;
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
static void *convert_Authored_labels_arm_2_Authored_description_map(arena *a,const void *input) {
    const thinkthen_cobol_LabelsArm2 *p=input;
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
static void *convert_RequestDefinition_Authored_score_member_batch_arm_1_Literal_max(arena *a,
const void *input) {
    const thinkthen_cobol_DefinitionScoreBatchArm1 *p=input;
    thinkthen_request_literal_max_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_RequestDefinition_Authored_score_member_batch_arm_2_Integer(arena *a,const void *input) {
    const thinkthen_cobol_DefinitionScoreBatchArm2 *p=input;
    thinkthen_request_integer_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    if(!p) {
        a->code=THINKTHEN_COBOL_REPRESENTATION;
        return out;
    }
    out->value=p->value;
    return out;
}
static void *convert_Authored_levels_arm_1_Authored_name_list(arena *a,const void *input) {
    const thinkthen_cobol_LevelsArm1 *p=input;
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
static void *convert_Authored_levels_arm_2_Authored_criterion_map(arena *a,const void *input) {
    const thinkthen_cobol_LevelsArm2 *p=input;
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
        case 1: out->data.arm_1=convert_AnnotateQuestion_decide_AnnotateQuestion_decide(a,p->value);
        break;
        case 2: out->data.arm_2=convert_AnnotateQuestion_choose_AnnotateQuestion_choose(a,p->value);
        break;
        case 3: out->data.arm_3=convert_AnnotateQuestion_tag_AnnotateQuestion_tag(a,p->value);
        break;
        case 4: out->data.arm_4=convert_AnnotateQuestion_score_AnnotateQuestion_score(a,p->value);
        break;
        default: a->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
    return out;
}
static void *convert_ImageMedia_arm_1_Literal_image_jpeg(arena *a,const void *input) {
    const thinkthen_cobol_ImageMediaArm1 *p=input;
    thinkthen_request_literal_image_jpeg_v1 *out=allocate(a,1,sizeof(*out));
    if(!out) return NULL;
    (void)p;
    return out;
}
static void *convert_ImageMedia_arm_2_Literal_image_png(arena *a,const void *input) {
    const thinkthen_cobol_ImageMediaArm2 *p=input;
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
static void *convert_Authored_questionText_arm_2_entry_JsonValue(arena *a,const void *input) {
    const thinkthen_cobol_QuestionTextArm2Entry *p=input;
    thinkthen_request_json_value_v1 *out=allocate(a,1,sizeof(*out));
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
static void *convert_Authored_questionText_arm_3_item_JsonValue(arena *a,const void *input) {
    const thinkthen_cobol_QuestionTextArm3Item *p=input;
    thinkthen_request_json_value_v1 *out=allocate(a,1,sizeof(*out));
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
static void *convert_Authored_criterion_arm_2_entry_JsonValue(arena *a,const void *input) {
    const thinkthen_cobol_CriterionArm2Entry *p=input;
    thinkthen_request_json_value_v1 *out=allocate(a,1,sizeof(*out));
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
static void *convert_Authored_criterion_arm_3_item_JsonValue(arena *a,const void *input) {
    const thinkthen_cobol_CriterionArm3Item *p=input;
    thinkthen_request_json_value_v1 *out=allocate(a,1,sizeof(*out));
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
static void *convert_Authored_pointers_arm_2_item_Text(arena *a,const void *input) {
    const thinkthen_cobol_PointersArm2Item *p=input;
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
        case 1: out->data.arm_1=convert_Authored_description_arm_1_Text(a,p->value);
        break;
        case 2: out->data.arm_2=convert_Authored_description_arm_2_JsonValue_map(a,p->value);
        break;
        case 3: out->data.arm_3=convert_Authored_description_arm_3_JsonValue_list(a,p->value);
        break;
        case 4: out->data.arm_4=convert_Authored_description_arm_4_Null(a,p->value);
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
        case 1: out->data.arm_1=convert_Authored_inputProperty_string_Authored_inputProperty_string(a,
        p->value);
        break;
        case 2: out->data.arm_2=convert_Authored_inputProperty_number_Authored_inputProperty_number(a,
        p->value);
        break;
        case 3: out->data.arm_3=convert_Authored_inputProperty_boolean_Authored_inputProperty_boolean(a,
        p->value);
        break;
        case 4: out->data.arm_4=convert_Authored_inputProperty_array_Authored_inputProperty_array(a,
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
static void *convert_Authored_description_arm_1_Text(arena *a,const void *input) {
    const thinkthen_cobol_DescriptionArm1 *p=input;
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
static void *convert_Authored_description_arm_2_JsonValue_map(arena *a,const void *input) {
    const thinkthen_cobol_DescriptionArm2 *p=input;
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
        items[i].value=convert_Authored_description_arm_2_entry_JsonValue(a,p->values[i]);
    }
    return out;
}
static void *convert_Authored_description_arm_3_JsonValue_list(arena *a,const void *input) {
    const thinkthen_cobol_DescriptionArm3 *p=input;
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
    i++) items[i]=convert_Authored_description_arm_3_item_JsonValue(a,p->data[i]);
    return out;
}
static void *convert_Authored_description_arm_4_Null(arena *a,const void *input) {
    const thinkthen_cobol_DescriptionArm4 *p=input;
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
static void *convert_Authored_description_arm_2_entry_JsonValue(arena *a,const void *input) {
    const thinkthen_cobol_DescriptionArm2Entry *p=input;
    thinkthen_request_json_value_v1 *out=allocate(a,1,sizeof(*out));
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
static void *convert_Authored_description_arm_3_item_JsonValue(arena *a,const void *input) {
    const thinkthen_cobol_DescriptionArm3Item *p=input;
    thinkthen_request_json_value_v1 *out=allocate(a,1,sizeof(*out));
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
