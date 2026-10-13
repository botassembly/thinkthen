/* Generated from the canonical Request graph; do not edit. */
#ifndef TT_COBOL_REQUESTS_GENERATED_H
#define TT_COBOL_REQUESTS_GENERATED_H
#include <stdint.h>
#define THINKTHEN_COBOL_REPRESENTATION 1001
#define THINKTHEN_COBOL_OVERFLOW 1002
#define THINKTHEN_COBOL_TEXT_CAPACITY 8192
typedef struct thinkthen_cobol_text {
    const char *data;
    uint64_t len;
} thinkthen_cobol_text;
#define THINKTHEN_COBOL_ANNOTATEQUESTIONDECIDE_KIND 1
#define THINKTHEN_COBOL_ANNOTATEQUESTIONCHOOSE_KIND 2
#define THINKTHEN_COBOL_ANNOTATEQUESTIONTAG_KIND 3
#define THINKTHEN_COBOL_ANNOTATEQUESTIONSCORE_KIND 4
#define THINKTHEN_COBOL_CRITERIONARM1_KIND 1
#define THINKTHEN_COBOL_CRITERIONARM2_KIND 2
#define THINKTHEN_COBOL_CRITERIONARM3_KIND 3
#define THINKTHEN_COBOL_CRITERIONARM4_KIND 4
#define THINKTHEN_COBOL_CUTARM1_KIND 1
#define THINKTHEN_COBOL_CUTARM2_KIND 2
#define THINKTHEN_COBOL_DESCRIPTIONARM1_KIND 1
#define THINKTHEN_COBOL_DESCRIPTIONARM2_KIND 2
#define THINKTHEN_COBOL_DESCRIPTIONARM3_KIND 3
#define THINKTHEN_COBOL_DESCRIPTIONARM4_KIND 4
#define THINKTHEN_COBOL_INPUTDECLARATIONSTRING_KIND 1
#define THINKTHEN_COBOL_INPUTDECLARATIONOBJECT_KIND 2
#define THINKTHEN_COBOL_INPUTPROPERTYSTRING_KIND 1
#define THINKTHEN_COBOL_INPUTPROPERTYNUMBER_KIND 2
#define THINKTHEN_COBOL_INPUTPROPERTYBOOLEAN_KIND 3
#define THINKTHEN_COBOL_INPUTPROPERTYARRAY_KIND 4
#define THINKTHEN_COBOL_LABELSARM1_KIND 1
#define THINKTHEN_COBOL_LABELSARM2_KIND 2
#define THINKTHEN_COBOL_LEVELSARM1_KIND 1
#define THINKTHEN_COBOL_LEVELSARM2_KIND 2
#define THINKTHEN_COBOL_OPTIONSARM1_KIND 1
#define THINKTHEN_COBOL_OPTIONSARM2_KIND 2
#define THINKTHEN_COBOL_POINTERSARM1_KIND 1
#define THINKTHEN_COBOL_POINTERSARM2_KIND 2
#define THINKTHEN_COBOL_QUESTIONTEXTARM1_KIND 1
#define THINKTHEN_COBOL_QUESTIONTEXTARM2_KIND 2
#define THINKTHEN_COBOL_QUESTIONTEXTARM3_KIND 3
#define THINKTHEN_COBOL_THRESHOLDARM1_KIND 1
#define THINKTHEN_COBOL_THRESHOLDARM2_KIND 2
#define THINKTHEN_COBOL_CONTEXTSCHEMAARM1_KIND 1
#define THINKTHEN_COBOL_CONTEXTSCHEMAARM2_KIND 2
#define THINKTHEN_COBOL_IMAGEMEDIAARM1_KIND 1
#define THINKTHEN_COBOL_IMAGEMEDIAARM2_KIND 2
#define THINKTHEN_COBOL_READERMEDIAARM1_KIND 1
#define THINKTHEN_COBOL_READERMEDIAARM2_KIND 2
#define THINKTHEN_COBOL_RECOGNITIONEXAMPLEARM1_KIND 1
#define THINKTHEN_COBOL_RECOGNITIONEXAMPLE_ARM_2_KIND 2
#define THINKTHEN_COBOL_RECOGNITIONMODEARM1_KIND 1
#define THINKTHEN_COBOL_RECOGNITIONMODEARM2_KIND 2
#define THINKTHEN_COBOL_REQUESTBATCHARM1_KIND 1
#define THINKTHEN_COBOL_REQUESTBATCHARM2_KIND 2
#define THINKTHEN_COBOL_DEFINITIONDECIDE_KIND 1
#define THINKTHEN_COBOL_DEFINITIONCHOOSE_KIND 2
#define THINKTHEN_COBOL_DEFINITIONTAG_KIND 3
#define THINKTHEN_COBOL_DEFINITIONSCORE_KIND 4
#define THINKTHEN_COBOL_DEFINITIONRELATE_KIND 5
#define THINKTHEN_COBOL_DEFINITIONFIND_KIND 6
#define THINKTHEN_COBOL_DEFINITIONRECOGNIZEDEFINITION_KIND 7
#define THINKTHEN_COBOL_DEFINITIONANNOTATEDEFINITION_KIND 8
#define THINKTHEN_COBOL_REQUESTFRAMINGARM1_KIND 1
#define THINKTHEN_COBOL_REQUESTFRAMINGARM2_KIND 2
#define THINKTHEN_COBOL_REQUESTFRAMINGARM3_KIND 3
#define THINKTHEN_COBOL_REQUESTFRAMINGARM4_KIND 4
#define THINKTHEN_COBOL_REQUESTFRAMINGARM5_KIND 5
#define THINKTHEN_COBOL_REQUESTFUNCTIONARM1_KIND 1
#define THINKTHEN_COBOL_REQUESTFUNCTIONARM2_KIND 2
#define THINKTHEN_COBOL_REQUESTFUNCTIONARM3_KIND 3
#define THINKTHEN_COBOL_REQUESTFUNCTIONARM4_KIND 4
#define THINKTHEN_COBOL_REQUESTFUNCTIONARM5_KIND 5
#define THINKTHEN_COBOL_REQUESTFUNCTIONARM6_KIND 6
#define THINKTHEN_COBOL_REQUESTFUNCTIONARM7_KIND 7
#define THINKTHEN_COBOL_REQUESTFUNCTIONARM8_KIND 8
#define THINKTHEN_COBOL_REQUESTFUNCTIONARM9_KIND 9
#define THINKTHEN_COBOL_REQUESTFUNCTIONARM10_KIND 10
#define THINKTHEN_COBOL_REQUESTIMAGEFILE_KIND 1
#define THINKTHEN_COBOL_REQUESTIMAGEBYTES_KIND 2
#define THINKTHEN_COBOL_REQUESTINPUTTEXT_KIND 1
#define THINKTHEN_COBOL_REQUESTINPUTJSON_KIND 2
#define THINKTHEN_COBOL_REQUESTINPUTRECORDS_KIND 3
#define THINKTHEN_COBOL_REQUESTINPUTUNITS_KIND 4
#define THINKTHEN_COBOL_REQUESTINPUTENTITIES_KIND 5
#define THINKTHEN_COBOL_REQUESTINPUTSOURCE_KIND 6
#define THINKTHEN_COBOL_REQUESTINPUTFEED_KIND 7
#define THINKTHEN_COBOL_REQUESTORIGINALTEXT_KIND 1
#define THINKTHEN_COBOL_REQUESTORIGINALJSON_KIND 2
#define THINKTHEN_COBOL_REQUESTQUESTIONTEXT_KIND 1
#define THINKTHEN_COBOL_REQUESTQUESTIONDEFINITION_KIND 2
#define THINKTHEN_COBOL_REQUESTQUESTIONFILE_KIND 3
#define THINKTHEN_COBOL_REQUESTQUESTIONNAME_KIND 4
#define THINKTHEN_COBOL_REQUESTQUESTIONREFERENCE_KIND 5
#define THINKTHEN_COBOL_REQUESTTHRESHOLDARM1_KIND 1
#define THINKTHEN_COBOL_REQUESTTHRESHOLDARM2_KIND 2
#define THINKTHEN_COBOL_SOURCEUNITARM1_KIND 1
#define THINKTHEN_COBOL_SOURCEUNITARM2_KIND 2
#define THINKTHEN_COBOL_SOURCEUNITARM3_KIND 3
#define THINKTHEN_COBOL_CHOOSEBATCHARM1_KIND 1
#define THINKTHEN_COBOL_CHOOSEBATCHARM2_KIND 2
#define THINKTHEN_COBOL_DECIDEBATCHARM1_KIND 1
#define THINKTHEN_COBOL_DECIDEBATCHARM2_KIND 2
#define THINKTHEN_COBOL_SCOREBATCHARM1_KIND 1
#define THINKTHEN_COBOL_SCOREBATCHARM2_KIND 2
#define THINKTHEN_COBOL_TAGBATCHARM1_KIND 1
#define THINKTHEN_COBOL_TAGBATCHARM2_KIND 2
#define THINKTHEN_COBOL_DEFINITIONCHOOSEBATCHARM1_KIND 1
#define THINKTHEN_COBOL_DEFINITIONCHOOSEBATCHARM2_KIND 2
#define THINKTHEN_COBOL_DEFINITIONDECIDEBATCHARM1_KIND 1
#define THINKTHEN_COBOL_DEFINITIONDECIDEBATCHARM2_KIND 2
#define THINKTHEN_COBOL_DEFINITIONSCOREBATCHARM1_KIND 1
#define THINKTHEN_COBOL_DEFINITIONSCOREBATCHARM2_KIND 2
#define THINKTHEN_COBOL_DEFINITIONTAGBATCHARM1_KIND 1
#define THINKTHEN_COBOL_DEFINITIONTAGBATCHARM2_KIND 2
typedef struct thinkthen_cobol_AnnotateDefinition {
    uint64_t reserved;
    const void * m_batch;
    const void * m_profile;
    const void * m_questions;
    const void * m_threshold;
    const void * m_version;
} thinkthen_cobol_AnnotateDefinition;
typedef struct thinkthen_cobol_AnnotateQuestion {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_AnnotateQuestion;
typedef struct thinkthen_cobol_AnnotateQuestionChoose {
    uint64_t reserved;
    const void * m_choose;
    const void * m_context_schema;
    const void * m_item_schema;
    const void * m_name;
    const void * m_on;
    const void * m_options;
    const void * m_threshold;
    const void * m_wording_version;
} thinkthen_cobol_AnnotateQuestionChoose;
typedef struct thinkthen_cobol_AnnotateQuestionDecide {
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
} thinkthen_cobol_AnnotateQuestionDecide;
typedef struct thinkthen_cobol_AnnotateQuestionScore {
    uint64_t reserved;
    const void * m_context_schema;
    const void * m_item_schema;
    const void * m_levels;
    const void * m_name;
    const void * m_on;
    const void * m_score;
    const void * m_wording_version;
} thinkthen_cobol_AnnotateQuestionScore;
typedef struct thinkthen_cobol_AnnotateQuestionTag {
    uint64_t reserved;
    const void * m_context_schema;
    const void * m_item_schema;
    const void * m_labels;
    const void * m_name;
    const void * m_on;
    const void * m_tag;
    const void * m_threshold;
    const void * m_wording_version;
} thinkthen_cobol_AnnotateQuestionTag;
typedef struct thinkthen_cobol_Choose {
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
} thinkthen_cobol_Choose;
typedef struct thinkthen_cobol_Criterion {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_Criterion;
typedef struct thinkthen_cobol_Cut {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_Cut;
typedef struct thinkthen_cobol_Decide {
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
} thinkthen_cobol_Decide;
typedef struct thinkthen_cobol_Description {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_Description;
typedef struct thinkthen_cobol_Find {
    uint64_t reserved;
    const void * m_context_schema;
    const void * m_find;
    const void * m_item_schema;
    const void * m_model;
    const void * m_name;
    const void * m_on;
    const void * m_profile;
    const void * m_wording_version;
} thinkthen_cobol_Find;
typedef struct thinkthen_cobol_InputDeclaration {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_InputDeclaration;
typedef struct thinkthen_cobol_InputDeclarationObject {
    uint64_t reserved;
    const void * m_properties;
    const void * m_required;
    const void * m_type;
} thinkthen_cobol_InputDeclarationObject;
typedef struct thinkthen_cobol_InputDeclarationString {
    uint64_t reserved;
    const void * m_type;
} thinkthen_cobol_InputDeclarationString;
typedef struct thinkthen_cobol_InputProperty {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_InputProperty;
typedef struct thinkthen_cobol_InputPropertyArray {
    uint64_t reserved;
    const void * m_items;
    const void * m_type;
} thinkthen_cobol_InputPropertyArray;
typedef struct thinkthen_cobol_InputPropertyBoolean {
    uint64_t reserved;
    const void * m_type;
} thinkthen_cobol_InputPropertyBoolean;
typedef struct thinkthen_cobol_InputPropertyNumber {
    uint64_t reserved;
    const void * m_type;
} thinkthen_cobol_InputPropertyNumber;
typedef struct thinkthen_cobol_InputPropertyString {
    uint64_t reserved;
    const void * m_type;
} thinkthen_cobol_InputPropertyString;
typedef struct thinkthen_cobol_Labels {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_Labels;
typedef struct thinkthen_cobol_Levels {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_Levels;
typedef struct thinkthen_cobol_Name {
    const char * data;
    uint64_t len;
} thinkthen_cobol_Name;
typedef struct thinkthen_cobol_Options {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_Options;
typedef struct thinkthen_cobol_Pointers {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_Pointers;
typedef struct thinkthen_cobol_Profile {
    const char * data;
    uint64_t len;
} thinkthen_cobol_Profile;
typedef struct thinkthen_cobol_QuestionText {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_QuestionText;
typedef struct thinkthen_cobol_Relate {
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
} thinkthen_cobol_Relate;
typedef struct thinkthen_cobol_Relation {
    uint64_t reserved;
    const void * m_either;
    const void * m_name;
    const void * m_reads;
    const void * m_single;
    const void * m_source;
    const void * m_target;
} thinkthen_cobol_Relation;
typedef struct thinkthen_cobol_Score {
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
} thinkthen_cobol_Score;
typedef struct thinkthen_cobol_Tag {
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
} thinkthen_cobol_Tag;
typedef struct thinkthen_cobol_Threshold {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_Threshold;
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
typedef struct thinkthen_cobol_RecognizeDefinition {
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
} thinkthen_cobol_RecognizeDefinition;
typedef struct thinkthen_cobol_RecognizeQuestion {
    uint64_t reserved;
    const void * m_entity_definition;
    const void * m_instructions;
    const void * m_kinds;
    const void * m_mode;
    const void * m_relations;
    const void * m_snippet_pieces;
    const void * m_stage_context;
} thinkthen_cobol_RecognizeQuestion;
typedef struct thinkthen_cobol_RelateFields {
    uint64_t reserved;
    const void * m_kind;
    const void * m_name;
} thinkthen_cobol_RelateFields;
typedef struct thinkthen_cobol_RelateQuestion {
    uint64_t reserved;
    const void * m_fields;
    const void * m_relations;
} thinkthen_cobol_RelateQuestion;
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
    uint64_t reserved;
    const void * m_function;
    const void * m_input;
    const void * m_options;
    const void * m_question;
} thinkthen_cobol_RequestCall;
typedef struct thinkthen_cobol_RequestDefinition {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_RequestDefinition;
typedef struct thinkthen_cobol_DefinitionAnnotateDefinition {
    uint64_t reserved;
    const void * m_batch;
    const void * m_profile;
    const void * m_questions;
    const void * m_threshold;
    const void * m_version;
} thinkthen_cobol_DefinitionAnnotateDefinition;
typedef struct thinkthen_cobol_DefinitionChoose {
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
} thinkthen_cobol_DefinitionChoose;
typedef struct thinkthen_cobol_DefinitionDecide {
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
} thinkthen_cobol_DefinitionDecide;
typedef struct thinkthen_cobol_DefinitionFind {
    uint64_t reserved;
    const void * m_context_schema;
    const void * m_find;
    const void * m_item_schema;
    const void * m_model;
    const void * m_name;
    const void * m_on;
    const void * m_profile;
    const void * m_wording_version;
} thinkthen_cobol_DefinitionFind;
typedef struct thinkthen_cobol_DefinitionRelate {
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
} thinkthen_cobol_DefinitionRelate;
typedef struct thinkthen_cobol_DefinitionScore {
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
} thinkthen_cobol_DefinitionScore;
typedef struct thinkthen_cobol_DefinitionTag {
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
} thinkthen_cobol_DefinitionTag;
typedef struct thinkthen_cobol_DefinitionRecognizeDefinition {
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
} thinkthen_cobol_DefinitionRecognizeDefinition;
typedef struct thinkthen_cobol_RequestFraming {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_RequestFraming;
typedef struct thinkthen_cobol_RequestFunction {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_RequestFunction;
typedef struct thinkthen_cobol_RequestImage {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_RequestImage;
typedef struct thinkthen_cobol_RequestImageBytes {
    uint64_t reserved;
    const void * m_bytes;
    const void * m_kind;
    const void * m_media;
} thinkthen_cobol_RequestImageBytes;
typedef struct thinkthen_cobol_RequestImageFile {
    uint64_t reserved;
    const void * m_kind;
    const void * m_media;
    const void * m_path;
} thinkthen_cobol_RequestImageFile;
typedef struct thinkthen_cobol_RequestInput {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_RequestInput;
typedef struct thinkthen_cobol_RequestInputEntities {
    uint64_t reserved;
    const void * m_items;
    const void * m_kind;
} thinkthen_cobol_RequestInputEntities;
typedef struct thinkthen_cobol_RequestInputFeed {
    uint64_t reserved;
    const void * m_framing;
    const void * m_images;
    const void * m_kind;
    const void * m_name;
    const void * m_reading;
} thinkthen_cobol_RequestInputFeed;
typedef struct thinkthen_cobol_RequestInputJson {
    uint64_t reserved;
    const void * m_images;
    const void * m_kind;
    const void * m_value;
} thinkthen_cobol_RequestInputJson;
typedef struct thinkthen_cobol_RequestInputRecords {
    uint64_t reserved;
    const void * m_items;
    const void * m_kind;
} thinkthen_cobol_RequestInputRecords;
typedef struct thinkthen_cobol_RequestInputSource {
    uint64_t reserved;
    const void * m_kind;
    const void * m_source;
} thinkthen_cobol_RequestInputSource;
typedef struct thinkthen_cobol_RequestInputText {
    uint64_t reserved;
    const void * m_images;
    const void * m_kind;
    const void * m_text;
} thinkthen_cobol_RequestInputText;
typedef struct thinkthen_cobol_RequestInputUnits {
    uint64_t reserved;
    const void * m_items;
    const void * m_kind;
} thinkthen_cobol_RequestInputUnits;
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
typedef struct thinkthen_cobol_RequestOriginalJson {
    uint64_t reserved;
    const void * m_kind;
    const void * m_value;
} thinkthen_cobol_RequestOriginalJson;
typedef struct thinkthen_cobol_RequestOriginalText {
    uint64_t reserved;
    const void * m_kind;
    const void * m_text;
} thinkthen_cobol_RequestOriginalText;
typedef struct thinkthen_cobol_RequestQuestion {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_RequestQuestion;
typedef struct thinkthen_cobol_RequestQuestionDefinition {
    uint64_t reserved;
    const void * m_kind;
    const void * m_value;
} thinkthen_cobol_RequestQuestionDefinition;
typedef struct thinkthen_cobol_RequestQuestionFile {
    uint64_t reserved;
    const void * m_kind;
    const void * m_path;
} thinkthen_cobol_RequestQuestionFile;
typedef struct thinkthen_cobol_RequestQuestionName {
    uint64_t reserved;
    const void * m_kind;
    const void * m_name;
} thinkthen_cobol_RequestQuestionName;
typedef struct thinkthen_cobol_RequestQuestionReference {
    uint64_t reserved;
    const void * m_kind;
    const void * m_reference;
} thinkthen_cobol_RequestQuestionReference;
typedef struct thinkthen_cobol_RequestQuestionText {
    uint64_t reserved;
    const void * m_kind;
    const void * m_text;
} thinkthen_cobol_RequestQuestionText;
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
    const void * m_framing;
    const void * m_media;
    const void * m_paths;
    const void * m_reading;
} thinkthen_cobol_RequestSource;
typedef struct thinkthen_cobol_RequestThreshold {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_RequestThreshold;
typedef struct thinkthen_cobol_RequestVersion {
    uint64_t reserved;
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
typedef struct thinkthen_cobol_AnnotateDefinitionBatch {
    const char * data;
    uint64_t len;
} thinkthen_cobol_AnnotateDefinitionBatch;
typedef struct thinkthen_cobol_AnnotateDefinitionQuestions {
    const thinkthen_cobol_text * keys;
    const void *const * values;
    uint64_t len;
} thinkthen_cobol_AnnotateDefinitionQuestions;
typedef struct thinkthen_cobol_AnnotateDefinitionVersion {
    uint64_t reserved;
} thinkthen_cobol_AnnotateDefinitionVersion;
typedef struct thinkthen_cobol_AnnotateQuestionChooseName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_AnnotateQuestionChooseName;
typedef struct thinkthen_cobol_AnnotateQuestionChooseWordingVersion {
    int64_t value;
} thinkthen_cobol_AnnotateQuestionChooseWordingVersion;
typedef struct thinkthen_cobol_AnnotateQuestionDecideName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_AnnotateQuestionDecideName;
typedef struct thinkthen_cobol_AnnotateQuestionDecideWordingVersion {
    int64_t value;
} thinkthen_cobol_AnnotateQuestionDecideWordingVersion;
typedef struct thinkthen_cobol_AnnotateQuestionScoreName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_AnnotateQuestionScoreName;
typedef struct thinkthen_cobol_AnnotateQuestionScoreWordingVersion {
    int64_t value;
} thinkthen_cobol_AnnotateQuestionScoreWordingVersion;
typedef struct thinkthen_cobol_AnnotateQuestionTagName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_AnnotateQuestionTagName;
typedef struct thinkthen_cobol_AnnotateQuestionTagWordingVersion {
    int64_t value;
} thinkthen_cobol_AnnotateQuestionTagWordingVersion;
typedef struct thinkthen_cobol_ChooseBatch {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_ChooseBatch;
typedef struct thinkthen_cobol_ChooseName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_ChooseName;
typedef struct thinkthen_cobol_ChooseWordingVersion {
    int64_t value;
} thinkthen_cobol_ChooseWordingVersion;
typedef struct thinkthen_cobol_CriterionArm1 {
    const char * data;
    uint64_t len;
} thinkthen_cobol_CriterionArm1;
typedef struct thinkthen_cobol_CriterionArm2 {
    const char * data;
    uint64_t len;
} thinkthen_cobol_CriterionArm2;
typedef struct thinkthen_cobol_CriterionArm3 {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_CriterionArm3;
typedef struct thinkthen_cobol_CriterionArm4 {
    uint64_t reserved;
} thinkthen_cobol_CriterionArm4;
typedef struct thinkthen_cobol_CutArm1 {
    double value;
} thinkthen_cobol_CutArm1;
typedef struct thinkthen_cobol_CutArm2 {
    const char * data;
    uint64_t len;
} thinkthen_cobol_CutArm2;
typedef struct thinkthen_cobol_DecideBatch {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_DecideBatch;
typedef struct thinkthen_cobol_DecideName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_DecideName;
typedef struct thinkthen_cobol_DecideWordingVersion {
    int64_t value;
} thinkthen_cobol_DecideWordingVersion;
typedef struct thinkthen_cobol_DescriptionArm1 {
    const char * data;
    uint64_t len;
} thinkthen_cobol_DescriptionArm1;
typedef struct thinkthen_cobol_DescriptionArm2 {
    const char * data;
    uint64_t len;
} thinkthen_cobol_DescriptionArm2;
typedef struct thinkthen_cobol_DescriptionArm3 {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_DescriptionArm3;
typedef struct thinkthen_cobol_DescriptionArm4 {
    uint64_t reserved;
} thinkthen_cobol_DescriptionArm4;
typedef struct thinkthen_cobol_FindName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_FindName;
typedef struct thinkthen_cobol_FindWordingVersion {
    int64_t value;
} thinkthen_cobol_FindWordingVersion;
typedef struct thinkthen_cobol_InputDeclarationObjectProperties {
    const thinkthen_cobol_text * keys;
    const void *const * values;
    uint64_t len;
} thinkthen_cobol_InputDeclarationObjectProperties;
typedef struct thinkthen_cobol_InputDeclarationObjectRequired {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_InputDeclarationObjectRequired;
typedef struct thinkthen_cobol_InputDeclarationObjectType {
    uint64_t reserved;
} thinkthen_cobol_InputDeclarationObjectType;
typedef struct thinkthen_cobol_InputDeclarationStringType {
    uint64_t reserved;
} thinkthen_cobol_InputDeclarationStringType;
typedef struct thinkthen_cobol_InputPropertyArrayItems {
    uint64_t reserved;
    const void * m_type;
} thinkthen_cobol_InputPropertyArrayItems;
typedef struct thinkthen_cobol_InputPropertyArrayType {
    uint64_t reserved;
} thinkthen_cobol_InputPropertyArrayType;
typedef struct thinkthen_cobol_InputPropertyBooleanType {
    uint64_t reserved;
} thinkthen_cobol_InputPropertyBooleanType;
typedef struct thinkthen_cobol_InputPropertyNumberType {
    uint64_t reserved;
} thinkthen_cobol_InputPropertyNumberType;
typedef struct thinkthen_cobol_InputPropertyStringType {
    uint64_t reserved;
} thinkthen_cobol_InputPropertyStringType;
typedef struct thinkthen_cobol_LabelsArm1 {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_LabelsArm1;
typedef struct thinkthen_cobol_LabelsArm2 {
    const thinkthen_cobol_text * keys;
    const void *const * values;
    uint64_t len;
} thinkthen_cobol_LabelsArm2;
typedef struct thinkthen_cobol_LevelsArm1 {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_LevelsArm1;
typedef struct thinkthen_cobol_LevelsArm2 {
    const thinkthen_cobol_text * keys;
    const void *const * values;
    uint64_t len;
} thinkthen_cobol_LevelsArm2;
typedef struct thinkthen_cobol_OptionsArm1 {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_OptionsArm1;
typedef struct thinkthen_cobol_OptionsArm2 {
    const thinkthen_cobol_text * keys;
    const void *const * values;
    uint64_t len;
} thinkthen_cobol_OptionsArm2;
typedef struct thinkthen_cobol_PointersArm1 {
    const char * data;
    uint64_t len;
} thinkthen_cobol_PointersArm1;
typedef struct thinkthen_cobol_PointersArm2 {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_PointersArm2;
typedef struct thinkthen_cobol_QuestionTextArm1 {
    const char * data;
    uint64_t len;
} thinkthen_cobol_QuestionTextArm1;
typedef struct thinkthen_cobol_QuestionTextArm2 {
    const char * data;
    uint64_t len;
} thinkthen_cobol_QuestionTextArm2;
typedef struct thinkthen_cobol_QuestionTextArm3 {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_QuestionTextArm3;
typedef struct thinkthen_cobol_RelateName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RelateName;
typedef struct thinkthen_cobol_RelateVersion {
    uint64_t reserved;
} thinkthen_cobol_RelateVersion;
typedef struct thinkthen_cobol_RelateWordingVersion {
    int64_t value;
} thinkthen_cobol_RelateWordingVersion;
typedef struct thinkthen_cobol_RelationEither {
    uint64_t value;
} thinkthen_cobol_RelationEither;
typedef struct thinkthen_cobol_RelationSingle {
    uint64_t value;
} thinkthen_cobol_RelationSingle;
typedef struct thinkthen_cobol_ScoreBatch {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_ScoreBatch;
typedef struct thinkthen_cobol_ScoreName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_ScoreName;
typedef struct thinkthen_cobol_ScoreWordingVersion {
    int64_t value;
} thinkthen_cobol_ScoreWordingVersion;
typedef struct thinkthen_cobol_TagBatch {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_TagBatch;
typedef struct thinkthen_cobol_TagName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_TagName;
typedef struct thinkthen_cobol_TagWordingVersion {
    int64_t value;
} thinkthen_cobol_TagWordingVersion;
typedef struct thinkthen_cobol_ThresholdArm1 {
    double value;
} thinkthen_cobol_ThresholdArm1;
typedef struct thinkthen_cobol_ThresholdArm2 {
    const char * data;
    uint64_t len;
} thinkthen_cobol_ThresholdArm2;
typedef struct thinkthen_cobol_ContextSchemaArm1 {
    const char * data;
    uint64_t len;
} thinkthen_cobol_ContextSchemaArm1;
typedef struct thinkthen_cobol_ContextSchemaArm2 {
    const char * data;
    uint64_t len;
} thinkthen_cobol_ContextSchemaArm2;
typedef struct thinkthen_cobol_ImageMediaArm1 {
    uint64_t reserved;
} thinkthen_cobol_ImageMediaArm1;
typedef struct thinkthen_cobol_ImageMediaArm2 {
    uint64_t reserved;
} thinkthen_cobol_ImageMediaArm2;
typedef struct thinkthen_cobol_OptionSchemaDescription {
    const char * data;
    uint64_t len;
} thinkthen_cobol_OptionSchemaDescription;
typedef struct thinkthen_cobol_OptionSchemaName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_OptionSchemaName;
typedef struct thinkthen_cobol_ReaderMediaArm1 {
    uint64_t reserved;
} thinkthen_cobol_ReaderMediaArm1;
typedef struct thinkthen_cobol_ReaderMediaArm2 {
    uint64_t reserved;
} thinkthen_cobol_ReaderMediaArm2;
typedef struct thinkthen_cobol_RecognitionExampleArm1 {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RecognitionExampleArm1;
typedef struct thinkthen_cobol_RecognitionExampleEntityEnd {
    uint64_t value;
} thinkthen_cobol_RecognitionExampleEntityEnd;
typedef struct thinkthen_cobol_RecognitionExampleEntityKind {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RecognitionExampleEntityKind;
typedef struct thinkthen_cobol_RecognitionExampleEntityStart {
    uint64_t value;
} thinkthen_cobol_RecognitionExampleEntityStart;
typedef struct thinkthen_cobol_RecognitionExampleTextEntities {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_RecognitionExampleTextEntities;
typedef struct thinkthen_cobol_RecognitionExampleTextKinds {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_RecognitionExampleTextKinds;
typedef struct thinkthen_cobol_RecognitionExampleTextText {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RecognitionExampleTextText;
typedef struct thinkthen_cobol_RecognitionModeArm1 {
    uint64_t reserved;
} thinkthen_cobol_RecognitionModeArm1;
typedef struct thinkthen_cobol_RecognitionModeArm2 {
    uint64_t reserved;
} thinkthen_cobol_RecognitionModeArm2;
typedef struct thinkthen_cobol_RecognitionSeedSpanEnd {
    uint64_t value;
} thinkthen_cobol_RecognitionSeedSpanEnd;
typedef struct thinkthen_cobol_RecognitionSeedSpanKind {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RecognitionSeedSpanKind;
typedef struct thinkthen_cobol_RecognitionSeedSpanStart {
    uint64_t value;
} thinkthen_cobol_RecognitionSeedSpanStart;
typedef struct thinkthen_cobol_RecognitionStageContextBoundary {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RecognitionStageContextBoundary;
typedef struct thinkthen_cobol_RecognitionStageContextKindEdge {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RecognitionStageContextKindEdge;
typedef struct thinkthen_cobol_RecognitionStageContextRelation {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RecognitionStageContextRelation;
typedef struct thinkthen_cobol_RecognizeDefinitionName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RecognizeDefinitionName;
typedef struct thinkthen_cobol_RecognizeDefinitionVersion {
    uint64_t reserved;
} thinkthen_cobol_RecognizeDefinitionVersion;
typedef struct thinkthen_cobol_RecognizeDefinitionWordingVersion {
    int64_t value;
} thinkthen_cobol_RecognizeDefinitionWordingVersion;
typedef struct thinkthen_cobol_RecognizeQuestionKinds {
    const thinkthen_cobol_text * keys;
    const void *const * values;
    uint64_t len;
} thinkthen_cobol_RecognizeQuestionKinds;
typedef struct thinkthen_cobol_RecognizeQuestionRelations {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_RecognizeQuestionRelations;
typedef struct thinkthen_cobol_RecognizeQuestionSnippetPieces {
    uint64_t value;
} thinkthen_cobol_RecognizeQuestionSnippetPieces;
typedef struct thinkthen_cobol_RelateFieldsKind {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RelateFieldsKind;
typedef struct thinkthen_cobol_RelateFieldsName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RelateFieldsName;
typedef struct thinkthen_cobol_RelateQuestionRelations {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_RelateQuestionRelations;
typedef struct thinkthen_cobol_RequestBatchArm1 {
    uint64_t value;
} thinkthen_cobol_RequestBatchArm1;
typedef struct thinkthen_cobol_RequestBatchArm2 {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestBatchArm2;
typedef struct thinkthen_cobol_DefinitionAnnotateDefinitionBatch {
    const char * data;
    uint64_t len;
} thinkthen_cobol_DefinitionAnnotateDefinitionBatch;
typedef struct thinkthen_cobol_DefinitionAnnotateDefinitionQuestions {
    const thinkthen_cobol_text * keys;
    const void *const * values;
    uint64_t len;
} thinkthen_cobol_DefinitionAnnotateDefinitionQuestions;
typedef struct thinkthen_cobol_DefinitionAnnotateDefinitionVersion {
    uint64_t reserved;
} thinkthen_cobol_DefinitionAnnotateDefinitionVersion;
typedef struct thinkthen_cobol_DefinitionChooseBatch {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_DefinitionChooseBatch;
typedef struct thinkthen_cobol_DefinitionChooseName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_DefinitionChooseName;
typedef struct thinkthen_cobol_DefinitionChooseWordingVersion {
    int64_t value;
} thinkthen_cobol_DefinitionChooseWordingVersion;
typedef struct thinkthen_cobol_DefinitionDecideBatch {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_DefinitionDecideBatch;
typedef struct thinkthen_cobol_DefinitionDecideName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_DefinitionDecideName;
typedef struct thinkthen_cobol_DefinitionDecideWordingVersion {
    int64_t value;
} thinkthen_cobol_DefinitionDecideWordingVersion;
typedef struct thinkthen_cobol_DefinitionFindName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_DefinitionFindName;
typedef struct thinkthen_cobol_DefinitionFindWordingVersion {
    int64_t value;
} thinkthen_cobol_DefinitionFindWordingVersion;
typedef struct thinkthen_cobol_DefinitionRelateName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_DefinitionRelateName;
typedef struct thinkthen_cobol_DefinitionRelateVersion {
    uint64_t reserved;
} thinkthen_cobol_DefinitionRelateVersion;
typedef struct thinkthen_cobol_DefinitionRelateWordingVersion {
    int64_t value;
} thinkthen_cobol_DefinitionRelateWordingVersion;
typedef struct thinkthen_cobol_DefinitionScoreBatch {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_DefinitionScoreBatch;
typedef struct thinkthen_cobol_DefinitionScoreName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_DefinitionScoreName;
typedef struct thinkthen_cobol_DefinitionScoreWordingVersion {
    int64_t value;
} thinkthen_cobol_DefinitionScoreWordingVersion;
typedef struct thinkthen_cobol_DefinitionTagBatch {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_DefinitionTagBatch;
typedef struct thinkthen_cobol_DefinitionTagName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_DefinitionTagName;
typedef struct thinkthen_cobol_DefinitionTagWordingVersion {
    int64_t value;
} thinkthen_cobol_DefinitionTagWordingVersion;
typedef struct thinkthen_cobol_DefinitionRecognizeDefinitionName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_DefinitionRecognizeDefinitionName;
typedef struct thinkthen_cobol_DefinitionRecognizeDefinitionVersion {
    uint64_t reserved;
} thinkthen_cobol_DefinitionRecognizeDefinitionVersion;
typedef struct thinkthen_cobol_DefinitionRecognizeDefinitionWordingVersion {
    int64_t value;
} thinkthen_cobol_DefinitionRecognizeDefinitionWordingVersion;
typedef struct thinkthen_cobol_RequestFramingArm1 {
    uint64_t reserved;
} thinkthen_cobol_RequestFramingArm1;
typedef struct thinkthen_cobol_RequestFramingArm2 {
    uint64_t reserved;
} thinkthen_cobol_RequestFramingArm2;
typedef struct thinkthen_cobol_RequestFramingArm3 {
    uint64_t reserved;
} thinkthen_cobol_RequestFramingArm3;
typedef struct thinkthen_cobol_RequestFramingArm4 {
    uint64_t reserved;
} thinkthen_cobol_RequestFramingArm4;
typedef struct thinkthen_cobol_RequestFramingArm5 {
    uint64_t reserved;
} thinkthen_cobol_RequestFramingArm5;
typedef struct thinkthen_cobol_RequestFunctionArm1 {
    uint64_t reserved;
} thinkthen_cobol_RequestFunctionArm1;
typedef struct thinkthen_cobol_RequestFunctionArm2 {
    uint64_t reserved;
} thinkthen_cobol_RequestFunctionArm2;
typedef struct thinkthen_cobol_RequestFunctionArm3 {
    uint64_t reserved;
} thinkthen_cobol_RequestFunctionArm3;
typedef struct thinkthen_cobol_RequestFunctionArm4 {
    uint64_t reserved;
} thinkthen_cobol_RequestFunctionArm4;
typedef struct thinkthen_cobol_RequestFunctionArm5 {
    uint64_t reserved;
} thinkthen_cobol_RequestFunctionArm5;
typedef struct thinkthen_cobol_RequestFunctionArm6 {
    uint64_t reserved;
} thinkthen_cobol_RequestFunctionArm6;
typedef struct thinkthen_cobol_RequestFunctionArm7 {
    uint64_t reserved;
} thinkthen_cobol_RequestFunctionArm7;
typedef struct thinkthen_cobol_RequestFunctionArm8 {
    uint64_t reserved;
} thinkthen_cobol_RequestFunctionArm8;
typedef struct thinkthen_cobol_RequestFunctionArm9 {
    uint64_t reserved;
} thinkthen_cobol_RequestFunctionArm9;
typedef struct thinkthen_cobol_RequestFunctionArm10 {
    uint64_t reserved;
} thinkthen_cobol_RequestFunctionArm10;
typedef struct thinkthen_cobol_RequestImageBytesBytes {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestImageBytesBytes;
typedef struct thinkthen_cobol_RequestImageBytesKind {
    uint64_t reserved;
} thinkthen_cobol_RequestImageBytesKind;
typedef struct thinkthen_cobol_RequestImageFileKind {
    uint64_t reserved;
} thinkthen_cobol_RequestImageFileKind;
typedef struct thinkthen_cobol_RequestImageFilePath {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestImageFilePath;
typedef struct thinkthen_cobol_RequestInputEntitiesItems {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_RequestInputEntitiesItems;
typedef struct thinkthen_cobol_RequestInputEntitiesKind {
    uint64_t reserved;
} thinkthen_cobol_RequestInputEntitiesKind;
typedef struct thinkthen_cobol_RequestInputFeedImages {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_RequestInputFeedImages;
typedef struct thinkthen_cobol_RequestInputFeedKind {
    uint64_t reserved;
} thinkthen_cobol_RequestInputFeedKind;
typedef struct thinkthen_cobol_RequestInputFeedName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestInputFeedName;
typedef struct thinkthen_cobol_RequestInputJsonImages {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_RequestInputJsonImages;
typedef struct thinkthen_cobol_RequestInputJsonKind {
    uint64_t reserved;
} thinkthen_cobol_RequestInputJsonKind;
typedef struct thinkthen_cobol_RequestInputJsonValue {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestInputJsonValue;
typedef struct thinkthen_cobol_RequestInputRecordsItems {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_RequestInputRecordsItems;
typedef struct thinkthen_cobol_RequestInputRecordsKind {
    uint64_t reserved;
} thinkthen_cobol_RequestInputRecordsKind;
typedef struct thinkthen_cobol_RequestInputSourceKind {
    uint64_t reserved;
} thinkthen_cobol_RequestInputSourceKind;
typedef struct thinkthen_cobol_RequestInputTextImages {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_RequestInputTextImages;
typedef struct thinkthen_cobol_RequestInputTextKind {
    uint64_t reserved;
} thinkthen_cobol_RequestInputTextKind;
typedef struct thinkthen_cobol_RequestInputTextText {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestInputTextText;
typedef struct thinkthen_cobol_RequestInputUnitsItems {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_RequestInputUnitsItems;
typedef struct thinkthen_cobol_RequestInputUnitsKind {
    uint64_t reserved;
} thinkthen_cobol_RequestInputUnitsKind;
typedef struct thinkthen_cobol_RequestItemExamples {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_RequestItemExamples;
typedef struct thinkthen_cobol_RequestItemImages {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_RequestItemImages;
typedef struct thinkthen_cobol_RequestItemOptions {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_RequestItemOptions;
typedef struct thinkthen_cobol_RequestItemSeedSpans {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_RequestItemSeedSpans;
typedef struct thinkthen_cobol_RequestOptionsAttempts {
    uint64_t value;
} thinkthen_cobol_RequestOptionsAttempts;
typedef struct thinkthen_cobol_RequestOptionsContext {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestOptionsContext;
typedef struct thinkthen_cobol_RequestOptionsContextField {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestOptionsContextField;
typedef struct thinkthen_cobol_RequestOptionsDeadlineMs {
    int64_t value;
} thinkthen_cobol_RequestOptionsDeadlineMs;
typedef struct thinkthen_cobol_RequestOptionsDetails {
    uint64_t value;
} thinkthen_cobol_RequestOptionsDetails;
typedef struct thinkthen_cobol_RequestOptionsExamples {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_RequestOptionsExamples;
typedef struct thinkthen_cobol_RequestOptionsExamplesField {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestOptionsExamplesField;
typedef struct thinkthen_cobol_RequestOptionsField {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_RequestOptionsField;
typedef struct thinkthen_cobol_RequestOptionsFilesOnly {
    uint64_t value;
} thinkthen_cobol_RequestOptionsFilesOnly;
typedef struct thinkthen_cobol_RequestOptionsMaxRequestsTotal {
    uint64_t value;
} thinkthen_cobol_RequestOptionsMaxRequestsTotal;
typedef struct thinkthen_cobol_RequestOptionsModel {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestOptionsModel;
typedef struct thinkthen_cobol_RequestOptionsNone {
    uint64_t value;
} thinkthen_cobol_RequestOptionsNone;
typedef struct thinkthen_cobol_RequestOptionsOptionsField {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestOptionsOptionsField;
typedef struct thinkthen_cobol_RequestOptionsSeedSpans {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_RequestOptionsSeedSpans;
typedef struct thinkthen_cobol_RequestOptionsSeedSpansField {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestOptionsSeedSpansField;
typedef struct thinkthen_cobol_RequestOptionsSnippetPieces {
    uint64_t value;
} thinkthen_cobol_RequestOptionsSnippetPieces;
typedef struct thinkthen_cobol_RequestOptionsTop {
    uint64_t value;
} thinkthen_cobol_RequestOptionsTop;
typedef struct thinkthen_cobol_RequestOriginalJsonKind {
    uint64_t reserved;
} thinkthen_cobol_RequestOriginalJsonKind;
typedef struct thinkthen_cobol_RequestOriginalJsonValue {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestOriginalJsonValue;
typedef struct thinkthen_cobol_RequestOriginalTextKind {
    uint64_t reserved;
} thinkthen_cobol_RequestOriginalTextKind;
typedef struct thinkthen_cobol_RequestOriginalTextText {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestOriginalTextText;
typedef struct thinkthen_cobol_RequestQuestionDefinitionKind {
    uint64_t reserved;
} thinkthen_cobol_RequestQuestionDefinitionKind;
typedef struct thinkthen_cobol_RequestQuestionFileKind {
    uint64_t reserved;
} thinkthen_cobol_RequestQuestionFileKind;
typedef struct thinkthen_cobol_RequestQuestionFilePath {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestQuestionFilePath;
typedef struct thinkthen_cobol_RequestQuestionNameKind {
    uint64_t reserved;
} thinkthen_cobol_RequestQuestionNameKind;
typedef struct thinkthen_cobol_RequestQuestionNameName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestQuestionNameName;
typedef struct thinkthen_cobol_RequestQuestionReferenceKind {
    uint64_t reserved;
} thinkthen_cobol_RequestQuestionReferenceKind;
typedef struct thinkthen_cobol_RequestQuestionReferenceReference {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestQuestionReferenceReference;
typedef struct thinkthen_cobol_RequestQuestionTextKind {
    uint64_t reserved;
} thinkthen_cobol_RequestQuestionTextKind;
typedef struct thinkthen_cobol_RequestQuestionTextText {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestQuestionTextText;
typedef struct thinkthen_cobol_RequestReaderWindow {
    uint64_t value;
} thinkthen_cobol_RequestReaderWindow;
typedef struct thinkthen_cobol_RequestSourcePaths {
    const void *const * data;
    uint64_t len;
} thinkthen_cobol_RequestSourcePaths;
typedef struct thinkthen_cobol_RequestThresholdArm1 {
    double value;
} thinkthen_cobol_RequestThresholdArm1;
typedef struct thinkthen_cobol_RequestThresholdArm2 {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestThresholdArm2;
typedef struct thinkthen_cobol_SessionSourceLocationFile {
    const char * data;
    uint64_t len;
} thinkthen_cobol_SessionSourceLocationFile;
typedef struct thinkthen_cobol_SessionSourceLocationFirstLine {
    uint64_t value;
} thinkthen_cobol_SessionSourceLocationFirstLine;
typedef struct thinkthen_cobol_SessionSourceLocationLastLine {
    uint64_t value;
} thinkthen_cobol_SessionSourceLocationLastLine;
typedef struct thinkthen_cobol_SourceUnitArm1 {
    uint64_t reserved;
} thinkthen_cobol_SourceUnitArm1;
typedef struct thinkthen_cobol_SourceUnitArm2 {
    uint64_t reserved;
} thinkthen_cobol_SourceUnitArm2;
typedef struct thinkthen_cobol_SourceUnitArm3 {
    uint64_t reserved;
} thinkthen_cobol_SourceUnitArm3;
typedef struct thinkthen_cobol_ChooseBatchArm1 {
    uint64_t reserved;
} thinkthen_cobol_ChooseBatchArm1;
typedef struct thinkthen_cobol_ChooseBatchArm2 {
    int64_t value;
} thinkthen_cobol_ChooseBatchArm2;
typedef struct thinkthen_cobol_CriterionArm3Item {
    const char * data;
    uint64_t len;
} thinkthen_cobol_CriterionArm3Item;
typedef struct thinkthen_cobol_DecideBatchArm1 {
    uint64_t reserved;
} thinkthen_cobol_DecideBatchArm1;
typedef struct thinkthen_cobol_DecideBatchArm2 {
    int64_t value;
} thinkthen_cobol_DecideBatchArm2;
typedef struct thinkthen_cobol_DescriptionArm3Item {
    const char * data;
    uint64_t len;
} thinkthen_cobol_DescriptionArm3Item;
typedef struct thinkthen_cobol_InputDeclarationObjectRequiredItem {
    const char * data;
    uint64_t len;
} thinkthen_cobol_InputDeclarationObjectRequiredItem;
typedef struct thinkthen_cobol_InputPropertyArrayItemsType {
    uint64_t reserved;
} thinkthen_cobol_InputPropertyArrayItemsType;
typedef struct thinkthen_cobol_PointersArm2Item {
    const char * data;
    uint64_t len;
} thinkthen_cobol_PointersArm2Item;
typedef struct thinkthen_cobol_QuestionTextArm3Item {
    const char * data;
    uint64_t len;
} thinkthen_cobol_QuestionTextArm3Item;
typedef struct thinkthen_cobol_ScoreBatchArm1 {
    uint64_t reserved;
} thinkthen_cobol_ScoreBatchArm1;
typedef struct thinkthen_cobol_ScoreBatchArm2 {
    int64_t value;
} thinkthen_cobol_ScoreBatchArm2;
typedef struct thinkthen_cobol_TagBatchArm1 {
    uint64_t reserved;
} thinkthen_cobol_TagBatchArm1;
typedef struct thinkthen_cobol_TagBatchArm2 {
    int64_t value;
} thinkthen_cobol_TagBatchArm2;
typedef struct thinkthen_cobol_RecognitionExampleTextKindsItem {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RecognitionExampleTextKindsItem;
typedef struct thinkthen_cobol_DefinitionChooseBatchArm1 {
    uint64_t reserved;
} thinkthen_cobol_DefinitionChooseBatchArm1;
typedef struct thinkthen_cobol_DefinitionChooseBatchArm2 {
    int64_t value;
} thinkthen_cobol_DefinitionChooseBatchArm2;
typedef struct thinkthen_cobol_DefinitionDecideBatchArm1 {
    uint64_t reserved;
} thinkthen_cobol_DefinitionDecideBatchArm1;
typedef struct thinkthen_cobol_DefinitionDecideBatchArm2 {
    int64_t value;
} thinkthen_cobol_DefinitionDecideBatchArm2;
typedef struct thinkthen_cobol_DefinitionScoreBatchArm1 {
    uint64_t reserved;
} thinkthen_cobol_DefinitionScoreBatchArm1;
typedef struct thinkthen_cobol_DefinitionScoreBatchArm2 {
    int64_t value;
} thinkthen_cobol_DefinitionScoreBatchArm2;
typedef struct thinkthen_cobol_DefinitionTagBatchArm1 {
    uint64_t reserved;
} thinkthen_cobol_DefinitionTagBatchArm1;
typedef struct thinkthen_cobol_DefinitionTagBatchArm2 {
    int64_t value;
} thinkthen_cobol_DefinitionTagBatchArm2;
typedef struct thinkthen_cobol_RequestOptionsFieldItem {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestOptionsFieldItem;
typedef struct thinkthen_cobol_RequestSourcePathsItem {
    const char * data;
    uint64_t len;
} thinkthen_cobol_RequestSourcePathsItem;
int TT_SESSION_DECIDE(const void *,const thinkthen_cobol_RequestCall *,void **);
int TT_SESSION_CHOOSE(const void *,const thinkthen_cobol_RequestCall *,void **);
int TT_SESSION_TAG(const void *,const thinkthen_cobol_RequestCall *,void **);
int TT_SESSION_SCORE(const void *,const thinkthen_cobol_RequestCall *,void **);
int TT_SESSION_FILTER(const void *,const thinkthen_cobol_RequestCall *,void **);
int TT_SESSION_RANK(const void *,const thinkthen_cobol_RequestCall *,void **);
int TT_SESSION_FIND(const void *,const thinkthen_cobol_RequestCall *,void **);
int TT_SESSION_ANNOTATE(const void *,const thinkthen_cobol_RequestCall *,void **);
int TT_SESSION_RECOGNIZE(const void *,const thinkthen_cobol_RequestCall *,void **);
int TT_SESSION_RELATE(const void *,const thinkthen_cobol_RequestCall *,void **);
int TT_SESSION_PUSH(const void *,const thinkthen_cobol_RequestSessionDescriptor *,uint32_t *);
int TT_SESSION_REQUEST(const void *engine,const thinkthen_cobol_Request *request,void **out);
#endif
