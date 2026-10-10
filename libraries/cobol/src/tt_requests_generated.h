/* Generated from the canonical Request graph; do not edit. */
#ifndef TT_COBOL_REQUESTS_GENERATED_H
#define TT_COBOL_REQUESTS_GENERATED_H
#include <stdint.h>
typedef struct thinkthen_cobol_text {const char *data; uint64_t len;} thinkthen_cobol_text;
typedef struct thinkthen_cobol_Authored_choose {
 uint64_t reserved;
 const void * m_batch;
 const void * m_choose;
 const void * m_context_schema;
 const void * m_item_schema;
 const void * m_model;
 const void * m_name;
 const void * m_on;
 const void * m_options;
 const void * m_profile;
 const void * m_threshold;
 const void * m_wording_version;
} thinkthen_cobol_Authored_choose;
typedef struct thinkthen_cobol_Authored_criterion {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_Authored_criterion;
typedef struct thinkthen_cobol_Authored_cut {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_Authored_cut;
typedef struct thinkthen_cobol_Authored_decide {
 uint64_t reserved;
 const void * m_batch;
 const void * m_context_schema;
 const void * m_decide;
 const void * m_false;
 const void * m_item_schema;
 const void * m_model;
 const void * m_name;
 const void * m_on;
 const void * m_profile;
 const void * m_threshold;
 const void * m_true;
 const void * m_wording_version;
} thinkthen_cobol_Authored_decide;
typedef struct thinkthen_cobol_Authored_description {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_Authored_description;
typedef struct thinkthen_cobol_Authored_find {
 uint64_t reserved;
 const void * m_context_schema;
 const void * m_find;
 const void * m_item_schema;
 const void * m_model;
 const void * m_name;
 const void * m_on;
 const void * m_profile;
 const void * m_wording_version;
} thinkthen_cobol_Authored_find;
typedef struct thinkthen_cobol_Authored_inputDeclaration {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_Authored_inputDeclaration;
typedef struct thinkthen_cobol_Authored_inputDeclaration_object {
 uint64_t reserved;
 const void * m_properties;
 const void * m_required;
 const void * m_type;
} thinkthen_cobol_Authored_inputDeclaration_object;
typedef struct thinkthen_cobol_Authored_inputDeclaration_string {
 uint64_t reserved;
 const void * m_type;
} thinkthen_cobol_Authored_inputDeclaration_string;
typedef struct thinkthen_cobol_Authored_inputProperty {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_Authored_inputProperty;
typedef struct thinkthen_cobol_Authored_inputProperty_array {
 uint64_t reserved;
 const void * m_items;
 const void * m_type;
} thinkthen_cobol_Authored_inputProperty_array;
typedef struct thinkthen_cobol_Authored_inputProperty_boolean {
 uint64_t reserved;
 const void * m_type;
} thinkthen_cobol_Authored_inputProperty_boolean;
typedef struct thinkthen_cobol_Authored_inputProperty_number {
 uint64_t reserved;
 const void * m_type;
} thinkthen_cobol_Authored_inputProperty_number;
typedef struct thinkthen_cobol_Authored_inputProperty_string {
 uint64_t reserved;
 const void * m_type;
} thinkthen_cobol_Authored_inputProperty_string;
typedef struct thinkthen_cobol_Authored_labels {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_Authored_labels;
typedef struct thinkthen_cobol_Authored_levels {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_Authored_levels;
typedef struct thinkthen_cobol_Authored_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_name;
typedef struct thinkthen_cobol_Authored_options {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_Authored_options;
typedef struct thinkthen_cobol_Authored_pointers {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_Authored_pointers;
typedef struct thinkthen_cobol_Authored_profile {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_profile;
typedef struct thinkthen_cobol_Authored_questionText {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_Authored_questionText;
typedef struct thinkthen_cobol_Authored_relate {
 uint64_t reserved;
 const void * m_context_schema;
 const void * m_item_schema;
 const void * m_model;
 const void * m_name;
 const void * m_profile;
 const void * m_relate;
 const void * m_threshold;
 const void * m_version;
 const void * m_wording_version;
} thinkthen_cobol_Authored_relate;
typedef struct thinkthen_cobol_Authored_relation {
 uint64_t reserved;
 const void * m_either;
 const void * m_name;
 const void * m_reads;
 const void * m_single;
 const void * m_source;
 const void * m_target;
} thinkthen_cobol_Authored_relation;
typedef struct thinkthen_cobol_Authored_score {
 uint64_t reserved;
 const void * m_batch;
 const void * m_context_schema;
 const void * m_item_schema;
 const void * m_levels;
 const void * m_model;
 const void * m_name;
 const void * m_on;
 const void * m_profile;
 const void * m_score;
 const void * m_wording_version;
} thinkthen_cobol_Authored_score;
typedef struct thinkthen_cobol_Authored_tag {
 uint64_t reserved;
 const void * m_batch;
 const void * m_context_schema;
 const void * m_item_schema;
 const void * m_labels;
 const void * m_model;
 const void * m_name;
 const void * m_on;
 const void * m_profile;
 const void * m_tag;
 const void * m_threshold;
 const void * m_wording_version;
} thinkthen_cobol_Authored_tag;
typedef struct thinkthen_cobol_Authored_threshold {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_Authored_threshold;
typedef struct thinkthen_cobol_ContextSchema {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_ContextSchema;
typedef struct thinkthen_cobol_ImageMedia {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_ImageMedia;
typedef struct thinkthen_cobol_OptionSchema {
 uint64_t reserved;
 const void * m_description;
 const void * m_name;
} thinkthen_cobol_OptionSchema;
typedef struct thinkthen_cobol_ReaderMedia {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_ReaderMedia;
typedef struct thinkthen_cobol_RecognitionExample {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_RecognitionExample;
typedef struct thinkthen_cobol_RecognitionExampleEntity {
 uint64_t reserved;
 const void * m_end;
 const void * m_kind;
 const void * m_start;
} thinkthen_cobol_RecognitionExampleEntity;
typedef struct thinkthen_cobol_RecognitionExampleText {
 uint64_t reserved;
 const void * m_entities;
 const void * m_kinds;
 const void * m_text;
} thinkthen_cobol_RecognitionExampleText;
typedef struct thinkthen_cobol_RecognitionMode {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_RecognitionMode;
typedef struct thinkthen_cobol_RecognitionSeedSpan {
 uint64_t reserved;
 const void * m_end;
 const void * m_kind;
 const void * m_start;
} thinkthen_cobol_RecognitionSeedSpan;
typedef struct thinkthen_cobol_RecognitionStageContext {
 uint64_t reserved;
 const void * m_boundary;
 const void * m_kind_edge;
 const void * m_relation;
} thinkthen_cobol_RecognitionStageContext;
typedef struct thinkthen_cobol_Request {
 uint64_t reserved;
 const void * m_call;
 const void * m_schema;
} thinkthen_cobol_Request;
typedef struct thinkthen_cobol_RequestBatch {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_RequestBatch;
typedef struct thinkthen_cobol_RequestCall {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_RequestCall;
typedef struct thinkthen_cobol_RequestCall_annotate {
 uint64_t reserved;
 const void * m_function;
 const void * m_input;
 const void * m_options;
 const void * m_question;
} thinkthen_cobol_RequestCall_annotate;
typedef struct thinkthen_cobol_RequestCall_choose {
 uint64_t reserved;
 const void * m_function;
 const void * m_input;
 const void * m_options;
 const void * m_question;
} thinkthen_cobol_RequestCall_choose;
typedef struct thinkthen_cobol_RequestCall_decide {
 uint64_t reserved;
 const void * m_function;
 const void * m_input;
 const void * m_options;
 const void * m_question;
} thinkthen_cobol_RequestCall_decide;
typedef struct thinkthen_cobol_RequestCall_filter {
 uint64_t reserved;
 const void * m_function;
 const void * m_input;
 const void * m_options;
 const void * m_question;
} thinkthen_cobol_RequestCall_filter;
typedef struct thinkthen_cobol_RequestCall_find {
 uint64_t reserved;
 const void * m_function;
 const void * m_input;
 const void * m_options;
 const void * m_question;
} thinkthen_cobol_RequestCall_find;
typedef struct thinkthen_cobol_RequestCall_rank {
 uint64_t reserved;
 const void * m_function;
 const void * m_input;
 const void * m_options;
 const void * m_question;
} thinkthen_cobol_RequestCall_rank;
typedef struct thinkthen_cobol_RequestCall_recognize {
 uint64_t reserved;
 const void * m_function;
 const void * m_input;
 const void * m_options;
 const void * m_question;
} thinkthen_cobol_RequestCall_recognize;
typedef struct thinkthen_cobol_RequestCall_relate {
 uint64_t reserved;
 const void * m_function;
 const void * m_input;
 const void * m_options;
 const void * m_question;
} thinkthen_cobol_RequestCall_relate;
typedef struct thinkthen_cobol_RequestCall_score {
 uint64_t reserved;
 const void * m_function;
 const void * m_input;
 const void * m_options;
 const void * m_question;
} thinkthen_cobol_RequestCall_score;
typedef struct thinkthen_cobol_RequestCall_tag {
 uint64_t reserved;
 const void * m_function;
 const void * m_input;
 const void * m_options;
 const void * m_question;
} thinkthen_cobol_RequestCall_tag;
typedef struct thinkthen_cobol_RequestDefinition {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_RequestDefinition;
typedef struct thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties;
typedef struct thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose {
 uint64_t reserved;
 const void * m_choose;
 const void * m_context_schema;
 const void * m_item_schema;
 const void * m_name;
 const void * m_on;
 const void * m_options;
 const void * m_threshold;
 const void * m_wording_version;
} thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose;
typedef struct thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide {
 uint64_t reserved;
 const void * m_context_schema;
 const void * m_decide;
 const void * m_false;
 const void * m_item_schema;
 const void * m_name;
 const void * m_on;
 const void * m_threshold;
 const void * m_true;
 const void * m_wording_version;
} thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide;
typedef struct thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score {
 uint64_t reserved;
 const void * m_context_schema;
 const void * m_item_schema;
 const void * m_levels;
 const void * m_name;
 const void * m_on;
 const void * m_score;
 const void * m_wording_version;
} thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score;
typedef struct thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag {
 uint64_t reserved;
 const void * m_context_schema;
 const void * m_item_schema;
 const void * m_labels;
 const void * m_name;
 const void * m_on;
 const void * m_tag;
 const void * m_threshold;
 const void * m_wording_version;
} thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag;
typedef struct thinkthen_cobol_RequestDefinition_fields_choose {
 uint64_t reserved;
 const void * m_batch;
 const void * m_choose;
 const void * m_context_schema;
 const void * m_item_schema;
 const void * m_model;
 const void * m_name;
 const void * m_on;
 const void * m_options;
 const void * m_profile;
 const void * m_threshold;
 const void * m_wording_version;
} thinkthen_cobol_RequestDefinition_fields_choose;
typedef struct thinkthen_cobol_RequestDefinition_fields_decide {
 uint64_t reserved;
 const void * m_batch;
 const void * m_context_schema;
 const void * m_decide;
 const void * m_false;
 const void * m_item_schema;
 const void * m_model;
 const void * m_name;
 const void * m_on;
 const void * m_profile;
 const void * m_threshold;
 const void * m_true;
 const void * m_wording_version;
} thinkthen_cobol_RequestDefinition_fields_decide;
typedef struct thinkthen_cobol_RequestDefinition_fields_find {
 uint64_t reserved;
 const void * m_context_schema;
 const void * m_find;
 const void * m_item_schema;
 const void * m_model;
 const void * m_name;
 const void * m_on;
 const void * m_profile;
 const void * m_wording_version;
} thinkthen_cobol_RequestDefinition_fields_find;
typedef struct thinkthen_cobol_RequestDefinition_fields_questions_version {
 uint64_t reserved;
 const void * m_batch;
 const void * m_profile;
 const void * m_questions;
 const void * m_threshold;
 const void * m_version;
} thinkthen_cobol_RequestDefinition_fields_questions_version;
typedef struct thinkthen_cobol_RequestDefinition_fields_recognize_version {
 uint64_t reserved;
 const void * m_context_schema;
 const void * m_item_schema;
 const void * m_model;
 const void * m_name;
 const void * m_on;
 const void * m_profile;
 const void * m_recognize;
 const void * m_relation_threshold;
 const void * m_threshold;
 const void * m_version;
 const void * m_wording_version;
} thinkthen_cobol_RequestDefinition_fields_recognize_version;
typedef struct thinkthen_cobol_RequestDefinition_fields_relate_version {
 uint64_t reserved;
 const void * m_context_schema;
 const void * m_item_schema;
 const void * m_model;
 const void * m_name;
 const void * m_profile;
 const void * m_relate;
 const void * m_threshold;
 const void * m_version;
 const void * m_wording_version;
} thinkthen_cobol_RequestDefinition_fields_relate_version;
typedef struct thinkthen_cobol_RequestDefinition_fields_score {
 uint64_t reserved;
 const void * m_batch;
 const void * m_context_schema;
 const void * m_item_schema;
 const void * m_levels;
 const void * m_model;
 const void * m_name;
 const void * m_on;
 const void * m_profile;
 const void * m_score;
 const void * m_wording_version;
} thinkthen_cobol_RequestDefinition_fields_score;
typedef struct thinkthen_cobol_RequestDefinition_fields_tag {
 uint64_t reserved;
 const void * m_batch;
 const void * m_context_schema;
 const void * m_item_schema;
 const void * m_labels;
 const void * m_model;
 const void * m_name;
 const void * m_on;
 const void * m_profile;
 const void * m_tag;
 const void * m_threshold;
 const void * m_wording_version;
} thinkthen_cobol_RequestDefinition_fields_tag;
typedef struct thinkthen_cobol_RequestFraming {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_RequestFraming;
typedef struct thinkthen_cobol_RequestImage {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_RequestImage;
typedef struct thinkthen_cobol_RequestImage_bytes {
 uint64_t reserved;
 const void * m_bytes;
 const void * m_kind;
 const void * m_media;
} thinkthen_cobol_RequestImage_bytes;
typedef struct thinkthen_cobol_RequestImage_file {
 uint64_t reserved;
 const void * m_kind;
 const void * m_media;
 const void * m_path;
} thinkthen_cobol_RequestImage_file;
typedef struct thinkthen_cobol_RequestInput {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_RequestInput;
typedef struct thinkthen_cobol_RequestInput_entities {
 uint64_t reserved;
 const void * m_items;
 const void * m_kind;
} thinkthen_cobol_RequestInput_entities;
typedef struct thinkthen_cobol_RequestInput_feed {
 uint64_t reserved;
 const void * m_framing;
 const void * m_images;
 const void * m_kind;
 const void * m_name;
 const void * m_reading;
} thinkthen_cobol_RequestInput_feed;
typedef struct thinkthen_cobol_RequestInput_json {
 uint64_t reserved;
 const void * m_images;
 const void * m_kind;
 const void * m_value;
} thinkthen_cobol_RequestInput_json;
typedef struct thinkthen_cobol_RequestInput_records {
 uint64_t reserved;
 const void * m_items;
 const void * m_kind;
} thinkthen_cobol_RequestInput_records;
typedef struct thinkthen_cobol_RequestInput_source {
 uint64_t reserved;
 const void * m_kind;
 const void * m_source;
} thinkthen_cobol_RequestInput_source;
typedef struct thinkthen_cobol_RequestInput_text {
 uint64_t reserved;
 const void * m_images;
 const void * m_kind;
 const void * m_text;
} thinkthen_cobol_RequestInput_text;
typedef struct thinkthen_cobol_RequestInput_units {
 uint64_t reserved;
 const void * m_items;
 const void * m_kind;
} thinkthen_cobol_RequestInput_units;
typedef struct thinkthen_cobol_RequestItem {
 uint64_t reserved;
 const void * m_context;
 const void * m_examples;
 const void * m_images;
 const void * m_options;
 const void * m_original;
 const void * m_seed_spans;
} thinkthen_cobol_RequestItem;
typedef struct thinkthen_cobol_RequestOptions {
 uint64_t reserved;
 const void * m_attempts;
 const void * m_batch;
 const void * m_context;
 const void * m_context_field;
 const void * m_deadline_ms;
 const void * m_details;
 const void * m_examples;
 const void * m_examples_field;
 const void * m_field;
 const void * m_files_only;
 const void * m_max_requests_total;
 const void * m_mode;
 const void * m_model;
 const void * m_none;
 const void * m_options_field;
 const void * m_relation_threshold;
 const void * m_seed_spans;
 const void * m_seed_spans_field;
 const void * m_snippet_pieces;
 const void * m_stage_context;
 const void * m_threshold;
 const void * m_top;
} thinkthen_cobol_RequestOptions;
typedef struct thinkthen_cobol_RequestOriginal {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_RequestOriginal;
typedef struct thinkthen_cobol_RequestOriginal_json {
 uint64_t reserved;
 const void * m_kind;
 const void * m_value;
} thinkthen_cobol_RequestOriginal_json;
typedef struct thinkthen_cobol_RequestOriginal_text {
 uint64_t reserved;
 const void * m_kind;
 const void * m_text;
} thinkthen_cobol_RequestOriginal_text;
typedef struct thinkthen_cobol_RequestQuestion {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_RequestQuestion;
typedef struct thinkthen_cobol_RequestQuestion_definition {
 uint64_t reserved;
 const void * m_kind;
 const void * m_value;
} thinkthen_cobol_RequestQuestion_definition;
typedef struct thinkthen_cobol_RequestQuestion_file {
 uint64_t reserved;
 const void * m_kind;
 const void * m_path;
} thinkthen_cobol_RequestQuestion_file;
typedef struct thinkthen_cobol_RequestQuestion_name {
 uint64_t reserved;
 const void * m_kind;
 const void * m_name;
} thinkthen_cobol_RequestQuestion_name;
typedef struct thinkthen_cobol_RequestQuestion_reference {
 uint64_t reserved;
 const void * m_kind;
 const void * m_reference;
} thinkthen_cobol_RequestQuestion_reference;
typedef struct thinkthen_cobol_RequestQuestion_text {
 uint64_t reserved;
 const void * m_kind;
 const void * m_text;
} thinkthen_cobol_RequestQuestion_text;
typedef struct thinkthen_cobol_RequestReader {
 uint64_t reserved;
 const void * m_unit;
 const void * m_window;
} thinkthen_cobol_RequestReader;
typedef struct thinkthen_cobol_RequestSessionDescriptor {
 uint64_t reserved;
 const void * m_item;
 const void * m_location;
} thinkthen_cobol_RequestSessionDescriptor;
typedef struct thinkthen_cobol_RequestSource {
 uint64_t reserved;
 const void * m_media;
 const void * m_paths;
 const void * m_reading;
} thinkthen_cobol_RequestSource;
typedef struct thinkthen_cobol_RequestThreshold {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_RequestThreshold;
typedef struct thinkthen_cobol_RequestVersion {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_RequestVersion;
typedef struct thinkthen_cobol_SessionSourceLocation {
 uint64_t reserved;
 const void * m_file;
 const void * m_first_line;
 const void * m_last_line;
} thinkthen_cobol_SessionSourceLocation;
typedef struct thinkthen_cobol_SourceUnit {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_SourceUnit;
typedef struct thinkthen_cobol_Authored_choose_member_batch {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_Authored_choose_member_batch;
typedef struct thinkthen_cobol_Authored_choose_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_choose_member_name;
typedef struct thinkthen_cobol_Authored_choose_member_wording_version {
 int64_t value;
} thinkthen_cobol_Authored_choose_member_wording_version;
typedef struct thinkthen_cobol_Authored_criterion_arm_1 {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_criterion_arm_1;
typedef struct thinkthen_cobol_Authored_criterion_arm_2 {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_criterion_arm_2;
typedef struct thinkthen_cobol_Authored_criterion_arm_3 {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_Authored_criterion_arm_3;
typedef struct thinkthen_cobol_Authored_criterion_arm_4 {
 uint64_t reserved;
} thinkthen_cobol_Authored_criterion_arm_4;
typedef struct thinkthen_cobol_Authored_cut_arm_1 {
 double value;
} thinkthen_cobol_Authored_cut_arm_1;
typedef struct thinkthen_cobol_Authored_cut_arm_2 {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_cut_arm_2;
typedef struct thinkthen_cobol_Authored_decide_member_batch {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_Authored_decide_member_batch;
typedef struct thinkthen_cobol_Authored_decide_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_decide_member_name;
typedef struct thinkthen_cobol_Authored_decide_member_wording_version {
 int64_t value;
} thinkthen_cobol_Authored_decide_member_wording_version;
typedef struct thinkthen_cobol_Authored_description_arm_1 {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_description_arm_1;
typedef struct thinkthen_cobol_Authored_description_arm_2 {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_description_arm_2;
typedef struct thinkthen_cobol_Authored_description_arm_3 {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_Authored_description_arm_3;
typedef struct thinkthen_cobol_Authored_description_arm_4 {
 uint64_t reserved;
} thinkthen_cobol_Authored_description_arm_4;
typedef struct thinkthen_cobol_Authored_find_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_find_member_name;
typedef struct thinkthen_cobol_Authored_find_member_wording_version {
 int64_t value;
} thinkthen_cobol_Authored_find_member_wording_version;
typedef struct thinkthen_cobol_Authored_inputDeclaration_object_member_properties {
 const thinkthen_cobol_text * keys;
 const void *const * values;
 uint64_t len;
} thinkthen_cobol_Authored_inputDeclaration_object_member_properties;
typedef struct thinkthen_cobol_Authored_inputDeclaration_object_member_required {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_Authored_inputDeclaration_object_member_required;
typedef struct thinkthen_cobol_Authored_inputDeclaration_object_member_type {
 uint64_t reserved;
} thinkthen_cobol_Authored_inputDeclaration_object_member_type;
typedef struct thinkthen_cobol_Authored_inputDeclaration_string_member_type {
 uint64_t reserved;
} thinkthen_cobol_Authored_inputDeclaration_string_member_type;
typedef struct thinkthen_cobol_Authored_inputProperty_array_member_items {
 uint64_t reserved;
 const void * m_type;
} thinkthen_cobol_Authored_inputProperty_array_member_items;
typedef struct thinkthen_cobol_Authored_inputProperty_array_member_type {
 uint64_t reserved;
} thinkthen_cobol_Authored_inputProperty_array_member_type;
typedef struct thinkthen_cobol_Authored_inputProperty_boolean_member_type {
 uint64_t reserved;
} thinkthen_cobol_Authored_inputProperty_boolean_member_type;
typedef struct thinkthen_cobol_Authored_inputProperty_number_member_type {
 uint64_t reserved;
} thinkthen_cobol_Authored_inputProperty_number_member_type;
typedef struct thinkthen_cobol_Authored_inputProperty_string_member_type {
 uint64_t reserved;
} thinkthen_cobol_Authored_inputProperty_string_member_type;
typedef struct thinkthen_cobol_Authored_labels_arm_1 {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_Authored_labels_arm_1;
typedef struct thinkthen_cobol_Authored_labels_arm_2 {
 const thinkthen_cobol_text * keys;
 const void *const * values;
 uint64_t len;
} thinkthen_cobol_Authored_labels_arm_2;
typedef struct thinkthen_cobol_Authored_levels_arm_1 {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_Authored_levels_arm_1;
typedef struct thinkthen_cobol_Authored_levels_arm_2 {
 const thinkthen_cobol_text * keys;
 const void *const * values;
 uint64_t len;
} thinkthen_cobol_Authored_levels_arm_2;
typedef struct thinkthen_cobol_Authored_options_arm_1 {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_Authored_options_arm_1;
typedef struct thinkthen_cobol_Authored_options_arm_2 {
 const thinkthen_cobol_text * keys;
 const void *const * values;
 uint64_t len;
} thinkthen_cobol_Authored_options_arm_2;
typedef struct thinkthen_cobol_Authored_pointers_arm_1 {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_pointers_arm_1;
typedef struct thinkthen_cobol_Authored_pointers_arm_2 {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_Authored_pointers_arm_2;
typedef struct thinkthen_cobol_Authored_questionText_arm_1 {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_questionText_arm_1;
typedef struct thinkthen_cobol_Authored_questionText_arm_2 {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_questionText_arm_2;
typedef struct thinkthen_cobol_Authored_questionText_arm_3 {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_Authored_questionText_arm_3;
typedef struct thinkthen_cobol_Authored_relate_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_relate_member_name;
typedef struct thinkthen_cobol_Authored_relate_member_relate {
 uint64_t reserved;
 const void * m_fields;
 const void * m_relations;
} thinkthen_cobol_Authored_relate_member_relate;
typedef struct thinkthen_cobol_Authored_relate_member_version {
 uint64_t reserved;
} thinkthen_cobol_Authored_relate_member_version;
typedef struct thinkthen_cobol_Authored_relate_member_wording_version {
 int64_t value;
} thinkthen_cobol_Authored_relate_member_wording_version;
typedef struct thinkthen_cobol_Authored_relation_member_either {
 uint64_t value;
} thinkthen_cobol_Authored_relation_member_either;
typedef struct thinkthen_cobol_Authored_relation_member_single {
 uint64_t value;
} thinkthen_cobol_Authored_relation_member_single;
typedef struct thinkthen_cobol_Authored_score_member_batch {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_Authored_score_member_batch;
typedef struct thinkthen_cobol_Authored_score_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_score_member_name;
typedef struct thinkthen_cobol_Authored_score_member_wording_version {
 int64_t value;
} thinkthen_cobol_Authored_score_member_wording_version;
typedef struct thinkthen_cobol_Authored_tag_member_batch {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_Authored_tag_member_batch;
typedef struct thinkthen_cobol_Authored_tag_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_tag_member_name;
typedef struct thinkthen_cobol_Authored_tag_member_wording_version {
 int64_t value;
} thinkthen_cobol_Authored_tag_member_wording_version;
typedef struct thinkthen_cobol_Authored_threshold_arm_1 {
 double value;
} thinkthen_cobol_Authored_threshold_arm_1;
typedef struct thinkthen_cobol_Authored_threshold_arm_2 {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_threshold_arm_2;
typedef struct thinkthen_cobol_ContextSchema_arm_1 {
 const char * data;
 uint64_t len;
} thinkthen_cobol_ContextSchema_arm_1;
typedef struct thinkthen_cobol_ContextSchema_arm_2 {
 const char * data;
 uint64_t len;
} thinkthen_cobol_ContextSchema_arm_2;
typedef struct thinkthen_cobol_ImageMedia_arm_1 {
 uint64_t reserved;
} thinkthen_cobol_ImageMedia_arm_1;
typedef struct thinkthen_cobol_ImageMedia_arm_2 {
 uint64_t reserved;
} thinkthen_cobol_ImageMedia_arm_2;
typedef struct thinkthen_cobol_OptionSchema_member_description {
 const char * data;
 uint64_t len;
} thinkthen_cobol_OptionSchema_member_description;
typedef struct thinkthen_cobol_OptionSchema_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_OptionSchema_member_name;
typedef struct thinkthen_cobol_ReaderMedia_arm_1 {
 uint64_t reserved;
} thinkthen_cobol_ReaderMedia_arm_1;
typedef struct thinkthen_cobol_ReaderMedia_arm_2 {
 uint64_t reserved;
} thinkthen_cobol_ReaderMedia_arm_2;
typedef struct thinkthen_cobol_RecognitionExample_arm_1 {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RecognitionExample_arm_1;
typedef struct thinkthen_cobol_RecognitionExampleEntity_member_end {
 uint64_t value;
} thinkthen_cobol_RecognitionExampleEntity_member_end;
typedef struct thinkthen_cobol_RecognitionExampleEntity_member_kind {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RecognitionExampleEntity_member_kind;
typedef struct thinkthen_cobol_RecognitionExampleEntity_member_start {
 uint64_t value;
} thinkthen_cobol_RecognitionExampleEntity_member_start;
typedef struct thinkthen_cobol_RecognitionExampleText_member_entities {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_RecognitionExampleText_member_entities;
typedef struct thinkthen_cobol_RecognitionExampleText_member_kinds {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_RecognitionExampleText_member_kinds;
typedef struct thinkthen_cobol_RecognitionExampleText_member_text {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RecognitionExampleText_member_text;
typedef struct thinkthen_cobol_RecognitionMode_arm_1 {
 uint64_t reserved;
} thinkthen_cobol_RecognitionMode_arm_1;
typedef struct thinkthen_cobol_RecognitionMode_arm_2 {
 uint64_t reserved;
} thinkthen_cobol_RecognitionMode_arm_2;
typedef struct thinkthen_cobol_RecognitionSeedSpan_member_end {
 uint64_t value;
} thinkthen_cobol_RecognitionSeedSpan_member_end;
typedef struct thinkthen_cobol_RecognitionSeedSpan_member_kind {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RecognitionSeedSpan_member_kind;
typedef struct thinkthen_cobol_RecognitionSeedSpan_member_start {
 uint64_t value;
} thinkthen_cobol_RecognitionSeedSpan_member_start;
typedef struct thinkthen_cobol_RecognitionStageContext_member_boundary {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RecognitionStageContext_member_boundary;
typedef struct thinkthen_cobol_RecognitionStageContext_member_kind_edge {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RecognitionStageContext_member_kind_edge;
typedef struct thinkthen_cobol_RecognitionStageContext_member_relation {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RecognitionStageContext_member_relation;
typedef struct thinkthen_cobol_RequestBatch_arm_1 {
 uint64_t value;
} thinkthen_cobol_RequestBatch_arm_1;
typedef struct thinkthen_cobol_RequestBatch_arm_2 {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestBatch_arm_2;
typedef struct thinkthen_cobol_RequestCall_annotate_member_function {
 uint64_t reserved;
} thinkthen_cobol_RequestCall_annotate_member_function;
typedef struct thinkthen_cobol_RequestCall_choose_member_function {
 uint64_t reserved;
} thinkthen_cobol_RequestCall_choose_member_function;
typedef struct thinkthen_cobol_RequestCall_decide_member_function {
 uint64_t reserved;
} thinkthen_cobol_RequestCall_decide_member_function;
typedef struct thinkthen_cobol_RequestCall_filter_member_function {
 uint64_t reserved;
} thinkthen_cobol_RequestCall_filter_member_function;
typedef struct thinkthen_cobol_RequestCall_find_member_function {
 uint64_t reserved;
} thinkthen_cobol_RequestCall_find_member_function;
typedef struct thinkthen_cobol_RequestCall_rank_member_function {
 uint64_t reserved;
} thinkthen_cobol_RequestCall_rank_member_function;
typedef struct thinkthen_cobol_RequestCall_recognize_member_function {
 uint64_t reserved;
} thinkthen_cobol_RequestCall_recognize_member_function;
typedef struct thinkthen_cobol_RequestCall_relate_member_function {
 uint64_t reserved;
} thinkthen_cobol_RequestCall_relate_member_function;
typedef struct thinkthen_cobol_RequestCall_score_member_function {
 uint64_t reserved;
} thinkthen_cobol_RequestCall_score_member_function;
typedef struct thinkthen_cobol_RequestCall_tag_member_function {
 uint64_t reserved;
} thinkthen_cobol_RequestCall_tag_member_function;
typedef struct thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_member_name;
typedef struct thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_member_wording_version {
 int64_t value;
} thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_choose_member_wording_version;
typedef struct thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_member_name;
typedef struct thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_member_wording_version {
 int64_t value;
} thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_decide_member_wording_version;
typedef struct thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_member_name;
typedef struct thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_member_wording_version {
 int64_t value;
} thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_score_member_wording_version;
typedef struct thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_member_name;
typedef struct thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_member_wording_version {
 int64_t value;
} thinkthen_cobol_RequestDefinition_anyOf_7_properties_questions_additionalProperties_fields_tag_member_wording_version;
typedef struct thinkthen_cobol_RequestDefinition_fields_choose_member_batch {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_RequestDefinition_fields_choose_member_batch;
typedef struct thinkthen_cobol_RequestDefinition_fields_choose_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestDefinition_fields_choose_member_name;
typedef struct thinkthen_cobol_RequestDefinition_fields_choose_member_wording_version {
 int64_t value;
} thinkthen_cobol_RequestDefinition_fields_choose_member_wording_version;
typedef struct thinkthen_cobol_RequestDefinition_fields_decide_member_batch {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_RequestDefinition_fields_decide_member_batch;
typedef struct thinkthen_cobol_RequestDefinition_fields_decide_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestDefinition_fields_decide_member_name;
typedef struct thinkthen_cobol_RequestDefinition_fields_decide_member_wording_version {
 int64_t value;
} thinkthen_cobol_RequestDefinition_fields_decide_member_wording_version;
typedef struct thinkthen_cobol_RequestDefinition_fields_find_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestDefinition_fields_find_member_name;
typedef struct thinkthen_cobol_RequestDefinition_fields_find_member_wording_version {
 int64_t value;
} thinkthen_cobol_RequestDefinition_fields_find_member_wording_version;
typedef struct thinkthen_cobol_RequestDefinition_fields_questions_version_member_batch {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestDefinition_fields_questions_version_member_batch;
typedef struct thinkthen_cobol_RequestDefinition_fields_questions_version_member_questions {
 const thinkthen_cobol_text * keys;
 const void *const * values;
 uint64_t len;
} thinkthen_cobol_RequestDefinition_fields_questions_version_member_questions;
typedef struct thinkthen_cobol_RequestDefinition_fields_questions_version_member_version {
 uint64_t reserved;
} thinkthen_cobol_RequestDefinition_fields_questions_version_member_version;
typedef struct thinkthen_cobol_RequestDefinition_fields_recognize_version_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestDefinition_fields_recognize_version_member_name;
typedef struct thinkthen_cobol_RequestDefinition_fields_recognize_version_member_recognize {
 uint64_t reserved;
 const void * m_entity_definition;
 const void * m_instructions;
 const void * m_kinds;
 const void * m_mode;
 const void * m_relations;
 const void * m_snippet_pieces;
 const void * m_stage_context;
} thinkthen_cobol_RequestDefinition_fields_recognize_version_member_recognize;
typedef struct thinkthen_cobol_RequestDefinition_fields_recognize_version_member_version {
 uint64_t reserved;
} thinkthen_cobol_RequestDefinition_fields_recognize_version_member_version;
typedef struct thinkthen_cobol_RequestDefinition_fields_recognize_version_member_wording_version {
 int64_t value;
} thinkthen_cobol_RequestDefinition_fields_recognize_version_member_wording_version;
typedef struct thinkthen_cobol_RequestDefinition_fields_relate_version_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestDefinition_fields_relate_version_member_name;
typedef struct thinkthen_cobol_RequestDefinition_fields_relate_version_member_relate {
 uint64_t reserved;
 const void * m_fields;
 const void * m_relations;
} thinkthen_cobol_RequestDefinition_fields_relate_version_member_relate;
typedef struct thinkthen_cobol_RequestDefinition_fields_relate_version_member_version {
 uint64_t reserved;
} thinkthen_cobol_RequestDefinition_fields_relate_version_member_version;
typedef struct thinkthen_cobol_RequestDefinition_fields_relate_version_member_wording_version {
 int64_t value;
} thinkthen_cobol_RequestDefinition_fields_relate_version_member_wording_version;
typedef struct thinkthen_cobol_RequestDefinition_fields_score_member_batch {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_RequestDefinition_fields_score_member_batch;
typedef struct thinkthen_cobol_RequestDefinition_fields_score_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestDefinition_fields_score_member_name;
typedef struct thinkthen_cobol_RequestDefinition_fields_score_member_wording_version {
 int64_t value;
} thinkthen_cobol_RequestDefinition_fields_score_member_wording_version;
typedef struct thinkthen_cobol_RequestDefinition_fields_tag_member_batch {
 uint64_t kind;
 const void * value;
} thinkthen_cobol_RequestDefinition_fields_tag_member_batch;
typedef struct thinkthen_cobol_RequestDefinition_fields_tag_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestDefinition_fields_tag_member_name;
typedef struct thinkthen_cobol_RequestDefinition_fields_tag_member_wording_version {
 int64_t value;
} thinkthen_cobol_RequestDefinition_fields_tag_member_wording_version;
typedef struct thinkthen_cobol_RequestFraming_arm_1 {
 uint64_t reserved;
} thinkthen_cobol_RequestFraming_arm_1;
typedef struct thinkthen_cobol_RequestFraming_arm_2 {
 uint64_t reserved;
} thinkthen_cobol_RequestFraming_arm_2;
typedef struct thinkthen_cobol_RequestFraming_arm_3 {
 uint64_t reserved;
} thinkthen_cobol_RequestFraming_arm_3;
typedef struct thinkthen_cobol_RequestFraming_arm_4 {
 uint64_t reserved;
} thinkthen_cobol_RequestFraming_arm_4;
typedef struct thinkthen_cobol_RequestFraming_arm_5 {
 uint64_t reserved;
} thinkthen_cobol_RequestFraming_arm_5;
typedef struct thinkthen_cobol_RequestImage_bytes_member_bytes {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestImage_bytes_member_bytes;
typedef struct thinkthen_cobol_RequestImage_bytes_member_kind {
 uint64_t reserved;
} thinkthen_cobol_RequestImage_bytes_member_kind;
typedef struct thinkthen_cobol_RequestImage_file_member_kind {
 uint64_t reserved;
} thinkthen_cobol_RequestImage_file_member_kind;
typedef struct thinkthen_cobol_RequestImage_file_member_path {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestImage_file_member_path;
typedef struct thinkthen_cobol_RequestInput_entities_member_items {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_RequestInput_entities_member_items;
typedef struct thinkthen_cobol_RequestInput_entities_member_kind {
 uint64_t reserved;
} thinkthen_cobol_RequestInput_entities_member_kind;
typedef struct thinkthen_cobol_RequestInput_feed_member_images {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_RequestInput_feed_member_images;
typedef struct thinkthen_cobol_RequestInput_feed_member_kind {
 uint64_t reserved;
} thinkthen_cobol_RequestInput_feed_member_kind;
typedef struct thinkthen_cobol_RequestInput_feed_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestInput_feed_member_name;
typedef struct thinkthen_cobol_RequestInput_json_member_images {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_RequestInput_json_member_images;
typedef struct thinkthen_cobol_RequestInput_json_member_kind {
 uint64_t reserved;
} thinkthen_cobol_RequestInput_json_member_kind;
typedef struct thinkthen_cobol_RequestInput_json_member_value {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestInput_json_member_value;
typedef struct thinkthen_cobol_RequestInput_records_member_items {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_RequestInput_records_member_items;
typedef struct thinkthen_cobol_RequestInput_records_member_kind {
 uint64_t reserved;
} thinkthen_cobol_RequestInput_records_member_kind;
typedef struct thinkthen_cobol_RequestInput_source_member_kind {
 uint64_t reserved;
} thinkthen_cobol_RequestInput_source_member_kind;
typedef struct thinkthen_cobol_RequestInput_text_member_images {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_RequestInput_text_member_images;
typedef struct thinkthen_cobol_RequestInput_text_member_kind {
 uint64_t reserved;
} thinkthen_cobol_RequestInput_text_member_kind;
typedef struct thinkthen_cobol_RequestInput_text_member_text {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestInput_text_member_text;
typedef struct thinkthen_cobol_RequestInput_units_member_items {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_RequestInput_units_member_items;
typedef struct thinkthen_cobol_RequestInput_units_member_kind {
 uint64_t reserved;
} thinkthen_cobol_RequestInput_units_member_kind;
typedef struct thinkthen_cobol_RequestItem_member_examples {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_RequestItem_member_examples;
typedef struct thinkthen_cobol_RequestItem_member_images {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_RequestItem_member_images;
typedef struct thinkthen_cobol_RequestItem_member_options {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_RequestItem_member_options;
typedef struct thinkthen_cobol_RequestItem_member_seed_spans {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_RequestItem_member_seed_spans;
typedef struct thinkthen_cobol_RequestOptions_member_attempts {
 uint64_t value;
} thinkthen_cobol_RequestOptions_member_attempts;
typedef struct thinkthen_cobol_RequestOptions_member_context {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestOptions_member_context;
typedef struct thinkthen_cobol_RequestOptions_member_context_field {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestOptions_member_context_field;
typedef struct thinkthen_cobol_RequestOptions_member_deadline_ms {
 int64_t value;
} thinkthen_cobol_RequestOptions_member_deadline_ms;
typedef struct thinkthen_cobol_RequestOptions_member_details {
 uint64_t value;
} thinkthen_cobol_RequestOptions_member_details;
typedef struct thinkthen_cobol_RequestOptions_member_examples {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_RequestOptions_member_examples;
typedef struct thinkthen_cobol_RequestOptions_member_examples_field {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestOptions_member_examples_field;
typedef struct thinkthen_cobol_RequestOptions_member_field {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_RequestOptions_member_field;
typedef struct thinkthen_cobol_RequestOptions_member_files_only {
 uint64_t value;
} thinkthen_cobol_RequestOptions_member_files_only;
typedef struct thinkthen_cobol_RequestOptions_member_max_requests_total {
 uint64_t value;
} thinkthen_cobol_RequestOptions_member_max_requests_total;
typedef struct thinkthen_cobol_RequestOptions_member_model {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestOptions_member_model;
typedef struct thinkthen_cobol_RequestOptions_member_none {
 uint64_t value;
} thinkthen_cobol_RequestOptions_member_none;
typedef struct thinkthen_cobol_RequestOptions_member_options_field {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestOptions_member_options_field;
typedef struct thinkthen_cobol_RequestOptions_member_seed_spans {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_RequestOptions_member_seed_spans;
typedef struct thinkthen_cobol_RequestOptions_member_seed_spans_field {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestOptions_member_seed_spans_field;
typedef struct thinkthen_cobol_RequestOptions_member_snippet_pieces {
 uint64_t value;
} thinkthen_cobol_RequestOptions_member_snippet_pieces;
typedef struct thinkthen_cobol_RequestOptions_member_top {
 uint64_t value;
} thinkthen_cobol_RequestOptions_member_top;
typedef struct thinkthen_cobol_RequestOriginal_json_member_kind {
 uint64_t reserved;
} thinkthen_cobol_RequestOriginal_json_member_kind;
typedef struct thinkthen_cobol_RequestOriginal_json_member_value {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestOriginal_json_member_value;
typedef struct thinkthen_cobol_RequestOriginal_text_member_kind {
 uint64_t reserved;
} thinkthen_cobol_RequestOriginal_text_member_kind;
typedef struct thinkthen_cobol_RequestOriginal_text_member_text {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestOriginal_text_member_text;
typedef struct thinkthen_cobol_RequestQuestion_definition_member_kind {
 uint64_t reserved;
} thinkthen_cobol_RequestQuestion_definition_member_kind;
typedef struct thinkthen_cobol_RequestQuestion_file_member_kind {
 uint64_t reserved;
} thinkthen_cobol_RequestQuestion_file_member_kind;
typedef struct thinkthen_cobol_RequestQuestion_file_member_path {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestQuestion_file_member_path;
typedef struct thinkthen_cobol_RequestQuestion_name_member_kind {
 uint64_t reserved;
} thinkthen_cobol_RequestQuestion_name_member_kind;
typedef struct thinkthen_cobol_RequestQuestion_name_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestQuestion_name_member_name;
typedef struct thinkthen_cobol_RequestQuestion_reference_member_kind {
 uint64_t reserved;
} thinkthen_cobol_RequestQuestion_reference_member_kind;
typedef struct thinkthen_cobol_RequestQuestion_reference_member_reference {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestQuestion_reference_member_reference;
typedef struct thinkthen_cobol_RequestQuestion_text_member_kind {
 uint64_t reserved;
} thinkthen_cobol_RequestQuestion_text_member_kind;
typedef struct thinkthen_cobol_RequestQuestion_text_member_text {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestQuestion_text_member_text;
typedef struct thinkthen_cobol_RequestReader_member_window {
 uint64_t value;
} thinkthen_cobol_RequestReader_member_window;
typedef struct thinkthen_cobol_RequestSource_member_paths {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_RequestSource_member_paths;
typedef struct thinkthen_cobol_RequestThreshold_arm_1 {
 double value;
} thinkthen_cobol_RequestThreshold_arm_1;
typedef struct thinkthen_cobol_RequestThreshold_arm_2 {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestThreshold_arm_2;
typedef struct thinkthen_cobol_RequestVersion_arm_1 {
 uint64_t reserved;
} thinkthen_cobol_RequestVersion_arm_1;
typedef struct thinkthen_cobol_SessionSourceLocation_member_file {
 const char * data;
 uint64_t len;
} thinkthen_cobol_SessionSourceLocation_member_file;
typedef struct thinkthen_cobol_SessionSourceLocation_member_first_line {
 uint64_t value;
} thinkthen_cobol_SessionSourceLocation_member_first_line;
typedef struct thinkthen_cobol_SessionSourceLocation_member_last_line {
 uint64_t value;
} thinkthen_cobol_SessionSourceLocation_member_last_line;
typedef struct thinkthen_cobol_SourceUnit_arm_1 {
 uint64_t reserved;
} thinkthen_cobol_SourceUnit_arm_1;
typedef struct thinkthen_cobol_SourceUnit_arm_2 {
 uint64_t reserved;
} thinkthen_cobol_SourceUnit_arm_2;
typedef struct thinkthen_cobol_SourceUnit_arm_3 {
 uint64_t reserved;
} thinkthen_cobol_SourceUnit_arm_3;
typedef struct thinkthen_cobol_Authored_choose_member_batch_arm_1 {
 uint64_t reserved;
} thinkthen_cobol_Authored_choose_member_batch_arm_1;
typedef struct thinkthen_cobol_Authored_choose_member_batch_arm_2 {
 int64_t value;
} thinkthen_cobol_Authored_choose_member_batch_arm_2;
typedef struct thinkthen_cobol_Authored_criterion_arm_3_item {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_criterion_arm_3_item;
typedef struct thinkthen_cobol_Authored_decide_member_batch_arm_1 {
 uint64_t reserved;
} thinkthen_cobol_Authored_decide_member_batch_arm_1;
typedef struct thinkthen_cobol_Authored_decide_member_batch_arm_2 {
 int64_t value;
} thinkthen_cobol_Authored_decide_member_batch_arm_2;
typedef struct thinkthen_cobol_Authored_description_arm_3_item {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_description_arm_3_item;
typedef struct thinkthen_cobol_Authored_inputDeclaration_object_member_required_item {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_inputDeclaration_object_member_required_item;
typedef struct thinkthen_cobol_Authored_inputProperty_array_member_items_member_type {
 uint64_t reserved;
} thinkthen_cobol_Authored_inputProperty_array_member_items_member_type;
typedef struct thinkthen_cobol_Authored_pointers_arm_2_item {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_pointers_arm_2_item;
typedef struct thinkthen_cobol_Authored_questionText_arm_3_item {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_questionText_arm_3_item;
typedef struct thinkthen_cobol_Authored_relate_member_relate_member_fields {
 uint64_t reserved;
 const void * m_kind;
 const void * m_name;
} thinkthen_cobol_Authored_relate_member_relate_member_fields;
typedef struct thinkthen_cobol_Authored_relate_member_relate_member_relations {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_Authored_relate_member_relate_member_relations;
typedef struct thinkthen_cobol_Authored_score_member_batch_arm_1 {
 uint64_t reserved;
} thinkthen_cobol_Authored_score_member_batch_arm_1;
typedef struct thinkthen_cobol_Authored_score_member_batch_arm_2 {
 int64_t value;
} thinkthen_cobol_Authored_score_member_batch_arm_2;
typedef struct thinkthen_cobol_Authored_tag_member_batch_arm_1 {
 uint64_t reserved;
} thinkthen_cobol_Authored_tag_member_batch_arm_1;
typedef struct thinkthen_cobol_Authored_tag_member_batch_arm_2 {
 int64_t value;
} thinkthen_cobol_Authored_tag_member_batch_arm_2;
typedef struct thinkthen_cobol_RecognitionExampleText_member_kinds_item {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RecognitionExampleText_member_kinds_item;
typedef struct thinkthen_cobol_RequestDefinition_fields_choose_member_batch_arm_1 {
 uint64_t reserved;
} thinkthen_cobol_RequestDefinition_fields_choose_member_batch_arm_1;
typedef struct thinkthen_cobol_RequestDefinition_fields_choose_member_batch_arm_2 {
 int64_t value;
} thinkthen_cobol_RequestDefinition_fields_choose_member_batch_arm_2;
typedef struct thinkthen_cobol_RequestDefinition_fields_decide_member_batch_arm_1 {
 uint64_t reserved;
} thinkthen_cobol_RequestDefinition_fields_decide_member_batch_arm_1;
typedef struct thinkthen_cobol_RequestDefinition_fields_decide_member_batch_arm_2 {
 int64_t value;
} thinkthen_cobol_RequestDefinition_fields_decide_member_batch_arm_2;
typedef struct thinkthen_cobol_RequestDefinition_fields_recognize_version_member_recognize_member_kinds {
 const thinkthen_cobol_text * keys;
 const void *const * values;
 uint64_t len;
} thinkthen_cobol_RequestDefinition_fields_recognize_version_member_recognize_member_kinds;
typedef struct thinkthen_cobol_RequestDefinition_fields_recognize_version_member_recognize_member_relations {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_RequestDefinition_fields_recognize_version_member_recognize_member_relations;
typedef struct thinkthen_cobol_RequestDefinition_fields_recognize_version_member_recognize_member_snippet_pieces {
 uint64_t value;
} thinkthen_cobol_RequestDefinition_fields_recognize_version_member_recognize_member_snippet_pieces;
typedef struct thinkthen_cobol_RequestDefinition_fields_relate_version_member_relate_member_fields {
 uint64_t reserved;
 const void * m_kind;
 const void * m_name;
} thinkthen_cobol_RequestDefinition_fields_relate_version_member_relate_member_fields;
typedef struct thinkthen_cobol_RequestDefinition_fields_relate_version_member_relate_member_relations {
 const void *const * data;
 uint64_t len;
} thinkthen_cobol_RequestDefinition_fields_relate_version_member_relate_member_relations;
typedef struct thinkthen_cobol_RequestDefinition_fields_score_member_batch_arm_1 {
 uint64_t reserved;
} thinkthen_cobol_RequestDefinition_fields_score_member_batch_arm_1;
typedef struct thinkthen_cobol_RequestDefinition_fields_score_member_batch_arm_2 {
 int64_t value;
} thinkthen_cobol_RequestDefinition_fields_score_member_batch_arm_2;
typedef struct thinkthen_cobol_RequestDefinition_fields_tag_member_batch_arm_1 {
 uint64_t reserved;
} thinkthen_cobol_RequestDefinition_fields_tag_member_batch_arm_1;
typedef struct thinkthen_cobol_RequestDefinition_fields_tag_member_batch_arm_2 {
 int64_t value;
} thinkthen_cobol_RequestDefinition_fields_tag_member_batch_arm_2;
typedef struct thinkthen_cobol_RequestOptions_member_field_item {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestOptions_member_field_item;
typedef struct thinkthen_cobol_RequestSource_member_paths_item {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestSource_member_paths_item;
typedef struct thinkthen_cobol_Authored_relate_member_relate_member_fields_member_kind {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_relate_member_relate_member_fields_member_kind;
typedef struct thinkthen_cobol_Authored_relate_member_relate_member_fields_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_Authored_relate_member_relate_member_fields_member_name;
typedef struct thinkthen_cobol_RequestDefinition_fields_relate_version_member_relate_member_fields_member_kind {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestDefinition_fields_relate_version_member_relate_member_fields_member_kind;
typedef struct thinkthen_cobol_RequestDefinition_fields_relate_version_member_relate_member_fields_member_name {
 const char * data;
 uint64_t len;
} thinkthen_cobol_RequestDefinition_fields_relate_version_member_relate_member_fields_member_name;
int TT_SESSION_DECIDE(const void *,const thinkthen_cobol_RequestCall_decide *,void **);
int TT_SESSION_CHOOSE(const void *,const thinkthen_cobol_RequestCall_choose *,void **);
int TT_SESSION_TAG(const void *,const thinkthen_cobol_RequestCall_tag *,void **);
int TT_SESSION_SCORE(const void *,const thinkthen_cobol_RequestCall_score *,void **);
int TT_SESSION_FILTER(const void *,const thinkthen_cobol_RequestCall_filter *,void **);
int TT_SESSION_RANK(const void *,const thinkthen_cobol_RequestCall_rank *,void **);
int TT_SESSION_FIND(const void *,const thinkthen_cobol_RequestCall_find *,void **);
int TT_SESSION_ANNOTATE(const void *,const thinkthen_cobol_RequestCall_annotate *,void **);
int TT_SESSION_RECOGNIZE(const void *,const thinkthen_cobol_RequestCall_recognize *,void **);
int TT_SESSION_RELATE(const void *,const thinkthen_cobol_RequestCall_relate *,void **);
int TT_SESSION_PUSH(const void *,const thinkthen_cobol_RequestSessionDescriptor *,uint32_t *);
int TT_SESSION_REQUEST(const void *engine,const thinkthen_cobol_Request *request,void **out);
#endif
