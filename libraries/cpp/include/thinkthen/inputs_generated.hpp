// Generated from request.schema.json; do not edit.
#pragma once
#include "result_value.hpp"
namespace tt::inputs {
using results::Node;
class AuthoredChoose;
class AuthoredCriterion;
class AuthoredCut;
class AuthoredDecide;
class AuthoredDescription;
class AuthoredFind;
class AuthoredInputDeclaration;
class AuthoredInputDeclarationObject;
class AuthoredInputDeclarationString;
class AuthoredInputProperty;
class AuthoredInputPropertyArray;
class AuthoredInputPropertyBoolean;
class AuthoredInputPropertyNumber;
class AuthoredInputPropertyString;
class AuthoredLabels;
class AuthoredLevels;
class AuthoredName;
class AuthoredOptions;
class AuthoredPointers;
class AuthoredProfile;
class AuthoredQuestionText;
class AuthoredRelate;
class AuthoredRelation;
class AuthoredScore;
class AuthoredTag;
class AuthoredThreshold;
class ContextSchema;
class ImageMedia;
class OptionSchema;
class ReaderMedia;
class RecognitionExample;
class RecognitionExampleEntity;
class RecognitionExampleText;
class RecognitionMode;
class RecognitionSeedSpan;
class RecognitionStageContext;
class RequestBatch;
class RequestDefinition;
class RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties;
class RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose;
class RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide;
class RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore;
class RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag;
class RequestDefinitionFieldsChoose;
class RequestDefinitionFieldsDecide;
class RequestDefinitionFieldsFind;
class RequestDefinitionFieldsQuestionsVersion;
class RequestDefinitionFieldsRecognizeVersion;
class RequestDefinitionFieldsRelateVersion;
class RequestDefinitionFieldsScore;
class RequestDefinitionFieldsTag;
class RequestFraming;
class RequestImage;
class RequestImageBytes;
class RequestImageFile;
class RequestInput;
class RequestInputEntities;
class RequestInputFeed;
class RequestInputJson;
class RequestInputRecords;
class RequestInputSource;
class RequestInputText;
class RequestInputUnits;
class RequestItem;
class RequestOptions;
class RequestOriginal;
class RequestOriginalJson;
class RequestOriginalText;
class RequestQuestion;
class RequestQuestionDefinition;
class RequestQuestionFile;
class RequestQuestionName;
class RequestQuestionReference;
class RequestQuestionText;
class RequestReader;
class RequestSessionDescriptor;
class RequestSource;
class RequestThreshold;
class SessionSourceLocation;
class SourceUnit;
class AuthoredChoose : public Node { public:
    AuthoredChoose():Node(Json(Json::Object{})) {}
    AuthoredChoose& set_batch(const std::variant<std::string, uint64_t>& value);
    AuthoredChoose& set_choose(const AuthoredQuestionText& value);
    AuthoredChoose& set_context_schema(const AuthoredInputDeclaration& value);
    AuthoredChoose& set_item_schema(const AuthoredInputDeclaration& value);
    AuthoredChoose& set_model(const AuthoredName& value);
    AuthoredChoose& set_name(const std::string& value);
    AuthoredChoose& set_on(const AuthoredPointers& value);
    AuthoredChoose& set_options(const AuthoredOptions& value);
    AuthoredChoose& set_profile(const AuthoredProfile& value);
    AuthoredChoose& set_threshold(const AuthoredCut& value);
    AuthoredChoose& set_wording_version(const uint64_t& value);
};
class AuthoredCriterion : public Node { public:
    explicit AuthoredCriterion(const std::variant<std::string, Json, std::vector<Json>, std::nullptr_t>& value);
};
class AuthoredCut : public Node { public:
    explicit AuthoredCut(const std::variant<double, std::string>& value);
};
class AuthoredDecide : public Node { public:
    AuthoredDecide():Node(Json(Json::Object{})) {}
    AuthoredDecide& set_batch(const std::variant<std::string, uint64_t>& value);
    AuthoredDecide& set_context_schema(const AuthoredInputDeclaration& value);
    AuthoredDecide& set_decide(const AuthoredQuestionText& value);
    AuthoredDecide& set_false(const AuthoredCriterion& value);
    AuthoredDecide& set_item_schema(const AuthoredInputDeclaration& value);
    AuthoredDecide& set_model(const AuthoredName& value);
    AuthoredDecide& set_name(const std::string& value);
    AuthoredDecide& set_on(const AuthoredPointers& value);
    AuthoredDecide& set_profile(const AuthoredProfile& value);
    AuthoredDecide& set_threshold(const AuthoredThreshold& value);
    AuthoredDecide& set_true(const AuthoredCriterion& value);
    AuthoredDecide& set_wording_version(const uint64_t& value);
};
class AuthoredDescription : public Node { public:
    explicit AuthoredDescription(const std::variant<std::string, Json, std::vector<Json>, std::nullptr_t>& value);
};
class AuthoredFind : public Node { public:
    AuthoredFind():Node(Json(Json::Object{})) {}
    AuthoredFind& set_context_schema(const AuthoredInputDeclaration& value);
    AuthoredFind& set_find(const AuthoredQuestionText& value);
    AuthoredFind& set_item_schema(const AuthoredInputDeclaration& value);
    AuthoredFind& set_model(const AuthoredName& value);
    AuthoredFind& set_name(const std::string& value);
    AuthoredFind& set_on(const AuthoredPointers& value);
    AuthoredFind& set_profile(const AuthoredProfile& value);
    AuthoredFind& set_wording_version(const uint64_t& value);
};
class AuthoredInputDeclaration : public Node { public:
    AuthoredInputDeclaration(const AuthoredInputDeclarationString& value);
    AuthoredInputDeclaration(const AuthoredInputDeclarationObject& value);
};
class AuthoredInputDeclarationObject : public Node { public:
    AuthoredInputDeclarationObject():Node(Json(Json::Object{{"type",Json("object")}})) {}
    AuthoredInputDeclarationObject& set_properties(const std::vector<std::pair<std::string, AuthoredInputProperty>>& value);
    AuthoredInputDeclarationObject& set_required(const std::vector<std::string>& value);
};
class AuthoredInputDeclarationString : public Node { public:
    AuthoredInputDeclarationString():Node(Json(Json::Object{{"type",Json("string")}})) {}
};
class AuthoredInputProperty : public Node { public:
    AuthoredInputProperty(const AuthoredInputPropertyString& value);
    AuthoredInputProperty(const AuthoredInputPropertyNumber& value);
    AuthoredInputProperty(const AuthoredInputPropertyBoolean& value);
    AuthoredInputProperty(const AuthoredInputPropertyArray& value);
};
class AuthoredInputPropertyArray : public Node { public:
    AuthoredInputPropertyArray():Node(Json(Json::Object{{"type",Json("array")}})) {}
    AuthoredInputPropertyArray& set_items(const Json& value);
};
class AuthoredInputPropertyBoolean : public Node { public:
    AuthoredInputPropertyBoolean():Node(Json(Json::Object{{"type",Json("boolean")}})) {}
};
class AuthoredInputPropertyNumber : public Node { public:
    AuthoredInputPropertyNumber():Node(Json(Json::Object{{"type",Json("number")}})) {}
};
class AuthoredInputPropertyString : public Node { public:
    AuthoredInputPropertyString():Node(Json(Json::Object{{"type",Json("string")}})) {}
};
class AuthoredLabels : public Node { public:
    explicit AuthoredLabels(const std::variant<std::vector<AuthoredName>, std::vector<std::pair<std::string, AuthoredDescription>>>& value);
};
class AuthoredLevels : public Node { public:
    explicit AuthoredLevels(const std::variant<std::vector<AuthoredName>, std::vector<std::pair<std::string, AuthoredCriterion>>>& value);
};
class AuthoredName : public Node { public:
    explicit AuthoredName(const std::string& value);
};
class AuthoredOptions : public Node { public:
    explicit AuthoredOptions(const std::variant<std::vector<AuthoredName>, std::vector<std::pair<std::string, AuthoredDescription>>>& value);
};
class AuthoredPointers : public Node { public:
    explicit AuthoredPointers(const std::variant<std::string, std::vector<std::string>>& value);
};
class AuthoredProfile : public Node { public:
    explicit AuthoredProfile(const std::string& value);
};
class AuthoredQuestionText : public Node { public:
    explicit AuthoredQuestionText(const std::variant<std::string, Json, std::vector<Json>>& value);
};
class AuthoredRelate : public Node { public:
    AuthoredRelate():Node(Json(Json::Object{{"version",Json(1)}})) {}
    AuthoredRelate& set_context_schema(const AuthoredInputDeclaration& value);
    AuthoredRelate& set_item_schema(const AuthoredInputDeclaration& value);
    AuthoredRelate& set_model(const AuthoredName& value);
    AuthoredRelate& set_name(const std::string& value);
    AuthoredRelate& set_profile(const AuthoredProfile& value);
    AuthoredRelate& set_relate(const Json& value);
    AuthoredRelate& set_threshold(const AuthoredCut& value);
    AuthoredRelate& set_wording_version(const uint64_t& value);
};
class AuthoredRelation : public Node { public:
    AuthoredRelation():Node(Json(Json::Object{})) {}
    AuthoredRelation& set_either(const bool& value);
    AuthoredRelation& set_name(const AuthoredName& value);
    AuthoredRelation& set_reads(const AuthoredName& value);
    AuthoredRelation& set_single(const bool& value);
    AuthoredRelation& set_source(const AuthoredName& value);
    AuthoredRelation& set_target(const AuthoredName& value);
};
class AuthoredScore : public Node { public:
    AuthoredScore():Node(Json(Json::Object{})) {}
    AuthoredScore& set_batch(const std::variant<std::string, uint64_t>& value);
    AuthoredScore& set_context_schema(const AuthoredInputDeclaration& value);
    AuthoredScore& set_item_schema(const AuthoredInputDeclaration& value);
    AuthoredScore& set_levels(const AuthoredLevels& value);
    AuthoredScore& set_model(const AuthoredName& value);
    AuthoredScore& set_name(const std::string& value);
    AuthoredScore& set_on(const AuthoredPointers& value);
    AuthoredScore& set_profile(const AuthoredProfile& value);
    AuthoredScore& set_score(const AuthoredQuestionText& value);
    AuthoredScore& set_wording_version(const uint64_t& value);
};
class AuthoredTag : public Node { public:
    AuthoredTag():Node(Json(Json::Object{})) {}
    AuthoredTag& set_batch(const std::variant<std::string, uint64_t>& value);
    AuthoredTag& set_context_schema(const AuthoredInputDeclaration& value);
    AuthoredTag& set_item_schema(const AuthoredInputDeclaration& value);
    AuthoredTag& set_labels(const AuthoredLabels& value);
    AuthoredTag& set_model(const AuthoredName& value);
    AuthoredTag& set_name(const std::string& value);
    AuthoredTag& set_on(const AuthoredPointers& value);
    AuthoredTag& set_profile(const AuthoredProfile& value);
    AuthoredTag& set_tag(const AuthoredQuestionText& value);
    AuthoredTag& set_threshold(const AuthoredCut& value);
    AuthoredTag& set_wording_version(const uint64_t& value);
};
class AuthoredThreshold : public Node { public:
    explicit AuthoredThreshold(const std::variant<double, std::string>& value);
};
class ContextSchema : public Node { public:
    explicit ContextSchema(const std::variant<std::string, Json>& value);
};
class ImageMedia : public Node { public:
    explicit ImageMedia(const std::string& value);
};
class OptionSchema : public Node { public:
    OptionSchema():Node(Json(Json::Object{})) {}
    OptionSchema& set_description(const Json& value);
    OptionSchema& set_name(const std::string& value);
};
class ReaderMedia : public Node { public:
    explicit ReaderMedia(const std::string& value);
};
class RecognitionExample : public Node { public:
    explicit RecognitionExample(const std::variant<std::string, RecognitionExampleText>& value);
};
class RecognitionExampleEntity : public Node { public:
    RecognitionExampleEntity():Node(Json(Json::Object{})) {}
    RecognitionExampleEntity& set_end(const uint64_t& value);
    RecognitionExampleEntity& set_kind(const std::string& value);
    RecognitionExampleEntity& set_start(const uint64_t& value);
};
class RecognitionExampleText : public Node { public:
    RecognitionExampleText():Node(Json(Json::Object{})) {}
    RecognitionExampleText& set_entities(const std::vector<RecognitionExampleEntity>& value);
    RecognitionExampleText& set_kinds(const std::vector<std::string>& value);
    RecognitionExampleText& set_text(const std::string& value);
};
class RecognitionMode : public Node { public:
    explicit RecognitionMode(const std::string& value);
};
class RecognitionSeedSpan : public Node { public:
    RecognitionSeedSpan():Node(Json(Json::Object{})) {}
    RecognitionSeedSpan& set_end(const uint64_t& value);
    RecognitionSeedSpan& set_kind(const std::string& value);
    RecognitionSeedSpan& set_start(const uint64_t& value);
};
class RecognitionStageContext : public Node { public:
    RecognitionStageContext():Node(Json(Json::Object{})) {}
    RecognitionStageContext& set_boundary(const std::string& value);
    RecognitionStageContext& set_kind_edge(const std::string& value);
    RecognitionStageContext& set_relation(const std::string& value);
};
class RequestBatch : public Node { public:
    explicit RequestBatch(const std::variant<uint64_t, std::string>& value);
};
class RequestDefinition : public Node { public:
    RequestDefinition(const RequestDefinitionFieldsDecide& value);
    RequestDefinition(const RequestDefinitionFieldsChoose& value);
    RequestDefinition(const RequestDefinitionFieldsTag& value);
    RequestDefinition(const RequestDefinitionFieldsScore& value);
    RequestDefinition(const RequestDefinitionFieldsRelateVersion& value);
    RequestDefinition(const RequestDefinitionFieldsFind& value);
    RequestDefinition(const RequestDefinitionFieldsRecognizeVersion& value);
    RequestDefinition(const RequestDefinitionFieldsQuestionsVersion& value);
};
class RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties : public Node { public:
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties(const RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties(const RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties(const RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties(const RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore& value);
};
class RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose : public Node { public:
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose():Node(Json(Json::Object{})) {}
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose& set_choose(const AuthoredQuestionText& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose& set_context_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose& set_item_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose& set_name(const std::string& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose& set_on(const AuthoredPointers& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose& set_options(const AuthoredOptions& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose& set_threshold(const AuthoredCut& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose& set_wording_version(const uint64_t& value);
};
class RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide : public Node { public:
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide():Node(Json(Json::Object{})) {}
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& set_context_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& set_decide(const AuthoredQuestionText& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& set_false(const AuthoredCriterion& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& set_item_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& set_name(const std::string& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& set_on(const AuthoredPointers& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& set_threshold(const AuthoredThreshold& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& set_true(const AuthoredCriterion& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& set_wording_version(const uint64_t& value);
};
class RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore : public Node { public:
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore():Node(Json(Json::Object{})) {}
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore& set_context_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore& set_item_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore& set_levels(const AuthoredLevels& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore& set_name(const std::string& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore& set_on(const AuthoredPointers& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore& set_score(const AuthoredQuestionText& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore& set_wording_version(const uint64_t& value);
};
class RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag : public Node { public:
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag():Node(Json(Json::Object{})) {}
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag& set_context_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag& set_item_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag& set_labels(const AuthoredLabels& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag& set_name(const std::string& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag& set_on(const AuthoredPointers& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag& set_tag(const AuthoredQuestionText& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag& set_threshold(const AuthoredCut& value);
    RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag& set_wording_version(const uint64_t& value);
};
class RequestDefinitionFieldsChoose : public Node { public:
    RequestDefinitionFieldsChoose():Node(Json(Json::Object{})) {}
    RequestDefinitionFieldsChoose& set_batch(const std::variant<std::string, uint64_t>& value);
    RequestDefinitionFieldsChoose& set_choose(const AuthoredQuestionText& value);
    RequestDefinitionFieldsChoose& set_context_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionFieldsChoose& set_item_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionFieldsChoose& set_model(const AuthoredName& value);
    RequestDefinitionFieldsChoose& set_name(const std::string& value);
    RequestDefinitionFieldsChoose& set_on(const AuthoredPointers& value);
    RequestDefinitionFieldsChoose& set_options(const AuthoredOptions& value);
    RequestDefinitionFieldsChoose& set_profile(const AuthoredProfile& value);
    RequestDefinitionFieldsChoose& set_threshold(const AuthoredCut& value);
    RequestDefinitionFieldsChoose& set_wording_version(const uint64_t& value);
};
class RequestDefinitionFieldsDecide : public Node { public:
    RequestDefinitionFieldsDecide():Node(Json(Json::Object{})) {}
    RequestDefinitionFieldsDecide& set_batch(const std::variant<std::string, uint64_t>& value);
    RequestDefinitionFieldsDecide& set_context_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionFieldsDecide& set_decide(const AuthoredQuestionText& value);
    RequestDefinitionFieldsDecide& set_false(const AuthoredCriterion& value);
    RequestDefinitionFieldsDecide& set_item_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionFieldsDecide& set_model(const AuthoredName& value);
    RequestDefinitionFieldsDecide& set_name(const std::string& value);
    RequestDefinitionFieldsDecide& set_on(const AuthoredPointers& value);
    RequestDefinitionFieldsDecide& set_profile(const AuthoredProfile& value);
    RequestDefinitionFieldsDecide& set_threshold(const AuthoredThreshold& value);
    RequestDefinitionFieldsDecide& set_true(const AuthoredCriterion& value);
    RequestDefinitionFieldsDecide& set_wording_version(const uint64_t& value);
};
class RequestDefinitionFieldsFind : public Node { public:
    RequestDefinitionFieldsFind():Node(Json(Json::Object{})) {}
    RequestDefinitionFieldsFind& set_context_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionFieldsFind& set_find(const AuthoredQuestionText& value);
    RequestDefinitionFieldsFind& set_item_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionFieldsFind& set_model(const AuthoredName& value);
    RequestDefinitionFieldsFind& set_name(const std::string& value);
    RequestDefinitionFieldsFind& set_on(const AuthoredPointers& value);
    RequestDefinitionFieldsFind& set_profile(const AuthoredProfile& value);
    RequestDefinitionFieldsFind& set_wording_version(const uint64_t& value);
};
class RequestDefinitionFieldsQuestionsVersion : public Node { public:
    RequestDefinitionFieldsQuestionsVersion():Node(Json(Json::Object{{"version",Json(1)}})) {}
    RequestDefinitionFieldsQuestionsVersion& set_batch(const Json& value);
    RequestDefinitionFieldsQuestionsVersion& set_profile(const AuthoredProfile& value);
    RequestDefinitionFieldsQuestionsVersion& set_questions(const std::vector<std::pair<std::string, RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties>>& value);
    RequestDefinitionFieldsQuestionsVersion& set_threshold(const AuthoredThreshold& value);
};
class RequestDefinitionFieldsRecognizeVersion : public Node { public:
    RequestDefinitionFieldsRecognizeVersion():Node(Json(Json::Object{{"version",Json(1)}})) {}
    RequestDefinitionFieldsRecognizeVersion& set_context_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionFieldsRecognizeVersion& set_item_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionFieldsRecognizeVersion& set_model(const AuthoredName& value);
    RequestDefinitionFieldsRecognizeVersion& set_name(const std::string& value);
    RequestDefinitionFieldsRecognizeVersion& set_on(const AuthoredPointers& value);
    RequestDefinitionFieldsRecognizeVersion& set_profile(const AuthoredProfile& value);
    RequestDefinitionFieldsRecognizeVersion& set_recognize(const Json& value);
    RequestDefinitionFieldsRecognizeVersion& set_relation_threshold(const AuthoredCut& value);
    RequestDefinitionFieldsRecognizeVersion& set_threshold(const AuthoredCut& value);
    RequestDefinitionFieldsRecognizeVersion& set_wording_version(const uint64_t& value);
};
class RequestDefinitionFieldsRelateVersion : public Node { public:
    RequestDefinitionFieldsRelateVersion():Node(Json(Json::Object{{"version",Json(1)}})) {}
    RequestDefinitionFieldsRelateVersion& set_context_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionFieldsRelateVersion& set_item_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionFieldsRelateVersion& set_model(const AuthoredName& value);
    RequestDefinitionFieldsRelateVersion& set_name(const std::string& value);
    RequestDefinitionFieldsRelateVersion& set_profile(const AuthoredProfile& value);
    RequestDefinitionFieldsRelateVersion& set_relate(const Json& value);
    RequestDefinitionFieldsRelateVersion& set_threshold(const AuthoredCut& value);
    RequestDefinitionFieldsRelateVersion& set_wording_version(const uint64_t& value);
};
class RequestDefinitionFieldsScore : public Node { public:
    RequestDefinitionFieldsScore():Node(Json(Json::Object{})) {}
    RequestDefinitionFieldsScore& set_batch(const std::variant<std::string, uint64_t>& value);
    RequestDefinitionFieldsScore& set_context_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionFieldsScore& set_item_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionFieldsScore& set_levels(const AuthoredLevels& value);
    RequestDefinitionFieldsScore& set_model(const AuthoredName& value);
    RequestDefinitionFieldsScore& set_name(const std::string& value);
    RequestDefinitionFieldsScore& set_on(const AuthoredPointers& value);
    RequestDefinitionFieldsScore& set_profile(const AuthoredProfile& value);
    RequestDefinitionFieldsScore& set_score(const AuthoredQuestionText& value);
    RequestDefinitionFieldsScore& set_wording_version(const uint64_t& value);
};
class RequestDefinitionFieldsTag : public Node { public:
    RequestDefinitionFieldsTag():Node(Json(Json::Object{})) {}
    RequestDefinitionFieldsTag& set_batch(const std::variant<std::string, uint64_t>& value);
    RequestDefinitionFieldsTag& set_context_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionFieldsTag& set_item_schema(const AuthoredInputDeclaration& value);
    RequestDefinitionFieldsTag& set_labels(const AuthoredLabels& value);
    RequestDefinitionFieldsTag& set_model(const AuthoredName& value);
    RequestDefinitionFieldsTag& set_name(const std::string& value);
    RequestDefinitionFieldsTag& set_on(const AuthoredPointers& value);
    RequestDefinitionFieldsTag& set_profile(const AuthoredProfile& value);
    RequestDefinitionFieldsTag& set_tag(const AuthoredQuestionText& value);
    RequestDefinitionFieldsTag& set_threshold(const AuthoredCut& value);
    RequestDefinitionFieldsTag& set_wording_version(const uint64_t& value);
};
class RequestFraming : public Node { public:
    explicit RequestFraming(const std::string& value);
};
class RequestImage : public Node { public:
    RequestImage(const RequestImageFile& value);
    RequestImage(const RequestImageBytes& value);
};
class RequestImageBytes : public Node { public:
    RequestImageBytes():Node(Json(Json::Object{{"kind",Json("bytes")}})) {}
    RequestImageBytes& set_bytes(const std::string& value);
    RequestImageBytes& set_media(const ImageMedia& value);
};
class RequestImageFile : public Node { public:
    RequestImageFile():Node(Json(Json::Object{{"kind",Json("file")}})) {}
    RequestImageFile& set_media(const ImageMedia& value);
    RequestImageFile& set_path(const std::string& value);
};
class RequestInput : public Node { public:
    RequestInput(const RequestInputText& value);
    RequestInput(const RequestInputJson& value);
    RequestInput(const RequestInputRecords& value);
    RequestInput(const RequestInputUnits& value);
    RequestInput(const RequestInputEntities& value);
    RequestInput(const RequestInputSource& value);
    RequestInput(const RequestInputFeed& value);
};
class RequestInputEntities : public Node { public:
    RequestInputEntities():Node(Json(Json::Object{{"kind",Json("entities")}})) {}
    RequestInputEntities& set_items(const std::vector<RequestItem>& value);
};
class RequestInputFeed : public Node { public:
    RequestInputFeed():Node(Json(Json::Object{{"kind",Json("feed")}})) {}
    RequestInputFeed& set_framing(const RequestFraming& value);
    RequestInputFeed& set_images(const std::vector<RequestImage>& value);
    RequestInputFeed& set_name(const std::string& value);
    RequestInputFeed& set_reading(const RequestReader& value);
};
class RequestInputJson : public Node { public:
    RequestInputJson():Node(Json(Json::Object{{"kind",Json("json")}})) {}
    RequestInputJson& set_images(const std::vector<RequestImage>& value);
    RequestInputJson& set_value(const Json& value);
};
class RequestInputRecords : public Node { public:
    RequestInputRecords():Node(Json(Json::Object{{"kind",Json("records")}})) {}
    RequestInputRecords& set_items(const std::vector<RequestItem>& value);
};
class RequestInputSource : public Node { public:
    RequestInputSource():Node(Json(Json::Object{{"kind",Json("source")}})) {}
    RequestInputSource& set_source(const RequestSource& value);
};
class RequestInputText : public Node { public:
    RequestInputText():Node(Json(Json::Object{{"kind",Json("text")}})) {}
    RequestInputText& set_images(const std::vector<RequestImage>& value);
    RequestInputText& set_text(const std::string& value);
};
class RequestInputUnits : public Node { public:
    RequestInputUnits():Node(Json(Json::Object{{"kind",Json("units")}})) {}
    RequestInputUnits& set_items(const std::vector<RequestItem>& value);
};
class RequestItem : public Node { public:
    RequestItem():Node(Json(Json::Object{})) {}
    RequestItem& set_context(const ContextSchema& value);
    RequestItem& set_examples(const std::vector<RecognitionExample>& value);
    RequestItem& set_images(const std::vector<RequestImage>& value);
    RequestItem& set_options(const std::vector<OptionSchema>& value);
    RequestItem& set_original(const RequestOriginal& value);
    RequestItem& set_seed_spans(const std::vector<RecognitionSeedSpan>& value);
};
class RequestOptions : public Node { public:
    RequestOptions():Node(Json(Json::Object{})) {}
    RequestOptions& set_attempts(const bool& value);
    RequestOptions& set_batch(const RequestBatch& value);
    RequestOptions& set_context(const std::string& value);
    RequestOptions& set_context_field(const std::string& value);
    RequestOptions& set_deadline_ms(const int64_t& value);
    RequestOptions& set_details(const bool& value);
    RequestOptions& set_examples(const std::vector<RecognitionExample>& value);
    RequestOptions& set_examples_field(const std::string& value);
    RequestOptions& set_field(const std::vector<std::string>& value);
    RequestOptions& set_files_only(const bool& value);
    RequestOptions& set_max_requests_total(const uint64_t& value);
    RequestOptions& set_mode(const RecognitionMode& value);
    RequestOptions& set_model(const std::string& value);
    RequestOptions& set_none(const bool& value);
    RequestOptions& set_options_field(const std::string& value);
    RequestOptions& set_relation_threshold(const RequestThreshold& value);
    RequestOptions& set_seed_spans(const std::vector<RecognitionSeedSpan>& value);
    RequestOptions& set_seed_spans_field(const std::string& value);
    RequestOptions& set_snippet_pieces(const uint32_t& value);
    RequestOptions& set_stage_context(const RecognitionStageContext& value);
    RequestOptions& set_threshold(const RequestThreshold& value);
    RequestOptions& set_top(const uint64_t& value);
};
class RequestOriginal : public Node { public:
    RequestOriginal(const RequestOriginalText& value);
    RequestOriginal(const RequestOriginalJson& value);
};
class RequestOriginalJson : public Node { public:
    RequestOriginalJson():Node(Json(Json::Object{{"kind",Json("json")}})) {}
    RequestOriginalJson& set_value(const Json& value);
};
class RequestOriginalText : public Node { public:
    RequestOriginalText():Node(Json(Json::Object{{"kind",Json("text")}})) {}
    RequestOriginalText& set_text(const std::string& value);
};
class RequestQuestion : public Node { public:
    RequestQuestion(const RequestQuestionText& value);
    RequestQuestion(const RequestQuestionDefinition& value);
    RequestQuestion(const RequestQuestionFile& value);
    RequestQuestion(const RequestQuestionName& value);
    RequestQuestion(const RequestQuestionReference& value);
};
class RequestQuestionDefinition : public Node { public:
    RequestQuestionDefinition():Node(Json(Json::Object{{"kind",Json("definition")}})) {}
    RequestQuestionDefinition& set_value(const RequestDefinition& value);
};
class RequestQuestionFile : public Node { public:
    RequestQuestionFile():Node(Json(Json::Object{{"kind",Json("file")}})) {}
    RequestQuestionFile& set_path(const std::string& value);
};
class RequestQuestionName : public Node { public:
    RequestQuestionName():Node(Json(Json::Object{{"kind",Json("name")}})) {}
    RequestQuestionName& set_name(const std::string& value);
};
class RequestQuestionReference : public Node { public:
    RequestQuestionReference():Node(Json(Json::Object{{"kind",Json("reference")}})) {}
    RequestQuestionReference& set_reference(const std::string& value);
};
class RequestQuestionText : public Node { public:
    RequestQuestionText():Node(Json(Json::Object{{"kind",Json("text")}})) {}
    RequestQuestionText& set_text(const std::string& value);
};
class RequestReader : public Node { public:
    RequestReader():Node(Json(Json::Object{})) {}
    RequestReader& set_unit(const SourceUnit& value);
    RequestReader& set_window(const uint64_t& value);
};
class RequestSessionDescriptor : public Node { public:
    RequestSessionDescriptor():Node(Json(Json::Object{})) {}
    RequestSessionDescriptor& set_item(const RequestItem& value);
    RequestSessionDescriptor& set_location(const SessionSourceLocation& value);
};
class RequestSource : public Node { public:
    RequestSource():Node(Json(Json::Object{})) {}
    RequestSource& set_media(const ReaderMedia& value);
    RequestSource& set_paths(const std::vector<std::string>& value);
    RequestSource& set_reading(const RequestReader& value);
};
class RequestThreshold : public Node { public:
    explicit RequestThreshold(const std::variant<double, std::string>& value);
};
class SessionSourceLocation : public Node { public:
    SessionSourceLocation():Node(Json(Json::Object{})) {}
    SessionSourceLocation& set_file(const std::string& value);
    SessionSourceLocation& set_first_line(const uint64_t& value);
    SessionSourceLocation& set_last_line(const uint64_t& value);
};
class SourceUnit : public Node { public:
    explicit SourceUnit(const std::string& value);
};
inline AuthoredChoose& AuthoredChoose::set_batch(const std::variant<std::string, uint64_t>& value) { set_member("batch",results::encode(value)); return *this; }
inline AuthoredChoose& AuthoredChoose::set_choose(const AuthoredQuestionText& value) { set_member("choose",results::encode(value)); return *this; }
inline AuthoredChoose& AuthoredChoose::set_context_schema(const AuthoredInputDeclaration& value) { set_member("context_schema",results::encode(value)); return *this; }
inline AuthoredChoose& AuthoredChoose::set_item_schema(const AuthoredInputDeclaration& value) { set_member("item_schema",results::encode(value)); return *this; }
inline AuthoredChoose& AuthoredChoose::set_model(const AuthoredName& value) { set_member("model",results::encode(value)); return *this; }
inline AuthoredChoose& AuthoredChoose::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline AuthoredChoose& AuthoredChoose::set_on(const AuthoredPointers& value) { set_member("on",results::encode(value)); return *this; }
inline AuthoredChoose& AuthoredChoose::set_options(const AuthoredOptions& value) { set_member("options",results::encode(value)); return *this; }
inline AuthoredChoose& AuthoredChoose::set_profile(const AuthoredProfile& value) { set_member("profile",results::encode(value)); return *this; }
inline AuthoredChoose& AuthoredChoose::set_threshold(const AuthoredCut& value) { set_member("threshold",results::encode(value)); return *this; }
inline AuthoredChoose& AuthoredChoose::set_wording_version(const uint64_t& value) { set_member("wording_version",results::encode(value)); return *this; }
inline AuthoredCriterion::AuthoredCriterion(const std::variant<std::string, Json, std::vector<Json>, std::nullptr_t>& value):Node(results::encode(value)) {}
inline AuthoredCut::AuthoredCut(const std::variant<double, std::string>& value):Node(results::encode(value)) {}
inline AuthoredDecide& AuthoredDecide::set_batch(const std::variant<std::string, uint64_t>& value) { set_member("batch",results::encode(value)); return *this; }
inline AuthoredDecide& AuthoredDecide::set_context_schema(const AuthoredInputDeclaration& value) { set_member("context_schema",results::encode(value)); return *this; }
inline AuthoredDecide& AuthoredDecide::set_decide(const AuthoredQuestionText& value) { set_member("decide",results::encode(value)); return *this; }
inline AuthoredDecide& AuthoredDecide::set_false(const AuthoredCriterion& value) { set_member("false",results::encode(value)); return *this; }
inline AuthoredDecide& AuthoredDecide::set_item_schema(const AuthoredInputDeclaration& value) { set_member("item_schema",results::encode(value)); return *this; }
inline AuthoredDecide& AuthoredDecide::set_model(const AuthoredName& value) { set_member("model",results::encode(value)); return *this; }
inline AuthoredDecide& AuthoredDecide::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline AuthoredDecide& AuthoredDecide::set_on(const AuthoredPointers& value) { set_member("on",results::encode(value)); return *this; }
inline AuthoredDecide& AuthoredDecide::set_profile(const AuthoredProfile& value) { set_member("profile",results::encode(value)); return *this; }
inline AuthoredDecide& AuthoredDecide::set_threshold(const AuthoredThreshold& value) { set_member("threshold",results::encode(value)); return *this; }
inline AuthoredDecide& AuthoredDecide::set_true(const AuthoredCriterion& value) { set_member("true",results::encode(value)); return *this; }
inline AuthoredDecide& AuthoredDecide::set_wording_version(const uint64_t& value) { set_member("wording_version",results::encode(value)); return *this; }
inline AuthoredDescription::AuthoredDescription(const std::variant<std::string, Json, std::vector<Json>, std::nullptr_t>& value):Node(results::encode(value)) {}
inline AuthoredFind& AuthoredFind::set_context_schema(const AuthoredInputDeclaration& value) { set_member("context_schema",results::encode(value)); return *this; }
inline AuthoredFind& AuthoredFind::set_find(const AuthoredQuestionText& value) { set_member("find",results::encode(value)); return *this; }
inline AuthoredFind& AuthoredFind::set_item_schema(const AuthoredInputDeclaration& value) { set_member("item_schema",results::encode(value)); return *this; }
inline AuthoredFind& AuthoredFind::set_model(const AuthoredName& value) { set_member("model",results::encode(value)); return *this; }
inline AuthoredFind& AuthoredFind::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline AuthoredFind& AuthoredFind::set_on(const AuthoredPointers& value) { set_member("on",results::encode(value)); return *this; }
inline AuthoredFind& AuthoredFind::set_profile(const AuthoredProfile& value) { set_member("profile",results::encode(value)); return *this; }
inline AuthoredFind& AuthoredFind::set_wording_version(const uint64_t& value) { set_member("wording_version",results::encode(value)); return *this; }
inline AuthoredInputDeclaration::AuthoredInputDeclaration(const AuthoredInputDeclarationString& value):Node(value.document()) {}
inline AuthoredInputDeclaration::AuthoredInputDeclaration(const AuthoredInputDeclarationObject& value):Node(value.document()) {}
inline AuthoredInputDeclarationObject& AuthoredInputDeclarationObject::set_properties(const std::vector<std::pair<std::string, AuthoredInputProperty>>& value) { set_member("properties",results::encode(value)); return *this; }
inline AuthoredInputDeclarationObject& AuthoredInputDeclarationObject::set_required(const std::vector<std::string>& value) { set_member("required",results::encode(value)); return *this; }
inline AuthoredInputProperty::AuthoredInputProperty(const AuthoredInputPropertyString& value):Node(value.document()) {}
inline AuthoredInputProperty::AuthoredInputProperty(const AuthoredInputPropertyNumber& value):Node(value.document()) {}
inline AuthoredInputProperty::AuthoredInputProperty(const AuthoredInputPropertyBoolean& value):Node(value.document()) {}
inline AuthoredInputProperty::AuthoredInputProperty(const AuthoredInputPropertyArray& value):Node(value.document()) {}
inline AuthoredInputPropertyArray& AuthoredInputPropertyArray::set_items(const Json& value) { set_member("items",results::encode(value)); return *this; }
inline AuthoredLabels::AuthoredLabels(const std::variant<std::vector<AuthoredName>, std::vector<std::pair<std::string, AuthoredDescription>>>& value):Node(results::encode(value)) {}
inline AuthoredLevels::AuthoredLevels(const std::variant<std::vector<AuthoredName>, std::vector<std::pair<std::string, AuthoredCriterion>>>& value):Node(results::encode(value)) {}
inline AuthoredName::AuthoredName(const std::string& value):Node(results::encode(value)) {}
inline AuthoredOptions::AuthoredOptions(const std::variant<std::vector<AuthoredName>, std::vector<std::pair<std::string, AuthoredDescription>>>& value):Node(results::encode(value)) {}
inline AuthoredPointers::AuthoredPointers(const std::variant<std::string, std::vector<std::string>>& value):Node(results::encode(value)) {}
inline AuthoredProfile::AuthoredProfile(const std::string& value):Node(results::encode(value)) {}
inline AuthoredQuestionText::AuthoredQuestionText(const std::variant<std::string, Json, std::vector<Json>>& value):Node(results::encode(value)) {}
inline AuthoredRelate& AuthoredRelate::set_context_schema(const AuthoredInputDeclaration& value) { set_member("context_schema",results::encode(value)); return *this; }
inline AuthoredRelate& AuthoredRelate::set_item_schema(const AuthoredInputDeclaration& value) { set_member("item_schema",results::encode(value)); return *this; }
inline AuthoredRelate& AuthoredRelate::set_model(const AuthoredName& value) { set_member("model",results::encode(value)); return *this; }
inline AuthoredRelate& AuthoredRelate::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline AuthoredRelate& AuthoredRelate::set_profile(const AuthoredProfile& value) { set_member("profile",results::encode(value)); return *this; }
inline AuthoredRelate& AuthoredRelate::set_relate(const Json& value) { set_member("relate",results::encode(value)); return *this; }
inline AuthoredRelate& AuthoredRelate::set_threshold(const AuthoredCut& value) { set_member("threshold",results::encode(value)); return *this; }
inline AuthoredRelate& AuthoredRelate::set_wording_version(const uint64_t& value) { set_member("wording_version",results::encode(value)); return *this; }
inline AuthoredRelation& AuthoredRelation::set_either(const bool& value) { set_member("either",results::encode(value)); return *this; }
inline AuthoredRelation& AuthoredRelation::set_name(const AuthoredName& value) { set_member("name",results::encode(value)); return *this; }
inline AuthoredRelation& AuthoredRelation::set_reads(const AuthoredName& value) { set_member("reads",results::encode(value)); return *this; }
inline AuthoredRelation& AuthoredRelation::set_single(const bool& value) { set_member("single",results::encode(value)); return *this; }
inline AuthoredRelation& AuthoredRelation::set_source(const AuthoredName& value) { set_member("source",results::encode(value)); return *this; }
inline AuthoredRelation& AuthoredRelation::set_target(const AuthoredName& value) { set_member("target",results::encode(value)); return *this; }
inline AuthoredScore& AuthoredScore::set_batch(const std::variant<std::string, uint64_t>& value) { set_member("batch",results::encode(value)); return *this; }
inline AuthoredScore& AuthoredScore::set_context_schema(const AuthoredInputDeclaration& value) { set_member("context_schema",results::encode(value)); return *this; }
inline AuthoredScore& AuthoredScore::set_item_schema(const AuthoredInputDeclaration& value) { set_member("item_schema",results::encode(value)); return *this; }
inline AuthoredScore& AuthoredScore::set_levels(const AuthoredLevels& value) { set_member("levels",results::encode(value)); return *this; }
inline AuthoredScore& AuthoredScore::set_model(const AuthoredName& value) { set_member("model",results::encode(value)); return *this; }
inline AuthoredScore& AuthoredScore::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline AuthoredScore& AuthoredScore::set_on(const AuthoredPointers& value) { set_member("on",results::encode(value)); return *this; }
inline AuthoredScore& AuthoredScore::set_profile(const AuthoredProfile& value) { set_member("profile",results::encode(value)); return *this; }
inline AuthoredScore& AuthoredScore::set_score(const AuthoredQuestionText& value) { set_member("score",results::encode(value)); return *this; }
inline AuthoredScore& AuthoredScore::set_wording_version(const uint64_t& value) { set_member("wording_version",results::encode(value)); return *this; }
inline AuthoredTag& AuthoredTag::set_batch(const std::variant<std::string, uint64_t>& value) { set_member("batch",results::encode(value)); return *this; }
inline AuthoredTag& AuthoredTag::set_context_schema(const AuthoredInputDeclaration& value) { set_member("context_schema",results::encode(value)); return *this; }
inline AuthoredTag& AuthoredTag::set_item_schema(const AuthoredInputDeclaration& value) { set_member("item_schema",results::encode(value)); return *this; }
inline AuthoredTag& AuthoredTag::set_labels(const AuthoredLabels& value) { set_member("labels",results::encode(value)); return *this; }
inline AuthoredTag& AuthoredTag::set_model(const AuthoredName& value) { set_member("model",results::encode(value)); return *this; }
inline AuthoredTag& AuthoredTag::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline AuthoredTag& AuthoredTag::set_on(const AuthoredPointers& value) { set_member("on",results::encode(value)); return *this; }
inline AuthoredTag& AuthoredTag::set_profile(const AuthoredProfile& value) { set_member("profile",results::encode(value)); return *this; }
inline AuthoredTag& AuthoredTag::set_tag(const AuthoredQuestionText& value) { set_member("tag",results::encode(value)); return *this; }
inline AuthoredTag& AuthoredTag::set_threshold(const AuthoredCut& value) { set_member("threshold",results::encode(value)); return *this; }
inline AuthoredTag& AuthoredTag::set_wording_version(const uint64_t& value) { set_member("wording_version",results::encode(value)); return *this; }
inline AuthoredThreshold::AuthoredThreshold(const std::variant<double, std::string>& value):Node(results::encode(value)) {}
inline ContextSchema::ContextSchema(const std::variant<std::string, Json>& value):Node(results::encode(value)) {}
inline ImageMedia::ImageMedia(const std::string& value):Node(results::encode(value)) {}
inline OptionSchema& OptionSchema::set_description(const Json& value) { set_member("description",results::encode(value)); return *this; }
inline OptionSchema& OptionSchema::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline ReaderMedia::ReaderMedia(const std::string& value):Node(results::encode(value)) {}
inline RecognitionExample::RecognitionExample(const std::variant<std::string, RecognitionExampleText>& value):Node(results::encode(value)) {}
inline RecognitionExampleEntity& RecognitionExampleEntity::set_end(const uint64_t& value) { set_member("end",results::encode(value)); return *this; }
inline RecognitionExampleEntity& RecognitionExampleEntity::set_kind(const std::string& value) { set_member("kind",results::encode(value)); return *this; }
inline RecognitionExampleEntity& RecognitionExampleEntity::set_start(const uint64_t& value) { set_member("start",results::encode(value)); return *this; }
inline RecognitionExampleText& RecognitionExampleText::set_entities(const std::vector<RecognitionExampleEntity>& value) { set_member("entities",results::encode(value)); return *this; }
inline RecognitionExampleText& RecognitionExampleText::set_kinds(const std::vector<std::string>& value) { set_member("kinds",results::encode(value)); return *this; }
inline RecognitionExampleText& RecognitionExampleText::set_text(const std::string& value) { set_member("text",results::encode(value)); return *this; }
inline RecognitionMode::RecognitionMode(const std::string& value):Node(results::encode(value)) {}
inline RecognitionSeedSpan& RecognitionSeedSpan::set_end(const uint64_t& value) { set_member("end",results::encode(value)); return *this; }
inline RecognitionSeedSpan& RecognitionSeedSpan::set_kind(const std::string& value) { set_member("kind",results::encode(value)); return *this; }
inline RecognitionSeedSpan& RecognitionSeedSpan::set_start(const uint64_t& value) { set_member("start",results::encode(value)); return *this; }
inline RecognitionStageContext& RecognitionStageContext::set_boundary(const std::string& value) { set_member("boundary",results::encode(value)); return *this; }
inline RecognitionStageContext& RecognitionStageContext::set_kind_edge(const std::string& value) { set_member("kind_edge",results::encode(value)); return *this; }
inline RecognitionStageContext& RecognitionStageContext::set_relation(const std::string& value) { set_member("relation",results::encode(value)); return *this; }
inline RequestBatch::RequestBatch(const std::variant<uint64_t, std::string>& value):Node(results::encode(value)) {}
inline RequestDefinition::RequestDefinition(const RequestDefinitionFieldsDecide& value):Node(value.document()) {}
inline RequestDefinition::RequestDefinition(const RequestDefinitionFieldsChoose& value):Node(value.document()) {}
inline RequestDefinition::RequestDefinition(const RequestDefinitionFieldsTag& value):Node(value.document()) {}
inline RequestDefinition::RequestDefinition(const RequestDefinitionFieldsScore& value):Node(value.document()) {}
inline RequestDefinition::RequestDefinition(const RequestDefinitionFieldsRelateVersion& value):Node(value.document()) {}
inline RequestDefinition::RequestDefinition(const RequestDefinitionFieldsFind& value):Node(value.document()) {}
inline RequestDefinition::RequestDefinition(const RequestDefinitionFieldsRecognizeVersion& value):Node(value.document()) {}
inline RequestDefinition::RequestDefinition(const RequestDefinitionFieldsQuestionsVersion& value):Node(value.document()) {}
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties::RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties(const RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& value):Node(value.document()) {}
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties::RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties(const RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose& value):Node(value.document()) {}
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties::RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties(const RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag& value):Node(value.document()) {}
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties::RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties(const RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore& value):Node(value.document()) {}
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose::set_choose(const AuthoredQuestionText& value) { set_member("choose",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose::set_context_schema(const AuthoredInputDeclaration& value) { set_member("context_schema",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose::set_item_schema(const AuthoredInputDeclaration& value) { set_member("item_schema",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose::set_on(const AuthoredPointers& value) { set_member("on",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose::set_options(const AuthoredOptions& value) { set_member("options",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose::set_threshold(const AuthoredCut& value) { set_member("threshold",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose::set_wording_version(const uint64_t& value) { set_member("wording_version",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide::set_context_schema(const AuthoredInputDeclaration& value) { set_member("context_schema",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide::set_decide(const AuthoredQuestionText& value) { set_member("decide",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide::set_false(const AuthoredCriterion& value) { set_member("false",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide::set_item_schema(const AuthoredInputDeclaration& value) { set_member("item_schema",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide::set_on(const AuthoredPointers& value) { set_member("on",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide::set_threshold(const AuthoredThreshold& value) { set_member("threshold",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide::set_true(const AuthoredCriterion& value) { set_member("true",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide::set_wording_version(const uint64_t& value) { set_member("wording_version",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore::set_context_schema(const AuthoredInputDeclaration& value) { set_member("context_schema",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore::set_item_schema(const AuthoredInputDeclaration& value) { set_member("item_schema",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore::set_levels(const AuthoredLevels& value) { set_member("levels",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore::set_on(const AuthoredPointers& value) { set_member("on",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore::set_score(const AuthoredQuestionText& value) { set_member("score",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore::set_wording_version(const uint64_t& value) { set_member("wording_version",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag::set_context_schema(const AuthoredInputDeclaration& value) { set_member("context_schema",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag::set_item_schema(const AuthoredInputDeclaration& value) { set_member("item_schema",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag::set_labels(const AuthoredLabels& value) { set_member("labels",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag::set_on(const AuthoredPointers& value) { set_member("on",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag::set_tag(const AuthoredQuestionText& value) { set_member("tag",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag::set_threshold(const AuthoredCut& value) { set_member("threshold",results::encode(value)); return *this; }
inline RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag& RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag::set_wording_version(const uint64_t& value) { set_member("wording_version",results::encode(value)); return *this; }
inline RequestDefinitionFieldsChoose& RequestDefinitionFieldsChoose::set_batch(const std::variant<std::string, uint64_t>& value) { set_member("batch",results::encode(value)); return *this; }
inline RequestDefinitionFieldsChoose& RequestDefinitionFieldsChoose::set_choose(const AuthoredQuestionText& value) { set_member("choose",results::encode(value)); return *this; }
inline RequestDefinitionFieldsChoose& RequestDefinitionFieldsChoose::set_context_schema(const AuthoredInputDeclaration& value) { set_member("context_schema",results::encode(value)); return *this; }
inline RequestDefinitionFieldsChoose& RequestDefinitionFieldsChoose::set_item_schema(const AuthoredInputDeclaration& value) { set_member("item_schema",results::encode(value)); return *this; }
inline RequestDefinitionFieldsChoose& RequestDefinitionFieldsChoose::set_model(const AuthoredName& value) { set_member("model",results::encode(value)); return *this; }
inline RequestDefinitionFieldsChoose& RequestDefinitionFieldsChoose::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline RequestDefinitionFieldsChoose& RequestDefinitionFieldsChoose::set_on(const AuthoredPointers& value) { set_member("on",results::encode(value)); return *this; }
inline RequestDefinitionFieldsChoose& RequestDefinitionFieldsChoose::set_options(const AuthoredOptions& value) { set_member("options",results::encode(value)); return *this; }
inline RequestDefinitionFieldsChoose& RequestDefinitionFieldsChoose::set_profile(const AuthoredProfile& value) { set_member("profile",results::encode(value)); return *this; }
inline RequestDefinitionFieldsChoose& RequestDefinitionFieldsChoose::set_threshold(const AuthoredCut& value) { set_member("threshold",results::encode(value)); return *this; }
inline RequestDefinitionFieldsChoose& RequestDefinitionFieldsChoose::set_wording_version(const uint64_t& value) { set_member("wording_version",results::encode(value)); return *this; }
inline RequestDefinitionFieldsDecide& RequestDefinitionFieldsDecide::set_batch(const std::variant<std::string, uint64_t>& value) { set_member("batch",results::encode(value)); return *this; }
inline RequestDefinitionFieldsDecide& RequestDefinitionFieldsDecide::set_context_schema(const AuthoredInputDeclaration& value) { set_member("context_schema",results::encode(value)); return *this; }
inline RequestDefinitionFieldsDecide& RequestDefinitionFieldsDecide::set_decide(const AuthoredQuestionText& value) { set_member("decide",results::encode(value)); return *this; }
inline RequestDefinitionFieldsDecide& RequestDefinitionFieldsDecide::set_false(const AuthoredCriterion& value) { set_member("false",results::encode(value)); return *this; }
inline RequestDefinitionFieldsDecide& RequestDefinitionFieldsDecide::set_item_schema(const AuthoredInputDeclaration& value) { set_member("item_schema",results::encode(value)); return *this; }
inline RequestDefinitionFieldsDecide& RequestDefinitionFieldsDecide::set_model(const AuthoredName& value) { set_member("model",results::encode(value)); return *this; }
inline RequestDefinitionFieldsDecide& RequestDefinitionFieldsDecide::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline RequestDefinitionFieldsDecide& RequestDefinitionFieldsDecide::set_on(const AuthoredPointers& value) { set_member("on",results::encode(value)); return *this; }
inline RequestDefinitionFieldsDecide& RequestDefinitionFieldsDecide::set_profile(const AuthoredProfile& value) { set_member("profile",results::encode(value)); return *this; }
inline RequestDefinitionFieldsDecide& RequestDefinitionFieldsDecide::set_threshold(const AuthoredThreshold& value) { set_member("threshold",results::encode(value)); return *this; }
inline RequestDefinitionFieldsDecide& RequestDefinitionFieldsDecide::set_true(const AuthoredCriterion& value) { set_member("true",results::encode(value)); return *this; }
inline RequestDefinitionFieldsDecide& RequestDefinitionFieldsDecide::set_wording_version(const uint64_t& value) { set_member("wording_version",results::encode(value)); return *this; }
inline RequestDefinitionFieldsFind& RequestDefinitionFieldsFind::set_context_schema(const AuthoredInputDeclaration& value) { set_member("context_schema",results::encode(value)); return *this; }
inline RequestDefinitionFieldsFind& RequestDefinitionFieldsFind::set_find(const AuthoredQuestionText& value) { set_member("find",results::encode(value)); return *this; }
inline RequestDefinitionFieldsFind& RequestDefinitionFieldsFind::set_item_schema(const AuthoredInputDeclaration& value) { set_member("item_schema",results::encode(value)); return *this; }
inline RequestDefinitionFieldsFind& RequestDefinitionFieldsFind::set_model(const AuthoredName& value) { set_member("model",results::encode(value)); return *this; }
inline RequestDefinitionFieldsFind& RequestDefinitionFieldsFind::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline RequestDefinitionFieldsFind& RequestDefinitionFieldsFind::set_on(const AuthoredPointers& value) { set_member("on",results::encode(value)); return *this; }
inline RequestDefinitionFieldsFind& RequestDefinitionFieldsFind::set_profile(const AuthoredProfile& value) { set_member("profile",results::encode(value)); return *this; }
inline RequestDefinitionFieldsFind& RequestDefinitionFieldsFind::set_wording_version(const uint64_t& value) { set_member("wording_version",results::encode(value)); return *this; }
inline RequestDefinitionFieldsQuestionsVersion& RequestDefinitionFieldsQuestionsVersion::set_batch(const Json& value) { set_member("batch",results::encode(value)); return *this; }
inline RequestDefinitionFieldsQuestionsVersion& RequestDefinitionFieldsQuestionsVersion::set_profile(const AuthoredProfile& value) { set_member("profile",results::encode(value)); return *this; }
inline RequestDefinitionFieldsQuestionsVersion& RequestDefinitionFieldsQuestionsVersion::set_questions(const std::vector<std::pair<std::string, RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties>>& value) { set_member("questions",results::encode(value)); return *this; }
inline RequestDefinitionFieldsQuestionsVersion& RequestDefinitionFieldsQuestionsVersion::set_threshold(const AuthoredThreshold& value) { set_member("threshold",results::encode(value)); return *this; }
inline RequestDefinitionFieldsRecognizeVersion& RequestDefinitionFieldsRecognizeVersion::set_context_schema(const AuthoredInputDeclaration& value) { set_member("context_schema",results::encode(value)); return *this; }
inline RequestDefinitionFieldsRecognizeVersion& RequestDefinitionFieldsRecognizeVersion::set_item_schema(const AuthoredInputDeclaration& value) { set_member("item_schema",results::encode(value)); return *this; }
inline RequestDefinitionFieldsRecognizeVersion& RequestDefinitionFieldsRecognizeVersion::set_model(const AuthoredName& value) { set_member("model",results::encode(value)); return *this; }
inline RequestDefinitionFieldsRecognizeVersion& RequestDefinitionFieldsRecognizeVersion::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline RequestDefinitionFieldsRecognizeVersion& RequestDefinitionFieldsRecognizeVersion::set_on(const AuthoredPointers& value) { set_member("on",results::encode(value)); return *this; }
inline RequestDefinitionFieldsRecognizeVersion& RequestDefinitionFieldsRecognizeVersion::set_profile(const AuthoredProfile& value) { set_member("profile",results::encode(value)); return *this; }
inline RequestDefinitionFieldsRecognizeVersion& RequestDefinitionFieldsRecognizeVersion::set_recognize(const Json& value) { set_member("recognize",results::encode(value)); return *this; }
inline RequestDefinitionFieldsRecognizeVersion& RequestDefinitionFieldsRecognizeVersion::set_relation_threshold(const AuthoredCut& value) { set_member("relation_threshold",results::encode(value)); return *this; }
inline RequestDefinitionFieldsRecognizeVersion& RequestDefinitionFieldsRecognizeVersion::set_threshold(const AuthoredCut& value) { set_member("threshold",results::encode(value)); return *this; }
inline RequestDefinitionFieldsRecognizeVersion& RequestDefinitionFieldsRecognizeVersion::set_wording_version(const uint64_t& value) { set_member("wording_version",results::encode(value)); return *this; }
inline RequestDefinitionFieldsRelateVersion& RequestDefinitionFieldsRelateVersion::set_context_schema(const AuthoredInputDeclaration& value) { set_member("context_schema",results::encode(value)); return *this; }
inline RequestDefinitionFieldsRelateVersion& RequestDefinitionFieldsRelateVersion::set_item_schema(const AuthoredInputDeclaration& value) { set_member("item_schema",results::encode(value)); return *this; }
inline RequestDefinitionFieldsRelateVersion& RequestDefinitionFieldsRelateVersion::set_model(const AuthoredName& value) { set_member("model",results::encode(value)); return *this; }
inline RequestDefinitionFieldsRelateVersion& RequestDefinitionFieldsRelateVersion::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline RequestDefinitionFieldsRelateVersion& RequestDefinitionFieldsRelateVersion::set_profile(const AuthoredProfile& value) { set_member("profile",results::encode(value)); return *this; }
inline RequestDefinitionFieldsRelateVersion& RequestDefinitionFieldsRelateVersion::set_relate(const Json& value) { set_member("relate",results::encode(value)); return *this; }
inline RequestDefinitionFieldsRelateVersion& RequestDefinitionFieldsRelateVersion::set_threshold(const AuthoredCut& value) { set_member("threshold",results::encode(value)); return *this; }
inline RequestDefinitionFieldsRelateVersion& RequestDefinitionFieldsRelateVersion::set_wording_version(const uint64_t& value) { set_member("wording_version",results::encode(value)); return *this; }
inline RequestDefinitionFieldsScore& RequestDefinitionFieldsScore::set_batch(const std::variant<std::string, uint64_t>& value) { set_member("batch",results::encode(value)); return *this; }
inline RequestDefinitionFieldsScore& RequestDefinitionFieldsScore::set_context_schema(const AuthoredInputDeclaration& value) { set_member("context_schema",results::encode(value)); return *this; }
inline RequestDefinitionFieldsScore& RequestDefinitionFieldsScore::set_item_schema(const AuthoredInputDeclaration& value) { set_member("item_schema",results::encode(value)); return *this; }
inline RequestDefinitionFieldsScore& RequestDefinitionFieldsScore::set_levels(const AuthoredLevels& value) { set_member("levels",results::encode(value)); return *this; }
inline RequestDefinitionFieldsScore& RequestDefinitionFieldsScore::set_model(const AuthoredName& value) { set_member("model",results::encode(value)); return *this; }
inline RequestDefinitionFieldsScore& RequestDefinitionFieldsScore::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline RequestDefinitionFieldsScore& RequestDefinitionFieldsScore::set_on(const AuthoredPointers& value) { set_member("on",results::encode(value)); return *this; }
inline RequestDefinitionFieldsScore& RequestDefinitionFieldsScore::set_profile(const AuthoredProfile& value) { set_member("profile",results::encode(value)); return *this; }
inline RequestDefinitionFieldsScore& RequestDefinitionFieldsScore::set_score(const AuthoredQuestionText& value) { set_member("score",results::encode(value)); return *this; }
inline RequestDefinitionFieldsScore& RequestDefinitionFieldsScore::set_wording_version(const uint64_t& value) { set_member("wording_version",results::encode(value)); return *this; }
inline RequestDefinitionFieldsTag& RequestDefinitionFieldsTag::set_batch(const std::variant<std::string, uint64_t>& value) { set_member("batch",results::encode(value)); return *this; }
inline RequestDefinitionFieldsTag& RequestDefinitionFieldsTag::set_context_schema(const AuthoredInputDeclaration& value) { set_member("context_schema",results::encode(value)); return *this; }
inline RequestDefinitionFieldsTag& RequestDefinitionFieldsTag::set_item_schema(const AuthoredInputDeclaration& value) { set_member("item_schema",results::encode(value)); return *this; }
inline RequestDefinitionFieldsTag& RequestDefinitionFieldsTag::set_labels(const AuthoredLabels& value) { set_member("labels",results::encode(value)); return *this; }
inline RequestDefinitionFieldsTag& RequestDefinitionFieldsTag::set_model(const AuthoredName& value) { set_member("model",results::encode(value)); return *this; }
inline RequestDefinitionFieldsTag& RequestDefinitionFieldsTag::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline RequestDefinitionFieldsTag& RequestDefinitionFieldsTag::set_on(const AuthoredPointers& value) { set_member("on",results::encode(value)); return *this; }
inline RequestDefinitionFieldsTag& RequestDefinitionFieldsTag::set_profile(const AuthoredProfile& value) { set_member("profile",results::encode(value)); return *this; }
inline RequestDefinitionFieldsTag& RequestDefinitionFieldsTag::set_tag(const AuthoredQuestionText& value) { set_member("tag",results::encode(value)); return *this; }
inline RequestDefinitionFieldsTag& RequestDefinitionFieldsTag::set_threshold(const AuthoredCut& value) { set_member("threshold",results::encode(value)); return *this; }
inline RequestDefinitionFieldsTag& RequestDefinitionFieldsTag::set_wording_version(const uint64_t& value) { set_member("wording_version",results::encode(value)); return *this; }
inline RequestFraming::RequestFraming(const std::string& value):Node(results::encode(value)) {}
inline RequestImage::RequestImage(const RequestImageFile& value):Node(value.document()) {}
inline RequestImage::RequestImage(const RequestImageBytes& value):Node(value.document()) {}
inline RequestImageBytes& RequestImageBytes::set_bytes(const std::string& value) { set_member("bytes",results::encode(value)); return *this; }
inline RequestImageBytes& RequestImageBytes::set_media(const ImageMedia& value) { set_member("media",results::encode(value)); return *this; }
inline RequestImageFile& RequestImageFile::set_media(const ImageMedia& value) { set_member("media",results::encode(value)); return *this; }
inline RequestImageFile& RequestImageFile::set_path(const std::string& value) { set_member("path",results::encode(value)); return *this; }
inline RequestInput::RequestInput(const RequestInputText& value):Node(value.document()) {}
inline RequestInput::RequestInput(const RequestInputJson& value):Node(value.document()) {}
inline RequestInput::RequestInput(const RequestInputRecords& value):Node(value.document()) {}
inline RequestInput::RequestInput(const RequestInputUnits& value):Node(value.document()) {}
inline RequestInput::RequestInput(const RequestInputEntities& value):Node(value.document()) {}
inline RequestInput::RequestInput(const RequestInputSource& value):Node(value.document()) {}
inline RequestInput::RequestInput(const RequestInputFeed& value):Node(value.document()) {}
inline RequestInputEntities& RequestInputEntities::set_items(const std::vector<RequestItem>& value) { set_member("items",results::encode(value)); return *this; }
inline RequestInputFeed& RequestInputFeed::set_framing(const RequestFraming& value) { set_member("framing",results::encode(value)); return *this; }
inline RequestInputFeed& RequestInputFeed::set_images(const std::vector<RequestImage>& value) { set_member("images",results::encode(value)); return *this; }
inline RequestInputFeed& RequestInputFeed::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline RequestInputFeed& RequestInputFeed::set_reading(const RequestReader& value) { set_member("reading",results::encode(value)); return *this; }
inline RequestInputJson& RequestInputJson::set_images(const std::vector<RequestImage>& value) { set_member("images",results::encode(value)); return *this; }
inline RequestInputJson& RequestInputJson::set_value(const Json& value) { set_member("value",results::encode(value)); return *this; }
inline RequestInputRecords& RequestInputRecords::set_items(const std::vector<RequestItem>& value) { set_member("items",results::encode(value)); return *this; }
inline RequestInputSource& RequestInputSource::set_source(const RequestSource& value) { set_member("source",results::encode(value)); return *this; }
inline RequestInputText& RequestInputText::set_images(const std::vector<RequestImage>& value) { set_member("images",results::encode(value)); return *this; }
inline RequestInputText& RequestInputText::set_text(const std::string& value) { set_member("text",results::encode(value)); return *this; }
inline RequestInputUnits& RequestInputUnits::set_items(const std::vector<RequestItem>& value) { set_member("items",results::encode(value)); return *this; }
inline RequestItem& RequestItem::set_context(const ContextSchema& value) { set_member("context",results::encode(value)); return *this; }
inline RequestItem& RequestItem::set_examples(const std::vector<RecognitionExample>& value) { set_member("examples",results::encode(value)); return *this; }
inline RequestItem& RequestItem::set_images(const std::vector<RequestImage>& value) { set_member("images",results::encode(value)); return *this; }
inline RequestItem& RequestItem::set_options(const std::vector<OptionSchema>& value) { set_member("options",results::encode(value)); return *this; }
inline RequestItem& RequestItem::set_original(const RequestOriginal& value) { set_member("original",results::encode(value)); return *this; }
inline RequestItem& RequestItem::set_seed_spans(const std::vector<RecognitionSeedSpan>& value) { set_member("seed_spans",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_attempts(const bool& value) { set_member("attempts",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_batch(const RequestBatch& value) { set_member("batch",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_context(const std::string& value) { set_member("context",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_context_field(const std::string& value) { set_member("context_field",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_deadline_ms(const int64_t& value) { set_member("deadline_ms",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_details(const bool& value) { set_member("details",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_examples(const std::vector<RecognitionExample>& value) { set_member("examples",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_examples_field(const std::string& value) { set_member("examples_field",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_field(const std::vector<std::string>& value) { set_member("field",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_files_only(const bool& value) { set_member("files_only",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_max_requests_total(const uint64_t& value) { set_member("max_requests_total",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_mode(const RecognitionMode& value) { set_member("mode",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_model(const std::string& value) { set_member("model",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_none(const bool& value) { set_member("none",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_options_field(const std::string& value) { set_member("options_field",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_relation_threshold(const RequestThreshold& value) { set_member("relation_threshold",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_seed_spans(const std::vector<RecognitionSeedSpan>& value) { set_member("seed_spans",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_seed_spans_field(const std::string& value) { set_member("seed_spans_field",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_snippet_pieces(const uint32_t& value) { set_member("snippet_pieces",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_stage_context(const RecognitionStageContext& value) { set_member("stage_context",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_threshold(const RequestThreshold& value) { set_member("threshold",results::encode(value)); return *this; }
inline RequestOptions& RequestOptions::set_top(const uint64_t& value) { set_member("top",results::encode(value)); return *this; }
inline RequestOriginal::RequestOriginal(const RequestOriginalText& value):Node(value.document()) {}
inline RequestOriginal::RequestOriginal(const RequestOriginalJson& value):Node(value.document()) {}
inline RequestOriginalJson& RequestOriginalJson::set_value(const Json& value) { set_member("value",results::encode(value)); return *this; }
inline RequestOriginalText& RequestOriginalText::set_text(const std::string& value) { set_member("text",results::encode(value)); return *this; }
inline RequestQuestion::RequestQuestion(const RequestQuestionText& value):Node(value.document()) {}
inline RequestQuestion::RequestQuestion(const RequestQuestionDefinition& value):Node(value.document()) {}
inline RequestQuestion::RequestQuestion(const RequestQuestionFile& value):Node(value.document()) {}
inline RequestQuestion::RequestQuestion(const RequestQuestionName& value):Node(value.document()) {}
inline RequestQuestion::RequestQuestion(const RequestQuestionReference& value):Node(value.document()) {}
inline RequestQuestionDefinition& RequestQuestionDefinition::set_value(const RequestDefinition& value) { set_member("value",results::encode(value)); return *this; }
inline RequestQuestionFile& RequestQuestionFile::set_path(const std::string& value) { set_member("path",results::encode(value)); return *this; }
inline RequestQuestionName& RequestQuestionName::set_name(const std::string& value) { set_member("name",results::encode(value)); return *this; }
inline RequestQuestionReference& RequestQuestionReference::set_reference(const std::string& value) { set_member("reference",results::encode(value)); return *this; }
inline RequestQuestionText& RequestQuestionText::set_text(const std::string& value) { set_member("text",results::encode(value)); return *this; }
inline RequestReader& RequestReader::set_unit(const SourceUnit& value) { set_member("unit",results::encode(value)); return *this; }
inline RequestReader& RequestReader::set_window(const uint64_t& value) { set_member("window",results::encode(value)); return *this; }
inline RequestSessionDescriptor& RequestSessionDescriptor::set_item(const RequestItem& value) { set_member("item",results::encode(value)); return *this; }
inline RequestSessionDescriptor& RequestSessionDescriptor::set_location(const SessionSourceLocation& value) { set_member("location",results::encode(value)); return *this; }
inline RequestSource& RequestSource::set_media(const ReaderMedia& value) { set_member("media",results::encode(value)); return *this; }
inline RequestSource& RequestSource::set_paths(const std::vector<std::string>& value) { set_member("paths",results::encode(value)); return *this; }
inline RequestSource& RequestSource::set_reading(const RequestReader& value) { set_member("reading",results::encode(value)); return *this; }
inline RequestThreshold::RequestThreshold(const std::variant<double, std::string>& value):Node(results::encode(value)) {}
inline SessionSourceLocation& SessionSourceLocation::set_file(const std::string& value) { set_member("file",results::encode(value)); return *this; }
inline SessionSourceLocation& SessionSourceLocation::set_first_line(const uint64_t& value) { set_member("first_line",results::encode(value)); return *this; }
inline SessionSourceLocation& SessionSourceLocation::set_last_line(const uint64_t& value) { set_member("last_line",results::encode(value)); return *this; }
inline SourceUnit::SourceUnit(const std::string& value):Node(results::encode(value)) {}
}
