// Generated from request.schema.json; do not edit.
#pragma once
#include "result_value.hpp"
namespace tt::inputs {
    using results::Node;
    class AnnotateDefinition;
    class AnnotateQuestion;
    class AnnotateQuestionChoose;
    class AnnotateQuestionDecide;
    class AnnotateQuestionScore;
    class AnnotateQuestionTag;
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
    class RecognizeDefinition;
    class RecognizeQuestion;
    class RelateFields;
    class RelateQuestion;
    class Request;
    class RequestBatch;
    class RequestCall;
    class RequestDefinition;
    class RequestDefinitionAnnotateDefinition;
    class RequestDefinitionAuthoredChoose;
    class RequestDefinitionAuthoredDecide;
    class RequestDefinitionAuthoredFind;
    class RequestDefinitionAuthoredRelate;
    class RequestDefinitionAuthoredScore;
    class RequestDefinitionAuthoredTag;
    class RequestDefinitionRecognizeDefinition;
    class RequestFraming;
    class RequestFunction;
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
    class RequestVersion;
    class SessionSourceLocation;
    class SourceUnit;
    class AnnotateDefinition : public Node {
        AnnotateDefinition(const Json& value, int):Node(value) {
        }
        public: static AnnotateDefinition from_document(const Json& value) {
            return AnnotateDefinition(value,0);
        }
        AnnotateDefinition():Node(Json(Json::Object{
            {
                "version",Json(1)
            }
        })) {
        }
        AnnotateDefinition& set_batch(const Json& value);
        AnnotateDefinition& set_profile(const AuthoredProfile& value);
        AnnotateDefinition& set_questions(const std::vector<std::pair<std::string, AnnotateQuestion>>& value);
        AnnotateDefinition& set_threshold(const AuthoredThreshold& value);
    };
    class AnnotateQuestion : public Node {
        AnnotateQuestion(const Json& value, int):Node(value) {
        }
        public: static AnnotateQuestion from_document(const Json& value) {
            return AnnotateQuestion(value,0);
        }
        AnnotateQuestion(const AnnotateQuestionDecide& value);
        AnnotateQuestion(const AnnotateQuestionChoose& value);
        AnnotateQuestion(const AnnotateQuestionTag& value);
        AnnotateQuestion(const AnnotateQuestionScore& value);
    };
    class AnnotateQuestionChoose : public Node {
        AnnotateQuestionChoose(const Json& value, int):Node(value) {
        }
        public: static AnnotateQuestionChoose from_document(const Json& value) {
            return AnnotateQuestionChoose(value,0);
        }
        AnnotateQuestionChoose():Node(Json(Json::Object{
        })) {
        }
        AnnotateQuestionChoose& set_choose(const AuthoredQuestionText& value);
        AnnotateQuestionChoose& set_context_schema(const AuthoredInputDeclaration& value);
        AnnotateQuestionChoose& set_item_schema(const AuthoredInputDeclaration& value);
        AnnotateQuestionChoose& set_name(const std::string& value);
        AnnotateQuestionChoose& set_on(const AuthoredPointers& value);
        AnnotateQuestionChoose& set_options(const AuthoredOptions& value);
        AnnotateQuestionChoose& set_threshold(const AuthoredCut& value);
        AnnotateQuestionChoose& set_wording_version(const uint64_t& value);
    };
    class AnnotateQuestionDecide : public Node {
        AnnotateQuestionDecide(const Json& value, int):Node(value) {
        }
        public: static AnnotateQuestionDecide from_document(const Json& value) {
            return AnnotateQuestionDecide(value,0);
        }
        AnnotateQuestionDecide():Node(Json(Json::Object{
        })) {
        }
        AnnotateQuestionDecide& set_context_schema(const AuthoredInputDeclaration& value);
        AnnotateQuestionDecide& set_decide(const AuthoredQuestionText& value);
        AnnotateQuestionDecide& set_false(const AuthoredCriterion& value);
        AnnotateQuestionDecide& set_item_schema(const AuthoredInputDeclaration& value);
        AnnotateQuestionDecide& set_name(const std::string& value);
        AnnotateQuestionDecide& set_on(const AuthoredPointers& value);
        AnnotateQuestionDecide& set_threshold(const AuthoredThreshold& value);
        AnnotateQuestionDecide& set_true(const AuthoredCriterion& value);
        AnnotateQuestionDecide& set_wording_version(const uint64_t& value);
    };
    class AnnotateQuestionScore : public Node {
        AnnotateQuestionScore(const Json& value, int):Node(value) {
        }
        public: static AnnotateQuestionScore from_document(const Json& value) {
            return AnnotateQuestionScore(value,0);
        }
        AnnotateQuestionScore():Node(Json(Json::Object{
        })) {
        }
        AnnotateQuestionScore& set_context_schema(const AuthoredInputDeclaration& value);
        AnnotateQuestionScore& set_item_schema(const AuthoredInputDeclaration& value);
        AnnotateQuestionScore& set_levels(const AuthoredLevels& value);
        AnnotateQuestionScore& set_name(const std::string& value);
        AnnotateQuestionScore& set_on(const AuthoredPointers& value);
        AnnotateQuestionScore& set_score(const AuthoredQuestionText& value);
        AnnotateQuestionScore& set_wording_version(const uint64_t& value);
    };
    class AnnotateQuestionTag : public Node {
        AnnotateQuestionTag(const Json& value, int):Node(value) {
        }
        public: static AnnotateQuestionTag from_document(const Json& value) {
            return AnnotateQuestionTag(value,0);
        }
        AnnotateQuestionTag():Node(Json(Json::Object{
        })) {
        }
        AnnotateQuestionTag& set_context_schema(const AuthoredInputDeclaration& value);
        AnnotateQuestionTag& set_item_schema(const AuthoredInputDeclaration& value);
        AnnotateQuestionTag& set_labels(const AuthoredLabels& value);
        AnnotateQuestionTag& set_name(const std::string& value);
        AnnotateQuestionTag& set_on(const AuthoredPointers& value);
        AnnotateQuestionTag& set_tag(const AuthoredQuestionText& value);
        AnnotateQuestionTag& set_threshold(const AuthoredCut& value);
        AnnotateQuestionTag& set_wording_version(const uint64_t& value);
    };
    class AuthoredChoose : public Node {
        AuthoredChoose(const Json& value, int):Node(value) {
        }
        public: static AuthoredChoose from_document(const Json& value) {
            return AuthoredChoose(value,0);
        }
        AuthoredChoose():Node(Json(Json::Object{
        })) {
        }
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
    class AuthoredCriterion : public Node {
        AuthoredCriterion(const Json& value, int):Node(value) {
        }
        public: static AuthoredCriterion from_document(const Json& value) {
            return AuthoredCriterion(value,0);
        }
        explicit AuthoredCriterion(const std::variant<std::string, Json, std::vector<Json>, std::nullptr_t>& value);
    };
    class AuthoredCut : public Node {
        AuthoredCut(const Json& value, int):Node(value) {
        }
        public: static AuthoredCut from_document(const Json& value) {
            return AuthoredCut(value,0);
        }
        explicit AuthoredCut(const std::variant<double, std::string>& value);
    };
    class AuthoredDecide : public Node {
        AuthoredDecide(const Json& value, int):Node(value) {
        }
        public: static AuthoredDecide from_document(const Json& value) {
            return AuthoredDecide(value,0);
        }
        AuthoredDecide():Node(Json(Json::Object{
        })) {
        }
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
    class AuthoredDescription : public Node {
        AuthoredDescription(const Json& value, int):Node(value) {
        }
        public: static AuthoredDescription from_document(const Json& value) {
            return AuthoredDescription(value,0);
        }
        explicit AuthoredDescription(const std::variant<std::string, Json, std::vector<Json>, std::nullptr_t>& value);
    };
    class AuthoredFind : public Node {
        AuthoredFind(const Json& value, int):Node(value) {
        }
        public: static AuthoredFind from_document(const Json& value) {
            return AuthoredFind(value,0);
        }
        AuthoredFind():Node(Json(Json::Object{
        })) {
        }
        AuthoredFind& set_context_schema(const AuthoredInputDeclaration& value);
        AuthoredFind& set_find(const AuthoredQuestionText& value);
        AuthoredFind& set_item_schema(const AuthoredInputDeclaration& value);
        AuthoredFind& set_model(const AuthoredName& value);
        AuthoredFind& set_name(const std::string& value);
        AuthoredFind& set_on(const AuthoredPointers& value);
        AuthoredFind& set_profile(const AuthoredProfile& value);
        AuthoredFind& set_wording_version(const uint64_t& value);
    };
    class AuthoredInputDeclaration : public Node {
        AuthoredInputDeclaration(const Json& value, int):Node(value) {
        }
        public: static AuthoredInputDeclaration from_document(const Json& value) {
            return AuthoredInputDeclaration(value,0);
        }
        AuthoredInputDeclaration(const AuthoredInputDeclarationString& value);
        AuthoredInputDeclaration(const AuthoredInputDeclarationObject& value);
    };
    class AuthoredInputDeclarationObject : public Node {
        AuthoredInputDeclarationObject(const Json& value, int):Node(value) {
        }
        public: static AuthoredInputDeclarationObject from_document(const Json& value) {
            return AuthoredInputDeclarationObject(value,0);
        }
        AuthoredInputDeclarationObject():Node(Json(Json::Object{
            {
                "type",Json("object")
            }
        })) {
        }
        AuthoredInputDeclarationObject& set_properties(const std::vector<std::pair<std::string,
        AuthoredInputProperty>>& value);
        AuthoredInputDeclarationObject& set_required(const std::vector<std::string>& value);
    };
    class AuthoredInputDeclarationString : public Node {
        AuthoredInputDeclarationString(const Json& value, int):Node(value) {
        }
        public: static AuthoredInputDeclarationString from_document(const Json& value) {
            return AuthoredInputDeclarationString(value,0);
        }
        AuthoredInputDeclarationString():Node(Json(Json::Object{
            {
                "type",Json("string")
            }
        })) {
        }
    };
    class AuthoredInputProperty : public Node {
        AuthoredInputProperty(const Json& value, int):Node(value) {
        }
        public: static AuthoredInputProperty from_document(const Json& value) {
            return AuthoredInputProperty(value,0);
        }
        AuthoredInputProperty(const AuthoredInputPropertyString& value);
        AuthoredInputProperty(const AuthoredInputPropertyNumber& value);
        AuthoredInputProperty(const AuthoredInputPropertyBoolean& value);
        AuthoredInputProperty(const AuthoredInputPropertyArray& value);
    };
    class AuthoredInputPropertyArray : public Node {
        AuthoredInputPropertyArray(const Json& value, int):Node(value) {
        }
        public: static AuthoredInputPropertyArray from_document(const Json& value) {
            return AuthoredInputPropertyArray(value,0);
        }
        AuthoredInputPropertyArray():Node(Json(Json::Object{
            {
                "type",Json("array")
            }
        })) {
        }
        AuthoredInputPropertyArray& set_items(const Json& value);
    };
    class AuthoredInputPropertyBoolean : public Node {
        AuthoredInputPropertyBoolean(const Json& value, int):Node(value) {
        }
        public: static AuthoredInputPropertyBoolean from_document(const Json& value) {
            return AuthoredInputPropertyBoolean(value,0);
        }
        AuthoredInputPropertyBoolean():Node(Json(Json::Object{
            {
                "type",Json("boolean")
            }
        })) {
        }
    };
    class AuthoredInputPropertyNumber : public Node {
        AuthoredInputPropertyNumber(const Json& value, int):Node(value) {
        }
        public: static AuthoredInputPropertyNumber from_document(const Json& value) {
            return AuthoredInputPropertyNumber(value,0);
        }
        AuthoredInputPropertyNumber():Node(Json(Json::Object{
            {
                "type",Json("number")
            }
        })) {
        }
    };
    class AuthoredInputPropertyString : public Node {
        AuthoredInputPropertyString(const Json& value, int):Node(value) {
        }
        public: static AuthoredInputPropertyString from_document(const Json& value) {
            return AuthoredInputPropertyString(value,0);
        }
        AuthoredInputPropertyString():Node(Json(Json::Object{
            {
                "type",Json("string")
            }
        })) {
        }
    };
    class AuthoredLabels : public Node {
        AuthoredLabels(const Json& value, int):Node(value) {
        }
        public: static AuthoredLabels from_document(const Json& value) {
            return AuthoredLabels(value,0);
        }
        explicit AuthoredLabels(const std::variant<std::vector<AuthoredName>, std::vector<std::pair<std::string,
        AuthoredDescription>>>& value);
    };
    class AuthoredLevels : public Node {
        AuthoredLevels(const Json& value, int):Node(value) {
        }
        public: static AuthoredLevels from_document(const Json& value) {
            return AuthoredLevels(value,0);
        }
        explicit AuthoredLevels(const std::variant<std::vector<AuthoredName>, std::vector<std::pair<std::string,
        AuthoredCriterion>>>& value);
    };
    class AuthoredName : public Node {
        AuthoredName(const Json& value, int):Node(value) {
        }
        public: static AuthoredName from_document(const Json& value) {
            return AuthoredName(value,0);
        }
        explicit AuthoredName(const std::string& value);
    };
    class AuthoredOptions : public Node {
        AuthoredOptions(const Json& value, int):Node(value) {
        }
        public: static AuthoredOptions from_document(const Json& value) {
            return AuthoredOptions(value,0);
        }
        explicit AuthoredOptions(const std::variant<std::vector<AuthoredName>, std::vector<std::pair<std::string,
        AuthoredDescription>>>& value);
    };
    class AuthoredPointers : public Node {
        AuthoredPointers(const Json& value, int):Node(value) {
        }
        public: static AuthoredPointers from_document(const Json& value) {
            return AuthoredPointers(value,0);
        }
        explicit AuthoredPointers(const std::variant<std::string, std::vector<std::string>>& value);
    };
    class AuthoredProfile : public Node {
        AuthoredProfile(const Json& value, int):Node(value) {
        }
        public: static AuthoredProfile from_document(const Json& value) {
            return AuthoredProfile(value,0);
        }
        explicit AuthoredProfile(const std::string& value);
    };
    class AuthoredQuestionText : public Node {
        AuthoredQuestionText(const Json& value, int):Node(value) {
        }
        public: static AuthoredQuestionText from_document(const Json& value) {
            return AuthoredQuestionText(value,0);
        }
        explicit AuthoredQuestionText(const std::variant<std::string, Json, std::vector<Json>>& value);
    };
    class AuthoredRelate : public Node {
        AuthoredRelate(const Json& value, int):Node(value) {
        }
        public: static AuthoredRelate from_document(const Json& value) {
            return AuthoredRelate(value,0);
        }
        AuthoredRelate():Node(Json(Json::Object{
            {
                "version",Json(1)
            }
        })) {
        }
        AuthoredRelate& set_context_schema(const AuthoredInputDeclaration& value);
        AuthoredRelate& set_item_schema(const AuthoredInputDeclaration& value);
        AuthoredRelate& set_model(const AuthoredName& value);
        AuthoredRelate& set_name(const std::string& value);
        AuthoredRelate& set_profile(const AuthoredProfile& value);
        AuthoredRelate& set_relate(const RelateQuestion& value);
        AuthoredRelate& set_threshold(const AuthoredCut& value);
        AuthoredRelate& set_wording_version(const uint64_t& value);
    };
    class AuthoredRelation : public Node {
        AuthoredRelation(const Json& value, int):Node(value) {
        }
        public: static AuthoredRelation from_document(const Json& value) {
            return AuthoredRelation(value,0);
        }
        AuthoredRelation():Node(Json(Json::Object{
        })) {
        }
        AuthoredRelation& set_either(const bool& value);
        AuthoredRelation& set_name(const AuthoredName& value);
        AuthoredRelation& set_reads(const AuthoredName& value);
        AuthoredRelation& set_single(const bool& value);
        AuthoredRelation& set_source(const AuthoredName& value);
        AuthoredRelation& set_target(const AuthoredName& value);
    };
    class AuthoredScore : public Node {
        AuthoredScore(const Json& value, int):Node(value) {
        }
        public: static AuthoredScore from_document(const Json& value) {
            return AuthoredScore(value,0);
        }
        AuthoredScore():Node(Json(Json::Object{
        })) {
        }
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
    class AuthoredTag : public Node {
        AuthoredTag(const Json& value, int):Node(value) {
        }
        public: static AuthoredTag from_document(const Json& value) {
            return AuthoredTag(value,0);
        }
        AuthoredTag():Node(Json(Json::Object{
        })) {
        }
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
    class AuthoredThreshold : public Node {
        AuthoredThreshold(const Json& value, int):Node(value) {
        }
        public: static AuthoredThreshold from_document(const Json& value) {
            return AuthoredThreshold(value,0);
        }
        explicit AuthoredThreshold(const std::variant<double, std::string>& value);
    };
    class ContextSchema : public Node {
        ContextSchema(const Json& value, int):Node(value) {
        }
        public: static ContextSchema from_document(const Json& value) {
            return ContextSchema(value,0);
        }
        explicit ContextSchema(const std::variant<std::string, Json>& value);
    };
    class ImageMedia : public Node {
        ImageMedia(const Json& value, int):Node(value) {
        }
        public: static ImageMedia from_document(const Json& value) {
            return ImageMedia(value,0);
        }
        explicit ImageMedia(const std::string& value);
    };
    class OptionSchema : public Node {
        OptionSchema(const Json& value, int):Node(value) {
        }
        public: static OptionSchema from_document(const Json& value) {
            return OptionSchema(value,0);
        }
        OptionSchema():Node(Json(Json::Object{
        })) {
        }
        OptionSchema& set_description(const Json& value);
        OptionSchema& set_name(const std::string& value);
    };
    class ReaderMedia : public Node {
        ReaderMedia(const Json& value, int):Node(value) {
        }
        public: static ReaderMedia from_document(const Json& value) {
            return ReaderMedia(value,0);
        }
        explicit ReaderMedia(const std::string& value);
    };
    class RecognitionExample : public Node {
        RecognitionExample(const Json& value, int):Node(value) {
        }
        public: static RecognitionExample from_document(const Json& value) {
            return RecognitionExample(value,0);
        }
        explicit RecognitionExample(const std::variant<std::string, RecognitionExampleText>& value);
    };
    class RecognitionExampleEntity : public Node {
        RecognitionExampleEntity(const Json& value, int):Node(value) {
        }
        public: static RecognitionExampleEntity from_document(const Json& value) {
            return RecognitionExampleEntity(value,0);
        }
        RecognitionExampleEntity():Node(Json(Json::Object{
        })) {
        }
        RecognitionExampleEntity& set_end(const uint64_t& value);
        RecognitionExampleEntity& set_kind(const std::string& value);
        RecognitionExampleEntity& set_start(const uint64_t& value);
    };
    class RecognitionExampleText : public Node {
        RecognitionExampleText(const Json& value, int):Node(value) {
        }
        public: static RecognitionExampleText from_document(const Json& value) {
            return RecognitionExampleText(value,0);
        }
        RecognitionExampleText():Node(Json(Json::Object{
        })) {
        }
        RecognitionExampleText& set_entities(const std::vector<RecognitionExampleEntity>& value);
        RecognitionExampleText& set_kinds(const std::vector<std::string>& value);
        RecognitionExampleText& set_text(const std::string& value);
    };
    class RecognitionMode : public Node {
        RecognitionMode(const Json& value, int):Node(value) {
        }
        public: static RecognitionMode from_document(const Json& value) {
            return RecognitionMode(value,0);
        }
        explicit RecognitionMode(const std::string& value);
    };
    class RecognitionSeedSpan : public Node {
        RecognitionSeedSpan(const Json& value, int):Node(value) {
        }
        public: static RecognitionSeedSpan from_document(const Json& value) {
            return RecognitionSeedSpan(value,0);
        }
        RecognitionSeedSpan():Node(Json(Json::Object{
        })) {
        }
        RecognitionSeedSpan& set_end(const uint64_t& value);
        RecognitionSeedSpan& set_kind(const std::string& value);
        RecognitionSeedSpan& set_start(const uint64_t& value);
    };
    class RecognitionStageContext : public Node {
        RecognitionStageContext(const Json& value, int):Node(value) {
        }
        public: static RecognitionStageContext from_document(const Json& value) {
            return RecognitionStageContext(value,0);
        }
        RecognitionStageContext():Node(Json(Json::Object{
        })) {
        }
        RecognitionStageContext& set_boundary(const std::string& value);
        RecognitionStageContext& set_kind_edge(const std::string& value);
        RecognitionStageContext& set_relation(const std::string& value);
    };
    class RecognizeDefinition : public Node {
        RecognizeDefinition(const Json& value, int):Node(value) {
        }
        public: static RecognizeDefinition from_document(const Json& value) {
            return RecognizeDefinition(value,0);
        }
        RecognizeDefinition():Node(Json(Json::Object{
            {
                "version",Json(1)
            }
        })) {
        }
        RecognizeDefinition& set_context_schema(const AuthoredInputDeclaration& value);
        RecognizeDefinition& set_item_schema(const AuthoredInputDeclaration& value);
        RecognizeDefinition& set_model(const AuthoredName& value);
        RecognizeDefinition& set_name(const std::string& value);
        RecognizeDefinition& set_on(const AuthoredPointers& value);
        RecognizeDefinition& set_profile(const AuthoredProfile& value);
        RecognizeDefinition& set_recognize(const RecognizeQuestion& value);
        RecognizeDefinition& set_relation_threshold(const AuthoredCut& value);
        RecognizeDefinition& set_threshold(const AuthoredCut& value);
        RecognizeDefinition& set_wording_version(const uint64_t& value);
    };
    class RecognizeQuestion : public Node {
        RecognizeQuestion(const Json& value, int):Node(value) {
        }
        public: static RecognizeQuestion from_document(const Json& value) {
            return RecognizeQuestion(value,0);
        }
        RecognizeQuestion():Node(Json(Json::Object{
        })) {
        }
        RecognizeQuestion& set_entity_definition(const AuthoredQuestionText& value);
        RecognizeQuestion& set_instructions(const AuthoredQuestionText& value);
        RecognizeQuestion& set_kinds(const std::vector<std::pair<std::string, AuthoredDescription>>& value);
        RecognizeQuestion& set_mode(const RecognitionMode& value);
        RecognizeQuestion& set_relations(const std::vector<AuthoredRelation>& value);
        RecognizeQuestion& set_snippet_pieces(const uint32_t& value);
        RecognizeQuestion& set_stage_context(const RecognitionStageContext& value);
    };
    class RelateFields : public Node {
        RelateFields(const Json& value, int):Node(value) {
        }
        public: static RelateFields from_document(const Json& value) {
            return RelateFields(value,0);
        }
        RelateFields():Node(Json(Json::Object{
        })) {
        }
        RelateFields& set_kind(const std::string& value);
        RelateFields& set_name(const std::string& value);
    };
    class RelateQuestion : public Node {
        RelateQuestion(const Json& value, int):Node(value) {
        }
        public: static RelateQuestion from_document(const Json& value) {
            return RelateQuestion(value,0);
        }
        RelateQuestion():Node(Json(Json::Object{
        })) {
        }
        RelateQuestion& set_fields(const RelateFields& value);
        RelateQuestion& set_relations(const std::vector<AuthoredRelation>& value);
    };
    class Request : public Node {
        Request(const Json& value, int):Node(value) {
        }
        public: static Request from_document(const Json& value) {
            return Request(value,0);
        }
        Request():Node(Json(Json::Object{
            {
                "schema",Json("thinkthen.request/1")
            }
        })) {
        }
        Request& set_call(const RequestCall& value);
        Request& set_schema(const RequestVersion& value);
    };
    class RequestBatch : public Node {
        RequestBatch(const Json& value, int):Node(value) {
        }
        public: static RequestBatch from_document(const Json& value) {
            return RequestBatch(value,0);
        }
        explicit RequestBatch(const std::variant<uint64_t, std::string>& value);
    };
    class RequestCall : public Node {
        RequestCall(const Json& value, int):Node(value) {
        }
        public: static RequestCall from_document(const Json& value) {
            return RequestCall(value,0);
        }
        RequestCall():Node(Json(Json::Object{
        })) {
        }
        RequestCall& set_function(const RequestFunction& value);
        RequestCall& set_input(const RequestInput& value);
        RequestCall& set_options(const RequestOptions& value);
        RequestCall& set_question(const RequestQuestion& value);
    };
    class RequestDefinition : public Node {
        RequestDefinition(const Json& value, int):Node(value) {
        }
        public: static RequestDefinition from_document(const Json& value) {
            return RequestDefinition(value,0);
        }
        RequestDefinition(const RequestDefinitionAuthoredDecide& value);
        RequestDefinition(const RequestDefinitionAuthoredChoose& value);
        RequestDefinition(const RequestDefinitionAuthoredTag& value);
        RequestDefinition(const RequestDefinitionAuthoredScore& value);
        RequestDefinition(const RequestDefinitionAuthoredRelate& value);
        RequestDefinition(const RequestDefinitionAuthoredFind& value);
        RequestDefinition(const RequestDefinitionRecognizeDefinition& value);
        RequestDefinition(const RequestDefinitionAnnotateDefinition& value);
    };
    class RequestDefinitionAnnotateDefinition : public Node {
        RequestDefinitionAnnotateDefinition(const Json& value, int):Node(value) {
        }
        public: static RequestDefinitionAnnotateDefinition from_document(const Json& value) {
            return RequestDefinitionAnnotateDefinition(value,0);
        }
        RequestDefinitionAnnotateDefinition():Node(Json(Json::Object{
            {
                "version",Json(1)
            }
        })) {
        }
        RequestDefinitionAnnotateDefinition& set_batch(const Json& value);
        RequestDefinitionAnnotateDefinition& set_profile(const AuthoredProfile& value);
        RequestDefinitionAnnotateDefinition& set_questions(const std::vector<std::pair<std::string,
        AnnotateQuestion>>& value);
        RequestDefinitionAnnotateDefinition& set_threshold(const AuthoredThreshold& value);
    };
    class RequestDefinitionAuthoredChoose : public Node {
        RequestDefinitionAuthoredChoose(const Json& value, int):Node(value) {
        }
        public: static RequestDefinitionAuthoredChoose from_document(const Json& value) {
            return RequestDefinitionAuthoredChoose(value,0);
        }
        RequestDefinitionAuthoredChoose():Node(Json(Json::Object{
        })) {
        }
        RequestDefinitionAuthoredChoose& set_batch(const std::variant<std::string, uint64_t>& value);
        RequestDefinitionAuthoredChoose& set_choose(const AuthoredQuestionText& value);
        RequestDefinitionAuthoredChoose& set_context_schema(const AuthoredInputDeclaration& value);
        RequestDefinitionAuthoredChoose& set_item_schema(const AuthoredInputDeclaration& value);
        RequestDefinitionAuthoredChoose& set_model(const AuthoredName& value);
        RequestDefinitionAuthoredChoose& set_name(const std::string& value);
        RequestDefinitionAuthoredChoose& set_on(const AuthoredPointers& value);
        RequestDefinitionAuthoredChoose& set_options(const AuthoredOptions& value);
        RequestDefinitionAuthoredChoose& set_profile(const AuthoredProfile& value);
        RequestDefinitionAuthoredChoose& set_threshold(const AuthoredCut& value);
        RequestDefinitionAuthoredChoose& set_wording_version(const uint64_t& value);
    };
    class RequestDefinitionAuthoredDecide : public Node {
        RequestDefinitionAuthoredDecide(const Json& value, int):Node(value) {
        }
        public: static RequestDefinitionAuthoredDecide from_document(const Json& value) {
            return RequestDefinitionAuthoredDecide(value,0);
        }
        RequestDefinitionAuthoredDecide():Node(Json(Json::Object{
        })) {
        }
        RequestDefinitionAuthoredDecide& set_batch(const std::variant<std::string, uint64_t>& value);
        RequestDefinitionAuthoredDecide& set_context_schema(const AuthoredInputDeclaration& value);
        RequestDefinitionAuthoredDecide& set_decide(const AuthoredQuestionText& value);
        RequestDefinitionAuthoredDecide& set_false(const AuthoredCriterion& value);
        RequestDefinitionAuthoredDecide& set_item_schema(const AuthoredInputDeclaration& value);
        RequestDefinitionAuthoredDecide& set_model(const AuthoredName& value);
        RequestDefinitionAuthoredDecide& set_name(const std::string& value);
        RequestDefinitionAuthoredDecide& set_on(const AuthoredPointers& value);
        RequestDefinitionAuthoredDecide& set_profile(const AuthoredProfile& value);
        RequestDefinitionAuthoredDecide& set_threshold(const AuthoredThreshold& value);
        RequestDefinitionAuthoredDecide& set_true(const AuthoredCriterion& value);
        RequestDefinitionAuthoredDecide& set_wording_version(const uint64_t& value);
    };
    class RequestDefinitionAuthoredFind : public Node {
        RequestDefinitionAuthoredFind(const Json& value, int):Node(value) {
        }
        public: static RequestDefinitionAuthoredFind from_document(const Json& value) {
            return RequestDefinitionAuthoredFind(value,0);
        }
        RequestDefinitionAuthoredFind():Node(Json(Json::Object{
        })) {
        }
        RequestDefinitionAuthoredFind& set_context_schema(const AuthoredInputDeclaration& value);
        RequestDefinitionAuthoredFind& set_find(const AuthoredQuestionText& value);
        RequestDefinitionAuthoredFind& set_item_schema(const AuthoredInputDeclaration& value);
        RequestDefinitionAuthoredFind& set_model(const AuthoredName& value);
        RequestDefinitionAuthoredFind& set_name(const std::string& value);
        RequestDefinitionAuthoredFind& set_on(const AuthoredPointers& value);
        RequestDefinitionAuthoredFind& set_profile(const AuthoredProfile& value);
        RequestDefinitionAuthoredFind& set_wording_version(const uint64_t& value);
    };
    class RequestDefinitionAuthoredRelate : public Node {
        RequestDefinitionAuthoredRelate(const Json& value, int):Node(value) {
        }
        public: static RequestDefinitionAuthoredRelate from_document(const Json& value) {
            return RequestDefinitionAuthoredRelate(value,0);
        }
        RequestDefinitionAuthoredRelate():Node(Json(Json::Object{
            {
                "version",Json(1)
            }
        })) {
        }
        RequestDefinitionAuthoredRelate& set_context_schema(const AuthoredInputDeclaration& value);
        RequestDefinitionAuthoredRelate& set_item_schema(const AuthoredInputDeclaration& value);
        RequestDefinitionAuthoredRelate& set_model(const AuthoredName& value);
        RequestDefinitionAuthoredRelate& set_name(const std::string& value);
        RequestDefinitionAuthoredRelate& set_profile(const AuthoredProfile& value);
        RequestDefinitionAuthoredRelate& set_relate(const RelateQuestion& value);
        RequestDefinitionAuthoredRelate& set_threshold(const AuthoredCut& value);
        RequestDefinitionAuthoredRelate& set_wording_version(const uint64_t& value);
    };
    class RequestDefinitionAuthoredScore : public Node {
        RequestDefinitionAuthoredScore(const Json& value, int):Node(value) {
        }
        public: static RequestDefinitionAuthoredScore from_document(const Json& value) {
            return RequestDefinitionAuthoredScore(value,0);
        }
        RequestDefinitionAuthoredScore():Node(Json(Json::Object{
        })) {
        }
        RequestDefinitionAuthoredScore& set_batch(const std::variant<std::string, uint64_t>& value);
        RequestDefinitionAuthoredScore& set_context_schema(const AuthoredInputDeclaration& value);
        RequestDefinitionAuthoredScore& set_item_schema(const AuthoredInputDeclaration& value);
        RequestDefinitionAuthoredScore& set_levels(const AuthoredLevels& value);
        RequestDefinitionAuthoredScore& set_model(const AuthoredName& value);
        RequestDefinitionAuthoredScore& set_name(const std::string& value);
        RequestDefinitionAuthoredScore& set_on(const AuthoredPointers& value);
        RequestDefinitionAuthoredScore& set_profile(const AuthoredProfile& value);
        RequestDefinitionAuthoredScore& set_score(const AuthoredQuestionText& value);
        RequestDefinitionAuthoredScore& set_wording_version(const uint64_t& value);
    };
    class RequestDefinitionAuthoredTag : public Node {
        RequestDefinitionAuthoredTag(const Json& value, int):Node(value) {
        }
        public: static RequestDefinitionAuthoredTag from_document(const Json& value) {
            return RequestDefinitionAuthoredTag(value,0);
        }
        RequestDefinitionAuthoredTag():Node(Json(Json::Object{
        })) {
        }
        RequestDefinitionAuthoredTag& set_batch(const std::variant<std::string, uint64_t>& value);
        RequestDefinitionAuthoredTag& set_context_schema(const AuthoredInputDeclaration& value);
        RequestDefinitionAuthoredTag& set_item_schema(const AuthoredInputDeclaration& value);
        RequestDefinitionAuthoredTag& set_labels(const AuthoredLabels& value);
        RequestDefinitionAuthoredTag& set_model(const AuthoredName& value);
        RequestDefinitionAuthoredTag& set_name(const std::string& value);
        RequestDefinitionAuthoredTag& set_on(const AuthoredPointers& value);
        RequestDefinitionAuthoredTag& set_profile(const AuthoredProfile& value);
        RequestDefinitionAuthoredTag& set_tag(const AuthoredQuestionText& value);
        RequestDefinitionAuthoredTag& set_threshold(const AuthoredCut& value);
        RequestDefinitionAuthoredTag& set_wording_version(const uint64_t& value);
    };
    class RequestDefinitionRecognizeDefinition : public Node {
        RequestDefinitionRecognizeDefinition(const Json& value, int):Node(value) {
        }
        public: static RequestDefinitionRecognizeDefinition from_document(const Json& value) {
            return RequestDefinitionRecognizeDefinition(value,0);
        }
        RequestDefinitionRecognizeDefinition():Node(Json(Json::Object{
            {
                "version",Json(1)
            }
        })) {
        }
        RequestDefinitionRecognizeDefinition& set_context_schema(const AuthoredInputDeclaration& value);
        RequestDefinitionRecognizeDefinition& set_item_schema(const AuthoredInputDeclaration& value);
        RequestDefinitionRecognizeDefinition& set_model(const AuthoredName& value);
        RequestDefinitionRecognizeDefinition& set_name(const std::string& value);
        RequestDefinitionRecognizeDefinition& set_on(const AuthoredPointers& value);
        RequestDefinitionRecognizeDefinition& set_profile(const AuthoredProfile& value);
        RequestDefinitionRecognizeDefinition& set_recognize(const RecognizeQuestion& value);
        RequestDefinitionRecognizeDefinition& set_relation_threshold(const AuthoredCut& value);
        RequestDefinitionRecognizeDefinition& set_threshold(const AuthoredCut& value);
        RequestDefinitionRecognizeDefinition& set_wording_version(const uint64_t& value);
    };
    class RequestFraming : public Node {
        RequestFraming(const Json& value, int):Node(value) {
        }
        public: static RequestFraming from_document(const Json& value) {
            return RequestFraming(value,0);
        }
        explicit RequestFraming(const std::string& value);
    };
    class RequestFunction : public Node {
        RequestFunction(const Json& value, int):Node(value) {
        }
        public: static RequestFunction from_document(const Json& value) {
            return RequestFunction(value,0);
        }
        explicit RequestFunction(const std::string& value);
    };
    class RequestImage : public Node {
        RequestImage(const Json& value, int):Node(value) {
        }
        public: static RequestImage from_document(const Json& value) {
            return RequestImage(value,0);
        }
        RequestImage(const RequestImageFile& value);
        RequestImage(const RequestImageBytes& value);
    };
    class RequestImageBytes : public Node {
        RequestImageBytes(const Json& value, int):Node(value) {
        }
        public: static RequestImageBytes from_document(const Json& value) {
            return RequestImageBytes(value,0);
        }
        RequestImageBytes():Node(Json(Json::Object{
            {
                "kind",Json("bytes")
            }
        })) {
        }
        RequestImageBytes& set_bytes(const std::string& value);
        RequestImageBytes& set_media(const ImageMedia& value);
    };
    class RequestImageFile : public Node {
        RequestImageFile(const Json& value, int):Node(value) {
        }
        public: static RequestImageFile from_document(const Json& value) {
            return RequestImageFile(value,0);
        }
        RequestImageFile():Node(Json(Json::Object{
            {
                "kind",Json("file")
            }
        })) {
        }
        RequestImageFile& set_media(const ImageMedia& value);
        RequestImageFile& set_path(const std::string& value);
    };
    class RequestInput : public Node {
        RequestInput(const Json& value, int):Node(value) {
        }
        public: static RequestInput from_document(const Json& value) {
            return RequestInput(value,0);
        }
        RequestInput(const RequestInputText& value);
        RequestInput(const RequestInputJson& value);
        RequestInput(const RequestInputRecords& value);
        RequestInput(const RequestInputUnits& value);
        RequestInput(const RequestInputEntities& value);
        RequestInput(const RequestInputSource& value);
        RequestInput(const RequestInputFeed& value);
    };
    class RequestInputEntities : public Node {
        RequestInputEntities(const Json& value, int):Node(value) {
        }
        public: static RequestInputEntities from_document(const Json& value) {
            return RequestInputEntities(value,0);
        }
        RequestInputEntities():Node(Json(Json::Object{
            {
                "kind",Json("entities")
            }
        })) {
        }
        RequestInputEntities& set_items(const std::vector<RequestItem>& value);
    };
    class RequestInputFeed : public Node {
        RequestInputFeed(const Json& value, int):Node(value) {
        }
        public: static RequestInputFeed from_document(const Json& value) {
            return RequestInputFeed(value,0);
        }
        RequestInputFeed():Node(Json(Json::Object{
            {
                "kind",Json("feed")
            }
        })) {
        }
        RequestInputFeed& set_framing(const RequestFraming& value);
        RequestInputFeed& set_images(const std::vector<RequestImage>& value);
        RequestInputFeed& set_name(const std::string& value);
        RequestInputFeed& set_reading(const RequestReader& value);
    };
    class RequestInputJson : public Node {
        RequestInputJson(const Json& value, int):Node(value) {
        }
        public: static RequestInputJson from_document(const Json& value) {
            return RequestInputJson(value,0);
        }
        RequestInputJson():Node(Json(Json::Object{
            {
                "kind",Json("json")
            }
        })) {
        }
        RequestInputJson& set_images(const std::vector<RequestImage>& value);
        RequestInputJson& set_value(const Json& value);
    };
    class RequestInputRecords : public Node {
        RequestInputRecords(const Json& value, int):Node(value) {
        }
        public: static RequestInputRecords from_document(const Json& value) {
            return RequestInputRecords(value,0);
        }
        RequestInputRecords():Node(Json(Json::Object{
            {
                "kind",Json("records")
            }
        })) {
        }
        RequestInputRecords& set_items(const std::vector<RequestItem>& value);
    };
    class RequestInputSource : public Node {
        RequestInputSource(const Json& value, int):Node(value) {
        }
        public: static RequestInputSource from_document(const Json& value) {
            return RequestInputSource(value,0);
        }
        RequestInputSource():Node(Json(Json::Object{
            {
                "kind",Json("source")
            }
        })) {
        }
        RequestInputSource& set_source(const RequestSource& value);
    };
    class RequestInputText : public Node {
        RequestInputText(const Json& value, int):Node(value) {
        }
        public: static RequestInputText from_document(const Json& value) {
            return RequestInputText(value,0);
        }
        RequestInputText():Node(Json(Json::Object{
            {
                "kind",Json("text")
            }
        })) {
        }
        RequestInputText& set_images(const std::vector<RequestImage>& value);
        RequestInputText& set_text(const std::string& value);
    };
    class RequestInputUnits : public Node {
        RequestInputUnits(const Json& value, int):Node(value) {
        }
        public: static RequestInputUnits from_document(const Json& value) {
            return RequestInputUnits(value,0);
        }
        RequestInputUnits():Node(Json(Json::Object{
            {
                "kind",Json("units")
            }
        })) {
        }
        RequestInputUnits& set_items(const std::vector<RequestItem>& value);
    };
    class RequestItem : public Node {
        RequestItem(const Json& value, int):Node(value) {
        }
        public: static RequestItem from_document(const Json& value) {
            return RequestItem(value,0);
        }
        RequestItem():Node(Json(Json::Object{
        })) {
        }
        RequestItem& set_context(const ContextSchema& value);
        RequestItem& set_examples(const std::vector<RecognitionExample>& value);
        RequestItem& set_images(const std::vector<RequestImage>& value);
        RequestItem& set_options(const std::vector<OptionSchema>& value);
        RequestItem& set_original(const RequestOriginal& value);
        RequestItem& set_seed_spans(const std::vector<RecognitionSeedSpan>& value);
    };
    class RequestOptions : public Node {
        RequestOptions(const Json& value, int):Node(value) {
        }
        public: static RequestOptions from_document(const Json& value) {
            return RequestOptions(value,0);
        }
        RequestOptions():Node(Json(Json::Object{
        })) {
        }
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
    class RequestOriginal : public Node {
        RequestOriginal(const Json& value, int):Node(value) {
        }
        public: static RequestOriginal from_document(const Json& value) {
            return RequestOriginal(value,0);
        }
        RequestOriginal(const RequestOriginalText& value);
        RequestOriginal(const RequestOriginalJson& value);
    };
    class RequestOriginalJson : public Node {
        RequestOriginalJson(const Json& value, int):Node(value) {
        }
        public: static RequestOriginalJson from_document(const Json& value) {
            return RequestOriginalJson(value,0);
        }
        RequestOriginalJson():Node(Json(Json::Object{
            {
                "kind",Json("json")
            }
        })) {
        }
        RequestOriginalJson& set_value(const Json& value);
    };
    class RequestOriginalText : public Node {
        RequestOriginalText(const Json& value, int):Node(value) {
        }
        public: static RequestOriginalText from_document(const Json& value) {
            return RequestOriginalText(value,0);
        }
        RequestOriginalText():Node(Json(Json::Object{
            {
                "kind",Json("text")
            }
        })) {
        }
        RequestOriginalText& set_text(const std::string& value);
    };
    class RequestQuestion : public Node {
        RequestQuestion(const Json& value, int):Node(value) {
        }
        public: static RequestQuestion from_document(const Json& value) {
            return RequestQuestion(value,0);
        }
        RequestQuestion(const RequestQuestionText& value);
        RequestQuestion(const RequestQuestionDefinition& value);
        RequestQuestion(const RequestQuestionFile& value);
        RequestQuestion(const RequestQuestionName& value);
        RequestQuestion(const RequestQuestionReference& value);
    };
    class RequestQuestionDefinition : public Node {
        RequestQuestionDefinition(const Json& value, int):Node(value) {
        }
        public: static RequestQuestionDefinition from_document(const Json& value) {
            return RequestQuestionDefinition(value,0);
        }
        RequestQuestionDefinition():Node(Json(Json::Object{
            {
                "kind",Json("definition")
            }
        })) {
        }
        RequestQuestionDefinition& set_value(const RequestDefinition& value);
    };
    class RequestQuestionFile : public Node {
        RequestQuestionFile(const Json& value, int):Node(value) {
        }
        public: static RequestQuestionFile from_document(const Json& value) {
            return RequestQuestionFile(value,0);
        }
        RequestQuestionFile():Node(Json(Json::Object{
            {
                "kind",Json("file")
            }
        })) {
        }
        RequestQuestionFile& set_path(const std::string& value);
    };
    class RequestQuestionName : public Node {
        RequestQuestionName(const Json& value, int):Node(value) {
        }
        public: static RequestQuestionName from_document(const Json& value) {
            return RequestQuestionName(value,0);
        }
        RequestQuestionName():Node(Json(Json::Object{
            {
                "kind",Json("name")
            }
        })) {
        }
        RequestQuestionName& set_name(const std::string& value);
    };
    class RequestQuestionReference : public Node {
        RequestQuestionReference(const Json& value, int):Node(value) {
        }
        public: static RequestQuestionReference from_document(const Json& value) {
            return RequestQuestionReference(value,0);
        }
        RequestQuestionReference():Node(Json(Json::Object{
            {
                "kind",Json("reference")
            }
        })) {
        }
        RequestQuestionReference& set_reference(const std::string& value);
    };
    class RequestQuestionText : public Node {
        RequestQuestionText(const Json& value, int):Node(value) {
        }
        public: static RequestQuestionText from_document(const Json& value) {
            return RequestQuestionText(value,0);
        }
        RequestQuestionText():Node(Json(Json::Object{
            {
                "kind",Json("text")
            }
        })) {
        }
        RequestQuestionText& set_text(const std::string& value);
    };
    class RequestReader : public Node {
        RequestReader(const Json& value, int):Node(value) {
        }
        public: static RequestReader from_document(const Json& value) {
            return RequestReader(value,0);
        }
        RequestReader():Node(Json(Json::Object{
        })) {
        }
        RequestReader& set_unit(const SourceUnit& value);
        RequestReader& set_window(const uint64_t& value);
    };
    class RequestSessionDescriptor : public Node {
        RequestSessionDescriptor(const Json& value, int):Node(value) {
        }
        public: static RequestSessionDescriptor from_document(const Json& value) {
            return RequestSessionDescriptor(value,0);
        }
        RequestSessionDescriptor():Node(Json(Json::Object{
        })) {
        }
        RequestSessionDescriptor& set_item(const RequestItem& value);
        RequestSessionDescriptor& set_location(const SessionSourceLocation& value);
    };
    class RequestSource : public Node {
        RequestSource(const Json& value, int):Node(value) {
        }
        public: static RequestSource from_document(const Json& value) {
            return RequestSource(value,0);
        }
        RequestSource():Node(Json(Json::Object{
        })) {
        }
        RequestSource& set_framing(const RequestFraming& value);
        RequestSource& set_media(const ReaderMedia& value);
        RequestSource& set_paths(const std::vector<std::string>& value);
        RequestSource& set_reading(const RequestReader& value);
    };
    class RequestThreshold : public Node {
        RequestThreshold(const Json& value, int):Node(value) {
        }
        public: static RequestThreshold from_document(const Json& value) {
            return RequestThreshold(value,0);
        }
        explicit RequestThreshold(const std::variant<double, std::string>& value);
    };
    class RequestVersion : public Node {
        RequestVersion(const Json& value, int):Node(value) {
        }
        public: static RequestVersion from_document(const Json& value) {
            return RequestVersion(value,0);
        }
        explicit RequestVersion(const std::string& value);
    };
    class SessionSourceLocation : public Node {
        SessionSourceLocation(const Json& value, int):Node(value) {
        }
        public: static SessionSourceLocation from_document(const Json& value) {
            return SessionSourceLocation(value,0);
        }
        SessionSourceLocation():Node(Json(Json::Object{
        })) {
        }
        SessionSourceLocation& set_file(const std::string& value);
        SessionSourceLocation& set_first_line(const uint64_t& value);
        SessionSourceLocation& set_last_line(const uint64_t& value);
    };
    class SourceUnit : public Node {
        SourceUnit(const Json& value, int):Node(value) {
        }
        public: static SourceUnit from_document(const Json& value) {
            return SourceUnit(value,0);
        }
        explicit SourceUnit(const std::string& value);
    };
    inline AnnotateDefinition& AnnotateDefinition::set_batch(const Json& value) {
        set_member("batch",results::encode(value));
        return *this;
    }
    inline AnnotateDefinition& AnnotateDefinition::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::encode(value));
        return *this;
    }
    inline AnnotateDefinition& AnnotateDefinition::set_questions(const std::vector<std::pair<std::string,
    AnnotateQuestion>>& value) {
        set_member("questions",results::encode(value));
        return *this;
    }
    inline AnnotateDefinition& AnnotateDefinition::set_threshold(const AuthoredThreshold& value) {
        set_member("threshold",results::encode(value));
        return *this;
    }
    inline AnnotateQuestion::AnnotateQuestion(const AnnotateQuestionDecide& value):Node(value.document()) {
    }
    inline AnnotateQuestion::AnnotateQuestion(const AnnotateQuestionChoose& value):Node(value.document()) {
    }
    inline AnnotateQuestion::AnnotateQuestion(const AnnotateQuestionTag& value):Node(value.document()) {
    }
    inline AnnotateQuestion::AnnotateQuestion(const AnnotateQuestionScore& value):Node(value.document()) {
    }
    inline AnnotateQuestionChoose& AnnotateQuestionChoose::set_choose(const AuthoredQuestionText& value) {
        set_member("choose",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionChoose& AnnotateQuestionChoose::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionChoose& AnnotateQuestionChoose::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionChoose& AnnotateQuestionChoose::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionChoose& AnnotateQuestionChoose::set_on(const AuthoredPointers& value) {
        set_member("on",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionChoose& AnnotateQuestionChoose::set_options(const AuthoredOptions& value) {
        set_member("options",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionChoose& AnnotateQuestionChoose::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionChoose& AnnotateQuestionChoose::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionDecide& AnnotateQuestionDecide::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionDecide& AnnotateQuestionDecide::set_decide(const AuthoredQuestionText& value) {
        set_member("decide",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionDecide& AnnotateQuestionDecide::set_false(const AuthoredCriterion& value) {
        set_member("false",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionDecide& AnnotateQuestionDecide::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionDecide& AnnotateQuestionDecide::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionDecide& AnnotateQuestionDecide::set_on(const AuthoredPointers& value) {
        set_member("on",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionDecide& AnnotateQuestionDecide::set_threshold(const AuthoredThreshold& value) {
        set_member("threshold",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionDecide& AnnotateQuestionDecide::set_true(const AuthoredCriterion& value) {
        set_member("true",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionDecide& AnnotateQuestionDecide::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionScore& AnnotateQuestionScore::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionScore& AnnotateQuestionScore::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionScore& AnnotateQuestionScore::set_levels(const AuthoredLevels& value) {
        set_member("levels",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionScore& AnnotateQuestionScore::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionScore& AnnotateQuestionScore::set_on(const AuthoredPointers& value) {
        set_member("on",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionScore& AnnotateQuestionScore::set_score(const AuthoredQuestionText& value) {
        set_member("score",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionScore& AnnotateQuestionScore::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionTag& AnnotateQuestionTag::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionTag& AnnotateQuestionTag::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionTag& AnnotateQuestionTag::set_labels(const AuthoredLabels& value) {
        set_member("labels",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionTag& AnnotateQuestionTag::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionTag& AnnotateQuestionTag::set_on(const AuthoredPointers& value) {
        set_member("on",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionTag& AnnotateQuestionTag::set_tag(const AuthoredQuestionText& value) {
        set_member("tag",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionTag& AnnotateQuestionTag::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::encode(value));
        return *this;
    }
    inline AnnotateQuestionTag& AnnotateQuestionTag::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_batch(const std::variant<std::string, uint64_t>& value) {
        set_member("batch",results::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_choose(const AuthoredQuestionText& value) {
        set_member("choose",results::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_model(const AuthoredName& value) {
        set_member("model",results::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_on(const AuthoredPointers& value) {
        set_member("on",results::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_options(const AuthoredOptions& value) {
        set_member("options",results::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::encode(value));
        return *this;
    }
    inline AuthoredCriterion::AuthoredCriterion(const std::variant<std::string, Json, std::vector<Json>,
    std::nullptr_t>& value):Node(results::encode(value)) {
    }
    inline AuthoredCut::AuthoredCut(const std::variant<double, std::string>& value):Node(results::encode(value)) {
    }
    inline AuthoredDecide& AuthoredDecide::set_batch(const std::variant<std::string, uint64_t>& value) {
        set_member("batch",results::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_decide(const AuthoredQuestionText& value) {
        set_member("decide",results::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_false(const AuthoredCriterion& value) {
        set_member("false",results::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_model(const AuthoredName& value) {
        set_member("model",results::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_on(const AuthoredPointers& value) {
        set_member("on",results::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_threshold(const AuthoredThreshold& value) {
        set_member("threshold",results::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_true(const AuthoredCriterion& value) {
        set_member("true",results::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::encode(value));
        return *this;
    }
    inline AuthoredDescription::AuthoredDescription(const std::variant<std::string, Json, std::vector<Json>,
    std::nullptr_t>& value):Node(results::encode(value)) {
    }
    inline AuthoredFind& AuthoredFind::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::encode(value));
        return *this;
    }
    inline AuthoredFind& AuthoredFind::set_find(const AuthoredQuestionText& value) {
        set_member("find",results::encode(value));
        return *this;
    }
    inline AuthoredFind& AuthoredFind::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::encode(value));
        return *this;
    }
    inline AuthoredFind& AuthoredFind::set_model(const AuthoredName& value) {
        set_member("model",results::encode(value));
        return *this;
    }
    inline AuthoredFind& AuthoredFind::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline AuthoredFind& AuthoredFind::set_on(const AuthoredPointers& value) {
        set_member("on",results::encode(value));
        return *this;
    }
    inline AuthoredFind& AuthoredFind::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::encode(value));
        return *this;
    }
    inline AuthoredFind& AuthoredFind::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::encode(value));
        return *this;
    }
    inline AuthoredInputDeclaration::AuthoredInputDeclaration(const AuthoredInputDeclarationString& value):Node(value.document()) {
    }
    inline AuthoredInputDeclaration::AuthoredInputDeclaration(const AuthoredInputDeclarationObject& value):Node(value.document()) {
    }
    inline AuthoredInputDeclarationObject& AuthoredInputDeclarationObject::set_properties(const std::vector<std::pair<std::string,
    AuthoredInputProperty>>& value) {
        set_member("properties",results::encode(value));
        return *this;
    }
    inline AuthoredInputDeclarationObject& AuthoredInputDeclarationObject::set_required(const std::vector<std::string>& value) {
        set_member("required",results::encode(value));
        return *this;
    }
    inline AuthoredInputProperty::AuthoredInputProperty(const AuthoredInputPropertyString& value):Node(value.document()) {
    }
    inline AuthoredInputProperty::AuthoredInputProperty(const AuthoredInputPropertyNumber& value):Node(value.document()) {
    }
    inline AuthoredInputProperty::AuthoredInputProperty(const AuthoredInputPropertyBoolean& value):Node(value.document()) {
    }
    inline AuthoredInputProperty::AuthoredInputProperty(const AuthoredInputPropertyArray& value):Node(value.document()) {
    }
    inline AuthoredInputPropertyArray& AuthoredInputPropertyArray::set_items(const Json& value) {
        set_member("items",results::encode(value));
        return *this;
    }
    inline AuthoredLabels::AuthoredLabels(const std::variant<std::vector<AuthoredName>, std::vector<std::pair<std::string,
    AuthoredDescription>>>& value):Node(results::encode(value)) {
    }
    inline AuthoredLevels::AuthoredLevels(const std::variant<std::vector<AuthoredName>, std::vector<std::pair<std::string,
    AuthoredCriterion>>>& value):Node(results::encode(value)) {
    }
    inline AuthoredName::AuthoredName(const std::string& value):Node(results::encode(value)) {
    }
    inline AuthoredOptions::AuthoredOptions(const std::variant<std::vector<AuthoredName>, std::vector<std::pair<std::string,
    AuthoredDescription>>>& value):Node(results::encode(value)) {
    }
    inline AuthoredPointers::AuthoredPointers(const std::variant<std::string, std::vector<std::string>>& value):Node(results::encode(value)) {
    }
    inline AuthoredProfile::AuthoredProfile(const std::string& value):Node(results::encode(value)) {
    }
    inline AuthoredQuestionText::AuthoredQuestionText(const std::variant<std::string, Json, std::vector<Json>>& value):Node(results::encode(value)) {
    }
    inline AuthoredRelate& AuthoredRelate::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::encode(value));
        return *this;
    }
    inline AuthoredRelate& AuthoredRelate::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::encode(value));
        return *this;
    }
    inline AuthoredRelate& AuthoredRelate::set_model(const AuthoredName& value) {
        set_member("model",results::encode(value));
        return *this;
    }
    inline AuthoredRelate& AuthoredRelate::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline AuthoredRelate& AuthoredRelate::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::encode(value));
        return *this;
    }
    inline AuthoredRelate& AuthoredRelate::set_relate(const RelateQuestion& value) {
        set_member("relate",results::encode(value));
        return *this;
    }
    inline AuthoredRelate& AuthoredRelate::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::encode(value));
        return *this;
    }
    inline AuthoredRelate& AuthoredRelate::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::encode(value));
        return *this;
    }
    inline AuthoredRelation& AuthoredRelation::set_either(const bool& value) {
        set_member("either",results::encode(value));
        return *this;
    }
    inline AuthoredRelation& AuthoredRelation::set_name(const AuthoredName& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline AuthoredRelation& AuthoredRelation::set_reads(const AuthoredName& value) {
        set_member("reads",results::encode(value));
        return *this;
    }
    inline AuthoredRelation& AuthoredRelation::set_single(const bool& value) {
        set_member("single",results::encode(value));
        return *this;
    }
    inline AuthoredRelation& AuthoredRelation::set_source(const AuthoredName& value) {
        set_member("source",results::encode(value));
        return *this;
    }
    inline AuthoredRelation& AuthoredRelation::set_target(const AuthoredName& value) {
        set_member("target",results::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_batch(const std::variant<std::string, uint64_t>& value) {
        set_member("batch",results::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_levels(const AuthoredLevels& value) {
        set_member("levels",results::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_model(const AuthoredName& value) {
        set_member("model",results::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_on(const AuthoredPointers& value) {
        set_member("on",results::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_score(const AuthoredQuestionText& value) {
        set_member("score",results::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_batch(const std::variant<std::string, uint64_t>& value) {
        set_member("batch",results::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_labels(const AuthoredLabels& value) {
        set_member("labels",results::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_model(const AuthoredName& value) {
        set_member("model",results::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_on(const AuthoredPointers& value) {
        set_member("on",results::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_tag(const AuthoredQuestionText& value) {
        set_member("tag",results::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::encode(value));
        return *this;
    }
    inline AuthoredThreshold::AuthoredThreshold(const std::variant<double, std::string>& value):Node(results::encode(value)) {
    }
    inline ContextSchema::ContextSchema(const std::variant<std::string, Json>& value):Node(results::encode(value)) {
    }
    inline ImageMedia::ImageMedia(const std::string& value):Node(results::encode(value)) {
    }
    inline OptionSchema& OptionSchema::set_description(const Json& value) {
        set_member("description",results::encode(value));
        return *this;
    }
    inline OptionSchema& OptionSchema::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline ReaderMedia::ReaderMedia(const std::string& value):Node(results::encode(value)) {
    }
    inline RecognitionExample::RecognitionExample(const std::variant<std::string, RecognitionExampleText>& value):Node(results::encode(value)) {
    }
    inline RecognitionExampleEntity& RecognitionExampleEntity::set_end(const uint64_t& value) {
        set_member("end",results::encode(value));
        return *this;
    }
    inline RecognitionExampleEntity& RecognitionExampleEntity::set_kind(const std::string& value) {
        set_member("kind",results::encode(value));
        return *this;
    }
    inline RecognitionExampleEntity& RecognitionExampleEntity::set_start(const uint64_t& value) {
        set_member("start",results::encode(value));
        return *this;
    }
    inline RecognitionExampleText& RecognitionExampleText::set_entities(const std::vector<RecognitionExampleEntity>& value) {
        set_member("entities",results::encode(value));
        return *this;
    }
    inline RecognitionExampleText& RecognitionExampleText::set_kinds(const std::vector<std::string>& value) {
        set_member("kinds",results::encode(value));
        return *this;
    }
    inline RecognitionExampleText& RecognitionExampleText::set_text(const std::string& value) {
        set_member("text",results::encode(value));
        return *this;
    }
    inline RecognitionMode::RecognitionMode(const std::string& value):Node(results::encode(value)) {
    }
    inline RecognitionSeedSpan& RecognitionSeedSpan::set_end(const uint64_t& value) {
        set_member("end",results::encode(value));
        return *this;
    }
    inline RecognitionSeedSpan& RecognitionSeedSpan::set_kind(const std::string& value) {
        set_member("kind",results::encode(value));
        return *this;
    }
    inline RecognitionSeedSpan& RecognitionSeedSpan::set_start(const uint64_t& value) {
        set_member("start",results::encode(value));
        return *this;
    }
    inline RecognitionStageContext& RecognitionStageContext::set_boundary(const std::string& value) {
        set_member("boundary",results::encode(value));
        return *this;
    }
    inline RecognitionStageContext& RecognitionStageContext::set_kind_edge(const std::string& value) {
        set_member("kind_edge",results::encode(value));
        return *this;
    }
    inline RecognitionStageContext& RecognitionStageContext::set_relation(const std::string& value) {
        set_member("relation",results::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_model(const AuthoredName& value) {
        set_member("model",results::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_on(const AuthoredPointers& value) {
        set_member("on",results::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_recognize(const RecognizeQuestion& value) {
        set_member("recognize",results::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_relation_threshold(const AuthoredCut& value) {
        set_member("relation_threshold",results::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::encode(value));
        return *this;
    }
    inline RecognizeQuestion& RecognizeQuestion::set_entity_definition(const AuthoredQuestionText& value) {
        set_member("entity_definition",results::encode(value));
        return *this;
    }
    inline RecognizeQuestion& RecognizeQuestion::set_instructions(const AuthoredQuestionText& value) {
        set_member("instructions",results::encode(value));
        return *this;
    }
    inline RecognizeQuestion& RecognizeQuestion::set_kinds(const std::vector<std::pair<std::string,
    AuthoredDescription>>& value) {
        set_member("kinds",results::encode(value));
        return *this;
    }
    inline RecognizeQuestion& RecognizeQuestion::set_mode(const RecognitionMode& value) {
        set_member("mode",results::encode(value));
        return *this;
    }
    inline RecognizeQuestion& RecognizeQuestion::set_relations(const std::vector<AuthoredRelation>& value) {
        set_member("relations",results::encode(value));
        return *this;
    }
    inline RecognizeQuestion& RecognizeQuestion::set_snippet_pieces(const uint32_t& value) {
        set_member("snippet_pieces",results::encode(value));
        return *this;
    }
    inline RecognizeQuestion& RecognizeQuestion::set_stage_context(const RecognitionStageContext& value) {
        set_member("stage_context",results::encode(value));
        return *this;
    }
    inline RelateFields& RelateFields::set_kind(const std::string& value) {
        set_member("kind",results::encode(value));
        return *this;
    }
    inline RelateFields& RelateFields::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline RelateQuestion& RelateQuestion::set_fields(const RelateFields& value) {
        set_member("fields",results::encode(value));
        return *this;
    }
    inline RelateQuestion& RelateQuestion::set_relations(const std::vector<AuthoredRelation>& value) {
        set_member("relations",results::encode(value));
        return *this;
    }
    inline Request& Request::set_call(const RequestCall& value) {
        set_member("call",results::encode(value));
        return *this;
    }
    inline Request& Request::set_schema(const RequestVersion& value) {
        set_member("schema",results::encode(value));
        return *this;
    }
    inline RequestBatch::RequestBatch(const std::variant<uint64_t, std::string>& value):Node(results::encode(value)) {
    }
    inline RequestCall& RequestCall::set_function(const RequestFunction& value) {
        set_member("function",results::encode(value));
        return *this;
    }
    inline RequestCall& RequestCall::set_input(const RequestInput& value) {
        set_member("input",results::encode(value));
        return *this;
    }
    inline RequestCall& RequestCall::set_options(const RequestOptions& value) {
        set_member("options",results::encode(value));
        return *this;
    }
    inline RequestCall& RequestCall::set_question(const RequestQuestion& value) {
        set_member("question",results::encode(value));
        return *this;
    }
    inline RequestDefinition::RequestDefinition(const RequestDefinitionAuthoredDecide& value):Node(value.document()) {
    }
    inline RequestDefinition::RequestDefinition(const RequestDefinitionAuthoredChoose& value):Node(value.document()) {
    }
    inline RequestDefinition::RequestDefinition(const RequestDefinitionAuthoredTag& value):Node(value.document()) {
    }
    inline RequestDefinition::RequestDefinition(const RequestDefinitionAuthoredScore& value):Node(value.document()) {
    }
    inline RequestDefinition::RequestDefinition(const RequestDefinitionAuthoredRelate& value):Node(value.document()) {
    }
    inline RequestDefinition::RequestDefinition(const RequestDefinitionAuthoredFind& value):Node(value.document()) {
    }
    inline RequestDefinition::RequestDefinition(const RequestDefinitionRecognizeDefinition& value):Node(value.document()) {
    }
    inline RequestDefinition::RequestDefinition(const RequestDefinitionAnnotateDefinition& value):Node(value.document()) {
    }
    inline RequestDefinitionAnnotateDefinition& RequestDefinitionAnnotateDefinition::set_batch(const Json& value) {
        set_member("batch",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAnnotateDefinition& RequestDefinitionAnnotateDefinition::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAnnotateDefinition& RequestDefinitionAnnotateDefinition::set_questions(const std::vector<std::pair<std::string,
    AnnotateQuestion>>& value) {
        set_member("questions",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAnnotateDefinition& RequestDefinitionAnnotateDefinition::set_threshold(const AuthoredThreshold& value) {
        set_member("threshold",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_batch(const std::variant<std::string,
    uint64_t>& value) {
        set_member("batch",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_choose(const AuthoredQuestionText& value) {
        set_member("choose",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_model(const AuthoredName& value) {
        set_member("model",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_on(const AuthoredPointers& value) {
        set_member("on",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_options(const AuthoredOptions& value) {
        set_member("options",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_batch(const std::variant<std::string,
    uint64_t>& value) {
        set_member("batch",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_decide(const AuthoredQuestionText& value) {
        set_member("decide",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_false(const AuthoredCriterion& value) {
        set_member("false",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_model(const AuthoredName& value) {
        set_member("model",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_on(const AuthoredPointers& value) {
        set_member("on",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_threshold(const AuthoredThreshold& value) {
        set_member("threshold",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_true(const AuthoredCriterion& value) {
        set_member("true",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredFind& RequestDefinitionAuthoredFind::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredFind& RequestDefinitionAuthoredFind::set_find(const AuthoredQuestionText& value) {
        set_member("find",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredFind& RequestDefinitionAuthoredFind::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredFind& RequestDefinitionAuthoredFind::set_model(const AuthoredName& value) {
        set_member("model",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredFind& RequestDefinitionAuthoredFind::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredFind& RequestDefinitionAuthoredFind::set_on(const AuthoredPointers& value) {
        set_member("on",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredFind& RequestDefinitionAuthoredFind::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredFind& RequestDefinitionAuthoredFind::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredRelate& RequestDefinitionAuthoredRelate::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredRelate& RequestDefinitionAuthoredRelate::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredRelate& RequestDefinitionAuthoredRelate::set_model(const AuthoredName& value) {
        set_member("model",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredRelate& RequestDefinitionAuthoredRelate::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredRelate& RequestDefinitionAuthoredRelate::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredRelate& RequestDefinitionAuthoredRelate::set_relate(const RelateQuestion& value) {
        set_member("relate",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredRelate& RequestDefinitionAuthoredRelate::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredRelate& RequestDefinitionAuthoredRelate::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_batch(const std::variant<std::string,
    uint64_t>& value) {
        set_member("batch",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_levels(const AuthoredLevels& value) {
        set_member("levels",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_model(const AuthoredName& value) {
        set_member("model",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_on(const AuthoredPointers& value) {
        set_member("on",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_score(const AuthoredQuestionText& value) {
        set_member("score",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_batch(const std::variant<std::string,
    uint64_t>& value) {
        set_member("batch",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_labels(const AuthoredLabels& value) {
        set_member("labels",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_model(const AuthoredName& value) {
        set_member("model",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_on(const AuthoredPointers& value) {
        set_member("on",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_tag(const AuthoredQuestionText& value) {
        set_member("tag",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_model(const AuthoredName& value) {
        set_member("model",results::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_on(const AuthoredPointers& value) {
        set_member("on",results::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_recognize(const RecognizeQuestion& value) {
        set_member("recognize",results::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_relation_threshold(const AuthoredCut& value) {
        set_member("relation_threshold",results::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::encode(value));
        return *this;
    }
    inline RequestFraming::RequestFraming(const std::string& value):Node(results::encode(value)) {
    }
    inline RequestFunction::RequestFunction(const std::string& value):Node(results::encode(value)) {
    }
    inline RequestImage::RequestImage(const RequestImageFile& value):Node(value.document()) {
    }
    inline RequestImage::RequestImage(const RequestImageBytes& value):Node(value.document()) {
    }
    inline RequestImageBytes& RequestImageBytes::set_bytes(const std::string& value) {
        set_member("bytes",results::encode(value));
        return *this;
    }
    inline RequestImageBytes& RequestImageBytes::set_media(const ImageMedia& value) {
        set_member("media",results::encode(value));
        return *this;
    }
    inline RequestImageFile& RequestImageFile::set_media(const ImageMedia& value) {
        set_member("media",results::encode(value));
        return *this;
    }
    inline RequestImageFile& RequestImageFile::set_path(const std::string& value) {
        set_member("path",results::encode(value));
        return *this;
    }
    inline RequestInput::RequestInput(const RequestInputText& value):Node(value.document()) {
    }
    inline RequestInput::RequestInput(const RequestInputJson& value):Node(value.document()) {
    }
    inline RequestInput::RequestInput(const RequestInputRecords& value):Node(value.document()) {
    }
    inline RequestInput::RequestInput(const RequestInputUnits& value):Node(value.document()) {
    }
    inline RequestInput::RequestInput(const RequestInputEntities& value):Node(value.document()) {
    }
    inline RequestInput::RequestInput(const RequestInputSource& value):Node(value.document()) {
    }
    inline RequestInput::RequestInput(const RequestInputFeed& value):Node(value.document()) {
    }
    inline RequestInputEntities& RequestInputEntities::set_items(const std::vector<RequestItem>& value) {
        set_member("items",results::encode(value));
        return *this;
    }
    inline RequestInputFeed& RequestInputFeed::set_framing(const RequestFraming& value) {
        set_member("framing",results::encode(value));
        return *this;
    }
    inline RequestInputFeed& RequestInputFeed::set_images(const std::vector<RequestImage>& value) {
        set_member("images",results::encode(value));
        return *this;
    }
    inline RequestInputFeed& RequestInputFeed::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline RequestInputFeed& RequestInputFeed::set_reading(const RequestReader& value) {
        set_member("reading",results::encode(value));
        return *this;
    }
    inline RequestInputJson& RequestInputJson::set_images(const std::vector<RequestImage>& value) {
        set_member("images",results::encode(value));
        return *this;
    }
    inline RequestInputJson& RequestInputJson::set_value(const Json& value) {
        set_member("value",results::encode(value));
        return *this;
    }
    inline RequestInputRecords& RequestInputRecords::set_items(const std::vector<RequestItem>& value) {
        set_member("items",results::encode(value));
        return *this;
    }
    inline RequestInputSource& RequestInputSource::set_source(const RequestSource& value) {
        set_member("source",results::encode(value));
        return *this;
    }
    inline RequestInputText& RequestInputText::set_images(const std::vector<RequestImage>& value) {
        set_member("images",results::encode(value));
        return *this;
    }
    inline RequestInputText& RequestInputText::set_text(const std::string& value) {
        set_member("text",results::encode(value));
        return *this;
    }
    inline RequestInputUnits& RequestInputUnits::set_items(const std::vector<RequestItem>& value) {
        set_member("items",results::encode(value));
        return *this;
    }
    inline RequestItem& RequestItem::set_context(const ContextSchema& value) {
        set_member("context",results::encode(value));
        return *this;
    }
    inline RequestItem& RequestItem::set_examples(const std::vector<RecognitionExample>& value) {
        set_member("examples",results::encode(value));
        return *this;
    }
    inline RequestItem& RequestItem::set_images(const std::vector<RequestImage>& value) {
        set_member("images",results::encode(value));
        return *this;
    }
    inline RequestItem& RequestItem::set_options(const std::vector<OptionSchema>& value) {
        set_member("options",results::encode(value));
        return *this;
    }
    inline RequestItem& RequestItem::set_original(const RequestOriginal& value) {
        set_member("original",results::encode(value));
        return *this;
    }
    inline RequestItem& RequestItem::set_seed_spans(const std::vector<RecognitionSeedSpan>& value) {
        set_member("seed_spans",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_attempts(const bool& value) {
        set_member("attempts",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_batch(const RequestBatch& value) {
        set_member("batch",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_context(const std::string& value) {
        set_member("context",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_context_field(const std::string& value) {
        set_member("context_field",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_deadline_ms(const int64_t& value) {
        set_member("deadline_ms",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_details(const bool& value) {
        set_member("details",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_examples(const std::vector<RecognitionExample>& value) {
        set_member("examples",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_examples_field(const std::string& value) {
        set_member("examples_field",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_field(const std::vector<std::string>& value) {
        set_member("field",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_files_only(const bool& value) {
        set_member("files_only",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_max_requests_total(const uint64_t& value) {
        set_member("max_requests_total",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_mode(const RecognitionMode& value) {
        set_member("mode",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_model(const std::string& value) {
        set_member("model",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_none(const bool& value) {
        set_member("none",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_options_field(const std::string& value) {
        set_member("options_field",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_relation_threshold(const RequestThreshold& value) {
        set_member("relation_threshold",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_seed_spans(const std::vector<RecognitionSeedSpan>& value) {
        set_member("seed_spans",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_seed_spans_field(const std::string& value) {
        set_member("seed_spans_field",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_snippet_pieces(const uint32_t& value) {
        set_member("snippet_pieces",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_stage_context(const RecognitionStageContext& value) {
        set_member("stage_context",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_threshold(const RequestThreshold& value) {
        set_member("threshold",results::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_top(const uint64_t& value) {
        set_member("top",results::encode(value));
        return *this;
    }
    inline RequestOriginal::RequestOriginal(const RequestOriginalText& value):Node(value.document()) {
    }
    inline RequestOriginal::RequestOriginal(const RequestOriginalJson& value):Node(value.document()) {
    }
    inline RequestOriginalJson& RequestOriginalJson::set_value(const Json& value) {
        set_member("value",results::encode(value));
        return *this;
    }
    inline RequestOriginalText& RequestOriginalText::set_text(const std::string& value) {
        set_member("text",results::encode(value));
        return *this;
    }
    inline RequestQuestion::RequestQuestion(const RequestQuestionText& value):Node(value.document()) {
    }
    inline RequestQuestion::RequestQuestion(const RequestQuestionDefinition& value):Node(value.document()) {
    }
    inline RequestQuestion::RequestQuestion(const RequestQuestionFile& value):Node(value.document()) {
    }
    inline RequestQuestion::RequestQuestion(const RequestQuestionName& value):Node(value.document()) {
    }
    inline RequestQuestion::RequestQuestion(const RequestQuestionReference& value):Node(value.document()) {
    }
    inline RequestQuestionDefinition& RequestQuestionDefinition::set_value(const RequestDefinition& value) {
        set_member("value",results::encode(value));
        return *this;
    }
    inline RequestQuestionFile& RequestQuestionFile::set_path(const std::string& value) {
        set_member("path",results::encode(value));
        return *this;
    }
    inline RequestQuestionName& RequestQuestionName::set_name(const std::string& value) {
        set_member("name",results::encode(value));
        return *this;
    }
    inline RequestQuestionReference& RequestQuestionReference::set_reference(const std::string& value) {
        set_member("reference",results::encode(value));
        return *this;
    }
    inline RequestQuestionText& RequestQuestionText::set_text(const std::string& value) {
        set_member("text",results::encode(value));
        return *this;
    }
    inline RequestReader& RequestReader::set_unit(const SourceUnit& value) {
        set_member("unit",results::encode(value));
        return *this;
    }
    inline RequestReader& RequestReader::set_window(const uint64_t& value) {
        set_member("window",results::encode(value));
        return *this;
    }
    inline RequestSessionDescriptor& RequestSessionDescriptor::set_item(const RequestItem& value) {
        set_member("item",results::encode(value));
        return *this;
    }
    inline RequestSessionDescriptor& RequestSessionDescriptor::set_location(const SessionSourceLocation& value) {
        set_member("location",results::encode(value));
        return *this;
    }
    inline RequestSource& RequestSource::set_framing(const RequestFraming& value) {
        set_member("framing",results::encode(value));
        return *this;
    }
    inline RequestSource& RequestSource::set_media(const ReaderMedia& value) {
        set_member("media",results::encode(value));
        return *this;
    }
    inline RequestSource& RequestSource::set_paths(const std::vector<std::string>& value) {
        set_member("paths",results::encode(value));
        return *this;
    }
    inline RequestSource& RequestSource::set_reading(const RequestReader& value) {
        set_member("reading",results::encode(value));
        return *this;
    }
    inline RequestThreshold::RequestThreshold(const std::variant<double, std::string>& value):Node(results::encode(value)) {
    }
    inline RequestVersion::RequestVersion(const std::string& value):Node(results::encode(value)) {
    }
    inline SessionSourceLocation& SessionSourceLocation::set_file(const std::string& value) {
        set_member("file",results::encode(value));
        return *this;
    }
    inline SessionSourceLocation& SessionSourceLocation::set_first_line(const uint64_t& value) {
        set_member("first_line",results::encode(value));
        return *this;
    }
    inline SessionSourceLocation& SessionSourceLocation::set_last_line(const uint64_t& value) {
        set_member("last_line",results::encode(value));
        return *this;
    }
    inline SourceUnit::SourceUnit(const std::string& value):Node(results::encode(value)) {
    }
}
