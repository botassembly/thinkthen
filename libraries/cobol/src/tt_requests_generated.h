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
#define THINKTHEN_COBOL_CACHEDOCUMENTARM1_KIND 1
#define THINKTHEN_COBOL_CACHEDOCUMENT_ARM_2_KIND 2
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
#define THINKTHEN_COBOL_REQUESTREADERFAILUREPRODUCER_KIND 1
#define THINKTHEN_COBOL_REQUESTREADERFAILUREIO_KIND 2
#define THINKTHEN_COBOL_REQUESTREADERFAILUREUTF8_KIND 3
#define THINKTHEN_COBOL_REQUESTREADERFAILUREINVALIDINPUT_KIND 4
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
#define THINKTHEN_COBOL_SETTINGSINPUTTOKENBUDGETARM1_KIND 1
#define THINKTHEN_COBOL_SETTINGSINPUTTOKENBUDGETARM2_KIND 2
#define THINKTHEN_COBOL_SETTINGSMAXREQUESTSARM1_KIND 1
#define THINKTHEN_COBOL_SETTINGSMAXREQUESTSARM2_KIND 2
#define THINKTHEN_COBOL_SETTINGSMAXREQUESTSTOTALARM1_KIND 1
#define THINKTHEN_COBOL_SETTINGSMAXREQUESTSTOTALARM2_KIND 2
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
typedef struct thinkthen_cobol_CacheDocument {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_CacheDocument;
typedef struct thinkthen_cobol_ContextSchema {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_ContextSchema;
typedef struct thinkthen_cobol_DisabledCache {
    uint64_t reserved;
} thinkthen_cobol_DisabledCache;
typedef struct thinkthen_cobol_Settings {
    uint64_t reserved;
    const void * m_backend;
    const void * m_base_url;
    const void * m_batch;
    const void * m_cache;
    const void * m_max_estimated_input_tokens_total;
    const void * m_max_request_bytes;
    const void * m_max_requests;
    const void * m_max_requests_total;
    const void * m_max_retries;
    const void * m_model;
    const void * m_profile;
    const void * m_proxy;
    const void * m_record;
    const void * m_refresh_cache;
    const void * m_replay;
    const void * m_throttle;
    const void * m_timeout;
    const void * m_usd_per_million_input;
    const void * m_usd_per_million_output;
} thinkthen_cobol_Settings;
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
typedef struct thinkthen_cobol_RequestReaderFailure {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_RequestReaderFailure;
typedef struct thinkthen_cobol_RequestReaderFailureInvalidInput {
    uint64_t reserved;
    const void * m_kind;
    const void * m_location;
} thinkthen_cobol_RequestReaderFailureInvalidInput;
typedef struct thinkthen_cobol_RequestReaderFailureIo {
    uint64_t reserved;
    const void * m_kind;
    const void * m_location;
} thinkthen_cobol_RequestReaderFailureIo;
typedef struct thinkthen_cobol_RequestReaderFailureProducer {
    uint64_t reserved;
    const void * m_kind;
} thinkthen_cobol_RequestReaderFailureProducer;
typedef struct thinkthen_cobol_RequestReaderFailureUtf8 {
    uint64_t reserved;
    const void * m_kind;
    const void * m_location;
} thinkthen_cobol_RequestReaderFailureUtf8;
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
    const struct thinkthen_complete_json_v1 * value;
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
    const thinkthen_cobol_text * keys;
    const void *const * values;
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
    const thinkthen_cobol_text * keys;
    const void *const * values;
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
    const thinkthen_cobol_text * keys;
    const void *const * values;
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
typedef struct thinkthen_cobol_CacheDocumentArm1 {
    const char * data;
    uint64_t len;
} thinkthen_cobol_CacheDocumentArm1;
typedef struct thinkthen_cobol_ContextSchemaArm1 {
    const char * data;
    uint64_t len;
} thinkthen_cobol_ContextSchemaArm1;
typedef struct thinkthen_cobol_ContextSchemaArm2 {
    const thinkthen_cobol_text * keys;
    const void *const * values;
    uint64_t len;
} thinkthen_cobol_ContextSchemaArm2;
typedef struct thinkthen_cobol_SettingsBackend {
    const char * data;
    uint64_t len;
} thinkthen_cobol_SettingsBackend;
typedef struct thinkthen_cobol_SettingsBaseUrl {
    const char * data;
    uint64_t len;
} thinkthen_cobol_SettingsBaseUrl;
typedef struct thinkthen_cobol_SettingsInputTokenBudget {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_SettingsInputTokenBudget;
typedef struct thinkthen_cobol_SettingsMaxRequestBytes {
    uint64_t value;
} thinkthen_cobol_SettingsMaxRequestBytes;
typedef struct thinkthen_cobol_SettingsMaxRequests {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_SettingsMaxRequests;
typedef struct thinkthen_cobol_SettingsMaxRequestsTotal {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_SettingsMaxRequestsTotal;
typedef struct thinkthen_cobol_SettingsMaxRetries {
    uint64_t value;
} thinkthen_cobol_SettingsMaxRetries;
typedef struct thinkthen_cobol_SettingsModel {
    const char * data;
    uint64_t len;
} thinkthen_cobol_SettingsModel;
typedef struct thinkthen_cobol_SettingsProfile {
    const char * data;
    uint64_t len;
} thinkthen_cobol_SettingsProfile;
typedef struct thinkthen_cobol_SettingsProxy {
    const struct thinkthen_complete_json_v1 * value;
} thinkthen_cobol_SettingsProxy;
typedef struct thinkthen_cobol_SettingsRecord {
    const char * data;
    uint64_t len;
} thinkthen_cobol_SettingsRecord;
typedef struct thinkthen_cobol_SettingsRefreshCache {
    uint64_t value;
} thinkthen_cobol_SettingsRefreshCache;
typedef struct thinkthen_cobol_SettingsReplay {
    const char * data;
    uint64_t len;
} thinkthen_cobol_SettingsReplay;
typedef struct thinkthen_cobol_SettingsThrottle {
    uint64_t value;
} thinkthen_cobol_SettingsThrottle;
typedef struct thinkthen_cobol_SettingsTimeout {
    uint64_t value;
} thinkthen_cobol_SettingsTimeout;
typedef struct thinkthen_cobol_SettingsUsdPerMillionInput {
    const char * data;
    uint64_t len;
} thinkthen_cobol_SettingsUsdPerMillionInput;
typedef struct thinkthen_cobol_SettingsUsdPerMillionOutput {
    const char * data;
    uint64_t len;
} thinkthen_cobol_SettingsUsdPerMillionOutput;
typedef struct thinkthen_cobol_ImageMediaArm1 {
    uint64_t reserved;
} thinkthen_cobol_ImageMediaArm1;
typedef struct thinkthen_cobol_ImageMediaArm2 {
    uint64_t reserved;
} thinkthen_cobol_ImageMediaArm2;
typedef struct thinkthen_cobol_OptionSchemaDescription {
    const struct thinkthen_complete_json_v1 * value;
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
    const struct thinkthen_complete_json_v1 * value;
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
    const struct thinkthen_complete_json_v1 * value;
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
    const struct thinkthen_complete_json_v1 * value;
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
typedef struct thinkthen_cobol_RequestReaderFailureInvalidInputKind {
    uint64_t reserved;
} thinkthen_cobol_RequestReaderFailureInvalidInputKind;
typedef struct thinkthen_cobol_RequestReaderFailureIoKind {
    uint64_t reserved;
} thinkthen_cobol_RequestReaderFailureIoKind;
typedef struct thinkthen_cobol_RequestReaderFailureProducerKind {
    uint64_t reserved;
} thinkthen_cobol_RequestReaderFailureProducerKind;
typedef struct thinkthen_cobol_RequestReaderFailureUtf8Kind {
    uint64_t reserved;
} thinkthen_cobol_RequestReaderFailureUtf8Kind;
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
typedef struct thinkthen_cobol_CriterionArm2Entry {
    const struct thinkthen_complete_json_v1 * value;
} thinkthen_cobol_CriterionArm2Entry;
typedef struct thinkthen_cobol_CriterionArm3Item {
    const struct thinkthen_complete_json_v1 * value;
} thinkthen_cobol_CriterionArm3Item;
typedef struct thinkthen_cobol_DecideBatchArm1 {
    uint64_t reserved;
} thinkthen_cobol_DecideBatchArm1;
typedef struct thinkthen_cobol_DecideBatchArm2 {
    int64_t value;
} thinkthen_cobol_DecideBatchArm2;
typedef struct thinkthen_cobol_DescriptionArm2Entry {
    const struct thinkthen_complete_json_v1 * value;
} thinkthen_cobol_DescriptionArm2Entry;
typedef struct thinkthen_cobol_DescriptionArm3Item {
    const struct thinkthen_complete_json_v1 * value;
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
typedef struct thinkthen_cobol_QuestionTextArm2Entry {
    const struct thinkthen_complete_json_v1 * value;
} thinkthen_cobol_QuestionTextArm2Entry;
typedef struct thinkthen_cobol_QuestionTextArm3Item {
    const struct thinkthen_complete_json_v1 * value;
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
typedef struct thinkthen_cobol_ContextSchemaArm2Entry {
    const struct thinkthen_complete_json_v1 * value;
} thinkthen_cobol_ContextSchemaArm2Entry;
typedef struct thinkthen_cobol_SettingsInputTokenBudgetArm1 {
    uint64_t value;
} thinkthen_cobol_SettingsInputTokenBudgetArm1;
typedef struct thinkthen_cobol_SettingsInputTokenBudgetArm2 {
    uint64_t reserved;
} thinkthen_cobol_SettingsInputTokenBudgetArm2;
typedef struct thinkthen_cobol_SettingsMaxRequestsArm1 {
    uint64_t value;
} thinkthen_cobol_SettingsMaxRequestsArm1;
typedef struct thinkthen_cobol_SettingsMaxRequestsArm2 {
    uint64_t reserved;
} thinkthen_cobol_SettingsMaxRequestsArm2;
typedef struct thinkthen_cobol_SettingsMaxRequestsTotalArm1 {
    uint64_t value;
} thinkthen_cobol_SettingsMaxRequestsTotalArm1;
typedef struct thinkthen_cobol_SettingsMaxRequestsTotalArm2 {
    uint64_t reserved;
} thinkthen_cobol_SettingsMaxRequestsTotalArm2;
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
#define TT_VALUE_NULL_KIND 1
#define TT_VALUE_BOOLEAN_KIND 2
#define TT_VALUE_NUMBER_KIND 3
#define TT_VALUE_STRING_KIND 4
#define TT_VALUE_ARRAY_KIND 5
#define TT_VALUE_OBJECT_KIND 6
typedef struct tt_value tt_value;
int TT_ENGINE_NEW(const thinkthen_cobol_Settings *,void **);
int TT_PLAN(const void *,const char *,uint64_t,const thinkthen_cobol_RequestCall *,tt_value **);
int TT_SESSION_PUSH(const void *,const thinkthen_cobol_RequestSessionDescriptor *,uint32_t *);
int TT_SESSION_REQUEST(const void *engine,const thinkthen_cobol_Request *request,void **out);
typedef struct tt_answer_owner tt_answer_owner;
typedef struct thinkthen_cobol_AnswerText {
    uint64_t length;
    char text[8192];
} thinkthen_cobol_AnswerText;
typedef struct thinkthen_cobol_AnswerDecision {
    uint64_t present;
    uint64_t value;
} thinkthen_cobol_AnswerDecision;
typedef struct thinkthen_cobol_AnswerChoice {
    uint64_t present;
    thinkthen_cobol_AnswerText value;
} thinkthen_cobol_AnswerChoice;
typedef struct thinkthen_cobol_AnswerScore {
    double value;
} thinkthen_cobol_AnswerScore;
typedef struct thinkthen_cobol_AnswerRankIndex {
    uint64_t value;
} thinkthen_cobol_AnswerRankIndex;
typedef struct thinkthen_cobol_AnswerFilterOriginal {
    const tt_value *original;
} thinkthen_cobol_AnswerFilterOriginal;
typedef struct thinkthen_cobol_AnswerFindSelection {
    uint64_t present;
    uint64_t index;
    const tt_value *original;
} thinkthen_cobol_AnswerFindSelection;
typedef struct thinkthen_cobol_AnswerAnnotationField {
    uint64_t record;
    thinkthen_cobol_AnswerText name;
    uint64_t kind;
    uint64_t boolean;
    double number;
    thinkthen_cobol_AnswerText text;
    const thinkthen_cobol_AnswerText *tags;
    uint64_t tag_count;
    uint64_t failure_kind;
    uint64_t failure_cause;
} thinkthen_cobol_AnswerAnnotationField;
typedef struct thinkthen_cobol_AnswerRecognition {
    uint64_t record;
    uint64_t proposal;
    thinkthen_cobol_AnswerText kind;
    thinkthen_cobol_AnswerText text;
    uint64_t start;
    uint64_t end;
    uint64_t length;
    double probability;
    uint64_t mode;
    uint64_t file_present;
    thinkthen_cobol_AnswerText file;
    uint64_t first_line_present;
    uint64_t first_line;
    uint64_t last_line_present;
    uint64_t last_line;
    const struct thinkthen_cobol_AnswerExtensions *extensions;
} thinkthen_cobol_AnswerRecognition;
typedef struct thinkthen_cobol_AnswerRecognitionEdge {
    uint64_t record;
    uint64_t either_present;
    uint64_t either;
    double probability;
    thinkthen_cobol_AnswerText relation;
    thinkthen_cobol_AnswerRecognition source;
    thinkthen_cobol_AnswerRecognition target;
    const struct thinkthen_cobol_AnswerExtensions *extensions;
} thinkthen_cobol_AnswerRecognitionEdge;
typedef struct thinkthen_cobol_AnswerRelation {
    uint64_t record;
    thinkthen_cobol_AnswerText relation;
    thinkthen_cobol_AnswerText source;
    thinkthen_cobol_AnswerText source_kind;
    thinkthen_cobol_AnswerText target;
    thinkthen_cobol_AnswerText target_kind;
    double probability;
    uint64_t either_present;
    uint64_t either;
} thinkthen_cobol_AnswerRelation;
typedef struct thinkthen_cobol_AnswerError thinkthen_cobol_AnswerError;
typedef struct thinkthen_cobol_AnswerExtensions thinkthen_cobol_AnswerExtensions;
typedef struct thinkthen_cobol_AnswerExtension thinkthen_cobol_AnswerExtension;
typedef struct thinkthen_cobol_AnswerStopped thinkthen_cobol_AnswerStopped;
typedef struct thinkthen_cobol_AnswerU64Presence thinkthen_cobol_AnswerU64Presence;
typedef struct thinkthen_cobol_AnswerStopCause thinkthen_cobol_AnswerStopCause;
typedef struct thinkthen_cobol_AnswerSendDenialPresence thinkthen_cobol_AnswerSendDenialPresence;
typedef struct thinkthen_cobol_AnswerSendDenial thinkthen_cobol_AnswerSendDenial;
typedef struct thinkthen_cobol_AnswerSendDenialData thinkthen_cobol_AnswerSendDenialData;
typedef struct thinkthen_cobol_AnswerSendDenialBeforeRetry thinkthen_cobol_AnswerSendDenialBeforeRetry;
typedef struct thinkthen_cobol_AnswerSendDenialBeforeAdditionalSend thinkthen_cobol_AnswerSendDenialBeforeAdditionalSend;
typedef struct thinkthen_cobol_AnswerSendDenialBeforeFirstSend thinkthen_cobol_AnswerSendDenialBeforeFirstSend;
typedef struct thinkthen_cobol_AnswerFailureKind thinkthen_cobol_AnswerFailureKind;
typedef struct thinkthen_cobol_AnswerInputDenialPresence thinkthen_cobol_AnswerInputDenialPresence;
typedef struct thinkthen_cobol_AnswerInputDenial thinkthen_cobol_AnswerInputDenial;
typedef struct thinkthen_cobol_AnswerInputDenialData thinkthen_cobol_AnswerInputDenialData;
typedef struct thinkthen_cobol_AnswerInputDenialRetry thinkthen_cobol_AnswerInputDenialRetry;
typedef struct thinkthen_cobol_AnswerInputDenialAdditionalRequest thinkthen_cobol_AnswerInputDenialAdditionalRequest;
typedef struct thinkthen_cobol_AnswerInputDenialInitialRequest thinkthen_cobol_AnswerInputDenialInitialRequest;
typedef struct thinkthen_cobol_AnswerImmediateFacts thinkthen_cobol_AnswerImmediateFacts;
typedef struct thinkthen_cobol_AnswerPersistenceObservationPresence thinkthen_cobol_AnswerPersistenceObservationPresence;
typedef struct thinkthen_cobol_AnswerPersistenceObservation thinkthen_cobol_AnswerPersistenceObservation;
typedef struct thinkthen_cobol_AnswerUsagePersistence thinkthen_cobol_AnswerUsagePersistence;
typedef struct thinkthen_cobol_AnswerUtf8Presence thinkthen_cobol_AnswerUtf8Presence;
typedef struct thinkthen_cobol_AnswerU32Presence thinkthen_cobol_AnswerU32Presence;
typedef struct thinkthen_cobol_AnswerCallIdPresence thinkthen_cobol_AnswerCallIdPresence;
typedef struct thinkthen_cobol_AnswerCallId thinkthen_cobol_AnswerCallId;
typedef struct thinkthen_cobol_AnswerAttemptListPresence thinkthen_cobol_AnswerAttemptListPresence;
typedef struct thinkthen_cobol_AnswerAttemptList thinkthen_cobol_AnswerAttemptList;
typedef struct thinkthen_cobol_AnswerAttempt thinkthen_cobol_AnswerAttempt;
typedef struct thinkthen_cobol_AnswerSdkRequestId thinkthen_cobol_AnswerSdkRequestId;
typedef struct thinkthen_cobol_AnswerAttemptOutcome thinkthen_cobol_AnswerAttemptOutcome;
typedef struct thinkthen_cobol_AnswerFacts thinkthen_cobol_AnswerFacts;
struct thinkthen_cobol_AnswerExtensions {
    const thinkthen_cobol_AnswerExtension *data;
    size_t len;
};
struct thinkthen_cobol_AnswerExtension {
    thinkthen_cobol_AnswerText name;
    const tt_value *value;
};
struct thinkthen_cobol_AnswerU64Presence {
    uint32_t presence;
    uint64_t value;
};
struct thinkthen_cobol_AnswerStopCause {
    uint32_t kind;
};
struct thinkthen_cobol_AnswerSendDenialPresence {
    uint32_t presence;
    const thinkthen_cobol_AnswerSendDenial *value;
};
struct thinkthen_cobol_AnswerSendDenialData {
    const thinkthen_cobol_AnswerSendDenialBeforeFirstSend *before_first_send;
    const thinkthen_cobol_AnswerSendDenialBeforeAdditionalSend *before_additional_send;
    const thinkthen_cobol_AnswerSendDenialBeforeRetry *before_retry;
};
struct thinkthen_cobol_AnswerSendDenialBeforeRetry {
    thinkthen_cobol_AnswerText kind;
    uint64_t last_status;
    thinkthen_cobol_AnswerExtensions extensions;
};
struct thinkthen_cobol_AnswerSendDenialBeforeAdditionalSend {
    thinkthen_cobol_AnswerText kind;
    thinkthen_cobol_AnswerExtensions extensions;
};
struct thinkthen_cobol_AnswerSendDenialBeforeFirstSend {
    thinkthen_cobol_AnswerText kind;
    thinkthen_cobol_AnswerExtensions extensions;
};
struct thinkthen_cobol_AnswerFailureKind {
    uint32_t kind;
};
struct thinkthen_cobol_AnswerInputDenialPresence {
    uint32_t presence;
    const thinkthen_cobol_AnswerInputDenial *value;
};
struct thinkthen_cobol_AnswerInputDenialData {
    const thinkthen_cobol_AnswerInputDenialInitialRequest *initial_request;
    const thinkthen_cobol_AnswerInputDenialAdditionalRequest *additional_request;
    const thinkthen_cobol_AnswerInputDenialRetry *retry;
};
struct thinkthen_cobol_AnswerInputDenialRetry {
    thinkthen_cobol_AnswerText kind;
    uint64_t last_status;
    uint64_t limit;
    thinkthen_cobol_AnswerExtensions extensions;
};
struct thinkthen_cobol_AnswerInputDenialAdditionalRequest {
    thinkthen_cobol_AnswerText kind;
    uint64_t limit;
    thinkthen_cobol_AnswerExtensions extensions;
};
struct thinkthen_cobol_AnswerInputDenialInitialRequest {
    thinkthen_cobol_AnswerText kind;
    uint64_t limit;
    thinkthen_cobol_AnswerExtensions extensions;
};
struct thinkthen_cobol_AnswerPersistenceObservationPresence {
    uint32_t presence;
    const thinkthen_cobol_AnswerPersistenceObservation *value;
};
struct thinkthen_cobol_AnswerUsagePersistence {
    uint32_t kind;
};
struct thinkthen_cobol_AnswerUtf8Presence {
    uint32_t presence;
    thinkthen_cobol_AnswerText value;
};
struct thinkthen_cobol_AnswerU32Presence {
    uint32_t presence;
    uint32_t value;
};
struct thinkthen_cobol_AnswerCallIdPresence {
    uint32_t presence;
    const thinkthen_cobol_AnswerCallId *value;
};
struct thinkthen_cobol_AnswerCallId {
    thinkthen_cobol_AnswerText value;
};
struct thinkthen_cobol_AnswerAttemptList {
    const thinkthen_cobol_AnswerAttempt *data;
    size_t len;
};
struct thinkthen_cobol_AnswerAttempt {
    uint64_t ordinal;
    const thinkthen_cobol_AnswerAttemptOutcome *outcome;
    thinkthen_cobol_AnswerUtf8Presence request_id;
    thinkthen_cobol_AnswerText request_sha256;
    const thinkthen_cobol_AnswerSdkRequestId *sdk_request_id;
    thinkthen_cobol_AnswerU64Presence server_ms;
    thinkthen_cobol_AnswerU64Presence status;
    uint64_t wall_ms;
    thinkthen_cobol_AnswerExtensions extensions;
};
struct thinkthen_cobol_AnswerSdkRequestId {
    thinkthen_cobol_AnswerText value;
};
struct thinkthen_cobol_AnswerAttemptOutcome {
    uint32_t kind;
};
struct thinkthen_cobol_AnswerError {
    thinkthen_cobol_AnswerInputDenialPresence estimated_input_denial;
    const thinkthen_cobol_AnswerFailureKind *kind;
    thinkthen_cobol_AnswerText message;
    uint32_t retryable;
    thinkthen_cobol_AnswerSendDenialPresence send_budget_denial;
    const thinkthen_cobol_AnswerStopped *stopped;
    thinkthen_cobol_AnswerExtensions extensions;
};
struct thinkthen_cobol_AnswerStopped {
    thinkthen_cobol_AnswerU64Presence at;
    const thinkthen_cobol_AnswerStopCause *cause;
    uint32_t retryable;
    thinkthen_cobol_AnswerU64Presence status;
    thinkthen_cobol_AnswerExtensions extensions;
};
struct thinkthen_cobol_AnswerSendDenial {
    uint32_t kind;
    thinkthen_cobol_AnswerSendDenialData data;
};
struct thinkthen_cobol_AnswerInputDenial {
    uint32_t kind;
    thinkthen_cobol_AnswerInputDenialData data;
};
struct thinkthen_cobol_AnswerPersistenceObservation {
    thinkthen_cobol_AnswerUtf8Presence advice;
    thinkthen_cobol_AnswerText observed_at;
    const thinkthen_cobol_AnswerUsagePersistence *state;
    thinkthen_cobol_AnswerExtensions extensions;
};
struct thinkthen_cobol_AnswerAttemptListPresence {
    uint32_t presence;
    thinkthen_cobol_AnswerAttemptList value;
};
struct thinkthen_cobol_AnswerFacts {
    thinkthen_cobol_AnswerAttemptListPresence attempts;
    uint64_t cache_answers;
    const thinkthen_cobol_AnswerCallId *call_id;
    thinkthen_cobol_AnswerUtf8Presence estimated_cost_usd;
    thinkthen_cobol_AnswerU32Presence held_model_mismatch;
    thinkthen_cobol_AnswerU64Presence input_tokens;
    uint64_t largest_request_bytes;
    thinkthen_cobol_AnswerU64Presence largest_request_estimated_input_tokens;
    thinkthen_cobol_AnswerUtf8Presence model;
    thinkthen_cobol_AnswerU64Presence output_tokens;
    uint64_t records;
    uint64_t requests_sent;
    double seconds;
    thinkthen_cobol_AnswerText token_estimate_method;
    thinkthen_cobol_AnswerPersistenceObservationPresence usage_persistence;
    thinkthen_cobol_AnswerExtensions extensions;
};
struct thinkthen_cobol_AnswerImmediateFacts {
    thinkthen_cobol_AnswerAttemptListPresence attempts;
    uint64_t cache_answers;
    thinkthen_cobol_AnswerCallIdPresence call_id;
    thinkthen_cobol_AnswerUtf8Presence estimated_cost_usd;
    thinkthen_cobol_AnswerU32Presence held_model_mismatch;
    thinkthen_cobol_AnswerU64Presence input_tokens;
    uint64_t largest_request_bytes;
    thinkthen_cobol_AnswerU64Presence largest_request_estimated_input_tokens;
    thinkthen_cobol_AnswerUtf8Presence model;
    thinkthen_cobol_AnswerU64Presence output_tokens;
    uint64_t records;
    uint64_t requests_sent;
    double seconds;
    thinkthen_cobol_AnswerText token_estimate_method;
    thinkthen_cobol_AnswerPersistenceObservationPresence usage_persistence;
    thinkthen_cobol_AnswerExtensions extensions;
};
typedef struct thinkthen_cobol_AnswerStatus {
    int64_t code;
    uint64_t retryable;
    uint64_t failure_kind;
    thinkthen_cobol_AnswerText message;
    uint64_t facts_present;
    thinkthen_cobol_AnswerFacts facts;
    const thinkthen_cobol_AnswerImmediateFacts *immediate_facts;
    const thinkthen_cobol_AnswerError *details;
    tt_answer_owner *owner;
} thinkthen_cobol_AnswerStatus;
typedef struct thinkthen_cobol_AnswerTable {
    tt_answer_owner *owner;
    const void *rows;
    uint64_t count;
    const thinkthen_cobol_AnswerRecognitionEdge *edges;
    uint64_t edge_count;
} thinkthen_cobol_AnswerTable;
void TT_ANSWER_FREE(tt_answer_owner *);
typedef struct thinkthen_cobol_AnswerTokenBand {
    uint64_t lower;
    uint64_t upper;
} thinkthen_cobol_AnswerTokenBand;
typedef struct thinkthen_cobol_AnswerPlan {
    uint64_t estimated_bytes;
    thinkthen_cobol_AnswerTokenBand estimated_input_tokens;
    uint64_t first_body_utf8_presence;
    thinkthen_cobol_AnswerText first_body_utf8;
    uint64_t largest_request_bytes;
    uint64_t largest_request_estimated_input_tokens;
    uint64_t records;
    uint64_t requests;
    thinkthen_cobol_AnswerText token_estimate_method;
    uint64_t upper_bound;
} thinkthen_cobol_AnswerPlan;
int TT_PREVIEW(const void *engine,const char *function,const uint64_t *length,const thinkthen_cobol_RequestQuestion *question,
const thinkthen_cobol_RequestInput *input,const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerPlan *out,
thinkthen_cobol_AnswerStatus *status_out);
typedef struct thinkthen_cobol_AnswerDecideResult {
    tt_answer_owner *owner;
    const thinkthen_cobol_AnswerDecision *rows;
    uint64_t count;
    thinkthen_cobol_AnswerDecision first;
} thinkthen_cobol_AnswerDecideResult;
int TT_DECIDE(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerDecideResult *out,thinkthen_cobol_AnswerStatus *status_out);
typedef struct thinkthen_cobol_AnswerChooseResult {
    tt_answer_owner *owner;
    const thinkthen_cobol_AnswerChoice *rows;
    uint64_t count;
    thinkthen_cobol_AnswerChoice first;
} thinkthen_cobol_AnswerChooseResult;
int TT_CHOOSE(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerChooseResult *out,thinkthen_cobol_AnswerStatus *status_out);
typedef struct thinkthen_cobol_AnswerTagResult {
    tt_answer_owner *owner;
    const thinkthen_cobol_AnswerTable *rows;
    uint64_t count;
    thinkthen_cobol_AnswerTable first;
} thinkthen_cobol_AnswerTagResult;
int TT_TAG(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerTagResult *out,thinkthen_cobol_AnswerStatus *status_out);
typedef struct thinkthen_cobol_AnswerScoreResult {
    tt_answer_owner *owner;
    const thinkthen_cobol_AnswerScore *rows;
    uint64_t count;
    thinkthen_cobol_AnswerScore first;
} thinkthen_cobol_AnswerScoreResult;
int TT_SCORE(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerScoreResult *out,thinkthen_cobol_AnswerStatus *status_out);
typedef struct thinkthen_cobol_AnswerFilterResult {
    tt_answer_owner *owner;
    const thinkthen_cobol_AnswerFilterOriginal *rows;
    uint64_t count;
    thinkthen_cobol_AnswerFilterOriginal first;
} thinkthen_cobol_AnswerFilterResult;
int TT_FILTER(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerFilterResult *out,thinkthen_cobol_AnswerStatus *status_out);
typedef struct thinkthen_cobol_AnswerRankResult {
    tt_answer_owner *owner;
    const thinkthen_cobol_AnswerRankIndex *rows;
    uint64_t count;
    thinkthen_cobol_AnswerRankIndex first;
} thinkthen_cobol_AnswerRankResult;
int TT_RANK(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerRankResult *out,thinkthen_cobol_AnswerStatus *status_out);
typedef struct thinkthen_cobol_AnswerFindResult {
    tt_answer_owner *owner;
    const thinkthen_cobol_AnswerFindSelection *rows;
    uint64_t count;
    thinkthen_cobol_AnswerFindSelection first;
} thinkthen_cobol_AnswerFindResult;
int TT_FIND(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerFindResult *out,thinkthen_cobol_AnswerStatus *status_out);
typedef struct thinkthen_cobol_AnswerAnnotateResult {
    tt_answer_owner *owner;
    const thinkthen_cobol_AnswerAnnotationField *rows;
    uint64_t count;
    thinkthen_cobol_AnswerAnnotationField first;
} thinkthen_cobol_AnswerAnnotateResult;
int TT_ANNOTATE(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerAnnotateResult *out,thinkthen_cobol_AnswerStatus *status_out);
typedef struct thinkthen_cobol_AnswerRecognizeResult {
    tt_answer_owner *owner;
    const thinkthen_cobol_AnswerRecognition *rows;
    uint64_t count;
    thinkthen_cobol_AnswerRecognition first;
    const thinkthen_cobol_AnswerRecognitionEdge *edges;
    uint64_t edge_count;
} thinkthen_cobol_AnswerRecognizeResult;
int TT_RECOGNIZE(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerRecognizeResult *out,thinkthen_cobol_AnswerStatus *status_out);
typedef struct thinkthen_cobol_AnswerRelateResult {
    tt_answer_owner *owner;
    const thinkthen_cobol_AnswerRelation *rows;
    uint64_t count;
    thinkthen_cobol_AnswerRelation first;
} thinkthen_cobol_AnswerRelateResult;
int TT_RELATE(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerRelateResult *out,thinkthen_cobol_AnswerStatus *status_out);
void TT_VALUE_FREE(tt_value *);
int TT_VALUE_KIND(const tt_value *,uint32_t *);
int TT_VALUE_LENGTH(const tt_value *,uint64_t *);
int TT_VALUE_BOOLEAN(const tt_value *,uint32_t *);
int TT_VALUE_TEXT(const tt_value *,char *,uint64_t *);
int TT_VALUE_NAME(const tt_value *,uint64_t,char *,uint64_t *);
int TT_VALUE_ELEMENT(const tt_value *,uint64_t,tt_value **);
int TT_VALUE_FIELD(const tt_value *,const char *,uint64_t,tt_value **);
int TT_VALUE_HAS_FIELD(const tt_value *,const char *,uint64_t,uint32_t *);
int TT_VALUE_NULL(tt_value **);
int TT_VALUE_BOOL(uint32_t,tt_value **);
int TT_VALUE_STRING(const char *,uint64_t,tt_value **);
int TT_VALUE_NUMBER(const char *,uint64_t,tt_value **);
int TT_VALUE_ARRAY(const tt_value *const *,uint64_t,tt_value **);
int TT_VALUE_OBJECT(const thinkthen_cobol_text *,const tt_value *const *,uint64_t,tt_value **);
int TT_QUESTION_TEXT(thinkthen_cobol_RequestQuestion *selector,thinkthen_cobol_RequestQuestionText *arm,
thinkthen_cobol_RequestQuestionTextText *scalar,const char *data,uint64_t length);
int TT_QUESTION_FILE(thinkthen_cobol_RequestQuestion *selector,thinkthen_cobol_RequestQuestionFile *arm,
thinkthen_cobol_RequestQuestionFilePath *scalar,const char *data,uint64_t length);
int TT_QUESTION_NAME(thinkthen_cobol_RequestQuestion *selector,thinkthen_cobol_RequestQuestionName *arm,
thinkthen_cobol_RequestQuestionNameName *scalar,const char *data,uint64_t length);
int TT_QUESTION_REFERENCE(thinkthen_cobol_RequestQuestion *selector,thinkthen_cobol_RequestQuestionReference *arm,
thinkthen_cobol_RequestQuestionReferenceReference *scalar,const char *data,uint64_t length);
int TT_QUESTION_RELATE(thinkthen_cobol_RequestQuestion *selector,thinkthen_cobol_RequestQuestionDefinition *arm,
thinkthen_cobol_RequestDefinition *definition,const thinkthen_cobol_DefinitionRelate *value);
int TT_QUESTION_RECOGNIZE(thinkthen_cobol_RequestQuestion *selector,thinkthen_cobol_RequestQuestionDefinition *arm,
thinkthen_cobol_RequestDefinition *definition,const thinkthen_cobol_DefinitionRecognizeDefinition *value);
int TT_TEXT_ITEM(thinkthen_cobol_RequestItem *item,thinkthen_cobol_RequestOriginal *original,
thinkthen_cobol_RequestOriginalText *arm,thinkthen_cobol_RequestOriginalTextText *scalar,const char *data,
uint64_t length);
int TT_JSON_ITEM(thinkthen_cobol_RequestItem *item,thinkthen_cobol_RequestOriginal *original,
thinkthen_cobol_RequestOriginalJson *arm,thinkthen_cobol_RequestOriginalJsonValue *scalar,const tt_value *value);
int TT_FILE_INPUT(thinkthen_cobol_RequestInput *selector,thinkthen_cobol_RequestInputSource *arm,
thinkthen_cobol_RequestSource *source,thinkthen_cobol_RequestSourcePaths *paths,thinkthen_cobol_RequestSourcePathsItem *path,
const void **slots,const char *data,uint64_t length);
#endif
