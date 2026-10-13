/* Generated mechanical Request conversion; Rust owns admission. */
#define _POSIX_C_SOURCE 200809L
#include "tt_requests_generated.h"
#include "thinkthen.h"
#include <locale.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
typedef struct {
    char *data;
    size_t len, capacity;
    int code;
} writer;
static void append(writer *w,const char *data,size_t len) {
    if(w->code) return;
    if(len>SIZE_MAX-w->len-1) {
        w->code=THINKTHEN_COBOL_OVERFLOW;
        return;
    }
    size_t needed=w->len+len+1;
    if(needed>w->capacity) {
        char *next=realloc(w->data,needed);
        if(!next) {
            w->code=THINKTHEN_COBOL_REPRESENTATION;
            return;
        } w->data=next;
        w->capacity=needed;
    }
    if(len) memcpy(w->data+w->len,data,len);
    w->len+=len;
    w->data[w->len]=0;
}
static void raw(writer *w,const char *value) {
    append(w,value,strlen(value));
}
static void text(writer *w,const char *data,uint64_t len,int quote) {
    if(len>THINKTHEN_COBOL_TEXT_CAPACITY) {
        w->code=THINKTHEN_COBOL_OVERFLOW;
        return;
    }
    if(len && !data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    if(!quote) {
        append(w,data,(size_t)len);
        return;
    }
    raw(w,"\"");
    for(uint64_t i=0;
    i<len && !w->code;
    i++) {
        unsigned char c=(unsigned char)data[i];
        if(c=='"' || c=='\\') {
            char escaped[2]={
                '\\',(char)c
            };
            append(w,escaped,2);
        }
        else if(c<32) {
            char escaped[7];
            snprintf(escaped,sizeof(escaped),"\\u%04x",c);
            raw(w,escaped);
        }
        else append(w,data+i,1);
    }
    raw(w,"\"");
}
static int encode(writer *w,const void *input,void (*convert)(writer *,const void *)) {
    locale_t locale=newlocale(LC_NUMERIC_MASK,"C",(locale_t)0);
    if(!locale) return THINKTHEN_COBOL_REPRESENTATION;
    locale_t previous=uselocale(locale);
    if(!previous) {
        freelocale(locale);
        return THINKTHEN_COBOL_REPRESENTATION;
    }
    convert(w,input);
    uselocale(previous);
    freelocale(locale);
    return w->code;
}
static inline void emit_AnnotateDefinition(writer *,const void *);
static inline void emit_AnnotateQuestion(writer *,const void *);
static inline void emit_AnnotateQuestion_choose(writer *,const void *);
static inline void emit_AnnotateQuestion_decide(writer *,const void *);
static inline void emit_AnnotateQuestion_score(writer *,const void *);
static inline void emit_AnnotateQuestion_tag(writer *,const void *);
static inline void emit_Authored_choose(writer *,const void *);
static inline void emit_Authored_criterion(writer *,const void *);
static inline void emit_Authored_cut(writer *,const void *);
static inline void emit_Authored_decide(writer *,const void *);
static inline void emit_Authored_description(writer *,const void *);
static inline void emit_Authored_find(writer *,const void *);
static inline void emit_Authored_inputDeclaration(writer *,const void *);
static inline void emit_Authored_inputDeclaration_object(writer *,const void *);
static inline void emit_Authored_inputDeclaration_string(writer *,const void *);
static inline void emit_Authored_inputProperty(writer *,const void *);
static inline void emit_Authored_inputProperty_array(writer *,const void *);
static inline void emit_Authored_inputProperty_boolean(writer *,const void *);
static inline void emit_Authored_inputProperty_number(writer *,const void *);
static inline void emit_Authored_inputProperty_string(writer *,const void *);
static inline void emit_Authored_labels(writer *,const void *);
static inline void emit_Authored_levels(writer *,const void *);
static inline void emit_Authored_name(writer *,const void *);
static inline void emit_Authored_options(writer *,const void *);
static inline void emit_Authored_pointers(writer *,const void *);
static inline void emit_Authored_profile(writer *,const void *);
static inline void emit_Authored_questionText(writer *,const void *);
static inline void emit_Authored_relate(writer *,const void *);
static inline void emit_Authored_relation(writer *,const void *);
static inline void emit_Authored_score(writer *,const void *);
static inline void emit_Authored_tag(writer *,const void *);
static inline void emit_Authored_threshold(writer *,const void *);
static inline void emit_ContextSchema(writer *,const void *);
static inline void emit_ImageMedia(writer *,const void *);
static inline void emit_OptionSchema(writer *,const void *);
static inline void emit_ReaderMedia(writer *,const void *);
static inline void emit_RecognitionExample(writer *,const void *);
static inline void emit_RecognitionExampleEntity(writer *,const void *);
static inline void emit_RecognitionExampleText(writer *,const void *);
static inline void emit_RecognitionMode(writer *,const void *);
static inline void emit_RecognitionSeedSpan(writer *,const void *);
static inline void emit_RecognitionStageContext(writer *,const void *);
static inline void emit_RecognizeDefinition(writer *,const void *);
static inline void emit_RecognizeQuestion(writer *,const void *);
static inline void emit_RelateFields(writer *,const void *);
static inline void emit_RelateQuestion(writer *,const void *);
static inline void emit_Request(writer *,const void *);
static inline void emit_RequestBatch(writer *,const void *);
static inline void emit_RequestCall(writer *,const void *);
static inline void emit_RequestDefinition(writer *,const void *);
static inline void emit_RequestDefinition_AnnotateDefinition(writer *,const void *);
static inline void emit_RequestDefinition_Authored_choose(writer *,const void *);
static inline void emit_RequestDefinition_Authored_decide(writer *,const void *);
static inline void emit_RequestDefinition_Authored_find(writer *,const void *);
static inline void emit_RequestDefinition_Authored_relate(writer *,const void *);
static inline void emit_RequestDefinition_Authored_score(writer *,const void *);
static inline void emit_RequestDefinition_Authored_tag(writer *,const void *);
static inline void emit_RequestDefinition_RecognizeDefinition(writer *,const void *);
static inline void emit_RequestFraming(writer *,const void *);
static inline void emit_RequestFunction(writer *,const void *);
static inline void emit_RequestImage(writer *,const void *);
static inline void emit_RequestImage_bytes(writer *,const void *);
static inline void emit_RequestImage_file(writer *,const void *);
static inline void emit_RequestInput(writer *,const void *);
static inline void emit_RequestInput_entities(writer *,const void *);
static inline void emit_RequestInput_feed(writer *,const void *);
static inline void emit_RequestInput_json(writer *,const void *);
static inline void emit_RequestInput_records(writer *,const void *);
static inline void emit_RequestInput_source(writer *,const void *);
static inline void emit_RequestInput_text(writer *,const void *);
static inline void emit_RequestInput_units(writer *,const void *);
static inline void emit_RequestItem(writer *,const void *);
static inline void emit_RequestOptions(writer *,const void *);
static inline void emit_RequestOriginal(writer *,const void *);
static inline void emit_RequestOriginal_json(writer *,const void *);
static inline void emit_RequestOriginal_text(writer *,const void *);
static inline void emit_RequestQuestion(writer *,const void *);
static inline void emit_RequestQuestion_definition(writer *,const void *);
static inline void emit_RequestQuestion_file(writer *,const void *);
static inline void emit_RequestQuestion_name(writer *,const void *);
static inline void emit_RequestQuestion_reference(writer *,const void *);
static inline void emit_RequestQuestion_text(writer *,const void *);
static inline void emit_RequestReader(writer *,const void *);
static inline void emit_RequestSessionDescriptor(writer *,const void *);
static inline void emit_RequestSource(writer *,const void *);
static inline void emit_RequestThreshold(writer *,const void *);
static inline void emit_RequestVersion(writer *,const void *);
static inline void emit_SessionSourceLocation(writer *,const void *);
static inline void emit_SourceUnit(writer *,const void *);
static inline void emit_AnnotateDefinition_member_batch(writer *,const void *);
static inline void emit_AnnotateDefinition_member_questions(writer *,const void *);
static inline void emit_AnnotateDefinition_member_version(writer *,const void *);
static inline void emit_AnnotateQuestion_choose_member_name(writer *,const void *);
static inline void emit_AnnotateQuestion_choose_member_wording_version(writer *,const void *);
static inline void emit_AnnotateQuestion_decide_member_name(writer *,const void *);
static inline void emit_AnnotateQuestion_decide_member_wording_version(writer *,const void *);
static inline void emit_AnnotateQuestion_score_member_name(writer *,const void *);
static inline void emit_AnnotateQuestion_score_member_wording_version(writer *,const void *);
static inline void emit_AnnotateQuestion_tag_member_name(writer *,const void *);
static inline void emit_AnnotateQuestion_tag_member_wording_version(writer *,const void *);
static inline void emit_Authored_choose_member_batch(writer *,const void *);
static inline void emit_Authored_choose_member_name(writer *,const void *);
static inline void emit_Authored_choose_member_wording_version(writer *,const void *);
static inline void emit_Authored_criterion_arm_1(writer *,const void *);
static inline void emit_Authored_criterion_arm_2(writer *,const void *);
static inline void emit_Authored_criterion_arm_3(writer *,const void *);
static inline void emit_Authored_criterion_arm_4(writer *,const void *);
static inline void emit_Authored_cut_arm_1(writer *,const void *);
static inline void emit_Authored_cut_arm_2(writer *,const void *);
static inline void emit_Authored_decide_member_batch(writer *,const void *);
static inline void emit_Authored_decide_member_name(writer *,const void *);
static inline void emit_Authored_decide_member_wording_version(writer *,const void *);
static inline void emit_Authored_description_arm_1(writer *,const void *);
static inline void emit_Authored_description_arm_2(writer *,const void *);
static inline void emit_Authored_description_arm_3(writer *,const void *);
static inline void emit_Authored_description_arm_4(writer *,const void *);
static inline void emit_Authored_find_member_name(writer *,const void *);
static inline void emit_Authored_find_member_wording_version(writer *,const void *);
static inline void emit_Authored_inputDeclaration_object_member_properties(writer *,const void *);
static inline void emit_Authored_inputDeclaration_object_member_required(writer *,const void *);
static inline void emit_Authored_inputDeclaration_object_member_type(writer *,const void *);
static inline void emit_Authored_inputDeclaration_string_member_type(writer *,const void *);
static inline void emit_Authored_inputProperty_array_member_items(writer *,const void *);
static inline void emit_Authored_inputProperty_array_member_type(writer *,const void *);
static inline void emit_Authored_inputProperty_boolean_member_type(writer *,const void *);
static inline void emit_Authored_inputProperty_number_member_type(writer *,const void *);
static inline void emit_Authored_inputProperty_string_member_type(writer *,const void *);
static inline void emit_Authored_labels_arm_1(writer *,const void *);
static inline void emit_Authored_labels_arm_2(writer *,const void *);
static inline void emit_Authored_levels_arm_1(writer *,const void *);
static inline void emit_Authored_levels_arm_2(writer *,const void *);
static inline void emit_Authored_options_arm_1(writer *,const void *);
static inline void emit_Authored_options_arm_2(writer *,const void *);
static inline void emit_Authored_pointers_arm_1(writer *,const void *);
static inline void emit_Authored_pointers_arm_2(writer *,const void *);
static inline void emit_Authored_questionText_arm_1(writer *,const void *);
static inline void emit_Authored_questionText_arm_2(writer *,const void *);
static inline void emit_Authored_questionText_arm_3(writer *,const void *);
static inline void emit_Authored_relate_member_name(writer *,const void *);
static inline void emit_Authored_relate_member_version(writer *,const void *);
static inline void emit_Authored_relate_member_wording_version(writer *,const void *);
static inline void emit_Authored_relation_member_either(writer *,const void *);
static inline void emit_Authored_relation_member_single(writer *,const void *);
static inline void emit_Authored_score_member_batch(writer *,const void *);
static inline void emit_Authored_score_member_name(writer *,const void *);
static inline void emit_Authored_score_member_wording_version(writer *,const void *);
static inline void emit_Authored_tag_member_batch(writer *,const void *);
static inline void emit_Authored_tag_member_name(writer *,const void *);
static inline void emit_Authored_tag_member_wording_version(writer *,const void *);
static inline void emit_Authored_threshold_arm_1(writer *,const void *);
static inline void emit_Authored_threshold_arm_2(writer *,const void *);
static inline void emit_ContextSchema_arm_1(writer *,const void *);
static inline void emit_ContextSchema_arm_2(writer *,const void *);
static inline void emit_ImageMedia_arm_1(writer *,const void *);
static inline void emit_ImageMedia_arm_2(writer *,const void *);
static inline void emit_OptionSchema_member_description(writer *,const void *);
static inline void emit_OptionSchema_member_name(writer *,const void *);
static inline void emit_ReaderMedia_arm_1(writer *,const void *);
static inline void emit_ReaderMedia_arm_2(writer *,const void *);
static inline void emit_RecognitionExample_arm_1(writer *,const void *);
static inline void emit_RecognitionExampleEntity_member_end(writer *,const void *);
static inline void emit_RecognitionExampleEntity_member_kind(writer *,const void *);
static inline void emit_RecognitionExampleEntity_member_start(writer *,const void *);
static inline void emit_RecognitionExampleText_member_entities(writer *,const void *);
static inline void emit_RecognitionExampleText_member_kinds(writer *,const void *);
static inline void emit_RecognitionExampleText_member_text(writer *,const void *);
static inline void emit_RecognitionMode_arm_1(writer *,const void *);
static inline void emit_RecognitionMode_arm_2(writer *,const void *);
static inline void emit_RecognitionSeedSpan_member_end(writer *,const void *);
static inline void emit_RecognitionSeedSpan_member_kind(writer *,const void *);
static inline void emit_RecognitionSeedSpan_member_start(writer *,const void *);
static inline void emit_RecognitionStageContext_member_boundary(writer *,const void *);
static inline void emit_RecognitionStageContext_member_kind_edge(writer *,const void *);
static inline void emit_RecognitionStageContext_member_relation(writer *,const void *);
static inline void emit_RecognizeDefinition_member_name(writer *,const void *);
static inline void emit_RecognizeDefinition_member_version(writer *,const void *);
static inline void emit_RecognizeDefinition_member_wording_version(writer *,const void *);
static inline void emit_RecognizeQuestion_member_kinds(writer *,const void *);
static inline void emit_RecognizeQuestion_member_relations(writer *,const void *);
static inline void emit_RecognizeQuestion_member_snippet_pieces(writer *,const void *);
static inline void emit_RelateFields_member_kind(writer *,const void *);
static inline void emit_RelateFields_member_name(writer *,const void *);
static inline void emit_RelateQuestion_member_relations(writer *,const void *);
static inline void emit_RequestBatch_arm_1(writer *,const void *);
static inline void emit_RequestBatch_arm_2(writer *,const void *);
static inline void emit_RequestDefinition_AnnotateDefinition_member_batch(writer *,const void *);
static inline void emit_RequestDefinition_AnnotateDefinition_member_questions(writer *,const void *);
static inline void emit_RequestDefinition_AnnotateDefinition_member_version(writer *,const void *);
static inline void emit_RequestDefinition_Authored_choose_member_batch(writer *,const void *);
static inline void emit_RequestDefinition_Authored_choose_member_name(writer *,const void *);
static inline void emit_RequestDefinition_Authored_choose_member_wording_version(writer *,const void *);
static inline void emit_RequestDefinition_Authored_decide_member_batch(writer *,const void *);
static inline void emit_RequestDefinition_Authored_decide_member_name(writer *,const void *);
static inline void emit_RequestDefinition_Authored_decide_member_wording_version(writer *,const void *);
static inline void emit_RequestDefinition_Authored_find_member_name(writer *,const void *);
static inline void emit_RequestDefinition_Authored_find_member_wording_version(writer *,const void *);
static inline void emit_RequestDefinition_Authored_relate_member_name(writer *,const void *);
static inline void emit_RequestDefinition_Authored_relate_member_version(writer *,const void *);
static inline void emit_RequestDefinition_Authored_relate_member_wording_version(writer *,const void *);
static inline void emit_RequestDefinition_Authored_score_member_batch(writer *,const void *);
static inline void emit_RequestDefinition_Authored_score_member_name(writer *,const void *);
static inline void emit_RequestDefinition_Authored_score_member_wording_version(writer *,const void *);
static inline void emit_RequestDefinition_Authored_tag_member_batch(writer *,const void *);
static inline void emit_RequestDefinition_Authored_tag_member_name(writer *,const void *);
static inline void emit_RequestDefinition_Authored_tag_member_wording_version(writer *,const void *);
static inline void emit_RequestDefinition_RecognizeDefinition_member_name(writer *,const void *);
static inline void emit_RequestDefinition_RecognizeDefinition_member_version(writer *,const void *);
static inline void emit_RequestDefinition_RecognizeDefinition_member_wording_version(writer *,
const void *);
static inline void emit_RequestFraming_arm_1(writer *,const void *);
static inline void emit_RequestFraming_arm_2(writer *,const void *);
static inline void emit_RequestFraming_arm_3(writer *,const void *);
static inline void emit_RequestFraming_arm_4(writer *,const void *);
static inline void emit_RequestFraming_arm_5(writer *,const void *);
static inline void emit_RequestFunction_arm_1(writer *,const void *);
static inline void emit_RequestFunction_arm_2(writer *,const void *);
static inline void emit_RequestFunction_arm_3(writer *,const void *);
static inline void emit_RequestFunction_arm_4(writer *,const void *);
static inline void emit_RequestFunction_arm_5(writer *,const void *);
static inline void emit_RequestFunction_arm_6(writer *,const void *);
static inline void emit_RequestFunction_arm_7(writer *,const void *);
static inline void emit_RequestFunction_arm_8(writer *,const void *);
static inline void emit_RequestFunction_arm_9(writer *,const void *);
static inline void emit_RequestFunction_arm_10(writer *,const void *);
static inline void emit_RequestImage_bytes_member_bytes(writer *,const void *);
static inline void emit_RequestImage_bytes_member_kind(writer *,const void *);
static inline void emit_RequestImage_file_member_kind(writer *,const void *);
static inline void emit_RequestImage_file_member_path(writer *,const void *);
static inline void emit_RequestInput_entities_member_items(writer *,const void *);
static inline void emit_RequestInput_entities_member_kind(writer *,const void *);
static inline void emit_RequestInput_feed_member_images(writer *,const void *);
static inline void emit_RequestInput_feed_member_kind(writer *,const void *);
static inline void emit_RequestInput_feed_member_name(writer *,const void *);
static inline void emit_RequestInput_json_member_images(writer *,const void *);
static inline void emit_RequestInput_json_member_kind(writer *,const void *);
static inline void emit_RequestInput_json_member_value(writer *,const void *);
static inline void emit_RequestInput_records_member_items(writer *,const void *);
static inline void emit_RequestInput_records_member_kind(writer *,const void *);
static inline void emit_RequestInput_source_member_kind(writer *,const void *);
static inline void emit_RequestInput_text_member_images(writer *,const void *);
static inline void emit_RequestInput_text_member_kind(writer *,const void *);
static inline void emit_RequestInput_text_member_text(writer *,const void *);
static inline void emit_RequestInput_units_member_items(writer *,const void *);
static inline void emit_RequestInput_units_member_kind(writer *,const void *);
static inline void emit_RequestItem_member_examples(writer *,const void *);
static inline void emit_RequestItem_member_images(writer *,const void *);
static inline void emit_RequestItem_member_options(writer *,const void *);
static inline void emit_RequestItem_member_seed_spans(writer *,const void *);
static inline void emit_RequestOptions_member_attempts(writer *,const void *);
static inline void emit_RequestOptions_member_context(writer *,const void *);
static inline void emit_RequestOptions_member_context_field(writer *,const void *);
static inline void emit_RequestOptions_member_deadline_ms(writer *,const void *);
static inline void emit_RequestOptions_member_details(writer *,const void *);
static inline void emit_RequestOptions_member_examples(writer *,const void *);
static inline void emit_RequestOptions_member_examples_field(writer *,const void *);
static inline void emit_RequestOptions_member_field(writer *,const void *);
static inline void emit_RequestOptions_member_files_only(writer *,const void *);
static inline void emit_RequestOptions_member_max_requests_total(writer *,const void *);
static inline void emit_RequestOptions_member_model(writer *,const void *);
static inline void emit_RequestOptions_member_none(writer *,const void *);
static inline void emit_RequestOptions_member_options_field(writer *,const void *);
static inline void emit_RequestOptions_member_seed_spans(writer *,const void *);
static inline void emit_RequestOptions_member_seed_spans_field(writer *,const void *);
static inline void emit_RequestOptions_member_snippet_pieces(writer *,const void *);
static inline void emit_RequestOptions_member_top(writer *,const void *);
static inline void emit_RequestOriginal_json_member_kind(writer *,const void *);
static inline void emit_RequestOriginal_json_member_value(writer *,const void *);
static inline void emit_RequestOriginal_text_member_kind(writer *,const void *);
static inline void emit_RequestOriginal_text_member_text(writer *,const void *);
static inline void emit_RequestQuestion_definition_member_kind(writer *,const void *);
static inline void emit_RequestQuestion_file_member_kind(writer *,const void *);
static inline void emit_RequestQuestion_file_member_path(writer *,const void *);
static inline void emit_RequestQuestion_name_member_kind(writer *,const void *);
static inline void emit_RequestQuestion_name_member_name(writer *,const void *);
static inline void emit_RequestQuestion_reference_member_kind(writer *,const void *);
static inline void emit_RequestQuestion_reference_member_reference(writer *,const void *);
static inline void emit_RequestQuestion_text_member_kind(writer *,const void *);
static inline void emit_RequestQuestion_text_member_text(writer *,const void *);
static inline void emit_RequestReader_member_window(writer *,const void *);
static inline void emit_RequestSource_member_paths(writer *,const void *);
static inline void emit_RequestThreshold_arm_1(writer *,const void *);
static inline void emit_RequestThreshold_arm_2(writer *,const void *);
static inline void emit_SessionSourceLocation_member_file(writer *,const void *);
static inline void emit_SessionSourceLocation_member_first_line(writer *,const void *);
static inline void emit_SessionSourceLocation_member_last_line(writer *,const void *);
static inline void emit_SourceUnit_arm_1(writer *,const void *);
static inline void emit_SourceUnit_arm_2(writer *,const void *);
static inline void emit_SourceUnit_arm_3(writer *,const void *);
static inline void emit_Authored_choose_member_batch_arm_1(writer *,const void *);
static inline void emit_Authored_choose_member_batch_arm_2(writer *,const void *);
static inline void emit_Authored_criterion_arm_3_item(writer *,const void *);
static inline void emit_Authored_decide_member_batch_arm_1(writer *,const void *);
static inline void emit_Authored_decide_member_batch_arm_2(writer *,const void *);
static inline void emit_Authored_description_arm_3_item(writer *,const void *);
static inline void emit_Authored_inputDeclaration_object_member_required_item(writer *,const void *);
static inline void emit_Authored_inputProperty_array_member_items_member_type(writer *,const void *);
static inline void emit_Authored_pointers_arm_2_item(writer *,const void *);
static inline void emit_Authored_questionText_arm_3_item(writer *,const void *);
static inline void emit_Authored_score_member_batch_arm_1(writer *,const void *);
static inline void emit_Authored_score_member_batch_arm_2(writer *,const void *);
static inline void emit_Authored_tag_member_batch_arm_1(writer *,const void *);
static inline void emit_Authored_tag_member_batch_arm_2(writer *,const void *);
static inline void emit_RecognitionExampleText_member_kinds_item(writer *,const void *);
static inline void emit_RequestDefinition_Authored_choose_member_batch_arm_1(writer *,const void *);
static inline void emit_RequestDefinition_Authored_choose_member_batch_arm_2(writer *,const void *);
static inline void emit_RequestDefinition_Authored_decide_member_batch_arm_1(writer *,const void *);
static inline void emit_RequestDefinition_Authored_decide_member_batch_arm_2(writer *,const void *);
static inline void emit_RequestDefinition_Authored_score_member_batch_arm_1(writer *,const void *);
static inline void emit_RequestDefinition_Authored_score_member_batch_arm_2(writer *,const void *);
static inline void emit_RequestDefinition_Authored_tag_member_batch_arm_1(writer *,const void *);
static inline void emit_RequestDefinition_Authored_tag_member_batch_arm_2(writer *,const void *);
static inline void emit_RequestOptions_member_field_item(writer *,const void *);
static inline void emit_RequestSource_member_paths_item(writer *,const void *);
static inline void emit_AnnotateDefinition(writer *w,const void *input) {
    const thinkthen_cobol_AnnotateDefinition *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_batch) {
        if(comma++) raw(w,",");
        raw(w,"\"batch\":");
        emit_AnnotateDefinition_member_batch(w,p->m_batch);
    }
    if(p->m_profile) {
        if(comma++) raw(w,",");
        raw(w,"\"profile\":");
        emit_Authored_profile(w,p->m_profile);
    }
    if(p->m_questions) {
        if(comma++) raw(w,",");
        raw(w,"\"questions\":");
        emit_AnnotateDefinition_member_questions(w,p->m_questions);
    }
    if(p->m_threshold) {
        if(comma++) raw(w,",");
        raw(w,"\"threshold\":");
        emit_Authored_threshold(w,p->m_threshold);
    }
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"version\":");
        emit_AnnotateDefinition_member_version(w,p->m_version);
    }
    raw(w,"}");
}
static inline void emit_AnnotateQuestion(writer *w,const void *input) {
    const thinkthen_cobol_AnnotateQuestion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_AnnotateQuestion_decide(w, p->value);
        break;
        case 2: emit_AnnotateQuestion_choose(w, p->value);
        break;
        case 3: emit_AnnotateQuestion_tag(w, p->value);
        break;
        case 4: emit_AnnotateQuestion_score(w, p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_AnnotateQuestion_choose(writer *w,const void *input) {
    const thinkthen_cobol_AnnotateQuestionChoose *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_choose) {
        if(comma++) raw(w,",");
        raw(w,"\"choose\":");
        emit_Authored_questionText(w,p->m_choose);
    }
    if(p->m_context_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"context_schema\":");
        emit_Authored_inputDeclaration(w,p->m_context_schema);
    }
    if(p->m_item_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"item_schema\":");
        emit_Authored_inputDeclaration(w,p->m_item_schema);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_AnnotateQuestion_choose_member_name(w,p->m_name);
    }
    if(p->m_on) {
        if(comma++) raw(w,",");
        raw(w,"\"on\":");
        emit_Authored_pointers(w,p->m_on);
    }
    if(p->m_options) {
        if(comma++) raw(w,",");
        raw(w,"\"options\":");
        emit_Authored_options(w,p->m_options);
    }
    if(p->m_threshold) {
        if(comma++) raw(w,",");
        raw(w,"\"threshold\":");
        emit_Authored_cut(w,p->m_threshold);
    }
    if(p->m_wording_version) {
        if(comma++) raw(w,",");
        raw(w,"\"wording_version\":");
        emit_AnnotateQuestion_choose_member_wording_version(w,p->m_wording_version);
    }
    raw(w,"}");
}
static inline void emit_AnnotateQuestion_decide(writer *w,const void *input) {
    const thinkthen_cobol_AnnotateQuestionDecide *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_context_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"context_schema\":");
        emit_Authored_inputDeclaration(w,p->m_context_schema);
    }
    if(p->m_decide) {
        if(comma++) raw(w,",");
        raw(w,"\"decide\":");
        emit_Authored_questionText(w,p->m_decide);
    }
    if(p->m_false) {
        if(comma++) raw(w,",");
        raw(w,"\"false\":");
        emit_Authored_criterion(w,p->m_false);
    }
    if(p->m_item_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"item_schema\":");
        emit_Authored_inputDeclaration(w,p->m_item_schema);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_AnnotateQuestion_decide_member_name(w,p->m_name);
    }
    if(p->m_on) {
        if(comma++) raw(w,",");
        raw(w,"\"on\":");
        emit_Authored_pointers(w,p->m_on);
    }
    if(p->m_threshold) {
        if(comma++) raw(w,",");
        raw(w,"\"threshold\":");
        emit_Authored_threshold(w,p->m_threshold);
    }
    if(p->m_true) {
        if(comma++) raw(w,",");
        raw(w,"\"true\":");
        emit_Authored_criterion(w,p->m_true);
    }
    if(p->m_wording_version) {
        if(comma++) raw(w,",");
        raw(w,"\"wording_version\":");
        emit_AnnotateQuestion_decide_member_wording_version(w,p->m_wording_version);
    }
    raw(w,"}");
}
static inline void emit_AnnotateQuestion_score(writer *w,const void *input) {
    const thinkthen_cobol_AnnotateQuestionScore *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_context_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"context_schema\":");
        emit_Authored_inputDeclaration(w,p->m_context_schema);
    }
    if(p->m_item_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"item_schema\":");
        emit_Authored_inputDeclaration(w,p->m_item_schema);
    }
    if(p->m_levels) {
        if(comma++) raw(w,",");
        raw(w,"\"levels\":");
        emit_Authored_levels(w,p->m_levels);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_AnnotateQuestion_score_member_name(w,p->m_name);
    }
    if(p->m_on) {
        if(comma++) raw(w,",");
        raw(w,"\"on\":");
        emit_Authored_pointers(w,p->m_on);
    }
    if(p->m_score) {
        if(comma++) raw(w,",");
        raw(w,"\"score\":");
        emit_Authored_questionText(w,p->m_score);
    }
    if(p->m_wording_version) {
        if(comma++) raw(w,",");
        raw(w,"\"wording_version\":");
        emit_AnnotateQuestion_score_member_wording_version(w,p->m_wording_version);
    }
    raw(w,"}");
}
static inline void emit_AnnotateQuestion_tag(writer *w,const void *input) {
    const thinkthen_cobol_AnnotateQuestionTag *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_context_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"context_schema\":");
        emit_Authored_inputDeclaration(w,p->m_context_schema);
    }
    if(p->m_item_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"item_schema\":");
        emit_Authored_inputDeclaration(w,p->m_item_schema);
    }
    if(p->m_labels) {
        if(comma++) raw(w,",");
        raw(w,"\"labels\":");
        emit_Authored_labels(w,p->m_labels);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_AnnotateQuestion_tag_member_name(w,p->m_name);
    }
    if(p->m_on) {
        if(comma++) raw(w,",");
        raw(w,"\"on\":");
        emit_Authored_pointers(w,p->m_on);
    }
    if(p->m_tag) {
        if(comma++) raw(w,",");
        raw(w,"\"tag\":");
        emit_Authored_questionText(w,p->m_tag);
    }
    if(p->m_threshold) {
        if(comma++) raw(w,",");
        raw(w,"\"threshold\":");
        emit_Authored_cut(w,p->m_threshold);
    }
    if(p->m_wording_version) {
        if(comma++) raw(w,",");
        raw(w,"\"wording_version\":");
        emit_AnnotateQuestion_tag_member_wording_version(w,p->m_wording_version);
    }
    raw(w,"}");
}
static inline void emit_Authored_choose(writer *w,const void *input) {
    const thinkthen_cobol_Choose *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_batch) {
        if(comma++) raw(w,",");
        raw(w,"\"batch\":");
        emit_Authored_choose_member_batch(w,p->m_batch);
    }
    if(p->m_choose) {
        if(comma++) raw(w,",");
        raw(w,"\"choose\":");
        emit_Authored_questionText(w,p->m_choose);
    }
    if(p->m_context_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"context_schema\":");
        emit_Authored_inputDeclaration(w,p->m_context_schema);
    }
    if(p->m_item_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"item_schema\":");
        emit_Authored_inputDeclaration(w,p->m_item_schema);
    }
    if(p->m_model) {
        if(comma++) raw(w,",");
        raw(w,"\"model\":");
        emit_Authored_name(w,p->m_model);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_Authored_choose_member_name(w,p->m_name);
    }
    if(p->m_on) {
        if(comma++) raw(w,",");
        raw(w,"\"on\":");
        emit_Authored_pointers(w,p->m_on);
    }
    if(p->m_options) {
        if(comma++) raw(w,",");
        raw(w,"\"options\":");
        emit_Authored_options(w,p->m_options);
    }
    if(p->m_profile) {
        if(comma++) raw(w,",");
        raw(w,"\"profile\":");
        emit_Authored_profile(w,p->m_profile);
    }
    if(p->m_threshold) {
        if(comma++) raw(w,",");
        raw(w,"\"threshold\":");
        emit_Authored_cut(w,p->m_threshold);
    }
    if(p->m_wording_version) {
        if(comma++) raw(w,",");
        raw(w,"\"wording_version\":");
        emit_Authored_choose_member_wording_version(w,p->m_wording_version);
    }
    raw(w,"}");
}
static inline void emit_Authored_criterion(writer *w,const void *input) {
    const thinkthen_cobol_Criterion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_Authored_criterion_arm_1(w,p->value);
        break;
        case 2: emit_Authored_criterion_arm_2(w,p->value);
        break;
        case 3: emit_Authored_criterion_arm_3(w,p->value);
        break;
        case 4: emit_Authored_criterion_arm_4(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_Authored_cut(writer *w,const void *input) {
    const thinkthen_cobol_Cut *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_Authored_cut_arm_1(w,p->value);
        break;
        case 2: emit_Authored_cut_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_Authored_decide(writer *w,const void *input) {
    const thinkthen_cobol_Decide *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_batch) {
        if(comma++) raw(w,",");
        raw(w,"\"batch\":");
        emit_Authored_decide_member_batch(w,p->m_batch);
    }
    if(p->m_context_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"context_schema\":");
        emit_Authored_inputDeclaration(w,p->m_context_schema);
    }
    if(p->m_decide) {
        if(comma++) raw(w,",");
        raw(w,"\"decide\":");
        emit_Authored_questionText(w,p->m_decide);
    }
    if(p->m_false) {
        if(comma++) raw(w,",");
        raw(w,"\"false\":");
        emit_Authored_criterion(w,p->m_false);
    }
    if(p->m_item_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"item_schema\":");
        emit_Authored_inputDeclaration(w,p->m_item_schema);
    }
    if(p->m_model) {
        if(comma++) raw(w,",");
        raw(w,"\"model\":");
        emit_Authored_name(w,p->m_model);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_Authored_decide_member_name(w,p->m_name);
    }
    if(p->m_on) {
        if(comma++) raw(w,",");
        raw(w,"\"on\":");
        emit_Authored_pointers(w,p->m_on);
    }
    if(p->m_profile) {
        if(comma++) raw(w,",");
        raw(w,"\"profile\":");
        emit_Authored_profile(w,p->m_profile);
    }
    if(p->m_threshold) {
        if(comma++) raw(w,",");
        raw(w,"\"threshold\":");
        emit_Authored_threshold(w,p->m_threshold);
    }
    if(p->m_true) {
        if(comma++) raw(w,",");
        raw(w,"\"true\":");
        emit_Authored_criterion(w,p->m_true);
    }
    if(p->m_wording_version) {
        if(comma++) raw(w,",");
        raw(w,"\"wording_version\":");
        emit_Authored_decide_member_wording_version(w,p->m_wording_version);
    }
    raw(w,"}");
}
static inline void emit_Authored_description(writer *w,const void *input) {
    const thinkthen_cobol_Description *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_Authored_description_arm_1(w,p->value);
        break;
        case 2: emit_Authored_description_arm_2(w,p->value);
        break;
        case 3: emit_Authored_description_arm_3(w,p->value);
        break;
        case 4: emit_Authored_description_arm_4(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_Authored_find(writer *w,const void *input) {
    const thinkthen_cobol_Find *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_context_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"context_schema\":");
        emit_Authored_inputDeclaration(w,p->m_context_schema);
    }
    if(p->m_find) {
        if(comma++) raw(w,",");
        raw(w,"\"find\":");
        emit_Authored_questionText(w,p->m_find);
    }
    if(p->m_item_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"item_schema\":");
        emit_Authored_inputDeclaration(w,p->m_item_schema);
    }
    if(p->m_model) {
        if(comma++) raw(w,",");
        raw(w,"\"model\":");
        emit_Authored_name(w,p->m_model);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_Authored_find_member_name(w,p->m_name);
    }
    if(p->m_on) {
        if(comma++) raw(w,",");
        raw(w,"\"on\":");
        emit_Authored_pointers(w,p->m_on);
    }
    if(p->m_profile) {
        if(comma++) raw(w,",");
        raw(w,"\"profile\":");
        emit_Authored_profile(w,p->m_profile);
    }
    if(p->m_wording_version) {
        if(comma++) raw(w,",");
        raw(w,"\"wording_version\":");
        emit_Authored_find_member_wording_version(w,p->m_wording_version);
    }
    raw(w,"}");
}
static inline void emit_Authored_inputDeclaration(writer *w,const void *input) {
    const thinkthen_cobol_InputDeclaration *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_Authored_inputDeclaration_string(w, p->value);
        break;
        case 2: emit_Authored_inputDeclaration_object(w, p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_Authored_inputDeclaration_object(writer *w,const void *input) {
    const thinkthen_cobol_InputDeclarationObject *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_properties) {
        if(comma++) raw(w,",");
        raw(w,"\"properties\":");
        emit_Authored_inputDeclaration_object_member_properties(w,p->m_properties);
    }
    if(p->m_required) {
        if(comma++) raw(w,",");
        raw(w,"\"required\":");
        emit_Authored_inputDeclaration_object_member_required(w,p->m_required);
    }
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"type\":");
        emit_Authored_inputDeclaration_object_member_type(w,p->m_type);
    }
    raw(w,"}");
}
static inline void emit_Authored_inputDeclaration_string(writer *w,const void *input) {
    const thinkthen_cobol_InputDeclarationString *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"type\":");
        emit_Authored_inputDeclaration_string_member_type(w,p->m_type);
    }
    raw(w,"}");
}
static inline void emit_Authored_inputProperty(writer *w,const void *input) {
    const thinkthen_cobol_InputProperty *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_Authored_inputProperty_string(w, p->value);
        break;
        case 2: emit_Authored_inputProperty_number(w, p->value);
        break;
        case 3: emit_Authored_inputProperty_boolean(w, p->value);
        break;
        case 4: emit_Authored_inputProperty_array(w, p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_Authored_inputProperty_array(writer *w,const void *input) {
    const thinkthen_cobol_InputPropertyArray *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_items) {
        if(comma++) raw(w,",");
        raw(w,"\"items\":");
        emit_Authored_inputProperty_array_member_items(w,p->m_items);
    }
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"type\":");
        emit_Authored_inputProperty_array_member_type(w,p->m_type);
    }
    raw(w,"}");
}
static inline void emit_Authored_inputProperty_boolean(writer *w,const void *input) {
    const thinkthen_cobol_InputPropertyBoolean *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"type\":");
        emit_Authored_inputProperty_boolean_member_type(w,p->m_type);
    }
    raw(w,"}");
}
static inline void emit_Authored_inputProperty_number(writer *w,const void *input) {
    const thinkthen_cobol_InputPropertyNumber *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"type\":");
        emit_Authored_inputProperty_number_member_type(w,p->m_type);
    }
    raw(w,"}");
}
static inline void emit_Authored_inputProperty_string(writer *w,const void *input) {
    const thinkthen_cobol_InputPropertyString *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"type\":");
        emit_Authored_inputProperty_string_member_type(w,p->m_type);
    }
    raw(w,"}");
}
static inline void emit_Authored_labels(writer *w,const void *input) {
    const thinkthen_cobol_Labels *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_Authored_labels_arm_1(w,p->value);
        break;
        case 2: emit_Authored_labels_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_Authored_levels(writer *w,const void *input) {
    const thinkthen_cobol_Levels *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_Authored_levels_arm_1(w,p->value);
        break;
        case 2: emit_Authored_levels_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_Authored_name(writer *w,const void *input) {
    const thinkthen_cobol_Name *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_Authored_options(writer *w,const void *input) {
    const thinkthen_cobol_Options *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_Authored_options_arm_1(w,p->value);
        break;
        case 2: emit_Authored_options_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_Authored_pointers(writer *w,const void *input) {
    const thinkthen_cobol_Pointers *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_Authored_pointers_arm_1(w,p->value);
        break;
        case 2: emit_Authored_pointers_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_Authored_profile(writer *w,const void *input) {
    const thinkthen_cobol_Profile *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_Authored_questionText(writer *w,const void *input) {
    const thinkthen_cobol_QuestionText *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_Authored_questionText_arm_1(w,p->value);
        break;
        case 2: emit_Authored_questionText_arm_2(w,p->value);
        break;
        case 3: emit_Authored_questionText_arm_3(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_Authored_relate(writer *w,const void *input) {
    const thinkthen_cobol_Relate *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_context_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"context_schema\":");
        emit_Authored_inputDeclaration(w,p->m_context_schema);
    }
    if(p->m_item_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"item_schema\":");
        emit_Authored_inputDeclaration(w,p->m_item_schema);
    }
    if(p->m_model) {
        if(comma++) raw(w,",");
        raw(w,"\"model\":");
        emit_Authored_name(w,p->m_model);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_Authored_relate_member_name(w,p->m_name);
    }
    if(p->m_profile) {
        if(comma++) raw(w,",");
        raw(w,"\"profile\":");
        emit_Authored_profile(w,p->m_profile);
    }
    if(p->m_relate) {
        if(comma++) raw(w,",");
        raw(w,"\"relate\":");
        emit_RelateQuestion(w,p->m_relate);
    }
    if(p->m_threshold) {
        if(comma++) raw(w,",");
        raw(w,"\"threshold\":");
        emit_Authored_cut(w,p->m_threshold);
    }
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"version\":");
        emit_Authored_relate_member_version(w,p->m_version);
    }
    if(p->m_wording_version) {
        if(comma++) raw(w,",");
        raw(w,"\"wording_version\":");
        emit_Authored_relate_member_wording_version(w,p->m_wording_version);
    }
    raw(w,"}");
}
static inline void emit_Authored_relation(writer *w,const void *input) {
    const thinkthen_cobol_Relation *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_either) {
        if(comma++) raw(w,",");
        raw(w,"\"either\":");
        emit_Authored_relation_member_either(w,p->m_either);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_Authored_name(w,p->m_name);
    }
    if(p->m_reads) {
        if(comma++) raw(w,",");
        raw(w,"\"reads\":");
        emit_Authored_name(w,p->m_reads);
    }
    if(p->m_single) {
        if(comma++) raw(w,",");
        raw(w,"\"single\":");
        emit_Authored_relation_member_single(w,p->m_single);
    }
    if(p->m_source) {
        if(comma++) raw(w,",");
        raw(w,"\"source\":");
        emit_Authored_name(w,p->m_source);
    }
    if(p->m_target) {
        if(comma++) raw(w,",");
        raw(w,"\"target\":");
        emit_Authored_name(w,p->m_target);
    }
    raw(w,"}");
}
static inline void emit_Authored_score(writer *w,const void *input) {
    const thinkthen_cobol_Score *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_batch) {
        if(comma++) raw(w,",");
        raw(w,"\"batch\":");
        emit_Authored_score_member_batch(w,p->m_batch);
    }
    if(p->m_context_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"context_schema\":");
        emit_Authored_inputDeclaration(w,p->m_context_schema);
    }
    if(p->m_item_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"item_schema\":");
        emit_Authored_inputDeclaration(w,p->m_item_schema);
    }
    if(p->m_levels) {
        if(comma++) raw(w,",");
        raw(w,"\"levels\":");
        emit_Authored_levels(w,p->m_levels);
    }
    if(p->m_model) {
        if(comma++) raw(w,",");
        raw(w,"\"model\":");
        emit_Authored_name(w,p->m_model);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_Authored_score_member_name(w,p->m_name);
    }
    if(p->m_on) {
        if(comma++) raw(w,",");
        raw(w,"\"on\":");
        emit_Authored_pointers(w,p->m_on);
    }
    if(p->m_profile) {
        if(comma++) raw(w,",");
        raw(w,"\"profile\":");
        emit_Authored_profile(w,p->m_profile);
    }
    if(p->m_score) {
        if(comma++) raw(w,",");
        raw(w,"\"score\":");
        emit_Authored_questionText(w,p->m_score);
    }
    if(p->m_wording_version) {
        if(comma++) raw(w,",");
        raw(w,"\"wording_version\":");
        emit_Authored_score_member_wording_version(w,p->m_wording_version);
    }
    raw(w,"}");
}
static inline void emit_Authored_tag(writer *w,const void *input) {
    const thinkthen_cobol_Tag *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_batch) {
        if(comma++) raw(w,",");
        raw(w,"\"batch\":");
        emit_Authored_tag_member_batch(w,p->m_batch);
    }
    if(p->m_context_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"context_schema\":");
        emit_Authored_inputDeclaration(w,p->m_context_schema);
    }
    if(p->m_item_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"item_schema\":");
        emit_Authored_inputDeclaration(w,p->m_item_schema);
    }
    if(p->m_labels) {
        if(comma++) raw(w,",");
        raw(w,"\"labels\":");
        emit_Authored_labels(w,p->m_labels);
    }
    if(p->m_model) {
        if(comma++) raw(w,",");
        raw(w,"\"model\":");
        emit_Authored_name(w,p->m_model);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_Authored_tag_member_name(w,p->m_name);
    }
    if(p->m_on) {
        if(comma++) raw(w,",");
        raw(w,"\"on\":");
        emit_Authored_pointers(w,p->m_on);
    }
    if(p->m_profile) {
        if(comma++) raw(w,",");
        raw(w,"\"profile\":");
        emit_Authored_profile(w,p->m_profile);
    }
    if(p->m_tag) {
        if(comma++) raw(w,",");
        raw(w,"\"tag\":");
        emit_Authored_questionText(w,p->m_tag);
    }
    if(p->m_threshold) {
        if(comma++) raw(w,",");
        raw(w,"\"threshold\":");
        emit_Authored_cut(w,p->m_threshold);
    }
    if(p->m_wording_version) {
        if(comma++) raw(w,",");
        raw(w,"\"wording_version\":");
        emit_Authored_tag_member_wording_version(w,p->m_wording_version);
    }
    raw(w,"}");
}
static inline void emit_Authored_threshold(writer *w,const void *input) {
    const thinkthen_cobol_Threshold *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_Authored_threshold_arm_1(w,p->value);
        break;
        case 2: emit_Authored_threshold_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_ContextSchema(writer *w,const void *input) {
    const thinkthen_cobol_ContextSchema *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_ContextSchema_arm_1(w,p->value);
        break;
        case 2: emit_ContextSchema_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_ImageMedia(writer *w,const void *input) {
    const thinkthen_cobol_ImageMedia *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_ImageMedia_arm_1(w,p->value);
        break;
        case 2: emit_ImageMedia_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_OptionSchema(writer *w,const void *input) {
    const thinkthen_cobol_OptionSchema *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_description) {
        if(comma++) raw(w,",");
        raw(w,"\"description\":");
        emit_OptionSchema_member_description(w,p->m_description);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_OptionSchema_member_name(w,p->m_name);
    }
    raw(w,"}");
}
static inline void emit_ReaderMedia(writer *w,const void *input) {
    const thinkthen_cobol_ReaderMedia *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_ReaderMedia_arm_1(w,p->value);
        break;
        case 2: emit_ReaderMedia_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_RecognitionExample(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionExample *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_RecognitionExample_arm_1(w,p->value);
        break;
        case 2: emit_RecognitionExampleText(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_RecognitionExampleEntity(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionExampleEntity *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_end) {
        if(comma++) raw(w,",");
        raw(w,"\"end\":");
        emit_RecognitionExampleEntity_member_end(w,p->m_end);
    }
    if(p->m_kind) {
        if(comma++) raw(w,",");
        raw(w,"\"kind\":");
        emit_RecognitionExampleEntity_member_kind(w,p->m_kind);
    }
    if(p->m_start) {
        if(comma++) raw(w,",");
        raw(w,"\"start\":");
        emit_RecognitionExampleEntity_member_start(w,p->m_start);
    }
    raw(w,"}");
}
static inline void emit_RecognitionExampleText(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionExampleText *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_entities) {
        if(comma++) raw(w,",");
        raw(w,"\"entities\":");
        emit_RecognitionExampleText_member_entities(w,p->m_entities);
    }
    if(p->m_kinds) {
        if(comma++) raw(w,",");
        raw(w,"\"kinds\":");
        emit_RecognitionExampleText_member_kinds(w,p->m_kinds);
    }
    if(p->m_text) {
        if(comma++) raw(w,",");
        raw(w,"\"text\":");
        emit_RecognitionExampleText_member_text(w,p->m_text);
    }
    raw(w,"}");
}
static inline void emit_RecognitionMode(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionMode *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_RecognitionMode_arm_1(w,p->value);
        break;
        case 2: emit_RecognitionMode_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_RecognitionSeedSpan(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionSeedSpan *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_end) {
        if(comma++) raw(w,",");
        raw(w,"\"end\":");
        emit_RecognitionSeedSpan_member_end(w,p->m_end);
    }
    if(p->m_kind) {
        if(comma++) raw(w,",");
        raw(w,"\"kind\":");
        emit_RecognitionSeedSpan_member_kind(w,p->m_kind);
    }
    if(p->m_start) {
        if(comma++) raw(w,",");
        raw(w,"\"start\":");
        emit_RecognitionSeedSpan_member_start(w,p->m_start);
    }
    raw(w,"}");
}
static inline void emit_RecognitionStageContext(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionStageContext *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_boundary) {
        if(comma++) raw(w,",");
        raw(w,"\"boundary\":");
        emit_RecognitionStageContext_member_boundary(w,p->m_boundary);
    }
    if(p->m_kind_edge) {
        if(comma++) raw(w,",");
        raw(w,"\"kind_edge\":");
        emit_RecognitionStageContext_member_kind_edge(w,p->m_kind_edge);
    }
    if(p->m_relation) {
        if(comma++) raw(w,",");
        raw(w,"\"relation\":");
        emit_RecognitionStageContext_member_relation(w,p->m_relation);
    }
    raw(w,"}");
}
static inline void emit_RecognizeDefinition(writer *w,const void *input) {
    const thinkthen_cobol_RecognizeDefinition *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_context_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"context_schema\":");
        emit_Authored_inputDeclaration(w,p->m_context_schema);
    }
    if(p->m_item_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"item_schema\":");
        emit_Authored_inputDeclaration(w,p->m_item_schema);
    }
    if(p->m_model) {
        if(comma++) raw(w,",");
        raw(w,"\"model\":");
        emit_Authored_name(w,p->m_model);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_RecognizeDefinition_member_name(w,p->m_name);
    }
    if(p->m_on) {
        if(comma++) raw(w,",");
        raw(w,"\"on\":");
        emit_Authored_pointers(w,p->m_on);
    }
    if(p->m_profile) {
        if(comma++) raw(w,",");
        raw(w,"\"profile\":");
        emit_Authored_profile(w,p->m_profile);
    }
    if(p->m_recognize) {
        if(comma++) raw(w,",");
        raw(w,"\"recognize\":");
        emit_RecognizeQuestion(w,p->m_recognize);
    }
    if(p->m_relation_threshold) {
        if(comma++) raw(w,",");
        raw(w,"\"relation_threshold\":");
        emit_Authored_cut(w,p->m_relation_threshold);
    }
    if(p->m_threshold) {
        if(comma++) raw(w,",");
        raw(w,"\"threshold\":");
        emit_Authored_cut(w,p->m_threshold);
    }
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"version\":");
        emit_RecognizeDefinition_member_version(w,p->m_version);
    }
    if(p->m_wording_version) {
        if(comma++) raw(w,",");
        raw(w,"\"wording_version\":");
        emit_RecognizeDefinition_member_wording_version(w,p->m_wording_version);
    }
    raw(w,"}");
}
static inline void emit_RecognizeQuestion(writer *w,const void *input) {
    const thinkthen_cobol_RecognizeQuestion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_entity_definition) {
        if(comma++) raw(w,",");
        raw(w,"\"entity_definition\":");
        emit_Authored_questionText(w,p->m_entity_definition);
    }
    if(p->m_instructions) {
        if(comma++) raw(w,",");
        raw(w,"\"instructions\":");
        emit_Authored_questionText(w,p->m_instructions);
    }
    if(p->m_kinds) {
        if(comma++) raw(w,",");
        raw(w,"\"kinds\":");
        emit_RecognizeQuestion_member_kinds(w,p->m_kinds);
    }
    if(p->m_mode) {
        if(comma++) raw(w,",");
        raw(w,"\"mode\":");
        emit_RecognitionMode(w,p->m_mode);
    }
    if(p->m_relations) {
        if(comma++) raw(w,",");
        raw(w,"\"relations\":");
        emit_RecognizeQuestion_member_relations(w,p->m_relations);
    }
    if(p->m_snippet_pieces) {
        if(comma++) raw(w,",");
        raw(w,"\"snippet_pieces\":");
        emit_RecognizeQuestion_member_snippet_pieces(w,p->m_snippet_pieces);
    }
    if(p->m_stage_context) {
        if(comma++) raw(w,",");
        raw(w,"\"stage_context\":");
        emit_RecognitionStageContext(w,p->m_stage_context);
    }
    raw(w,"}");
}
static inline void emit_RelateFields(writer *w,const void *input) {
    const thinkthen_cobol_RelateFields *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_kind) {
        if(comma++) raw(w,",");
        raw(w,"\"kind\":");
        emit_RelateFields_member_kind(w,p->m_kind);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_RelateFields_member_name(w,p->m_name);
    }
    raw(w,"}");
}
static inline void emit_RelateQuestion(writer *w,const void *input) {
    const thinkthen_cobol_RelateQuestion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_fields) {
        if(comma++) raw(w,",");
        raw(w,"\"fields\":");
        emit_RelateFields(w,p->m_fields);
    }
    if(p->m_relations) {
        if(comma++) raw(w,",");
        raw(w,"\"relations\":");
        emit_RelateQuestion_member_relations(w,p->m_relations);
    }
    raw(w,"}");
}
static inline void emit_Request(writer *w,const void *input) {
    const thinkthen_cobol_Request *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_call) {
        if(comma++) raw(w,",");
        raw(w,"\"call\":");
        emit_RequestCall(w,p->m_call);
    }
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"schema\":");
        emit_RequestVersion(w,p->m_schema);
    }
    raw(w,"}");
}
static inline void emit_RequestBatch(writer *w,const void *input) {
    const thinkthen_cobol_RequestBatch *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_RequestBatch_arm_1(w,p->value);
        break;
        case 2: emit_RequestBatch_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_RequestCall(writer *w,const void *input) {
    const thinkthen_cobol_RequestCall *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_function) {
        if(comma++) raw(w,",");
        raw(w,"\"function\":");
        emit_RequestFunction(w,p->m_function);
    }
    if(p->m_input) {
        if(comma++) raw(w,",");
        raw(w,"\"input\":");
        emit_RequestInput(w,p->m_input);
    }
    if(p->m_options) {
        if(comma++) raw(w,",");
        raw(w,"\"options\":");
        emit_RequestOptions(w,p->m_options);
    }
    if(p->m_question) {
        if(comma++) raw(w,",");
        raw(w,"\"question\":");
        emit_RequestQuestion(w,p->m_question);
    }
    raw(w,"}");
}
static inline void emit_RequestDefinition(writer *w,const void *input) {
    const thinkthen_cobol_RequestDefinition *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_RequestDefinition_Authored_decide(w, p->value);
        break;
        case 2: emit_RequestDefinition_Authored_choose(w, p->value);
        break;
        case 3: emit_RequestDefinition_Authored_tag(w, p->value);
        break;
        case 4: emit_RequestDefinition_Authored_score(w, p->value);
        break;
        case 5: emit_RequestDefinition_Authored_relate(w, p->value);
        break;
        case 6: emit_RequestDefinition_Authored_find(w, p->value);
        break;
        case 7: emit_RequestDefinition_RecognizeDefinition(w, p->value);
        break;
        case 8: emit_RequestDefinition_AnnotateDefinition(w, p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_RequestDefinition_AnnotateDefinition(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionAnnotateDefinition *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_batch) {
        if(comma++) raw(w,",");
        raw(w,"\"batch\":");
        emit_RequestDefinition_AnnotateDefinition_member_batch(w,p->m_batch);
    }
    if(p->m_profile) {
        if(comma++) raw(w,",");
        raw(w,"\"profile\":");
        emit_Authored_profile(w,p->m_profile);
    }
    if(p->m_questions) {
        if(comma++) raw(w,",");
        raw(w,"\"questions\":");
        emit_RequestDefinition_AnnotateDefinition_member_questions(w,p->m_questions);
    }
    if(p->m_threshold) {
        if(comma++) raw(w,",");
        raw(w,"\"threshold\":");
        emit_Authored_threshold(w,p->m_threshold);
    }
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"version\":");
        emit_RequestDefinition_AnnotateDefinition_member_version(w,p->m_version);
    }
    raw(w,"}");
}
static inline void emit_RequestDefinition_Authored_choose(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionChoose *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_batch) {
        if(comma++) raw(w,",");
        raw(w,"\"batch\":");
        emit_RequestDefinition_Authored_choose_member_batch(w,p->m_batch);
    }
    if(p->m_choose) {
        if(comma++) raw(w,",");
        raw(w,"\"choose\":");
        emit_Authored_questionText(w,p->m_choose);
    }
    if(p->m_context_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"context_schema\":");
        emit_Authored_inputDeclaration(w,p->m_context_schema);
    }
    if(p->m_item_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"item_schema\":");
        emit_Authored_inputDeclaration(w,p->m_item_schema);
    }
    if(p->m_model) {
        if(comma++) raw(w,",");
        raw(w,"\"model\":");
        emit_Authored_name(w,p->m_model);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_RequestDefinition_Authored_choose_member_name(w,p->m_name);
    }
    if(p->m_on) {
        if(comma++) raw(w,",");
        raw(w,"\"on\":");
        emit_Authored_pointers(w,p->m_on);
    }
    if(p->m_options) {
        if(comma++) raw(w,",");
        raw(w,"\"options\":");
        emit_Authored_options(w,p->m_options);
    }
    if(p->m_profile) {
        if(comma++) raw(w,",");
        raw(w,"\"profile\":");
        emit_Authored_profile(w,p->m_profile);
    }
    if(p->m_threshold) {
        if(comma++) raw(w,",");
        raw(w,"\"threshold\":");
        emit_Authored_cut(w,p->m_threshold);
    }
    if(p->m_wording_version) {
        if(comma++) raw(w,",");
        raw(w,"\"wording_version\":");
        emit_RequestDefinition_Authored_choose_member_wording_version(w,p->m_wording_version);
    }
    raw(w,"}");
}
static inline void emit_RequestDefinition_Authored_decide(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionDecide *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_batch) {
        if(comma++) raw(w,",");
        raw(w,"\"batch\":");
        emit_RequestDefinition_Authored_decide_member_batch(w,p->m_batch);
    }
    if(p->m_context_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"context_schema\":");
        emit_Authored_inputDeclaration(w,p->m_context_schema);
    }
    if(p->m_decide) {
        if(comma++) raw(w,",");
        raw(w,"\"decide\":");
        emit_Authored_questionText(w,p->m_decide);
    }
    if(p->m_false) {
        if(comma++) raw(w,",");
        raw(w,"\"false\":");
        emit_Authored_criterion(w,p->m_false);
    }
    if(p->m_item_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"item_schema\":");
        emit_Authored_inputDeclaration(w,p->m_item_schema);
    }
    if(p->m_model) {
        if(comma++) raw(w,",");
        raw(w,"\"model\":");
        emit_Authored_name(w,p->m_model);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_RequestDefinition_Authored_decide_member_name(w,p->m_name);
    }
    if(p->m_on) {
        if(comma++) raw(w,",");
        raw(w,"\"on\":");
        emit_Authored_pointers(w,p->m_on);
    }
    if(p->m_profile) {
        if(comma++) raw(w,",");
        raw(w,"\"profile\":");
        emit_Authored_profile(w,p->m_profile);
    }
    if(p->m_threshold) {
        if(comma++) raw(w,",");
        raw(w,"\"threshold\":");
        emit_Authored_threshold(w,p->m_threshold);
    }
    if(p->m_true) {
        if(comma++) raw(w,",");
        raw(w,"\"true\":");
        emit_Authored_criterion(w,p->m_true);
    }
    if(p->m_wording_version) {
        if(comma++) raw(w,",");
        raw(w,"\"wording_version\":");
        emit_RequestDefinition_Authored_decide_member_wording_version(w,p->m_wording_version);
    }
    raw(w,"}");
}
static inline void emit_RequestDefinition_Authored_find(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionFind *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_context_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"context_schema\":");
        emit_Authored_inputDeclaration(w,p->m_context_schema);
    }
    if(p->m_find) {
        if(comma++) raw(w,",");
        raw(w,"\"find\":");
        emit_Authored_questionText(w,p->m_find);
    }
    if(p->m_item_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"item_schema\":");
        emit_Authored_inputDeclaration(w,p->m_item_schema);
    }
    if(p->m_model) {
        if(comma++) raw(w,",");
        raw(w,"\"model\":");
        emit_Authored_name(w,p->m_model);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_RequestDefinition_Authored_find_member_name(w,p->m_name);
    }
    if(p->m_on) {
        if(comma++) raw(w,",");
        raw(w,"\"on\":");
        emit_Authored_pointers(w,p->m_on);
    }
    if(p->m_profile) {
        if(comma++) raw(w,",");
        raw(w,"\"profile\":");
        emit_Authored_profile(w,p->m_profile);
    }
    if(p->m_wording_version) {
        if(comma++) raw(w,",");
        raw(w,"\"wording_version\":");
        emit_RequestDefinition_Authored_find_member_wording_version(w,p->m_wording_version);
    }
    raw(w,"}");
}
static inline void emit_RequestDefinition_Authored_relate(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionRelate *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_context_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"context_schema\":");
        emit_Authored_inputDeclaration(w,p->m_context_schema);
    }
    if(p->m_item_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"item_schema\":");
        emit_Authored_inputDeclaration(w,p->m_item_schema);
    }
    if(p->m_model) {
        if(comma++) raw(w,",");
        raw(w,"\"model\":");
        emit_Authored_name(w,p->m_model);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_RequestDefinition_Authored_relate_member_name(w,p->m_name);
    }
    if(p->m_profile) {
        if(comma++) raw(w,",");
        raw(w,"\"profile\":");
        emit_Authored_profile(w,p->m_profile);
    }
    if(p->m_relate) {
        if(comma++) raw(w,",");
        raw(w,"\"relate\":");
        emit_RelateQuestion(w,p->m_relate);
    }
    if(p->m_threshold) {
        if(comma++) raw(w,",");
        raw(w,"\"threshold\":");
        emit_Authored_cut(w,p->m_threshold);
    }
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"version\":");
        emit_RequestDefinition_Authored_relate_member_version(w,p->m_version);
    }
    if(p->m_wording_version) {
        if(comma++) raw(w,",");
        raw(w,"\"wording_version\":");
        emit_RequestDefinition_Authored_relate_member_wording_version(w,p->m_wording_version);
    }
    raw(w,"}");
}
static inline void emit_RequestDefinition_Authored_score(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionScore *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_batch) {
        if(comma++) raw(w,",");
        raw(w,"\"batch\":");
        emit_RequestDefinition_Authored_score_member_batch(w,p->m_batch);
    }
    if(p->m_context_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"context_schema\":");
        emit_Authored_inputDeclaration(w,p->m_context_schema);
    }
    if(p->m_item_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"item_schema\":");
        emit_Authored_inputDeclaration(w,p->m_item_schema);
    }
    if(p->m_levels) {
        if(comma++) raw(w,",");
        raw(w,"\"levels\":");
        emit_Authored_levels(w,p->m_levels);
    }
    if(p->m_model) {
        if(comma++) raw(w,",");
        raw(w,"\"model\":");
        emit_Authored_name(w,p->m_model);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_RequestDefinition_Authored_score_member_name(w,p->m_name);
    }
    if(p->m_on) {
        if(comma++) raw(w,",");
        raw(w,"\"on\":");
        emit_Authored_pointers(w,p->m_on);
    }
    if(p->m_profile) {
        if(comma++) raw(w,",");
        raw(w,"\"profile\":");
        emit_Authored_profile(w,p->m_profile);
    }
    if(p->m_score) {
        if(comma++) raw(w,",");
        raw(w,"\"score\":");
        emit_Authored_questionText(w,p->m_score);
    }
    if(p->m_wording_version) {
        if(comma++) raw(w,",");
        raw(w,"\"wording_version\":");
        emit_RequestDefinition_Authored_score_member_wording_version(w,p->m_wording_version);
    }
    raw(w,"}");
}
static inline void emit_RequestDefinition_Authored_tag(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionTag *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_batch) {
        if(comma++) raw(w,",");
        raw(w,"\"batch\":");
        emit_RequestDefinition_Authored_tag_member_batch(w,p->m_batch);
    }
    if(p->m_context_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"context_schema\":");
        emit_Authored_inputDeclaration(w,p->m_context_schema);
    }
    if(p->m_item_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"item_schema\":");
        emit_Authored_inputDeclaration(w,p->m_item_schema);
    }
    if(p->m_labels) {
        if(comma++) raw(w,",");
        raw(w,"\"labels\":");
        emit_Authored_labels(w,p->m_labels);
    }
    if(p->m_model) {
        if(comma++) raw(w,",");
        raw(w,"\"model\":");
        emit_Authored_name(w,p->m_model);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_RequestDefinition_Authored_tag_member_name(w,p->m_name);
    }
    if(p->m_on) {
        if(comma++) raw(w,",");
        raw(w,"\"on\":");
        emit_Authored_pointers(w,p->m_on);
    }
    if(p->m_profile) {
        if(comma++) raw(w,",");
        raw(w,"\"profile\":");
        emit_Authored_profile(w,p->m_profile);
    }
    if(p->m_tag) {
        if(comma++) raw(w,",");
        raw(w,"\"tag\":");
        emit_Authored_questionText(w,p->m_tag);
    }
    if(p->m_threshold) {
        if(comma++) raw(w,",");
        raw(w,"\"threshold\":");
        emit_Authored_cut(w,p->m_threshold);
    }
    if(p->m_wording_version) {
        if(comma++) raw(w,",");
        raw(w,"\"wording_version\":");
        emit_RequestDefinition_Authored_tag_member_wording_version(w,p->m_wording_version);
    }
    raw(w,"}");
}
static inline void emit_RequestDefinition_RecognizeDefinition(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionRecognizeDefinition *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_context_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"context_schema\":");
        emit_Authored_inputDeclaration(w,p->m_context_schema);
    }
    if(p->m_item_schema) {
        if(comma++) raw(w,",");
        raw(w,"\"item_schema\":");
        emit_Authored_inputDeclaration(w,p->m_item_schema);
    }
    if(p->m_model) {
        if(comma++) raw(w,",");
        raw(w,"\"model\":");
        emit_Authored_name(w,p->m_model);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_RequestDefinition_RecognizeDefinition_member_name(w,p->m_name);
    }
    if(p->m_on) {
        if(comma++) raw(w,",");
        raw(w,"\"on\":");
        emit_Authored_pointers(w,p->m_on);
    }
    if(p->m_profile) {
        if(comma++) raw(w,",");
        raw(w,"\"profile\":");
        emit_Authored_profile(w,p->m_profile);
    }
    if(p->m_recognize) {
        if(comma++) raw(w,",");
        raw(w,"\"recognize\":");
        emit_RecognizeQuestion(w,p->m_recognize);
    }
    if(p->m_relation_threshold) {
        if(comma++) raw(w,",");
        raw(w,"\"relation_threshold\":");
        emit_Authored_cut(w,p->m_relation_threshold);
    }
    if(p->m_threshold) {
        if(comma++) raw(w,",");
        raw(w,"\"threshold\":");
        emit_Authored_cut(w,p->m_threshold);
    }
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"version\":");
        emit_RequestDefinition_RecognizeDefinition_member_version(w,p->m_version);
    }
    if(p->m_wording_version) {
        if(comma++) raw(w,",");
        raw(w,"\"wording_version\":");
        emit_RequestDefinition_RecognizeDefinition_member_wording_version(w,p->m_wording_version);
    }
    raw(w,"}");
}
static inline void emit_RequestFraming(writer *w,const void *input) {
    const thinkthen_cobol_RequestFraming *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_RequestFraming_arm_1(w,p->value);
        break;
        case 2: emit_RequestFraming_arm_2(w,p->value);
        break;
        case 3: emit_RequestFraming_arm_3(w,p->value);
        break;
        case 4: emit_RequestFraming_arm_4(w,p->value);
        break;
        case 5: emit_RequestFraming_arm_5(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_RequestFunction(writer *w,const void *input) {
    const thinkthen_cobol_RequestFunction *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_RequestFunction_arm_1(w,p->value);
        break;
        case 2: emit_RequestFunction_arm_2(w,p->value);
        break;
        case 3: emit_RequestFunction_arm_3(w,p->value);
        break;
        case 4: emit_RequestFunction_arm_4(w,p->value);
        break;
        case 5: emit_RequestFunction_arm_5(w,p->value);
        break;
        case 6: emit_RequestFunction_arm_6(w,p->value);
        break;
        case 7: emit_RequestFunction_arm_7(w,p->value);
        break;
        case 8: emit_RequestFunction_arm_8(w,p->value);
        break;
        case 9: emit_RequestFunction_arm_9(w,p->value);
        break;
        case 10: emit_RequestFunction_arm_10(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_RequestImage(writer *w,const void *input) {
    const thinkthen_cobol_RequestImage *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_RequestImage_file(w, p->value);
        break;
        case 2: emit_RequestImage_bytes(w, p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_RequestImage_bytes(writer *w,const void *input) {
    const thinkthen_cobol_RequestImageBytes *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_bytes) {
        if(comma++) raw(w,",");
        raw(w,"\"bytes\":");
        emit_RequestImage_bytes_member_bytes(w,p->m_bytes);
    }
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"kind\":");
        emit_RequestImage_bytes_member_kind(w,p->m_kind);
    }
    if(p->m_media) {
        if(comma++) raw(w,",");
        raw(w,"\"media\":");
        emit_ImageMedia(w,p->m_media);
    }
    raw(w,"}");
}
static inline void emit_RequestImage_file(writer *w,const void *input) {
    const thinkthen_cobol_RequestImageFile *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"kind\":");
        emit_RequestImage_file_member_kind(w,p->m_kind);
    }
    if(p->m_media) {
        if(comma++) raw(w,",");
        raw(w,"\"media\":");
        emit_ImageMedia(w,p->m_media);
    }
    if(p->m_path) {
        if(comma++) raw(w,",");
        raw(w,"\"path\":");
        emit_RequestImage_file_member_path(w,p->m_path);
    }
    raw(w,"}");
}
static inline void emit_RequestInput(writer *w,const void *input) {
    const thinkthen_cobol_RequestInput *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_RequestInput_text(w, p->value);
        break;
        case 2: emit_RequestInput_json(w, p->value);
        break;
        case 3: emit_RequestInput_records(w, p->value);
        break;
        case 4: emit_RequestInput_units(w, p->value);
        break;
        case 5: emit_RequestInput_entities(w, p->value);
        break;
        case 6: emit_RequestInput_source(w, p->value);
        break;
        case 7: emit_RequestInput_feed(w, p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_RequestInput_entities(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputEntities *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_items) {
        if(comma++) raw(w,",");
        raw(w,"\"items\":");
        emit_RequestInput_entities_member_items(w,p->m_items);
    }
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"kind\":");
        emit_RequestInput_entities_member_kind(w,p->m_kind);
    }
    raw(w,"}");
}
static inline void emit_RequestInput_feed(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputFeed *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_framing) {
        if(comma++) raw(w,",");
        raw(w,"\"framing\":");
        emit_RequestFraming(w,p->m_framing);
    }
    if(p->m_images) {
        if(comma++) raw(w,",");
        raw(w,"\"images\":");
        emit_RequestInput_feed_member_images(w,p->m_images);
    }
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"kind\":");
        emit_RequestInput_feed_member_kind(w,p->m_kind);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_RequestInput_feed_member_name(w,p->m_name);
    }
    if(p->m_reading) {
        if(comma++) raw(w,",");
        raw(w,"\"reading\":");
        emit_RequestReader(w,p->m_reading);
    }
    raw(w,"}");
}
static inline void emit_RequestInput_json(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputJson *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_images) {
        if(comma++) raw(w,",");
        raw(w,"\"images\":");
        emit_RequestInput_json_member_images(w,p->m_images);
    }
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"kind\":");
        emit_RequestInput_json_member_kind(w,p->m_kind);
    }
    if(p->m_value) {
        if(comma++) raw(w,",");
        raw(w,"\"value\":");
        emit_RequestInput_json_member_value(w,p->m_value);
    }
    raw(w,"}");
}
static inline void emit_RequestInput_records(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputRecords *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_items) {
        if(comma++) raw(w,",");
        raw(w,"\"items\":");
        emit_RequestInput_records_member_items(w,p->m_items);
    }
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"kind\":");
        emit_RequestInput_records_member_kind(w,p->m_kind);
    }
    raw(w,"}");
}
static inline void emit_RequestInput_source(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputSource *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"kind\":");
        emit_RequestInput_source_member_kind(w,p->m_kind);
    }
    if(p->m_source) {
        if(comma++) raw(w,",");
        raw(w,"\"source\":");
        emit_RequestSource(w,p->m_source);
    }
    raw(w,"}");
}
static inline void emit_RequestInput_text(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputText *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_images) {
        if(comma++) raw(w,",");
        raw(w,"\"images\":");
        emit_RequestInput_text_member_images(w,p->m_images);
    }
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"kind\":");
        emit_RequestInput_text_member_kind(w,p->m_kind);
    }
    if(p->m_text) {
        if(comma++) raw(w,",");
        raw(w,"\"text\":");
        emit_RequestInput_text_member_text(w,p->m_text);
    }
    raw(w,"}");
}
static inline void emit_RequestInput_units(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputUnits *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_items) {
        if(comma++) raw(w,",");
        raw(w,"\"items\":");
        emit_RequestInput_units_member_items(w,p->m_items);
    }
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"kind\":");
        emit_RequestInput_units_member_kind(w,p->m_kind);
    }
    raw(w,"}");
}
static inline void emit_RequestItem(writer *w,const void *input) {
    const thinkthen_cobol_RequestItem *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_context) {
        if(comma++) raw(w,",");
        raw(w,"\"context\":");
        emit_ContextSchema(w,p->m_context);
    }
    if(p->m_examples) {
        if(comma++) raw(w,",");
        raw(w,"\"examples\":");
        emit_RequestItem_member_examples(w,p->m_examples);
    }
    if(p->m_images) {
        if(comma++) raw(w,",");
        raw(w,"\"images\":");
        emit_RequestItem_member_images(w,p->m_images);
    }
    if(p->m_options) {
        if(comma++) raw(w,",");
        raw(w,"\"options\":");
        emit_RequestItem_member_options(w,p->m_options);
    }
    if(p->m_original) {
        if(comma++) raw(w,",");
        raw(w,"\"original\":");
        emit_RequestOriginal(w,p->m_original);
    }
    if(p->m_seed_spans) {
        if(comma++) raw(w,",");
        raw(w,"\"seed_spans\":");
        emit_RequestItem_member_seed_spans(w,p->m_seed_spans);
    }
    raw(w,"}");
}
static inline void emit_RequestOptions(writer *w,const void *input) {
    const thinkthen_cobol_RequestOptions *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_attempts) {
        if(comma++) raw(w,",");
        raw(w,"\"attempts\":");
        emit_RequestOptions_member_attempts(w,p->m_attempts);
    }
    if(p->m_batch) {
        if(comma++) raw(w,",");
        raw(w,"\"batch\":");
        emit_RequestBatch(w,p->m_batch);
    }
    if(p->m_context) {
        if(comma++) raw(w,",");
        raw(w,"\"context\":");
        emit_RequestOptions_member_context(w,p->m_context);
    }
    if(p->m_context_field) {
        if(comma++) raw(w,",");
        raw(w,"\"context_field\":");
        emit_RequestOptions_member_context_field(w,p->m_context_field);
    }
    if(p->m_deadline_ms) {
        if(comma++) raw(w,",");
        raw(w,"\"deadline_ms\":");
        emit_RequestOptions_member_deadline_ms(w,p->m_deadline_ms);
    }
    if(p->m_details) {
        if(comma++) raw(w,",");
        raw(w,"\"details\":");
        emit_RequestOptions_member_details(w,p->m_details);
    }
    if(p->m_examples) {
        if(comma++) raw(w,",");
        raw(w,"\"examples\":");
        emit_RequestOptions_member_examples(w,p->m_examples);
    }
    if(p->m_examples_field) {
        if(comma++) raw(w,",");
        raw(w,"\"examples_field\":");
        emit_RequestOptions_member_examples_field(w,p->m_examples_field);
    }
    if(p->m_field) {
        if(comma++) raw(w,",");
        raw(w,"\"field\":");
        emit_RequestOptions_member_field(w,p->m_field);
    }
    if(p->m_files_only) {
        if(comma++) raw(w,",");
        raw(w,"\"files_only\":");
        emit_RequestOptions_member_files_only(w,p->m_files_only);
    }
    if(p->m_max_requests_total) {
        if(comma++) raw(w,",");
        raw(w,"\"max_requests_total\":");
        emit_RequestOptions_member_max_requests_total(w,p->m_max_requests_total);
    }
    if(p->m_mode) {
        if(comma++) raw(w,",");
        raw(w,"\"mode\":");
        emit_RecognitionMode(w,p->m_mode);
    }
    if(p->m_model) {
        if(comma++) raw(w,",");
        raw(w,"\"model\":");
        emit_RequestOptions_member_model(w,p->m_model);
    }
    if(p->m_none) {
        if(comma++) raw(w,",");
        raw(w,"\"none\":");
        emit_RequestOptions_member_none(w,p->m_none);
    }
    if(p->m_options_field) {
        if(comma++) raw(w,",");
        raw(w,"\"options_field\":");
        emit_RequestOptions_member_options_field(w,p->m_options_field);
    }
    if(p->m_relation_threshold) {
        if(comma++) raw(w,",");
        raw(w,"\"relation_threshold\":");
        emit_RequestThreshold(w,p->m_relation_threshold);
    }
    if(p->m_seed_spans) {
        if(comma++) raw(w,",");
        raw(w,"\"seed_spans\":");
        emit_RequestOptions_member_seed_spans(w,p->m_seed_spans);
    }
    if(p->m_seed_spans_field) {
        if(comma++) raw(w,",");
        raw(w,"\"seed_spans_field\":");
        emit_RequestOptions_member_seed_spans_field(w,p->m_seed_spans_field);
    }
    if(p->m_snippet_pieces) {
        if(comma++) raw(w,",");
        raw(w,"\"snippet_pieces\":");
        emit_RequestOptions_member_snippet_pieces(w,p->m_snippet_pieces);
    }
    if(p->m_stage_context) {
        if(comma++) raw(w,",");
        raw(w,"\"stage_context\":");
        emit_RecognitionStageContext(w,p->m_stage_context);
    }
    if(p->m_threshold) {
        if(comma++) raw(w,",");
        raw(w,"\"threshold\":");
        emit_RequestThreshold(w,p->m_threshold);
    }
    if(p->m_top) {
        if(comma++) raw(w,",");
        raw(w,"\"top\":");
        emit_RequestOptions_member_top(w,p->m_top);
    }
    raw(w,"}");
}
static inline void emit_RequestOriginal(writer *w,const void *input) {
    const thinkthen_cobol_RequestOriginal *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_RequestOriginal_text(w, p->value);
        break;
        case 2: emit_RequestOriginal_json(w, p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_RequestOriginal_json(writer *w,const void *input) {
    const thinkthen_cobol_RequestOriginalJson *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"kind\":");
        emit_RequestOriginal_json_member_kind(w,p->m_kind);
    }
    if(p->m_value) {
        if(comma++) raw(w,",");
        raw(w,"\"value\":");
        emit_RequestOriginal_json_member_value(w,p->m_value);
    }
    raw(w,"}");
}
static inline void emit_RequestOriginal_text(writer *w,const void *input) {
    const thinkthen_cobol_RequestOriginalText *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"kind\":");
        emit_RequestOriginal_text_member_kind(w,p->m_kind);
    }
    if(p->m_text) {
        if(comma++) raw(w,",");
        raw(w,"\"text\":");
        emit_RequestOriginal_text_member_text(w,p->m_text);
    }
    raw(w,"}");
}
static inline void emit_RequestQuestion(writer *w,const void *input) {
    const thinkthen_cobol_RequestQuestion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_RequestQuestion_text(w, p->value);
        break;
        case 2: emit_RequestQuestion_definition(w, p->value);
        break;
        case 3: emit_RequestQuestion_file(w, p->value);
        break;
        case 4: emit_RequestQuestion_name(w, p->value);
        break;
        case 5: emit_RequestQuestion_reference(w, p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_RequestQuestion_definition(writer *w,const void *input) {
    const thinkthen_cobol_RequestQuestionDefinition *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"kind\":");
        emit_RequestQuestion_definition_member_kind(w,p->m_kind);
    }
    if(p->m_value) {
        if(comma++) raw(w,",");
        raw(w,"\"value\":");
        emit_RequestDefinition(w,p->m_value);
    }
    raw(w,"}");
}
static inline void emit_RequestQuestion_file(writer *w,const void *input) {
    const thinkthen_cobol_RequestQuestionFile *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"kind\":");
        emit_RequestQuestion_file_member_kind(w,p->m_kind);
    }
    if(p->m_path) {
        if(comma++) raw(w,",");
        raw(w,"\"path\":");
        emit_RequestQuestion_file_member_path(w,p->m_path);
    }
    raw(w,"}");
}
static inline void emit_RequestQuestion_name(writer *w,const void *input) {
    const thinkthen_cobol_RequestQuestionName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"kind\":");
        emit_RequestQuestion_name_member_kind(w,p->m_kind);
    }
    if(p->m_name) {
        if(comma++) raw(w,",");
        raw(w,"\"name\":");
        emit_RequestQuestion_name_member_name(w,p->m_name);
    }
    raw(w,"}");
}
static inline void emit_RequestQuestion_reference(writer *w,const void *input) {
    const thinkthen_cobol_RequestQuestionReference *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"kind\":");
        emit_RequestQuestion_reference_member_kind(w,p->m_kind);
    }
    if(p->m_reference) {
        if(comma++) raw(w,",");
        raw(w,"\"reference\":");
        emit_RequestQuestion_reference_member_reference(w,p->m_reference);
    }
    raw(w,"}");
}
static inline void emit_RequestQuestion_text(writer *w,const void *input) {
    const thinkthen_cobol_RequestQuestionText *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"kind\":");
        emit_RequestQuestion_text_member_kind(w,p->m_kind);
    }
    if(p->m_text) {
        if(comma++) raw(w,",");
        raw(w,"\"text\":");
        emit_RequestQuestion_text_member_text(w,p->m_text);
    }
    raw(w,"}");
}
static inline void emit_RequestReader(writer *w,const void *input) {
    const thinkthen_cobol_RequestReader *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_unit) {
        if(comma++) raw(w,",");
        raw(w,"\"unit\":");
        emit_SourceUnit(w,p->m_unit);
    }
    if(p->m_window) {
        if(comma++) raw(w,",");
        raw(w,"\"window\":");
        emit_RequestReader_member_window(w,p->m_window);
    }
    raw(w,"}");
}
static inline void emit_RequestSessionDescriptor(writer *w,const void *input) {
    const thinkthen_cobol_RequestSessionDescriptor *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_item) {
        if(comma++) raw(w,",");
        raw(w,"\"item\":");
        emit_RequestItem(w,p->m_item);
    }
    if(p->m_location) {
        if(comma++) raw(w,",");
        raw(w,"\"location\":");
        emit_SessionSourceLocation(w,p->m_location);
    }
    raw(w,"}");
}
static inline void emit_RequestSource(writer *w,const void *input) {
    const thinkthen_cobol_RequestSource *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_framing) {
        if(comma++) raw(w,",");
        raw(w,"\"framing\":");
        emit_RequestFraming(w,p->m_framing);
    }
    if(p->m_media) {
        if(comma++) raw(w,",");
        raw(w,"\"media\":");
        emit_ReaderMedia(w,p->m_media);
    }
    if(p->m_paths) {
        if(comma++) raw(w,",");
        raw(w,"\"paths\":");
        emit_RequestSource_member_paths(w,p->m_paths);
    }
    if(p->m_reading) {
        if(comma++) raw(w,",");
        raw(w,"\"reading\":");
        emit_RequestReader(w,p->m_reading);
    }
    raw(w,"}");
}
static inline void emit_RequestThreshold(writer *w,const void *input) {
    const thinkthen_cobol_RequestThreshold *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_RequestThreshold_arm_1(w,p->value);
        break;
        case 2: emit_RequestThreshold_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_RequestVersion(writer *w,const void *input) {
    const thinkthen_cobol_RequestVersion *p=input;
    (void)p;
    raw(w,"\"thinkthen.request/1\"");
}
static inline void emit_SessionSourceLocation(writer *w,const void *input) {
    const thinkthen_cobol_SessionSourceLocation *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(p->m_file) {
        if(comma++) raw(w,",");
        raw(w,"\"file\":");
        emit_SessionSourceLocation_member_file(w,p->m_file);
    }
    if(p->m_first_line) {
        if(comma++) raw(w,",");
        raw(w,"\"first_line\":");
        emit_SessionSourceLocation_member_first_line(w,p->m_first_line);
    }
    if(p->m_last_line) {
        if(comma++) raw(w,",");
        raw(w,"\"last_line\":");
        emit_SessionSourceLocation_member_last_line(w,p->m_last_line);
    }
    raw(w,"}");
}
static inline void emit_SourceUnit(writer *w,const void *input) {
    const thinkthen_cobol_SourceUnit *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_SourceUnit_arm_1(w,p->value);
        break;
        case 2: emit_SourceUnit_arm_2(w,p->value);
        break;
        case 3: emit_SourceUnit_arm_3(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_AnnotateDefinition_member_batch(writer *w,const void *input) {
    const thinkthen_cobol_AnnotateDefinitionBatch *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,0);
}
static inline void emit_AnnotateDefinition_member_questions(writer *w,const void *input) {
    const thinkthen_cobol_AnnotateDefinitionQuestions *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"{");
    if(p->len && (!p->keys || !p->values)) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        text(w,p->keys[i].data,p->keys[i].len,1);
        raw(w,":");
        emit_AnnotateQuestion(w,p->values[i]);
    }
    raw(w,"}");
}
static inline void emit_AnnotateDefinition_member_version(writer *w,const void *input) {
    const thinkthen_cobol_AnnotateDefinitionVersion *p=input;
    (void)p;
    raw(w,"1");
}
static inline void emit_AnnotateQuestion_choose_member_name(writer *w,const void *input) {
    const thinkthen_cobol_AnnotateQuestionChooseName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_AnnotateQuestion_choose_member_wording_version(writer *w,const void *input) {
    const thinkthen_cobol_AnnotateQuestionChooseWordingVersion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_AnnotateQuestion_decide_member_name(writer *w,const void *input) {
    const thinkthen_cobol_AnnotateQuestionDecideName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_AnnotateQuestion_decide_member_wording_version(writer *w,const void *input) {
    const thinkthen_cobol_AnnotateQuestionDecideWordingVersion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_AnnotateQuestion_score_member_name(writer *w,const void *input) {
    const thinkthen_cobol_AnnotateQuestionScoreName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_AnnotateQuestion_score_member_wording_version(writer *w,const void *input) {
    const thinkthen_cobol_AnnotateQuestionScoreWordingVersion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_AnnotateQuestion_tag_member_name(writer *w,const void *input) {
    const thinkthen_cobol_AnnotateQuestionTagName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_AnnotateQuestion_tag_member_wording_version(writer *w,const void *input) {
    const thinkthen_cobol_AnnotateQuestionTagWordingVersion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_Authored_choose_member_batch(writer *w,const void *input) {
    const thinkthen_cobol_ChooseBatch *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_Authored_choose_member_batch_arm_1(w,p->value);
        break;
        case 2: emit_Authored_choose_member_batch_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_Authored_choose_member_name(writer *w,const void *input) {
    const thinkthen_cobol_ChooseName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_Authored_choose_member_wording_version(writer *w,const void *input) {
    const thinkthen_cobol_ChooseWordingVersion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_Authored_criterion_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_CriterionArm1 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_Authored_criterion_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_CriterionArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,0);
}
static inline void emit_Authored_criterion_arm_3(writer *w,const void *input) {
    const thinkthen_cobol_CriterionArm3 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_Authored_criterion_arm_3_item(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_Authored_criterion_arm_4(writer *w,const void *input) {
    const thinkthen_cobol_CriterionArm4 *p=input;
    (void)p;
    raw(w,"null");
}
static inline void emit_Authored_cut_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_CutArm1 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    if(!isfinite(p->value)) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    snprintf(bytes,sizeof(bytes),"%.17g",p->value);
    raw(w,bytes);
}
static inline void emit_Authored_cut_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_CutArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_Authored_decide_member_batch(writer *w,const void *input) {
    const thinkthen_cobol_DecideBatch *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_Authored_decide_member_batch_arm_1(w,p->value);
        break;
        case 2: emit_Authored_decide_member_batch_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_Authored_decide_member_name(writer *w,const void *input) {
    const thinkthen_cobol_DecideName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_Authored_decide_member_wording_version(writer *w,const void *input) {
    const thinkthen_cobol_DecideWordingVersion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_Authored_description_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_DescriptionArm1 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_Authored_description_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_DescriptionArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,0);
}
static inline void emit_Authored_description_arm_3(writer *w,const void *input) {
    const thinkthen_cobol_DescriptionArm3 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_Authored_description_arm_3_item(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_Authored_description_arm_4(writer *w,const void *input) {
    const thinkthen_cobol_DescriptionArm4 *p=input;
    (void)p;
    raw(w,"null");
}
static inline void emit_Authored_find_member_name(writer *w,const void *input) {
    const thinkthen_cobol_FindName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_Authored_find_member_wording_version(writer *w,const void *input) {
    const thinkthen_cobol_FindWordingVersion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_Authored_inputDeclaration_object_member_properties(writer *w,const void *input) {
    const thinkthen_cobol_InputDeclarationObjectProperties *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"{");
    if(p->len && (!p->keys || !p->values)) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        text(w,p->keys[i].data,p->keys[i].len,1);
        raw(w,":");
        emit_Authored_inputProperty(w,p->values[i]);
    }
    raw(w,"}");
}
static inline void emit_Authored_inputDeclaration_object_member_required(writer *w,const void *input) {
    const thinkthen_cobol_InputDeclarationObjectRequired *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_Authored_inputDeclaration_object_member_required_item(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_Authored_inputDeclaration_object_member_type(writer *w,const void *input) {
    const thinkthen_cobol_InputDeclarationObjectType *p=input;
    (void)p;
    raw(w,"\"object\"");
}
static inline void emit_Authored_inputDeclaration_string_member_type(writer *w,const void *input) {
    const thinkthen_cobol_InputDeclarationStringType *p=input;
    (void)p;
    raw(w,"\"string\"");
}
static inline void emit_Authored_inputProperty_array_member_items(writer *w,const void *input) {
    const thinkthen_cobol_InputPropertyArrayItems *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    int comma=0;
    raw(w,"{");
    if(1) {
        if(comma++) raw(w,",");
        raw(w,"\"type\":");
        emit_Authored_inputProperty_array_member_items_member_type(w,p->m_type);
    }
    raw(w,"}");
}
static inline void emit_Authored_inputProperty_array_member_type(writer *w,const void *input) {
    const thinkthen_cobol_InputPropertyArrayType *p=input;
    (void)p;
    raw(w,"\"array\"");
}
static inline void emit_Authored_inputProperty_boolean_member_type(writer *w,const void *input) {
    const thinkthen_cobol_InputPropertyBooleanType *p=input;
    (void)p;
    raw(w,"\"boolean\"");
}
static inline void emit_Authored_inputProperty_number_member_type(writer *w,const void *input) {
    const thinkthen_cobol_InputPropertyNumberType *p=input;
    (void)p;
    raw(w,"\"number\"");
}
static inline void emit_Authored_inputProperty_string_member_type(writer *w,const void *input) {
    const thinkthen_cobol_InputPropertyStringType *p=input;
    (void)p;
    raw(w,"\"string\"");
}
static inline void emit_Authored_labels_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_LabelsArm1 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_Authored_name(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_Authored_labels_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_LabelsArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"{");
    if(p->len && (!p->keys || !p->values)) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        text(w,p->keys[i].data,p->keys[i].len,1);
        raw(w,":");
        emit_Authored_description(w,p->values[i]);
    }
    raw(w,"}");
}
static inline void emit_Authored_levels_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_LevelsArm1 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_Authored_name(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_Authored_levels_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_LevelsArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"{");
    if(p->len && (!p->keys || !p->values)) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        text(w,p->keys[i].data,p->keys[i].len,1);
        raw(w,":");
        emit_Authored_criterion(w,p->values[i]);
    }
    raw(w,"}");
}
static inline void emit_Authored_options_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_OptionsArm1 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_Authored_name(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_Authored_options_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_OptionsArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"{");
    if(p->len && (!p->keys || !p->values)) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        text(w,p->keys[i].data,p->keys[i].len,1);
        raw(w,":");
        emit_Authored_description(w,p->values[i]);
    }
    raw(w,"}");
}
static inline void emit_Authored_pointers_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_PointersArm1 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_Authored_pointers_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_PointersArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_Authored_pointers_arm_2_item(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_Authored_questionText_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_QuestionTextArm1 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_Authored_questionText_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_QuestionTextArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,0);
}
static inline void emit_Authored_questionText_arm_3(writer *w,const void *input) {
    const thinkthen_cobol_QuestionTextArm3 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_Authored_questionText_arm_3_item(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_Authored_relate_member_name(writer *w,const void *input) {
    const thinkthen_cobol_RelateName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_Authored_relate_member_version(writer *w,const void *input) {
    const thinkthen_cobol_RelateVersion *p=input;
    (void)p;
    raw(w,"1");
}
static inline void emit_Authored_relate_member_wording_version(writer *w,const void *input) {
    const thinkthen_cobol_RelateWordingVersion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_Authored_relation_member_either(writer *w,const void *input) {
    const thinkthen_cobol_RelationEither *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,p->value ? "true" : "false");
}
static inline void emit_Authored_relation_member_single(writer *w,const void *input) {
    const thinkthen_cobol_RelationSingle *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,p->value ? "true" : "false");
}
static inline void emit_Authored_score_member_batch(writer *w,const void *input) {
    const thinkthen_cobol_ScoreBatch *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_Authored_score_member_batch_arm_1(w,p->value);
        break;
        case 2: emit_Authored_score_member_batch_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_Authored_score_member_name(writer *w,const void *input) {
    const thinkthen_cobol_ScoreName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_Authored_score_member_wording_version(writer *w,const void *input) {
    const thinkthen_cobol_ScoreWordingVersion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_Authored_tag_member_batch(writer *w,const void *input) {
    const thinkthen_cobol_TagBatch *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_Authored_tag_member_batch_arm_1(w,p->value);
        break;
        case 2: emit_Authored_tag_member_batch_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_Authored_tag_member_name(writer *w,const void *input) {
    const thinkthen_cobol_TagName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_Authored_tag_member_wording_version(writer *w,const void *input) {
    const thinkthen_cobol_TagWordingVersion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_Authored_threshold_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_ThresholdArm1 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    if(!isfinite(p->value)) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    snprintf(bytes,sizeof(bytes),"%.17g",p->value);
    raw(w,bytes);
}
static inline void emit_Authored_threshold_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_ThresholdArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_ContextSchema_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_ContextSchemaArm1 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_ContextSchema_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_ContextSchemaArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,0);
}
static inline void emit_ImageMedia_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_ImageMediaArm1 *p=input;
    (void)p;
    raw(w,"\"image/jpeg\"");
}
static inline void emit_ImageMedia_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_ImageMediaArm2 *p=input;
    (void)p;
    raw(w,"\"image/png\"");
}
static inline void emit_OptionSchema_member_description(writer *w,const void *input) {
    const thinkthen_cobol_OptionSchemaDescription *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,0);
}
static inline void emit_OptionSchema_member_name(writer *w,const void *input) {
    const thinkthen_cobol_OptionSchemaName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_ReaderMedia_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_ReaderMediaArm1 *p=input;
    (void)p;
    raw(w,"\"text\"");
}
static inline void emit_ReaderMedia_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_ReaderMediaArm2 *p=input;
    (void)p;
    raw(w,"\"image\"");
}
static inline void emit_RecognitionExample_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionExampleArm1 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RecognitionExampleEntity_member_end(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionExampleEntityEnd *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value);
    raw(w,bytes);
}
static inline void emit_RecognitionExampleEntity_member_kind(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionExampleEntityKind *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RecognitionExampleEntity_member_start(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionExampleEntityStart *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value);
    raw(w,bytes);
}
static inline void emit_RecognitionExampleText_member_entities(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionExampleTextEntities *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_RecognitionExampleEntity(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_RecognitionExampleText_member_kinds(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionExampleTextKinds *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_RecognitionExampleText_member_kinds_item(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_RecognitionExampleText_member_text(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionExampleTextText *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RecognitionMode_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionModeArm1 *p=input;
    (void)p;
    raw(w,"\"whole\"");
}
static inline void emit_RecognitionMode_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionModeArm2 *p=input;
    (void)p;
    raw(w,"\"boundary_only\"");
}
static inline void emit_RecognitionSeedSpan_member_end(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionSeedSpanEnd *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value);
    raw(w,bytes);
}
static inline void emit_RecognitionSeedSpan_member_kind(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionSeedSpanKind *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RecognitionSeedSpan_member_start(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionSeedSpanStart *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value);
    raw(w,bytes);
}
static inline void emit_RecognitionStageContext_member_boundary(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionStageContextBoundary *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RecognitionStageContext_member_kind_edge(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionStageContextKindEdge *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RecognitionStageContext_member_relation(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionStageContextRelation *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RecognizeDefinition_member_name(writer *w,const void *input) {
    const thinkthen_cobol_RecognizeDefinitionName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RecognizeDefinition_member_version(writer *w,const void *input) {
    const thinkthen_cobol_RecognizeDefinitionVersion *p=input;
    (void)p;
    raw(w,"1");
}
static inline void emit_RecognizeDefinition_member_wording_version(writer *w,const void *input) {
    const thinkthen_cobol_RecognizeDefinitionWordingVersion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_RecognizeQuestion_member_kinds(writer *w,const void *input) {
    const thinkthen_cobol_RecognizeQuestionKinds *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"{");
    if(p->len && (!p->keys || !p->values)) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        text(w,p->keys[i].data,p->keys[i].len,1);
        raw(w,":");
        emit_Authored_description(w,p->values[i]);
    }
    raw(w,"}");
}
static inline void emit_RecognizeQuestion_member_relations(writer *w,const void *input) {
    const thinkthen_cobol_RecognizeQuestionRelations *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_Authored_relation(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_RecognizeQuestion_member_snippet_pieces(writer *w,const void *input) {
    const thinkthen_cobol_RecognizeQuestionSnippetPieces *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value);
    raw(w,bytes);
}
static inline void emit_RelateFields_member_kind(writer *w,const void *input) {
    const thinkthen_cobol_RelateFieldsKind *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RelateFields_member_name(writer *w,const void *input) {
    const thinkthen_cobol_RelateFieldsName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RelateQuestion_member_relations(writer *w,const void *input) {
    const thinkthen_cobol_RelateQuestionRelations *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_Authored_relation(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_RequestBatch_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_RequestBatchArm1 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value);
    raw(w,bytes);
}
static inline void emit_RequestBatch_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_RequestBatchArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_AnnotateDefinition_member_batch(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionAnnotateDefinitionBatch *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,0);
}
static inline void emit_RequestDefinition_AnnotateDefinition_member_questions(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionAnnotateDefinitionQuestions *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"{");
    if(p->len && (!p->keys || !p->values)) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        text(w,p->keys[i].data,p->keys[i].len,1);
        raw(w,":");
        emit_AnnotateQuestion(w,p->values[i]);
    }
    raw(w,"}");
}
static inline void emit_RequestDefinition_AnnotateDefinition_member_version(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionAnnotateDefinitionVersion *p=input;
    (void)p;
    raw(w,"1");
}
static inline void emit_RequestDefinition_Authored_choose_member_batch(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionChooseBatch *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_RequestDefinition_Authored_choose_member_batch_arm_1(w,p->value);
        break;
        case 2: emit_RequestDefinition_Authored_choose_member_batch_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_RequestDefinition_Authored_choose_member_name(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionChooseName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_Authored_choose_member_wording_version(writer *w,
const void *input) {
    const thinkthen_cobol_DefinitionChooseWordingVersion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_RequestDefinition_Authored_decide_member_batch(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionDecideBatch *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_RequestDefinition_Authored_decide_member_batch_arm_1(w,p->value);
        break;
        case 2: emit_RequestDefinition_Authored_decide_member_batch_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_RequestDefinition_Authored_decide_member_name(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionDecideName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_Authored_decide_member_wording_version(writer *w,
const void *input) {
    const thinkthen_cobol_DefinitionDecideWordingVersion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_RequestDefinition_Authored_find_member_name(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionFindName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_Authored_find_member_wording_version(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionFindWordingVersion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_RequestDefinition_Authored_relate_member_name(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionRelateName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_Authored_relate_member_version(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionRelateVersion *p=input;
    (void)p;
    raw(w,"1");
}
static inline void emit_RequestDefinition_Authored_relate_member_wording_version(writer *w,
const void *input) {
    const thinkthen_cobol_DefinitionRelateWordingVersion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_RequestDefinition_Authored_score_member_batch(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionScoreBatch *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_RequestDefinition_Authored_score_member_batch_arm_1(w,p->value);
        break;
        case 2: emit_RequestDefinition_Authored_score_member_batch_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_RequestDefinition_Authored_score_member_name(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionScoreName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_Authored_score_member_wording_version(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionScoreWordingVersion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_RequestDefinition_Authored_tag_member_batch(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionTagBatch *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    switch(p->kind) {
        case 1: emit_RequestDefinition_Authored_tag_member_batch_arm_1(w,p->value);
        break;
        case 2: emit_RequestDefinition_Authored_tag_member_batch_arm_2(w,p->value);
        break;
        default: w->code=THINKTHEN_COBOL_REPRESENTATION;
        break;
    }
}
static inline void emit_RequestDefinition_Authored_tag_member_name(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionTagName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_Authored_tag_member_wording_version(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionTagWordingVersion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_RequestDefinition_RecognizeDefinition_member_name(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionRecognizeDefinitionName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_RecognizeDefinition_member_version(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionRecognizeDefinitionVersion *p=input;
    (void)p;
    raw(w,"1");
}
static inline void emit_RequestDefinition_RecognizeDefinition_member_wording_version(writer *w,
const void *input) {
    const thinkthen_cobol_DefinitionRecognizeDefinitionWordingVersion *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_RequestFraming_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_RequestFramingArm1 *p=input;
    (void)p;
    raw(w,"\"document\"");
}
static inline void emit_RequestFraming_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_RequestFramingArm2 *p=input;
    (void)p;
    raw(w,"\"lines\"");
}
static inline void emit_RequestFraming_arm_3(writer *w,const void *input) {
    const thinkthen_cobol_RequestFramingArm3 *p=input;
    (void)p;
    raw(w,"\"jsonl\"");
}
static inline void emit_RequestFraming_arm_4(writer *w,const void *input) {
    const thinkthen_cobol_RequestFramingArm4 *p=input;
    (void)p;
    raw(w,"\"csv\"");
}
static inline void emit_RequestFraming_arm_5(writer *w,const void *input) {
    const thinkthen_cobol_RequestFramingArm5 *p=input;
    (void)p;
    raw(w,"\"tsv\"");
}
static inline void emit_RequestFunction_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_RequestFunctionArm1 *p=input;
    (void)p;
    raw(w,"\"decide\"");
}
static inline void emit_RequestFunction_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_RequestFunctionArm2 *p=input;
    (void)p;
    raw(w,"\"choose\"");
}
static inline void emit_RequestFunction_arm_3(writer *w,const void *input) {
    const thinkthen_cobol_RequestFunctionArm3 *p=input;
    (void)p;
    raw(w,"\"tag\"");
}
static inline void emit_RequestFunction_arm_4(writer *w,const void *input) {
    const thinkthen_cobol_RequestFunctionArm4 *p=input;
    (void)p;
    raw(w,"\"score\"");
}
static inline void emit_RequestFunction_arm_5(writer *w,const void *input) {
    const thinkthen_cobol_RequestFunctionArm5 *p=input;
    (void)p;
    raw(w,"\"filter\"");
}
static inline void emit_RequestFunction_arm_6(writer *w,const void *input) {
    const thinkthen_cobol_RequestFunctionArm6 *p=input;
    (void)p;
    raw(w,"\"rank\"");
}
static inline void emit_RequestFunction_arm_7(writer *w,const void *input) {
    const thinkthen_cobol_RequestFunctionArm7 *p=input;
    (void)p;
    raw(w,"\"find\"");
}
static inline void emit_RequestFunction_arm_8(writer *w,const void *input) {
    const thinkthen_cobol_RequestFunctionArm8 *p=input;
    (void)p;
    raw(w,"\"annotate\"");
}
static inline void emit_RequestFunction_arm_9(writer *w,const void *input) {
    const thinkthen_cobol_RequestFunctionArm9 *p=input;
    (void)p;
    raw(w,"\"recognize\"");
}
static inline void emit_RequestFunction_arm_10(writer *w,const void *input) {
    const thinkthen_cobol_RequestFunctionArm10 *p=input;
    (void)p;
    raw(w,"\"relate\"");
}
static inline void emit_RequestImage_bytes_member_bytes(writer *w,const void *input) {
    const thinkthen_cobol_RequestImageBytesBytes *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestImage_bytes_member_kind(writer *w,const void *input) {
    const thinkthen_cobol_RequestImageBytesKind *p=input;
    (void)p;
    raw(w,"\"bytes\"");
}
static inline void emit_RequestImage_file_member_kind(writer *w,const void *input) {
    const thinkthen_cobol_RequestImageFileKind *p=input;
    (void)p;
    raw(w,"\"file\"");
}
static inline void emit_RequestImage_file_member_path(writer *w,const void *input) {
    const thinkthen_cobol_RequestImageFilePath *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestInput_entities_member_items(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputEntitiesItems *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_RequestItem(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_RequestInput_entities_member_kind(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputEntitiesKind *p=input;
    (void)p;
    raw(w,"\"entities\"");
}
static inline void emit_RequestInput_feed_member_images(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputFeedImages *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_RequestImage(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_RequestInput_feed_member_kind(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputFeedKind *p=input;
    (void)p;
    raw(w,"\"feed\"");
}
static inline void emit_RequestInput_feed_member_name(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputFeedName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestInput_json_member_images(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputJsonImages *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_RequestImage(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_RequestInput_json_member_kind(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputJsonKind *p=input;
    (void)p;
    raw(w,"\"json\"");
}
static inline void emit_RequestInput_json_member_value(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputJsonValue *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,0);
}
static inline void emit_RequestInput_records_member_items(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputRecordsItems *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_RequestItem(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_RequestInput_records_member_kind(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputRecordsKind *p=input;
    (void)p;
    raw(w,"\"records\"");
}
static inline void emit_RequestInput_source_member_kind(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputSourceKind *p=input;
    (void)p;
    raw(w,"\"source\"");
}
static inline void emit_RequestInput_text_member_images(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputTextImages *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_RequestImage(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_RequestInput_text_member_kind(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputTextKind *p=input;
    (void)p;
    raw(w,"\"text\"");
}
static inline void emit_RequestInput_text_member_text(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputTextText *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestInput_units_member_items(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputUnitsItems *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_RequestItem(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_RequestInput_units_member_kind(writer *w,const void *input) {
    const thinkthen_cobol_RequestInputUnitsKind *p=input;
    (void)p;
    raw(w,"\"units\"");
}
static inline void emit_RequestItem_member_examples(writer *w,const void *input) {
    const thinkthen_cobol_RequestItemExamples *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_RecognitionExample(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_RequestItem_member_images(writer *w,const void *input) {
    const thinkthen_cobol_RequestItemImages *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_RequestImage(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_RequestItem_member_options(writer *w,const void *input) {
    const thinkthen_cobol_RequestItemOptions *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_OptionSchema(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_RequestItem_member_seed_spans(writer *w,const void *input) {
    const thinkthen_cobol_RequestItemSeedSpans *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_RecognitionSeedSpan(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_RequestOptions_member_attempts(writer *w,const void *input) {
    const thinkthen_cobol_RequestOptionsAttempts *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,p->value ? "true" : "false");
}
static inline void emit_RequestOptions_member_context(writer *w,const void *input) {
    const thinkthen_cobol_RequestOptionsContext *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestOptions_member_context_field(writer *w,const void *input) {
    const thinkthen_cobol_RequestOptionsContextField *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestOptions_member_deadline_ms(writer *w,const void *input) {
    const thinkthen_cobol_RequestOptionsDeadlineMs *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_RequestOptions_member_details(writer *w,const void *input) {
    const thinkthen_cobol_RequestOptionsDetails *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,p->value ? "true" : "false");
}
static inline void emit_RequestOptions_member_examples(writer *w,const void *input) {
    const thinkthen_cobol_RequestOptionsExamples *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_RecognitionExample(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_RequestOptions_member_examples_field(writer *w,const void *input) {
    const thinkthen_cobol_RequestOptionsExamplesField *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestOptions_member_field(writer *w,const void *input) {
    const thinkthen_cobol_RequestOptionsField *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_RequestOptions_member_field_item(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_RequestOptions_member_files_only(writer *w,const void *input) {
    const thinkthen_cobol_RequestOptionsFilesOnly *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,p->value ? "true" : "false");
}
static inline void emit_RequestOptions_member_max_requests_total(writer *w,const void *input) {
    const thinkthen_cobol_RequestOptionsMaxRequestsTotal *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value);
    raw(w,bytes);
}
static inline void emit_RequestOptions_member_model(writer *w,const void *input) {
    const thinkthen_cobol_RequestOptionsModel *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestOptions_member_none(writer *w,const void *input) {
    const thinkthen_cobol_RequestOptionsNone *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,p->value ? "true" : "false");
}
static inline void emit_RequestOptions_member_options_field(writer *w,const void *input) {
    const thinkthen_cobol_RequestOptionsOptionsField *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestOptions_member_seed_spans(writer *w,const void *input) {
    const thinkthen_cobol_RequestOptionsSeedSpans *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_RecognitionSeedSpan(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_RequestOptions_member_seed_spans_field(writer *w,const void *input) {
    const thinkthen_cobol_RequestOptionsSeedSpansField *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestOptions_member_snippet_pieces(writer *w,const void *input) {
    const thinkthen_cobol_RequestOptionsSnippetPieces *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value);
    raw(w,bytes);
}
static inline void emit_RequestOptions_member_top(writer *w,const void *input) {
    const thinkthen_cobol_RequestOptionsTop *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value);
    raw(w,bytes);
}
static inline void emit_RequestOriginal_json_member_kind(writer *w,const void *input) {
    const thinkthen_cobol_RequestOriginalJsonKind *p=input;
    (void)p;
    raw(w,"\"json\"");
}
static inline void emit_RequestOriginal_json_member_value(writer *w,const void *input) {
    const thinkthen_cobol_RequestOriginalJsonValue *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,0);
}
static inline void emit_RequestOriginal_text_member_kind(writer *w,const void *input) {
    const thinkthen_cobol_RequestOriginalTextKind *p=input;
    (void)p;
    raw(w,"\"text\"");
}
static inline void emit_RequestOriginal_text_member_text(writer *w,const void *input) {
    const thinkthen_cobol_RequestOriginalTextText *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestQuestion_definition_member_kind(writer *w,const void *input) {
    const thinkthen_cobol_RequestQuestionDefinitionKind *p=input;
    (void)p;
    raw(w,"\"definition\"");
}
static inline void emit_RequestQuestion_file_member_kind(writer *w,const void *input) {
    const thinkthen_cobol_RequestQuestionFileKind *p=input;
    (void)p;
    raw(w,"\"file\"");
}
static inline void emit_RequestQuestion_file_member_path(writer *w,const void *input) {
    const thinkthen_cobol_RequestQuestionFilePath *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestQuestion_name_member_kind(writer *w,const void *input) {
    const thinkthen_cobol_RequestQuestionNameKind *p=input;
    (void)p;
    raw(w,"\"name\"");
}
static inline void emit_RequestQuestion_name_member_name(writer *w,const void *input) {
    const thinkthen_cobol_RequestQuestionNameName *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestQuestion_reference_member_kind(writer *w,const void *input) {
    const thinkthen_cobol_RequestQuestionReferenceKind *p=input;
    (void)p;
    raw(w,"\"reference\"");
}
static inline void emit_RequestQuestion_reference_member_reference(writer *w,const void *input) {
    const thinkthen_cobol_RequestQuestionReferenceReference *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestQuestion_text_member_kind(writer *w,const void *input) {
    const thinkthen_cobol_RequestQuestionTextKind *p=input;
    (void)p;
    raw(w,"\"text\"");
}
static inline void emit_RequestQuestion_text_member_text(writer *w,const void *input) {
    const thinkthen_cobol_RequestQuestionTextText *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestReader_member_window(writer *w,const void *input) {
    const thinkthen_cobol_RequestReaderWindow *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value);
    raw(w,bytes);
}
static inline void emit_RequestSource_member_paths(writer *w,const void *input) {
    const thinkthen_cobol_RequestSourcePaths *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    raw(w,"[");
    if(p->len && !p->data) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    for(uint64_t i=0;
    i<p->len && !w->code;
    i++) {
        if(i) raw(w,",");
        emit_RequestSource_member_paths_item(w,p->data[i]);
    }
    raw(w,"]");
}
static inline void emit_RequestThreshold_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_RequestThresholdArm1 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    if(!isfinite(p->value)) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    snprintf(bytes,sizeof(bytes),"%.17g",p->value);
    raw(w,bytes);
}
static inline void emit_RequestThreshold_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_RequestThresholdArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_SessionSourceLocation_member_file(writer *w,const void *input) {
    const thinkthen_cobol_SessionSourceLocationFile *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_SessionSourceLocation_member_first_line(writer *w,const void *input) {
    const thinkthen_cobol_SessionSourceLocationFirstLine *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value);
    raw(w,bytes);
}
static inline void emit_SessionSourceLocation_member_last_line(writer *w,const void *input) {
    const thinkthen_cobol_SessionSourceLocationLastLine *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value);
    raw(w,bytes);
}
static inline void emit_SourceUnit_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_SourceUnitArm1 *p=input;
    (void)p;
    raw(w,"\"line\"");
}
static inline void emit_SourceUnit_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_SourceUnitArm2 *p=input;
    (void)p;
    raw(w,"\"window\"");
}
static inline void emit_SourceUnit_arm_3(writer *w,const void *input) {
    const thinkthen_cobol_SourceUnitArm3 *p=input;
    (void)p;
    raw(w,"\"file\"");
}
static inline void emit_Authored_choose_member_batch_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_ChooseBatchArm1 *p=input;
    (void)p;
    raw(w,"\"max\"");
}
static inline void emit_Authored_choose_member_batch_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_ChooseBatchArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_Authored_criterion_arm_3_item(writer *w,const void *input) {
    const thinkthen_cobol_CriterionArm3Item *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,0);
}
static inline void emit_Authored_decide_member_batch_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_DecideBatchArm1 *p=input;
    (void)p;
    raw(w,"\"max\"");
}
static inline void emit_Authored_decide_member_batch_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_DecideBatchArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_Authored_description_arm_3_item(writer *w,const void *input) {
    const thinkthen_cobol_DescriptionArm3Item *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,0);
}
static inline void emit_Authored_inputDeclaration_object_member_required_item(writer *w,const void *input) {
    const thinkthen_cobol_InputDeclarationObjectRequiredItem *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_Authored_inputProperty_array_member_items_member_type(writer *w,const void *input) {
    const thinkthen_cobol_InputPropertyArrayItemsType *p=input;
    (void)p;
    raw(w,"\"string\"");
}
static inline void emit_Authored_pointers_arm_2_item(writer *w,const void *input) {
    const thinkthen_cobol_PointersArm2Item *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_Authored_questionText_arm_3_item(writer *w,const void *input) {
    const thinkthen_cobol_QuestionTextArm3Item *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,0);
}
static inline void emit_Authored_score_member_batch_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_ScoreBatchArm1 *p=input;
    (void)p;
    raw(w,"\"max\"");
}
static inline void emit_Authored_score_member_batch_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_ScoreBatchArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_Authored_tag_member_batch_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_TagBatchArm1 *p=input;
    (void)p;
    raw(w,"\"max\"");
}
static inline void emit_Authored_tag_member_batch_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_TagBatchArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_RecognitionExampleText_member_kinds_item(writer *w,const void *input) {
    const thinkthen_cobol_RecognitionExampleTextKindsItem *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_Authored_choose_member_batch_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionChooseBatchArm1 *p=input;
    (void)p;
    raw(w,"\"max\"");
}
static inline void emit_RequestDefinition_Authored_choose_member_batch_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionChooseBatchArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_RequestDefinition_Authored_decide_member_batch_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionDecideBatchArm1 *p=input;
    (void)p;
    raw(w,"\"max\"");
}
static inline void emit_RequestDefinition_Authored_decide_member_batch_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionDecideBatchArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_RequestDefinition_Authored_score_member_batch_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionScoreBatchArm1 *p=input;
    (void)p;
    raw(w,"\"max\"");
}
static inline void emit_RequestDefinition_Authored_score_member_batch_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionScoreBatchArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_RequestDefinition_Authored_tag_member_batch_arm_1(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionTagBatchArm1 *p=input;
    (void)p;
    raw(w,"\"max\"");
}
static inline void emit_RequestDefinition_Authored_tag_member_batch_arm_2(writer *w,const void *input) {
    const thinkthen_cobol_DefinitionTagBatchArm2 *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    char bytes[64];
    snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value);
    raw(w,bytes);
}
static inline void emit_RequestOptions_member_field_item(writer *w,const void *input) {
    const thinkthen_cobol_RequestOptionsFieldItem *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
static inline void emit_RequestSource_member_paths_item(writer *w,const void *input) {
    const thinkthen_cobol_RequestSourcePathsItem *p=input;
    if(!p) {
        w->code=THINKTHEN_COBOL_REPRESENTATION;
        return;
    }
    text(w,p->data,p->len,1);
}
int TT_SESSION_REQUEST(const void *engine,const thinkthen_cobol_Request *request,void **out) {
    writer w={
        0
    };
    int encoded=encode(&w,request,emit_Request);
    int code=encoded ? encoded : thinkthen_session_new_with_surface(engine,w.data,w.len,"cobol",
    5,(thinkthen_session **)out);
    free(w.data);
    return code;
}
int TT_SESSION_PUSH(const void *session,const thinkthen_cobol_RequestSessionDescriptor *descriptor,
uint32_t *status) {
    writer w={
        0
    };
    int encoded=encode(&w,descriptor,emit_RequestSessionDescriptor);
    int code=encoded ? encoded : thinkthen_session_try_push((thinkthen_session *)session,w.data,
    w.len,status);
    free(w.data);
    return code;
}
int TT_SESSION_DECIDE(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    thinkthen_cobol_RequestFunction function={
        .kind=1,.value=NULL
    };
    if(!input) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_RequestCall call=*input;
    call.m_function=&function;
    thinkthen_cobol_Request request={
        .m_call=&call
    };
    return TT_SESSION_REQUEST(engine,&request,out);
}
int TT_SESSION_CHOOSE(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    thinkthen_cobol_RequestFunction function={
        .kind=2,.value=NULL
    };
    if(!input) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_RequestCall call=*input;
    call.m_function=&function;
    thinkthen_cobol_Request request={
        .m_call=&call
    };
    return TT_SESSION_REQUEST(engine,&request,out);
}
int TT_SESSION_TAG(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    thinkthen_cobol_RequestFunction function={
        .kind=3,.value=NULL
    };
    if(!input) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_RequestCall call=*input;
    call.m_function=&function;
    thinkthen_cobol_Request request={
        .m_call=&call
    };
    return TT_SESSION_REQUEST(engine,&request,out);
}
int TT_SESSION_SCORE(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    thinkthen_cobol_RequestFunction function={
        .kind=4,.value=NULL
    };
    if(!input) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_RequestCall call=*input;
    call.m_function=&function;
    thinkthen_cobol_Request request={
        .m_call=&call
    };
    return TT_SESSION_REQUEST(engine,&request,out);
}
int TT_SESSION_FILTER(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    thinkthen_cobol_RequestFunction function={
        .kind=5,.value=NULL
    };
    if(!input) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_RequestCall call=*input;
    call.m_function=&function;
    thinkthen_cobol_Request request={
        .m_call=&call
    };
    return TT_SESSION_REQUEST(engine,&request,out);
}
int TT_SESSION_RANK(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    thinkthen_cobol_RequestFunction function={
        .kind=6,.value=NULL
    };
    if(!input) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_RequestCall call=*input;
    call.m_function=&function;
    thinkthen_cobol_Request request={
        .m_call=&call
    };
    return TT_SESSION_REQUEST(engine,&request,out);
}
int TT_SESSION_FIND(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    thinkthen_cobol_RequestFunction function={
        .kind=7,.value=NULL
    };
    if(!input) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_RequestCall call=*input;
    call.m_function=&function;
    thinkthen_cobol_Request request={
        .m_call=&call
    };
    return TT_SESSION_REQUEST(engine,&request,out);
}
int TT_SESSION_ANNOTATE(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    thinkthen_cobol_RequestFunction function={
        .kind=8,.value=NULL
    };
    if(!input) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_RequestCall call=*input;
    call.m_function=&function;
    thinkthen_cobol_Request request={
        .m_call=&call
    };
    return TT_SESSION_REQUEST(engine,&request,out);
}
int TT_SESSION_RECOGNIZE(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    thinkthen_cobol_RequestFunction function={
        .kind=9,.value=NULL
    };
    if(!input) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_RequestCall call=*input;
    call.m_function=&function;
    thinkthen_cobol_Request request={
        .m_call=&call
    };
    return TT_SESSION_REQUEST(engine,&request,out);
}
int TT_SESSION_RELATE(const void *engine,const thinkthen_cobol_RequestCall *input,void **out) {
    thinkthen_cobol_RequestFunction function={
        .kind=10,.value=NULL
    };
    if(!input) return THINKTHEN_COBOL_REPRESENTATION;
    thinkthen_cobol_RequestCall call=*input;
    call.m_function=&function;
    thinkthen_cobol_Request request={
        .m_call=&call
    };
    return TT_SESSION_REQUEST(engine,&request,out);
}
