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
    class AuthoredQuestionBatch;
    class AuthoredQuestionText;
    class AuthoredRelate;
    class AuthoredRelation;
    class AuthoredScore;
    class AuthoredTag;
    class AuthoredThreshold;
    class CacheDocument;
    class ContextSchema;
    class DisabledCache;
    class EngineSettings;
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
        AnnotateDefinition(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AnnotateDefinition():Node(Data(Data::Object{
            {
                "version",Data(1)
            }
        })) {
        }
        AnnotateDefinition& set_batch(const Data& value);
        AnnotateDefinition& set_profile(const AuthoredProfile& value);
        AnnotateDefinition& set_questions(const std::vector<std::pair<std::string, AnnotateQuestion>>& value);
        AnnotateDefinition& set_threshold(const AuthoredThreshold& value);
    };
    class AnnotateQuestion : public Node {
        AnnotateQuestion(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AnnotateQuestion(const AnnotateQuestionDecide& value);
        AnnotateQuestion(const AnnotateQuestionChoose& value);
        AnnotateQuestion(const AnnotateQuestionTag& value);
        AnnotateQuestion(const AnnotateQuestionScore& value);
    };
    class AnnotateQuestionChoose : public Node {
        AnnotateQuestionChoose(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AnnotateQuestionChoose():Node(Data(Data::Object{
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
        AnnotateQuestionDecide(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AnnotateQuestionDecide():Node(Data(Data::Object{
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
        AnnotateQuestionScore(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AnnotateQuestionScore():Node(Data(Data::Object{
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
        AnnotateQuestionTag(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AnnotateQuestionTag():Node(Data(Data::Object{
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
        AuthoredChoose(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AuthoredChoose():Node(Data(Data::Object{
        })) {
        }
        AuthoredChoose& set_batch(const AuthoredQuestionBatch& value);
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
        AuthoredCriterion(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit AuthoredCriterion(const std::variant<std::string, Data, std::vector<Data>, std::nullptr_t>& value);
    };
    class AuthoredCut : public Node {
        AuthoredCut(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit AuthoredCut(const std::variant<double, std::string>& value);
    };
    class AuthoredDecide : public Node {
        AuthoredDecide(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AuthoredDecide():Node(Data(Data::Object{
        })) {
        }
        AuthoredDecide& set_batch(const AuthoredQuestionBatch& value);
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
        AuthoredDescription(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit AuthoredDescription(const std::variant<std::string, Data, std::vector<Data>, std::nullptr_t>& value);
    };
    class AuthoredFind : public Node {
        AuthoredFind(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AuthoredFind():Node(Data(Data::Object{
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
        AuthoredInputDeclaration(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AuthoredInputDeclaration(const AuthoredInputDeclarationString& value);
        AuthoredInputDeclaration(const AuthoredInputDeclarationObject& value);
    };
    class AuthoredInputDeclarationObject : public Node {
        AuthoredInputDeclarationObject(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AuthoredInputDeclarationObject():Node(Data(Data::Object{
            {
                "type",Data("object")
            }
        })) {
        }
        AuthoredInputDeclarationObject& set_properties(const std::vector<std::pair<std::string,
        AuthoredInputProperty>>& value);
        AuthoredInputDeclarationObject& set_required(const std::vector<std::string>& value);
    };
    class AuthoredInputDeclarationString : public Node {
        AuthoredInputDeclarationString(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AuthoredInputDeclarationString():Node(Data(Data::Object{
            {
                "type",Data("string")
            }
        })) {
        }
    };
    class AuthoredInputProperty : public Node {
        AuthoredInputProperty(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AuthoredInputProperty(const AuthoredInputPropertyString& value);
        AuthoredInputProperty(const AuthoredInputPropertyNumber& value);
        AuthoredInputProperty(const AuthoredInputPropertyBoolean& value);
        AuthoredInputProperty(const AuthoredInputPropertyArray& value);
    };
    class AuthoredInputPropertyArray : public Node {
        AuthoredInputPropertyArray(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AuthoredInputPropertyArray():Node(Data(Data::Object{
            {
                "type",Data("array")
            }
        })) {
        }
        AuthoredInputPropertyArray& set_items(const Data& value);
    };
    class AuthoredInputPropertyBoolean : public Node {
        AuthoredInputPropertyBoolean(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AuthoredInputPropertyBoolean():Node(Data(Data::Object{
            {
                "type",Data("boolean")
            }
        })) {
        }
    };
    class AuthoredInputPropertyNumber : public Node {
        AuthoredInputPropertyNumber(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AuthoredInputPropertyNumber():Node(Data(Data::Object{
            {
                "type",Data("number")
            }
        })) {
        }
    };
    class AuthoredInputPropertyString : public Node {
        AuthoredInputPropertyString(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AuthoredInputPropertyString():Node(Data(Data::Object{
            {
                "type",Data("string")
            }
        })) {
        }
    };
    class AuthoredLabels : public Node {
        AuthoredLabels(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit AuthoredLabels(const std::variant<std::vector<AuthoredName>, std::vector<std::pair<std::string,
        AuthoredDescription>>>& value);
    };
    class AuthoredLevels : public Node {
        AuthoredLevels(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit AuthoredLevels(const std::variant<std::vector<AuthoredName>, std::vector<std::pair<std::string,
        AuthoredCriterion>>>& value);
    };
    class AuthoredName : public Node {
        AuthoredName(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit AuthoredName(const std::string& value);
    };
    class AuthoredOptions : public Node {
        AuthoredOptions(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit AuthoredOptions(const std::variant<std::vector<AuthoredName>, std::vector<std::pair<std::string,
        AuthoredDescription>>>& value);
    };
    class AuthoredPointers : public Node {
        AuthoredPointers(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit AuthoredPointers(const std::variant<std::string, std::vector<std::string>>& value);
    };
    class AuthoredProfile : public Node {
        AuthoredProfile(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit AuthoredProfile(const std::string& value);
    };
    class AuthoredQuestionBatch : public Node {
        AuthoredQuestionBatch(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit AuthoredQuestionBatch(const std::variant<std::string, uint64_t>& value);
    };
    class AuthoredQuestionText : public Node {
        AuthoredQuestionText(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit AuthoredQuestionText(const std::variant<std::string, Data, std::vector<Data>>& value);
    };
    class AuthoredRelate : public Node {
        AuthoredRelate(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AuthoredRelate():Node(Data(Data::Object{
            {
                "version",Data(1)
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
        AuthoredRelation(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AuthoredRelation():Node(Data(Data::Object{
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
        AuthoredScore(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AuthoredScore():Node(Data(Data::Object{
        })) {
        }
        AuthoredScore& set_batch(const AuthoredQuestionBatch& value);
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
        AuthoredTag(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        AuthoredTag():Node(Data(Data::Object{
        })) {
        }
        AuthoredTag& set_batch(const AuthoredQuestionBatch& value);
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
        AuthoredThreshold(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit AuthoredThreshold(const std::variant<double, std::string>& value);
    };
    class CacheDocument : public Node {
        CacheDocument(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit CacheDocument(const std::variant<std::string, DisabledCache>& value);
    };
    class ContextSchema : public Node {
        ContextSchema(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit ContextSchema(const std::variant<std::string, Data>& value);
    };
    class DisabledCache : public Node {
        DisabledCache(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit DisabledCache(const bool& value);
    };
    class EngineSettings : public Node {
        EngineSettings(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        EngineSettings():Node(Data(Data::Object{
        })) {
        }
        EngineSettings& set_backend(const std::string& value);
        EngineSettings& set_base_url(const std::string& value);
        EngineSettings& set_batch(const RequestBatch& value);
        EngineSettings& set_cache(const CacheDocument& value);
        EngineSettings& set_max_estimated_input_tokens_total(const std::variant<uint64_t, std::nullptr_t>& value);
        EngineSettings& set_max_request_bytes(const uint64_t& value);
        EngineSettings& set_max_requests(const std::variant<uint64_t, std::nullptr_t>& value);
        EngineSettings& set_max_requests_total(const std::variant<uint64_t, std::nullptr_t>& value);
        EngineSettings& set_max_retries(const uint64_t& value);
        EngineSettings& set_model(const std::string& value);
        EngineSettings& set_profile(const std::string& value);
        EngineSettings& set_proxy(const Data& value);
        EngineSettings& set_record(const std::string& value);
        EngineSettings& set_refresh_cache(const bool& value);
        EngineSettings& set_replay(const std::string& value);
        EngineSettings& set_throttle(const uint8_t& value);
        EngineSettings& set_timeout(const uint64_t& value);
        EngineSettings& set_usd_per_million_input(const std::string& value);
        EngineSettings& set_usd_per_million_output(const std::string& value);
    };
    class ImageMedia : public Node {
        ImageMedia(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit ImageMedia(const std::string& value);
    };
    class OptionSchema : public Node {
        OptionSchema(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        OptionSchema():Node(Data(Data::Object{
        })) {
        }
        OptionSchema& set_description(const Data& value);
        OptionSchema& set_name(const std::string& value);
    };
    class ReaderMedia : public Node {
        ReaderMedia(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit ReaderMedia(const std::string& value);
    };
    class RecognitionExample : public Node {
        RecognitionExample(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit RecognitionExample(const std::variant<std::string, RecognitionExampleText>& value);
    };
    class RecognitionExampleEntity : public Node {
        RecognitionExampleEntity(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RecognitionExampleEntity():Node(Data(Data::Object{
        })) {
        }
        RecognitionExampleEntity& set_end(const uint64_t& value);
        RecognitionExampleEntity& set_kind(const std::string& value);
        RecognitionExampleEntity& set_start(const uint64_t& value);
    };
    class RecognitionExampleText : public Node {
        RecognitionExampleText(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RecognitionExampleText():Node(Data(Data::Object{
        })) {
        }
        RecognitionExampleText& set_entities(const std::vector<RecognitionExampleEntity>& value);
        RecognitionExampleText& set_kinds(const std::vector<std::string>& value);
        RecognitionExampleText& set_text(const std::string& value);
    };
    class RecognitionMode : public Node {
        RecognitionMode(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit RecognitionMode(const std::string& value);
    };
    class RecognitionSeedSpan : public Node {
        RecognitionSeedSpan(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RecognitionSeedSpan():Node(Data(Data::Object{
        })) {
        }
        RecognitionSeedSpan& set_end(const uint64_t& value);
        RecognitionSeedSpan& set_kind(const std::string& value);
        RecognitionSeedSpan& set_start(const uint64_t& value);
    };
    class RecognitionStageContext : public Node {
        RecognitionStageContext(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RecognitionStageContext():Node(Data(Data::Object{
        })) {
        }
        RecognitionStageContext& set_boundary(const std::string& value);
        RecognitionStageContext& set_kind_edge(const std::string& value);
        RecognitionStageContext& set_relation(const std::string& value);
    };
    class RecognizeDefinition : public Node {
        RecognizeDefinition(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RecognizeDefinition():Node(Data(Data::Object{
            {
                "version",Data(1)
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
        RecognizeQuestion(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RecognizeQuestion():Node(Data(Data::Object{
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
        RelateFields(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RelateFields():Node(Data(Data::Object{
        })) {
        }
        RelateFields& set_kind(const std::string& value);
        RelateFields& set_name(const std::string& value);
    };
    class RelateQuestion : public Node {
        RelateQuestion(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RelateQuestion():Node(Data(Data::Object{
        })) {
        }
        RelateQuestion& set_fields(const RelateFields& value);
        RelateQuestion& set_relations(const std::vector<AuthoredRelation>& value);
    };
    class Request : public Node {
        Request(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        Request():Node(Data(Data::Object{
            {
                "schema",Data("thinkthen.request/1")
            }
        })) {
        }
        Request& set_call(const RequestCall& value);
        Request& set_schema(const RequestVersion& value);
    };
    class RequestBatch : public Node {
        RequestBatch(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit RequestBatch(const std::variant<uint64_t, std::string>& value);
    };
    class RequestCall : public Node {
        RequestCall(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestCall():Node(Data(Data::Object{
        })) {
        }
        RequestCall& set_function(const RequestFunction& value);
        RequestCall& set_input(const RequestInput& value);
        RequestCall& set_options(const RequestOptions& value);
        RequestCall& set_question(const RequestQuestion& value);
    };
    class RequestDefinition : public Node {
        RequestDefinition(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
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
        RequestDefinitionAnnotateDefinition(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestDefinitionAnnotateDefinition():Node(Data(Data::Object{
            {
                "version",Data(1)
            }
        })) {
        }
        RequestDefinitionAnnotateDefinition& set_batch(const Data& value);
        RequestDefinitionAnnotateDefinition& set_profile(const AuthoredProfile& value);
        RequestDefinitionAnnotateDefinition& set_questions(const std::vector<std::pair<std::string,
        AnnotateQuestion>>& value);
        RequestDefinitionAnnotateDefinition& set_threshold(const AuthoredThreshold& value);
    };
    class RequestDefinitionAuthoredChoose : public Node {
        RequestDefinitionAuthoredChoose(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestDefinitionAuthoredChoose():Node(Data(Data::Object{
        })) {
        }
        RequestDefinitionAuthoredChoose& set_batch(const AuthoredQuestionBatch& value);
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
        RequestDefinitionAuthoredDecide(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestDefinitionAuthoredDecide():Node(Data(Data::Object{
        })) {
        }
        RequestDefinitionAuthoredDecide& set_batch(const AuthoredQuestionBatch& value);
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
        RequestDefinitionAuthoredFind(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestDefinitionAuthoredFind():Node(Data(Data::Object{
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
        RequestDefinitionAuthoredRelate(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestDefinitionAuthoredRelate():Node(Data(Data::Object{
            {
                "version",Data(1)
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
        RequestDefinitionAuthoredScore(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestDefinitionAuthoredScore():Node(Data(Data::Object{
        })) {
        }
        RequestDefinitionAuthoredScore& set_batch(const AuthoredQuestionBatch& value);
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
        RequestDefinitionAuthoredTag(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestDefinitionAuthoredTag():Node(Data(Data::Object{
        })) {
        }
        RequestDefinitionAuthoredTag& set_batch(const AuthoredQuestionBatch& value);
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
        RequestDefinitionRecognizeDefinition(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestDefinitionRecognizeDefinition():Node(Data(Data::Object{
            {
                "version",Data(1)
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
        RequestFraming(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit RequestFraming(const std::string& value);
    };
    class RequestFunction : public Node {
        RequestFunction(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit RequestFunction(const std::string& value);
    };
    class RequestImage : public Node {
        RequestImage(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestImage(const RequestImageFile& value);
        RequestImage(const RequestImageBytes& value);
    };
    class RequestImageBytes : public Node {
        RequestImageBytes(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestImageBytes():Node(Data(Data::Object{
            {
                "kind",Data("bytes")
            }
        })) {
        }
        RequestImageBytes& set_bytes(const std::string& value);
        RequestImageBytes& set_media(const ImageMedia& value);
    };
    class RequestImageFile : public Node {
        RequestImageFile(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestImageFile():Node(Data(Data::Object{
            {
                "kind",Data("file")
            }
        })) {
        }
        RequestImageFile& set_media(const ImageMedia& value);
        RequestImageFile& set_path(const std::string& value);
    };
    class RequestInput : public Node {
        RequestInput(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestInput(const RequestInputText& value);
        RequestInput(const RequestInputJson& value);
        RequestInput(const RequestInputRecords& value);
        RequestInput(const RequestInputUnits& value);
        RequestInput(const RequestInputEntities& value);
        RequestInput(const RequestInputSource& value);
        RequestInput(const RequestInputFeed& value);
    };
    class RequestInputEntities : public Node {
        RequestInputEntities(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestInputEntities():Node(Data(Data::Object{
            {
                "kind",Data("entities")
            }
        })) {
        }
        RequestInputEntities& set_items(const std::vector<RequestItem>& value);
    };
    class RequestInputFeed : public Node {
        RequestInputFeed(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestInputFeed():Node(Data(Data::Object{
            {
                "kind",Data("feed")
            }
        })) {
        }
        RequestInputFeed& set_framing(const RequestFraming& value);
        RequestInputFeed& set_images(const std::vector<RequestImage>& value);
        RequestInputFeed& set_name(const std::string& value);
        RequestInputFeed& set_reading(const RequestReader& value);
    };
    class RequestInputJson : public Node {
        RequestInputJson(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestInputJson():Node(Data(Data::Object{
            {
                "kind",Data("json")
            }
        })) {
        }
        RequestInputJson& set_images(const std::vector<RequestImage>& value);
        RequestInputJson& set_value(const Data& value);
    };
    class RequestInputRecords : public Node {
        RequestInputRecords(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestInputRecords():Node(Data(Data::Object{
            {
                "kind",Data("records")
            }
        })) {
        }
        RequestInputRecords& set_items(const std::vector<RequestItem>& value);
    };
    class RequestInputSource : public Node {
        RequestInputSource(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestInputSource():Node(Data(Data::Object{
            {
                "kind",Data("source")
            }
        })) {
        }
        RequestInputSource& set_source(const RequestSource& value);
    };
    class RequestInputText : public Node {
        RequestInputText(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestInputText():Node(Data(Data::Object{
            {
                "kind",Data("text")
            }
        })) {
        }
        RequestInputText& set_images(const std::vector<RequestImage>& value);
        RequestInputText& set_text(const std::string& value);
    };
    class RequestInputUnits : public Node {
        RequestInputUnits(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestInputUnits():Node(Data(Data::Object{
            {
                "kind",Data("units")
            }
        })) {
        }
        RequestInputUnits& set_items(const std::vector<RequestItem>& value);
    };
    class RequestItem : public Node {
        RequestItem(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestItem():Node(Data(Data::Object{
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
        RequestOptions(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestOptions():Node(Data(Data::Object{
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
        RequestOriginal(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestOriginal(const RequestOriginalText& value);
        RequestOriginal(const RequestOriginalJson& value);
    };
    class RequestOriginalJson : public Node {
        RequestOriginalJson(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestOriginalJson():Node(Data(Data::Object{
            {
                "kind",Data("json")
            }
        })) {
        }
        RequestOriginalJson& set_value(const Data& value);
    };
    class RequestOriginalText : public Node {
        RequestOriginalText(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestOriginalText():Node(Data(Data::Object{
            {
                "kind",Data("text")
            }
        })) {
        }
        RequestOriginalText& set_text(const std::string& value);
    };
    class RequestQuestion : public Node {
        RequestQuestion(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestQuestion(const RequestQuestionText& value);
        RequestQuestion(const RequestQuestionDefinition& value);
        RequestQuestion(const RequestQuestionFile& value);
        RequestQuestion(const RequestQuestionName& value);
        RequestQuestion(const RequestQuestionReference& value);
    };
    class RequestQuestionDefinition : public Node {
        RequestQuestionDefinition(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestQuestionDefinition():Node(Data(Data::Object{
            {
                "kind",Data("definition")
            }
        })) {
        }
        RequestQuestionDefinition& set_value(const RequestDefinition& value);
    };
    class RequestQuestionFile : public Node {
        RequestQuestionFile(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestQuestionFile():Node(Data(Data::Object{
            {
                "kind",Data("file")
            }
        })) {
        }
        RequestQuestionFile& set_path(const std::string& value);
    };
    class RequestQuestionName : public Node {
        RequestQuestionName(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestQuestionName():Node(Data(Data::Object{
            {
                "kind",Data("name")
            }
        })) {
        }
        RequestQuestionName& set_name(const std::string& value);
    };
    class RequestQuestionReference : public Node {
        RequestQuestionReference(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestQuestionReference():Node(Data(Data::Object{
            {
                "kind",Data("reference")
            }
        })) {
        }
        RequestQuestionReference& set_reference(const std::string& value);
    };
    class RequestQuestionText : public Node {
        RequestQuestionText(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestQuestionText():Node(Data(Data::Object{
            {
                "kind",Data("text")
            }
        })) {
        }
        RequestQuestionText& set_text(const std::string& value);
    };
    class RequestReader : public Node {
        RequestReader(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestReader():Node(Data(Data::Object{
        })) {
        }
        RequestReader& set_unit(const SourceUnit& value);
        RequestReader& set_window(const uint64_t& value);
    };
    class RequestSessionDescriptor : public Node {
        RequestSessionDescriptor(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestSessionDescriptor():Node(Data(Data::Object{
        })) {
        }
        RequestSessionDescriptor& set_item(const RequestItem& value);
        RequestSessionDescriptor& set_location(const SessionSourceLocation& value);
    };
    class RequestSource : public Node {
        RequestSource(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        RequestSource():Node(Data(Data::Object{
        })) {
        }
        RequestSource& set_framing(const RequestFraming& value);
        RequestSource& set_media(const ReaderMedia& value);
        RequestSource& set_paths(const std::vector<std::string>& value);
        RequestSource& set_reading(const RequestReader& value);
    };
    class RequestThreshold : public Node {
        RequestThreshold(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit RequestThreshold(const std::variant<double, std::string>& value);
    };
    class RequestVersion : public Node {
        RequestVersion(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit RequestVersion(const std::string& value);
    };
    class SessionSourceLocation : public Node {
        SessionSourceLocation(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        SessionSourceLocation():Node(Data(Data::Object{
        })) {
        }
        SessionSourceLocation& set_file(const std::string& value);
        SessionSourceLocation& set_first_line(const uint64_t& value);
        SessionSourceLocation& set_last_line(const uint64_t& value);
    };
    class SourceUnit : public Node {
        SourceUnit(const Data& value, int):Node(value) {
        }
        friend struct ::tt::results::detail::Loader;
        public:
        explicit SourceUnit(const std::string& value);
    };
    inline AnnotateDefinition& AnnotateDefinition::set_batch(const Data& value) {
        set_member("batch",results::detail::encode(value));
        return *this;
    }
    inline AnnotateDefinition& AnnotateDefinition::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::detail::encode(value));
        return *this;
    }
    inline AnnotateDefinition& AnnotateDefinition::set_questions(const std::vector<std::pair<std::string,
    AnnotateQuestion>>& value) {
        set_member("questions",results::detail::encode(value));
        return *this;
    }
    inline AnnotateDefinition& AnnotateDefinition::set_threshold(const AuthoredThreshold& value) {
        set_member("threshold",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestion::AnnotateQuestion(const AnnotateQuestionDecide& value):Node(::tt::results::detail::document(value)) {
    }
    inline AnnotateQuestion::AnnotateQuestion(const AnnotateQuestionChoose& value):Node(::tt::results::detail::document(value)) {
    }
    inline AnnotateQuestion::AnnotateQuestion(const AnnotateQuestionTag& value):Node(::tt::results::detail::document(value)) {
    }
    inline AnnotateQuestion::AnnotateQuestion(const AnnotateQuestionScore& value):Node(::tt::results::detail::document(value)) {
    }
    inline AnnotateQuestionChoose& AnnotateQuestionChoose::set_choose(const AuthoredQuestionText& value) {
        set_member("choose",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionChoose& AnnotateQuestionChoose::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionChoose& AnnotateQuestionChoose::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionChoose& AnnotateQuestionChoose::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionChoose& AnnotateQuestionChoose::set_on(const AuthoredPointers& value) {
        set_member("on",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionChoose& AnnotateQuestionChoose::set_options(const AuthoredOptions& value) {
        set_member("options",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionChoose& AnnotateQuestionChoose::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionChoose& AnnotateQuestionChoose::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionDecide& AnnotateQuestionDecide::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionDecide& AnnotateQuestionDecide::set_decide(const AuthoredQuestionText& value) {
        set_member("decide",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionDecide& AnnotateQuestionDecide::set_false(const AuthoredCriterion& value) {
        set_member("false",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionDecide& AnnotateQuestionDecide::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionDecide& AnnotateQuestionDecide::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionDecide& AnnotateQuestionDecide::set_on(const AuthoredPointers& value) {
        set_member("on",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionDecide& AnnotateQuestionDecide::set_threshold(const AuthoredThreshold& value) {
        set_member("threshold",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionDecide& AnnotateQuestionDecide::set_true(const AuthoredCriterion& value) {
        set_member("true",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionDecide& AnnotateQuestionDecide::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionScore& AnnotateQuestionScore::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionScore& AnnotateQuestionScore::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionScore& AnnotateQuestionScore::set_levels(const AuthoredLevels& value) {
        set_member("levels",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionScore& AnnotateQuestionScore::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionScore& AnnotateQuestionScore::set_on(const AuthoredPointers& value) {
        set_member("on",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionScore& AnnotateQuestionScore::set_score(const AuthoredQuestionText& value) {
        set_member("score",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionScore& AnnotateQuestionScore::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionTag& AnnotateQuestionTag::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionTag& AnnotateQuestionTag::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionTag& AnnotateQuestionTag::set_labels(const AuthoredLabels& value) {
        set_member("labels",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionTag& AnnotateQuestionTag::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionTag& AnnotateQuestionTag::set_on(const AuthoredPointers& value) {
        set_member("on",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionTag& AnnotateQuestionTag::set_tag(const AuthoredQuestionText& value) {
        set_member("tag",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionTag& AnnotateQuestionTag::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::detail::encode(value));
        return *this;
    }
    inline AnnotateQuestionTag& AnnotateQuestionTag::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::detail::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_batch(const AuthoredQuestionBatch& value) {
        set_member("batch",results::detail::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_choose(const AuthoredQuestionText& value) {
        set_member("choose",results::detail::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::detail::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::detail::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_model(const AuthoredName& value) {
        set_member("model",results::detail::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_on(const AuthoredPointers& value) {
        set_member("on",results::detail::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_options(const AuthoredOptions& value) {
        set_member("options",results::detail::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::detail::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::detail::encode(value));
        return *this;
    }
    inline AuthoredChoose& AuthoredChoose::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::detail::encode(value));
        return *this;
    }
    inline AuthoredCriterion::AuthoredCriterion(const std::variant<std::string, Data, std::vector<Data>,
    std::nullptr_t>& value):Node(results::detail::encode(value)) {
    }
    inline AuthoredCut::AuthoredCut(const std::variant<double, std::string>& value):Node(results::detail::encode(value)) {
    }
    inline AuthoredDecide& AuthoredDecide::set_batch(const AuthoredQuestionBatch& value) {
        set_member("batch",results::detail::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::detail::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_decide(const AuthoredQuestionText& value) {
        set_member("decide",results::detail::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_false(const AuthoredCriterion& value) {
        set_member("false",results::detail::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::detail::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_model(const AuthoredName& value) {
        set_member("model",results::detail::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_on(const AuthoredPointers& value) {
        set_member("on",results::detail::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::detail::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_threshold(const AuthoredThreshold& value) {
        set_member("threshold",results::detail::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_true(const AuthoredCriterion& value) {
        set_member("true",results::detail::encode(value));
        return *this;
    }
    inline AuthoredDecide& AuthoredDecide::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::detail::encode(value));
        return *this;
    }
    inline AuthoredDescription::AuthoredDescription(const std::variant<std::string, Data, std::vector<Data>,
    std::nullptr_t>& value):Node(results::detail::encode(value)) {
    }
    inline AuthoredFind& AuthoredFind::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::detail::encode(value));
        return *this;
    }
    inline AuthoredFind& AuthoredFind::set_find(const AuthoredQuestionText& value) {
        set_member("find",results::detail::encode(value));
        return *this;
    }
    inline AuthoredFind& AuthoredFind::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::detail::encode(value));
        return *this;
    }
    inline AuthoredFind& AuthoredFind::set_model(const AuthoredName& value) {
        set_member("model",results::detail::encode(value));
        return *this;
    }
    inline AuthoredFind& AuthoredFind::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline AuthoredFind& AuthoredFind::set_on(const AuthoredPointers& value) {
        set_member("on",results::detail::encode(value));
        return *this;
    }
    inline AuthoredFind& AuthoredFind::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::detail::encode(value));
        return *this;
    }
    inline AuthoredFind& AuthoredFind::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::detail::encode(value));
        return *this;
    }
    inline AuthoredInputDeclaration::AuthoredInputDeclaration(const AuthoredInputDeclarationString& value):Node(::tt::results::detail::document(value)) {
    }
    inline AuthoredInputDeclaration::AuthoredInputDeclaration(const AuthoredInputDeclarationObject& value):Node(::tt::results::detail::document(value)) {
    }
    inline AuthoredInputDeclarationObject& AuthoredInputDeclarationObject::set_properties(const std::vector<std::pair<std::string,
    AuthoredInputProperty>>& value) {
        set_member("properties",results::detail::encode(value));
        return *this;
    }
    inline AuthoredInputDeclarationObject& AuthoredInputDeclarationObject::set_required(const std::vector<std::string>& value) {
        set_member("required",results::detail::encode(value));
        return *this;
    }
    inline AuthoredInputProperty::AuthoredInputProperty(const AuthoredInputPropertyString& value):Node(::tt::results::detail::document(value)) {
    }
    inline AuthoredInputProperty::AuthoredInputProperty(const AuthoredInputPropertyNumber& value):Node(::tt::results::detail::document(value)) {
    }
    inline AuthoredInputProperty::AuthoredInputProperty(const AuthoredInputPropertyBoolean& value):Node(::tt::results::detail::document(value)) {
    }
    inline AuthoredInputProperty::AuthoredInputProperty(const AuthoredInputPropertyArray& value):Node(::tt::results::detail::document(value)) {
    }
    inline AuthoredInputPropertyArray& AuthoredInputPropertyArray::set_items(const Data& value) {
        set_member("items",results::detail::encode(value));
        return *this;
    }
    inline AuthoredLabels::AuthoredLabels(const std::variant<std::vector<AuthoredName>, std::vector<std::pair<std::string,
    AuthoredDescription>>>& value):Node(results::detail::encode(value)) {
    }
    inline AuthoredLevels::AuthoredLevels(const std::variant<std::vector<AuthoredName>, std::vector<std::pair<std::string,
    AuthoredCriterion>>>& value):Node(results::detail::encode(value)) {
    }
    inline AuthoredName::AuthoredName(const std::string& value):Node(results::detail::encode(value)) {
    }
    inline AuthoredOptions::AuthoredOptions(const std::variant<std::vector<AuthoredName>, std::vector<std::pair<std::string,
    AuthoredDescription>>>& value):Node(results::detail::encode(value)) {
    }
    inline AuthoredPointers::AuthoredPointers(const std::variant<std::string, std::vector<std::string>>& value):Node(results::detail::encode(value)) {
    }
    inline AuthoredProfile::AuthoredProfile(const std::string& value):Node(results::detail::encode(value)) {
    }
    inline AuthoredQuestionBatch::AuthoredQuestionBatch(const std::variant<std::string, uint64_t>& value):Node(results::detail::encode(value)) {
    }
    inline AuthoredQuestionText::AuthoredQuestionText(const std::variant<std::string, Data, std::vector<Data>>& value):Node(results::detail::encode(value)) {
    }
    inline AuthoredRelate& AuthoredRelate::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::detail::encode(value));
        return *this;
    }
    inline AuthoredRelate& AuthoredRelate::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::detail::encode(value));
        return *this;
    }
    inline AuthoredRelate& AuthoredRelate::set_model(const AuthoredName& value) {
        set_member("model",results::detail::encode(value));
        return *this;
    }
    inline AuthoredRelate& AuthoredRelate::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline AuthoredRelate& AuthoredRelate::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::detail::encode(value));
        return *this;
    }
    inline AuthoredRelate& AuthoredRelate::set_relate(const RelateQuestion& value) {
        set_member("relate",results::detail::encode(value));
        return *this;
    }
    inline AuthoredRelate& AuthoredRelate::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::detail::encode(value));
        return *this;
    }
    inline AuthoredRelate& AuthoredRelate::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::detail::encode(value));
        return *this;
    }
    inline AuthoredRelation& AuthoredRelation::set_either(const bool& value) {
        set_member("either",results::detail::encode(value));
        return *this;
    }
    inline AuthoredRelation& AuthoredRelation::set_name(const AuthoredName& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline AuthoredRelation& AuthoredRelation::set_reads(const AuthoredName& value) {
        set_member("reads",results::detail::encode(value));
        return *this;
    }
    inline AuthoredRelation& AuthoredRelation::set_single(const bool& value) {
        set_member("single",results::detail::encode(value));
        return *this;
    }
    inline AuthoredRelation& AuthoredRelation::set_source(const AuthoredName& value) {
        set_member("source",results::detail::encode(value));
        return *this;
    }
    inline AuthoredRelation& AuthoredRelation::set_target(const AuthoredName& value) {
        set_member("target",results::detail::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_batch(const AuthoredQuestionBatch& value) {
        set_member("batch",results::detail::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::detail::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::detail::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_levels(const AuthoredLevels& value) {
        set_member("levels",results::detail::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_model(const AuthoredName& value) {
        set_member("model",results::detail::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_on(const AuthoredPointers& value) {
        set_member("on",results::detail::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::detail::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_score(const AuthoredQuestionText& value) {
        set_member("score",results::detail::encode(value));
        return *this;
    }
    inline AuthoredScore& AuthoredScore::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::detail::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_batch(const AuthoredQuestionBatch& value) {
        set_member("batch",results::detail::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::detail::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::detail::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_labels(const AuthoredLabels& value) {
        set_member("labels",results::detail::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_model(const AuthoredName& value) {
        set_member("model",results::detail::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_on(const AuthoredPointers& value) {
        set_member("on",results::detail::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::detail::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_tag(const AuthoredQuestionText& value) {
        set_member("tag",results::detail::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::detail::encode(value));
        return *this;
    }
    inline AuthoredTag& AuthoredTag::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::detail::encode(value));
        return *this;
    }
    inline AuthoredThreshold::AuthoredThreshold(const std::variant<double, std::string>& value):Node(results::detail::encode(value)) {
    }
    inline CacheDocument::CacheDocument(const std::variant<std::string, DisabledCache>& value):Node(results::detail::encode(value)) {
    }
    inline ContextSchema::ContextSchema(const std::variant<std::string, Data>& value):Node(results::detail::encode(value)) {
    }
    inline DisabledCache::DisabledCache(const bool& value):Node(results::detail::encode(value)) {
    }
    inline EngineSettings& EngineSettings::set_backend(const std::string& value) {
        set_member("backend",results::detail::encode(value));
        return *this;
    }
    inline EngineSettings& EngineSettings::set_base_url(const std::string& value) {
        set_member("base_url",results::detail::encode(value));
        return *this;
    }
    inline EngineSettings& EngineSettings::set_batch(const RequestBatch& value) {
        set_member("batch",results::detail::encode(value));
        return *this;
    }
    inline EngineSettings& EngineSettings::set_cache(const CacheDocument& value) {
        set_member("cache",results::detail::encode(value));
        return *this;
    }
    inline EngineSettings& EngineSettings::set_max_estimated_input_tokens_total(const std::variant<uint64_t,
    std::nullptr_t>& value) {
        set_member("max_estimated_input_tokens_total",results::detail::encode(value));
        return *this;
    }
    inline EngineSettings& EngineSettings::set_max_request_bytes(const uint64_t& value) {
        set_member("max_request_bytes",results::detail::encode(value));
        return *this;
    }
    inline EngineSettings& EngineSettings::set_max_requests(const std::variant<uint64_t, std::nullptr_t>& value) {
        set_member("max_requests",results::detail::encode(value));
        return *this;
    }
    inline EngineSettings& EngineSettings::set_max_requests_total(const std::variant<uint64_t, std::nullptr_t>& value) {
        set_member("max_requests_total",results::detail::encode(value));
        return *this;
    }
    inline EngineSettings& EngineSettings::set_max_retries(const uint64_t& value) {
        set_member("max_retries",results::detail::encode(value));
        return *this;
    }
    inline EngineSettings& EngineSettings::set_model(const std::string& value) {
        set_member("model",results::detail::encode(value));
        return *this;
    }
    inline EngineSettings& EngineSettings::set_profile(const std::string& value) {
        set_member("profile",results::detail::encode(value));
        return *this;
    }
    inline EngineSettings& EngineSettings::set_proxy(const Data& value) {
        set_member("proxy",results::detail::encode(value));
        return *this;
    }
    inline EngineSettings& EngineSettings::set_record(const std::string& value) {
        set_member("record",results::detail::encode(value));
        return *this;
    }
    inline EngineSettings& EngineSettings::set_refresh_cache(const bool& value) {
        set_member("refresh_cache",results::detail::encode(value));
        return *this;
    }
    inline EngineSettings& EngineSettings::set_replay(const std::string& value) {
        set_member("replay",results::detail::encode(value));
        return *this;
    }
    inline EngineSettings& EngineSettings::set_throttle(const uint8_t& value) {
        set_member("throttle",results::detail::encode(value));
        return *this;
    }
    inline EngineSettings& EngineSettings::set_timeout(const uint64_t& value) {
        set_member("timeout",results::detail::encode(value));
        return *this;
    }
    inline EngineSettings& EngineSettings::set_usd_per_million_input(const std::string& value) {
        set_member("usd_per_million_input",results::detail::encode(value));
        return *this;
    }
    inline EngineSettings& EngineSettings::set_usd_per_million_output(const std::string& value) {
        set_member("usd_per_million_output",results::detail::encode(value));
        return *this;
    }
    inline ImageMedia::ImageMedia(const std::string& value):Node(results::detail::encode(value)) {
    }
    inline OptionSchema& OptionSchema::set_description(const Data& value) {
        set_member("description",results::detail::encode(value));
        return *this;
    }
    inline OptionSchema& OptionSchema::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline ReaderMedia::ReaderMedia(const std::string& value):Node(results::detail::encode(value)) {
    }
    inline RecognitionExample::RecognitionExample(const std::variant<std::string, RecognitionExampleText>& value):Node(results::detail::encode(value)) {
    }
    inline RecognitionExampleEntity& RecognitionExampleEntity::set_end(const uint64_t& value) {
        set_member("end",results::detail::encode(value));
        return *this;
    }
    inline RecognitionExampleEntity& RecognitionExampleEntity::set_kind(const std::string& value) {
        set_member("kind",results::detail::encode(value));
        return *this;
    }
    inline RecognitionExampleEntity& RecognitionExampleEntity::set_start(const uint64_t& value) {
        set_member("start",results::detail::encode(value));
        return *this;
    }
    inline RecognitionExampleText& RecognitionExampleText::set_entities(const std::vector<RecognitionExampleEntity>& value) {
        set_member("entities",results::detail::encode(value));
        return *this;
    }
    inline RecognitionExampleText& RecognitionExampleText::set_kinds(const std::vector<std::string>& value) {
        set_member("kinds",results::detail::encode(value));
        return *this;
    }
    inline RecognitionExampleText& RecognitionExampleText::set_text(const std::string& value) {
        set_member("text",results::detail::encode(value));
        return *this;
    }
    inline RecognitionMode::RecognitionMode(const std::string& value):Node(results::detail::encode(value)) {
    }
    inline RecognitionSeedSpan& RecognitionSeedSpan::set_end(const uint64_t& value) {
        set_member("end",results::detail::encode(value));
        return *this;
    }
    inline RecognitionSeedSpan& RecognitionSeedSpan::set_kind(const std::string& value) {
        set_member("kind",results::detail::encode(value));
        return *this;
    }
    inline RecognitionSeedSpan& RecognitionSeedSpan::set_start(const uint64_t& value) {
        set_member("start",results::detail::encode(value));
        return *this;
    }
    inline RecognitionStageContext& RecognitionStageContext::set_boundary(const std::string& value) {
        set_member("boundary",results::detail::encode(value));
        return *this;
    }
    inline RecognitionStageContext& RecognitionStageContext::set_kind_edge(const std::string& value) {
        set_member("kind_edge",results::detail::encode(value));
        return *this;
    }
    inline RecognitionStageContext& RecognitionStageContext::set_relation(const std::string& value) {
        set_member("relation",results::detail::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::detail::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::detail::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_model(const AuthoredName& value) {
        set_member("model",results::detail::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_on(const AuthoredPointers& value) {
        set_member("on",results::detail::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::detail::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_recognize(const RecognizeQuestion& value) {
        set_member("recognize",results::detail::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_relation_threshold(const AuthoredCut& value) {
        set_member("relation_threshold",results::detail::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::detail::encode(value));
        return *this;
    }
    inline RecognizeDefinition& RecognizeDefinition::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::detail::encode(value));
        return *this;
    }
    inline RecognizeQuestion& RecognizeQuestion::set_entity_definition(const AuthoredQuestionText& value) {
        set_member("entity_definition",results::detail::encode(value));
        return *this;
    }
    inline RecognizeQuestion& RecognizeQuestion::set_instructions(const AuthoredQuestionText& value) {
        set_member("instructions",results::detail::encode(value));
        return *this;
    }
    inline RecognizeQuestion& RecognizeQuestion::set_kinds(const std::vector<std::pair<std::string,
    AuthoredDescription>>& value) {
        set_member("kinds",results::detail::encode(value));
        return *this;
    }
    inline RecognizeQuestion& RecognizeQuestion::set_mode(const RecognitionMode& value) {
        set_member("mode",results::detail::encode(value));
        return *this;
    }
    inline RecognizeQuestion& RecognizeQuestion::set_relations(const std::vector<AuthoredRelation>& value) {
        set_member("relations",results::detail::encode(value));
        return *this;
    }
    inline RecognizeQuestion& RecognizeQuestion::set_snippet_pieces(const uint32_t& value) {
        set_member("snippet_pieces",results::detail::encode(value));
        return *this;
    }
    inline RecognizeQuestion& RecognizeQuestion::set_stage_context(const RecognitionStageContext& value) {
        set_member("stage_context",results::detail::encode(value));
        return *this;
    }
    inline RelateFields& RelateFields::set_kind(const std::string& value) {
        set_member("kind",results::detail::encode(value));
        return *this;
    }
    inline RelateFields& RelateFields::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline RelateQuestion& RelateQuestion::set_fields(const RelateFields& value) {
        set_member("fields",results::detail::encode(value));
        return *this;
    }
    inline RelateQuestion& RelateQuestion::set_relations(const std::vector<AuthoredRelation>& value) {
        set_member("relations",results::detail::encode(value));
        return *this;
    }
    inline Request& Request::set_call(const RequestCall& value) {
        set_member("call",results::detail::encode(value));
        return *this;
    }
    inline Request& Request::set_schema(const RequestVersion& value) {
        set_member("schema",results::detail::encode(value));
        return *this;
    }
    inline RequestBatch::RequestBatch(const std::variant<uint64_t, std::string>& value):Node(results::detail::encode(value)) {
    }
    inline RequestCall& RequestCall::set_function(const RequestFunction& value) {
        set_member("function",results::detail::encode(value));
        return *this;
    }
    inline RequestCall& RequestCall::set_input(const RequestInput& value) {
        set_member("input",results::detail::encode(value));
        return *this;
    }
    inline RequestCall& RequestCall::set_options(const RequestOptions& value) {
        set_member("options",results::detail::encode(value));
        return *this;
    }
    inline RequestCall& RequestCall::set_question(const RequestQuestion& value) {
        set_member("question",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinition::RequestDefinition(const RequestDefinitionAuthoredDecide& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestDefinition::RequestDefinition(const RequestDefinitionAuthoredChoose& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestDefinition::RequestDefinition(const RequestDefinitionAuthoredTag& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestDefinition::RequestDefinition(const RequestDefinitionAuthoredScore& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestDefinition::RequestDefinition(const RequestDefinitionAuthoredRelate& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestDefinition::RequestDefinition(const RequestDefinitionAuthoredFind& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestDefinition::RequestDefinition(const RequestDefinitionRecognizeDefinition& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestDefinition::RequestDefinition(const RequestDefinitionAnnotateDefinition& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestDefinitionAnnotateDefinition& RequestDefinitionAnnotateDefinition::set_batch(const Data& value) {
        set_member("batch",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAnnotateDefinition& RequestDefinitionAnnotateDefinition::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAnnotateDefinition& RequestDefinitionAnnotateDefinition::set_questions(const std::vector<std::pair<std::string,
    AnnotateQuestion>>& value) {
        set_member("questions",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAnnotateDefinition& RequestDefinitionAnnotateDefinition::set_threshold(const AuthoredThreshold& value) {
        set_member("threshold",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_batch(const AuthoredQuestionBatch& value) {
        set_member("batch",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_choose(const AuthoredQuestionText& value) {
        set_member("choose",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_model(const AuthoredName& value) {
        set_member("model",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_on(const AuthoredPointers& value) {
        set_member("on",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_options(const AuthoredOptions& value) {
        set_member("options",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredChoose& RequestDefinitionAuthoredChoose::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_batch(const AuthoredQuestionBatch& value) {
        set_member("batch",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_decide(const AuthoredQuestionText& value) {
        set_member("decide",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_false(const AuthoredCriterion& value) {
        set_member("false",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_model(const AuthoredName& value) {
        set_member("model",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_on(const AuthoredPointers& value) {
        set_member("on",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_threshold(const AuthoredThreshold& value) {
        set_member("threshold",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_true(const AuthoredCriterion& value) {
        set_member("true",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredDecide& RequestDefinitionAuthoredDecide::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredFind& RequestDefinitionAuthoredFind::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredFind& RequestDefinitionAuthoredFind::set_find(const AuthoredQuestionText& value) {
        set_member("find",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredFind& RequestDefinitionAuthoredFind::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredFind& RequestDefinitionAuthoredFind::set_model(const AuthoredName& value) {
        set_member("model",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredFind& RequestDefinitionAuthoredFind::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredFind& RequestDefinitionAuthoredFind::set_on(const AuthoredPointers& value) {
        set_member("on",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredFind& RequestDefinitionAuthoredFind::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredFind& RequestDefinitionAuthoredFind::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredRelate& RequestDefinitionAuthoredRelate::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredRelate& RequestDefinitionAuthoredRelate::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredRelate& RequestDefinitionAuthoredRelate::set_model(const AuthoredName& value) {
        set_member("model",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredRelate& RequestDefinitionAuthoredRelate::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredRelate& RequestDefinitionAuthoredRelate::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredRelate& RequestDefinitionAuthoredRelate::set_relate(const RelateQuestion& value) {
        set_member("relate",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredRelate& RequestDefinitionAuthoredRelate::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredRelate& RequestDefinitionAuthoredRelate::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_batch(const AuthoredQuestionBatch& value) {
        set_member("batch",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_levels(const AuthoredLevels& value) {
        set_member("levels",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_model(const AuthoredName& value) {
        set_member("model",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_on(const AuthoredPointers& value) {
        set_member("on",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_score(const AuthoredQuestionText& value) {
        set_member("score",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredScore& RequestDefinitionAuthoredScore::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_batch(const AuthoredQuestionBatch& value) {
        set_member("batch",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_labels(const AuthoredLabels& value) {
        set_member("labels",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_model(const AuthoredName& value) {
        set_member("model",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_on(const AuthoredPointers& value) {
        set_member("on",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_tag(const AuthoredQuestionText& value) {
        set_member("tag",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionAuthoredTag& RequestDefinitionAuthoredTag::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_context_schema(const AuthoredInputDeclaration& value) {
        set_member("context_schema",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_item_schema(const AuthoredInputDeclaration& value) {
        set_member("item_schema",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_model(const AuthoredName& value) {
        set_member("model",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_on(const AuthoredPointers& value) {
        set_member("on",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_profile(const AuthoredProfile& value) {
        set_member("profile",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_recognize(const RecognizeQuestion& value) {
        set_member("recognize",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_relation_threshold(const AuthoredCut& value) {
        set_member("relation_threshold",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_threshold(const AuthoredCut& value) {
        set_member("threshold",results::detail::encode(value));
        return *this;
    }
    inline RequestDefinitionRecognizeDefinition& RequestDefinitionRecognizeDefinition::set_wording_version(const uint64_t& value) {
        set_member("wording_version",results::detail::encode(value));
        return *this;
    }
    inline RequestFraming::RequestFraming(const std::string& value):Node(results::detail::encode(value)) {
    }
    inline RequestFunction::RequestFunction(const std::string& value):Node(results::detail::encode(value)) {
    }
    inline RequestImage::RequestImage(const RequestImageFile& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestImage::RequestImage(const RequestImageBytes& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestImageBytes& RequestImageBytes::set_bytes(const std::string& value) {
        set_member("bytes",results::detail::encode(value));
        return *this;
    }
    inline RequestImageBytes& RequestImageBytes::set_media(const ImageMedia& value) {
        set_member("media",results::detail::encode(value));
        return *this;
    }
    inline RequestImageFile& RequestImageFile::set_media(const ImageMedia& value) {
        set_member("media",results::detail::encode(value));
        return *this;
    }
    inline RequestImageFile& RequestImageFile::set_path(const std::string& value) {
        set_member("path",results::detail::encode(value));
        return *this;
    }
    inline RequestInput::RequestInput(const RequestInputText& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestInput::RequestInput(const RequestInputJson& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestInput::RequestInput(const RequestInputRecords& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestInput::RequestInput(const RequestInputUnits& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestInput::RequestInput(const RequestInputEntities& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestInput::RequestInput(const RequestInputSource& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestInput::RequestInput(const RequestInputFeed& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestInputEntities& RequestInputEntities::set_items(const std::vector<RequestItem>& value) {
        set_member("items",results::detail::encode(value));
        return *this;
    }
    inline RequestInputFeed& RequestInputFeed::set_framing(const RequestFraming& value) {
        set_member("framing",results::detail::encode(value));
        return *this;
    }
    inline RequestInputFeed& RequestInputFeed::set_images(const std::vector<RequestImage>& value) {
        set_member("images",results::detail::encode(value));
        return *this;
    }
    inline RequestInputFeed& RequestInputFeed::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline RequestInputFeed& RequestInputFeed::set_reading(const RequestReader& value) {
        set_member("reading",results::detail::encode(value));
        return *this;
    }
    inline RequestInputJson& RequestInputJson::set_images(const std::vector<RequestImage>& value) {
        set_member("images",results::detail::encode(value));
        return *this;
    }
    inline RequestInputJson& RequestInputJson::set_value(const Data& value) {
        set_member("value",results::detail::encode(value));
        return *this;
    }
    inline RequestInputRecords& RequestInputRecords::set_items(const std::vector<RequestItem>& value) {
        set_member("items",results::detail::encode(value));
        return *this;
    }
    inline RequestInputSource& RequestInputSource::set_source(const RequestSource& value) {
        set_member("source",results::detail::encode(value));
        return *this;
    }
    inline RequestInputText& RequestInputText::set_images(const std::vector<RequestImage>& value) {
        set_member("images",results::detail::encode(value));
        return *this;
    }
    inline RequestInputText& RequestInputText::set_text(const std::string& value) {
        set_member("text",results::detail::encode(value));
        return *this;
    }
    inline RequestInputUnits& RequestInputUnits::set_items(const std::vector<RequestItem>& value) {
        set_member("items",results::detail::encode(value));
        return *this;
    }
    inline RequestItem& RequestItem::set_context(const ContextSchema& value) {
        set_member("context",results::detail::encode(value));
        return *this;
    }
    inline RequestItem& RequestItem::set_examples(const std::vector<RecognitionExample>& value) {
        set_member("examples",results::detail::encode(value));
        return *this;
    }
    inline RequestItem& RequestItem::set_images(const std::vector<RequestImage>& value) {
        set_member("images",results::detail::encode(value));
        return *this;
    }
    inline RequestItem& RequestItem::set_options(const std::vector<OptionSchema>& value) {
        set_member("options",results::detail::encode(value));
        return *this;
    }
    inline RequestItem& RequestItem::set_original(const RequestOriginal& value) {
        set_member("original",results::detail::encode(value));
        return *this;
    }
    inline RequestItem& RequestItem::set_seed_spans(const std::vector<RecognitionSeedSpan>& value) {
        set_member("seed_spans",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_attempts(const bool& value) {
        set_member("attempts",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_batch(const RequestBatch& value) {
        set_member("batch",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_context(const std::string& value) {
        set_member("context",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_context_field(const std::string& value) {
        set_member("context_field",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_deadline_ms(const int64_t& value) {
        set_member("deadline_ms",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_details(const bool& value) {
        set_member("details",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_examples(const std::vector<RecognitionExample>& value) {
        set_member("examples",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_examples_field(const std::string& value) {
        set_member("examples_field",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_field(const std::vector<std::string>& value) {
        set_member("field",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_files_only(const bool& value) {
        set_member("files_only",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_max_requests_total(const uint64_t& value) {
        set_member("max_requests_total",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_mode(const RecognitionMode& value) {
        set_member("mode",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_model(const std::string& value) {
        set_member("model",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_none(const bool& value) {
        set_member("none",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_options_field(const std::string& value) {
        set_member("options_field",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_relation_threshold(const RequestThreshold& value) {
        set_member("relation_threshold",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_seed_spans(const std::vector<RecognitionSeedSpan>& value) {
        set_member("seed_spans",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_seed_spans_field(const std::string& value) {
        set_member("seed_spans_field",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_snippet_pieces(const uint32_t& value) {
        set_member("snippet_pieces",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_stage_context(const RecognitionStageContext& value) {
        set_member("stage_context",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_threshold(const RequestThreshold& value) {
        set_member("threshold",results::detail::encode(value));
        return *this;
    }
    inline RequestOptions& RequestOptions::set_top(const uint64_t& value) {
        set_member("top",results::detail::encode(value));
        return *this;
    }
    inline RequestOriginal::RequestOriginal(const RequestOriginalText& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestOriginal::RequestOriginal(const RequestOriginalJson& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestOriginalJson& RequestOriginalJson::set_value(const Data& value) {
        set_member("value",results::detail::encode(value));
        return *this;
    }
    inline RequestOriginalText& RequestOriginalText::set_text(const std::string& value) {
        set_member("text",results::detail::encode(value));
        return *this;
    }
    inline RequestQuestion::RequestQuestion(const RequestQuestionText& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestQuestion::RequestQuestion(const RequestQuestionDefinition& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestQuestion::RequestQuestion(const RequestQuestionFile& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestQuestion::RequestQuestion(const RequestQuestionName& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestQuestion::RequestQuestion(const RequestQuestionReference& value):Node(::tt::results::detail::document(value)) {
    }
    inline RequestQuestionDefinition& RequestQuestionDefinition::set_value(const RequestDefinition& value) {
        set_member("value",results::detail::encode(value));
        return *this;
    }
    inline RequestQuestionFile& RequestQuestionFile::set_path(const std::string& value) {
        set_member("path",results::detail::encode(value));
        return *this;
    }
    inline RequestQuestionName& RequestQuestionName::set_name(const std::string& value) {
        set_member("name",results::detail::encode(value));
        return *this;
    }
    inline RequestQuestionReference& RequestQuestionReference::set_reference(const std::string& value) {
        set_member("reference",results::detail::encode(value));
        return *this;
    }
    inline RequestQuestionText& RequestQuestionText::set_text(const std::string& value) {
        set_member("text",results::detail::encode(value));
        return *this;
    }
    inline RequestReader& RequestReader::set_unit(const SourceUnit& value) {
        set_member("unit",results::detail::encode(value));
        return *this;
    }
    inline RequestReader& RequestReader::set_window(const uint64_t& value) {
        set_member("window",results::detail::encode(value));
        return *this;
    }
    inline RequestSessionDescriptor& RequestSessionDescriptor::set_item(const RequestItem& value) {
        set_member("item",results::detail::encode(value));
        return *this;
    }
    inline RequestSessionDescriptor& RequestSessionDescriptor::set_location(const SessionSourceLocation& value) {
        set_member("location",results::detail::encode(value));
        return *this;
    }
    inline RequestSource& RequestSource::set_framing(const RequestFraming& value) {
        set_member("framing",results::detail::encode(value));
        return *this;
    }
    inline RequestSource& RequestSource::set_media(const ReaderMedia& value) {
        set_member("media",results::detail::encode(value));
        return *this;
    }
    inline RequestSource& RequestSource::set_paths(const std::vector<std::string>& value) {
        set_member("paths",results::detail::encode(value));
        return *this;
    }
    inline RequestSource& RequestSource::set_reading(const RequestReader& value) {
        set_member("reading",results::detail::encode(value));
        return *this;
    }
    inline RequestThreshold::RequestThreshold(const std::variant<double, std::string>& value):Node(results::detail::encode(value)) {
    }
    inline RequestVersion::RequestVersion(const std::string& value):Node(results::detail::encode(value)) {
    }
    inline SessionSourceLocation& SessionSourceLocation::set_file(const std::string& value) {
        set_member("file",results::detail::encode(value));
        return *this;
    }
    inline SessionSourceLocation& SessionSourceLocation::set_first_line(const uint64_t& value) {
        set_member("first_line",results::detail::encode(value));
        return *this;
    }
    inline SessionSourceLocation& SessionSourceLocation::set_last_line(const uint64_t& value) {
        set_member("last_line",results::detail::encode(value));
        return *this;
    }
    inline SourceUnit::SourceUnit(const std::string& value):Node(results::detail::encode(value)) {
    }
    inline RequestQuestion question_file(const std::string& path) {
        return RequestQuestionFile().set_path(path);
    }
    inline RequestQuestion question_name(const std::string& name) {
        return RequestQuestionName().set_name(name);
    }
    inline RequestQuestion question_reference(const std::string& reference) {
        return RequestQuestionReference().set_reference(reference);
    }
    inline RequestItem item(const RequestOriginal& original) {
        return RequestItem().set_original(original);
    }
    inline RequestInput files(const std::vector<std::string>& paths, const RequestReader& reading={
    }) {
        return RequestInputSource().set_source(RequestSource().set_paths(paths).set_reading(reading));
    }
}
