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
#define THINKTHEN_COBOL_QUESTIONBATCHARM1_KIND 1
#define THINKTHEN_COBOL_QUESTIONBATCHARM2_KIND 2
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
#define THINKTHEN_COBOL_SETTINGSINPUTTOKENBUDGETARM1_KIND 1
#define THINKTHEN_COBOL_SETTINGSINPUTTOKENBUDGETARM2_KIND 2
#define THINKTHEN_COBOL_SETTINGSMAXREQUESTSARM1_KIND 1
#define THINKTHEN_COBOL_SETTINGSMAXREQUESTSARM2_KIND 2
#define THINKTHEN_COBOL_SETTINGSMAXREQUESTSTOTALARM1_KIND 1
#define THINKTHEN_COBOL_SETTINGSMAXREQUESTSTOTALARM2_KIND 2
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
typedef struct thinkthen_cobol_QuestionBatch {
    uint64_t kind;
    const void * value;
} thinkthen_cobol_QuestionBatch;
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
typedef struct thinkthen_cobol_QuestionBatchArm1 {
    uint64_t reserved;
} thinkthen_cobol_QuestionBatchArm1;
typedef struct thinkthen_cobol_QuestionBatchArm2 {
    int64_t value;
} thinkthen_cobol_QuestionBatchArm2;
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
typedef struct thinkthen_cobol_ScoreName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_ScoreName;
typedef struct thinkthen_cobol_ScoreWordingVersion {
    int64_t value;
} thinkthen_cobol_ScoreWordingVersion;
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
typedef struct thinkthen_cobol_DefinitionChooseName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_DefinitionChooseName;
typedef struct thinkthen_cobol_DefinitionChooseWordingVersion {
    int64_t value;
} thinkthen_cobol_DefinitionChooseWordingVersion;
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
typedef struct thinkthen_cobol_DefinitionScoreName {
    const char * data;
    uint64_t len;
} thinkthen_cobol_DefinitionScoreName;
typedef struct thinkthen_cobol_DefinitionScoreWordingVersion {
    int64_t value;
} thinkthen_cobol_DefinitionScoreWordingVersion;
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
typedef struct thinkthen_cobol_CriterionArm2Entry {
    const struct thinkthen_complete_json_v1 * value;
} thinkthen_cobol_CriterionArm2Entry;
typedef struct thinkthen_cobol_CriterionArm3Item {
    const struct thinkthen_complete_json_v1 * value;
} thinkthen_cobol_CriterionArm3Item;
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
    const tt_value *value;
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
    double strength;
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
typedef struct thinkthen_cobol_AnswerNumber {
    uint64_t present;
    double value;
} thinkthen_cobol_AnswerNumber;
typedef struct thinkthen_cobol_AnswerProbabilityList {
    const thinkthen_cobol_AnswerNumber *data;
    uint64_t count;
} thinkthen_cobol_AnswerProbabilityList;
typedef struct thinkthen_cobol_AnswerProbability {
    uint64_t scalar;
    thinkthen_cobol_AnswerNumber first;
    const thinkthen_cobol_AnswerNumber *data;
    uint64_t count;
    const thinkthen_cobol_AnswerProbabilityList *lists;
    uint64_t list_count;
} thinkthen_cobol_AnswerProbability;
typedef struct thinkthen_cobol_ReceiptRelation thinkthen_cobol_ReceiptRelation;
typedef struct thinkthen_cobol_ReceiptExtensions thinkthen_cobol_ReceiptExtensions;
typedef struct thinkthen_cobol_ReceiptExtension thinkthen_cobol_ReceiptExtension;
typedef struct thinkthen_cobol_ReceiptRelatedEntityEdgeList thinkthen_cobol_ReceiptRelatedEntityEdgeList;
typedef struct thinkthen_cobol_ReceiptRelatedEntityEdge thinkthen_cobol_ReceiptRelatedEntityEdge;
typedef struct thinkthen_cobol_ReceiptRelationEndpoint thinkthen_cobol_ReceiptRelationEndpoint;
typedef struct thinkthen_cobol_ReceiptRelationEndpointData thinkthen_cobol_ReceiptRelationEndpointData;
typedef struct thinkthen_cobol_ReceiptRelationEndpointSource thinkthen_cobol_ReceiptRelationEndpointSource;
typedef struct thinkthen_cobol_ReceiptJsonPresence thinkthen_cobol_ReceiptJsonPresence;
typedef struct thinkthen_cobol_ReceiptU64Presence thinkthen_cobol_ReceiptU64Presence;
typedef struct thinkthen_cobol_ReceiptUtf8Presence thinkthen_cobol_ReceiptUtf8Presence;
typedef struct thinkthen_cobol_ReceiptRelationEndpointRelatedEntity thinkthen_cobol_ReceiptRelationEndpointRelatedEntity;
typedef struct thinkthen_cobol_ReceiptU32Presence thinkthen_cobol_ReceiptU32Presence;
typedef struct thinkthen_cobol_ReceiptVersion thinkthen_cobol_ReceiptVersion;
typedef struct thinkthen_cobol_ReceiptRelateQuestionDocument thinkthen_cobol_ReceiptRelateQuestionDocument;
typedef struct thinkthen_cobol_ReceiptWordingVersionPresence thinkthen_cobol_ReceiptWordingVersionPresence;
typedef struct thinkthen_cobol_ReceiptWordingVersion thinkthen_cobol_ReceiptWordingVersion;
typedef struct thinkthen_cobol_ReceiptThreshold thinkthen_cobol_ReceiptThreshold;
typedef struct thinkthen_cobol_ReceiptThresholdData thinkthen_cobol_ReceiptThresholdData;
typedef struct thinkthen_cobol_ReceiptRelationRuleList thinkthen_cobol_ReceiptRelationRuleList;
typedef struct thinkthen_cobol_ReceiptRelationRule thinkthen_cobol_ReceiptRelationRule;
typedef struct thinkthen_cobol_ReceiptStringListPresence thinkthen_cobol_ReceiptStringListPresence;
typedef struct thinkthen_cobol_ReceiptStringList thinkthen_cobol_ReceiptStringList;
typedef struct thinkthen_cobol_ReceiptQuestionNamePresence thinkthen_cobol_ReceiptQuestionNamePresence;
typedef struct thinkthen_cobol_ReceiptQuestionName thinkthen_cobol_ReceiptQuestionName;
typedef struct thinkthen_cobol_ReceiptLabelListPresence thinkthen_cobol_ReceiptLabelListPresence;
typedef struct thinkthen_cobol_ReceiptLabelList thinkthen_cobol_ReceiptLabelList;
typedef struct thinkthen_cobol_ReceiptLabel thinkthen_cobol_ReceiptLabel;
typedef struct thinkthen_cobol_ReceiptInputDeclarationPresence thinkthen_cobol_ReceiptInputDeclarationPresence;
typedef struct thinkthen_cobol_ReceiptInputDeclaration thinkthen_cobol_ReceiptInputDeclaration;
typedef struct thinkthen_cobol_ReceiptInputDeclarationData thinkthen_cobol_ReceiptInputDeclarationData;
typedef struct thinkthen_cobol_ReceiptInputDeclarationObjectRoot thinkthen_cobol_ReceiptInputDeclarationObjectRoot;
typedef struct thinkthen_cobol_ReceiptObjectType thinkthen_cobol_ReceiptObjectType;
typedef struct thinkthen_cobol_ReceiptInputPropertyTypeMap thinkthen_cobol_ReceiptInputPropertyTypeMap;
typedef struct thinkthen_cobol_ReceiptInputPropertyTypeMapEntry thinkthen_cobol_ReceiptInputPropertyTypeMapEntry;
typedef struct thinkthen_cobol_ReceiptInputPropertyType thinkthen_cobol_ReceiptInputPropertyType;
typedef struct thinkthen_cobol_ReceiptInputPropertyTypeData thinkthen_cobol_ReceiptInputPropertyTypeData;
typedef struct thinkthen_cobol_ReceiptInputPropertyTypeArray thinkthen_cobol_ReceiptInputPropertyTypeArray;
typedef struct thinkthen_cobol_ReceiptStringRoot thinkthen_cobol_ReceiptStringRoot;
typedef struct thinkthen_cobol_ReceiptStringType thinkthen_cobol_ReceiptStringType;
typedef struct thinkthen_cobol_ReceiptInputPropertyTypeBoolean thinkthen_cobol_ReceiptInputPropertyTypeBoolean;
typedef struct thinkthen_cobol_ReceiptInputPropertyTypeNumber thinkthen_cobol_ReceiptInputPropertyTypeNumber;
typedef struct thinkthen_cobol_ReceiptInputPropertyTypeString thinkthen_cobol_ReceiptInputPropertyTypeString;
typedef struct thinkthen_cobol_ReceiptInputDeclarationStringRoot thinkthen_cobol_ReceiptInputDeclarationStringRoot;
typedef struct thinkthen_cobol_ReceiptRelateFieldsPresence thinkthen_cobol_ReceiptRelateFieldsPresence;
typedef struct thinkthen_cobol_ReceiptRelateFields thinkthen_cobol_ReceiptRelateFields;
typedef struct thinkthen_cobol_ReceiptBatchPresence thinkthen_cobol_ReceiptBatchPresence;
typedef struct thinkthen_cobol_ReceiptBatch thinkthen_cobol_ReceiptBatch;
typedef struct thinkthen_cobol_ReceiptBatchData thinkthen_cobol_ReceiptBatchData;
typedef struct thinkthen_cobol_ReceiptBatchString thinkthen_cobol_ReceiptBatchString;
typedef struct thinkthen_cobol_ReceiptPositionPresence thinkthen_cobol_ReceiptPositionPresence;
typedef struct thinkthen_cobol_ReceiptPosition thinkthen_cobol_ReceiptPosition;
typedef struct thinkthen_cobol_ReceiptMeta thinkthen_cobol_ReceiptMeta;
typedef struct thinkthen_cobol_ReceiptUsagePresence thinkthen_cobol_ReceiptUsagePresence;
typedef struct thinkthen_cobol_ReceiptUsage thinkthen_cobol_ReceiptUsage;
typedef struct thinkthen_cobol_ReceiptQuestionSourceList thinkthen_cobol_ReceiptQuestionSourceList;
typedef struct thinkthen_cobol_ReceiptQuestionSource thinkthen_cobol_ReceiptQuestionSource;
typedef struct thinkthen_cobol_ReceiptOrigin thinkthen_cobol_ReceiptOrigin;
typedef struct thinkthen_cobol_ReceiptProfileWarningPresence thinkthen_cobol_ReceiptProfileWarningPresence;
typedef struct thinkthen_cobol_ReceiptProfileWarning thinkthen_cobol_ReceiptProfileWarning;
typedef struct thinkthen_cobol_ReceiptOriginPresence thinkthen_cobol_ReceiptOriginPresence;
typedef struct thinkthen_cobol_ReceiptObservationList thinkthen_cobol_ReceiptObservationList;
typedef struct thinkthen_cobol_ReceiptObservation thinkthen_cobol_ReceiptObservation;
typedef struct thinkthen_cobol_ReceiptObservationData thinkthen_cobol_ReceiptObservationData;
typedef struct thinkthen_cobol_ReceiptObservationFailureId thinkthen_cobol_ReceiptObservationFailureId;
typedef struct thinkthen_cobol_ReceiptFailureId thinkthen_cobol_ReceiptFailureId;
typedef struct thinkthen_cobol_ReceiptObservationObservationId thinkthen_cobol_ReceiptObservationObservationId;
typedef struct thinkthen_cobol_ReceiptObservationId thinkthen_cobol_ReceiptObservationId;
typedef struct thinkthen_cobol_ReceiptBatchWarningPresence thinkthen_cobol_ReceiptBatchWarningPresence;
typedef struct thinkthen_cobol_ReceiptBatchWarning thinkthen_cobol_ReceiptBatchWarning;
typedef struct thinkthen_cobol_ReceiptBatchSetting thinkthen_cobol_ReceiptBatchSetting;
typedef struct thinkthen_cobol_ReceiptBatchSettingData thinkthen_cobol_ReceiptBatchSettingData;
typedef struct thinkthen_cobol_ReceiptBatchSettingPresence thinkthen_cobol_ReceiptBatchSettingPresence;
typedef struct thinkthen_cobol_ReceiptAttemptListPresence thinkthen_cobol_ReceiptAttemptListPresence;
typedef struct thinkthen_cobol_ReceiptAttemptList thinkthen_cobol_ReceiptAttemptList;
typedef struct thinkthen_cobol_ReceiptAttempt thinkthen_cobol_ReceiptAttempt;
typedef struct thinkthen_cobol_ReceiptSdkRequestId thinkthen_cobol_ReceiptSdkRequestId;
typedef struct thinkthen_cobol_ReceiptAttemptOutcome thinkthen_cobol_ReceiptAttemptOutcome;
typedef struct thinkthen_cobol_ReceiptSessionInputSourceListPresence thinkthen_cobol_ReceiptSessionInputSourceListPresence;
typedef struct thinkthen_cobol_ReceiptSessionInputSourceList thinkthen_cobol_ReceiptSessionInputSourceList;
typedef struct thinkthen_cobol_ReceiptSessionInputSource thinkthen_cobol_ReceiptSessionInputSource;
typedef struct thinkthen_cobol_ReceiptPhysicalSource thinkthen_cobol_ReceiptPhysicalSource;
typedef struct thinkthen_cobol_ReceiptAnswerId thinkthen_cobol_ReceiptAnswerId;
typedef struct thinkthen_cobol_ReceiptAnswers thinkthen_cobol_ReceiptAnswers;
typedef struct thinkthen_cobol_ReceiptRelationMemberList thinkthen_cobol_ReceiptRelationMemberList;
typedef struct thinkthen_cobol_ReceiptRelationMember thinkthen_cobol_ReceiptRelationMember;
typedef struct thinkthen_cobol_ReceiptRelationMemberData thinkthen_cobol_ReceiptRelationMemberData;
typedef struct thinkthen_cobol_ReceiptRelationMemberFailureId thinkthen_cobol_ReceiptRelationMemberFailureId;
typedef struct thinkthen_cobol_ReceiptFailure thinkthen_cobol_ReceiptFailure;
typedef struct thinkthen_cobol_ReceiptFailureFieldKind thinkthen_cobol_ReceiptFailureFieldKind;
typedef struct thinkthen_cobol_ReceiptFailureCause thinkthen_cobol_ReceiptFailureCause;
typedef struct thinkthen_cobol_ReceiptRelatedEntityPresence thinkthen_cobol_ReceiptRelatedEntityPresence;
typedef struct thinkthen_cobol_ReceiptRelatedEntity thinkthen_cobol_ReceiptRelatedEntity;
typedef struct thinkthen_cobol_ReceiptQuestionDocument thinkthen_cobol_ReceiptQuestionDocument;
typedef struct thinkthen_cobol_ReceiptQuestionDocumentData thinkthen_cobol_ReceiptQuestionDocumentData;
typedef struct thinkthen_cobol_ReceiptQuestionDocumentScore thinkthen_cobol_ReceiptQuestionDocumentScore;
typedef struct thinkthen_cobol_ReceiptQuestionDocumentTag thinkthen_cobol_ReceiptQuestionDocumentTag;
typedef struct thinkthen_cobol_ReceiptQuestionDocumentChoose thinkthen_cobol_ReceiptQuestionDocumentChoose;
typedef struct thinkthen_cobol_ReceiptQuestionDocumentDecide thinkthen_cobol_ReceiptQuestionDocumentDecide;
typedef struct thinkthen_cobol_ReceiptRelationMethod thinkthen_cobol_ReceiptRelationMethod;
typedef struct thinkthen_cobol_ReceiptRelationDirection thinkthen_cobol_ReceiptRelationDirection;
typedef struct thinkthen_cobol_ReceiptRelationMemberAnswerId thinkthen_cobol_ReceiptRelationMemberAnswerId;
typedef struct thinkthen_cobol_ReceiptAnswer thinkthen_cobol_ReceiptAnswer;
typedef struct thinkthen_cobol_ReceiptAnswerData thinkthen_cobol_ReceiptAnswerData;
typedef struct thinkthen_cobol_ReceiptAnswerScore thinkthen_cobol_ReceiptAnswerScore;
typedef struct thinkthen_cobol_ReceiptNumberMap thinkthen_cobol_ReceiptNumberMap;
typedef struct thinkthen_cobol_ReceiptNumberMapEntry thinkthen_cobol_ReceiptNumberMapEntry;
typedef struct thinkthen_cobol_ReceiptF64Presence thinkthen_cobol_ReceiptF64Presence;
typedef struct thinkthen_cobol_ReceiptAnswerTag thinkthen_cobol_ReceiptAnswerTag;
typedef struct thinkthen_cobol_ReceiptAnswerChoice thinkthen_cobol_ReceiptAnswerChoice;
typedef struct thinkthen_cobol_ReceiptAnswerYesNo thinkthen_cobol_ReceiptAnswerYesNo;
typedef struct thinkthen_cobol_ReceiptRecognitionList thinkthen_cobol_ReceiptRecognitionList;
typedef struct thinkthen_cobol_ReceiptRecognition thinkthen_cobol_ReceiptRecognition;
typedef struct thinkthen_cobol_ReceiptRecognize thinkthen_cobol_ReceiptRecognize;
typedef struct thinkthen_cobol_ReceiptRecognizeData thinkthen_cobol_ReceiptRecognizeData;
typedef struct thinkthen_cobol_ReceiptRecognizeModeProposals thinkthen_cobol_ReceiptRecognizeModeProposals;
typedef struct thinkthen_cobol_ReceiptBoundaryProposalList thinkthen_cobol_ReceiptBoundaryProposalList;
typedef struct thinkthen_cobol_ReceiptBoundaryProposal thinkthen_cobol_ReceiptBoundaryProposal;
typedef struct thinkthen_cobol_ReceiptBoundaryMode thinkthen_cobol_ReceiptBoundaryMode;
typedef struct thinkthen_cobol_ReceiptRecognizeEntities thinkthen_cobol_ReceiptRecognizeEntities;
typedef struct thinkthen_cobol_ReceiptEntityEdgeListPresence thinkthen_cobol_ReceiptEntityEdgeListPresence;
typedef struct thinkthen_cobol_ReceiptEntityEdgeList thinkthen_cobol_ReceiptEntityEdgeList;
typedef struct thinkthen_cobol_ReceiptEntityEdge thinkthen_cobol_ReceiptEntityEdge;
typedef struct thinkthen_cobol_ReceiptEntity thinkthen_cobol_ReceiptEntity;
typedef struct thinkthen_cobol_ReceiptEntityList thinkthen_cobol_ReceiptEntityList;
typedef struct thinkthen_cobol_ReceiptPhysicalSourcePresence thinkthen_cobol_ReceiptPhysicalSourcePresence;
typedef struct thinkthen_cobol_ReceiptRecognizeQuestionDocument thinkthen_cobol_ReceiptRecognizeQuestionDocument;
typedef struct thinkthen_cobol_ReceiptVerb thinkthen_cobol_ReceiptVerb;
typedef struct thinkthen_cobol_ReceiptRecognitionStageContextPresence thinkthen_cobol_ReceiptRecognitionStageContextPresence;
typedef struct thinkthen_cobol_ReceiptRecognitionStageContext thinkthen_cobol_ReceiptRecognitionStageContext;
typedef struct thinkthen_cobol_ReceiptRelationRuleListPresence thinkthen_cobol_ReceiptRelationRuleListPresence;
typedef struct thinkthen_cobol_ReceiptThresholdPresence thinkthen_cobol_ReceiptThresholdPresence;
typedef struct thinkthen_cobol_ReceiptRecognitionModePresence thinkthen_cobol_ReceiptRecognitionModePresence;
typedef struct thinkthen_cobol_ReceiptRecognitionMode thinkthen_cobol_ReceiptRecognitionMode;
typedef struct thinkthen_cobol_ReceiptJsonMap thinkthen_cobol_ReceiptJsonMap;
typedef struct thinkthen_cobol_ReceiptJsonMapEntry thinkthen_cobol_ReceiptJsonMapEntry;
typedef struct thinkthen_cobol_ReceiptRecognitionOdds thinkthen_cobol_ReceiptRecognitionOdds;
typedef struct thinkthen_cobol_ReceiptRecognitionOddsData thinkthen_cobol_ReceiptRecognitionOddsData;
typedef struct thinkthen_cobol_ReceiptRecognitionOddsBoundaryOdds thinkthen_cobol_ReceiptRecognitionOddsBoundaryOdds;
typedef struct thinkthen_cobol_ReceiptPieceOddsList thinkthen_cobol_ReceiptPieceOddsList;
typedef struct thinkthen_cobol_ReceiptPieceOdds thinkthen_cobol_ReceiptPieceOdds;
typedef struct thinkthen_cobol_ReceiptRecognitionOddsRecognizeAnswer thinkthen_cobol_ReceiptRecognitionOddsRecognizeAnswer;
typedef struct thinkthen_cobol_ReceiptRecognitionProposalList thinkthen_cobol_ReceiptRecognitionProposalList;
typedef struct thinkthen_cobol_ReceiptRecognitionProposal thinkthen_cobol_ReceiptRecognitionProposal;
typedef struct thinkthen_cobol_ReceiptPlacePresence thinkthen_cobol_ReceiptPlacePresence;
typedef struct thinkthen_cobol_ReceiptPlace thinkthen_cobol_ReceiptPlace;
typedef struct thinkthen_cobol_ReceiptPairOddsList thinkthen_cobol_ReceiptPairOddsList;
typedef struct thinkthen_cobol_ReceiptPairOdds thinkthen_cobol_ReceiptPairOdds;
typedef struct thinkthen_cobol_ReceiptNameOddsList thinkthen_cobol_ReceiptNameOddsList;
typedef struct thinkthen_cobol_ReceiptNameOdds thinkthen_cobol_ReceiptNameOdds;
typedef struct thinkthen_cobol_ReceiptNumberMapPresence thinkthen_cobol_ReceiptNumberMapPresence;
typedef struct thinkthen_cobol_ReceiptAnnotationList thinkthen_cobol_ReceiptAnnotationList;
typedef struct thinkthen_cobol_ReceiptAnnotation thinkthen_cobol_ReceiptAnnotation;
typedef struct thinkthen_cobol_ReceiptAnnotatedRow thinkthen_cobol_ReceiptAnnotatedRow;
typedef struct thinkthen_cobol_ReceiptAnnotatedFieldMap thinkthen_cobol_ReceiptAnnotatedFieldMap;
typedef struct thinkthen_cobol_ReceiptAnnotatedFieldMapEntry thinkthen_cobol_ReceiptAnnotatedFieldMapEntry;
typedef struct thinkthen_cobol_ReceiptAnnotatedField thinkthen_cobol_ReceiptAnnotatedField;
typedef struct thinkthen_cobol_ReceiptAnnotatedFieldData thinkthen_cobol_ReceiptAnnotatedFieldData;
typedef struct thinkthen_cobol_ReceiptFailed thinkthen_cobol_ReceiptFailed;
typedef struct thinkthen_cobol_ReceiptAnnotationMemberMap thinkthen_cobol_ReceiptAnnotationMemberMap;
typedef struct thinkthen_cobol_ReceiptAnnotationMemberMapEntry thinkthen_cobol_ReceiptAnnotationMemberMapEntry;
typedef struct thinkthen_cobol_ReceiptAnnotationMember thinkthen_cobol_ReceiptAnnotationMember;
typedef struct thinkthen_cobol_ReceiptAnnotationMemberData thinkthen_cobol_ReceiptAnnotationMemberData;
typedef struct thinkthen_cobol_ReceiptAnnotationMemberFailureId thinkthen_cobol_ReceiptAnnotationMemberFailureId;
typedef struct thinkthen_cobol_ReceiptAnnotationMemberAnswerId thinkthen_cobol_ReceiptAnnotationMemberAnswerId;
typedef struct thinkthen_cobol_ReceiptValuePresence thinkthen_cobol_ReceiptValuePresence;
typedef struct thinkthen_cobol_ReceiptValue thinkthen_cobol_ReceiptValue;
typedef struct thinkthen_cobol_ReceiptValueData thinkthen_cobol_ReceiptValueData;
typedef struct thinkthen_cobol_ReceiptFind thinkthen_cobol_ReceiptFind;
typedef struct thinkthen_cobol_ReceiptFindQuestionDocument thinkthen_cobol_ReceiptFindQuestionDocument;
typedef struct thinkthen_cobol_ReceiptFindCandidateListPresence thinkthen_cobol_ReceiptFindCandidateListPresence;
typedef struct thinkthen_cobol_ReceiptFindCandidateList thinkthen_cobol_ReceiptFindCandidateList;
typedef struct thinkthen_cobol_ReceiptFindCandidate thinkthen_cobol_ReceiptFindCandidate;
typedef struct thinkthen_cobol_ReceiptFindAnswer thinkthen_cobol_ReceiptFindAnswer;
typedef struct thinkthen_cobol_ReceiptAtomicNonZeroUsizeList thinkthen_cobol_ReceiptAtomicNonZeroUsizeList;
typedef struct thinkthen_cobol_ReceiptAtomicNonZeroUsize thinkthen_cobol_ReceiptAtomicNonZeroUsize;
typedef struct thinkthen_cobol_ReceiptRankMemberListPresence thinkthen_cobol_ReceiptRankMemberListPresence;
typedef struct thinkthen_cobol_ReceiptRankMemberList thinkthen_cobol_ReceiptRankMemberList;
typedef struct thinkthen_cobol_ReceiptRankMember thinkthen_cobol_ReceiptRankMember;
typedef struct thinkthen_cobol_ReceiptRankMemberResult thinkthen_cobol_ReceiptRankMemberResult;
typedef struct thinkthen_cobol_ReceiptImageListPresence thinkthen_cobol_ReceiptImageListPresence;
typedef struct thinkthen_cobol_ReceiptImageList thinkthen_cobol_ReceiptImageList;
typedef struct thinkthen_cobol_ReceiptImage thinkthen_cobol_ReceiptImage;
typedef struct thinkthen_cobol_ReceiptImageMedia thinkthen_cobol_ReceiptImageMedia;
typedef struct thinkthen_cobol_ReceiptAtomicBooleanList thinkthen_cobol_ReceiptAtomicBooleanList;
typedef struct thinkthen_cobol_ReceiptAtomicBoolean thinkthen_cobol_ReceiptAtomicBoolean;
typedef struct thinkthen_cobol_ReceiptAtomicDoubleList thinkthen_cobol_ReceiptAtomicDoubleList;
typedef struct thinkthen_cobol_ReceiptAtomicDouble thinkthen_cobol_ReceiptAtomicDouble;
typedef struct thinkthen_cobol_ReceiptAtomicArrayOfStringList thinkthen_cobol_ReceiptAtomicArrayOfStringList;
typedef struct thinkthen_cobol_ReceiptAtomicArrayOfString thinkthen_cobol_ReceiptAtomicArrayOfString;
typedef struct thinkthen_cobol_ReceiptAtomicNullableStringList thinkthen_cobol_ReceiptAtomicNullableStringList;
typedef struct thinkthen_cobol_ReceiptAtomicNullableString thinkthen_cobol_ReceiptAtomicNullableString;
typedef struct thinkthen_cobol_ReceiptAtomicDecideValueList thinkthen_cobol_ReceiptAtomicDecideValueList;
typedef struct thinkthen_cobol_ReceiptAtomicDecideValue thinkthen_cobol_ReceiptAtomicDecideValue;
typedef struct thinkthen_cobol_ReceiptDecideValuePresence thinkthen_cobol_ReceiptDecideValuePresence;
typedef struct thinkthen_cobol_ReceiptDecideValue thinkthen_cobol_ReceiptDecideValue;
struct thinkthen_cobol_ReceiptExtensions {
    const thinkthen_cobol_ReceiptExtension *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptExtension {
    thinkthen_cobol_AnswerText name;
    const tt_value *value;
};
struct thinkthen_cobol_ReceiptRelatedEntityEdgeList {
    const thinkthen_cobol_ReceiptRelatedEntityEdge *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptRelationEndpointData {
    const thinkthen_cobol_ReceiptRelationEndpointRelatedEntity *related_entity;
    const thinkthen_cobol_ReceiptRelationEndpointSource *source;
};
struct thinkthen_cobol_ReceiptJsonPresence {
    uint32_t presence;
    const tt_value *value;
};
struct thinkthen_cobol_ReceiptU64Presence {
    uint32_t presence;
    uint64_t value;
};
struct thinkthen_cobol_ReceiptUtf8Presence {
    uint32_t presence;
    thinkthen_cobol_AnswerText value;
};
struct thinkthen_cobol_ReceiptRelationEndpointRelatedEntity {
    thinkthen_cobol_AnswerText kind;
    thinkthen_cobol_AnswerText name;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptU32Presence {
    uint32_t presence;
    uint32_t value;
};
struct thinkthen_cobol_ReceiptVersion {
    uint32_t kind;
};
struct thinkthen_cobol_ReceiptWordingVersionPresence {
    uint32_t presence;
    const thinkthen_cobol_ReceiptWordingVersion *value;
};
struct thinkthen_cobol_ReceiptWordingVersion {
    uint64_t value;
};
struct thinkthen_cobol_ReceiptThresholdData {
    double number;
    thinkthen_cobol_AnswerText string;
};
struct thinkthen_cobol_ReceiptRelationRuleList {
    const thinkthen_cobol_ReceiptRelationRule *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptRelationRule {
    uint32_t either;
    thinkthen_cobol_AnswerText name;
    thinkthen_cobol_AnswerText reads;
    thinkthen_cobol_ReceiptU32Presence single;
    thinkthen_cobol_AnswerText source;
    thinkthen_cobol_AnswerText target;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptStringList {
    const thinkthen_cobol_AnswerText *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptQuestionNamePresence {
    uint32_t presence;
    const thinkthen_cobol_ReceiptQuestionName *value;
};
struct thinkthen_cobol_ReceiptQuestionName {
    thinkthen_cobol_AnswerText value;
};
struct thinkthen_cobol_ReceiptLabelList {
    const thinkthen_cobol_ReceiptLabel *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptLabel {
    thinkthen_cobol_ReceiptJsonPresence description;
    thinkthen_cobol_AnswerText name;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptInputDeclarationPresence {
    uint32_t presence;
    const thinkthen_cobol_ReceiptInputDeclaration *value;
};
struct thinkthen_cobol_ReceiptInputDeclarationData {
    const thinkthen_cobol_ReceiptInputDeclarationStringRoot *string_root;
    const thinkthen_cobol_ReceiptInputDeclarationObjectRoot *object_root;
};
struct thinkthen_cobol_ReceiptObjectType {
    uint32_t kind;
};
struct thinkthen_cobol_ReceiptInputPropertyTypeMap {
    const thinkthen_cobol_ReceiptInputPropertyTypeMapEntry *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptInputPropertyTypeMapEntry {
    thinkthen_cobol_AnswerText name;
    const thinkthen_cobol_ReceiptInputPropertyType *value;
};
struct thinkthen_cobol_ReceiptInputPropertyTypeData {
    const thinkthen_cobol_ReceiptInputPropertyTypeString *string;
    const thinkthen_cobol_ReceiptInputPropertyTypeNumber *number;
    const thinkthen_cobol_ReceiptInputPropertyTypeBoolean *boolean;
    const thinkthen_cobol_ReceiptInputPropertyTypeArray *array;
};
struct thinkthen_cobol_ReceiptInputPropertyTypeArray {
    const thinkthen_cobol_ReceiptStringRoot *items;
    thinkthen_cobol_AnswerText type;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptStringRoot {
    const thinkthen_cobol_ReceiptStringType *type;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptStringType {
    uint32_t kind;
};
struct thinkthen_cobol_ReceiptInputPropertyTypeBoolean {
    thinkthen_cobol_AnswerText type;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptInputPropertyTypeNumber {
    thinkthen_cobol_AnswerText type;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptInputPropertyTypeString {
    thinkthen_cobol_AnswerText type;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptInputDeclarationStringRoot {
    const thinkthen_cobol_ReceiptStringType *type;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptRelateFieldsPresence {
    uint32_t presence;
    const thinkthen_cobol_ReceiptRelateFields *value;
};
struct thinkthen_cobol_ReceiptRelateFields {
    thinkthen_cobol_AnswerText kind;
    thinkthen_cobol_AnswerText name;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptBatchPresence {
    uint32_t presence;
    const thinkthen_cobol_ReceiptBatch *value;
};
struct thinkthen_cobol_ReceiptBatchData {
    uint64_t integer;
    const thinkthen_cobol_ReceiptBatchString *string;
};
struct thinkthen_cobol_ReceiptBatchString {
    uint32_t kind;
};
struct thinkthen_cobol_ReceiptPositionPresence {
    uint32_t presence;
    const thinkthen_cobol_ReceiptPosition *value;
};
struct thinkthen_cobol_ReceiptUsagePresence {
    uint32_t presence;
    const thinkthen_cobol_ReceiptUsage *value;
};
struct thinkthen_cobol_ReceiptUsage {
    thinkthen_cobol_ReceiptU64Presence input_tokens;
    thinkthen_cobol_ReceiptU64Presence output_tokens;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptQuestionSourceList {
    const thinkthen_cobol_ReceiptQuestionSource *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptQuestionSource {
    thinkthen_cobol_AnswerText answered_by;
    thinkthen_cobol_ReceiptU64Presence batch_size;
    const thinkthen_cobol_ReceiptOrigin *origin;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptOrigin {
    uint32_t kind;
};
struct thinkthen_cobol_ReceiptProfileWarningPresence {
    uint32_t presence;
    const thinkthen_cobol_ReceiptProfileWarning *value;
};
struct thinkthen_cobol_ReceiptProfileWarning {
    thinkthen_cobol_AnswerText running;
    thinkthen_cobol_AnswerText tuned_for;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptOriginPresence {
    uint32_t presence;
    const thinkthen_cobol_ReceiptOrigin *value;
};
struct thinkthen_cobol_ReceiptObservationList {
    const thinkthen_cobol_ReceiptObservation *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptObservationData {
    const thinkthen_cobol_ReceiptObservationObservationId *observation_id;
    const thinkthen_cobol_ReceiptObservationFailureId *failure_id;
};
struct thinkthen_cobol_ReceiptObservationFailureId {
    const thinkthen_cobol_ReceiptFailureId *failure_id;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptFailureId {
    thinkthen_cobol_AnswerText value;
};
struct thinkthen_cobol_ReceiptObservationObservationId {
    const thinkthen_cobol_ReceiptObservationId *observation_id;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptObservationId {
    thinkthen_cobol_AnswerText value;
};
struct thinkthen_cobol_ReceiptBatchWarningPresence {
    uint32_t presence;
    const thinkthen_cobol_ReceiptBatchWarning *value;
};
struct thinkthen_cobol_ReceiptBatchWarning {
    const thinkthen_cobol_ReceiptBatchSetting *running;
    const thinkthen_cobol_ReceiptBatchSetting *tuned_for;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptBatchSettingData {
    uint64_t integer;
    thinkthen_cobol_AnswerText string;
};
struct thinkthen_cobol_ReceiptBatchSettingPresence {
    uint32_t presence;
    const thinkthen_cobol_ReceiptBatchSetting *value;
};
struct thinkthen_cobol_ReceiptAttemptList {
    const thinkthen_cobol_ReceiptAttempt *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptAttempt {
    uint64_t ordinal;
    const thinkthen_cobol_ReceiptAttemptOutcome *outcome;
    thinkthen_cobol_ReceiptUtf8Presence request_id;
    thinkthen_cobol_AnswerText request_sha256;
    const thinkthen_cobol_ReceiptSdkRequestId *sdk_request_id;
    thinkthen_cobol_ReceiptU64Presence server_ms;
    thinkthen_cobol_ReceiptU64Presence status;
    uint64_t wall_ms;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptSdkRequestId {
    thinkthen_cobol_AnswerText value;
};
struct thinkthen_cobol_ReceiptAttemptOutcome {
    uint32_t kind;
};
struct thinkthen_cobol_ReceiptSessionInputSourceList {
    const thinkthen_cobol_ReceiptSessionInputSource *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptSessionInputSource {
    uint64_t index;
    const thinkthen_cobol_ReceiptPhysicalSource *source;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptPhysicalSource {
    thinkthen_cobol_AnswerText file;
    thinkthen_cobol_ReceiptU64Presence first_line;
    thinkthen_cobol_ReceiptU64Presence last_line;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptAnswerId {
    thinkthen_cobol_AnswerText value;
};
struct thinkthen_cobol_ReceiptRelationMemberList {
    const thinkthen_cobol_ReceiptRelationMember *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptRelationMemberData {
    const thinkthen_cobol_ReceiptRelationMemberAnswerId *answer_id;
    const thinkthen_cobol_ReceiptRelationMemberFailureId *failure_id;
};
struct thinkthen_cobol_ReceiptFailure {
    const thinkthen_cobol_ReceiptFailureCause *cause;
    const thinkthen_cobol_ReceiptFailureFieldKind *kind;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptFailureFieldKind {
    uint32_t kind;
};
struct thinkthen_cobol_ReceiptFailureCause {
    uint32_t kind;
};
struct thinkthen_cobol_ReceiptRelatedEntityPresence {
    uint32_t presence;
    const thinkthen_cobol_ReceiptRelatedEntity *value;
};
struct thinkthen_cobol_ReceiptRelatedEntity {
    thinkthen_cobol_AnswerText kind;
    thinkthen_cobol_AnswerText name;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptQuestionDocumentData {
    const thinkthen_cobol_ReceiptQuestionDocumentDecide *decide;
    const thinkthen_cobol_ReceiptQuestionDocumentChoose *choose;
    const thinkthen_cobol_ReceiptQuestionDocumentTag *tag;
    const thinkthen_cobol_ReceiptQuestionDocumentScore *score;
};
struct thinkthen_cobol_ReceiptRelationMethod {
    uint32_t kind;
};
struct thinkthen_cobol_ReceiptRelationDirection {
    uint32_t kind;
};
struct thinkthen_cobol_ReceiptRelationMemberAnswerId {
    const thinkthen_cobol_ReceiptRelationDirection *direction;
    const thinkthen_cobol_ReceiptRelationMethod *method;
    thinkthen_cobol_ReceiptObservationList observations;
    const thinkthen_cobol_ReceiptQuestionDocument *question;
    thinkthen_cobol_ReceiptQuestionSourceList question_sources;
    thinkthen_cobol_AnswerText reads;
    thinkthen_cobol_AnswerText relation;
    thinkthen_cobol_AnswerText request;
    const thinkthen_cobol_ReceiptRelatedEntity *source;
    thinkthen_cobol_ReceiptRelatedEntityPresence target;
    const thinkthen_cobol_ReceiptThreshold *threshold;
    thinkthen_cobol_ReceiptUsagePresence usage;
    uint32_t accepted;
    const thinkthen_cobol_ReceiptAnswer *answer;
    const thinkthen_cobol_ReceiptAnswerId *answer_id;
    double probability;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptAnswerData {
    const thinkthen_cobol_ReceiptAnswerYesNo *yes_no;
    const thinkthen_cobol_ReceiptAnswerChoice *choice;
    const thinkthen_cobol_ReceiptAnswerTag *tag;
    const thinkthen_cobol_ReceiptAnswerScore *score;
};
struct thinkthen_cobol_ReceiptNumberMap {
    const thinkthen_cobol_ReceiptNumberMapEntry *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptNumberMapEntry {
    thinkthen_cobol_AnswerText name;
    double value;
};
struct thinkthen_cobol_ReceiptF64Presence {
    uint32_t presence;
    double value;
};
struct thinkthen_cobol_ReceiptAnswerTag {
    thinkthen_cobol_AnswerText kind;
    thinkthen_cobol_ReceiptNumberMap probabilities;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptAnswerChoice {
    thinkthen_cobol_ReceiptF64Presence confidence;
    thinkthen_cobol_AnswerText kind;
    thinkthen_cobol_AnswerText pick;
    thinkthen_cobol_ReceiptNumberMap probabilities;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptAnswerYesNo {
    thinkthen_cobol_AnswerText kind;
    double probability;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptRecognitionList {
    const thinkthen_cobol_ReceiptRecognition *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptRecognizeData {
    const thinkthen_cobol_ReceiptRecognizeEntities *entities;
    const thinkthen_cobol_ReceiptRecognizeModeProposals *mode_proposals;
};
struct thinkthen_cobol_ReceiptBoundaryProposalList {
    const thinkthen_cobol_ReceiptBoundaryProposal *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptBoundaryProposal {
    uint64_t end;
    uint64_t length;
    double probability;
    uint64_t start;
    thinkthen_cobol_AnswerText text;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptBoundaryMode {
    uint32_t kind;
};
struct thinkthen_cobol_ReceiptEntityEdgeList {
    const thinkthen_cobol_ReceiptEntityEdge *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptEntityEdge {
    thinkthen_cobol_ReceiptU32Presence either;
    double probability;
    thinkthen_cobol_AnswerText relation;
    const thinkthen_cobol_ReceiptEntity *source;
    const thinkthen_cobol_ReceiptEntity *target;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptEntity {
    uint64_t end;
    thinkthen_cobol_ReceiptUtf8Presence file;
    thinkthen_cobol_ReceiptU64Presence first_line;
    thinkthen_cobol_AnswerText kind;
    thinkthen_cobol_ReceiptU64Presence last_line;
    uint64_t length;
    uint64_t start;
    double strength;
    thinkthen_cobol_AnswerText text;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptEntityList {
    const thinkthen_cobol_ReceiptEntity *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptPhysicalSourcePresence {
    uint32_t presence;
    const thinkthen_cobol_ReceiptPhysicalSource *value;
};
struct thinkthen_cobol_ReceiptVerb {
    uint32_t kind;
};
struct thinkthen_cobol_ReceiptRecognitionStageContextPresence {
    uint32_t presence;
    const thinkthen_cobol_ReceiptRecognitionStageContext *value;
};
struct thinkthen_cobol_ReceiptRecognitionStageContext {
    thinkthen_cobol_ReceiptUtf8Presence boundary;
    thinkthen_cobol_ReceiptUtf8Presence kind_edge;
    thinkthen_cobol_ReceiptUtf8Presence relation;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptRelationRuleListPresence {
    uint32_t presence;
    thinkthen_cobol_ReceiptRelationRuleList value;
};
struct thinkthen_cobol_ReceiptThresholdPresence {
    uint32_t presence;
    const thinkthen_cobol_ReceiptThreshold *value;
};
struct thinkthen_cobol_ReceiptRecognitionModePresence {
    uint32_t presence;
    const thinkthen_cobol_ReceiptRecognitionMode *value;
};
struct thinkthen_cobol_ReceiptRecognitionMode {
    uint32_t kind;
};
struct thinkthen_cobol_ReceiptJsonMap {
    const thinkthen_cobol_ReceiptJsonMapEntry *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptJsonMapEntry {
    thinkthen_cobol_AnswerText name;
    const tt_value *value;
};
struct thinkthen_cobol_ReceiptRecognitionOddsData {
    const thinkthen_cobol_ReceiptRecognitionOddsRecognizeAnswer *recognize_answer;
    const thinkthen_cobol_ReceiptRecognitionOddsBoundaryOdds *boundary_odds;
};
struct thinkthen_cobol_ReceiptPieceOddsList {
    const thinkthen_cobol_ReceiptPieceOdds *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptPieceOdds {
    uint64_t end;
    uint64_t start;
    thinkthen_cobol_ReceiptNumberMap tags;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptRecognitionProposalList {
    const thinkthen_cobol_ReceiptRecognitionProposal *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptPlacePresence {
    uint32_t presence;
    const thinkthen_cobol_ReceiptPlace *value;
};
struct thinkthen_cobol_ReceiptPlace {
    uint64_t end;
    uint64_t start;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptPairOddsList {
    const thinkthen_cobol_ReceiptPairOdds *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptPairOdds {
    double probability;
    thinkthen_cobol_AnswerText relation;
    const thinkthen_cobol_ReceiptPlace *source;
    const thinkthen_cobol_ReceiptPlace *target;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptNameOddsList {
    const thinkthen_cobol_ReceiptNameOdds *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptNumberMapPresence {
    uint32_t presence;
    thinkthen_cobol_ReceiptNumberMap value;
};
struct thinkthen_cobol_ReceiptAnnotationList {
    const thinkthen_cobol_ReceiptAnnotation *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptAnnotatedFieldMap {
    const thinkthen_cobol_ReceiptAnnotatedFieldMapEntry *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptAnnotatedFieldMapEntry {
    thinkthen_cobol_AnswerText name;
    const thinkthen_cobol_ReceiptAnnotatedField *value;
};
struct thinkthen_cobol_ReceiptAnnotatedFieldData {
    uint32_t boolean;
    uint32_t null;
    thinkthen_cobol_AnswerText string;
    thinkthen_cobol_ReceiptStringList array;
    double number;
    const thinkthen_cobol_ReceiptFailed *object;
};
struct thinkthen_cobol_ReceiptFailed {
    const thinkthen_cobol_ReceiptFailure *failed;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptAnnotationMemberMap {
    const thinkthen_cobol_ReceiptAnnotationMemberMapEntry *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptAnnotationMemberMapEntry {
    thinkthen_cobol_AnswerText name;
    const thinkthen_cobol_ReceiptAnnotationMember *value;
};
struct thinkthen_cobol_ReceiptAnnotationMemberData {
    const thinkthen_cobol_ReceiptAnnotationMemberAnswerId *answer_id;
    const thinkthen_cobol_ReceiptAnnotationMemberFailureId *failure_id;
};
struct thinkthen_cobol_ReceiptAnnotationMemberFailureId {
    const thinkthen_cobol_ReceiptFailure *failure;
    const thinkthen_cobol_ReceiptFailureId *failure_id;
    thinkthen_cobol_ReceiptObservationList observations;
    const thinkthen_cobol_ReceiptQuestionDocument *question;
    thinkthen_cobol_ReceiptQuestionSourceList question_sources;
    thinkthen_cobol_AnswerText request;
    thinkthen_cobol_ReceiptThresholdPresence threshold;
    thinkthen_cobol_ReceiptUsagePresence usage;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptValuePresence {
    uint32_t presence;
    const thinkthen_cobol_ReceiptValue *value;
};
struct thinkthen_cobol_ReceiptValueData {
    uint32_t boolean;
    uint32_t null;
    thinkthen_cobol_AnswerText string;
    thinkthen_cobol_ReceiptStringList array;
    double number;
};
struct thinkthen_cobol_ReceiptFindCandidateList {
    const thinkthen_cobol_ReceiptFindCandidate *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptFindCandidate {
    thinkthen_cobol_ReceiptU64Presence index;
    thinkthen_cobol_ReceiptJsonPresence input;
    double probability;
    thinkthen_cobol_ReceiptPhysicalSourcePresence source;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptFindAnswer {
    thinkthen_cobol_ReceiptF64Presence confidence;
    thinkthen_cobol_AnswerText kind;
    thinkthen_cobol_AnswerText pick;
    thinkthen_cobol_ReceiptNumberMap probabilities;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptAtomicNonZeroUsizeList {
    const thinkthen_cobol_ReceiptAtomicNonZeroUsize *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptRankMemberList {
    const thinkthen_cobol_ReceiptRankMember *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptRankMember {
    thinkthen_cobol_AnswerText name;
    const thinkthen_cobol_ReceiptRankMemberResult *result;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptImageList {
    const thinkthen_cobol_ReceiptImage *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptImage {
    thinkthen_cobol_AnswerText base64;
    uint64_t height;
    const thinkthen_cobol_ReceiptImageMedia *media;
    uint64_t width;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptImageMedia {
    uint32_t kind;
};
struct thinkthen_cobol_ReceiptAtomicBooleanList {
    const thinkthen_cobol_ReceiptAtomicBoolean *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptAtomicDoubleList {
    const thinkthen_cobol_ReceiptAtomicDouble *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptAtomicArrayOfStringList {
    const thinkthen_cobol_ReceiptAtomicArrayOfString *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptAtomicNullableStringList {
    const thinkthen_cobol_ReceiptAtomicNullableString *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptAtomicDecideValueList {
    const thinkthen_cobol_ReceiptAtomicDecideValue *data;
    size_t len;
};
struct thinkthen_cobol_ReceiptDecideValuePresence {
    uint32_t presence;
    const thinkthen_cobol_ReceiptDecideValue *value;
};
struct thinkthen_cobol_ReceiptDecideValue {
    const tt_value *value;
};
struct thinkthen_cobol_ReceiptRelatedEntityEdge {
    thinkthen_cobol_ReceiptU32Presence either;
    double probability;
    thinkthen_cobol_AnswerText relation;
    const thinkthen_cobol_ReceiptRelationEndpoint *source;
    const thinkthen_cobol_ReceiptRelationEndpoint *target;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptRelationEndpoint {
    uint32_t kind;
    thinkthen_cobol_ReceiptRelationEndpointData data;
};
struct thinkthen_cobol_ReceiptRelationEndpointSource {
    thinkthen_cobol_ReceiptUtf8Presence file;
    thinkthen_cobol_ReceiptU64Presence first_line;
    thinkthen_cobol_AnswerText kind;
    thinkthen_cobol_ReceiptU64Presence last_line;
    thinkthen_cobol_AnswerText name;
    uint64_t ordinal;
    thinkthen_cobol_ReceiptJsonPresence record;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptThreshold {
    uint32_t kind;
    thinkthen_cobol_ReceiptThresholdData data;
};
struct thinkthen_cobol_ReceiptStringListPresence {
    uint32_t presence;
    thinkthen_cobol_ReceiptStringList value;
};
struct thinkthen_cobol_ReceiptLabelListPresence {
    uint32_t presence;
    thinkthen_cobol_ReceiptLabelList value;
};
struct thinkthen_cobol_ReceiptInputDeclaration {
    uint32_t kind;
    thinkthen_cobol_ReceiptInputDeclarationData data;
};
struct thinkthen_cobol_ReceiptInputDeclarationObjectRoot {
    thinkthen_cobol_ReceiptInputPropertyTypeMap properties;
    thinkthen_cobol_ReceiptStringListPresence required;
    const thinkthen_cobol_ReceiptObjectType *type;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptInputPropertyType {
    uint32_t kind;
    thinkthen_cobol_ReceiptInputPropertyTypeData data;
};
struct thinkthen_cobol_ReceiptBatch {
    uint32_t kind;
    thinkthen_cobol_ReceiptBatchData data;
};
struct thinkthen_cobol_ReceiptPosition {
    thinkthen_cobol_ReceiptUtf8Presence file;
    thinkthen_cobol_ReceiptU64Presence first;
    thinkthen_cobol_ReceiptStringListPresence images;
    thinkthen_cobol_ReceiptU64Presence last;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptObservation {
    uint32_t kind;
    thinkthen_cobol_ReceiptObservationData data;
};
struct thinkthen_cobol_ReceiptBatchSetting {
    uint32_t kind;
    thinkthen_cobol_ReceiptBatchSettingData data;
};
struct thinkthen_cobol_ReceiptAttemptListPresence {
    uint32_t presence;
    thinkthen_cobol_ReceiptAttemptList value;
};
struct thinkthen_cobol_ReceiptSessionInputSourceListPresence {
    uint32_t presence;
    thinkthen_cobol_ReceiptSessionInputSourceList value;
};
struct thinkthen_cobol_ReceiptAnswers {
    thinkthen_cobol_ReceiptRelationMemberList questions;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptRelationMember {
    uint32_t kind;
    thinkthen_cobol_ReceiptRelationMemberData data;
};
struct thinkthen_cobol_ReceiptRelationMemberFailureId {
    const thinkthen_cobol_ReceiptRelationDirection *direction;
    const thinkthen_cobol_ReceiptRelationMethod *method;
    thinkthen_cobol_ReceiptObservationList observations;
    const thinkthen_cobol_ReceiptQuestionDocument *question;
    thinkthen_cobol_ReceiptQuestionSourceList question_sources;
    thinkthen_cobol_AnswerText reads;
    thinkthen_cobol_AnswerText relation;
    thinkthen_cobol_AnswerText request;
    const thinkthen_cobol_ReceiptRelatedEntity *source;
    thinkthen_cobol_ReceiptRelatedEntityPresence target;
    const thinkthen_cobol_ReceiptThreshold *threshold;
    thinkthen_cobol_ReceiptUsagePresence usage;
    const thinkthen_cobol_ReceiptFailure *failure;
    const thinkthen_cobol_ReceiptFailureId *failure_id;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptQuestionDocument {
    uint32_t kind;
    thinkthen_cobol_ReceiptQuestionDocumentData data;
};
struct thinkthen_cobol_ReceiptQuestionDocumentScore {
    thinkthen_cobol_ReceiptBatchPresence batch;
    thinkthen_cobol_ReceiptInputDeclarationPresence context_schema;
    thinkthen_cobol_ReceiptInputDeclarationPresence item_schema;
    thinkthen_cobol_ReceiptLabelListPresence label_details;
    thinkthen_cobol_ReceiptUtf8Presence model;
    thinkthen_cobol_ReceiptQuestionNamePresence name;
    thinkthen_cobol_ReceiptStringListPresence on;
    thinkthen_cobol_ReceiptUtf8Presence profile;
    thinkthen_cobol_ReceiptWordingVersionPresence wording_version;
    thinkthen_cobol_ReceiptStringList levels;
    thinkthen_cobol_ReceiptJsonPresence text;
    thinkthen_cobol_AnswerText verb;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptQuestionDocumentTag {
    thinkthen_cobol_ReceiptBatchPresence batch;
    thinkthen_cobol_ReceiptInputDeclarationPresence context_schema;
    thinkthen_cobol_ReceiptInputDeclarationPresence item_schema;
    thinkthen_cobol_ReceiptLabelListPresence label_details;
    thinkthen_cobol_ReceiptUtf8Presence model;
    thinkthen_cobol_ReceiptQuestionNamePresence name;
    thinkthen_cobol_ReceiptStringListPresence on;
    thinkthen_cobol_ReceiptUtf8Presence profile;
    thinkthen_cobol_ReceiptWordingVersionPresence wording_version;
    thinkthen_cobol_ReceiptStringList labels;
    thinkthen_cobol_ReceiptJsonPresence text;
    thinkthen_cobol_AnswerText verb;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptQuestionDocumentChoose {
    thinkthen_cobol_ReceiptBatchPresence batch;
    thinkthen_cobol_ReceiptInputDeclarationPresence context_schema;
    thinkthen_cobol_ReceiptInputDeclarationPresence item_schema;
    thinkthen_cobol_ReceiptLabelListPresence label_details;
    thinkthen_cobol_ReceiptUtf8Presence model;
    thinkthen_cobol_ReceiptQuestionNamePresence name;
    thinkthen_cobol_ReceiptStringListPresence on;
    thinkthen_cobol_ReceiptUtf8Presence profile;
    thinkthen_cobol_ReceiptWordingVersionPresence wording_version;
    thinkthen_cobol_ReceiptStringList options;
    thinkthen_cobol_ReceiptJsonPresence text;
    thinkthen_cobol_AnswerText verb;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptQuestionDocumentDecide {
    thinkthen_cobol_ReceiptBatchPresence batch;
    thinkthen_cobol_ReceiptInputDeclarationPresence context_schema;
    thinkthen_cobol_ReceiptInputDeclarationPresence item_schema;
    thinkthen_cobol_ReceiptLabelListPresence label_details;
    thinkthen_cobol_ReceiptUtf8Presence model;
    thinkthen_cobol_ReceiptQuestionNamePresence name;
    thinkthen_cobol_ReceiptStringListPresence on;
    thinkthen_cobol_ReceiptUtf8Presence profile;
    thinkthen_cobol_ReceiptWordingVersionPresence wording_version;
    thinkthen_cobol_ReceiptJsonPresence false_;
    thinkthen_cobol_ReceiptJsonPresence text;
    thinkthen_cobol_ReceiptJsonPresence true_;
    thinkthen_cobol_AnswerText verb;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptAnswer {
    uint32_t kind;
    thinkthen_cobol_ReceiptAnswerData data;
};
struct thinkthen_cobol_ReceiptAnswerScore {
    thinkthen_cobol_ReceiptF64Presence confidence;
    thinkthen_cobol_AnswerText kind;
    thinkthen_cobol_AnswerText level;
    thinkthen_cobol_ReceiptNumberMap probabilities;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptRecognition {
    const thinkthen_cobol_ReceiptRecognitionOdds *answer;
    const thinkthen_cobol_ReceiptAnswerId *answer_id;
    thinkthen_cobol_ReceiptUtf8Presence file;
    thinkthen_cobol_ReceiptU64Presence first_line;
    thinkthen_cobol_ReceiptU64Presence index;
    thinkthen_cobol_ReceiptJsonPresence input;
    thinkthen_cobol_ReceiptU64Presence last_line;
    const thinkthen_cobol_ReceiptMeta *meta;
    thinkthen_cobol_ReceiptPositionPresence position;
    const thinkthen_cobol_ReceiptRecognizeQuestionDocument *question;
    const thinkthen_cobol_ReceiptVersion *schema;
    thinkthen_cobol_ReceiptPhysicalSourcePresence source;
    const thinkthen_cobol_ReceiptRecognize *value;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptRecognize {
    uint32_t kind;
    thinkthen_cobol_ReceiptRecognizeData data;
};
struct thinkthen_cobol_ReceiptRecognizeModeProposals {
    const thinkthen_cobol_ReceiptBoundaryMode *mode;
    thinkthen_cobol_ReceiptBoundaryProposalList proposals;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptEntityEdgeListPresence {
    uint32_t presence;
    thinkthen_cobol_ReceiptEntityEdgeList value;
};
struct thinkthen_cobol_ReceiptRecognizeQuestionDocument {
    thinkthen_cobol_ReceiptBatchPresence batch;
    thinkthen_cobol_ReceiptInputDeclarationPresence context_schema;
    thinkthen_cobol_ReceiptJsonPresence entity_definition;
    thinkthen_cobol_ReceiptJsonPresence instructions;
    thinkthen_cobol_ReceiptInputDeclarationPresence item_schema;
    thinkthen_cobol_ReceiptJsonMap kinds;
    thinkthen_cobol_ReceiptLabelListPresence label_details;
    thinkthen_cobol_ReceiptRecognitionModePresence mode;
    thinkthen_cobol_ReceiptUtf8Presence model;
    thinkthen_cobol_ReceiptQuestionNamePresence name;
    thinkthen_cobol_ReceiptStringListPresence on;
    thinkthen_cobol_ReceiptUtf8Presence profile;
    thinkthen_cobol_ReceiptThresholdPresence relation_threshold;
    thinkthen_cobol_ReceiptRelationRuleListPresence relations;
    thinkthen_cobol_ReceiptU64Presence snippet_pieces;
    thinkthen_cobol_ReceiptRecognitionStageContextPresence stage_context;
    const thinkthen_cobol_ReceiptThreshold *threshold;
    const thinkthen_cobol_ReceiptVerb *verb;
    thinkthen_cobol_ReceiptWordingVersionPresence wording_version;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptRecognitionOdds {
    uint32_t kind;
    thinkthen_cobol_ReceiptRecognitionOddsData data;
};
struct thinkthen_cobol_ReceiptRecognitionOddsBoundaryOdds {
    thinkthen_cobol_ReceiptPieceOddsList pieces;
    thinkthen_cobol_ReceiptBoundaryProposalList proposals;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptRecognitionOddsRecognizeAnswer {
    thinkthen_cobol_ReceiptNameOddsList names;
    thinkthen_cobol_ReceiptPairOddsList pairs;
    thinkthen_cobol_ReceiptPieceOddsList pieces;
    thinkthen_cobol_ReceiptRecognitionProposalList proposals;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptRecognitionProposal {
    uint64_t end;
    uint32_t kept;
    thinkthen_cobol_ReceiptUtf8Presence kind;
    thinkthen_cobol_ReceiptPlacePresence selected;
    double span_probability;
    uint64_t start;
    thinkthen_cobol_ReceiptF64Presence strength;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptNameOdds {
    thinkthen_cobol_ReceiptNumberMapPresence edges;
    uint64_t end;
    thinkthen_cobol_ReceiptNumberMapPresence kinds;
    uint64_t start;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptAnnotation {
    const thinkthen_cobol_ReceiptAnswerId *answer_id;
    thinkthen_cobol_ReceiptAnnotationMemberMap answers;
    thinkthen_cobol_ReceiptUtf8Presence file;
    thinkthen_cobol_ReceiptU64Presence first_line;
    thinkthen_cobol_ReceiptU64Presence index;
    thinkthen_cobol_ReceiptJsonPresence input;
    thinkthen_cobol_ReceiptU64Presence last_line;
    const thinkthen_cobol_ReceiptMeta *meta;
    thinkthen_cobol_ReceiptPositionPresence position;
    const thinkthen_cobol_ReceiptVersion *schema;
    thinkthen_cobol_ReceiptPhysicalSourcePresence source;
    const thinkthen_cobol_ReceiptAnnotatedRow *value;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptAnnotatedRow {
    thinkthen_cobol_ReceiptAnnotatedFieldMap value;
};
struct thinkthen_cobol_ReceiptAnnotatedField {
    uint32_t kind;
    thinkthen_cobol_ReceiptAnnotatedFieldData data;
};
struct thinkthen_cobol_ReceiptAnnotationMember {
    uint32_t kind;
    thinkthen_cobol_ReceiptAnnotationMemberData data;
};
struct thinkthen_cobol_ReceiptAnnotationMemberAnswerId {
    const thinkthen_cobol_ReceiptAnswer *answer;
    const thinkthen_cobol_ReceiptAnswerId *answer_id;
    thinkthen_cobol_ReceiptObservationList observations;
    const thinkthen_cobol_ReceiptQuestionDocument *question;
    thinkthen_cobol_ReceiptQuestionSourceList question_sources;
    thinkthen_cobol_AnswerText request;
    thinkthen_cobol_ReceiptThresholdPresence threshold;
    thinkthen_cobol_ReceiptUsagePresence usage;
    thinkthen_cobol_ReceiptValuePresence value;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptValue {
    uint32_t kind;
    thinkthen_cobol_ReceiptValueData data;
};
struct thinkthen_cobol_ReceiptFindQuestionDocument {
    thinkthen_cobol_ReceiptBatchPresence batch;
    thinkthen_cobol_ReceiptInputDeclarationPresence context_schema;
    thinkthen_cobol_ReceiptInputDeclarationPresence item_schema;
    thinkthen_cobol_ReceiptLabelListPresence label_details;
    thinkthen_cobol_ReceiptUtf8Presence model;
    thinkthen_cobol_ReceiptQuestionNamePresence name;
    uint32_t none;
    thinkthen_cobol_ReceiptStringListPresence on;
    thinkthen_cobol_ReceiptUtf8Presence profile;
    thinkthen_cobol_ReceiptJsonPresence text;
    thinkthen_cobol_AnswerText verb;
    thinkthen_cobol_ReceiptWordingVersionPresence wording_version;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptFindCandidateListPresence {
    uint32_t presence;
    thinkthen_cobol_ReceiptFindCandidateList value;
};
struct thinkthen_cobol_ReceiptRankMemberListPresence {
    uint32_t presence;
    thinkthen_cobol_ReceiptRankMemberList value;
};
struct thinkthen_cobol_ReceiptImageListPresence {
    uint32_t presence;
    thinkthen_cobol_ReceiptImageList value;
};
struct thinkthen_cobol_ReceiptAtomicBoolean {
    const thinkthen_cobol_ReceiptAnswer *answer;
    const thinkthen_cobol_ReceiptAnswerId *answer_id;
    thinkthen_cobol_ReceiptImageListPresence images;
    thinkthen_cobol_ReceiptU64Presence index;
    thinkthen_cobol_ReceiptJsonPresence input;
    thinkthen_cobol_ReceiptRankMemberListPresence members;
    const thinkthen_cobol_ReceiptMeta *meta;
    const thinkthen_cobol_ReceiptQuestionDocument *question;
    thinkthen_cobol_ReceiptUtf8Presence question_name;
    const thinkthen_cobol_ReceiptVersion *schema;
    thinkthen_cobol_ReceiptPhysicalSourcePresence source;
    thinkthen_cobol_ReceiptThresholdPresence threshold;
    uint32_t value;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptAtomicDouble {
    const thinkthen_cobol_ReceiptAnswer *answer;
    const thinkthen_cobol_ReceiptAnswerId *answer_id;
    thinkthen_cobol_ReceiptImageListPresence images;
    thinkthen_cobol_ReceiptU64Presence index;
    thinkthen_cobol_ReceiptJsonPresence input;
    thinkthen_cobol_ReceiptRankMemberListPresence members;
    const thinkthen_cobol_ReceiptMeta *meta;
    const thinkthen_cobol_ReceiptQuestionDocument *question;
    thinkthen_cobol_ReceiptUtf8Presence question_name;
    const thinkthen_cobol_ReceiptVersion *schema;
    thinkthen_cobol_ReceiptPhysicalSourcePresence source;
    thinkthen_cobol_ReceiptThresholdPresence threshold;
    double value;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptAtomicArrayOfString {
    const thinkthen_cobol_ReceiptAnswer *answer;
    const thinkthen_cobol_ReceiptAnswerId *answer_id;
    thinkthen_cobol_ReceiptImageListPresence images;
    thinkthen_cobol_ReceiptU64Presence index;
    thinkthen_cobol_ReceiptJsonPresence input;
    thinkthen_cobol_ReceiptRankMemberListPresence members;
    const thinkthen_cobol_ReceiptMeta *meta;
    const thinkthen_cobol_ReceiptQuestionDocument *question;
    thinkthen_cobol_ReceiptUtf8Presence question_name;
    const thinkthen_cobol_ReceiptVersion *schema;
    thinkthen_cobol_ReceiptPhysicalSourcePresence source;
    thinkthen_cobol_ReceiptThresholdPresence threshold;
    thinkthen_cobol_ReceiptStringList value;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptAtomicNullableString {
    const thinkthen_cobol_ReceiptAnswer *answer;
    const thinkthen_cobol_ReceiptAnswerId *answer_id;
    thinkthen_cobol_ReceiptImageListPresence images;
    thinkthen_cobol_ReceiptU64Presence index;
    thinkthen_cobol_ReceiptJsonPresence input;
    thinkthen_cobol_ReceiptRankMemberListPresence members;
    const thinkthen_cobol_ReceiptMeta *meta;
    const thinkthen_cobol_ReceiptQuestionDocument *question;
    thinkthen_cobol_ReceiptUtf8Presence question_name;
    const thinkthen_cobol_ReceiptVersion *schema;
    thinkthen_cobol_ReceiptPhysicalSourcePresence source;
    thinkthen_cobol_ReceiptThresholdPresence threshold;
    thinkthen_cobol_ReceiptUtf8Presence value;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptAtomicDecideValue {
    const thinkthen_cobol_ReceiptAnswer *answer;
    const thinkthen_cobol_ReceiptAnswerId *answer_id;
    thinkthen_cobol_ReceiptImageListPresence images;
    thinkthen_cobol_ReceiptU64Presence index;
    thinkthen_cobol_ReceiptJsonPresence input;
    thinkthen_cobol_ReceiptRankMemberListPresence members;
    const thinkthen_cobol_ReceiptMeta *meta;
    const thinkthen_cobol_ReceiptQuestionDocument *question;
    thinkthen_cobol_ReceiptUtf8Presence question_name;
    const thinkthen_cobol_ReceiptVersion *schema;
    thinkthen_cobol_ReceiptPhysicalSourcePresence source;
    thinkthen_cobol_ReceiptThresholdPresence threshold;
    thinkthen_cobol_ReceiptDecideValuePresence value;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptRelation {
    const thinkthen_cobol_ReceiptAnswers *answer;
    const thinkthen_cobol_ReceiptAnswerId *answer_id;
    thinkthen_cobol_ReceiptUtf8Presence file;
    thinkthen_cobol_ReceiptU64Presence first_line;
    thinkthen_cobol_ReceiptU64Presence index;
    thinkthen_cobol_ReceiptJsonPresence input;
    thinkthen_cobol_ReceiptSessionInputSourceListPresence input_sources;
    thinkthen_cobol_ReceiptU64Presence last_line;
    const thinkthen_cobol_ReceiptMeta *meta;
    thinkthen_cobol_ReceiptPositionPresence position;
    const thinkthen_cobol_ReceiptRelateQuestionDocument *question;
    const thinkthen_cobol_ReceiptVersion *schema;
    thinkthen_cobol_ReceiptRelatedEntityEdgeList value;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptRelateQuestionDocument {
    thinkthen_cobol_ReceiptBatchPresence batch;
    thinkthen_cobol_ReceiptInputDeclarationPresence context_schema;
    thinkthen_cobol_ReceiptRelateFieldsPresence fields;
    thinkthen_cobol_ReceiptInputDeclarationPresence item_schema;
    thinkthen_cobol_ReceiptLabelListPresence label_details;
    thinkthen_cobol_ReceiptUtf8Presence model;
    thinkthen_cobol_ReceiptQuestionNamePresence name;
    thinkthen_cobol_ReceiptStringListPresence on;
    thinkthen_cobol_ReceiptUtf8Presence profile;
    thinkthen_cobol_ReceiptRelationRuleList relations;
    const thinkthen_cobol_ReceiptThreshold *threshold;
    thinkthen_cobol_AnswerText verb;
    thinkthen_cobol_ReceiptWordingVersionPresence wording_version;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptMeta {
    thinkthen_cobol_ReceiptUtf8Presence answered_by;
    thinkthen_cobol_ReceiptAttemptListPresence attempts;
    thinkthen_cobol_ReceiptBatchSettingPresence batch_setting;
    thinkthen_cobol_ReceiptBatchWarningPresence batch_warning;
    uint32_t cached;
    thinkthen_cobol_ReceiptUtf8Presence context_sha256;
    uint64_t failed_questions;
    thinkthen_cobol_AnswerText model;
    thinkthen_cobol_ReceiptObservationList observations;
    thinkthen_cobol_ReceiptOriginPresence origin;
    thinkthen_cobol_ReceiptProfileWarningPresence profile_warning;
    thinkthen_cobol_ReceiptUtf8Presence question_sha256;
    thinkthen_cobol_ReceiptQuestionSourceList question_sources;
    thinkthen_cobol_ReceiptUtf8Presence questions_sha256;
    thinkthen_cobol_ReceiptStringList requests;
    uint64_t requests_sent;
    thinkthen_cobol_AnswerText tool;
    thinkthen_cobol_AnswerText url;
    thinkthen_cobol_ReceiptUsagePresence usage;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptRecognizeEntities {
    thinkthen_cobol_ReceiptEntityList entities;
    thinkthen_cobol_ReceiptEntityEdgeListPresence relations;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptFind {
    const thinkthen_cobol_ReceiptFindAnswer *answer;
    const thinkthen_cobol_ReceiptAnswerId *answer_id;
    thinkthen_cobol_ReceiptFindCandidateListPresence candidates;
    thinkthen_cobol_ReceiptUtf8Presence file;
    thinkthen_cobol_ReceiptU64Presence first_line;
    thinkthen_cobol_ReceiptU64Presence index;
    thinkthen_cobol_ReceiptU64Presence last_line;
    const thinkthen_cobol_ReceiptMeta *meta;
    thinkthen_cobol_ReceiptPositionPresence position;
    const thinkthen_cobol_ReceiptFindQuestionDocument *question;
    const thinkthen_cobol_ReceiptVersion *schema;
    thinkthen_cobol_ReceiptJsonPresence threshold;
    thinkthen_cobol_ReceiptJsonPresence value;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptAtomicNonZeroUsize {
    const thinkthen_cobol_ReceiptAnswer *answer;
    const thinkthen_cobol_ReceiptAnswerId *answer_id;
    thinkthen_cobol_ReceiptImageListPresence images;
    thinkthen_cobol_ReceiptU64Presence index;
    thinkthen_cobol_ReceiptJsonPresence input;
    thinkthen_cobol_ReceiptRankMemberListPresence members;
    const thinkthen_cobol_ReceiptMeta *meta;
    const thinkthen_cobol_ReceiptQuestionDocument *question;
    thinkthen_cobol_ReceiptUtf8Presence question_name;
    const thinkthen_cobol_ReceiptVersion *schema;
    thinkthen_cobol_ReceiptPhysicalSourcePresence source;
    thinkthen_cobol_ReceiptThresholdPresence threshold;
    uint64_t value;
    thinkthen_cobol_ReceiptExtensions extensions;
};
struct thinkthen_cobol_ReceiptRankMemberResult {
    const thinkthen_cobol_ReceiptAnswer *answer;
    const thinkthen_cobol_ReceiptAnswerId *answer_id;
    thinkthen_cobol_ReceiptImageListPresence images;
    const thinkthen_cobol_ReceiptMeta *meta;
    const thinkthen_cobol_ReceiptQuestionDocument *question;
    const thinkthen_cobol_ReceiptVersion *schema;
    thinkthen_cobol_ReceiptPhysicalSourcePresence source;
    thinkthen_cobol_ReceiptJsonPresence threshold;
    uint64_t value;
    thinkthen_cobol_ReceiptExtensions extensions;
};
typedef struct thinkthen_cobol_ReceiptFindList {
    const thinkthen_cobol_ReceiptFind *data;
    uint64_t len;
} thinkthen_cobol_ReceiptFindList;
typedef struct thinkthen_cobol_ReceiptRelationList {
    const thinkthen_cobol_ReceiptRelation *data;
    uint64_t len;
} thinkthen_cobol_ReceiptRelationList;
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
typedef struct thinkthen_cobol_AnswerDecideValue {
    uint64_t scalar;
    const thinkthen_cobol_AnswerDecision *rows;
    uint64_t count;
    thinkthen_cobol_AnswerDecision first;
} thinkthen_cobol_AnswerDecideValue;
typedef struct thinkthen_cobol_AnswerDecideResult {
    tt_answer_owner *owner;
    thinkthen_cobol_AnswerDecideValue value;
    thinkthen_cobol_AnswerProbability probability;
    thinkthen_cobol_ReceiptAtomicDecideValueList results;
    uint64_t facts_present;
    thinkthen_cobol_AnswerFacts facts;
} thinkthen_cobol_AnswerDecideResult;
int TT_DECIDE(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerDecideResult *out,thinkthen_cobol_AnswerStatus *status_out);
typedef struct thinkthen_cobol_AnswerChooseValue {
    uint64_t scalar;
    const thinkthen_cobol_AnswerChoice *rows;
    uint64_t count;
    thinkthen_cobol_AnswerChoice first;
} thinkthen_cobol_AnswerChooseValue;
typedef struct thinkthen_cobol_AnswerChooseResult {
    tt_answer_owner *owner;
    thinkthen_cobol_AnswerChooseValue value;
    thinkthen_cobol_AnswerProbability probability;
    thinkthen_cobol_ReceiptAtomicNullableStringList results;
    uint64_t facts_present;
    thinkthen_cobol_AnswerFacts facts;
} thinkthen_cobol_AnswerChooseResult;
int TT_CHOOSE(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerChooseResult *out,thinkthen_cobol_AnswerStatus *status_out);
typedef struct thinkthen_cobol_AnswerTagValue {
    uint64_t scalar;
    const thinkthen_cobol_AnswerTable *rows;
    uint64_t count;
    thinkthen_cobol_AnswerTable first;
} thinkthen_cobol_AnswerTagValue;
typedef struct thinkthen_cobol_AnswerTagResult {
    tt_answer_owner *owner;
    thinkthen_cobol_AnswerTagValue value;
    thinkthen_cobol_AnswerProbability probability;
    thinkthen_cobol_ReceiptAtomicArrayOfStringList results;
    uint64_t facts_present;
    thinkthen_cobol_AnswerFacts facts;
} thinkthen_cobol_AnswerTagResult;
int TT_TAG(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerTagResult *out,thinkthen_cobol_AnswerStatus *status_out);
typedef struct thinkthen_cobol_AnswerScoreValue {
    uint64_t scalar;
    const thinkthen_cobol_AnswerScore *rows;
    uint64_t count;
    thinkthen_cobol_AnswerScore first;
} thinkthen_cobol_AnswerScoreValue;
typedef struct thinkthen_cobol_AnswerScoreResult {
    tt_answer_owner *owner;
    thinkthen_cobol_AnswerScoreValue value;
    thinkthen_cobol_AnswerProbability probability;
    thinkthen_cobol_ReceiptAtomicDoubleList results;
    uint64_t facts_present;
    thinkthen_cobol_AnswerFacts facts;
} thinkthen_cobol_AnswerScoreResult;
int TT_SCORE(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerScoreResult *out,thinkthen_cobol_AnswerStatus *status_out);
typedef struct thinkthen_cobol_AnswerFilterValue {
    uint64_t scalar;
    const thinkthen_cobol_AnswerFilterOriginal *rows;
    uint64_t count;
    thinkthen_cobol_AnswerFilterOriginal first;
} thinkthen_cobol_AnswerFilterValue;
typedef struct thinkthen_cobol_AnswerFilterResult {
    tt_answer_owner *owner;
    thinkthen_cobol_AnswerFilterValue value;
    thinkthen_cobol_AnswerProbability probability;
    thinkthen_cobol_ReceiptAtomicBooleanList results;
    uint64_t facts_present;
    thinkthen_cobol_AnswerFacts facts;
} thinkthen_cobol_AnswerFilterResult;
int TT_FILTER(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerFilterResult *out,thinkthen_cobol_AnswerStatus *status_out);
typedef struct thinkthen_cobol_AnswerRankValue {
    uint64_t scalar;
    const thinkthen_cobol_AnswerRankIndex *rows;
    uint64_t count;
    thinkthen_cobol_AnswerRankIndex first;
} thinkthen_cobol_AnswerRankValue;
typedef struct thinkthen_cobol_AnswerRankResult {
    tt_answer_owner *owner;
    thinkthen_cobol_AnswerRankValue value;
    thinkthen_cobol_AnswerProbability probability;
    thinkthen_cobol_ReceiptAtomicNonZeroUsizeList results;
    uint64_t facts_present;
    thinkthen_cobol_AnswerFacts facts;
} thinkthen_cobol_AnswerRankResult;
int TT_RANK(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerRankResult *out,thinkthen_cobol_AnswerStatus *status_out);
typedef struct thinkthen_cobol_AnswerFindValue {
    uint64_t scalar;
    const thinkthen_cobol_AnswerFindSelection *rows;
    uint64_t count;
    thinkthen_cobol_AnswerFindSelection first;
} thinkthen_cobol_AnswerFindValue;
typedef struct thinkthen_cobol_AnswerFindResult {
    tt_answer_owner *owner;
    thinkthen_cobol_AnswerFindValue value;
    thinkthen_cobol_AnswerProbability probability;
    thinkthen_cobol_ReceiptFindList results;
    uint64_t facts_present;
    thinkthen_cobol_AnswerFacts facts;
} thinkthen_cobol_AnswerFindResult;
int TT_FIND(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerFindResult *out,thinkthen_cobol_AnswerStatus *status_out);
typedef struct thinkthen_cobol_AnswerAnnotateValue {
    uint64_t scalar;
    const thinkthen_cobol_AnswerAnnotationField *rows;
    uint64_t count;
    thinkthen_cobol_AnswerAnnotationField first;
} thinkthen_cobol_AnswerAnnotateValue;
typedef struct thinkthen_cobol_AnswerAnnotateResult {
    tt_answer_owner *owner;
    thinkthen_cobol_AnswerAnnotateValue value;
    thinkthen_cobol_AnswerProbability probability;
    thinkthen_cobol_ReceiptAnnotationList results;
    uint64_t facts_present;
    thinkthen_cobol_AnswerFacts facts;
} thinkthen_cobol_AnswerAnnotateResult;
int TT_ANNOTATE(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerAnnotateResult *out,thinkthen_cobol_AnswerStatus *status_out);
typedef struct thinkthen_cobol_AnswerRecognizeValue {
    uint64_t scalar;
    const thinkthen_cobol_AnswerRecognition *rows;
    uint64_t count;
    thinkthen_cobol_AnswerRecognition first;
    const thinkthen_cobol_AnswerRecognitionEdge *edges;
    uint64_t edge_count;
} thinkthen_cobol_AnswerRecognizeValue;
typedef struct thinkthen_cobol_AnswerRecognizeResult {
    tt_answer_owner *owner;
    thinkthen_cobol_AnswerRecognizeValue value;
    thinkthen_cobol_AnswerProbability probability;
    thinkthen_cobol_ReceiptRecognitionList results;
    uint64_t facts_present;
    thinkthen_cobol_AnswerFacts facts;
} thinkthen_cobol_AnswerRecognizeResult;
int TT_RECOGNIZE(const void *engine,const thinkthen_cobol_RequestQuestion *question,const thinkthen_cobol_RequestInput *input,
const thinkthen_cobol_RequestOptions *options,thinkthen_cobol_AnswerRecognizeResult *out,thinkthen_cobol_AnswerStatus *status_out);
typedef struct thinkthen_cobol_AnswerRelateValue {
    uint64_t scalar;
    const thinkthen_cobol_AnswerRelation *rows;
    uint64_t count;
    thinkthen_cobol_AnswerRelation first;
} thinkthen_cobol_AnswerRelateValue;
typedef struct thinkthen_cobol_AnswerRelateResult {
    tt_answer_owner *owner;
    thinkthen_cobol_AnswerRelateValue value;
    thinkthen_cobol_AnswerProbability probability;
    thinkthen_cobol_ReceiptRelationList results;
    uint64_t facts_present;
    thinkthen_cobol_AnswerFacts facts;
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
