/* Generated mechanical Request conversion; Rust owns admission. */
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
static inline void emit_Request(writer *,const void *);
static inline void emit_RequestBatch(writer *,const void *);
static inline void emit_RequestCall(writer *,const void *);
static inline void emit_RequestCall_annotate(writer *,const void *);
static inline void emit_RequestCall_choose(writer *,const void *);
static inline void emit_RequestCall_decide(writer *,const void *);
static inline void emit_RequestCall_filter(writer *,const void *);
static inline void emit_RequestCall_find(writer *,const void *);
static inline void emit_RequestCall_rank(writer *,const void *);
static inline void emit_RequestCall_recognize(writer *,const void *);
static inline void emit_RequestCall_relate(writer *,const void *);
static inline void emit_RequestCall_score(writer *,const void *);
static inline void emit_RequestCall_tag(writer *,const void *);
static inline void emit_RequestDefinition(writer *,const void *);
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties(writer *,const void *);
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose(writer *,const void *);
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide(writer *,const void *);
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score(writer *,const void *);
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag(writer *,const void *);
static inline void emit_RequestDefinition_fields_choose(writer *,const void *);
static inline void emit_RequestDefinition_fields_decide(writer *,const void *);
static inline void emit_RequestDefinition_fields_find(writer *,const void *);
static inline void emit_RequestDefinition_fields_questions_version(writer *,const void *);
static inline void emit_RequestDefinition_fields_recognize_version(writer *,const void *);
static inline void emit_RequestDefinition_fields_relate_version(writer *,const void *);
static inline void emit_RequestDefinition_fields_score(writer *,const void *);
static inline void emit_RequestDefinition_fields_tag(writer *,const void *);
static inline void emit_RequestFraming(writer *,const void *);
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
static inline void emit_Authored_relate_member_relate(writer *,const void *);
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
static inline void emit_RequestBatch_arm_1(writer *,const void *);
static inline void emit_RequestBatch_arm_2(writer *,const void *);
static inline void emit_RequestCall_annotate_member_function(writer *,const void *);
static inline void emit_RequestCall_choose_member_function(writer *,const void *);
static inline void emit_RequestCall_decide_member_function(writer *,const void *);
static inline void emit_RequestCall_filter_member_function(writer *,const void *);
static inline void emit_RequestCall_find_member_function(writer *,const void *);
static inline void emit_RequestCall_rank_member_function(writer *,const void *);
static inline void emit_RequestCall_recognize_member_function(writer *,const void *);
static inline void emit_RequestCall_relate_member_function(writer *,const void *);
static inline void emit_RequestCall_score_member_function(writer *,const void *);
static inline void emit_RequestCall_tag_member_function(writer *,const void *);
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_member_name(writer *,const void *);
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_member_wording_version(writer *,const void *);
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_member_name(writer *,const void *);
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_member_wording_version(writer *,const void *);
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_member_name(writer *,const void *);
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_member_wording_version(writer *,const void *);
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_member_name(writer *,const void *);
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_member_wording_version(writer *,const void *);
static inline void emit_RequestDefinition_fields_choose_member_batch(writer *,const void *);
static inline void emit_RequestDefinition_fields_choose_member_name(writer *,const void *);
static inline void emit_RequestDefinition_fields_choose_member_wording_version(writer *,const void *);
static inline void emit_RequestDefinition_fields_decide_member_batch(writer *,const void *);
static inline void emit_RequestDefinition_fields_decide_member_name(writer *,const void *);
static inline void emit_RequestDefinition_fields_decide_member_wording_version(writer *,const void *);
static inline void emit_RequestDefinition_fields_find_member_name(writer *,const void *);
static inline void emit_RequestDefinition_fields_find_member_wording_version(writer *,const void *);
static inline void emit_RequestDefinition_fields_questions_version_member_batch(writer *,const void *);
static inline void emit_RequestDefinition_fields_questions_version_member_questions(writer *,const void *);
static inline void emit_RequestDefinition_fields_questions_version_member_version(writer *,const void *);
static inline void emit_RequestDefinition_fields_recognize_version_member_name(writer *,const void *);
static inline void emit_RequestDefinition_fields_recognize_version_member_recognize(writer *,const void *);
static inline void emit_RequestDefinition_fields_recognize_version_member_version(writer *,const void *);
static inline void emit_RequestDefinition_fields_recognize_version_member_wording_version(writer *,const void *);
static inline void emit_RequestDefinition_fields_relate_version_member_name(writer *,const void *);
static inline void emit_RequestDefinition_fields_relate_version_member_relate(writer *,const void *);
static inline void emit_RequestDefinition_fields_relate_version_member_version(writer *,const void *);
static inline void emit_RequestDefinition_fields_relate_version_member_wording_version(writer *,const void *);
static inline void emit_RequestDefinition_fields_score_member_batch(writer *,const void *);
static inline void emit_RequestDefinition_fields_score_member_name(writer *,const void *);
static inline void emit_RequestDefinition_fields_score_member_wording_version(writer *,const void *);
static inline void emit_RequestDefinition_fields_tag_member_batch(writer *,const void *);
static inline void emit_RequestDefinition_fields_tag_member_name(writer *,const void *);
static inline void emit_RequestDefinition_fields_tag_member_wording_version(writer *,const void *);
static inline void emit_RequestFraming_arm_1(writer *,const void *);
static inline void emit_RequestFraming_arm_2(writer *,const void *);
static inline void emit_RequestFraming_arm_3(writer *,const void *);
static inline void emit_RequestFraming_arm_4(writer *,const void *);
static inline void emit_RequestFraming_arm_5(writer *,const void *);
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
static inline void emit_Authored_relate_member_relate_member_fields(writer *,const void *);
static inline void emit_Authored_relate_member_relate_member_relations(writer *,const void *);
static inline void emit_Authored_score_member_batch_arm_1(writer *,const void *);
static inline void emit_Authored_score_member_batch_arm_2(writer *,const void *);
static inline void emit_Authored_tag_member_batch_arm_1(writer *,const void *);
static inline void emit_Authored_tag_member_batch_arm_2(writer *,const void *);
static inline void emit_RecognitionExampleText_member_kinds_item(writer *,const void *);
static inline void emit_RequestDefinition_fields_choose_member_batch_arm_1(writer *,const void *);
static inline void emit_RequestDefinition_fields_choose_member_batch_arm_2(writer *,const void *);
static inline void emit_RequestDefinition_fields_decide_member_batch_arm_1(writer *,const void *);
static inline void emit_RequestDefinition_fields_decide_member_batch_arm_2(writer *,const void *);
static inline void emit_RequestDefinition_fields_recognize_version_member_recognize_member_kinds(writer *,const void *);
static inline void emit_RequestDefinition_fields_recognize_version_member_recognize_member_relations(writer *,const void *);
static inline void emit_RequestDefinition_fields_recognize_version_member_recognize_member_snippet_pieces(writer *,const void *);
static inline void emit_RequestDefinition_fields_relate_version_member_relate_member_fields(writer *,const void *);
static inline void emit_RequestDefinition_fields_relate_version_member_relate_member_relations(writer *,const void *);
static inline void emit_RequestDefinition_fields_score_member_batch_arm_1(writer *,const void *);
static inline void emit_RequestDefinition_fields_score_member_batch_arm_2(writer *,const void *);
static inline void emit_RequestDefinition_fields_tag_member_batch_arm_1(writer *,const void *);
static inline void emit_RequestDefinition_fields_tag_member_batch_arm_2(writer *,const void *);
static inline void emit_RequestOptions_member_field_item(writer *,const void *);
static inline void emit_RequestSource_member_paths_item(writer *,const void *);
static inline void emit_Authored_relate_member_relate_member_fields_member_kind(writer *,const void *);
static inline void emit_Authored_relate_member_relate_member_fields_member_name(writer *,const void *);
static inline void emit_RequestDefinition_fields_relate_version_member_relate_member_fields_member_kind(writer *,const void *);
static inline void emit_RequestDefinition_fields_relate_version_member_relate_member_fields_member_name(writer *,const void *);
static inline void emit_Authored_choose(writer *w,const void *input) { const thinkthen_cobol_Authored_choose *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_batch) { if(comma++) raw(w,","); raw(w,"\"batch\":"); emit_Authored_choose_member_batch(w,p->m_batch); }
if(p->m_choose) { if(comma++) raw(w,","); raw(w,"\"choose\":"); emit_Authored_questionText(w,p->m_choose); }
if(p->m_context_schema) { if(comma++) raw(w,","); raw(w,"\"context_schema\":"); emit_Authored_inputDeclaration(w,p->m_context_schema); }
if(p->m_item_schema) { if(comma++) raw(w,","); raw(w,"\"item_schema\":"); emit_Authored_inputDeclaration(w,p->m_item_schema); }
if(p->m_model) { if(comma++) raw(w,","); raw(w,"\"model\":"); emit_Authored_name(w,p->m_model); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_Authored_choose_member_name(w,p->m_name); }
if(p->m_on) { if(comma++) raw(w,","); raw(w,"\"on\":"); emit_Authored_pointers(w,p->m_on); }
if(p->m_options) { if(comma++) raw(w,","); raw(w,"\"options\":"); emit_Authored_options(w,p->m_options); }
if(p->m_profile) { if(comma++) raw(w,","); raw(w,"\"profile\":"); emit_Authored_profile(w,p->m_profile); }
if(p->m_threshold) { if(comma++) raw(w,","); raw(w,"\"threshold\":"); emit_Authored_cut(w,p->m_threshold); }
if(p->m_wording_version) { if(comma++) raw(w,","); raw(w,"\"wording_version\":"); emit_Authored_choose_member_wording_version(w,p->m_wording_version); }
raw(w,"}");
}
static inline void emit_Authored_criterion(writer *w,const void *input) { const thinkthen_cobol_Authored_criterion *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_Authored_criterion_arm_1(w,p->value); break;
case 2: emit_Authored_criterion_arm_2(w,p->value); break;
case 3: emit_Authored_criterion_arm_3(w,p->value); break;
case 4: emit_Authored_criterion_arm_4(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_Authored_cut(writer *w,const void *input) { const thinkthen_cobol_Authored_cut *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_Authored_cut_arm_1(w,p->value); break;
case 2: emit_Authored_cut_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_Authored_decide(writer *w,const void *input) { const thinkthen_cobol_Authored_decide *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_batch) { if(comma++) raw(w,","); raw(w,"\"batch\":"); emit_Authored_decide_member_batch(w,p->m_batch); }
if(p->m_context_schema) { if(comma++) raw(w,","); raw(w,"\"context_schema\":"); emit_Authored_inputDeclaration(w,p->m_context_schema); }
if(p->m_decide) { if(comma++) raw(w,","); raw(w,"\"decide\":"); emit_Authored_questionText(w,p->m_decide); }
if(p->m_false) { if(comma++) raw(w,","); raw(w,"\"false\":"); emit_Authored_criterion(w,p->m_false); }
if(p->m_item_schema) { if(comma++) raw(w,","); raw(w,"\"item_schema\":"); emit_Authored_inputDeclaration(w,p->m_item_schema); }
if(p->m_model) { if(comma++) raw(w,","); raw(w,"\"model\":"); emit_Authored_name(w,p->m_model); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_Authored_decide_member_name(w,p->m_name); }
if(p->m_on) { if(comma++) raw(w,","); raw(w,"\"on\":"); emit_Authored_pointers(w,p->m_on); }
if(p->m_profile) { if(comma++) raw(w,","); raw(w,"\"profile\":"); emit_Authored_profile(w,p->m_profile); }
if(p->m_threshold) { if(comma++) raw(w,","); raw(w,"\"threshold\":"); emit_Authored_threshold(w,p->m_threshold); }
if(p->m_true) { if(comma++) raw(w,","); raw(w,"\"true\":"); emit_Authored_criterion(w,p->m_true); }
if(p->m_wording_version) { if(comma++) raw(w,","); raw(w,"\"wording_version\":"); emit_Authored_decide_member_wording_version(w,p->m_wording_version); }
raw(w,"}");
}
static inline void emit_Authored_description(writer *w,const void *input) { const thinkthen_cobol_Authored_description *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_Authored_description_arm_1(w,p->value); break;
case 2: emit_Authored_description_arm_2(w,p->value); break;
case 3: emit_Authored_description_arm_3(w,p->value); break;
case 4: emit_Authored_description_arm_4(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_Authored_find(writer *w,const void *input) { const thinkthen_cobol_Authored_find *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_context_schema) { if(comma++) raw(w,","); raw(w,"\"context_schema\":"); emit_Authored_inputDeclaration(w,p->m_context_schema); }
if(p->m_find) { if(comma++) raw(w,","); raw(w,"\"find\":"); emit_Authored_questionText(w,p->m_find); }
if(p->m_item_schema) { if(comma++) raw(w,","); raw(w,"\"item_schema\":"); emit_Authored_inputDeclaration(w,p->m_item_schema); }
if(p->m_model) { if(comma++) raw(w,","); raw(w,"\"model\":"); emit_Authored_name(w,p->m_model); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_Authored_find_member_name(w,p->m_name); }
if(p->m_on) { if(comma++) raw(w,","); raw(w,"\"on\":"); emit_Authored_pointers(w,p->m_on); }
if(p->m_profile) { if(comma++) raw(w,","); raw(w,"\"profile\":"); emit_Authored_profile(w,p->m_profile); }
if(p->m_wording_version) { if(comma++) raw(w,","); raw(w,"\"wording_version\":"); emit_Authored_find_member_wording_version(w,p->m_wording_version); }
raw(w,"}");
}
static inline void emit_Authored_inputDeclaration(writer *w,const void *input) { const thinkthen_cobol_Authored_inputDeclaration *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_Authored_inputDeclaration_string(w, p->value); break;
case 2: emit_Authored_inputDeclaration_object(w, p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_Authored_inputDeclaration_object(writer *w,const void *input) { const thinkthen_cobol_Authored_inputDeclaration_object *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_properties) { if(comma++) raw(w,","); raw(w,"\"properties\":"); emit_Authored_inputDeclaration_object_member_properties(w,p->m_properties); }
if(p->m_required) { if(comma++) raw(w,","); raw(w,"\"required\":"); emit_Authored_inputDeclaration_object_member_required(w,p->m_required); }
if(1) { if(comma++) raw(w,","); raw(w,"\"type\":"); emit_Authored_inputDeclaration_object_member_type(w,p->m_type); }
raw(w,"}");
}
static inline void emit_Authored_inputDeclaration_string(writer *w,const void *input) { const thinkthen_cobol_Authored_inputDeclaration_string *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"type\":"); emit_Authored_inputDeclaration_string_member_type(w,p->m_type); }
raw(w,"}");
}
static inline void emit_Authored_inputProperty(writer *w,const void *input) { const thinkthen_cobol_Authored_inputProperty *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_Authored_inputProperty_string(w, p->value); break;
case 2: emit_Authored_inputProperty_number(w, p->value); break;
case 3: emit_Authored_inputProperty_boolean(w, p->value); break;
case 4: emit_Authored_inputProperty_array(w, p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_Authored_inputProperty_array(writer *w,const void *input) { const thinkthen_cobol_Authored_inputProperty_array *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_items) { if(comma++) raw(w,","); raw(w,"\"items\":"); emit_Authored_inputProperty_array_member_items(w,p->m_items); }
if(1) { if(comma++) raw(w,","); raw(w,"\"type\":"); emit_Authored_inputProperty_array_member_type(w,p->m_type); }
raw(w,"}");
}
static inline void emit_Authored_inputProperty_boolean(writer *w,const void *input) { const thinkthen_cobol_Authored_inputProperty_boolean *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"type\":"); emit_Authored_inputProperty_boolean_member_type(w,p->m_type); }
raw(w,"}");
}
static inline void emit_Authored_inputProperty_number(writer *w,const void *input) { const thinkthen_cobol_Authored_inputProperty_number *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"type\":"); emit_Authored_inputProperty_number_member_type(w,p->m_type); }
raw(w,"}");
}
static inline void emit_Authored_inputProperty_string(writer *w,const void *input) { const thinkthen_cobol_Authored_inputProperty_string *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"type\":"); emit_Authored_inputProperty_string_member_type(w,p->m_type); }
raw(w,"}");
}
static inline void emit_Authored_labels(writer *w,const void *input) { const thinkthen_cobol_Authored_labels *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_Authored_labels_arm_1(w,p->value); break;
case 2: emit_Authored_labels_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_Authored_levels(writer *w,const void *input) { const thinkthen_cobol_Authored_levels *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_Authored_levels_arm_1(w,p->value); break;
case 2: emit_Authored_levels_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_Authored_name(writer *w,const void *input) { const thinkthen_cobol_Authored_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_Authored_options(writer *w,const void *input) { const thinkthen_cobol_Authored_options *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_Authored_options_arm_1(w,p->value); break;
case 2: emit_Authored_options_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_Authored_pointers(writer *w,const void *input) { const thinkthen_cobol_Authored_pointers *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_Authored_pointers_arm_1(w,p->value); break;
case 2: emit_Authored_pointers_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_Authored_profile(writer *w,const void *input) { const thinkthen_cobol_Authored_profile *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_Authored_questionText(writer *w,const void *input) { const thinkthen_cobol_Authored_questionText *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_Authored_questionText_arm_1(w,p->value); break;
case 2: emit_Authored_questionText_arm_2(w,p->value); break;
case 3: emit_Authored_questionText_arm_3(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_Authored_relate(writer *w,const void *input) { const thinkthen_cobol_Authored_relate *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_context_schema) { if(comma++) raw(w,","); raw(w,"\"context_schema\":"); emit_Authored_inputDeclaration(w,p->m_context_schema); }
if(p->m_item_schema) { if(comma++) raw(w,","); raw(w,"\"item_schema\":"); emit_Authored_inputDeclaration(w,p->m_item_schema); }
if(p->m_model) { if(comma++) raw(w,","); raw(w,"\"model\":"); emit_Authored_name(w,p->m_model); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_Authored_relate_member_name(w,p->m_name); }
if(p->m_profile) { if(comma++) raw(w,","); raw(w,"\"profile\":"); emit_Authored_profile(w,p->m_profile); }
if(p->m_relate) { if(comma++) raw(w,","); raw(w,"\"relate\":"); emit_Authored_relate_member_relate(w,p->m_relate); }
if(p->m_threshold) { if(comma++) raw(w,","); raw(w,"\"threshold\":"); emit_Authored_cut(w,p->m_threshold); }
if(1) { if(comma++) raw(w,","); raw(w,"\"version\":"); emit_Authored_relate_member_version(w,p->m_version); }
if(p->m_wording_version) { if(comma++) raw(w,","); raw(w,"\"wording_version\":"); emit_Authored_relate_member_wording_version(w,p->m_wording_version); }
raw(w,"}");
}
static inline void emit_Authored_relation(writer *w,const void *input) { const thinkthen_cobol_Authored_relation *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_either) { if(comma++) raw(w,","); raw(w,"\"either\":"); emit_Authored_relation_member_either(w,p->m_either); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_Authored_name(w,p->m_name); }
if(p->m_reads) { if(comma++) raw(w,","); raw(w,"\"reads\":"); emit_Authored_name(w,p->m_reads); }
if(p->m_single) { if(comma++) raw(w,","); raw(w,"\"single\":"); emit_Authored_relation_member_single(w,p->m_single); }
if(p->m_source) { if(comma++) raw(w,","); raw(w,"\"source\":"); emit_Authored_name(w,p->m_source); }
if(p->m_target) { if(comma++) raw(w,","); raw(w,"\"target\":"); emit_Authored_name(w,p->m_target); }
raw(w,"}");
}
static inline void emit_Authored_score(writer *w,const void *input) { const thinkthen_cobol_Authored_score *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_batch) { if(comma++) raw(w,","); raw(w,"\"batch\":"); emit_Authored_score_member_batch(w,p->m_batch); }
if(p->m_context_schema) { if(comma++) raw(w,","); raw(w,"\"context_schema\":"); emit_Authored_inputDeclaration(w,p->m_context_schema); }
if(p->m_item_schema) { if(comma++) raw(w,","); raw(w,"\"item_schema\":"); emit_Authored_inputDeclaration(w,p->m_item_schema); }
if(p->m_levels) { if(comma++) raw(w,","); raw(w,"\"levels\":"); emit_Authored_levels(w,p->m_levels); }
if(p->m_model) { if(comma++) raw(w,","); raw(w,"\"model\":"); emit_Authored_name(w,p->m_model); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_Authored_score_member_name(w,p->m_name); }
if(p->m_on) { if(comma++) raw(w,","); raw(w,"\"on\":"); emit_Authored_pointers(w,p->m_on); }
if(p->m_profile) { if(comma++) raw(w,","); raw(w,"\"profile\":"); emit_Authored_profile(w,p->m_profile); }
if(p->m_score) { if(comma++) raw(w,","); raw(w,"\"score\":"); emit_Authored_questionText(w,p->m_score); }
if(p->m_wording_version) { if(comma++) raw(w,","); raw(w,"\"wording_version\":"); emit_Authored_score_member_wording_version(w,p->m_wording_version); }
raw(w,"}");
}
static inline void emit_Authored_tag(writer *w,const void *input) { const thinkthen_cobol_Authored_tag *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_batch) { if(comma++) raw(w,","); raw(w,"\"batch\":"); emit_Authored_tag_member_batch(w,p->m_batch); }
if(p->m_context_schema) { if(comma++) raw(w,","); raw(w,"\"context_schema\":"); emit_Authored_inputDeclaration(w,p->m_context_schema); }
if(p->m_item_schema) { if(comma++) raw(w,","); raw(w,"\"item_schema\":"); emit_Authored_inputDeclaration(w,p->m_item_schema); }
if(p->m_labels) { if(comma++) raw(w,","); raw(w,"\"labels\":"); emit_Authored_labels(w,p->m_labels); }
if(p->m_model) { if(comma++) raw(w,","); raw(w,"\"model\":"); emit_Authored_name(w,p->m_model); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_Authored_tag_member_name(w,p->m_name); }
if(p->m_on) { if(comma++) raw(w,","); raw(w,"\"on\":"); emit_Authored_pointers(w,p->m_on); }
if(p->m_profile) { if(comma++) raw(w,","); raw(w,"\"profile\":"); emit_Authored_profile(w,p->m_profile); }
if(p->m_tag) { if(comma++) raw(w,","); raw(w,"\"tag\":"); emit_Authored_questionText(w,p->m_tag); }
if(p->m_threshold) { if(comma++) raw(w,","); raw(w,"\"threshold\":"); emit_Authored_cut(w,p->m_threshold); }
if(p->m_wording_version) { if(comma++) raw(w,","); raw(w,"\"wording_version\":"); emit_Authored_tag_member_wording_version(w,p->m_wording_version); }
raw(w,"}");
}
static inline void emit_Authored_threshold(writer *w,const void *input) { const thinkthen_cobol_Authored_threshold *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_Authored_threshold_arm_1(w,p->value); break;
case 2: emit_Authored_threshold_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_ContextSchema(writer *w,const void *input) { const thinkthen_cobol_ContextSchema *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_ContextSchema_arm_1(w,p->value); break;
case 2: emit_ContextSchema_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_ImageMedia(writer *w,const void *input) { const thinkthen_cobol_ImageMedia *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_ImageMedia_arm_1(w,p->value); break;
case 2: emit_ImageMedia_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_OptionSchema(writer *w,const void *input) { const thinkthen_cobol_OptionSchema *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_description) { if(comma++) raw(w,","); raw(w,"\"description\":"); emit_OptionSchema_member_description(w,p->m_description); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_OptionSchema_member_name(w,p->m_name); }
raw(w,"}");
}
static inline void emit_ReaderMedia(writer *w,const void *input) { const thinkthen_cobol_ReaderMedia *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_ReaderMedia_arm_1(w,p->value); break;
case 2: emit_ReaderMedia_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_RecognitionExample(writer *w,const void *input) { const thinkthen_cobol_RecognitionExample *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_RecognitionExample_arm_1(w,p->value); break;
case 2: emit_RecognitionExampleText(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_RecognitionExampleEntity(writer *w,const void *input) { const thinkthen_cobol_RecognitionExampleEntity *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_end) { if(comma++) raw(w,","); raw(w,"\"end\":"); emit_RecognitionExampleEntity_member_end(w,p->m_end); }
if(p->m_kind) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_RecognitionExampleEntity_member_kind(w,p->m_kind); }
if(p->m_start) { if(comma++) raw(w,","); raw(w,"\"start\":"); emit_RecognitionExampleEntity_member_start(w,p->m_start); }
raw(w,"}");
}
static inline void emit_RecognitionExampleText(writer *w,const void *input) { const thinkthen_cobol_RecognitionExampleText *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_entities) { if(comma++) raw(w,","); raw(w,"\"entities\":"); emit_RecognitionExampleText_member_entities(w,p->m_entities); }
if(p->m_kinds) { if(comma++) raw(w,","); raw(w,"\"kinds\":"); emit_RecognitionExampleText_member_kinds(w,p->m_kinds); }
if(p->m_text) { if(comma++) raw(w,","); raw(w,"\"text\":"); emit_RecognitionExampleText_member_text(w,p->m_text); }
raw(w,"}");
}
static inline void emit_RecognitionMode(writer *w,const void *input) { const thinkthen_cobol_RecognitionMode *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_RecognitionMode_arm_1(w,p->value); break;
case 2: emit_RecognitionMode_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_RecognitionSeedSpan(writer *w,const void *input) { const thinkthen_cobol_RecognitionSeedSpan *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_end) { if(comma++) raw(w,","); raw(w,"\"end\":"); emit_RecognitionSeedSpan_member_end(w,p->m_end); }
if(p->m_kind) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_RecognitionSeedSpan_member_kind(w,p->m_kind); }
if(p->m_start) { if(comma++) raw(w,","); raw(w,"\"start\":"); emit_RecognitionSeedSpan_member_start(w,p->m_start); }
raw(w,"}");
}
static inline void emit_RecognitionStageContext(writer *w,const void *input) { const thinkthen_cobol_RecognitionStageContext *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_boundary) { if(comma++) raw(w,","); raw(w,"\"boundary\":"); emit_RecognitionStageContext_member_boundary(w,p->m_boundary); }
if(p->m_kind_edge) { if(comma++) raw(w,","); raw(w,"\"kind_edge\":"); emit_RecognitionStageContext_member_kind_edge(w,p->m_kind_edge); }
if(p->m_relation) { if(comma++) raw(w,","); raw(w,"\"relation\":"); emit_RecognitionStageContext_member_relation(w,p->m_relation); }
raw(w,"}");
}
static inline void emit_Request(writer *w,const void *input) { const thinkthen_cobol_Request *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_call) { if(comma++) raw(w,","); raw(w,"\"call\":"); emit_RequestCall(w,p->m_call); }
if(1) { if(comma++) raw(w,","); raw(w,"\"schema\":"); emit_RequestVersion(w,p->m_schema); }
raw(w,"}");
}
static inline void emit_RequestBatch(writer *w,const void *input) { const thinkthen_cobol_RequestBatch *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_RequestBatch_arm_1(w,p->value); break;
case 2: emit_RequestBatch_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_RequestCall(writer *w,const void *input) { const thinkthen_cobol_RequestCall *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_RequestCall_decide(w, p->value); break;
case 2: emit_RequestCall_choose(w, p->value); break;
case 3: emit_RequestCall_tag(w, p->value); break;
case 4: emit_RequestCall_score(w, p->value); break;
case 5: emit_RequestCall_filter(w, p->value); break;
case 6: emit_RequestCall_rank(w, p->value); break;
case 7: emit_RequestCall_find(w, p->value); break;
case 8: emit_RequestCall_annotate(w, p->value); break;
case 9: emit_RequestCall_recognize(w, p->value); break;
case 10: emit_RequestCall_relate(w, p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_RequestCall_annotate(writer *w,const void *input) { const thinkthen_cobol_RequestCall_annotate *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"function\":"); emit_RequestCall_annotate_member_function(w,p->m_function); }
if(p->m_input) { if(comma++) raw(w,","); raw(w,"\"input\":"); emit_RequestInput(w,p->m_input); }
if(p->m_options) { if(comma++) raw(w,","); raw(w,"\"options\":"); emit_RequestOptions(w,p->m_options); }
if(p->m_question) { if(comma++) raw(w,","); raw(w,"\"question\":"); emit_RequestQuestion(w,p->m_question); }
raw(w,"}");
}
static inline void emit_RequestCall_choose(writer *w,const void *input) { const thinkthen_cobol_RequestCall_choose *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"function\":"); emit_RequestCall_choose_member_function(w,p->m_function); }
if(p->m_input) { if(comma++) raw(w,","); raw(w,"\"input\":"); emit_RequestInput(w,p->m_input); }
if(p->m_options) { if(comma++) raw(w,","); raw(w,"\"options\":"); emit_RequestOptions(w,p->m_options); }
if(p->m_question) { if(comma++) raw(w,","); raw(w,"\"question\":"); emit_RequestQuestion(w,p->m_question); }
raw(w,"}");
}
static inline void emit_RequestCall_decide(writer *w,const void *input) { const thinkthen_cobol_RequestCall_decide *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"function\":"); emit_RequestCall_decide_member_function(w,p->m_function); }
if(p->m_input) { if(comma++) raw(w,","); raw(w,"\"input\":"); emit_RequestInput(w,p->m_input); }
if(p->m_options) { if(comma++) raw(w,","); raw(w,"\"options\":"); emit_RequestOptions(w,p->m_options); }
if(p->m_question) { if(comma++) raw(w,","); raw(w,"\"question\":"); emit_RequestQuestion(w,p->m_question); }
raw(w,"}");
}
static inline void emit_RequestCall_filter(writer *w,const void *input) { const thinkthen_cobol_RequestCall_filter *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"function\":"); emit_RequestCall_filter_member_function(w,p->m_function); }
if(p->m_input) { if(comma++) raw(w,","); raw(w,"\"input\":"); emit_RequestInput(w,p->m_input); }
if(p->m_options) { if(comma++) raw(w,","); raw(w,"\"options\":"); emit_RequestOptions(w,p->m_options); }
if(p->m_question) { if(comma++) raw(w,","); raw(w,"\"question\":"); emit_RequestQuestion(w,p->m_question); }
raw(w,"}");
}
static inline void emit_RequestCall_find(writer *w,const void *input) { const thinkthen_cobol_RequestCall_find *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"function\":"); emit_RequestCall_find_member_function(w,p->m_function); }
if(p->m_input) { if(comma++) raw(w,","); raw(w,"\"input\":"); emit_RequestInput(w,p->m_input); }
if(p->m_options) { if(comma++) raw(w,","); raw(w,"\"options\":"); emit_RequestOptions(w,p->m_options); }
if(p->m_question) { if(comma++) raw(w,","); raw(w,"\"question\":"); emit_RequestQuestion(w,p->m_question); }
raw(w,"}");
}
static inline void emit_RequestCall_rank(writer *w,const void *input) { const thinkthen_cobol_RequestCall_rank *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"function\":"); emit_RequestCall_rank_member_function(w,p->m_function); }
if(p->m_input) { if(comma++) raw(w,","); raw(w,"\"input\":"); emit_RequestInput(w,p->m_input); }
if(p->m_options) { if(comma++) raw(w,","); raw(w,"\"options\":"); emit_RequestOptions(w,p->m_options); }
if(p->m_question) { if(comma++) raw(w,","); raw(w,"\"question\":"); emit_RequestQuestion(w,p->m_question); }
raw(w,"}");
}
static inline void emit_RequestCall_recognize(writer *w,const void *input) { const thinkthen_cobol_RequestCall_recognize *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"function\":"); emit_RequestCall_recognize_member_function(w,p->m_function); }
if(p->m_input) { if(comma++) raw(w,","); raw(w,"\"input\":"); emit_RequestInput(w,p->m_input); }
if(p->m_options) { if(comma++) raw(w,","); raw(w,"\"options\":"); emit_RequestOptions(w,p->m_options); }
if(p->m_question) { if(comma++) raw(w,","); raw(w,"\"question\":"); emit_RequestQuestion(w,p->m_question); }
raw(w,"}");
}
static inline void emit_RequestCall_relate(writer *w,const void *input) { const thinkthen_cobol_RequestCall_relate *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"function\":"); emit_RequestCall_relate_member_function(w,p->m_function); }
if(p->m_input) { if(comma++) raw(w,","); raw(w,"\"input\":"); emit_RequestInput(w,p->m_input); }
if(p->m_options) { if(comma++) raw(w,","); raw(w,"\"options\":"); emit_RequestOptions(w,p->m_options); }
if(p->m_question) { if(comma++) raw(w,","); raw(w,"\"question\":"); emit_RequestQuestion(w,p->m_question); }
raw(w,"}");
}
static inline void emit_RequestCall_score(writer *w,const void *input) { const thinkthen_cobol_RequestCall_score *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"function\":"); emit_RequestCall_score_member_function(w,p->m_function); }
if(p->m_input) { if(comma++) raw(w,","); raw(w,"\"input\":"); emit_RequestInput(w,p->m_input); }
if(p->m_options) { if(comma++) raw(w,","); raw(w,"\"options\":"); emit_RequestOptions(w,p->m_options); }
if(p->m_question) { if(comma++) raw(w,","); raw(w,"\"question\":"); emit_RequestQuestion(w,p->m_question); }
raw(w,"}");
}
static inline void emit_RequestCall_tag(writer *w,const void *input) { const thinkthen_cobol_RequestCall_tag *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"function\":"); emit_RequestCall_tag_member_function(w,p->m_function); }
if(p->m_input) { if(comma++) raw(w,","); raw(w,"\"input\":"); emit_RequestInput(w,p->m_input); }
if(p->m_options) { if(comma++) raw(w,","); raw(w,"\"options\":"); emit_RequestOptions(w,p->m_options); }
if(p->m_question) { if(comma++) raw(w,","); raw(w,"\"question\":"); emit_RequestQuestion(w,p->m_question); }
raw(w,"}");
}
static inline void emit_RequestDefinition(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_RequestDefinition_fields_decide(w, p->value); break;
case 2: emit_RequestDefinition_fields_choose(w, p->value); break;
case 3: emit_RequestDefinition_fields_tag(w, p->value); break;
case 4: emit_RequestDefinition_fields_score(w, p->value); break;
case 5: emit_RequestDefinition_fields_relate_version(w, p->value); break;
case 6: emit_RequestDefinition_fields_find(w, p->value); break;
case 7: emit_RequestDefinition_fields_recognize_version(w, p->value); break;
case 8: emit_RequestDefinition_fields_questions_version(w, p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide(w, p->value); break;
case 2: emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose(w, p->value); break;
case 3: emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag(w, p->value); break;
case 4: emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score(w, p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_choose) { if(comma++) raw(w,","); raw(w,"\"choose\":"); emit_Authored_questionText(w,p->m_choose); }
if(p->m_context_schema) { if(comma++) raw(w,","); raw(w,"\"context_schema\":"); emit_Authored_inputDeclaration(w,p->m_context_schema); }
if(p->m_item_schema) { if(comma++) raw(w,","); raw(w,"\"item_schema\":"); emit_Authored_inputDeclaration(w,p->m_item_schema); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_member_name(w,p->m_name); }
if(p->m_on) { if(comma++) raw(w,","); raw(w,"\"on\":"); emit_Authored_pointers(w,p->m_on); }
if(p->m_options) { if(comma++) raw(w,","); raw(w,"\"options\":"); emit_Authored_options(w,p->m_options); }
if(p->m_threshold) { if(comma++) raw(w,","); raw(w,"\"threshold\":"); emit_Authored_cut(w,p->m_threshold); }
if(p->m_wording_version) { if(comma++) raw(w,","); raw(w,"\"wording_version\":"); emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_member_wording_version(w,p->m_wording_version); }
raw(w,"}");
}
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_context_schema) { if(comma++) raw(w,","); raw(w,"\"context_schema\":"); emit_Authored_inputDeclaration(w,p->m_context_schema); }
if(p->m_decide) { if(comma++) raw(w,","); raw(w,"\"decide\":"); emit_Authored_questionText(w,p->m_decide); }
if(p->m_false) { if(comma++) raw(w,","); raw(w,"\"false\":"); emit_Authored_criterion(w,p->m_false); }
if(p->m_item_schema) { if(comma++) raw(w,","); raw(w,"\"item_schema\":"); emit_Authored_inputDeclaration(w,p->m_item_schema); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_member_name(w,p->m_name); }
if(p->m_on) { if(comma++) raw(w,","); raw(w,"\"on\":"); emit_Authored_pointers(w,p->m_on); }
if(p->m_threshold) { if(comma++) raw(w,","); raw(w,"\"threshold\":"); emit_Authored_threshold(w,p->m_threshold); }
if(p->m_true) { if(comma++) raw(w,","); raw(w,"\"true\":"); emit_Authored_criterion(w,p->m_true); }
if(p->m_wording_version) { if(comma++) raw(w,","); raw(w,"\"wording_version\":"); emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_member_wording_version(w,p->m_wording_version); }
raw(w,"}");
}
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_context_schema) { if(comma++) raw(w,","); raw(w,"\"context_schema\":"); emit_Authored_inputDeclaration(w,p->m_context_schema); }
if(p->m_item_schema) { if(comma++) raw(w,","); raw(w,"\"item_schema\":"); emit_Authored_inputDeclaration(w,p->m_item_schema); }
if(p->m_levels) { if(comma++) raw(w,","); raw(w,"\"levels\":"); emit_Authored_levels(w,p->m_levels); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_member_name(w,p->m_name); }
if(p->m_on) { if(comma++) raw(w,","); raw(w,"\"on\":"); emit_Authored_pointers(w,p->m_on); }
if(p->m_score) { if(comma++) raw(w,","); raw(w,"\"score\":"); emit_Authored_questionText(w,p->m_score); }
if(p->m_wording_version) { if(comma++) raw(w,","); raw(w,"\"wording_version\":"); emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_member_wording_version(w,p->m_wording_version); }
raw(w,"}");
}
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_context_schema) { if(comma++) raw(w,","); raw(w,"\"context_schema\":"); emit_Authored_inputDeclaration(w,p->m_context_schema); }
if(p->m_item_schema) { if(comma++) raw(w,","); raw(w,"\"item_schema\":"); emit_Authored_inputDeclaration(w,p->m_item_schema); }
if(p->m_labels) { if(comma++) raw(w,","); raw(w,"\"labels\":"); emit_Authored_labels(w,p->m_labels); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_member_name(w,p->m_name); }
if(p->m_on) { if(comma++) raw(w,","); raw(w,"\"on\":"); emit_Authored_pointers(w,p->m_on); }
if(p->m_tag) { if(comma++) raw(w,","); raw(w,"\"tag\":"); emit_Authored_questionText(w,p->m_tag); }
if(p->m_threshold) { if(comma++) raw(w,","); raw(w,"\"threshold\":"); emit_Authored_cut(w,p->m_threshold); }
if(p->m_wording_version) { if(comma++) raw(w,","); raw(w,"\"wording_version\":"); emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_member_wording_version(w,p->m_wording_version); }
raw(w,"}");
}
static inline void emit_RequestDefinition_fields_choose(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_choose *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_batch) { if(comma++) raw(w,","); raw(w,"\"batch\":"); emit_RequestDefinition_fields_choose_member_batch(w,p->m_batch); }
if(p->m_choose) { if(comma++) raw(w,","); raw(w,"\"choose\":"); emit_Authored_questionText(w,p->m_choose); }
if(p->m_context_schema) { if(comma++) raw(w,","); raw(w,"\"context_schema\":"); emit_Authored_inputDeclaration(w,p->m_context_schema); }
if(p->m_item_schema) { if(comma++) raw(w,","); raw(w,"\"item_schema\":"); emit_Authored_inputDeclaration(w,p->m_item_schema); }
if(p->m_model) { if(comma++) raw(w,","); raw(w,"\"model\":"); emit_Authored_name(w,p->m_model); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_RequestDefinition_fields_choose_member_name(w,p->m_name); }
if(p->m_on) { if(comma++) raw(w,","); raw(w,"\"on\":"); emit_Authored_pointers(w,p->m_on); }
if(p->m_options) { if(comma++) raw(w,","); raw(w,"\"options\":"); emit_Authored_options(w,p->m_options); }
if(p->m_profile) { if(comma++) raw(w,","); raw(w,"\"profile\":"); emit_Authored_profile(w,p->m_profile); }
if(p->m_threshold) { if(comma++) raw(w,","); raw(w,"\"threshold\":"); emit_Authored_cut(w,p->m_threshold); }
if(p->m_wording_version) { if(comma++) raw(w,","); raw(w,"\"wording_version\":"); emit_RequestDefinition_fields_choose_member_wording_version(w,p->m_wording_version); }
raw(w,"}");
}
static inline void emit_RequestDefinition_fields_decide(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_decide *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_batch) { if(comma++) raw(w,","); raw(w,"\"batch\":"); emit_RequestDefinition_fields_decide_member_batch(w,p->m_batch); }
if(p->m_context_schema) { if(comma++) raw(w,","); raw(w,"\"context_schema\":"); emit_Authored_inputDeclaration(w,p->m_context_schema); }
if(p->m_decide) { if(comma++) raw(w,","); raw(w,"\"decide\":"); emit_Authored_questionText(w,p->m_decide); }
if(p->m_false) { if(comma++) raw(w,","); raw(w,"\"false\":"); emit_Authored_criterion(w,p->m_false); }
if(p->m_item_schema) { if(comma++) raw(w,","); raw(w,"\"item_schema\":"); emit_Authored_inputDeclaration(w,p->m_item_schema); }
if(p->m_model) { if(comma++) raw(w,","); raw(w,"\"model\":"); emit_Authored_name(w,p->m_model); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_RequestDefinition_fields_decide_member_name(w,p->m_name); }
if(p->m_on) { if(comma++) raw(w,","); raw(w,"\"on\":"); emit_Authored_pointers(w,p->m_on); }
if(p->m_profile) { if(comma++) raw(w,","); raw(w,"\"profile\":"); emit_Authored_profile(w,p->m_profile); }
if(p->m_threshold) { if(comma++) raw(w,","); raw(w,"\"threshold\":"); emit_Authored_threshold(w,p->m_threshold); }
if(p->m_true) { if(comma++) raw(w,","); raw(w,"\"true\":"); emit_Authored_criterion(w,p->m_true); }
if(p->m_wording_version) { if(comma++) raw(w,","); raw(w,"\"wording_version\":"); emit_RequestDefinition_fields_decide_member_wording_version(w,p->m_wording_version); }
raw(w,"}");
}
static inline void emit_RequestDefinition_fields_find(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_find *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_context_schema) { if(comma++) raw(w,","); raw(w,"\"context_schema\":"); emit_Authored_inputDeclaration(w,p->m_context_schema); }
if(p->m_find) { if(comma++) raw(w,","); raw(w,"\"find\":"); emit_Authored_questionText(w,p->m_find); }
if(p->m_item_schema) { if(comma++) raw(w,","); raw(w,"\"item_schema\":"); emit_Authored_inputDeclaration(w,p->m_item_schema); }
if(p->m_model) { if(comma++) raw(w,","); raw(w,"\"model\":"); emit_Authored_name(w,p->m_model); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_RequestDefinition_fields_find_member_name(w,p->m_name); }
if(p->m_on) { if(comma++) raw(w,","); raw(w,"\"on\":"); emit_Authored_pointers(w,p->m_on); }
if(p->m_profile) { if(comma++) raw(w,","); raw(w,"\"profile\":"); emit_Authored_profile(w,p->m_profile); }
if(p->m_wording_version) { if(comma++) raw(w,","); raw(w,"\"wording_version\":"); emit_RequestDefinition_fields_find_member_wording_version(w,p->m_wording_version); }
raw(w,"}");
}
static inline void emit_RequestDefinition_fields_questions_version(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_questions_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_batch) { if(comma++) raw(w,","); raw(w,"\"batch\":"); emit_RequestDefinition_fields_questions_version_member_batch(w,p->m_batch); }
if(p->m_profile) { if(comma++) raw(w,","); raw(w,"\"profile\":"); emit_Authored_profile(w,p->m_profile); }
if(p->m_questions) { if(comma++) raw(w,","); raw(w,"\"questions\":"); emit_RequestDefinition_fields_questions_version_member_questions(w,p->m_questions); }
if(p->m_threshold) { if(comma++) raw(w,","); raw(w,"\"threshold\":"); emit_Authored_threshold(w,p->m_threshold); }
if(1) { if(comma++) raw(w,","); raw(w,"\"version\":"); emit_RequestDefinition_fields_questions_version_member_version(w,p->m_version); }
raw(w,"}");
}
static inline void emit_RequestDefinition_fields_recognize_version(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_recognize_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_context_schema) { if(comma++) raw(w,","); raw(w,"\"context_schema\":"); emit_Authored_inputDeclaration(w,p->m_context_schema); }
if(p->m_item_schema) { if(comma++) raw(w,","); raw(w,"\"item_schema\":"); emit_Authored_inputDeclaration(w,p->m_item_schema); }
if(p->m_model) { if(comma++) raw(w,","); raw(w,"\"model\":"); emit_Authored_name(w,p->m_model); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_RequestDefinition_fields_recognize_version_member_name(w,p->m_name); }
if(p->m_on) { if(comma++) raw(w,","); raw(w,"\"on\":"); emit_Authored_pointers(w,p->m_on); }
if(p->m_profile) { if(comma++) raw(w,","); raw(w,"\"profile\":"); emit_Authored_profile(w,p->m_profile); }
if(p->m_recognize) { if(comma++) raw(w,","); raw(w,"\"recognize\":"); emit_RequestDefinition_fields_recognize_version_member_recognize(w,p->m_recognize); }
if(p->m_relation_threshold) { if(comma++) raw(w,","); raw(w,"\"relation_threshold\":"); emit_Authored_cut(w,p->m_relation_threshold); }
if(p->m_threshold) { if(comma++) raw(w,","); raw(w,"\"threshold\":"); emit_Authored_cut(w,p->m_threshold); }
if(1) { if(comma++) raw(w,","); raw(w,"\"version\":"); emit_RequestDefinition_fields_recognize_version_member_version(w,p->m_version); }
if(p->m_wording_version) { if(comma++) raw(w,","); raw(w,"\"wording_version\":"); emit_RequestDefinition_fields_recognize_version_member_wording_version(w,p->m_wording_version); }
raw(w,"}");
}
static inline void emit_RequestDefinition_fields_relate_version(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_relate_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_context_schema) { if(comma++) raw(w,","); raw(w,"\"context_schema\":"); emit_Authored_inputDeclaration(w,p->m_context_schema); }
if(p->m_item_schema) { if(comma++) raw(w,","); raw(w,"\"item_schema\":"); emit_Authored_inputDeclaration(w,p->m_item_schema); }
if(p->m_model) { if(comma++) raw(w,","); raw(w,"\"model\":"); emit_Authored_name(w,p->m_model); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_RequestDefinition_fields_relate_version_member_name(w,p->m_name); }
if(p->m_profile) { if(comma++) raw(w,","); raw(w,"\"profile\":"); emit_Authored_profile(w,p->m_profile); }
if(p->m_relate) { if(comma++) raw(w,","); raw(w,"\"relate\":"); emit_RequestDefinition_fields_relate_version_member_relate(w,p->m_relate); }
if(p->m_threshold) { if(comma++) raw(w,","); raw(w,"\"threshold\":"); emit_Authored_cut(w,p->m_threshold); }
if(1) { if(comma++) raw(w,","); raw(w,"\"version\":"); emit_RequestDefinition_fields_relate_version_member_version(w,p->m_version); }
if(p->m_wording_version) { if(comma++) raw(w,","); raw(w,"\"wording_version\":"); emit_RequestDefinition_fields_relate_version_member_wording_version(w,p->m_wording_version); }
raw(w,"}");
}
static inline void emit_RequestDefinition_fields_score(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_score *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_batch) { if(comma++) raw(w,","); raw(w,"\"batch\":"); emit_RequestDefinition_fields_score_member_batch(w,p->m_batch); }
if(p->m_context_schema) { if(comma++) raw(w,","); raw(w,"\"context_schema\":"); emit_Authored_inputDeclaration(w,p->m_context_schema); }
if(p->m_item_schema) { if(comma++) raw(w,","); raw(w,"\"item_schema\":"); emit_Authored_inputDeclaration(w,p->m_item_schema); }
if(p->m_levels) { if(comma++) raw(w,","); raw(w,"\"levels\":"); emit_Authored_levels(w,p->m_levels); }
if(p->m_model) { if(comma++) raw(w,","); raw(w,"\"model\":"); emit_Authored_name(w,p->m_model); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_RequestDefinition_fields_score_member_name(w,p->m_name); }
if(p->m_on) { if(comma++) raw(w,","); raw(w,"\"on\":"); emit_Authored_pointers(w,p->m_on); }
if(p->m_profile) { if(comma++) raw(w,","); raw(w,"\"profile\":"); emit_Authored_profile(w,p->m_profile); }
if(p->m_score) { if(comma++) raw(w,","); raw(w,"\"score\":"); emit_Authored_questionText(w,p->m_score); }
if(p->m_wording_version) { if(comma++) raw(w,","); raw(w,"\"wording_version\":"); emit_RequestDefinition_fields_score_member_wording_version(w,p->m_wording_version); }
raw(w,"}");
}
static inline void emit_RequestDefinition_fields_tag(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_tag *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_batch) { if(comma++) raw(w,","); raw(w,"\"batch\":"); emit_RequestDefinition_fields_tag_member_batch(w,p->m_batch); }
if(p->m_context_schema) { if(comma++) raw(w,","); raw(w,"\"context_schema\":"); emit_Authored_inputDeclaration(w,p->m_context_schema); }
if(p->m_item_schema) { if(comma++) raw(w,","); raw(w,"\"item_schema\":"); emit_Authored_inputDeclaration(w,p->m_item_schema); }
if(p->m_labels) { if(comma++) raw(w,","); raw(w,"\"labels\":"); emit_Authored_labels(w,p->m_labels); }
if(p->m_model) { if(comma++) raw(w,","); raw(w,"\"model\":"); emit_Authored_name(w,p->m_model); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_RequestDefinition_fields_tag_member_name(w,p->m_name); }
if(p->m_on) { if(comma++) raw(w,","); raw(w,"\"on\":"); emit_Authored_pointers(w,p->m_on); }
if(p->m_profile) { if(comma++) raw(w,","); raw(w,"\"profile\":"); emit_Authored_profile(w,p->m_profile); }
if(p->m_tag) { if(comma++) raw(w,","); raw(w,"\"tag\":"); emit_Authored_questionText(w,p->m_tag); }
if(p->m_threshold) { if(comma++) raw(w,","); raw(w,"\"threshold\":"); emit_Authored_cut(w,p->m_threshold); }
if(p->m_wording_version) { if(comma++) raw(w,","); raw(w,"\"wording_version\":"); emit_RequestDefinition_fields_tag_member_wording_version(w,p->m_wording_version); }
raw(w,"}");
}
static inline void emit_RequestFraming(writer *w,const void *input) { const thinkthen_cobol_RequestFraming *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_RequestFraming_arm_1(w,p->value); break;
case 2: emit_RequestFraming_arm_2(w,p->value); break;
case 3: emit_RequestFraming_arm_3(w,p->value); break;
case 4: emit_RequestFraming_arm_4(w,p->value); break;
case 5: emit_RequestFraming_arm_5(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_RequestImage(writer *w,const void *input) { const thinkthen_cobol_RequestImage *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_RequestImage_file(w, p->value); break;
case 2: emit_RequestImage_bytes(w, p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_RequestImage_bytes(writer *w,const void *input) { const thinkthen_cobol_RequestImage_bytes *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_bytes) { if(comma++) raw(w,","); raw(w,"\"bytes\":"); emit_RequestImage_bytes_member_bytes(w,p->m_bytes); }
if(1) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_RequestImage_bytes_member_kind(w,p->m_kind); }
if(p->m_media) { if(comma++) raw(w,","); raw(w,"\"media\":"); emit_ImageMedia(w,p->m_media); }
raw(w,"}");
}
static inline void emit_RequestImage_file(writer *w,const void *input) { const thinkthen_cobol_RequestImage_file *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_RequestImage_file_member_kind(w,p->m_kind); }
if(p->m_media) { if(comma++) raw(w,","); raw(w,"\"media\":"); emit_ImageMedia(w,p->m_media); }
if(p->m_path) { if(comma++) raw(w,","); raw(w,"\"path\":"); emit_RequestImage_file_member_path(w,p->m_path); }
raw(w,"}");
}
static inline void emit_RequestInput(writer *w,const void *input) { const thinkthen_cobol_RequestInput *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_RequestInput_text(w, p->value); break;
case 2: emit_RequestInput_json(w, p->value); break;
case 3: emit_RequestInput_records(w, p->value); break;
case 4: emit_RequestInput_units(w, p->value); break;
case 5: emit_RequestInput_entities(w, p->value); break;
case 6: emit_RequestInput_source(w, p->value); break;
case 7: emit_RequestInput_feed(w, p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_RequestInput_entities(writer *w,const void *input) { const thinkthen_cobol_RequestInput_entities *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_items) { if(comma++) raw(w,","); raw(w,"\"items\":"); emit_RequestInput_entities_member_items(w,p->m_items); }
if(1) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_RequestInput_entities_member_kind(w,p->m_kind); }
raw(w,"}");
}
static inline void emit_RequestInput_feed(writer *w,const void *input) { const thinkthen_cobol_RequestInput_feed *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_framing) { if(comma++) raw(w,","); raw(w,"\"framing\":"); emit_RequestFraming(w,p->m_framing); }
if(p->m_images) { if(comma++) raw(w,","); raw(w,"\"images\":"); emit_RequestInput_feed_member_images(w,p->m_images); }
if(1) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_RequestInput_feed_member_kind(w,p->m_kind); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_RequestInput_feed_member_name(w,p->m_name); }
if(p->m_reading) { if(comma++) raw(w,","); raw(w,"\"reading\":"); emit_RequestReader(w,p->m_reading); }
raw(w,"}");
}
static inline void emit_RequestInput_json(writer *w,const void *input) { const thinkthen_cobol_RequestInput_json *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_images) { if(comma++) raw(w,","); raw(w,"\"images\":"); emit_RequestInput_json_member_images(w,p->m_images); }
if(1) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_RequestInput_json_member_kind(w,p->m_kind); }
if(p->m_value) { if(comma++) raw(w,","); raw(w,"\"value\":"); emit_RequestInput_json_member_value(w,p->m_value); }
raw(w,"}");
}
static inline void emit_RequestInput_records(writer *w,const void *input) { const thinkthen_cobol_RequestInput_records *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_items) { if(comma++) raw(w,","); raw(w,"\"items\":"); emit_RequestInput_records_member_items(w,p->m_items); }
if(1) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_RequestInput_records_member_kind(w,p->m_kind); }
raw(w,"}");
}
static inline void emit_RequestInput_source(writer *w,const void *input) { const thinkthen_cobol_RequestInput_source *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_RequestInput_source_member_kind(w,p->m_kind); }
if(p->m_source) { if(comma++) raw(w,","); raw(w,"\"source\":"); emit_RequestSource(w,p->m_source); }
raw(w,"}");
}
static inline void emit_RequestInput_text(writer *w,const void *input) { const thinkthen_cobol_RequestInput_text *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_images) { if(comma++) raw(w,","); raw(w,"\"images\":"); emit_RequestInput_text_member_images(w,p->m_images); }
if(1) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_RequestInput_text_member_kind(w,p->m_kind); }
if(p->m_text) { if(comma++) raw(w,","); raw(w,"\"text\":"); emit_RequestInput_text_member_text(w,p->m_text); }
raw(w,"}");
}
static inline void emit_RequestInput_units(writer *w,const void *input) { const thinkthen_cobol_RequestInput_units *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_items) { if(comma++) raw(w,","); raw(w,"\"items\":"); emit_RequestInput_units_member_items(w,p->m_items); }
if(1) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_RequestInput_units_member_kind(w,p->m_kind); }
raw(w,"}");
}
static inline void emit_RequestItem(writer *w,const void *input) { const thinkthen_cobol_RequestItem *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_context) { if(comma++) raw(w,","); raw(w,"\"context\":"); emit_ContextSchema(w,p->m_context); }
if(p->m_examples) { if(comma++) raw(w,","); raw(w,"\"examples\":"); emit_RequestItem_member_examples(w,p->m_examples); }
if(p->m_images) { if(comma++) raw(w,","); raw(w,"\"images\":"); emit_RequestItem_member_images(w,p->m_images); }
if(p->m_options) { if(comma++) raw(w,","); raw(w,"\"options\":"); emit_RequestItem_member_options(w,p->m_options); }
if(p->m_original) { if(comma++) raw(w,","); raw(w,"\"original\":"); emit_RequestOriginal(w,p->m_original); }
if(p->m_seed_spans) { if(comma++) raw(w,","); raw(w,"\"seed_spans\":"); emit_RequestItem_member_seed_spans(w,p->m_seed_spans); }
raw(w,"}");
}
static inline void emit_RequestOptions(writer *w,const void *input) { const thinkthen_cobol_RequestOptions *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_attempts) { if(comma++) raw(w,","); raw(w,"\"attempts\":"); emit_RequestOptions_member_attempts(w,p->m_attempts); }
if(p->m_batch) { if(comma++) raw(w,","); raw(w,"\"batch\":"); emit_RequestBatch(w,p->m_batch); }
if(p->m_context) { if(comma++) raw(w,","); raw(w,"\"context\":"); emit_RequestOptions_member_context(w,p->m_context); }
if(p->m_context_field) { if(comma++) raw(w,","); raw(w,"\"context_field\":"); emit_RequestOptions_member_context_field(w,p->m_context_field); }
if(p->m_deadline_ms) { if(comma++) raw(w,","); raw(w,"\"deadline_ms\":"); emit_RequestOptions_member_deadline_ms(w,p->m_deadline_ms); }
if(p->m_details) { if(comma++) raw(w,","); raw(w,"\"details\":"); emit_RequestOptions_member_details(w,p->m_details); }
if(p->m_examples) { if(comma++) raw(w,","); raw(w,"\"examples\":"); emit_RequestOptions_member_examples(w,p->m_examples); }
if(p->m_examples_field) { if(comma++) raw(w,","); raw(w,"\"examples_field\":"); emit_RequestOptions_member_examples_field(w,p->m_examples_field); }
if(p->m_field) { if(comma++) raw(w,","); raw(w,"\"field\":"); emit_RequestOptions_member_field(w,p->m_field); }
if(p->m_files_only) { if(comma++) raw(w,","); raw(w,"\"files_only\":"); emit_RequestOptions_member_files_only(w,p->m_files_only); }
if(p->m_max_requests_total) { if(comma++) raw(w,","); raw(w,"\"max_requests_total\":"); emit_RequestOptions_member_max_requests_total(w,p->m_max_requests_total); }
if(p->m_mode) { if(comma++) raw(w,","); raw(w,"\"mode\":"); emit_RecognitionMode(w,p->m_mode); }
if(p->m_model) { if(comma++) raw(w,","); raw(w,"\"model\":"); emit_RequestOptions_member_model(w,p->m_model); }
if(p->m_none) { if(comma++) raw(w,","); raw(w,"\"none\":"); emit_RequestOptions_member_none(w,p->m_none); }
if(p->m_options_field) { if(comma++) raw(w,","); raw(w,"\"options_field\":"); emit_RequestOptions_member_options_field(w,p->m_options_field); }
if(p->m_relation_threshold) { if(comma++) raw(w,","); raw(w,"\"relation_threshold\":"); emit_RequestThreshold(w,p->m_relation_threshold); }
if(p->m_seed_spans) { if(comma++) raw(w,","); raw(w,"\"seed_spans\":"); emit_RequestOptions_member_seed_spans(w,p->m_seed_spans); }
if(p->m_seed_spans_field) { if(comma++) raw(w,","); raw(w,"\"seed_spans_field\":"); emit_RequestOptions_member_seed_spans_field(w,p->m_seed_spans_field); }
if(p->m_snippet_pieces) { if(comma++) raw(w,","); raw(w,"\"snippet_pieces\":"); emit_RequestOptions_member_snippet_pieces(w,p->m_snippet_pieces); }
if(p->m_stage_context) { if(comma++) raw(w,","); raw(w,"\"stage_context\":"); emit_RecognitionStageContext(w,p->m_stage_context); }
if(p->m_threshold) { if(comma++) raw(w,","); raw(w,"\"threshold\":"); emit_RequestThreshold(w,p->m_threshold); }
if(p->m_top) { if(comma++) raw(w,","); raw(w,"\"top\":"); emit_RequestOptions_member_top(w,p->m_top); }
raw(w,"}");
}
static inline void emit_RequestOriginal(writer *w,const void *input) { const thinkthen_cobol_RequestOriginal *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_RequestOriginal_text(w, p->value); break;
case 2: emit_RequestOriginal_json(w, p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_RequestOriginal_json(writer *w,const void *input) { const thinkthen_cobol_RequestOriginal_json *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_RequestOriginal_json_member_kind(w,p->m_kind); }
if(p->m_value) { if(comma++) raw(w,","); raw(w,"\"value\":"); emit_RequestOriginal_json_member_value(w,p->m_value); }
raw(w,"}");
}
static inline void emit_RequestOriginal_text(writer *w,const void *input) { const thinkthen_cobol_RequestOriginal_text *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_RequestOriginal_text_member_kind(w,p->m_kind); }
if(p->m_text) { if(comma++) raw(w,","); raw(w,"\"text\":"); emit_RequestOriginal_text_member_text(w,p->m_text); }
raw(w,"}");
}
static inline void emit_RequestQuestion(writer *w,const void *input) { const thinkthen_cobol_RequestQuestion *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_RequestQuestion_text(w, p->value); break;
case 2: emit_RequestQuestion_definition(w, p->value); break;
case 3: emit_RequestQuestion_file(w, p->value); break;
case 4: emit_RequestQuestion_name(w, p->value); break;
case 5: emit_RequestQuestion_reference(w, p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_RequestQuestion_definition(writer *w,const void *input) { const thinkthen_cobol_RequestQuestion_definition *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_RequestQuestion_definition_member_kind(w,p->m_kind); }
if(p->m_value) { if(comma++) raw(w,","); raw(w,"\"value\":"); emit_RequestDefinition(w,p->m_value); }
raw(w,"}");
}
static inline void emit_RequestQuestion_file(writer *w,const void *input) { const thinkthen_cobol_RequestQuestion_file *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_RequestQuestion_file_member_kind(w,p->m_kind); }
if(p->m_path) { if(comma++) raw(w,","); raw(w,"\"path\":"); emit_RequestQuestion_file_member_path(w,p->m_path); }
raw(w,"}");
}
static inline void emit_RequestQuestion_name(writer *w,const void *input) { const thinkthen_cobol_RequestQuestion_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_RequestQuestion_name_member_kind(w,p->m_kind); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_RequestQuestion_name_member_name(w,p->m_name); }
raw(w,"}");
}
static inline void emit_RequestQuestion_reference(writer *w,const void *input) { const thinkthen_cobol_RequestQuestion_reference *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_RequestQuestion_reference_member_kind(w,p->m_kind); }
if(p->m_reference) { if(comma++) raw(w,","); raw(w,"\"reference\":"); emit_RequestQuestion_reference_member_reference(w,p->m_reference); }
raw(w,"}");
}
static inline void emit_RequestQuestion_text(writer *w,const void *input) { const thinkthen_cobol_RequestQuestion_text *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_RequestQuestion_text_member_kind(w,p->m_kind); }
if(p->m_text) { if(comma++) raw(w,","); raw(w,"\"text\":"); emit_RequestQuestion_text_member_text(w,p->m_text); }
raw(w,"}");
}
static inline void emit_RequestReader(writer *w,const void *input) { const thinkthen_cobol_RequestReader *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_unit) { if(comma++) raw(w,","); raw(w,"\"unit\":"); emit_SourceUnit(w,p->m_unit); }
if(p->m_window) { if(comma++) raw(w,","); raw(w,"\"window\":"); emit_RequestReader_member_window(w,p->m_window); }
raw(w,"}");
}
static inline void emit_RequestSessionDescriptor(writer *w,const void *input) { const thinkthen_cobol_RequestSessionDescriptor *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_item) { if(comma++) raw(w,","); raw(w,"\"item\":"); emit_RequestItem(w,p->m_item); }
if(p->m_location) { if(comma++) raw(w,","); raw(w,"\"location\":"); emit_SessionSourceLocation(w,p->m_location); }
raw(w,"}");
}
static inline void emit_RequestSource(writer *w,const void *input) { const thinkthen_cobol_RequestSource *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_framing) { if(comma++) raw(w,","); raw(w,"\"framing\":"); emit_RequestFraming(w,p->m_framing); }
if(p->m_media) { if(comma++) raw(w,","); raw(w,"\"media\":"); emit_ReaderMedia(w,p->m_media); }
if(p->m_paths) { if(comma++) raw(w,","); raw(w,"\"paths\":"); emit_RequestSource_member_paths(w,p->m_paths); }
if(p->m_reading) { if(comma++) raw(w,","); raw(w,"\"reading\":"); emit_RequestReader(w,p->m_reading); }
raw(w,"}");
}
static inline void emit_RequestThreshold(writer *w,const void *input) { const thinkthen_cobol_RequestThreshold *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_RequestThreshold_arm_1(w,p->value); break;
case 2: emit_RequestThreshold_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_RequestVersion(writer *w,const void *input) { const thinkthen_cobol_RequestVersion *p=input;
(void)p;
raw(w,"\"thinkthen.request/1\"");
}
static inline void emit_SessionSourceLocation(writer *w,const void *input) { const thinkthen_cobol_SessionSourceLocation *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_file) { if(comma++) raw(w,","); raw(w,"\"file\":"); emit_SessionSourceLocation_member_file(w,p->m_file); }
if(p->m_first_line) { if(comma++) raw(w,","); raw(w,"\"first_line\":"); emit_SessionSourceLocation_member_first_line(w,p->m_first_line); }
if(p->m_last_line) { if(comma++) raw(w,","); raw(w,"\"last_line\":"); emit_SessionSourceLocation_member_last_line(w,p->m_last_line); }
raw(w,"}");
}
static inline void emit_SourceUnit(writer *w,const void *input) { const thinkthen_cobol_SourceUnit *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_SourceUnit_arm_1(w,p->value); break;
case 2: emit_SourceUnit_arm_2(w,p->value); break;
case 3: emit_SourceUnit_arm_3(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_Authored_choose_member_batch(writer *w,const void *input) { const thinkthen_cobol_Authored_choose_member_batch *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_Authored_choose_member_batch_arm_1(w,p->value); break;
case 2: emit_Authored_choose_member_batch_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_Authored_choose_member_name(writer *w,const void *input) { const thinkthen_cobol_Authored_choose_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_Authored_choose_member_wording_version(writer *w,const void *input) { const thinkthen_cobol_Authored_choose_member_wording_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_Authored_criterion_arm_1(writer *w,const void *input) { const thinkthen_cobol_Authored_criterion_arm_1 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_Authored_criterion_arm_2(writer *w,const void *input) { const thinkthen_cobol_Authored_criterion_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,0);
}
static inline void emit_Authored_criterion_arm_3(writer *w,const void *input) { const thinkthen_cobol_Authored_criterion_arm_3 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_Authored_criterion_arm_3_item(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_Authored_criterion_arm_4(writer *w,const void *input) { const thinkthen_cobol_Authored_criterion_arm_4 *p=input;
(void)p; raw(w,"null");
}
static inline void emit_Authored_cut_arm_1(writer *w,const void *input) { const thinkthen_cobol_Authored_cut_arm_1 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
if(!isfinite(p->value)) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
snprintf(bytes,sizeof(bytes),"%.17g",p->value); raw(w,bytes);
}
static inline void emit_Authored_cut_arm_2(writer *w,const void *input) { const thinkthen_cobol_Authored_cut_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_Authored_decide_member_batch(writer *w,const void *input) { const thinkthen_cobol_Authored_decide_member_batch *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_Authored_decide_member_batch_arm_1(w,p->value); break;
case 2: emit_Authored_decide_member_batch_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_Authored_decide_member_name(writer *w,const void *input) { const thinkthen_cobol_Authored_decide_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_Authored_decide_member_wording_version(writer *w,const void *input) { const thinkthen_cobol_Authored_decide_member_wording_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_Authored_description_arm_1(writer *w,const void *input) { const thinkthen_cobol_Authored_description_arm_1 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_Authored_description_arm_2(writer *w,const void *input) { const thinkthen_cobol_Authored_description_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,0);
}
static inline void emit_Authored_description_arm_3(writer *w,const void *input) { const thinkthen_cobol_Authored_description_arm_3 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_Authored_description_arm_3_item(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_Authored_description_arm_4(writer *w,const void *input) { const thinkthen_cobol_Authored_description_arm_4 *p=input;
(void)p; raw(w,"null");
}
static inline void emit_Authored_find_member_name(writer *w,const void *input) { const thinkthen_cobol_Authored_find_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_Authored_find_member_wording_version(writer *w,const void *input) { const thinkthen_cobol_Authored_find_member_wording_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_Authored_inputDeclaration_object_member_properties(writer *w,const void *input) { const thinkthen_cobol_Authored_inputDeclaration_object_member_properties *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"{");
if(p->len && (!p->keys || !p->values)) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); text(w,p->keys[i].data,p->keys[i].len,1); raw(w,":"); emit_Authored_inputProperty(w,p->values[i]); }
raw(w,"}");
}
static inline void emit_Authored_inputDeclaration_object_member_required(writer *w,const void *input) { const thinkthen_cobol_Authored_inputDeclaration_object_member_required *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_Authored_inputDeclaration_object_member_required_item(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_Authored_inputDeclaration_object_member_type(writer *w,const void *input) { const thinkthen_cobol_Authored_inputDeclaration_object_member_type *p=input;
(void)p;
raw(w,"\"object\"");
}
static inline void emit_Authored_inputDeclaration_string_member_type(writer *w,const void *input) { const thinkthen_cobol_Authored_inputDeclaration_string_member_type *p=input;
(void)p;
raw(w,"\"string\"");
}
static inline void emit_Authored_inputProperty_array_member_items(writer *w,const void *input) { const thinkthen_cobol_Authored_inputProperty_array_member_items *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(1) { if(comma++) raw(w,","); raw(w,"\"type\":"); emit_Authored_inputProperty_array_member_items_member_type(w,p->m_type); }
raw(w,"}");
}
static inline void emit_Authored_inputProperty_array_member_type(writer *w,const void *input) { const thinkthen_cobol_Authored_inputProperty_array_member_type *p=input;
(void)p;
raw(w,"\"array\"");
}
static inline void emit_Authored_inputProperty_boolean_member_type(writer *w,const void *input) { const thinkthen_cobol_Authored_inputProperty_boolean_member_type *p=input;
(void)p;
raw(w,"\"boolean\"");
}
static inline void emit_Authored_inputProperty_number_member_type(writer *w,const void *input) { const thinkthen_cobol_Authored_inputProperty_number_member_type *p=input;
(void)p;
raw(w,"\"number\"");
}
static inline void emit_Authored_inputProperty_string_member_type(writer *w,const void *input) { const thinkthen_cobol_Authored_inputProperty_string_member_type *p=input;
(void)p;
raw(w,"\"string\"");
}
static inline void emit_Authored_labels_arm_1(writer *w,const void *input) { const thinkthen_cobol_Authored_labels_arm_1 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_Authored_name(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_Authored_labels_arm_2(writer *w,const void *input) { const thinkthen_cobol_Authored_labels_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"{");
if(p->len && (!p->keys || !p->values)) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); text(w,p->keys[i].data,p->keys[i].len,1); raw(w,":"); emit_Authored_description(w,p->values[i]); }
raw(w,"}");
}
static inline void emit_Authored_levels_arm_1(writer *w,const void *input) { const thinkthen_cobol_Authored_levels_arm_1 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_Authored_name(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_Authored_levels_arm_2(writer *w,const void *input) { const thinkthen_cobol_Authored_levels_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"{");
if(p->len && (!p->keys || !p->values)) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); text(w,p->keys[i].data,p->keys[i].len,1); raw(w,":"); emit_Authored_criterion(w,p->values[i]); }
raw(w,"}");
}
static inline void emit_Authored_options_arm_1(writer *w,const void *input) { const thinkthen_cobol_Authored_options_arm_1 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_Authored_name(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_Authored_options_arm_2(writer *w,const void *input) { const thinkthen_cobol_Authored_options_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"{");
if(p->len && (!p->keys || !p->values)) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); text(w,p->keys[i].data,p->keys[i].len,1); raw(w,":"); emit_Authored_description(w,p->values[i]); }
raw(w,"}");
}
static inline void emit_Authored_pointers_arm_1(writer *w,const void *input) { const thinkthen_cobol_Authored_pointers_arm_1 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_Authored_pointers_arm_2(writer *w,const void *input) { const thinkthen_cobol_Authored_pointers_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_Authored_pointers_arm_2_item(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_Authored_questionText_arm_1(writer *w,const void *input) { const thinkthen_cobol_Authored_questionText_arm_1 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_Authored_questionText_arm_2(writer *w,const void *input) { const thinkthen_cobol_Authored_questionText_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,0);
}
static inline void emit_Authored_questionText_arm_3(writer *w,const void *input) { const thinkthen_cobol_Authored_questionText_arm_3 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_Authored_questionText_arm_3_item(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_Authored_relate_member_name(writer *w,const void *input) { const thinkthen_cobol_Authored_relate_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_Authored_relate_member_relate(writer *w,const void *input) { const thinkthen_cobol_Authored_relate_member_relate *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_fields) { if(comma++) raw(w,","); raw(w,"\"fields\":"); emit_Authored_relate_member_relate_member_fields(w,p->m_fields); }
if(p->m_relations) { if(comma++) raw(w,","); raw(w,"\"relations\":"); emit_Authored_relate_member_relate_member_relations(w,p->m_relations); }
raw(w,"}");
}
static inline void emit_Authored_relate_member_version(writer *w,const void *input) { const thinkthen_cobol_Authored_relate_member_version *p=input;
(void)p;
raw(w,"1");
}
static inline void emit_Authored_relate_member_wording_version(writer *w,const void *input) { const thinkthen_cobol_Authored_relate_member_wording_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_Authored_relation_member_either(writer *w,const void *input) { const thinkthen_cobol_Authored_relation_member_either *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,p->value ? "true" : "false");
}
static inline void emit_Authored_relation_member_single(writer *w,const void *input) { const thinkthen_cobol_Authored_relation_member_single *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,p->value ? "true" : "false");
}
static inline void emit_Authored_score_member_batch(writer *w,const void *input) { const thinkthen_cobol_Authored_score_member_batch *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_Authored_score_member_batch_arm_1(w,p->value); break;
case 2: emit_Authored_score_member_batch_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_Authored_score_member_name(writer *w,const void *input) { const thinkthen_cobol_Authored_score_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_Authored_score_member_wording_version(writer *w,const void *input) { const thinkthen_cobol_Authored_score_member_wording_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_Authored_tag_member_batch(writer *w,const void *input) { const thinkthen_cobol_Authored_tag_member_batch *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_Authored_tag_member_batch_arm_1(w,p->value); break;
case 2: emit_Authored_tag_member_batch_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_Authored_tag_member_name(writer *w,const void *input) { const thinkthen_cobol_Authored_tag_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_Authored_tag_member_wording_version(writer *w,const void *input) { const thinkthen_cobol_Authored_tag_member_wording_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_Authored_threshold_arm_1(writer *w,const void *input) { const thinkthen_cobol_Authored_threshold_arm_1 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
if(!isfinite(p->value)) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
snprintf(bytes,sizeof(bytes),"%.17g",p->value); raw(w,bytes);
}
static inline void emit_Authored_threshold_arm_2(writer *w,const void *input) { const thinkthen_cobol_Authored_threshold_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_ContextSchema_arm_1(writer *w,const void *input) { const thinkthen_cobol_ContextSchema_arm_1 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_ContextSchema_arm_2(writer *w,const void *input) { const thinkthen_cobol_ContextSchema_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,0);
}
static inline void emit_ImageMedia_arm_1(writer *w,const void *input) { const thinkthen_cobol_ImageMedia_arm_1 *p=input;
(void)p;
raw(w,"\"image/jpeg\"");
}
static inline void emit_ImageMedia_arm_2(writer *w,const void *input) { const thinkthen_cobol_ImageMedia_arm_2 *p=input;
(void)p;
raw(w,"\"image/png\"");
}
static inline void emit_OptionSchema_member_description(writer *w,const void *input) { const thinkthen_cobol_OptionSchema_member_description *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,0);
}
static inline void emit_OptionSchema_member_name(writer *w,const void *input) { const thinkthen_cobol_OptionSchema_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_ReaderMedia_arm_1(writer *w,const void *input) { const thinkthen_cobol_ReaderMedia_arm_1 *p=input;
(void)p;
raw(w,"\"text\"");
}
static inline void emit_ReaderMedia_arm_2(writer *w,const void *input) { const thinkthen_cobol_ReaderMedia_arm_2 *p=input;
(void)p;
raw(w,"\"image\"");
}
static inline void emit_RecognitionExample_arm_1(writer *w,const void *input) { const thinkthen_cobol_RecognitionExample_arm_1 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RecognitionExampleEntity_member_end(writer *w,const void *input) { const thinkthen_cobol_RecognitionExampleEntity_member_end *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value); raw(w,bytes);
}
static inline void emit_RecognitionExampleEntity_member_kind(writer *w,const void *input) { const thinkthen_cobol_RecognitionExampleEntity_member_kind *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RecognitionExampleEntity_member_start(writer *w,const void *input) { const thinkthen_cobol_RecognitionExampleEntity_member_start *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value); raw(w,bytes);
}
static inline void emit_RecognitionExampleText_member_entities(writer *w,const void *input) { const thinkthen_cobol_RecognitionExampleText_member_entities *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_RecognitionExampleEntity(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_RecognitionExampleText_member_kinds(writer *w,const void *input) { const thinkthen_cobol_RecognitionExampleText_member_kinds *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_RecognitionExampleText_member_kinds_item(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_RecognitionExampleText_member_text(writer *w,const void *input) { const thinkthen_cobol_RecognitionExampleText_member_text *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RecognitionMode_arm_1(writer *w,const void *input) { const thinkthen_cobol_RecognitionMode_arm_1 *p=input;
(void)p;
raw(w,"\"whole\"");
}
static inline void emit_RecognitionMode_arm_2(writer *w,const void *input) { const thinkthen_cobol_RecognitionMode_arm_2 *p=input;
(void)p;
raw(w,"\"boundary_only\"");
}
static inline void emit_RecognitionSeedSpan_member_end(writer *w,const void *input) { const thinkthen_cobol_RecognitionSeedSpan_member_end *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value); raw(w,bytes);
}
static inline void emit_RecognitionSeedSpan_member_kind(writer *w,const void *input) { const thinkthen_cobol_RecognitionSeedSpan_member_kind *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RecognitionSeedSpan_member_start(writer *w,const void *input) { const thinkthen_cobol_RecognitionSeedSpan_member_start *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value); raw(w,bytes);
}
static inline void emit_RecognitionStageContext_member_boundary(writer *w,const void *input) { const thinkthen_cobol_RecognitionStageContext_member_boundary *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RecognitionStageContext_member_kind_edge(writer *w,const void *input) { const thinkthen_cobol_RecognitionStageContext_member_kind_edge *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RecognitionStageContext_member_relation(writer *w,const void *input) { const thinkthen_cobol_RecognitionStageContext_member_relation *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestBatch_arm_1(writer *w,const void *input) { const thinkthen_cobol_RequestBatch_arm_1 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value); raw(w,bytes);
}
static inline void emit_RequestBatch_arm_2(writer *w,const void *input) { const thinkthen_cobol_RequestBatch_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestCall_annotate_member_function(writer *w,const void *input) { const thinkthen_cobol_RequestCall_annotate_member_function *p=input;
(void)p;
raw(w,"\"annotate\"");
}
static inline void emit_RequestCall_choose_member_function(writer *w,const void *input) { const thinkthen_cobol_RequestCall_choose_member_function *p=input;
(void)p;
raw(w,"\"choose\"");
}
static inline void emit_RequestCall_decide_member_function(writer *w,const void *input) { const thinkthen_cobol_RequestCall_decide_member_function *p=input;
(void)p;
raw(w,"\"decide\"");
}
static inline void emit_RequestCall_filter_member_function(writer *w,const void *input) { const thinkthen_cobol_RequestCall_filter_member_function *p=input;
(void)p;
raw(w,"\"filter\"");
}
static inline void emit_RequestCall_find_member_function(writer *w,const void *input) { const thinkthen_cobol_RequestCall_find_member_function *p=input;
(void)p;
raw(w,"\"find\"");
}
static inline void emit_RequestCall_rank_member_function(writer *w,const void *input) { const thinkthen_cobol_RequestCall_rank_member_function *p=input;
(void)p;
raw(w,"\"rank\"");
}
static inline void emit_RequestCall_recognize_member_function(writer *w,const void *input) { const thinkthen_cobol_RequestCall_recognize_member_function *p=input;
(void)p;
raw(w,"\"recognize\"");
}
static inline void emit_RequestCall_relate_member_function(writer *w,const void *input) { const thinkthen_cobol_RequestCall_relate_member_function *p=input;
(void)p;
raw(w,"\"relate\"");
}
static inline void emit_RequestCall_score_member_function(writer *w,const void *input) { const thinkthen_cobol_RequestCall_score_member_function *p=input;
(void)p;
raw(w,"\"score\"");
}
static inline void emit_RequestCall_tag_member_function(writer *w,const void *input) { const thinkthen_cobol_RequestCall_tag_member_function *p=input;
(void)p;
raw(w,"\"tag\"");
}
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_member_name(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_member_wording_version(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_member_wording_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_member_name(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_member_wording_version(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_member_wording_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_member_name(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_member_wording_version(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_member_wording_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_member_name(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_member_wording_version(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_member_wording_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_RequestDefinition_fields_choose_member_batch(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_choose_member_batch *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_RequestDefinition_fields_choose_member_batch_arm_1(w,p->value); break;
case 2: emit_RequestDefinition_fields_choose_member_batch_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_RequestDefinition_fields_choose_member_name(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_choose_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_fields_choose_member_wording_version(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_choose_member_wording_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_RequestDefinition_fields_decide_member_batch(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_decide_member_batch *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_RequestDefinition_fields_decide_member_batch_arm_1(w,p->value); break;
case 2: emit_RequestDefinition_fields_decide_member_batch_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_RequestDefinition_fields_decide_member_name(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_decide_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_fields_decide_member_wording_version(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_decide_member_wording_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_RequestDefinition_fields_find_member_name(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_find_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_fields_find_member_wording_version(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_find_member_wording_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_RequestDefinition_fields_questions_version_member_batch(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_questions_version_member_batch *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,0);
}
static inline void emit_RequestDefinition_fields_questions_version_member_questions(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_questions_version_member_questions *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"{");
if(p->len && (!p->keys || !p->values)) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); text(w,p->keys[i].data,p->keys[i].len,1); raw(w,":"); emit_RequestDefinition_anyOf_7_properties_questions_additionalProperties(w,p->values[i]); }
raw(w,"}");
}
static inline void emit_RequestDefinition_fields_questions_version_member_version(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_questions_version_member_version *p=input;
(void)p;
raw(w,"1");
}
static inline void emit_RequestDefinition_fields_recognize_version_member_name(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_recognize_version_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_fields_recognize_version_member_recognize(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_recognize_version_member_recognize *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_entity_definition) { if(comma++) raw(w,","); raw(w,"\"entity_definition\":"); emit_Authored_questionText(w,p->m_entity_definition); }
if(p->m_instructions) { if(comma++) raw(w,","); raw(w,"\"instructions\":"); emit_Authored_questionText(w,p->m_instructions); }
if(p->m_kinds) { if(comma++) raw(w,","); raw(w,"\"kinds\":"); emit_RequestDefinition_fields_recognize_version_member_recognize_member_kinds(w,p->m_kinds); }
if(p->m_mode) { if(comma++) raw(w,","); raw(w,"\"mode\":"); emit_RecognitionMode(w,p->m_mode); }
if(p->m_relations) { if(comma++) raw(w,","); raw(w,"\"relations\":"); emit_RequestDefinition_fields_recognize_version_member_recognize_member_relations(w,p->m_relations); }
if(p->m_snippet_pieces) { if(comma++) raw(w,","); raw(w,"\"snippet_pieces\":"); emit_RequestDefinition_fields_recognize_version_member_recognize_member_snippet_pieces(w,p->m_snippet_pieces); }
if(p->m_stage_context) { if(comma++) raw(w,","); raw(w,"\"stage_context\":"); emit_RecognitionStageContext(w,p->m_stage_context); }
raw(w,"}");
}
static inline void emit_RequestDefinition_fields_recognize_version_member_version(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_recognize_version_member_version *p=input;
(void)p;
raw(w,"1");
}
static inline void emit_RequestDefinition_fields_recognize_version_member_wording_version(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_recognize_version_member_wording_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_RequestDefinition_fields_relate_version_member_name(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_relate_version_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_fields_relate_version_member_relate(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_relate_version_member_relate *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_fields) { if(comma++) raw(w,","); raw(w,"\"fields\":"); emit_RequestDefinition_fields_relate_version_member_relate_member_fields(w,p->m_fields); }
if(p->m_relations) { if(comma++) raw(w,","); raw(w,"\"relations\":"); emit_RequestDefinition_fields_relate_version_member_relate_member_relations(w,p->m_relations); }
raw(w,"}");
}
static inline void emit_RequestDefinition_fields_relate_version_member_version(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_relate_version_member_version *p=input;
(void)p;
raw(w,"1");
}
static inline void emit_RequestDefinition_fields_relate_version_member_wording_version(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_relate_version_member_wording_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_RequestDefinition_fields_score_member_batch(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_score_member_batch *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_RequestDefinition_fields_score_member_batch_arm_1(w,p->value); break;
case 2: emit_RequestDefinition_fields_score_member_batch_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_RequestDefinition_fields_score_member_name(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_score_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_fields_score_member_wording_version(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_score_member_wording_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_RequestDefinition_fields_tag_member_batch(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_tag_member_batch *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
switch(p->kind) {
case 1: emit_RequestDefinition_fields_tag_member_batch_arm_1(w,p->value); break;
case 2: emit_RequestDefinition_fields_tag_member_batch_arm_2(w,p->value); break;
default: w->code=THINKTHEN_COBOL_REPRESENTATION; break;
}
}
static inline void emit_RequestDefinition_fields_tag_member_name(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_tag_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_fields_tag_member_wording_version(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_tag_member_wording_version *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_RequestFraming_arm_1(writer *w,const void *input) { const thinkthen_cobol_RequestFraming_arm_1 *p=input;
(void)p;
raw(w,"\"document\"");
}
static inline void emit_RequestFraming_arm_2(writer *w,const void *input) { const thinkthen_cobol_RequestFraming_arm_2 *p=input;
(void)p;
raw(w,"\"lines\"");
}
static inline void emit_RequestFraming_arm_3(writer *w,const void *input) { const thinkthen_cobol_RequestFraming_arm_3 *p=input;
(void)p;
raw(w,"\"jsonl\"");
}
static inline void emit_RequestFraming_arm_4(writer *w,const void *input) { const thinkthen_cobol_RequestFraming_arm_4 *p=input;
(void)p;
raw(w,"\"csv\"");
}
static inline void emit_RequestFraming_arm_5(writer *w,const void *input) { const thinkthen_cobol_RequestFraming_arm_5 *p=input;
(void)p;
raw(w,"\"tsv\"");
}
static inline void emit_RequestImage_bytes_member_bytes(writer *w,const void *input) { const thinkthen_cobol_RequestImage_bytes_member_bytes *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestImage_bytes_member_kind(writer *w,const void *input) { const thinkthen_cobol_RequestImage_bytes_member_kind *p=input;
(void)p;
raw(w,"\"bytes\"");
}
static inline void emit_RequestImage_file_member_kind(writer *w,const void *input) { const thinkthen_cobol_RequestImage_file_member_kind *p=input;
(void)p;
raw(w,"\"file\"");
}
static inline void emit_RequestImage_file_member_path(writer *w,const void *input) { const thinkthen_cobol_RequestImage_file_member_path *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestInput_entities_member_items(writer *w,const void *input) { const thinkthen_cobol_RequestInput_entities_member_items *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_RequestItem(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_RequestInput_entities_member_kind(writer *w,const void *input) { const thinkthen_cobol_RequestInput_entities_member_kind *p=input;
(void)p;
raw(w,"\"entities\"");
}
static inline void emit_RequestInput_feed_member_images(writer *w,const void *input) { const thinkthen_cobol_RequestInput_feed_member_images *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_RequestImage(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_RequestInput_feed_member_kind(writer *w,const void *input) { const thinkthen_cobol_RequestInput_feed_member_kind *p=input;
(void)p;
raw(w,"\"feed\"");
}
static inline void emit_RequestInput_feed_member_name(writer *w,const void *input) { const thinkthen_cobol_RequestInput_feed_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestInput_json_member_images(writer *w,const void *input) { const thinkthen_cobol_RequestInput_json_member_images *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_RequestImage(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_RequestInput_json_member_kind(writer *w,const void *input) { const thinkthen_cobol_RequestInput_json_member_kind *p=input;
(void)p;
raw(w,"\"json\"");
}
static inline void emit_RequestInput_json_member_value(writer *w,const void *input) { const thinkthen_cobol_RequestInput_json_member_value *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,0);
}
static inline void emit_RequestInput_records_member_items(writer *w,const void *input) { const thinkthen_cobol_RequestInput_records_member_items *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_RequestItem(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_RequestInput_records_member_kind(writer *w,const void *input) { const thinkthen_cobol_RequestInput_records_member_kind *p=input;
(void)p;
raw(w,"\"records\"");
}
static inline void emit_RequestInput_source_member_kind(writer *w,const void *input) { const thinkthen_cobol_RequestInput_source_member_kind *p=input;
(void)p;
raw(w,"\"source\"");
}
static inline void emit_RequestInput_text_member_images(writer *w,const void *input) { const thinkthen_cobol_RequestInput_text_member_images *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_RequestImage(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_RequestInput_text_member_kind(writer *w,const void *input) { const thinkthen_cobol_RequestInput_text_member_kind *p=input;
(void)p;
raw(w,"\"text\"");
}
static inline void emit_RequestInput_text_member_text(writer *w,const void *input) { const thinkthen_cobol_RequestInput_text_member_text *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestInput_units_member_items(writer *w,const void *input) { const thinkthen_cobol_RequestInput_units_member_items *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_RequestItem(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_RequestInput_units_member_kind(writer *w,const void *input) { const thinkthen_cobol_RequestInput_units_member_kind *p=input;
(void)p;
raw(w,"\"units\"");
}
static inline void emit_RequestItem_member_examples(writer *w,const void *input) { const thinkthen_cobol_RequestItem_member_examples *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_RecognitionExample(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_RequestItem_member_images(writer *w,const void *input) { const thinkthen_cobol_RequestItem_member_images *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_RequestImage(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_RequestItem_member_options(writer *w,const void *input) { const thinkthen_cobol_RequestItem_member_options *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_OptionSchema(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_RequestItem_member_seed_spans(writer *w,const void *input) { const thinkthen_cobol_RequestItem_member_seed_spans *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_RecognitionSeedSpan(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_RequestOptions_member_attempts(writer *w,const void *input) { const thinkthen_cobol_RequestOptions_member_attempts *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,p->value ? "true" : "false");
}
static inline void emit_RequestOptions_member_context(writer *w,const void *input) { const thinkthen_cobol_RequestOptions_member_context *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestOptions_member_context_field(writer *w,const void *input) { const thinkthen_cobol_RequestOptions_member_context_field *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestOptions_member_deadline_ms(writer *w,const void *input) { const thinkthen_cobol_RequestOptions_member_deadline_ms *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_RequestOptions_member_details(writer *w,const void *input) { const thinkthen_cobol_RequestOptions_member_details *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,p->value ? "true" : "false");
}
static inline void emit_RequestOptions_member_examples(writer *w,const void *input) { const thinkthen_cobol_RequestOptions_member_examples *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_RecognitionExample(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_RequestOptions_member_examples_field(writer *w,const void *input) { const thinkthen_cobol_RequestOptions_member_examples_field *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestOptions_member_field(writer *w,const void *input) { const thinkthen_cobol_RequestOptions_member_field *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_RequestOptions_member_field_item(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_RequestOptions_member_files_only(writer *w,const void *input) { const thinkthen_cobol_RequestOptions_member_files_only *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,p->value ? "true" : "false");
}
static inline void emit_RequestOptions_member_max_requests_total(writer *w,const void *input) { const thinkthen_cobol_RequestOptions_member_max_requests_total *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value); raw(w,bytes);
}
static inline void emit_RequestOptions_member_model(writer *w,const void *input) { const thinkthen_cobol_RequestOptions_member_model *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestOptions_member_none(writer *w,const void *input) { const thinkthen_cobol_RequestOptions_member_none *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,p->value ? "true" : "false");
}
static inline void emit_RequestOptions_member_options_field(writer *w,const void *input) { const thinkthen_cobol_RequestOptions_member_options_field *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestOptions_member_seed_spans(writer *w,const void *input) { const thinkthen_cobol_RequestOptions_member_seed_spans *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_RecognitionSeedSpan(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_RequestOptions_member_seed_spans_field(writer *w,const void *input) { const thinkthen_cobol_RequestOptions_member_seed_spans_field *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestOptions_member_snippet_pieces(writer *w,const void *input) { const thinkthen_cobol_RequestOptions_member_snippet_pieces *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value); raw(w,bytes);
}
static inline void emit_RequestOptions_member_top(writer *w,const void *input) { const thinkthen_cobol_RequestOptions_member_top *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value); raw(w,bytes);
}
static inline void emit_RequestOriginal_json_member_kind(writer *w,const void *input) { const thinkthen_cobol_RequestOriginal_json_member_kind *p=input;
(void)p;
raw(w,"\"json\"");
}
static inline void emit_RequestOriginal_json_member_value(writer *w,const void *input) { const thinkthen_cobol_RequestOriginal_json_member_value *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,0);
}
static inline void emit_RequestOriginal_text_member_kind(writer *w,const void *input) { const thinkthen_cobol_RequestOriginal_text_member_kind *p=input;
(void)p;
raw(w,"\"text\"");
}
static inline void emit_RequestOriginal_text_member_text(writer *w,const void *input) { const thinkthen_cobol_RequestOriginal_text_member_text *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestQuestion_definition_member_kind(writer *w,const void *input) { const thinkthen_cobol_RequestQuestion_definition_member_kind *p=input;
(void)p;
raw(w,"\"definition\"");
}
static inline void emit_RequestQuestion_file_member_kind(writer *w,const void *input) { const thinkthen_cobol_RequestQuestion_file_member_kind *p=input;
(void)p;
raw(w,"\"file\"");
}
static inline void emit_RequestQuestion_file_member_path(writer *w,const void *input) { const thinkthen_cobol_RequestQuestion_file_member_path *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestQuestion_name_member_kind(writer *w,const void *input) { const thinkthen_cobol_RequestQuestion_name_member_kind *p=input;
(void)p;
raw(w,"\"name\"");
}
static inline void emit_RequestQuestion_name_member_name(writer *w,const void *input) { const thinkthen_cobol_RequestQuestion_name_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestQuestion_reference_member_kind(writer *w,const void *input) { const thinkthen_cobol_RequestQuestion_reference_member_kind *p=input;
(void)p;
raw(w,"\"reference\"");
}
static inline void emit_RequestQuestion_reference_member_reference(writer *w,const void *input) { const thinkthen_cobol_RequestQuestion_reference_member_reference *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestQuestion_text_member_kind(writer *w,const void *input) { const thinkthen_cobol_RequestQuestion_text_member_kind *p=input;
(void)p;
raw(w,"\"text\"");
}
static inline void emit_RequestQuestion_text_member_text(writer *w,const void *input) { const thinkthen_cobol_RequestQuestion_text_member_text *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestReader_member_window(writer *w,const void *input) { const thinkthen_cobol_RequestReader_member_window *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value); raw(w,bytes);
}
static inline void emit_RequestSource_member_paths(writer *w,const void *input) { const thinkthen_cobol_RequestSource_member_paths *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_RequestSource_member_paths_item(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_RequestThreshold_arm_1(writer *w,const void *input) { const thinkthen_cobol_RequestThreshold_arm_1 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
if(!isfinite(p->value)) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
snprintf(bytes,sizeof(bytes),"%.17g",p->value); raw(w,bytes);
}
static inline void emit_RequestThreshold_arm_2(writer *w,const void *input) { const thinkthen_cobol_RequestThreshold_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_SessionSourceLocation_member_file(writer *w,const void *input) { const thinkthen_cobol_SessionSourceLocation_member_file *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_SessionSourceLocation_member_first_line(writer *w,const void *input) { const thinkthen_cobol_SessionSourceLocation_member_first_line *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value); raw(w,bytes);
}
static inline void emit_SessionSourceLocation_member_last_line(writer *w,const void *input) { const thinkthen_cobol_SessionSourceLocation_member_last_line *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value); raw(w,bytes);
}
static inline void emit_SourceUnit_arm_1(writer *w,const void *input) { const thinkthen_cobol_SourceUnit_arm_1 *p=input;
(void)p;
raw(w,"\"line\"");
}
static inline void emit_SourceUnit_arm_2(writer *w,const void *input) { const thinkthen_cobol_SourceUnit_arm_2 *p=input;
(void)p;
raw(w,"\"window\"");
}
static inline void emit_SourceUnit_arm_3(writer *w,const void *input) { const thinkthen_cobol_SourceUnit_arm_3 *p=input;
(void)p;
raw(w,"\"file\"");
}
static inline void emit_Authored_choose_member_batch_arm_1(writer *w,const void *input) { const thinkthen_cobol_Authored_choose_member_batch_arm_1 *p=input;
(void)p;
raw(w,"\"max\"");
}
static inline void emit_Authored_choose_member_batch_arm_2(writer *w,const void *input) { const thinkthen_cobol_Authored_choose_member_batch_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_Authored_criterion_arm_3_item(writer *w,const void *input) { const thinkthen_cobol_Authored_criterion_arm_3_item *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,0);
}
static inline void emit_Authored_decide_member_batch_arm_1(writer *w,const void *input) { const thinkthen_cobol_Authored_decide_member_batch_arm_1 *p=input;
(void)p;
raw(w,"\"max\"");
}
static inline void emit_Authored_decide_member_batch_arm_2(writer *w,const void *input) { const thinkthen_cobol_Authored_decide_member_batch_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_Authored_description_arm_3_item(writer *w,const void *input) { const thinkthen_cobol_Authored_description_arm_3_item *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,0);
}
static inline void emit_Authored_inputDeclaration_object_member_required_item(writer *w,const void *input) { const thinkthen_cobol_Authored_inputDeclaration_object_member_required_item *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_Authored_inputProperty_array_member_items_member_type(writer *w,const void *input) { const thinkthen_cobol_Authored_inputProperty_array_member_items_member_type *p=input;
(void)p;
raw(w,"\"string\"");
}
static inline void emit_Authored_pointers_arm_2_item(writer *w,const void *input) { const thinkthen_cobol_Authored_pointers_arm_2_item *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_Authored_questionText_arm_3_item(writer *w,const void *input) { const thinkthen_cobol_Authored_questionText_arm_3_item *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,0);
}
static inline void emit_Authored_relate_member_relate_member_fields(writer *w,const void *input) { const thinkthen_cobol_Authored_relate_member_relate_member_fields *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_kind) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_Authored_relate_member_relate_member_fields_member_kind(w,p->m_kind); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_Authored_relate_member_relate_member_fields_member_name(w,p->m_name); }
raw(w,"}");
}
static inline void emit_Authored_relate_member_relate_member_relations(writer *w,const void *input) { const thinkthen_cobol_Authored_relate_member_relate_member_relations *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_Authored_relation(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_Authored_score_member_batch_arm_1(writer *w,const void *input) { const thinkthen_cobol_Authored_score_member_batch_arm_1 *p=input;
(void)p;
raw(w,"\"max\"");
}
static inline void emit_Authored_score_member_batch_arm_2(writer *w,const void *input) { const thinkthen_cobol_Authored_score_member_batch_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_Authored_tag_member_batch_arm_1(writer *w,const void *input) { const thinkthen_cobol_Authored_tag_member_batch_arm_1 *p=input;
(void)p;
raw(w,"\"max\"");
}
static inline void emit_Authored_tag_member_batch_arm_2(writer *w,const void *input) { const thinkthen_cobol_Authored_tag_member_batch_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_RecognitionExampleText_member_kinds_item(writer *w,const void *input) { const thinkthen_cobol_RecognitionExampleText_member_kinds_item *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_fields_choose_member_batch_arm_1(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_choose_member_batch_arm_1 *p=input;
(void)p;
raw(w,"\"max\"");
}
static inline void emit_RequestDefinition_fields_choose_member_batch_arm_2(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_choose_member_batch_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_RequestDefinition_fields_decide_member_batch_arm_1(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_decide_member_batch_arm_1 *p=input;
(void)p;
raw(w,"\"max\"");
}
static inline void emit_RequestDefinition_fields_decide_member_batch_arm_2(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_decide_member_batch_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_RequestDefinition_fields_recognize_version_member_recognize_member_kinds(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_recognize_version_member_recognize_member_kinds *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"{");
if(p->len && (!p->keys || !p->values)) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); text(w,p->keys[i].data,p->keys[i].len,1); raw(w,":"); emit_Authored_description(w,p->values[i]); }
raw(w,"}");
}
static inline void emit_RequestDefinition_fields_recognize_version_member_recognize_member_relations(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_recognize_version_member_recognize_member_relations *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_Authored_relation(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_RequestDefinition_fields_recognize_version_member_recognize_member_snippet_pieces(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_recognize_version_member_recognize_member_snippet_pieces *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%llu",(unsigned long long)p->value); raw(w,bytes);
}
static inline void emit_RequestDefinition_fields_relate_version_member_relate_member_fields(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_relate_version_member_relate_member_fields *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
int comma=0; raw(w,"{");
if(p->m_kind) { if(comma++) raw(w,","); raw(w,"\"kind\":"); emit_RequestDefinition_fields_relate_version_member_relate_member_fields_member_kind(w,p->m_kind); }
if(p->m_name) { if(comma++) raw(w,","); raw(w,"\"name\":"); emit_RequestDefinition_fields_relate_version_member_relate_member_fields_member_name(w,p->m_name); }
raw(w,"}");
}
static inline void emit_RequestDefinition_fields_relate_version_member_relate_member_relations(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_relate_version_member_relate_member_relations *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
raw(w,"[");
if(p->len && !p->data) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
for(uint64_t i=0;i<p->len && !w->code;i++) { if(i) raw(w,","); emit_Authored_relation(w,p->data[i]); }
raw(w,"]");
}
static inline void emit_RequestDefinition_fields_score_member_batch_arm_1(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_score_member_batch_arm_1 *p=input;
(void)p;
raw(w,"\"max\"");
}
static inline void emit_RequestDefinition_fields_score_member_batch_arm_2(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_score_member_batch_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_RequestDefinition_fields_tag_member_batch_arm_1(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_tag_member_batch_arm_1 *p=input;
(void)p;
raw(w,"\"max\"");
}
static inline void emit_RequestDefinition_fields_tag_member_batch_arm_2(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_tag_member_batch_arm_2 *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
char bytes[64];
snprintf(bytes,sizeof(bytes),"%lld",(long long)p->value); raw(w,bytes);
}
static inline void emit_RequestOptions_member_field_item(writer *w,const void *input) { const thinkthen_cobol_RequestOptions_member_field_item *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestSource_member_paths_item(writer *w,const void *input) { const thinkthen_cobol_RequestSource_member_paths_item *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_Authored_relate_member_relate_member_fields_member_kind(writer *w,const void *input) { const thinkthen_cobol_Authored_relate_member_relate_member_fields_member_kind *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_Authored_relate_member_relate_member_fields_member_name(writer *w,const void *input) { const thinkthen_cobol_Authored_relate_member_relate_member_fields_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_fields_relate_version_member_relate_member_fields_member_kind(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_relate_version_member_relate_member_fields_member_kind *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
static inline void emit_RequestDefinition_fields_relate_version_member_relate_member_fields_member_name(writer *w,const void *input) { const thinkthen_cobol_RequestDefinition_fields_relate_version_member_relate_member_fields_member_name *p=input;
if(!p) {w->code=THINKTHEN_COBOL_REPRESENTATION; return;}
text(w,p->data,p->len,1);
}
int TT_SESSION_REQUEST(const void *engine,const thinkthen_cobol_Request *request,void **out) {
writer w={0}; int encoded=encode(&w,request,emit_Request);
int code=encoded ? encoded : thinkthen_session_new_with_surface(engine,w.data,w.len,"cobol",5,(thinkthen_session **)out);
free(w.data); return code;
}
int TT_SESSION_PUSH(const void *session,const thinkthen_cobol_RequestSessionDescriptor *descriptor,uint32_t *status) {
writer w={0}; int encoded=encode(&w,descriptor,emit_RequestSessionDescriptor);
int code=encoded ? encoded : thinkthen_session_try_push((thinkthen_session *)session,w.data,w.len,status);
free(w.data); return code;
}
int TT_SESSION_DECIDE(const void *engine,const thinkthen_cobol_RequestCall_decide *input,void **out) {
thinkthen_cobol_RequestCall call={.kind=1,.value=input};
thinkthen_cobol_Request request={.m_call=&call};
return TT_SESSION_REQUEST(engine,&request,out);
}
int TT_SESSION_CHOOSE(const void *engine,const thinkthen_cobol_RequestCall_choose *input,void **out) {
thinkthen_cobol_RequestCall call={.kind=2,.value=input};
thinkthen_cobol_Request request={.m_call=&call};
return TT_SESSION_REQUEST(engine,&request,out);
}
int TT_SESSION_TAG(const void *engine,const thinkthen_cobol_RequestCall_tag *input,void **out) {
thinkthen_cobol_RequestCall call={.kind=3,.value=input};
thinkthen_cobol_Request request={.m_call=&call};
return TT_SESSION_REQUEST(engine,&request,out);
}
int TT_SESSION_SCORE(const void *engine,const thinkthen_cobol_RequestCall_score *input,void **out) {
thinkthen_cobol_RequestCall call={.kind=4,.value=input};
thinkthen_cobol_Request request={.m_call=&call};
return TT_SESSION_REQUEST(engine,&request,out);
}
int TT_SESSION_FILTER(const void *engine,const thinkthen_cobol_RequestCall_filter *input,void **out) {
thinkthen_cobol_RequestCall call={.kind=5,.value=input};
thinkthen_cobol_Request request={.m_call=&call};
return TT_SESSION_REQUEST(engine,&request,out);
}
int TT_SESSION_RANK(const void *engine,const thinkthen_cobol_RequestCall_rank *input,void **out) {
thinkthen_cobol_RequestCall call={.kind=6,.value=input};
thinkthen_cobol_Request request={.m_call=&call};
return TT_SESSION_REQUEST(engine,&request,out);
}
int TT_SESSION_FIND(const void *engine,const thinkthen_cobol_RequestCall_find *input,void **out) {
thinkthen_cobol_RequestCall call={.kind=7,.value=input};
thinkthen_cobol_Request request={.m_call=&call};
return TT_SESSION_REQUEST(engine,&request,out);
}
int TT_SESSION_ANNOTATE(const void *engine,const thinkthen_cobol_RequestCall_annotate *input,void **out) {
thinkthen_cobol_RequestCall call={.kind=8,.value=input};
thinkthen_cobol_Request request={.m_call=&call};
return TT_SESSION_REQUEST(engine,&request,out);
}
int TT_SESSION_RECOGNIZE(const void *engine,const thinkthen_cobol_RequestCall_recognize *input,void **out) {
thinkthen_cobol_RequestCall call={.kind=9,.value=input};
thinkthen_cobol_Request request={.m_call=&call};
return TT_SESSION_REQUEST(engine,&request,out);
}
int TT_SESSION_RELATE(const void *engine,const thinkthen_cobol_RequestCall_relate *input,void **out) {
thinkthen_cobol_RequestCall call={.kind=10,.value=input};
thinkthen_cobol_Request request={.m_call=&call};
return TT_SESSION_REQUEST(engine,&request,out);
}
