// Generated from the shared Rust Request schema. Do not edit.
package thinkthen;
import java.util.*;
/** Typed wire builders. Native code owns grammar and admission errors. */
public final class Inputs {
    private Inputs() {
    }
    public abstract static class Builder<T extends Builder<T>> extends Values.Builder<T> {
    }
    public sealed interface AnnotateQuestion permits AnnotateQuestionDecide,AnnotateQuestionChoose,
    AnnotateQuestionTag,AnnotateQuestionScore {
        AnnotateQuestion contextSchema(AuthoredInputDeclaration value);
        AnnotateQuestion itemSchema(AuthoredInputDeclaration value);
        AnnotateQuestion name(String value);
        AnnotateQuestion on(AuthoredPointers value);
        AnnotateQuestion wordingVersion(Number value);
    }
    public static final class AnnotateQuestionChoose extends Builder<AnnotateQuestionChoose> implements AnnotateQuestion {
        public AnnotateQuestionChoose() {
        }
        static AnnotateQuestionChoose fromJson(Object value) {
            AnnotateQuestionChoose result = new AnnotateQuestionChoose();
            result.omit("choose");
            result.omit("context_schema");
            result.omit("item_schema");
            result.omit("name");
            result.omit("on");
            result.omit("options");
            result.omit("threshold");
            result.omit("wording_version");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected AnnotateQuestionChoose self() {
            return this;
        }
        public AnnotateQuestionChoose choose(AuthoredQuestionText value) {
            return put("choose", value);
        }
        public AnnotateQuestionChoose chooseNull() {
            return put("choose", null);
        }
        public AnnotateQuestionChoose omitChoose() {
            return omit("choose");
        }
        public AnnotateQuestionChoose contextSchema(AuthoredInputDeclaration value) {
            return put("context_schema", value);
        }
        public AnnotateQuestionChoose contextSchemaNull() {
            return put("context_schema", null);
        }
        public AnnotateQuestionChoose omitContextSchema() {
            return omit("context_schema");
        }
        public AnnotateQuestionChoose itemSchema(AuthoredInputDeclaration value) {
            return put("item_schema", value);
        }
        public AnnotateQuestionChoose itemSchemaNull() {
            return put("item_schema", null);
        }
        public AnnotateQuestionChoose omitItemSchema() {
            return omit("item_schema");
        }
        public AnnotateQuestionChoose name(String value) {
            return put("name", value);
        }
        public AnnotateQuestionChoose nameNull() {
            return put("name", null);
        }
        public AnnotateQuestionChoose omitName() {
            return omit("name");
        }
        public AnnotateQuestionChoose on(AuthoredPointers value) {
            return put("on", value);
        }
        public AnnotateQuestionChoose onNull() {
            return put("on", null);
        }
        public AnnotateQuestionChoose omitOn() {
            return omit("on");
        }
        public AnnotateQuestionChoose options(AuthoredOptions value) {
            return put("options", value);
        }
        public AnnotateQuestionChoose optionsNull() {
            return put("options", null);
        }
        public AnnotateQuestionChoose omitOptions() {
            return omit("options");
        }
        public AnnotateQuestionChoose threshold(AuthoredCut value) {
            return put("threshold", value);
        }
        public AnnotateQuestionChoose thresholdNull() {
            return put("threshold", null);
        }
        public AnnotateQuestionChoose omitThreshold() {
            return omit("threshold");
        }
        public AnnotateQuestionChoose wordingVersion(Number value) {
            return put("wording_version", value);
        }
        public AnnotateQuestionChoose wordingVersionNull() {
            return put("wording_version", null);
        }
        public AnnotateQuestionChoose omitWordingVersion() {
            return omit("wording_version");
        }
    }
    public static final class AnnotateQuestionDecide extends Builder<AnnotateQuestionDecide> implements AnnotateQuestion {
        public AnnotateQuestionDecide() {
        }
        static AnnotateQuestionDecide fromJson(Object value) {
            AnnotateQuestionDecide result = new AnnotateQuestionDecide();
            result.omit("context_schema");
            result.omit("decide");
            result.omit("false");
            result.omit("item_schema");
            result.omit("name");
            result.omit("on");
            result.omit("threshold");
            result.omit("true");
            result.omit("wording_version");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected AnnotateQuestionDecide self() {
            return this;
        }
        public AnnotateQuestionDecide contextSchema(AuthoredInputDeclaration value) {
            return put("context_schema", value);
        }
        public AnnotateQuestionDecide contextSchemaNull() {
            return put("context_schema", null);
        }
        public AnnotateQuestionDecide omitContextSchema() {
            return omit("context_schema");
        }
        public AnnotateQuestionDecide decide(AuthoredQuestionText value) {
            return put("decide", value);
        }
        public AnnotateQuestionDecide decideNull() {
            return put("decide", null);
        }
        public AnnotateQuestionDecide omitDecide() {
            return omit("decide");
        }
        public AnnotateQuestionDecide falseValue(AuthoredCriterion value) {
            return put("false", value);
        }
        public AnnotateQuestionDecide falseValueNull() {
            return put("false", null);
        }
        public AnnotateQuestionDecide omitFalseValue() {
            return omit("false");
        }
        public AnnotateQuestionDecide itemSchema(AuthoredInputDeclaration value) {
            return put("item_schema", value);
        }
        public AnnotateQuestionDecide itemSchemaNull() {
            return put("item_schema", null);
        }
        public AnnotateQuestionDecide omitItemSchema() {
            return omit("item_schema");
        }
        public AnnotateQuestionDecide name(String value) {
            return put("name", value);
        }
        public AnnotateQuestionDecide nameNull() {
            return put("name", null);
        }
        public AnnotateQuestionDecide omitName() {
            return omit("name");
        }
        public AnnotateQuestionDecide on(AuthoredPointers value) {
            return put("on", value);
        }
        public AnnotateQuestionDecide onNull() {
            return put("on", null);
        }
        public AnnotateQuestionDecide omitOn() {
            return omit("on");
        }
        public AnnotateQuestionDecide threshold(AuthoredThreshold value) {
            return put("threshold", value);
        }
        public AnnotateQuestionDecide thresholdNull() {
            return put("threshold", null);
        }
        public AnnotateQuestionDecide omitThreshold() {
            return omit("threshold");
        }
        public AnnotateQuestionDecide trueValue(AuthoredCriterion value) {
            return put("true", value);
        }
        public AnnotateQuestionDecide trueValueNull() {
            return put("true", null);
        }
        public AnnotateQuestionDecide omitTrueValue() {
            return omit("true");
        }
        public AnnotateQuestionDecide wordingVersion(Number value) {
            return put("wording_version", value);
        }
        public AnnotateQuestionDecide wordingVersionNull() {
            return put("wording_version", null);
        }
        public AnnotateQuestionDecide omitWordingVersion() {
            return omit("wording_version");
        }
    }
    public static final class AnnotateQuestionScore extends Builder<AnnotateQuestionScore> implements AnnotateQuestion {
        public AnnotateQuestionScore() {
        }
        static AnnotateQuestionScore fromJson(Object value) {
            AnnotateQuestionScore result = new AnnotateQuestionScore();
            result.omit("context_schema");
            result.omit("item_schema");
            result.omit("levels");
            result.omit("name");
            result.omit("on");
            result.omit("score");
            result.omit("wording_version");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected AnnotateQuestionScore self() {
            return this;
        }
        public AnnotateQuestionScore contextSchema(AuthoredInputDeclaration value) {
            return put("context_schema", value);
        }
        public AnnotateQuestionScore contextSchemaNull() {
            return put("context_schema", null);
        }
        public AnnotateQuestionScore omitContextSchema() {
            return omit("context_schema");
        }
        public AnnotateQuestionScore itemSchema(AuthoredInputDeclaration value) {
            return put("item_schema", value);
        }
        public AnnotateQuestionScore itemSchemaNull() {
            return put("item_schema", null);
        }
        public AnnotateQuestionScore omitItemSchema() {
            return omit("item_schema");
        }
        public AnnotateQuestionScore levels(AuthoredLevels value) {
            return put("levels", value);
        }
        public AnnotateQuestionScore levelsNull() {
            return put("levels", null);
        }
        public AnnotateQuestionScore omitLevels() {
            return omit("levels");
        }
        public AnnotateQuestionScore name(String value) {
            return put("name", value);
        }
        public AnnotateQuestionScore nameNull() {
            return put("name", null);
        }
        public AnnotateQuestionScore omitName() {
            return omit("name");
        }
        public AnnotateQuestionScore on(AuthoredPointers value) {
            return put("on", value);
        }
        public AnnotateQuestionScore onNull() {
            return put("on", null);
        }
        public AnnotateQuestionScore omitOn() {
            return omit("on");
        }
        public AnnotateQuestionScore score(AuthoredQuestionText value) {
            return put("score", value);
        }
        public AnnotateQuestionScore scoreNull() {
            return put("score", null);
        }
        public AnnotateQuestionScore omitScore() {
            return omit("score");
        }
        public AnnotateQuestionScore wordingVersion(Number value) {
            return put("wording_version", value);
        }
        public AnnotateQuestionScore wordingVersionNull() {
            return put("wording_version", null);
        }
        public AnnotateQuestionScore omitWordingVersion() {
            return omit("wording_version");
        }
    }
    public static final class AnnotateQuestionTag extends Builder<AnnotateQuestionTag> implements AnnotateQuestion {
        public AnnotateQuestionTag() {
        }
        static AnnotateQuestionTag fromJson(Object value) {
            AnnotateQuestionTag result = new AnnotateQuestionTag();
            result.omit("context_schema");
            result.omit("item_schema");
            result.omit("labels");
            result.omit("name");
            result.omit("on");
            result.omit("tag");
            result.omit("threshold");
            result.omit("wording_version");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected AnnotateQuestionTag self() {
            return this;
        }
        public AnnotateQuestionTag contextSchema(AuthoredInputDeclaration value) {
            return put("context_schema", value);
        }
        public AnnotateQuestionTag contextSchemaNull() {
            return put("context_schema", null);
        }
        public AnnotateQuestionTag omitContextSchema() {
            return omit("context_schema");
        }
        public AnnotateQuestionTag itemSchema(AuthoredInputDeclaration value) {
            return put("item_schema", value);
        }
        public AnnotateQuestionTag itemSchemaNull() {
            return put("item_schema", null);
        }
        public AnnotateQuestionTag omitItemSchema() {
            return omit("item_schema");
        }
        public AnnotateQuestionTag labels(AuthoredLabels value) {
            return put("labels", value);
        }
        public AnnotateQuestionTag labelsNull() {
            return put("labels", null);
        }
        public AnnotateQuestionTag omitLabels() {
            return omit("labels");
        }
        public AnnotateQuestionTag name(String value) {
            return put("name", value);
        }
        public AnnotateQuestionTag nameNull() {
            return put("name", null);
        }
        public AnnotateQuestionTag omitName() {
            return omit("name");
        }
        public AnnotateQuestionTag on(AuthoredPointers value) {
            return put("on", value);
        }
        public AnnotateQuestionTag onNull() {
            return put("on", null);
        }
        public AnnotateQuestionTag omitOn() {
            return omit("on");
        }
        public AnnotateQuestionTag tag(AuthoredQuestionText value) {
            return put("tag", value);
        }
        public AnnotateQuestionTag tagNull() {
            return put("tag", null);
        }
        public AnnotateQuestionTag omitTag() {
            return omit("tag");
        }
        public AnnotateQuestionTag threshold(AuthoredCut value) {
            return put("threshold", value);
        }
        public AnnotateQuestionTag thresholdNull() {
            return put("threshold", null);
        }
        public AnnotateQuestionTag omitThreshold() {
            return omit("threshold");
        }
        public AnnotateQuestionTag wordingVersion(Number value) {
            return put("wording_version", value);
        }
        public AnnotateQuestionTag wordingVersionNull() {
            return put("wording_version", null);
        }
        public AnnotateQuestionTag omitWordingVersion() {
            return omit("wording_version");
        }
    }
    public static final class AuthoredCriterion {
        private final Object value;
        public AuthoredCriterion(String value) {
            this.value = Values.freeze(value);
        }
        public AuthoredCriterion(Map<String,? extends Object> value) {
            this.value = Values.freeze(value);
        }
        public AuthoredCriterion(List<? extends Object> value) {
            this.value = Values.freeze(value);
        }
        Object json() {
            return value;
        }
    }
    public static final class AuthoredCut {
        private final Object value;
        public AuthoredCut(Number value) {
            this.value = Values.freeze(value);
        }
        public AuthoredCut(String value) {
            this.value = Values.freeze(value);
        }
        Object json() {
            return value;
        }
    }
    public static final class AuthoredDescription {
        private final Object value;
        public AuthoredDescription(String value) {
            this.value = Values.freeze(value);
        }
        public AuthoredDescription(Map<String,? extends Object> value) {
            this.value = Values.freeze(value);
        }
        public AuthoredDescription(List<? extends Object> value) {
            this.value = Values.freeze(value);
        }
        Object json() {
            return value;
        }
    }
    public sealed interface AuthoredInputDeclaration permits AuthoredInputDeclarationString,AuthoredInputDeclarationObject {
    }
    public static final class AuthoredInputDeclarationObject extends Builder<AuthoredInputDeclarationObject> implements AuthoredInputDeclaration {
        public AuthoredInputDeclarationObject() {
            put("type", "object");
        }
        static AuthoredInputDeclarationObject fromJson(Object value) {
            AuthoredInputDeclarationObject result = new AuthoredInputDeclarationObject();
            result.omit("properties");
            result.omit("required");
            result.omit("type");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected AuthoredInputDeclarationObject self() {
            return this;
        }
        public AuthoredInputDeclarationObject properties(Map<String,? extends AuthoredInputProperty> value) {
            return put("properties", value);
        }
        public AuthoredInputDeclarationObject propertiesNull() {
            return put("properties", null);
        }
        public AuthoredInputDeclarationObject omitProperties() {
            return omit("properties");
        }
        public AuthoredInputDeclarationObject required(List<? extends String> value) {
            return put("required", value);
        }
        public AuthoredInputDeclarationObject requiredNull() {
            return put("required", null);
        }
        public AuthoredInputDeclarationObject omitRequired() {
            return omit("required");
        }
    }
    public static final class AuthoredInputDeclarationString extends Builder<AuthoredInputDeclarationString> implements AuthoredInputDeclaration {
        public AuthoredInputDeclarationString() {
            put("type", "string");
        }
        static AuthoredInputDeclarationString fromJson(Object value) {
            AuthoredInputDeclarationString result = new AuthoredInputDeclarationString();
            result.omit("type");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected AuthoredInputDeclarationString self() {
            return this;
        }
    }
    public sealed interface AuthoredInputProperty permits AuthoredInputPropertyString,AuthoredInputPropertyNumber,
    AuthoredInputPropertyBoolean,AuthoredInputPropertyArray {
    }
    public static final class AuthoredInputPropertyArray extends Builder<AuthoredInputPropertyArray> implements AuthoredInputProperty {
        public AuthoredInputPropertyArray() {
            put("type", "array");
        }
        static AuthoredInputPropertyArray fromJson(Object value) {
            AuthoredInputPropertyArray result = new AuthoredInputPropertyArray();
            result.omit("items");
            result.omit("type");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected AuthoredInputPropertyArray self() {
            return this;
        }
        public AuthoredInputPropertyArray items(AuthoredInputPropertyArrayPropertiesItems value) {
            return put("items", value);
        }
        public AuthoredInputPropertyArray itemsNull() {
            return put("items", null);
        }
        public AuthoredInputPropertyArray omitItems() {
            return omit("items");
        }
    }
    public static final class AuthoredInputPropertyArrayPropertiesItems extends Builder<AuthoredInputPropertyArrayPropertiesItems> {
        public AuthoredInputPropertyArrayPropertiesItems() {
            put("type", "string");
        }
        static AuthoredInputPropertyArrayPropertiesItems fromJson(Object value) {
            AuthoredInputPropertyArrayPropertiesItems result = new AuthoredInputPropertyArrayPropertiesItems();
            result.omit("type");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected AuthoredInputPropertyArrayPropertiesItems self() {
            return this;
        }
    }
    public static final class AuthoredInputPropertyBoolean extends Builder<AuthoredInputPropertyBoolean> implements AuthoredInputProperty {
        public AuthoredInputPropertyBoolean() {
            put("type", "boolean");
        }
        static AuthoredInputPropertyBoolean fromJson(Object value) {
            AuthoredInputPropertyBoolean result = new AuthoredInputPropertyBoolean();
            result.omit("type");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected AuthoredInputPropertyBoolean self() {
            return this;
        }
    }
    public static final class AuthoredInputPropertyNumber extends Builder<AuthoredInputPropertyNumber> implements AuthoredInputProperty {
        public AuthoredInputPropertyNumber() {
            put("type", "number");
        }
        static AuthoredInputPropertyNumber fromJson(Object value) {
            AuthoredInputPropertyNumber result = new AuthoredInputPropertyNumber();
            result.omit("type");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected AuthoredInputPropertyNumber self() {
            return this;
        }
    }
    public static final class AuthoredInputPropertyString extends Builder<AuthoredInputPropertyString> implements AuthoredInputProperty {
        public AuthoredInputPropertyString() {
            put("type", "string");
        }
        static AuthoredInputPropertyString fromJson(Object value) {
            AuthoredInputPropertyString result = new AuthoredInputPropertyString();
            result.omit("type");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected AuthoredInputPropertyString self() {
            return this;
        }
    }
    public static final class AuthoredLabels {
        private final Object value;
        public AuthoredLabels(List<? extends String> value) {
            this.value = Values.freeze(value);
        }
        public AuthoredLabels(Map<String,? extends AuthoredDescription> value) {
            this.value = Values.freeze(value);
        }
        Object json() {
            return value;
        }
    }
    public static final class AuthoredLevels {
        private final Object value;
        public AuthoredLevels(List<? extends String> value) {
            this.value = Values.freeze(value);
        }
        public AuthoredLevels(Map<String,? extends AuthoredCriterion> value) {
            this.value = Values.freeze(value);
        }
        Object json() {
            return value;
        }
    }
    public static final class AuthoredOptions {
        private final Object value;
        public AuthoredOptions(List<? extends String> value) {
            this.value = Values.freeze(value);
        }
        public AuthoredOptions(Map<String,? extends AuthoredDescription> value) {
            this.value = Values.freeze(value);
        }
        Object json() {
            return value;
        }
    }
    public static final class AuthoredPointers {
        private final Object value;
        public AuthoredPointers(String value) {
            this.value = Values.freeze(value);
        }
        public AuthoredPointers(List<? extends String> value) {
            this.value = Values.freeze(value);
        }
        Object json() {
            return value;
        }
    }
    public static final class AuthoredQuestionText {
        private final Object value;
        public AuthoredQuestionText(String value) {
            this.value = Values.freeze(value);
        }
        public AuthoredQuestionText(Map<String,? extends Object> value) {
            this.value = Values.freeze(value);
        }
        public AuthoredQuestionText(List<? extends Object> value) {
            this.value = Values.freeze(value);
        }
        Object json() {
            return value;
        }
    }
    public static final class AuthoredRelation extends Builder<AuthoredRelation> {
        public AuthoredRelation() {
        }
        static AuthoredRelation fromJson(Object value) {
            AuthoredRelation result = new AuthoredRelation();
            result.omit("either");
            result.omit("name");
            result.omit("reads");
            result.omit("single");
            result.omit("source");
            result.omit("target");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected AuthoredRelation self() {
            return this;
        }
        public AuthoredRelation either(Boolean value) {
            return put("either", value);
        }
        public AuthoredRelation eitherNull() {
            return put("either", null);
        }
        public AuthoredRelation omitEither() {
            return omit("either");
        }
        public AuthoredRelation name(String value) {
            return put("name", value);
        }
        public AuthoredRelation nameNull() {
            return put("name", null);
        }
        public AuthoredRelation omitName() {
            return omit("name");
        }
        public AuthoredRelation reads(String value) {
            return put("reads", value);
        }
        public AuthoredRelation readsNull() {
            return put("reads", null);
        }
        public AuthoredRelation omitReads() {
            return omit("reads");
        }
        public AuthoredRelation single(Boolean value) {
            return put("single", value);
        }
        public AuthoredRelation singleNull() {
            return put("single", null);
        }
        public AuthoredRelation omitSingle() {
            return omit("single");
        }
        public AuthoredRelation source(String value) {
            return put("source", value);
        }
        public AuthoredRelation sourceNull() {
            return put("source", null);
        }
        public AuthoredRelation omitSource() {
            return omit("source");
        }
        public AuthoredRelation target(String value) {
            return put("target", value);
        }
        public AuthoredRelation targetNull() {
            return put("target", null);
        }
        public AuthoredRelation omitTarget() {
            return omit("target");
        }
    }
    public static final class AuthoredThreshold {
        private final Object value;
        public AuthoredThreshold(Number value) {
            this.value = Values.freeze(value);
        }
        public AuthoredThreshold(String value) {
            this.value = Values.freeze(value);
        }
        Object json() {
            return value;
        }
    }
    public static final class CacheDocument {
        private final Object value;
        public CacheDocument(String value) {
            this.value = Values.freeze(value);
        }
        public CacheDocument(Boolean value) {
            this.value = Values.freeze(value);
        }
        Object json() {
            return value;
        }
    }
    public static final class ContextSchema {
        private final Object value;
        public ContextSchema(String value) {
            this.value = Values.freeze(value);
        }
        public ContextSchema(Map<String,? extends Object> value) {
            this.value = Values.freeze(value);
        }
        Object json() {
            return value;
        }
    }
    public static final class EngineSettings extends Builder<EngineSettings> {
        public EngineSettings() {
        }
        static EngineSettings fromJson(Object value) {
            EngineSettings result = new EngineSettings();
            result.omit("backend");
            result.omit("base_url");
            result.omit("batch");
            result.omit("cache");
            result.omit("max_estimated_input_tokens_total");
            result.omit("max_request_bytes");
            result.omit("max_requests");
            result.omit("max_requests_total");
            result.omit("max_retries");
            result.omit("model");
            result.omit("profile");
            result.omit("proxy");
            result.omit("record");
            result.omit("refresh_cache");
            result.omit("replay");
            result.omit("throttle");
            result.omit("timeout");
            result.omit("usd_per_million_input");
            result.omit("usd_per_million_output");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected EngineSettings self() {
            return this;
        }
        public EngineSettings backend(String value) {
            return put("backend", value);
        }
        public EngineSettings backendNull() {
            return put("backend", null);
        }
        public EngineSettings omitBackend() {
            return omit("backend");
        }
        public EngineSettings baseUrl(String value) {
            return put("base_url", value);
        }
        public EngineSettings baseUrlNull() {
            return put("base_url", null);
        }
        public EngineSettings omitBaseUrl() {
            return omit("base_url");
        }
        public EngineSettings batch(RequestBatch value) {
            return put("batch", value);
        }
        public EngineSettings batchNull() {
            return put("batch", null);
        }
        public EngineSettings omitBatch() {
            return omit("batch");
        }
        public EngineSettings cache(CacheDocument value) {
            return put("cache", value);
        }
        public EngineSettings cacheNull() {
            return put("cache", null);
        }
        public EngineSettings omitCache() {
            return omit("cache");
        }
        public EngineSettings maxEstimatedInputTokensTotal(Number value) {
            return put("max_estimated_input_tokens_total", value);
        }
        public EngineSettings maxEstimatedInputTokensTotalNull() {
            return put("max_estimated_input_tokens_total", null);
        }
        public EngineSettings omitMaxEstimatedInputTokensTotal() {
            return omit("max_estimated_input_tokens_total");
        }
        public EngineSettings maxRequestBytes(Number value) {
            return put("max_request_bytes", value);
        }
        public EngineSettings maxRequestBytesNull() {
            return put("max_request_bytes", null);
        }
        public EngineSettings omitMaxRequestBytes() {
            return omit("max_request_bytes");
        }
        public EngineSettings maxRequests(Number value) {
            return put("max_requests", value);
        }
        public EngineSettings maxRequestsNull() {
            return put("max_requests", null);
        }
        public EngineSettings omitMaxRequests() {
            return omit("max_requests");
        }
        public EngineSettings maxRequestsTotal(Number value) {
            return put("max_requests_total", value);
        }
        public EngineSettings maxRequestsTotalNull() {
            return put("max_requests_total", null);
        }
        public EngineSettings omitMaxRequestsTotal() {
            return omit("max_requests_total");
        }
        public EngineSettings maxRetries(Number value) {
            return put("max_retries", value);
        }
        public EngineSettings maxRetriesNull() {
            return put("max_retries", null);
        }
        public EngineSettings omitMaxRetries() {
            return omit("max_retries");
        }
        public EngineSettings model(String value) {
            return put("model", value);
        }
        public EngineSettings modelNull() {
            return put("model", null);
        }
        public EngineSettings omitModel() {
            return omit("model");
        }
        public EngineSettings profile(String value) {
            return put("profile", value);
        }
        public EngineSettings profileNull() {
            return put("profile", null);
        }
        public EngineSettings omitProfile() {
            return omit("profile");
        }
        public EngineSettings proxy(Object value) {
            return put("proxy", value);
        }
        public EngineSettings proxyNull() {
            return put("proxy", null);
        }
        public EngineSettings omitProxy() {
            return omit("proxy");
        }
        public EngineSettings record(String value) {
            return put("record", value);
        }
        public EngineSettings recordNull() {
            return put("record", null);
        }
        public EngineSettings omitRecord() {
            return omit("record");
        }
        public EngineSettings refreshCache(Boolean value) {
            return put("refresh_cache", value);
        }
        public EngineSettings refreshCacheNull() {
            return put("refresh_cache", null);
        }
        public EngineSettings omitRefreshCache() {
            return omit("refresh_cache");
        }
        public EngineSettings replay(String value) {
            return put("replay", value);
        }
        public EngineSettings replayNull() {
            return put("replay", null);
        }
        public EngineSettings omitReplay() {
            return omit("replay");
        }
        public EngineSettings throttle(Number value) {
            return put("throttle", value);
        }
        public EngineSettings throttleNull() {
            return put("throttle", null);
        }
        public EngineSettings omitThrottle() {
            return omit("throttle");
        }
        public EngineSettings timeout(Number value) {
            return put("timeout", value);
        }
        public EngineSettings timeoutNull() {
            return put("timeout", null);
        }
        public EngineSettings omitTimeout() {
            return omit("timeout");
        }
        public EngineSettings usdPerMillionInput(String value) {
            return put("usd_per_million_input", value);
        }
        public EngineSettings usdPerMillionInputNull() {
            return put("usd_per_million_input", null);
        }
        public EngineSettings omitUsdPerMillionInput() {
            return omit("usd_per_million_input");
        }
        public EngineSettings usdPerMillionOutput(String value) {
            return put("usd_per_million_output", value);
        }
        public EngineSettings usdPerMillionOutputNull() {
            return put("usd_per_million_output", null);
        }
        public EngineSettings omitUsdPerMillionOutput() {
            return omit("usd_per_million_output");
        }
    }
    public enum ImageMedia {
        IMAGEJPEG("image/jpeg"),IMAGEPNG("image/png");
        private final String value;
        ImageMedia(String value) {
            this.value = value;
        }
        Object json() {
            return value;
        }
    }
    public static final class OptionSchema extends Builder<OptionSchema> {
        public OptionSchema() {
        }
        static OptionSchema fromJson(Object value) {
            OptionSchema result = new OptionSchema();
            result.omit("description");
            result.omit("name");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected OptionSchema self() {
            return this;
        }
        public OptionSchema description(Object value) {
            return put("description", value);
        }
        public OptionSchema descriptionNull() {
            return put("description", null);
        }
        public OptionSchema omitDescription() {
            return omit("description");
        }
        public OptionSchema name(String value) {
            return put("name", value);
        }
        public OptionSchema nameNull() {
            return put("name", null);
        }
        public OptionSchema omitName() {
            return omit("name");
        }
    }
    public enum ReaderMedia {
        TEXT("text"),IMAGE("image");
        private final String value;
        ReaderMedia(String value) {
            this.value = value;
        }
        Object json() {
            return value;
        }
    }
    public static final class RecognitionExample {
        private final Object value;
        public RecognitionExample(String value) {
            this.value = Values.freeze(value);
        }
        public RecognitionExample(RecognitionExampleText value) {
            this.value = Values.freeze(value);
        }
        Object json() {
            return value;
        }
    }
    public static final class RecognitionExampleEntity extends Builder<RecognitionExampleEntity> {
        public RecognitionExampleEntity() {
        }
        static RecognitionExampleEntity fromJson(Object value) {
            RecognitionExampleEntity result = new RecognitionExampleEntity();
            result.omit("end");
            result.omit("kind");
            result.omit("start");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RecognitionExampleEntity self() {
            return this;
        }
        public RecognitionExampleEntity end(Number value) {
            return put("end", value);
        }
        public RecognitionExampleEntity endNull() {
            return put("end", null);
        }
        public RecognitionExampleEntity omitEnd() {
            return omit("end");
        }
        public RecognitionExampleEntity kind(String value) {
            return put("kind", value);
        }
        public RecognitionExampleEntity kindNull() {
            return put("kind", null);
        }
        public RecognitionExampleEntity omitKind() {
            return omit("kind");
        }
        public RecognitionExampleEntity start(Number value) {
            return put("start", value);
        }
        public RecognitionExampleEntity startNull() {
            return put("start", null);
        }
        public RecognitionExampleEntity omitStart() {
            return omit("start");
        }
    }
    public static final class RecognitionExampleText extends Builder<RecognitionExampleText> {
        public RecognitionExampleText() {
        }
        static RecognitionExampleText fromJson(Object value) {
            RecognitionExampleText result = new RecognitionExampleText();
            result.omit("entities");
            result.omit("kinds");
            result.omit("text");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RecognitionExampleText self() {
            return this;
        }
        public RecognitionExampleText entities(List<? extends RecognitionExampleEntity> value) {
            return put("entities", value);
        }
        public RecognitionExampleText entitiesNull() {
            return put("entities", null);
        }
        public RecognitionExampleText omitEntities() {
            return omit("entities");
        }
        public RecognitionExampleText kinds(List<? extends String> value) {
            return put("kinds", value);
        }
        public RecognitionExampleText kindsNull() {
            return put("kinds", null);
        }
        public RecognitionExampleText omitKinds() {
            return omit("kinds");
        }
        public RecognitionExampleText text(String value) {
            return put("text", value);
        }
        public RecognitionExampleText textNull() {
            return put("text", null);
        }
        public RecognitionExampleText omitText() {
            return omit("text");
        }
    }
    public enum RecognitionMode {
        WHOLE("whole"),BOUNDARYONLY("boundary_only");
        private final String value;
        RecognitionMode(String value) {
            this.value = value;
        }
        Object json() {
            return value;
        }
    }
    public static final class RecognitionSeedSpan extends Builder<RecognitionSeedSpan> {
        public RecognitionSeedSpan() {
        }
        static RecognitionSeedSpan fromJson(Object value) {
            RecognitionSeedSpan result = new RecognitionSeedSpan();
            result.omit("end");
            result.omit("kind");
            result.omit("start");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RecognitionSeedSpan self() {
            return this;
        }
        public RecognitionSeedSpan end(Number value) {
            return put("end", value);
        }
        public RecognitionSeedSpan endNull() {
            return put("end", null);
        }
        public RecognitionSeedSpan omitEnd() {
            return omit("end");
        }
        public RecognitionSeedSpan kind(String value) {
            return put("kind", value);
        }
        public RecognitionSeedSpan kindNull() {
            return put("kind", null);
        }
        public RecognitionSeedSpan omitKind() {
            return omit("kind");
        }
        public RecognitionSeedSpan start(Number value) {
            return put("start", value);
        }
        public RecognitionSeedSpan startNull() {
            return put("start", null);
        }
        public RecognitionSeedSpan omitStart() {
            return omit("start");
        }
    }
    public static final class RecognitionStageContext extends Builder<RecognitionStageContext> {
        public RecognitionStageContext() {
        }
        static RecognitionStageContext fromJson(Object value) {
            RecognitionStageContext result = new RecognitionStageContext();
            result.omit("boundary");
            result.omit("kind_edge");
            result.omit("relation");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RecognitionStageContext self() {
            return this;
        }
        public RecognitionStageContext boundary(String value) {
            return put("boundary", value);
        }
        public RecognitionStageContext boundaryNull() {
            return put("boundary", null);
        }
        public RecognitionStageContext omitBoundary() {
            return omit("boundary");
        }
        public RecognitionStageContext kindEdge(String value) {
            return put("kind_edge", value);
        }
        public RecognitionStageContext kindEdgeNull() {
            return put("kind_edge", null);
        }
        public RecognitionStageContext omitKindEdge() {
            return omit("kind_edge");
        }
        public RecognitionStageContext relation(String value) {
            return put("relation", value);
        }
        public RecognitionStageContext relationNull() {
            return put("relation", null);
        }
        public RecognitionStageContext omitRelation() {
            return omit("relation");
        }
    }
    public static final class RecognizeQuestion extends Builder<RecognizeQuestion> {
        public RecognizeQuestion() {
        }
        static RecognizeQuestion fromJson(Object value) {
            RecognizeQuestion result = new RecognizeQuestion();
            result.omit("entity_definition");
            result.omit("instructions");
            result.omit("kinds");
            result.omit("mode");
            result.omit("relations");
            result.omit("snippet_pieces");
            result.omit("stage_context");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RecognizeQuestion self() {
            return this;
        }
        public RecognizeQuestion entityDefinition(AuthoredQuestionText value) {
            return put("entity_definition", value);
        }
        public RecognizeQuestion entityDefinitionNull() {
            return put("entity_definition", null);
        }
        public RecognizeQuestion omitEntityDefinition() {
            return omit("entity_definition");
        }
        public RecognizeQuestion instructions(AuthoredQuestionText value) {
            return put("instructions", value);
        }
        public RecognizeQuestion instructionsNull() {
            return put("instructions", null);
        }
        public RecognizeQuestion omitInstructions() {
            return omit("instructions");
        }
        public RecognizeQuestion kinds(Map<String,? extends AuthoredDescription> value) {
            return put("kinds", value);
        }
        public RecognizeQuestion kindsNull() {
            return put("kinds", null);
        }
        public RecognizeQuestion omitKinds() {
            return omit("kinds");
        }
        public RecognizeQuestion mode(RecognitionMode value) {
            return put("mode", value);
        }
        public RecognizeQuestion modeNull() {
            return put("mode", null);
        }
        public RecognizeQuestion omitMode() {
            return omit("mode");
        }
        public RecognizeQuestion relations(List<? extends AuthoredRelation> value) {
            return put("relations", value);
        }
        public RecognizeQuestion relationsNull() {
            return put("relations", null);
        }
        public RecognizeQuestion omitRelations() {
            return omit("relations");
        }
        public RecognizeQuestion snippetPieces(Number value) {
            return put("snippet_pieces", value);
        }
        public RecognizeQuestion snippetPiecesNull() {
            return put("snippet_pieces", null);
        }
        public RecognizeQuestion omitSnippetPieces() {
            return omit("snippet_pieces");
        }
        public RecognizeQuestion stageContext(RecognitionStageContext value) {
            return put("stage_context", value);
        }
        public RecognizeQuestion stageContextNull() {
            return put("stage_context", null);
        }
        public RecognizeQuestion omitStageContext() {
            return omit("stage_context");
        }
    }
    public static final class RelateFields extends Builder<RelateFields> {
        public RelateFields() {
        }
        static RelateFields fromJson(Object value) {
            RelateFields result = new RelateFields();
            result.omit("kind");
            result.omit("name");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RelateFields self() {
            return this;
        }
        public RelateFields kind(String value) {
            return put("kind", value);
        }
        public RelateFields kindNull() {
            return put("kind", null);
        }
        public RelateFields omitKind() {
            return omit("kind");
        }
        public RelateFields name(String value) {
            return put("name", value);
        }
        public RelateFields nameNull() {
            return put("name", null);
        }
        public RelateFields omitName() {
            return omit("name");
        }
    }
    public static final class RelateQuestion extends Builder<RelateQuestion> {
        public RelateQuestion() {
        }
        static RelateQuestion fromJson(Object value) {
            RelateQuestion result = new RelateQuestion();
            result.omit("fields");
            result.omit("relations");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RelateQuestion self() {
            return this;
        }
        public RelateQuestion fields(RelateFields value) {
            return put("fields", value);
        }
        public RelateQuestion fieldsNull() {
            return put("fields", null);
        }
        public RelateQuestion omitFields() {
            return omit("fields");
        }
        public RelateQuestion relations(List<? extends AuthoredRelation> value) {
            return put("relations", value);
        }
        public RelateQuestion relationsNull() {
            return put("relations", null);
        }
        public RelateQuestion omitRelations() {
            return omit("relations");
        }
    }
    public static final class Request extends Builder<Request> {
        public Request() {
            put("schema", "thinkthen.request/1");
        }
        static Request fromJson(Object value) {
            Request result = new Request();
            result.omit("call");
            result.omit("schema");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected Request self() {
            return this;
        }
        public Request call(RequestCall value) {
            return put("call", value);
        }
        public Request callNull() {
            return put("call", null);
        }
        public Request omitCall() {
            return omit("call");
        }
        public Request schema(RequestVersion value) {
            return put("schema", value);
        }
        public Request schemaNull() {
            return put("schema", null);
        }
        public Request omitSchema() {
            return omit("schema");
        }
    }
    public static final class RequestBatch {
        private final Object value;
        public RequestBatch(Number value) {
            this.value = Values.freeze(value);
        }
        public RequestBatch(String value) {
            this.value = Values.freeze(value);
        }
        Object json() {
            return value;
        }
    }
    public static final class RequestCall extends Builder<RequestCall> {
        public RequestCall() {
        }
        static RequestCall fromJson(Object value) {
            RequestCall result = new RequestCall();
            result.omit("function");
            result.omit("input");
            result.omit("options");
            result.omit("question");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestCall self() {
            return this;
        }
        public RequestCall function(RequestFunction value) {
            return put("function", value);
        }
        public RequestCall functionNull() {
            return put("function", null);
        }
        public RequestCall omitFunction() {
            return omit("function");
        }
        public RequestCall input(RequestInput value) {
            return put("input", value);
        }
        public RequestCall inputNull() {
            return put("input", null);
        }
        public RequestCall omitInput() {
            return omit("input");
        }
        public RequestCall options(RequestOptions value) {
            return put("options", value);
        }
        public RequestCall optionsNull() {
            return put("options", null);
        }
        public RequestCall omitOptions() {
            return omit("options");
        }
        public RequestCall question(RequestQuestion value) {
            return put("question", value);
        }
        public RequestCall questionNull() {
            return put("question", null);
        }
        public RequestCall omitQuestion() {
            return omit("question");
        }
    }
    public sealed interface RequestDefinition permits RequestDefinitionAuthoredDecide,RequestDefinitionAuthoredChoose,
    RequestDefinitionAuthoredTag,RequestDefinitionAuthoredScore,RequestDefinitionAuthoredRelate,
    RequestDefinitionAuthoredFind,RequestDefinitionRecognizeDefinition,RequestDefinitionAnnotateDefinition {
        RequestDefinition profile(String value);
    }
    public static final class RequestDefinitionAnnotateDefinition extends Builder<RequestDefinitionAnnotateDefinition> implements RequestDefinition {
        public RequestDefinitionAnnotateDefinition() {
            put("version", 1);
        }
        static RequestDefinitionAnnotateDefinition fromJson(Object value) {
            RequestDefinitionAnnotateDefinition result = new RequestDefinitionAnnotateDefinition();
            result.omit("batch");
            result.omit("profile");
            result.omit("questions");
            result.omit("threshold");
            result.omit("version");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestDefinitionAnnotateDefinition self() {
            return this;
        }
        public RequestDefinitionAnnotateDefinition batch(Object value) {
            return put("batch", value);
        }
        public RequestDefinitionAnnotateDefinition batchNull() {
            return put("batch", null);
        }
        public RequestDefinitionAnnotateDefinition omitBatch() {
            return omit("batch");
        }
        public RequestDefinitionAnnotateDefinition profile(String value) {
            return put("profile", value);
        }
        public RequestDefinitionAnnotateDefinition profileNull() {
            return put("profile", null);
        }
        public RequestDefinitionAnnotateDefinition omitProfile() {
            return omit("profile");
        }
        public RequestDefinitionAnnotateDefinition questions(Map<String,? extends AnnotateQuestion> value) {
            return put("questions", value);
        }
        public RequestDefinitionAnnotateDefinition questionsNull() {
            return put("questions", null);
        }
        public RequestDefinitionAnnotateDefinition omitQuestions() {
            return omit("questions");
        }
        public RequestDefinitionAnnotateDefinition threshold(AuthoredThreshold value) {
            return put("threshold", value);
        }
        public RequestDefinitionAnnotateDefinition thresholdNull() {
            return put("threshold", null);
        }
        public RequestDefinitionAnnotateDefinition omitThreshold() {
            return omit("threshold");
        }
    }
    public static final class RequestDefinitionAuthoredChoose extends Builder<RequestDefinitionAuthoredChoose> implements RequestDefinition {
        public RequestDefinitionAuthoredChoose() {
        }
        static RequestDefinitionAuthoredChoose fromJson(Object value) {
            RequestDefinitionAuthoredChoose result = new RequestDefinitionAuthoredChoose();
            result.omit("batch");
            result.omit("choose");
            result.omit("context_schema");
            result.omit("item_schema");
            result.omit("model");
            result.omit("name");
            result.omit("on");
            result.omit("options");
            result.omit("profile");
            result.omit("threshold");
            result.omit("wording_version");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestDefinitionAuthoredChoose self() {
            return this;
        }
        public RequestDefinitionAuthoredChoose batch(RequestDefinitionAuthoredChoosePropertiesBatch value) {
            return put("batch", value);
        }
        public RequestDefinitionAuthoredChoose batchNull() {
            return put("batch", null);
        }
        public RequestDefinitionAuthoredChoose omitBatch() {
            return omit("batch");
        }
        public RequestDefinitionAuthoredChoose choose(AuthoredQuestionText value) {
            return put("choose", value);
        }
        public RequestDefinitionAuthoredChoose chooseNull() {
            return put("choose", null);
        }
        public RequestDefinitionAuthoredChoose omitChoose() {
            return omit("choose");
        }
        public RequestDefinitionAuthoredChoose contextSchema(AuthoredInputDeclaration value) {
            return put("context_schema", value);
        }
        public RequestDefinitionAuthoredChoose contextSchemaNull() {
            return put("context_schema", null);
        }
        public RequestDefinitionAuthoredChoose omitContextSchema() {
            return omit("context_schema");
        }
        public RequestDefinitionAuthoredChoose itemSchema(AuthoredInputDeclaration value) {
            return put("item_schema", value);
        }
        public RequestDefinitionAuthoredChoose itemSchemaNull() {
            return put("item_schema", null);
        }
        public RequestDefinitionAuthoredChoose omitItemSchema() {
            return omit("item_schema");
        }
        public RequestDefinitionAuthoredChoose model(String value) {
            return put("model", value);
        }
        public RequestDefinitionAuthoredChoose modelNull() {
            return put("model", null);
        }
        public RequestDefinitionAuthoredChoose omitModel() {
            return omit("model");
        }
        public RequestDefinitionAuthoredChoose name(String value) {
            return put("name", value);
        }
        public RequestDefinitionAuthoredChoose nameNull() {
            return put("name", null);
        }
        public RequestDefinitionAuthoredChoose omitName() {
            return omit("name");
        }
        public RequestDefinitionAuthoredChoose on(AuthoredPointers value) {
            return put("on", value);
        }
        public RequestDefinitionAuthoredChoose onNull() {
            return put("on", null);
        }
        public RequestDefinitionAuthoredChoose omitOn() {
            return omit("on");
        }
        public RequestDefinitionAuthoredChoose options(AuthoredOptions value) {
            return put("options", value);
        }
        public RequestDefinitionAuthoredChoose optionsNull() {
            return put("options", null);
        }
        public RequestDefinitionAuthoredChoose omitOptions() {
            return omit("options");
        }
        public RequestDefinitionAuthoredChoose profile(String value) {
            return put("profile", value);
        }
        public RequestDefinitionAuthoredChoose profileNull() {
            return put("profile", null);
        }
        public RequestDefinitionAuthoredChoose omitProfile() {
            return omit("profile");
        }
        public RequestDefinitionAuthoredChoose threshold(AuthoredCut value) {
            return put("threshold", value);
        }
        public RequestDefinitionAuthoredChoose thresholdNull() {
            return put("threshold", null);
        }
        public RequestDefinitionAuthoredChoose omitThreshold() {
            return omit("threshold");
        }
        public RequestDefinitionAuthoredChoose wordingVersion(Number value) {
            return put("wording_version", value);
        }
        public RequestDefinitionAuthoredChoose wordingVersionNull() {
            return put("wording_version", null);
        }
        public RequestDefinitionAuthoredChoose omitWordingVersion() {
            return omit("wording_version");
        }
    }
    public static final class RequestDefinitionAuthoredChoosePropertiesBatch {
        private final Object value;
        public RequestDefinitionAuthoredChoosePropertiesBatch(String value) {
            this.value = Values.freeze(value);
        }
        public RequestDefinitionAuthoredChoosePropertiesBatch(Number value) {
            this.value = Values.freeze(value);
        }
        Object json() {
            return value;
        }
    }
    public static final class RequestDefinitionAuthoredDecide extends Builder<RequestDefinitionAuthoredDecide> implements RequestDefinition {
        public RequestDefinitionAuthoredDecide() {
        }
        static RequestDefinitionAuthoredDecide fromJson(Object value) {
            RequestDefinitionAuthoredDecide result = new RequestDefinitionAuthoredDecide();
            result.omit("batch");
            result.omit("context_schema");
            result.omit("decide");
            result.omit("false");
            result.omit("item_schema");
            result.omit("model");
            result.omit("name");
            result.omit("on");
            result.omit("profile");
            result.omit("threshold");
            result.omit("true");
            result.omit("wording_version");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestDefinitionAuthoredDecide self() {
            return this;
        }
        public RequestDefinitionAuthoredDecide batch(RequestDefinitionAuthoredDecidePropertiesBatch value) {
            return put("batch", value);
        }
        public RequestDefinitionAuthoredDecide batchNull() {
            return put("batch", null);
        }
        public RequestDefinitionAuthoredDecide omitBatch() {
            return omit("batch");
        }
        public RequestDefinitionAuthoredDecide contextSchema(AuthoredInputDeclaration value) {
            return put("context_schema", value);
        }
        public RequestDefinitionAuthoredDecide contextSchemaNull() {
            return put("context_schema", null);
        }
        public RequestDefinitionAuthoredDecide omitContextSchema() {
            return omit("context_schema");
        }
        public RequestDefinitionAuthoredDecide decide(AuthoredQuestionText value) {
            return put("decide", value);
        }
        public RequestDefinitionAuthoredDecide decideNull() {
            return put("decide", null);
        }
        public RequestDefinitionAuthoredDecide omitDecide() {
            return omit("decide");
        }
        public RequestDefinitionAuthoredDecide falseValue(AuthoredCriterion value) {
            return put("false", value);
        }
        public RequestDefinitionAuthoredDecide falseValueNull() {
            return put("false", null);
        }
        public RequestDefinitionAuthoredDecide omitFalseValue() {
            return omit("false");
        }
        public RequestDefinitionAuthoredDecide itemSchema(AuthoredInputDeclaration value) {
            return put("item_schema", value);
        }
        public RequestDefinitionAuthoredDecide itemSchemaNull() {
            return put("item_schema", null);
        }
        public RequestDefinitionAuthoredDecide omitItemSchema() {
            return omit("item_schema");
        }
        public RequestDefinitionAuthoredDecide model(String value) {
            return put("model", value);
        }
        public RequestDefinitionAuthoredDecide modelNull() {
            return put("model", null);
        }
        public RequestDefinitionAuthoredDecide omitModel() {
            return omit("model");
        }
        public RequestDefinitionAuthoredDecide name(String value) {
            return put("name", value);
        }
        public RequestDefinitionAuthoredDecide nameNull() {
            return put("name", null);
        }
        public RequestDefinitionAuthoredDecide omitName() {
            return omit("name");
        }
        public RequestDefinitionAuthoredDecide on(AuthoredPointers value) {
            return put("on", value);
        }
        public RequestDefinitionAuthoredDecide onNull() {
            return put("on", null);
        }
        public RequestDefinitionAuthoredDecide omitOn() {
            return omit("on");
        }
        public RequestDefinitionAuthoredDecide profile(String value) {
            return put("profile", value);
        }
        public RequestDefinitionAuthoredDecide profileNull() {
            return put("profile", null);
        }
        public RequestDefinitionAuthoredDecide omitProfile() {
            return omit("profile");
        }
        public RequestDefinitionAuthoredDecide threshold(AuthoredThreshold value) {
            return put("threshold", value);
        }
        public RequestDefinitionAuthoredDecide thresholdNull() {
            return put("threshold", null);
        }
        public RequestDefinitionAuthoredDecide omitThreshold() {
            return omit("threshold");
        }
        public RequestDefinitionAuthoredDecide trueValue(AuthoredCriterion value) {
            return put("true", value);
        }
        public RequestDefinitionAuthoredDecide trueValueNull() {
            return put("true", null);
        }
        public RequestDefinitionAuthoredDecide omitTrueValue() {
            return omit("true");
        }
        public RequestDefinitionAuthoredDecide wordingVersion(Number value) {
            return put("wording_version", value);
        }
        public RequestDefinitionAuthoredDecide wordingVersionNull() {
            return put("wording_version", null);
        }
        public RequestDefinitionAuthoredDecide omitWordingVersion() {
            return omit("wording_version");
        }
    }
    public static final class RequestDefinitionAuthoredDecidePropertiesBatch {
        private final Object value;
        public RequestDefinitionAuthoredDecidePropertiesBatch(String value) {
            this.value = Values.freeze(value);
        }
        public RequestDefinitionAuthoredDecidePropertiesBatch(Number value) {
            this.value = Values.freeze(value);
        }
        Object json() {
            return value;
        }
    }
    public static final class RequestDefinitionAuthoredFind extends Builder<RequestDefinitionAuthoredFind> implements RequestDefinition {
        public RequestDefinitionAuthoredFind() {
        }
        static RequestDefinitionAuthoredFind fromJson(Object value) {
            RequestDefinitionAuthoredFind result = new RequestDefinitionAuthoredFind();
            result.omit("context_schema");
            result.omit("find");
            result.omit("item_schema");
            result.omit("model");
            result.omit("name");
            result.omit("on");
            result.omit("profile");
            result.omit("wording_version");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestDefinitionAuthoredFind self() {
            return this;
        }
        public RequestDefinitionAuthoredFind contextSchema(AuthoredInputDeclaration value) {
            return put("context_schema", value);
        }
        public RequestDefinitionAuthoredFind contextSchemaNull() {
            return put("context_schema", null);
        }
        public RequestDefinitionAuthoredFind omitContextSchema() {
            return omit("context_schema");
        }
        public RequestDefinitionAuthoredFind find(AuthoredQuestionText value) {
            return put("find", value);
        }
        public RequestDefinitionAuthoredFind findNull() {
            return put("find", null);
        }
        public RequestDefinitionAuthoredFind omitFind() {
            return omit("find");
        }
        public RequestDefinitionAuthoredFind itemSchema(AuthoredInputDeclaration value) {
            return put("item_schema", value);
        }
        public RequestDefinitionAuthoredFind itemSchemaNull() {
            return put("item_schema", null);
        }
        public RequestDefinitionAuthoredFind omitItemSchema() {
            return omit("item_schema");
        }
        public RequestDefinitionAuthoredFind model(String value) {
            return put("model", value);
        }
        public RequestDefinitionAuthoredFind modelNull() {
            return put("model", null);
        }
        public RequestDefinitionAuthoredFind omitModel() {
            return omit("model");
        }
        public RequestDefinitionAuthoredFind name(String value) {
            return put("name", value);
        }
        public RequestDefinitionAuthoredFind nameNull() {
            return put("name", null);
        }
        public RequestDefinitionAuthoredFind omitName() {
            return omit("name");
        }
        public RequestDefinitionAuthoredFind on(AuthoredPointers value) {
            return put("on", value);
        }
        public RequestDefinitionAuthoredFind onNull() {
            return put("on", null);
        }
        public RequestDefinitionAuthoredFind omitOn() {
            return omit("on");
        }
        public RequestDefinitionAuthoredFind profile(String value) {
            return put("profile", value);
        }
        public RequestDefinitionAuthoredFind profileNull() {
            return put("profile", null);
        }
        public RequestDefinitionAuthoredFind omitProfile() {
            return omit("profile");
        }
        public RequestDefinitionAuthoredFind wordingVersion(Number value) {
            return put("wording_version", value);
        }
        public RequestDefinitionAuthoredFind wordingVersionNull() {
            return put("wording_version", null);
        }
        public RequestDefinitionAuthoredFind omitWordingVersion() {
            return omit("wording_version");
        }
    }
    public static final class RequestDefinitionAuthoredRelate extends Builder<RequestDefinitionAuthoredRelate> implements RequestDefinition {
        public RequestDefinitionAuthoredRelate() {
            put("version", 1);
        }
        static RequestDefinitionAuthoredRelate fromJson(Object value) {
            RequestDefinitionAuthoredRelate result = new RequestDefinitionAuthoredRelate();
            result.omit("context_schema");
            result.omit("item_schema");
            result.omit("model");
            result.omit("name");
            result.omit("profile");
            result.omit("relate");
            result.omit("threshold");
            result.omit("version");
            result.omit("wording_version");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestDefinitionAuthoredRelate self() {
            return this;
        }
        public RequestDefinitionAuthoredRelate contextSchema(AuthoredInputDeclaration value) {
            return put("context_schema", value);
        }
        public RequestDefinitionAuthoredRelate contextSchemaNull() {
            return put("context_schema", null);
        }
        public RequestDefinitionAuthoredRelate omitContextSchema() {
            return omit("context_schema");
        }
        public RequestDefinitionAuthoredRelate itemSchema(AuthoredInputDeclaration value) {
            return put("item_schema", value);
        }
        public RequestDefinitionAuthoredRelate itemSchemaNull() {
            return put("item_schema", null);
        }
        public RequestDefinitionAuthoredRelate omitItemSchema() {
            return omit("item_schema");
        }
        public RequestDefinitionAuthoredRelate model(String value) {
            return put("model", value);
        }
        public RequestDefinitionAuthoredRelate modelNull() {
            return put("model", null);
        }
        public RequestDefinitionAuthoredRelate omitModel() {
            return omit("model");
        }
        public RequestDefinitionAuthoredRelate name(String value) {
            return put("name", value);
        }
        public RequestDefinitionAuthoredRelate nameNull() {
            return put("name", null);
        }
        public RequestDefinitionAuthoredRelate omitName() {
            return omit("name");
        }
        public RequestDefinitionAuthoredRelate profile(String value) {
            return put("profile", value);
        }
        public RequestDefinitionAuthoredRelate profileNull() {
            return put("profile", null);
        }
        public RequestDefinitionAuthoredRelate omitProfile() {
            return omit("profile");
        }
        public RequestDefinitionAuthoredRelate relate(RelateQuestion value) {
            return put("relate", value);
        }
        public RequestDefinitionAuthoredRelate relateNull() {
            return put("relate", null);
        }
        public RequestDefinitionAuthoredRelate omitRelate() {
            return omit("relate");
        }
        public RequestDefinitionAuthoredRelate threshold(AuthoredCut value) {
            return put("threshold", value);
        }
        public RequestDefinitionAuthoredRelate thresholdNull() {
            return put("threshold", null);
        }
        public RequestDefinitionAuthoredRelate omitThreshold() {
            return omit("threshold");
        }
        public RequestDefinitionAuthoredRelate wordingVersion(Number value) {
            return put("wording_version", value);
        }
        public RequestDefinitionAuthoredRelate wordingVersionNull() {
            return put("wording_version", null);
        }
        public RequestDefinitionAuthoredRelate omitWordingVersion() {
            return omit("wording_version");
        }
    }
    public static final class RequestDefinitionAuthoredScore extends Builder<RequestDefinitionAuthoredScore> implements RequestDefinition {
        public RequestDefinitionAuthoredScore() {
        }
        static RequestDefinitionAuthoredScore fromJson(Object value) {
            RequestDefinitionAuthoredScore result = new RequestDefinitionAuthoredScore();
            result.omit("batch");
            result.omit("context_schema");
            result.omit("item_schema");
            result.omit("levels");
            result.omit("model");
            result.omit("name");
            result.omit("on");
            result.omit("profile");
            result.omit("score");
            result.omit("wording_version");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestDefinitionAuthoredScore self() {
            return this;
        }
        public RequestDefinitionAuthoredScore batch(RequestDefinitionAuthoredScorePropertiesBatch value) {
            return put("batch", value);
        }
        public RequestDefinitionAuthoredScore batchNull() {
            return put("batch", null);
        }
        public RequestDefinitionAuthoredScore omitBatch() {
            return omit("batch");
        }
        public RequestDefinitionAuthoredScore contextSchema(AuthoredInputDeclaration value) {
            return put("context_schema", value);
        }
        public RequestDefinitionAuthoredScore contextSchemaNull() {
            return put("context_schema", null);
        }
        public RequestDefinitionAuthoredScore omitContextSchema() {
            return omit("context_schema");
        }
        public RequestDefinitionAuthoredScore itemSchema(AuthoredInputDeclaration value) {
            return put("item_schema", value);
        }
        public RequestDefinitionAuthoredScore itemSchemaNull() {
            return put("item_schema", null);
        }
        public RequestDefinitionAuthoredScore omitItemSchema() {
            return omit("item_schema");
        }
        public RequestDefinitionAuthoredScore levels(AuthoredLevels value) {
            return put("levels", value);
        }
        public RequestDefinitionAuthoredScore levelsNull() {
            return put("levels", null);
        }
        public RequestDefinitionAuthoredScore omitLevels() {
            return omit("levels");
        }
        public RequestDefinitionAuthoredScore model(String value) {
            return put("model", value);
        }
        public RequestDefinitionAuthoredScore modelNull() {
            return put("model", null);
        }
        public RequestDefinitionAuthoredScore omitModel() {
            return omit("model");
        }
        public RequestDefinitionAuthoredScore name(String value) {
            return put("name", value);
        }
        public RequestDefinitionAuthoredScore nameNull() {
            return put("name", null);
        }
        public RequestDefinitionAuthoredScore omitName() {
            return omit("name");
        }
        public RequestDefinitionAuthoredScore on(AuthoredPointers value) {
            return put("on", value);
        }
        public RequestDefinitionAuthoredScore onNull() {
            return put("on", null);
        }
        public RequestDefinitionAuthoredScore omitOn() {
            return omit("on");
        }
        public RequestDefinitionAuthoredScore profile(String value) {
            return put("profile", value);
        }
        public RequestDefinitionAuthoredScore profileNull() {
            return put("profile", null);
        }
        public RequestDefinitionAuthoredScore omitProfile() {
            return omit("profile");
        }
        public RequestDefinitionAuthoredScore score(AuthoredQuestionText value) {
            return put("score", value);
        }
        public RequestDefinitionAuthoredScore scoreNull() {
            return put("score", null);
        }
        public RequestDefinitionAuthoredScore omitScore() {
            return omit("score");
        }
        public RequestDefinitionAuthoredScore wordingVersion(Number value) {
            return put("wording_version", value);
        }
        public RequestDefinitionAuthoredScore wordingVersionNull() {
            return put("wording_version", null);
        }
        public RequestDefinitionAuthoredScore omitWordingVersion() {
            return omit("wording_version");
        }
    }
    public static final class RequestDefinitionAuthoredScorePropertiesBatch {
        private final Object value;
        public RequestDefinitionAuthoredScorePropertiesBatch(String value) {
            this.value = Values.freeze(value);
        }
        public RequestDefinitionAuthoredScorePropertiesBatch(Number value) {
            this.value = Values.freeze(value);
        }
        Object json() {
            return value;
        }
    }
    public static final class RequestDefinitionAuthoredTag extends Builder<RequestDefinitionAuthoredTag> implements RequestDefinition {
        public RequestDefinitionAuthoredTag() {
        }
        static RequestDefinitionAuthoredTag fromJson(Object value) {
            RequestDefinitionAuthoredTag result = new RequestDefinitionAuthoredTag();
            result.omit("batch");
            result.omit("context_schema");
            result.omit("item_schema");
            result.omit("labels");
            result.omit("model");
            result.omit("name");
            result.omit("on");
            result.omit("profile");
            result.omit("tag");
            result.omit("threshold");
            result.omit("wording_version");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestDefinitionAuthoredTag self() {
            return this;
        }
        public RequestDefinitionAuthoredTag batch(RequestDefinitionAuthoredTagPropertiesBatch value) {
            return put("batch", value);
        }
        public RequestDefinitionAuthoredTag batchNull() {
            return put("batch", null);
        }
        public RequestDefinitionAuthoredTag omitBatch() {
            return omit("batch");
        }
        public RequestDefinitionAuthoredTag contextSchema(AuthoredInputDeclaration value) {
            return put("context_schema", value);
        }
        public RequestDefinitionAuthoredTag contextSchemaNull() {
            return put("context_schema", null);
        }
        public RequestDefinitionAuthoredTag omitContextSchema() {
            return omit("context_schema");
        }
        public RequestDefinitionAuthoredTag itemSchema(AuthoredInputDeclaration value) {
            return put("item_schema", value);
        }
        public RequestDefinitionAuthoredTag itemSchemaNull() {
            return put("item_schema", null);
        }
        public RequestDefinitionAuthoredTag omitItemSchema() {
            return omit("item_schema");
        }
        public RequestDefinitionAuthoredTag labels(AuthoredLabels value) {
            return put("labels", value);
        }
        public RequestDefinitionAuthoredTag labelsNull() {
            return put("labels", null);
        }
        public RequestDefinitionAuthoredTag omitLabels() {
            return omit("labels");
        }
        public RequestDefinitionAuthoredTag model(String value) {
            return put("model", value);
        }
        public RequestDefinitionAuthoredTag modelNull() {
            return put("model", null);
        }
        public RequestDefinitionAuthoredTag omitModel() {
            return omit("model");
        }
        public RequestDefinitionAuthoredTag name(String value) {
            return put("name", value);
        }
        public RequestDefinitionAuthoredTag nameNull() {
            return put("name", null);
        }
        public RequestDefinitionAuthoredTag omitName() {
            return omit("name");
        }
        public RequestDefinitionAuthoredTag on(AuthoredPointers value) {
            return put("on", value);
        }
        public RequestDefinitionAuthoredTag onNull() {
            return put("on", null);
        }
        public RequestDefinitionAuthoredTag omitOn() {
            return omit("on");
        }
        public RequestDefinitionAuthoredTag profile(String value) {
            return put("profile", value);
        }
        public RequestDefinitionAuthoredTag profileNull() {
            return put("profile", null);
        }
        public RequestDefinitionAuthoredTag omitProfile() {
            return omit("profile");
        }
        public RequestDefinitionAuthoredTag tag(AuthoredQuestionText value) {
            return put("tag", value);
        }
        public RequestDefinitionAuthoredTag tagNull() {
            return put("tag", null);
        }
        public RequestDefinitionAuthoredTag omitTag() {
            return omit("tag");
        }
        public RequestDefinitionAuthoredTag threshold(AuthoredCut value) {
            return put("threshold", value);
        }
        public RequestDefinitionAuthoredTag thresholdNull() {
            return put("threshold", null);
        }
        public RequestDefinitionAuthoredTag omitThreshold() {
            return omit("threshold");
        }
        public RequestDefinitionAuthoredTag wordingVersion(Number value) {
            return put("wording_version", value);
        }
        public RequestDefinitionAuthoredTag wordingVersionNull() {
            return put("wording_version", null);
        }
        public RequestDefinitionAuthoredTag omitWordingVersion() {
            return omit("wording_version");
        }
    }
    public static final class RequestDefinitionAuthoredTagPropertiesBatch {
        private final Object value;
        public RequestDefinitionAuthoredTagPropertiesBatch(String value) {
            this.value = Values.freeze(value);
        }
        public RequestDefinitionAuthoredTagPropertiesBatch(Number value) {
            this.value = Values.freeze(value);
        }
        Object json() {
            return value;
        }
    }
    public static final class RequestDefinitionRecognizeDefinition extends Builder<RequestDefinitionRecognizeDefinition> implements RequestDefinition {
        public RequestDefinitionRecognizeDefinition() {
            put("version", 1);
        }
        static RequestDefinitionRecognizeDefinition fromJson(Object value) {
            RequestDefinitionRecognizeDefinition result = new RequestDefinitionRecognizeDefinition();
            result.omit("context_schema");
            result.omit("item_schema");
            result.omit("model");
            result.omit("name");
            result.omit("on");
            result.omit("profile");
            result.omit("recognize");
            result.omit("relation_threshold");
            result.omit("threshold");
            result.omit("version");
            result.omit("wording_version");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestDefinitionRecognizeDefinition self() {
            return this;
        }
        public RequestDefinitionRecognizeDefinition contextSchema(AuthoredInputDeclaration value) {
            return put("context_schema", value);
        }
        public RequestDefinitionRecognizeDefinition contextSchemaNull() {
            return put("context_schema", null);
        }
        public RequestDefinitionRecognizeDefinition omitContextSchema() {
            return omit("context_schema");
        }
        public RequestDefinitionRecognizeDefinition itemSchema(AuthoredInputDeclaration value) {
            return put("item_schema", value);
        }
        public RequestDefinitionRecognizeDefinition itemSchemaNull() {
            return put("item_schema", null);
        }
        public RequestDefinitionRecognizeDefinition omitItemSchema() {
            return omit("item_schema");
        }
        public RequestDefinitionRecognizeDefinition model(String value) {
            return put("model", value);
        }
        public RequestDefinitionRecognizeDefinition modelNull() {
            return put("model", null);
        }
        public RequestDefinitionRecognizeDefinition omitModel() {
            return omit("model");
        }
        public RequestDefinitionRecognizeDefinition name(String value) {
            return put("name", value);
        }
        public RequestDefinitionRecognizeDefinition nameNull() {
            return put("name", null);
        }
        public RequestDefinitionRecognizeDefinition omitName() {
            return omit("name");
        }
        public RequestDefinitionRecognizeDefinition on(AuthoredPointers value) {
            return put("on", value);
        }
        public RequestDefinitionRecognizeDefinition onNull() {
            return put("on", null);
        }
        public RequestDefinitionRecognizeDefinition omitOn() {
            return omit("on");
        }
        public RequestDefinitionRecognizeDefinition profile(String value) {
            return put("profile", value);
        }
        public RequestDefinitionRecognizeDefinition profileNull() {
            return put("profile", null);
        }
        public RequestDefinitionRecognizeDefinition omitProfile() {
            return omit("profile");
        }
        public RequestDefinitionRecognizeDefinition recognize(RecognizeQuestion value) {
            return put("recognize", value);
        }
        public RequestDefinitionRecognizeDefinition recognizeNull() {
            return put("recognize", null);
        }
        public RequestDefinitionRecognizeDefinition omitRecognize() {
            return omit("recognize");
        }
        public RequestDefinitionRecognizeDefinition relationThreshold(AuthoredCut value) {
            return put("relation_threshold", value);
        }
        public RequestDefinitionRecognizeDefinition relationThresholdNull() {
            return put("relation_threshold", null);
        }
        public RequestDefinitionRecognizeDefinition omitRelationThreshold() {
            return omit("relation_threshold");
        }
        public RequestDefinitionRecognizeDefinition threshold(AuthoredCut value) {
            return put("threshold", value);
        }
        public RequestDefinitionRecognizeDefinition thresholdNull() {
            return put("threshold", null);
        }
        public RequestDefinitionRecognizeDefinition omitThreshold() {
            return omit("threshold");
        }
        public RequestDefinitionRecognizeDefinition wordingVersion(Number value) {
            return put("wording_version", value);
        }
        public RequestDefinitionRecognizeDefinition wordingVersionNull() {
            return put("wording_version", null);
        }
        public RequestDefinitionRecognizeDefinition omitWordingVersion() {
            return omit("wording_version");
        }
    }
    public enum RequestFraming {
        DOCUMENT("document"),LINES("lines"),JSONL("jsonl"),CSV("csv"),TSV("tsv");
        private final String value;
        RequestFraming(String value) {
            this.value = value;
        }
        Object json() {
            return value;
        }
    }
    public enum RequestFunction {
        DECIDE("decide"),CHOOSE("choose"),TAG("tag"),SCORE("score"),FILTER("filter"),RANK("rank"),
        FIND("find"),ANNOTATE("annotate"),RECOGNIZE("recognize"),RELATE("relate");
        private final String value;
        RequestFunction(String value) {
            this.value = value;
        }
        Object json() {
            return value;
        }
    }
    public sealed interface RequestImage permits RequestImageFile,RequestImageBytes {
    }
    public static final class RequestImageBytes extends Builder<RequestImageBytes> implements RequestImage {
        public RequestImageBytes() {
            put("kind", "bytes");
        }
        static RequestImageBytes fromJson(Object value) {
            RequestImageBytes result = new RequestImageBytes();
            result.omit("bytes");
            result.omit("kind");
            result.omit("media");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestImageBytes self() {
            return this;
        }
        public RequestImageBytes bytes(String value) {
            return put("bytes", value);
        }
        public RequestImageBytes bytesNull() {
            return put("bytes", null);
        }
        public RequestImageBytes omitBytes() {
            return omit("bytes");
        }
        public RequestImageBytes media(ImageMedia value) {
            return put("media", value);
        }
        public RequestImageBytes mediaNull() {
            return put("media", null);
        }
        public RequestImageBytes omitMedia() {
            return omit("media");
        }
    }
    public static final class RequestImageFile extends Builder<RequestImageFile> implements RequestImage {
        public RequestImageFile() {
            put("kind", "file");
        }
        static RequestImageFile fromJson(Object value) {
            RequestImageFile result = new RequestImageFile();
            result.omit("kind");
            result.omit("media");
            result.omit("path");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestImageFile self() {
            return this;
        }
        public RequestImageFile media(ImageMedia value) {
            return put("media", value);
        }
        public RequestImageFile mediaNull() {
            return put("media", null);
        }
        public RequestImageFile omitMedia() {
            return omit("media");
        }
        public RequestImageFile path(String value) {
            return put("path", value);
        }
        public RequestImageFile pathNull() {
            return put("path", null);
        }
        public RequestImageFile omitPath() {
            return omit("path");
        }
    }
    public sealed interface RequestInput permits RequestInputText,RequestInputJson,RequestInputRecords,
    RequestInputUnits,RequestInputEntities,RequestInputSource,RequestInputFeed {
    }
    public static final class RequestInputEntities extends Builder<RequestInputEntities> implements RequestInput {
        public RequestInputEntities() {
            put("kind", "entities");
        }
        static RequestInputEntities fromJson(Object value) {
            RequestInputEntities result = new RequestInputEntities();
            result.omit("items");
            result.omit("kind");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestInputEntities self() {
            return this;
        }
        public RequestInputEntities items(List<? extends RequestItem> value) {
            return put("items", value);
        }
        public RequestInputEntities itemsNull() {
            return put("items", null);
        }
        public RequestInputEntities omitItems() {
            return omit("items");
        }
    }
    public static final class RequestInputFeed extends Builder<RequestInputFeed> implements RequestInput {
        public RequestInputFeed() {
            put("kind", "feed");
        }
        static RequestInputFeed fromJson(Object value) {
            RequestInputFeed result = new RequestInputFeed();
            result.omit("framing");
            result.omit("images");
            result.omit("kind");
            result.omit("name");
            result.omit("reading");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestInputFeed self() {
            return this;
        }
        public RequestInputFeed framing(RequestFraming value) {
            return put("framing", value);
        }
        public RequestInputFeed framingNull() {
            return put("framing", null);
        }
        public RequestInputFeed omitFraming() {
            return omit("framing");
        }
        public RequestInputFeed images(List<? extends RequestImage> value) {
            return put("images", value);
        }
        public RequestInputFeed imagesNull() {
            return put("images", null);
        }
        public RequestInputFeed omitImages() {
            return omit("images");
        }
        public RequestInputFeed name(String value) {
            return put("name", value);
        }
        public RequestInputFeed nameNull() {
            return put("name", null);
        }
        public RequestInputFeed omitName() {
            return omit("name");
        }
        public RequestInputFeed reading(RequestReader value) {
            return put("reading", value);
        }
        public RequestInputFeed readingNull() {
            return put("reading", null);
        }
        public RequestInputFeed omitReading() {
            return omit("reading");
        }
    }
    public static final class RequestInputJson extends Builder<RequestInputJson> implements RequestInput {
        public RequestInputJson() {
            put("kind", "json");
        }
        static RequestInputJson fromJson(Object value) {
            RequestInputJson result = new RequestInputJson();
            result.omit("images");
            result.omit("kind");
            result.omit("value");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestInputJson self() {
            return this;
        }
        public RequestInputJson images(List<? extends RequestImage> value) {
            return put("images", value);
        }
        public RequestInputJson imagesNull() {
            return put("images", null);
        }
        public RequestInputJson omitImages() {
            return omit("images");
        }
        public RequestInputJson value(Object value) {
            return put("value", value);
        }
        public RequestInputJson valueNull() {
            return put("value", null);
        }
        public RequestInputJson omitValue() {
            return omit("value");
        }
    }
    public static final class RequestInputRecords extends Builder<RequestInputRecords> implements RequestInput {
        public RequestInputRecords() {
            put("kind", "records");
        }
        static RequestInputRecords fromJson(Object value) {
            RequestInputRecords result = new RequestInputRecords();
            result.omit("items");
            result.omit("kind");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestInputRecords self() {
            return this;
        }
        public RequestInputRecords items(List<? extends RequestItem> value) {
            return put("items", value);
        }
        public RequestInputRecords itemsNull() {
            return put("items", null);
        }
        public RequestInputRecords omitItems() {
            return omit("items");
        }
    }
    public static final class RequestInputSource extends Builder<RequestInputSource> implements RequestInput {
        public RequestInputSource() {
            put("kind", "source");
        }
        static RequestInputSource fromJson(Object value) {
            RequestInputSource result = new RequestInputSource();
            result.omit("kind");
            result.omit("source");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestInputSource self() {
            return this;
        }
        public RequestInputSource source(RequestSource value) {
            return put("source", value);
        }
        public RequestInputSource sourceNull() {
            return put("source", null);
        }
        public RequestInputSource omitSource() {
            return omit("source");
        }
    }
    public static final class RequestInputText extends Builder<RequestInputText> implements RequestInput {
        public RequestInputText() {
            put("kind", "text");
        }
        static RequestInputText fromJson(Object value) {
            RequestInputText result = new RequestInputText();
            result.omit("images");
            result.omit("kind");
            result.omit("text");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestInputText self() {
            return this;
        }
        public RequestInputText images(List<? extends RequestImage> value) {
            return put("images", value);
        }
        public RequestInputText imagesNull() {
            return put("images", null);
        }
        public RequestInputText omitImages() {
            return omit("images");
        }
        public RequestInputText text(String value) {
            return put("text", value);
        }
        public RequestInputText textNull() {
            return put("text", null);
        }
        public RequestInputText omitText() {
            return omit("text");
        }
    }
    public static final class RequestInputUnits extends Builder<RequestInputUnits> implements RequestInput {
        public RequestInputUnits() {
            put("kind", "units");
        }
        static RequestInputUnits fromJson(Object value) {
            RequestInputUnits result = new RequestInputUnits();
            result.omit("items");
            result.omit("kind");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestInputUnits self() {
            return this;
        }
        public RequestInputUnits items(List<? extends RequestItem> value) {
            return put("items", value);
        }
        public RequestInputUnits itemsNull() {
            return put("items", null);
        }
        public RequestInputUnits omitItems() {
            return omit("items");
        }
    }
    public static final class RequestItem extends Builder<RequestItem> {
        public RequestItem() {
        }
        static RequestItem fromJson(Object value) {
            RequestItem result = new RequestItem();
            result.omit("context");
            result.omit("examples");
            result.omit("images");
            result.omit("options");
            result.omit("original");
            result.omit("seed_spans");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestItem self() {
            return this;
        }
        public RequestItem context(ContextSchema value) {
            return put("context", value);
        }
        public RequestItem contextNull() {
            return put("context", null);
        }
        public RequestItem omitContext() {
            return omit("context");
        }
        public RequestItem examples(List<? extends RecognitionExample> value) {
            return put("examples", value);
        }
        public RequestItem examplesNull() {
            return put("examples", null);
        }
        public RequestItem omitExamples() {
            return omit("examples");
        }
        public RequestItem images(List<? extends RequestImage> value) {
            return put("images", value);
        }
        public RequestItem imagesNull() {
            return put("images", null);
        }
        public RequestItem omitImages() {
            return omit("images");
        }
        public RequestItem options(List<? extends OptionSchema> value) {
            return put("options", value);
        }
        public RequestItem optionsNull() {
            return put("options", null);
        }
        public RequestItem omitOptions() {
            return omit("options");
        }
        public RequestItem original(RequestOriginal value) {
            return put("original", value);
        }
        public RequestItem originalNull() {
            return put("original", null);
        }
        public RequestItem omitOriginal() {
            return omit("original");
        }
        public RequestItem seedSpans(List<? extends RecognitionSeedSpan> value) {
            return put("seed_spans", value);
        }
        public RequestItem seedSpansNull() {
            return put("seed_spans", null);
        }
        public RequestItem omitSeedSpans() {
            return omit("seed_spans");
        }
    }
    public static final class RequestOptions extends Builder<RequestOptions> {
        public RequestOptions() {
        }
        static RequestOptions fromJson(Object value) {
            RequestOptions result = new RequestOptions();
            result.omit("attempts");
            result.omit("batch");
            result.omit("context");
            result.omit("context_field");
            result.omit("deadline_ms");
            result.omit("details");
            result.omit("examples");
            result.omit("examples_field");
            result.omit("field");
            result.omit("files_only");
            result.omit("max_requests_total");
            result.omit("mode");
            result.omit("model");
            result.omit("none");
            result.omit("options_field");
            result.omit("relation_threshold");
            result.omit("seed_spans");
            result.omit("seed_spans_field");
            result.omit("snippet_pieces");
            result.omit("stage_context");
            result.omit("threshold");
            result.omit("top");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestOptions self() {
            return this;
        }
        public RequestOptions attempts(Boolean value) {
            return put("attempts", value);
        }
        public RequestOptions attemptsNull() {
            return put("attempts", null);
        }
        public RequestOptions omitAttempts() {
            return omit("attempts");
        }
        public RequestOptions batch(RequestBatch value) {
            return put("batch", value);
        }
        public RequestOptions batchNull() {
            return put("batch", null);
        }
        public RequestOptions omitBatch() {
            return omit("batch");
        }
        public RequestOptions context(String value) {
            return put("context", value);
        }
        public RequestOptions contextNull() {
            return put("context", null);
        }
        public RequestOptions omitContext() {
            return omit("context");
        }
        public RequestOptions contextField(String value) {
            return put("context_field", value);
        }
        public RequestOptions contextFieldNull() {
            return put("context_field", null);
        }
        public RequestOptions omitContextField() {
            return omit("context_field");
        }
        public RequestOptions deadlineMs(Number value) {
            return put("deadline_ms", value);
        }
        public RequestOptions deadlineMsNull() {
            return put("deadline_ms", null);
        }
        public RequestOptions omitDeadlineMs() {
            return omit("deadline_ms");
        }
        public RequestOptions details(Boolean value) {
            return put("details", value);
        }
        public RequestOptions detailsNull() {
            return put("details", null);
        }
        public RequestOptions omitDetails() {
            return omit("details");
        }
        public RequestOptions examples(List<? extends RecognitionExample> value) {
            return put("examples", value);
        }
        public RequestOptions examplesNull() {
            return put("examples", null);
        }
        public RequestOptions omitExamples() {
            return omit("examples");
        }
        public RequestOptions examplesField(String value) {
            return put("examples_field", value);
        }
        public RequestOptions examplesFieldNull() {
            return put("examples_field", null);
        }
        public RequestOptions omitExamplesField() {
            return omit("examples_field");
        }
        public RequestOptions field(List<? extends String> value) {
            return put("field", value);
        }
        public RequestOptions fieldNull() {
            return put("field", null);
        }
        public RequestOptions omitField() {
            return omit("field");
        }
        public RequestOptions filesOnly(Boolean value) {
            return put("files_only", value);
        }
        public RequestOptions filesOnlyNull() {
            return put("files_only", null);
        }
        public RequestOptions omitFilesOnly() {
            return omit("files_only");
        }
        public RequestOptions maxRequestsTotal(Number value) {
            return put("max_requests_total", value);
        }
        public RequestOptions maxRequestsTotalNull() {
            return put("max_requests_total", null);
        }
        public RequestOptions omitMaxRequestsTotal() {
            return omit("max_requests_total");
        }
        public RequestOptions mode(RecognitionMode value) {
            return put("mode", value);
        }
        public RequestOptions modeNull() {
            return put("mode", null);
        }
        public RequestOptions omitMode() {
            return omit("mode");
        }
        public RequestOptions model(String value) {
            return put("model", value);
        }
        public RequestOptions modelNull() {
            return put("model", null);
        }
        public RequestOptions omitModel() {
            return omit("model");
        }
        public RequestOptions none(Boolean value) {
            return put("none", value);
        }
        public RequestOptions noneNull() {
            return put("none", null);
        }
        public RequestOptions omitNone() {
            return omit("none");
        }
        public RequestOptions optionsField(String value) {
            return put("options_field", value);
        }
        public RequestOptions optionsFieldNull() {
            return put("options_field", null);
        }
        public RequestOptions omitOptionsField() {
            return omit("options_field");
        }
        public RequestOptions relationThreshold(RequestThreshold value) {
            return put("relation_threshold", value);
        }
        public RequestOptions relationThresholdNull() {
            return put("relation_threshold", null);
        }
        public RequestOptions omitRelationThreshold() {
            return omit("relation_threshold");
        }
        public RequestOptions seedSpans(List<? extends RecognitionSeedSpan> value) {
            return put("seed_spans", value);
        }
        public RequestOptions seedSpansNull() {
            return put("seed_spans", null);
        }
        public RequestOptions omitSeedSpans() {
            return omit("seed_spans");
        }
        public RequestOptions seedSpansField(String value) {
            return put("seed_spans_field", value);
        }
        public RequestOptions seedSpansFieldNull() {
            return put("seed_spans_field", null);
        }
        public RequestOptions omitSeedSpansField() {
            return omit("seed_spans_field");
        }
        public RequestOptions snippetPieces(Number value) {
            return put("snippet_pieces", value);
        }
        public RequestOptions snippetPiecesNull() {
            return put("snippet_pieces", null);
        }
        public RequestOptions omitSnippetPieces() {
            return omit("snippet_pieces");
        }
        public RequestOptions stageContext(RecognitionStageContext value) {
            return put("stage_context", value);
        }
        public RequestOptions stageContextNull() {
            return put("stage_context", null);
        }
        public RequestOptions omitStageContext() {
            return omit("stage_context");
        }
        public RequestOptions threshold(RequestThreshold value) {
            return put("threshold", value);
        }
        public RequestOptions thresholdNull() {
            return put("threshold", null);
        }
        public RequestOptions omitThreshold() {
            return omit("threshold");
        }
        public RequestOptions top(Number value) {
            return put("top", value);
        }
        public RequestOptions topNull() {
            return put("top", null);
        }
        public RequestOptions omitTop() {
            return omit("top");
        }
    }
    public sealed interface RequestOriginal permits RequestOriginalText,RequestOriginalJson {
    }
    public static final class RequestOriginalJson extends Builder<RequestOriginalJson> implements RequestOriginal {
        public RequestOriginalJson() {
            put("kind", "json");
        }
        static RequestOriginalJson fromJson(Object value) {
            RequestOriginalJson result = new RequestOriginalJson();
            result.omit("kind");
            result.omit("value");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestOriginalJson self() {
            return this;
        }
        public RequestOriginalJson value(Object value) {
            return put("value", value);
        }
        public RequestOriginalJson valueNull() {
            return put("value", null);
        }
        public RequestOriginalJson omitValue() {
            return omit("value");
        }
    }
    public static final class RequestOriginalText extends Builder<RequestOriginalText> implements RequestOriginal {
        public RequestOriginalText() {
            put("kind", "text");
        }
        static RequestOriginalText fromJson(Object value) {
            RequestOriginalText result = new RequestOriginalText();
            result.omit("kind");
            result.omit("text");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestOriginalText self() {
            return this;
        }
        public RequestOriginalText text(String value) {
            return put("text", value);
        }
        public RequestOriginalText textNull() {
            return put("text", null);
        }
        public RequestOriginalText omitText() {
            return omit("text");
        }
    }
    public sealed interface RequestQuestion permits RequestQuestionText,RequestQuestionDefinition,
    RequestQuestionFile,RequestQuestionName,RequestQuestionReference {
    }
    public static final class RequestQuestionDefinition extends Builder<RequestQuestionDefinition> implements RequestQuestion {
        public RequestQuestionDefinition() {
            put("kind", "definition");
        }
        static RequestQuestionDefinition fromJson(Object value) {
            RequestQuestionDefinition result = new RequestQuestionDefinition();
            result.omit("kind");
            result.omit("value");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestQuestionDefinition self() {
            return this;
        }
        public RequestQuestionDefinition value(RequestDefinition value) {
            return put("value", value);
        }
        public RequestQuestionDefinition valueNull() {
            return put("value", null);
        }
        public RequestQuestionDefinition omitValue() {
            return omit("value");
        }
    }
    public static final class RequestQuestionFile extends Builder<RequestQuestionFile> implements RequestQuestion {
        public RequestQuestionFile() {
            put("kind", "file");
        }
        static RequestQuestionFile fromJson(Object value) {
            RequestQuestionFile result = new RequestQuestionFile();
            result.omit("kind");
            result.omit("path");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestQuestionFile self() {
            return this;
        }
        public RequestQuestionFile path(String value) {
            return put("path", value);
        }
        public RequestQuestionFile pathNull() {
            return put("path", null);
        }
        public RequestQuestionFile omitPath() {
            return omit("path");
        }
    }
    public static final class RequestQuestionName extends Builder<RequestQuestionName> implements RequestQuestion {
        public RequestQuestionName() {
            put("kind", "name");
        }
        static RequestQuestionName fromJson(Object value) {
            RequestQuestionName result = new RequestQuestionName();
            result.omit("kind");
            result.omit("name");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestQuestionName self() {
            return this;
        }
        public RequestQuestionName name(String value) {
            return put("name", value);
        }
        public RequestQuestionName nameNull() {
            return put("name", null);
        }
        public RequestQuestionName omitName() {
            return omit("name");
        }
    }
    public static final class RequestQuestionReference extends Builder<RequestQuestionReference> implements RequestQuestion {
        public RequestQuestionReference() {
            put("kind", "reference");
        }
        static RequestQuestionReference fromJson(Object value) {
            RequestQuestionReference result = new RequestQuestionReference();
            result.omit("kind");
            result.omit("reference");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestQuestionReference self() {
            return this;
        }
        public RequestQuestionReference reference(String value) {
            return put("reference", value);
        }
        public RequestQuestionReference referenceNull() {
            return put("reference", null);
        }
        public RequestQuestionReference omitReference() {
            return omit("reference");
        }
    }
    public static final class RequestQuestionText extends Builder<RequestQuestionText> implements RequestQuestion {
        public RequestQuestionText() {
            put("kind", "text");
        }
        static RequestQuestionText fromJson(Object value) {
            RequestQuestionText result = new RequestQuestionText();
            result.omit("kind");
            result.omit("text");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestQuestionText self() {
            return this;
        }
        public RequestQuestionText text(String value) {
            return put("text", value);
        }
        public RequestQuestionText textNull() {
            return put("text", null);
        }
        public RequestQuestionText omitText() {
            return omit("text");
        }
    }
    public static final class RequestReader extends Builder<RequestReader> {
        public RequestReader() {
        }
        static RequestReader fromJson(Object value) {
            RequestReader result = new RequestReader();
            result.omit("unit");
            result.omit("window");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestReader self() {
            return this;
        }
        public RequestReader unit(SourceUnit value) {
            return put("unit", value);
        }
        public RequestReader unitNull() {
            return put("unit", null);
        }
        public RequestReader omitUnit() {
            return omit("unit");
        }
        public RequestReader window(Number value) {
            return put("window", value);
        }
        public RequestReader windowNull() {
            return put("window", null);
        }
        public RequestReader omitWindow() {
            return omit("window");
        }
    }
    public sealed interface RequestReaderFailure permits RequestReaderFailureProducer,RequestReaderFailureIo,
    RequestReaderFailureUtf8,RequestReaderFailureInvalidInput {
    }
    public static final class RequestReaderFailureInvalidInput extends Builder<RequestReaderFailureInvalidInput> implements RequestReaderFailure {
        public RequestReaderFailureInvalidInput() {
            put("kind", "invalid_input");
        }
        static RequestReaderFailureInvalidInput fromJson(Object value) {
            RequestReaderFailureInvalidInput result = new RequestReaderFailureInvalidInput();
            result.omit("kind");
            result.omit("location");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestReaderFailureInvalidInput self() {
            return this;
        }
        public RequestReaderFailureInvalidInput location(SessionSourceLocation value) {
            return put("location", value);
        }
        public RequestReaderFailureInvalidInput locationNull() {
            return put("location", null);
        }
        public RequestReaderFailureInvalidInput omitLocation() {
            return omit("location");
        }
    }
    public static final class RequestReaderFailureIo extends Builder<RequestReaderFailureIo> implements RequestReaderFailure {
        public RequestReaderFailureIo() {
            put("kind", "io");
        }
        static RequestReaderFailureIo fromJson(Object value) {
            RequestReaderFailureIo result = new RequestReaderFailureIo();
            result.omit("kind");
            result.omit("location");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestReaderFailureIo self() {
            return this;
        }
        public RequestReaderFailureIo location(SessionSourceLocation value) {
            return put("location", value);
        }
        public RequestReaderFailureIo locationNull() {
            return put("location", null);
        }
        public RequestReaderFailureIo omitLocation() {
            return omit("location");
        }
    }
    public static final class RequestReaderFailureProducer extends Builder<RequestReaderFailureProducer> implements RequestReaderFailure {
        public RequestReaderFailureProducer() {
            put("kind", "producer");
        }
        static RequestReaderFailureProducer fromJson(Object value) {
            RequestReaderFailureProducer result = new RequestReaderFailureProducer();
            result.omit("kind");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestReaderFailureProducer self() {
            return this;
        }
    }
    public static final class RequestReaderFailureUtf8 extends Builder<RequestReaderFailureUtf8> implements RequestReaderFailure {
        public RequestReaderFailureUtf8() {
            put("kind", "utf8");
        }
        static RequestReaderFailureUtf8 fromJson(Object value) {
            RequestReaderFailureUtf8 result = new RequestReaderFailureUtf8();
            result.omit("kind");
            result.omit("location");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestReaderFailureUtf8 self() {
            return this;
        }
        public RequestReaderFailureUtf8 location(SessionSourceLocation value) {
            return put("location", value);
        }
        public RequestReaderFailureUtf8 locationNull() {
            return put("location", null);
        }
        public RequestReaderFailureUtf8 omitLocation() {
            return omit("location");
        }
    }
    public static final class RequestSessionDescriptor extends Builder<RequestSessionDescriptor> {
        public RequestSessionDescriptor() {
        }
        static RequestSessionDescriptor fromJson(Object value) {
            RequestSessionDescriptor result = new RequestSessionDescriptor();
            result.omit("item");
            result.omit("location");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestSessionDescriptor self() {
            return this;
        }
        public RequestSessionDescriptor item(RequestItem value) {
            return put("item", value);
        }
        public RequestSessionDescriptor itemNull() {
            return put("item", null);
        }
        public RequestSessionDescriptor omitItem() {
            return omit("item");
        }
        public RequestSessionDescriptor location(SessionSourceLocation value) {
            return put("location", value);
        }
        public RequestSessionDescriptor locationNull() {
            return put("location", null);
        }
        public RequestSessionDescriptor omitLocation() {
            return omit("location");
        }
    }
    public static final class RequestSource extends Builder<RequestSource> {
        public RequestSource() {
        }
        static RequestSource fromJson(Object value) {
            RequestSource result = new RequestSource();
            result.omit("framing");
            result.omit("media");
            result.omit("paths");
            result.omit("reading");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected RequestSource self() {
            return this;
        }
        public RequestSource framing(RequestFraming value) {
            return put("framing", value);
        }
        public RequestSource framingNull() {
            return put("framing", null);
        }
        public RequestSource omitFraming() {
            return omit("framing");
        }
        public RequestSource media(ReaderMedia value) {
            return put("media", value);
        }
        public RequestSource mediaNull() {
            return put("media", null);
        }
        public RequestSource omitMedia() {
            return omit("media");
        }
        public RequestSource paths(List<? extends String> value) {
            return put("paths", value);
        }
        public RequestSource pathsNull() {
            return put("paths", null);
        }
        public RequestSource omitPaths() {
            return omit("paths");
        }
        public RequestSource reading(RequestReader value) {
            return put("reading", value);
        }
        public RequestSource readingNull() {
            return put("reading", null);
        }
        public RequestSource omitReading() {
            return omit("reading");
        }
    }
    public static final class RequestThreshold {
        private final Object value;
        public RequestThreshold(Number value) {
            this.value = Values.freeze(value);
        }
        public RequestThreshold(String value) {
            this.value = Values.freeze(value);
        }
        Object json() {
            return value;
        }
    }
    public enum RequestVersion {
        THINKTHENREQUEST1("thinkthen.request/1");
        private final String value;
        RequestVersion(String value) {
            this.value = value;
        }
        Object json() {
            return value;
        }
    }
    public static final class SessionSourceLocation extends Builder<SessionSourceLocation> {
        public SessionSourceLocation() {
        }
        static SessionSourceLocation fromJson(Object value) {
            SessionSourceLocation result = new SessionSourceLocation();
            result.omit("file");
            result.omit("first_line");
            result.omit("last_line");
            Values.object(value).forEach(result::extension);
            return result;
        }
        protected SessionSourceLocation self() {
            return this;
        }
        public SessionSourceLocation file(String value) {
            return put("file", value);
        }
        public SessionSourceLocation fileNull() {
            return put("file", null);
        }
        public SessionSourceLocation omitFile() {
            return omit("file");
        }
        public SessionSourceLocation firstLine(Number value) {
            return put("first_line", value);
        }
        public SessionSourceLocation firstLineNull() {
            return put("first_line", null);
        }
        public SessionSourceLocation omitFirstLine() {
            return omit("first_line");
        }
        public SessionSourceLocation lastLine(Number value) {
            return put("last_line", value);
        }
        public SessionSourceLocation lastLineNull() {
            return put("last_line", null);
        }
        public SessionSourceLocation omitLastLine() {
            return omit("last_line");
        }
    }
    public enum SourceUnit {
        LINE("line"),WINDOW("window"),FILE("file");
        private final String value;
        SourceUnit(String value) {
            this.value = value;
        }
        Object json() {
            return value;
        }
    }
    static AnnotateQuestion readAnnotateQuestion(Object value) {
        Map<String,Object> object = Values.object(value);
        if (object.containsKey("decide") && !object.containsKey("choose") && !object.containsKey("score") && !object.containsKey("tag")) return AnnotateQuestionDecide.fromJson(value);
        if (object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("score") && !object.containsKey("tag")) return AnnotateQuestionChoose.fromJson(value);
        if (object.containsKey("tag") && !object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("score")) return AnnotateQuestionTag.fromJson(value);
        if (object.containsKey("score") && !object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("tag")) return AnnotateQuestionScore.fromJson(value);
        return AnnotateQuestionDecide.fromJson(value);
    }
    static AuthoredInputDeclaration readAuthoredInputDeclaration(Object value) {
        Map<String,Object> object = Values.object(value);
        if ("string".equals(object.get("type"))) return AuthoredInputDeclarationString.fromJson(value);
        if ("object".equals(object.get("type"))) return AuthoredInputDeclarationObject.fromJson(value);
        return AuthoredInputDeclarationString.fromJson(value);
    }
    static AuthoredInputProperty readAuthoredInputProperty(Object value) {
        Map<String,Object> object = Values.object(value);
        if ("string".equals(object.get("type"))) return AuthoredInputPropertyString.fromJson(value);
        if ("number".equals(object.get("type"))) return AuthoredInputPropertyNumber.fromJson(value);
        if ("boolean".equals(object.get("type"))) return AuthoredInputPropertyBoolean.fromJson(value);
        if ("array".equals(object.get("type"))) return AuthoredInputPropertyArray.fromJson(value);
        return AuthoredInputPropertyString.fromJson(value);
    }
    static RequestDefinition readRequestDefinition(Object value) {
        Map<String,Object> object = Values.object(value);
        if (object.containsKey("decide") && !object.containsKey("choose") && !object.containsKey("find") && !object.containsKey("questions") && !object.containsKey("recognize") && !object.containsKey("relate") && !object.containsKey("score") && !object.containsKey("tag") && !object.containsKey("version")) return RequestDefinitionAuthoredDecide.fromJson(value);
        if (object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("find") && !object.containsKey("questions") && !object.containsKey("recognize") && !object.containsKey("relate") && !object.containsKey("score") && !object.containsKey("tag") && !object.containsKey("version")) return RequestDefinitionAuthoredChoose.fromJson(value);
        if (object.containsKey("tag") && !object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("find") && !object.containsKey("questions") && !object.containsKey("recognize") && !object.containsKey("relate") && !object.containsKey("score") && !object.containsKey("version")) return RequestDefinitionAuthoredTag.fromJson(value);
        if (object.containsKey("score") && !object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("find") && !object.containsKey("questions") && !object.containsKey("recognize") && !object.containsKey("relate") && !object.containsKey("tag") && !object.containsKey("version")) return RequestDefinitionAuthoredScore.fromJson(value);
        if (object.containsKey("relate") && object.containsKey("version") && !object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("find") && !object.containsKey("questions") && !object.containsKey("recognize") && !object.containsKey("score") && !object.containsKey("tag")) return RequestDefinitionAuthoredRelate.fromJson(value);
        if (object.containsKey("find") && !object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("questions") && !object.containsKey("recognize") && !object.containsKey("relate") && !object.containsKey("score") && !object.containsKey("tag") && !object.containsKey("version")) return RequestDefinitionAuthoredFind.fromJson(value);
        if (object.containsKey("recognize") && object.containsKey("version") && !object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("find") && !object.containsKey("questions") && !object.containsKey("relate") && !object.containsKey("score") && !object.containsKey("tag")) return RequestDefinitionRecognizeDefinition.fromJson(value);
        if (object.containsKey("questions") && object.containsKey("version") && !object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("find") && !object.containsKey("recognize") && !object.containsKey("relate") && !object.containsKey("score") && !object.containsKey("tag")) return RequestDefinitionAnnotateDefinition.fromJson(value);
        return RequestDefinitionAuthoredDecide.fromJson(value);
    }
    static RequestImage readRequestImage(Object value) {
        Map<String,Object> object = Values.object(value);
        if ("file".equals(object.get("kind"))) return RequestImageFile.fromJson(value);
        if ("bytes".equals(object.get("kind"))) return RequestImageBytes.fromJson(value);
        return RequestImageFile.fromJson(value);
    }
    static RequestInput readRequestInput(Object value) {
        Map<String,Object> object = Values.object(value);
        if ("text".equals(object.get("kind"))) return RequestInputText.fromJson(value);
        if ("json".equals(object.get("kind"))) return RequestInputJson.fromJson(value);
        if ("records".equals(object.get("kind"))) return RequestInputRecords.fromJson(value);
        if ("units".equals(object.get("kind"))) return RequestInputUnits.fromJson(value);
        if ("entities".equals(object.get("kind"))) return RequestInputEntities.fromJson(value);
        if ("source".equals(object.get("kind"))) return RequestInputSource.fromJson(value);
        if ("feed".equals(object.get("kind"))) return RequestInputFeed.fromJson(value);
        return RequestInputText.fromJson(value);
    }
    static RequestOriginal readRequestOriginal(Object value) {
        Map<String,Object> object = Values.object(value);
        if ("text".equals(object.get("kind"))) return RequestOriginalText.fromJson(value);
        if ("json".equals(object.get("kind"))) return RequestOriginalJson.fromJson(value);
        return RequestOriginalText.fromJson(value);
    }
    static RequestQuestion readRequestQuestion(Object value) {
        Map<String,Object> object = Values.object(value);
        if ("text".equals(object.get("kind"))) return RequestQuestionText.fromJson(value);
        if ("definition".equals(object.get("kind"))) return RequestQuestionDefinition.fromJson(value);
        if ("file".equals(object.get("kind"))) return RequestQuestionFile.fromJson(value);
        if ("name".equals(object.get("kind"))) return RequestQuestionName.fromJson(value);
        if ("reference".equals(object.get("kind"))) return RequestQuestionReference.fromJson(value);
        return RequestQuestionText.fromJson(value);
    }
    static RequestReaderFailure readRequestReaderFailure(Object value) {
        Map<String,Object> object = Values.object(value);
        if ("producer".equals(object.get("kind"))) return RequestReaderFailureProducer.fromJson(value);
        if ("io".equals(object.get("kind"))) return RequestReaderFailureIo.fromJson(value);
        if ("utf8".equals(object.get("kind"))) return RequestReaderFailureUtf8.fromJson(value);
        if ("invalid_input".equals(object.get("kind"))) return RequestReaderFailureInvalidInput.fromJson(value);
        return RequestReaderFailureProducer.fromJson(value);
    }
    static Object tree(Object value) {
        if (value instanceof AuthoredCriterion item) return item.json();
        if (value instanceof AuthoredCut item) return item.json();
        if (value instanceof AuthoredDescription item) return item.json();
        if (value instanceof AuthoredLabels item) return item.json();
        if (value instanceof AuthoredLevels item) return item.json();
        if (value instanceof AuthoredOptions item) return item.json();
        if (value instanceof AuthoredPointers item) return item.json();
        if (value instanceof AuthoredQuestionText item) return item.json();
        if (value instanceof AuthoredThreshold item) return item.json();
        if (value instanceof CacheDocument item) return item.json();
        if (value instanceof ContextSchema item) return item.json();
        if (value instanceof ImageMedia item) return item.json();
        if (value instanceof ReaderMedia item) return item.json();
        if (value instanceof RecognitionExample item) return item.json();
        if (value instanceof RecognitionMode item) return item.json();
        if (value instanceof RequestBatch item) return item.json();
        if (value instanceof RequestDefinitionAuthoredChoosePropertiesBatch item) return item.json();
        if (value instanceof RequestDefinitionAuthoredDecidePropertiesBatch item) return item.json();
        if (value instanceof RequestDefinitionAuthoredScorePropertiesBatch item) return item.json();
        if (value instanceof RequestDefinitionAuthoredTagPropertiesBatch item) return item.json();
        if (value instanceof RequestFraming item) return item.json();
        if (value instanceof RequestFunction item) return item.json();
        if (value instanceof RequestThreshold item) return item.json();
        if (value instanceof RequestVersion item) return item.json();
        if (value instanceof SourceUnit item) return item.json();
        return value;
    }
}
