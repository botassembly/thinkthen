// Generated from the shared Rust Request schema. Do not edit.
package thinkthen;
import java.util.*;
/** Typed wire builders. Native code owns grammar and admission errors. */
public final class Inputs {
private Inputs() {}
public static final class AuthoredCriterion implements Values.Value {
private final Object value;
public AuthoredCriterion(String value) { this.value = Values.freeze(value); }
public AuthoredCriterion(Map<String,? extends Object> value) { this.value = Values.freeze(value); }
public AuthoredCriterion(List<? extends Object> value) { this.value = Values.freeze(value); }
public Object json() { return value; }
}
public static final class AuthoredCut implements Values.Value {
private final Object value;
public AuthoredCut(Number value) { this.value = Values.freeze(value); }
public AuthoredCut(String value) { this.value = Values.freeze(value); }
public Object json() { return value; }
}
public static final class AuthoredDescription implements Values.Value {
private final Object value;
public AuthoredDescription(String value) { this.value = Values.freeze(value); }
public AuthoredDescription(Map<String,? extends Object> value) { this.value = Values.freeze(value); }
public AuthoredDescription(List<? extends Object> value) { this.value = Values.freeze(value); }
public Object json() { return value; }
}
public sealed interface AuthoredInputDeclaration extends Values.Value permits AuthoredInputDeclarationString,AuthoredInputDeclarationObject {
static AuthoredInputDeclaration fromJson(Object value) { Map<String,Object> object = Values.object(value);
if ("string".equals(object.get("type"))) return AuthoredInputDeclarationString.fromJson(value);
if ("object".equals(object.get("type"))) return AuthoredInputDeclarationObject.fromJson(value);
return AuthoredInputDeclarationString.fromJson(value);
}
}
public static final class AuthoredInputDeclarationObject extends Values.Builder<AuthoredInputDeclarationObject> implements AuthoredInputDeclaration {
public AuthoredInputDeclarationObject() {
put("type", "object");
}
public static AuthoredInputDeclarationObject fromJson(Object value) { AuthoredInputDeclarationObject result = new AuthoredInputDeclarationObject(); result.omit("properties"); result.omit("required"); result.omit("type"); Values.object(value).forEach(result::extension); return result; }
protected AuthoredInputDeclarationObject self() { return this; }
public AuthoredInputDeclarationObject properties(Map<String,? extends AuthoredInputProperty> value) { return put("properties", value); }
public AuthoredInputDeclarationObject propertiesNull() { return put("properties", null); }
public AuthoredInputDeclarationObject omitProperties() { return omit("properties"); }
public AuthoredInputDeclarationObject required(List<? extends String> value) { return put("required", value); }
public AuthoredInputDeclarationObject requiredNull() { return put("required", null); }
public AuthoredInputDeclarationObject omitRequired() { return omit("required"); }
}
public static final class AuthoredInputDeclarationString extends Values.Builder<AuthoredInputDeclarationString> implements AuthoredInputDeclaration {
public AuthoredInputDeclarationString() {
put("type", "string");
}
public static AuthoredInputDeclarationString fromJson(Object value) { AuthoredInputDeclarationString result = new AuthoredInputDeclarationString(); result.omit("type"); Values.object(value).forEach(result::extension); return result; }
protected AuthoredInputDeclarationString self() { return this; }
}
public sealed interface AuthoredInputProperty extends Values.Value permits AuthoredInputPropertyString,AuthoredInputPropertyNumber,AuthoredInputPropertyBoolean,AuthoredInputPropertyArray {
static AuthoredInputProperty fromJson(Object value) { Map<String,Object> object = Values.object(value);
if ("string".equals(object.get("type"))) return AuthoredInputPropertyString.fromJson(value);
if ("number".equals(object.get("type"))) return AuthoredInputPropertyNumber.fromJson(value);
if ("boolean".equals(object.get("type"))) return AuthoredInputPropertyBoolean.fromJson(value);
if ("array".equals(object.get("type"))) return AuthoredInputPropertyArray.fromJson(value);
return AuthoredInputPropertyString.fromJson(value);
}
}
public static final class AuthoredInputPropertyArray extends Values.Builder<AuthoredInputPropertyArray> implements AuthoredInputProperty {
public AuthoredInputPropertyArray() {
put("type", "array");
}
public static AuthoredInputPropertyArray fromJson(Object value) { AuthoredInputPropertyArray result = new AuthoredInputPropertyArray(); result.omit("items"); result.omit("type"); Values.object(value).forEach(result::extension); return result; }
protected AuthoredInputPropertyArray self() { return this; }
public AuthoredInputPropertyArray items(AuthoredInputPropertyArrayPropertiesItems value) { return put("items", value); }
public AuthoredInputPropertyArray itemsNull() { return put("items", null); }
public AuthoredInputPropertyArray omitItems() { return omit("items"); }
}
public static final class AuthoredInputPropertyArrayPropertiesItems extends Values.Builder<AuthoredInputPropertyArrayPropertiesItems> {
public AuthoredInputPropertyArrayPropertiesItems() {
put("type", "string");
}
public static AuthoredInputPropertyArrayPropertiesItems fromJson(Object value) { AuthoredInputPropertyArrayPropertiesItems result = new AuthoredInputPropertyArrayPropertiesItems(); result.omit("type"); Values.object(value).forEach(result::extension); return result; }
protected AuthoredInputPropertyArrayPropertiesItems self() { return this; }
}
public static final class AuthoredInputPropertyBoolean extends Values.Builder<AuthoredInputPropertyBoolean> implements AuthoredInputProperty {
public AuthoredInputPropertyBoolean() {
put("type", "boolean");
}
public static AuthoredInputPropertyBoolean fromJson(Object value) { AuthoredInputPropertyBoolean result = new AuthoredInputPropertyBoolean(); result.omit("type"); Values.object(value).forEach(result::extension); return result; }
protected AuthoredInputPropertyBoolean self() { return this; }
}
public static final class AuthoredInputPropertyNumber extends Values.Builder<AuthoredInputPropertyNumber> implements AuthoredInputProperty {
public AuthoredInputPropertyNumber() {
put("type", "number");
}
public static AuthoredInputPropertyNumber fromJson(Object value) { AuthoredInputPropertyNumber result = new AuthoredInputPropertyNumber(); result.omit("type"); Values.object(value).forEach(result::extension); return result; }
protected AuthoredInputPropertyNumber self() { return this; }
}
public static final class AuthoredInputPropertyString extends Values.Builder<AuthoredInputPropertyString> implements AuthoredInputProperty {
public AuthoredInputPropertyString() {
put("type", "string");
}
public static AuthoredInputPropertyString fromJson(Object value) { AuthoredInputPropertyString result = new AuthoredInputPropertyString(); result.omit("type"); Values.object(value).forEach(result::extension); return result; }
protected AuthoredInputPropertyString self() { return this; }
}
public static final class AuthoredLabels implements Values.Value {
private final Object value;
public AuthoredLabels(List<? extends String> value) { this.value = Values.freeze(value); }
public AuthoredLabels(Map<String,? extends AuthoredDescription> value) { this.value = Values.freeze(value); }
public Object json() { return value; }
}
public static final class AuthoredLevels implements Values.Value {
private final Object value;
public AuthoredLevels(List<? extends String> value) { this.value = Values.freeze(value); }
public AuthoredLevels(Map<String,? extends AuthoredCriterion> value) { this.value = Values.freeze(value); }
public Object json() { return value; }
}
public static final class AuthoredOptions implements Values.Value {
private final Object value;
public AuthoredOptions(List<? extends String> value) { this.value = Values.freeze(value); }
public AuthoredOptions(Map<String,? extends AuthoredDescription> value) { this.value = Values.freeze(value); }
public Object json() { return value; }
}
public static final class AuthoredPointers implements Values.Value {
private final Object value;
public AuthoredPointers(String value) { this.value = Values.freeze(value); }
public AuthoredPointers(List<? extends String> value) { this.value = Values.freeze(value); }
public Object json() { return value; }
}
public static final class AuthoredQuestionText implements Values.Value {
private final Object value;
public AuthoredQuestionText(String value) { this.value = Values.freeze(value); }
public AuthoredQuestionText(Map<String,? extends Object> value) { this.value = Values.freeze(value); }
public AuthoredQuestionText(List<? extends Object> value) { this.value = Values.freeze(value); }
public Object json() { return value; }
}
public static final class AuthoredRelation extends Values.Builder<AuthoredRelation> {
public AuthoredRelation() {
}
public static AuthoredRelation fromJson(Object value) { AuthoredRelation result = new AuthoredRelation(); result.omit("either"); result.omit("name"); result.omit("reads"); result.omit("single"); result.omit("source"); result.omit("target"); Values.object(value).forEach(result::extension); return result; }
protected AuthoredRelation self() { return this; }
public AuthoredRelation either(Boolean value) { return put("either", value); }
public AuthoredRelation eitherNull() { return put("either", null); }
public AuthoredRelation omitEither() { return omit("either"); }
public AuthoredRelation name(String value) { return put("name", value); }
public AuthoredRelation nameNull() { return put("name", null); }
public AuthoredRelation omitName() { return omit("name"); }
public AuthoredRelation reads(String value) { return put("reads", value); }
public AuthoredRelation readsNull() { return put("reads", null); }
public AuthoredRelation omitReads() { return omit("reads"); }
public AuthoredRelation single(Boolean value) { return put("single", value); }
public AuthoredRelation singleNull() { return put("single", null); }
public AuthoredRelation omitSingle() { return omit("single"); }
public AuthoredRelation source(String value) { return put("source", value); }
public AuthoredRelation sourceNull() { return put("source", null); }
public AuthoredRelation omitSource() { return omit("source"); }
public AuthoredRelation target(String value) { return put("target", value); }
public AuthoredRelation targetNull() { return put("target", null); }
public AuthoredRelation omitTarget() { return omit("target"); }
}
public static final class AuthoredThreshold implements Values.Value {
private final Object value;
public AuthoredThreshold(Number value) { this.value = Values.freeze(value); }
public AuthoredThreshold(String value) { this.value = Values.freeze(value); }
public Object json() { return value; }
}
public static final class CacheDocument implements Values.Value {
private final Object value;
public CacheDocument(String value) { this.value = Values.freeze(value); }
public CacheDocument(Boolean value) { this.value = Values.freeze(value); }
public Object json() { return value; }
}
public static final class ContextSchema implements Values.Value {
private final Object value;
public ContextSchema(String value) { this.value = Values.freeze(value); }
public ContextSchema(Map<String,? extends Object> value) { this.value = Values.freeze(value); }
public Object json() { return value; }
}
public static final class EngineSettings extends Values.Builder<EngineSettings> {
public EngineSettings() {
}
public static EngineSettings fromJson(Object value) { EngineSettings result = new EngineSettings(); result.omit("backend"); result.omit("base_url"); result.omit("batch"); result.omit("cache"); result.omit("max_estimated_input_tokens_total"); result.omit("max_request_bytes"); result.omit("max_requests"); result.omit("max_requests_total"); result.omit("max_retries"); result.omit("model"); result.omit("profile"); result.omit("proxy"); result.omit("record"); result.omit("refresh_cache"); result.omit("replay"); result.omit("throttle"); result.omit("timeout"); result.omit("usd_per_million_input"); result.omit("usd_per_million_output"); Values.object(value).forEach(result::extension); return result; }
protected EngineSettings self() { return this; }
public EngineSettings backend(String value) { return put("backend", value); }
public EngineSettings backendNull() { return put("backend", null); }
public EngineSettings omitBackend() { return omit("backend"); }
public EngineSettings baseUrl(String value) { return put("base_url", value); }
public EngineSettings baseUrlNull() { return put("base_url", null); }
public EngineSettings omitBaseUrl() { return omit("base_url"); }
public EngineSettings batch(RequestBatch value) { return put("batch", value); }
public EngineSettings batchNull() { return put("batch", null); }
public EngineSettings omitBatch() { return omit("batch"); }
public EngineSettings cache(CacheDocument value) { return put("cache", value); }
public EngineSettings cacheNull() { return put("cache", null); }
public EngineSettings omitCache() { return omit("cache"); }
public EngineSettings maxEstimatedInputTokensTotal(Number value) { return put("max_estimated_input_tokens_total", value); }
public EngineSettings maxEstimatedInputTokensTotalNull() { return put("max_estimated_input_tokens_total", null); }
public EngineSettings omitMaxEstimatedInputTokensTotal() { return omit("max_estimated_input_tokens_total"); }
public EngineSettings maxRequestBytes(Number value) { return put("max_request_bytes", value); }
public EngineSettings maxRequestBytesNull() { return put("max_request_bytes", null); }
public EngineSettings omitMaxRequestBytes() { return omit("max_request_bytes"); }
public EngineSettings maxRequests(Number value) { return put("max_requests", value); }
public EngineSettings maxRequestsNull() { return put("max_requests", null); }
public EngineSettings omitMaxRequests() { return omit("max_requests"); }
public EngineSettings maxRequestsTotal(Number value) { return put("max_requests_total", value); }
public EngineSettings maxRequestsTotalNull() { return put("max_requests_total", null); }
public EngineSettings omitMaxRequestsTotal() { return omit("max_requests_total"); }
public EngineSettings maxRetries(Number value) { return put("max_retries", value); }
public EngineSettings maxRetriesNull() { return put("max_retries", null); }
public EngineSettings omitMaxRetries() { return omit("max_retries"); }
public EngineSettings model(String value) { return put("model", value); }
public EngineSettings modelNull() { return put("model", null); }
public EngineSettings omitModel() { return omit("model"); }
public EngineSettings profile(String value) { return put("profile", value); }
public EngineSettings profileNull() { return put("profile", null); }
public EngineSettings omitProfile() { return omit("profile"); }
public EngineSettings proxy(Object value) { return put("proxy", value); }
public EngineSettings proxyNull() { return put("proxy", null); }
public EngineSettings omitProxy() { return omit("proxy"); }
public EngineSettings record(String value) { return put("record", value); }
public EngineSettings recordNull() { return put("record", null); }
public EngineSettings omitRecord() { return omit("record"); }
public EngineSettings refreshCache(Boolean value) { return put("refresh_cache", value); }
public EngineSettings refreshCacheNull() { return put("refresh_cache", null); }
public EngineSettings omitRefreshCache() { return omit("refresh_cache"); }
public EngineSettings replay(String value) { return put("replay", value); }
public EngineSettings replayNull() { return put("replay", null); }
public EngineSettings omitReplay() { return omit("replay"); }
public EngineSettings throttle(Number value) { return put("throttle", value); }
public EngineSettings throttleNull() { return put("throttle", null); }
public EngineSettings omitThrottle() { return omit("throttle"); }
public EngineSettings timeout(Number value) { return put("timeout", value); }
public EngineSettings timeoutNull() { return put("timeout", null); }
public EngineSettings omitTimeout() { return omit("timeout"); }
public EngineSettings usdPerMillionInput(String value) { return put("usd_per_million_input", value); }
public EngineSettings usdPerMillionInputNull() { return put("usd_per_million_input", null); }
public EngineSettings omitUsdPerMillionInput() { return omit("usd_per_million_input"); }
public EngineSettings usdPerMillionOutput(String value) { return put("usd_per_million_output", value); }
public EngineSettings usdPerMillionOutputNull() { return put("usd_per_million_output", null); }
public EngineSettings omitUsdPerMillionOutput() { return omit("usd_per_million_output"); }
}
public enum ImageMedia implements Values.Value { IMAGEJPEG("image/jpeg"),IMAGEPNG("image/png");
private final String value; ImageMedia(String value) { this.value = value; }
public Object json() { return value; }
}
public static final class OptionSchema extends Values.Builder<OptionSchema> {
public OptionSchema() {
}
public static OptionSchema fromJson(Object value) { OptionSchema result = new OptionSchema(); result.omit("description"); result.omit("name"); Values.object(value).forEach(result::extension); return result; }
protected OptionSchema self() { return this; }
public OptionSchema description(Object value) { return put("description", value); }
public OptionSchema descriptionNull() { return put("description", null); }
public OptionSchema omitDescription() { return omit("description"); }
public OptionSchema name(String value) { return put("name", value); }
public OptionSchema nameNull() { return put("name", null); }
public OptionSchema omitName() { return omit("name"); }
}
public enum ReaderMedia implements Values.Value { TEXT("text"),IMAGE("image");
private final String value; ReaderMedia(String value) { this.value = value; }
public Object json() { return value; }
}
public static final class RecognitionExample implements Values.Value {
private final Object value;
public RecognitionExample(String value) { this.value = Values.freeze(value); }
public RecognitionExample(RecognitionExampleText value) { this.value = Values.freeze(value); }
public Object json() { return value; }
}
public static final class RecognitionExampleEntity extends Values.Builder<RecognitionExampleEntity> {
public RecognitionExampleEntity() {
}
public static RecognitionExampleEntity fromJson(Object value) { RecognitionExampleEntity result = new RecognitionExampleEntity(); result.omit("end"); result.omit("kind"); result.omit("start"); Values.object(value).forEach(result::extension); return result; }
protected RecognitionExampleEntity self() { return this; }
public RecognitionExampleEntity end(Number value) { return put("end", value); }
public RecognitionExampleEntity endNull() { return put("end", null); }
public RecognitionExampleEntity omitEnd() { return omit("end"); }
public RecognitionExampleEntity kind(String value) { return put("kind", value); }
public RecognitionExampleEntity kindNull() { return put("kind", null); }
public RecognitionExampleEntity omitKind() { return omit("kind"); }
public RecognitionExampleEntity start(Number value) { return put("start", value); }
public RecognitionExampleEntity startNull() { return put("start", null); }
public RecognitionExampleEntity omitStart() { return omit("start"); }
}
public static final class RecognitionExampleText extends Values.Builder<RecognitionExampleText> {
public RecognitionExampleText() {
}
public static RecognitionExampleText fromJson(Object value) { RecognitionExampleText result = new RecognitionExampleText(); result.omit("entities"); result.omit("kinds"); result.omit("text"); Values.object(value).forEach(result::extension); return result; }
protected RecognitionExampleText self() { return this; }
public RecognitionExampleText entities(List<? extends RecognitionExampleEntity> value) { return put("entities", value); }
public RecognitionExampleText entitiesNull() { return put("entities", null); }
public RecognitionExampleText omitEntities() { return omit("entities"); }
public RecognitionExampleText kinds(List<? extends String> value) { return put("kinds", value); }
public RecognitionExampleText kindsNull() { return put("kinds", null); }
public RecognitionExampleText omitKinds() { return omit("kinds"); }
public RecognitionExampleText text(String value) { return put("text", value); }
public RecognitionExampleText textNull() { return put("text", null); }
public RecognitionExampleText omitText() { return omit("text"); }
}
public enum RecognitionMode implements Values.Value { WHOLE("whole"),BOUNDARYONLY("boundary_only");
private final String value; RecognitionMode(String value) { this.value = value; }
public Object json() { return value; }
}
public static final class RecognitionSeedSpan extends Values.Builder<RecognitionSeedSpan> {
public RecognitionSeedSpan() {
}
public static RecognitionSeedSpan fromJson(Object value) { RecognitionSeedSpan result = new RecognitionSeedSpan(); result.omit("end"); result.omit("kind"); result.omit("start"); Values.object(value).forEach(result::extension); return result; }
protected RecognitionSeedSpan self() { return this; }
public RecognitionSeedSpan end(Number value) { return put("end", value); }
public RecognitionSeedSpan endNull() { return put("end", null); }
public RecognitionSeedSpan omitEnd() { return omit("end"); }
public RecognitionSeedSpan kind(String value) { return put("kind", value); }
public RecognitionSeedSpan kindNull() { return put("kind", null); }
public RecognitionSeedSpan omitKind() { return omit("kind"); }
public RecognitionSeedSpan start(Number value) { return put("start", value); }
public RecognitionSeedSpan startNull() { return put("start", null); }
public RecognitionSeedSpan omitStart() { return omit("start"); }
}
public static final class RecognitionStageContext extends Values.Builder<RecognitionStageContext> {
public RecognitionStageContext() {
}
public static RecognitionStageContext fromJson(Object value) { RecognitionStageContext result = new RecognitionStageContext(); result.omit("boundary"); result.omit("kind_edge"); result.omit("relation"); Values.object(value).forEach(result::extension); return result; }
protected RecognitionStageContext self() { return this; }
public RecognitionStageContext boundary(String value) { return put("boundary", value); }
public RecognitionStageContext boundaryNull() { return put("boundary", null); }
public RecognitionStageContext omitBoundary() { return omit("boundary"); }
public RecognitionStageContext kindEdge(String value) { return put("kind_edge", value); }
public RecognitionStageContext kindEdgeNull() { return put("kind_edge", null); }
public RecognitionStageContext omitKindEdge() { return omit("kind_edge"); }
public RecognitionStageContext relation(String value) { return put("relation", value); }
public RecognitionStageContext relationNull() { return put("relation", null); }
public RecognitionStageContext omitRelation() { return omit("relation"); }
}
public static final class Request extends Values.Builder<Request> {
public Request() {
put("schema", "thinkthen.request/1");
}
public static Request fromJson(Object value) { Request result = new Request(); result.omit("call"); result.omit("schema"); Values.object(value).forEach(result::extension); return result; }
protected Request self() { return this; }
public Request call(RequestCall value) { return put("call", value); }
public Request callNull() { return put("call", null); }
public Request omitCall() { return omit("call"); }
public Request schema(RequestVersion value) { return put("schema", value); }
public Request schemaNull() { return put("schema", null); }
public Request omitSchema() { return omit("schema"); }
}
public static final class RequestBatch implements Values.Value {
private final Object value;
public RequestBatch(Number value) { this.value = Values.freeze(value); }
public RequestBatch(String value) { this.value = Values.freeze(value); }
public Object json() { return value; }
}
public sealed interface RequestCall extends Values.Value permits RequestCallDecide,RequestCallChoose,RequestCallTag,RequestCallScore,RequestCallFilter,RequestCallRank,RequestCallFind,RequestCallAnnotate,RequestCallRecognize,RequestCallRelate {
static RequestCall fromJson(Object value) { Map<String,Object> object = Values.object(value);
if ("decide".equals(object.get("function"))) return RequestCallDecide.fromJson(value);
if ("choose".equals(object.get("function"))) return RequestCallChoose.fromJson(value);
if ("tag".equals(object.get("function"))) return RequestCallTag.fromJson(value);
if ("score".equals(object.get("function"))) return RequestCallScore.fromJson(value);
if ("filter".equals(object.get("function"))) return RequestCallFilter.fromJson(value);
if ("rank".equals(object.get("function"))) return RequestCallRank.fromJson(value);
if ("find".equals(object.get("function"))) return RequestCallFind.fromJson(value);
if ("annotate".equals(object.get("function"))) return RequestCallAnnotate.fromJson(value);
if ("recognize".equals(object.get("function"))) return RequestCallRecognize.fromJson(value);
if ("relate".equals(object.get("function"))) return RequestCallRelate.fromJson(value);
return RequestCallDecide.fromJson(value);
}
}
public static final class RequestCallAnnotate extends Values.Builder<RequestCallAnnotate> implements RequestCall {
public RequestCallAnnotate() {
put("function", "annotate");
}
public static RequestCallAnnotate fromJson(Object value) { RequestCallAnnotate result = new RequestCallAnnotate(); result.omit("function"); result.omit("input"); result.omit("options"); result.omit("question"); Values.object(value).forEach(result::extension); return result; }
protected RequestCallAnnotate self() { return this; }
public RequestCallAnnotate input(RequestInput value) { return put("input", value); }
public RequestCallAnnotate inputNull() { return put("input", null); }
public RequestCallAnnotate omitInput() { return omit("input"); }
public RequestCallAnnotate options(RequestOptions value) { return put("options", value); }
public RequestCallAnnotate optionsNull() { return put("options", null); }
public RequestCallAnnotate omitOptions() { return omit("options"); }
public RequestCallAnnotate question(RequestQuestion value) { return put("question", value); }
public RequestCallAnnotate questionNull() { return put("question", null); }
public RequestCallAnnotate omitQuestion() { return omit("question"); }
}
public static final class RequestCallChoose extends Values.Builder<RequestCallChoose> implements RequestCall {
public RequestCallChoose() {
put("function", "choose");
}
public static RequestCallChoose fromJson(Object value) { RequestCallChoose result = new RequestCallChoose(); result.omit("function"); result.omit("input"); result.omit("options"); result.omit("question"); Values.object(value).forEach(result::extension); return result; }
protected RequestCallChoose self() { return this; }
public RequestCallChoose input(RequestInput value) { return put("input", value); }
public RequestCallChoose inputNull() { return put("input", null); }
public RequestCallChoose omitInput() { return omit("input"); }
public RequestCallChoose options(RequestOptions value) { return put("options", value); }
public RequestCallChoose optionsNull() { return put("options", null); }
public RequestCallChoose omitOptions() { return omit("options"); }
public RequestCallChoose question(RequestQuestion value) { return put("question", value); }
public RequestCallChoose questionNull() { return put("question", null); }
public RequestCallChoose omitQuestion() { return omit("question"); }
}
public static final class RequestCallDecide extends Values.Builder<RequestCallDecide> implements RequestCall {
public RequestCallDecide() {
put("function", "decide");
}
public static RequestCallDecide fromJson(Object value) { RequestCallDecide result = new RequestCallDecide(); result.omit("function"); result.omit("input"); result.omit("options"); result.omit("question"); Values.object(value).forEach(result::extension); return result; }
protected RequestCallDecide self() { return this; }
public RequestCallDecide input(RequestInput value) { return put("input", value); }
public RequestCallDecide inputNull() { return put("input", null); }
public RequestCallDecide omitInput() { return omit("input"); }
public RequestCallDecide options(RequestOptions value) { return put("options", value); }
public RequestCallDecide optionsNull() { return put("options", null); }
public RequestCallDecide omitOptions() { return omit("options"); }
public RequestCallDecide question(RequestQuestion value) { return put("question", value); }
public RequestCallDecide questionNull() { return put("question", null); }
public RequestCallDecide omitQuestion() { return omit("question"); }
}
public static final class RequestCallFilter extends Values.Builder<RequestCallFilter> implements RequestCall {
public RequestCallFilter() {
put("function", "filter");
}
public static RequestCallFilter fromJson(Object value) { RequestCallFilter result = new RequestCallFilter(); result.omit("function"); result.omit("input"); result.omit("options"); result.omit("question"); Values.object(value).forEach(result::extension); return result; }
protected RequestCallFilter self() { return this; }
public RequestCallFilter input(RequestInput value) { return put("input", value); }
public RequestCallFilter inputNull() { return put("input", null); }
public RequestCallFilter omitInput() { return omit("input"); }
public RequestCallFilter options(RequestOptions value) { return put("options", value); }
public RequestCallFilter optionsNull() { return put("options", null); }
public RequestCallFilter omitOptions() { return omit("options"); }
public RequestCallFilter question(RequestQuestion value) { return put("question", value); }
public RequestCallFilter questionNull() { return put("question", null); }
public RequestCallFilter omitQuestion() { return omit("question"); }
}
public static final class RequestCallFind extends Values.Builder<RequestCallFind> implements RequestCall {
public RequestCallFind() {
put("function", "find");
}
public static RequestCallFind fromJson(Object value) { RequestCallFind result = new RequestCallFind(); result.omit("function"); result.omit("input"); result.omit("options"); result.omit("question"); Values.object(value).forEach(result::extension); return result; }
protected RequestCallFind self() { return this; }
public RequestCallFind input(RequestInput value) { return put("input", value); }
public RequestCallFind inputNull() { return put("input", null); }
public RequestCallFind omitInput() { return omit("input"); }
public RequestCallFind options(RequestOptions value) { return put("options", value); }
public RequestCallFind optionsNull() { return put("options", null); }
public RequestCallFind omitOptions() { return omit("options"); }
public RequestCallFind question(RequestQuestion value) { return put("question", value); }
public RequestCallFind questionNull() { return put("question", null); }
public RequestCallFind omitQuestion() { return omit("question"); }
}
public static final class RequestCallRank extends Values.Builder<RequestCallRank> implements RequestCall {
public RequestCallRank() {
put("function", "rank");
}
public static RequestCallRank fromJson(Object value) { RequestCallRank result = new RequestCallRank(); result.omit("function"); result.omit("input"); result.omit("options"); result.omit("question"); Values.object(value).forEach(result::extension); return result; }
protected RequestCallRank self() { return this; }
public RequestCallRank input(RequestInput value) { return put("input", value); }
public RequestCallRank inputNull() { return put("input", null); }
public RequestCallRank omitInput() { return omit("input"); }
public RequestCallRank options(RequestOptions value) { return put("options", value); }
public RequestCallRank optionsNull() { return put("options", null); }
public RequestCallRank omitOptions() { return omit("options"); }
public RequestCallRank question(RequestQuestion value) { return put("question", value); }
public RequestCallRank questionNull() { return put("question", null); }
public RequestCallRank omitQuestion() { return omit("question"); }
}
public static final class RequestCallRecognize extends Values.Builder<RequestCallRecognize> implements RequestCall {
public RequestCallRecognize() {
put("function", "recognize");
}
public static RequestCallRecognize fromJson(Object value) { RequestCallRecognize result = new RequestCallRecognize(); result.omit("function"); result.omit("input"); result.omit("options"); result.omit("question"); Values.object(value).forEach(result::extension); return result; }
protected RequestCallRecognize self() { return this; }
public RequestCallRecognize input(RequestInput value) { return put("input", value); }
public RequestCallRecognize inputNull() { return put("input", null); }
public RequestCallRecognize omitInput() { return omit("input"); }
public RequestCallRecognize options(RequestOptions value) { return put("options", value); }
public RequestCallRecognize optionsNull() { return put("options", null); }
public RequestCallRecognize omitOptions() { return omit("options"); }
public RequestCallRecognize question(RequestQuestion value) { return put("question", value); }
public RequestCallRecognize questionNull() { return put("question", null); }
public RequestCallRecognize omitQuestion() { return omit("question"); }
}
public static final class RequestCallRelate extends Values.Builder<RequestCallRelate> implements RequestCall {
public RequestCallRelate() {
put("function", "relate");
}
public static RequestCallRelate fromJson(Object value) { RequestCallRelate result = new RequestCallRelate(); result.omit("function"); result.omit("input"); result.omit("options"); result.omit("question"); Values.object(value).forEach(result::extension); return result; }
protected RequestCallRelate self() { return this; }
public RequestCallRelate input(RequestInput value) { return put("input", value); }
public RequestCallRelate inputNull() { return put("input", null); }
public RequestCallRelate omitInput() { return omit("input"); }
public RequestCallRelate options(RequestOptions value) { return put("options", value); }
public RequestCallRelate optionsNull() { return put("options", null); }
public RequestCallRelate omitOptions() { return omit("options"); }
public RequestCallRelate question(RequestQuestion value) { return put("question", value); }
public RequestCallRelate questionNull() { return put("question", null); }
public RequestCallRelate omitQuestion() { return omit("question"); }
}
public static final class RequestCallScore extends Values.Builder<RequestCallScore> implements RequestCall {
public RequestCallScore() {
put("function", "score");
}
public static RequestCallScore fromJson(Object value) { RequestCallScore result = new RequestCallScore(); result.omit("function"); result.omit("input"); result.omit("options"); result.omit("question"); Values.object(value).forEach(result::extension); return result; }
protected RequestCallScore self() { return this; }
public RequestCallScore input(RequestInput value) { return put("input", value); }
public RequestCallScore inputNull() { return put("input", null); }
public RequestCallScore omitInput() { return omit("input"); }
public RequestCallScore options(RequestOptions value) { return put("options", value); }
public RequestCallScore optionsNull() { return put("options", null); }
public RequestCallScore omitOptions() { return omit("options"); }
public RequestCallScore question(RequestQuestion value) { return put("question", value); }
public RequestCallScore questionNull() { return put("question", null); }
public RequestCallScore omitQuestion() { return omit("question"); }
}
public static final class RequestCallTag extends Values.Builder<RequestCallTag> implements RequestCall {
public RequestCallTag() {
put("function", "tag");
}
public static RequestCallTag fromJson(Object value) { RequestCallTag result = new RequestCallTag(); result.omit("function"); result.omit("input"); result.omit("options"); result.omit("question"); Values.object(value).forEach(result::extension); return result; }
protected RequestCallTag self() { return this; }
public RequestCallTag input(RequestInput value) { return put("input", value); }
public RequestCallTag inputNull() { return put("input", null); }
public RequestCallTag omitInput() { return omit("input"); }
public RequestCallTag options(RequestOptions value) { return put("options", value); }
public RequestCallTag optionsNull() { return put("options", null); }
public RequestCallTag omitOptions() { return omit("options"); }
public RequestCallTag question(RequestQuestion value) { return put("question", value); }
public RequestCallTag questionNull() { return put("question", null); }
public RequestCallTag omitQuestion() { return omit("question"); }
}
public sealed interface RequestDefinition extends Values.Value permits RequestDefinitionFieldsDecide,RequestDefinitionFieldsChoose,RequestDefinitionFieldsTag,RequestDefinitionFieldsScore,RequestDefinitionFieldsRelateVersion,RequestDefinitionFieldsFind,RequestDefinitionFieldsRecognizeVersion,RequestDefinitionFieldsQuestionsVersion {
static RequestDefinition fromJson(Object value) { Map<String,Object> object = Values.object(value);
if (object.containsKey("decide") && !object.containsKey("choose") && !object.containsKey("find") && !object.containsKey("questions") && !object.containsKey("recognize") && !object.containsKey("relate") && !object.containsKey("score") && !object.containsKey("tag") && !object.containsKey("version")) return RequestDefinitionFieldsDecide.fromJson(value);
if (object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("find") && !object.containsKey("questions") && !object.containsKey("recognize") && !object.containsKey("relate") && !object.containsKey("score") && !object.containsKey("tag") && !object.containsKey("version")) return RequestDefinitionFieldsChoose.fromJson(value);
if (object.containsKey("tag") && !object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("find") && !object.containsKey("questions") && !object.containsKey("recognize") && !object.containsKey("relate") && !object.containsKey("score") && !object.containsKey("version")) return RequestDefinitionFieldsTag.fromJson(value);
if (object.containsKey("score") && !object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("find") && !object.containsKey("questions") && !object.containsKey("recognize") && !object.containsKey("relate") && !object.containsKey("tag") && !object.containsKey("version")) return RequestDefinitionFieldsScore.fromJson(value);
if (object.containsKey("relate") && object.containsKey("version") && !object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("find") && !object.containsKey("questions") && !object.containsKey("recognize") && !object.containsKey("score") && !object.containsKey("tag")) return RequestDefinitionFieldsRelateVersion.fromJson(value);
if (object.containsKey("find") && !object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("questions") && !object.containsKey("recognize") && !object.containsKey("relate") && !object.containsKey("score") && !object.containsKey("tag") && !object.containsKey("version")) return RequestDefinitionFieldsFind.fromJson(value);
if (object.containsKey("recognize") && object.containsKey("version") && !object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("find") && !object.containsKey("questions") && !object.containsKey("relate") && !object.containsKey("score") && !object.containsKey("tag")) return RequestDefinitionFieldsRecognizeVersion.fromJson(value);
if (object.containsKey("questions") && object.containsKey("version") && !object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("find") && !object.containsKey("recognize") && !object.containsKey("relate") && !object.containsKey("score") && !object.containsKey("tag")) return RequestDefinitionFieldsQuestionsVersion.fromJson(value);
return RequestDefinitionFieldsDecide.fromJson(value);
}
}
public sealed interface RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties extends Values.Value permits RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide,RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose,RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag,RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore {
static RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties fromJson(Object value) { Map<String,Object> object = Values.object(value);
if (object.containsKey("decide") && !object.containsKey("choose") && !object.containsKey("score") && !object.containsKey("tag")) return RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide.fromJson(value);
if (object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("score") && !object.containsKey("tag")) return RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose.fromJson(value);
if (object.containsKey("tag") && !object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("score")) return RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag.fromJson(value);
if (object.containsKey("score") && !object.containsKey("choose") && !object.containsKey("decide") && !object.containsKey("tag")) return RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore.fromJson(value);
return RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide.fromJson(value);
}
}
public static final class RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose extends Values.Builder<RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose> implements RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties {
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose() {
}
public static RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose fromJson(Object value) { RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose result = new RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose(); result.omit("choose"); result.omit("context_schema"); result.omit("item_schema"); result.omit("name"); result.omit("on"); result.omit("options"); result.omit("threshold"); result.omit("wording_version"); Values.object(value).forEach(result::extension); return result; }
protected RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose self() { return this; }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose choose(AuthoredQuestionText value) { return put("choose", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose chooseNull() { return put("choose", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose omitChoose() { return omit("choose"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose contextSchema(AuthoredInputDeclaration value) { return put("context_schema", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose contextSchemaNull() { return put("context_schema", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose omitContextSchema() { return omit("context_schema"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose itemSchema(AuthoredInputDeclaration value) { return put("item_schema", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose itemSchemaNull() { return put("item_schema", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose omitItemSchema() { return omit("item_schema"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose name(String value) { return put("name", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose nameNull() { return put("name", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose omitName() { return omit("name"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose on(AuthoredPointers value) { return put("on", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose onNull() { return put("on", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose omitOn() { return omit("on"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose options(AuthoredOptions value) { return put("options", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose optionsNull() { return put("options", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose omitOptions() { return omit("options"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose threshold(AuthoredCut value) { return put("threshold", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose thresholdNull() { return put("threshold", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose omitThreshold() { return omit("threshold"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose wordingVersion(Number value) { return put("wording_version", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose wordingVersionNull() { return put("wording_version", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose omitWordingVersion() { return omit("wording_version"); }
}
public static final class RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide extends Values.Builder<RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide> implements RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties {
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide() {
}
public static RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide fromJson(Object value) { RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide result = new RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide(); result.omit("context_schema"); result.omit("decide"); result.omit("false"); result.omit("item_schema"); result.omit("name"); result.omit("on"); result.omit("threshold"); result.omit("true"); result.omit("wording_version"); Values.object(value).forEach(result::extension); return result; }
protected RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide self() { return this; }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide contextSchema(AuthoredInputDeclaration value) { return put("context_schema", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide contextSchemaNull() { return put("context_schema", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide omitContextSchema() { return omit("context_schema"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide decide(AuthoredQuestionText value) { return put("decide", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide decideNull() { return put("decide", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide omitDecide() { return omit("decide"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide falseValue(AuthoredCriterion value) { return put("false", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide falseValueNull() { return put("false", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide omitFalseValue() { return omit("false"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide itemSchema(AuthoredInputDeclaration value) { return put("item_schema", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide itemSchemaNull() { return put("item_schema", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide omitItemSchema() { return omit("item_schema"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide name(String value) { return put("name", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide nameNull() { return put("name", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide omitName() { return omit("name"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide on(AuthoredPointers value) { return put("on", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide onNull() { return put("on", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide omitOn() { return omit("on"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide threshold(AuthoredThreshold value) { return put("threshold", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide thresholdNull() { return put("threshold", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide omitThreshold() { return omit("threshold"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide trueValue(AuthoredCriterion value) { return put("true", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide trueValueNull() { return put("true", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide omitTrueValue() { return omit("true"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide wordingVersion(Number value) { return put("wording_version", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide wordingVersionNull() { return put("wording_version", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide omitWordingVersion() { return omit("wording_version"); }
}
public static final class RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore extends Values.Builder<RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore> implements RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties {
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore() {
}
public static RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore fromJson(Object value) { RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore result = new RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore(); result.omit("context_schema"); result.omit("item_schema"); result.omit("levels"); result.omit("name"); result.omit("on"); result.omit("score"); result.omit("wording_version"); Values.object(value).forEach(result::extension); return result; }
protected RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore self() { return this; }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore contextSchema(AuthoredInputDeclaration value) { return put("context_schema", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore contextSchemaNull() { return put("context_schema", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore omitContextSchema() { return omit("context_schema"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore itemSchema(AuthoredInputDeclaration value) { return put("item_schema", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore itemSchemaNull() { return put("item_schema", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore omitItemSchema() { return omit("item_schema"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore levels(AuthoredLevels value) { return put("levels", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore levelsNull() { return put("levels", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore omitLevels() { return omit("levels"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore name(String value) { return put("name", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore nameNull() { return put("name", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore omitName() { return omit("name"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore on(AuthoredPointers value) { return put("on", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore onNull() { return put("on", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore omitOn() { return omit("on"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore score(AuthoredQuestionText value) { return put("score", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore scoreNull() { return put("score", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore omitScore() { return omit("score"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore wordingVersion(Number value) { return put("wording_version", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore wordingVersionNull() { return put("wording_version", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore omitWordingVersion() { return omit("wording_version"); }
}
public static final class RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag extends Values.Builder<RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag> implements RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties {
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag() {
}
public static RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag fromJson(Object value) { RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag result = new RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag(); result.omit("context_schema"); result.omit("item_schema"); result.omit("labels"); result.omit("name"); result.omit("on"); result.omit("tag"); result.omit("threshold"); result.omit("wording_version"); Values.object(value).forEach(result::extension); return result; }
protected RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag self() { return this; }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag contextSchema(AuthoredInputDeclaration value) { return put("context_schema", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag contextSchemaNull() { return put("context_schema", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag omitContextSchema() { return omit("context_schema"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag itemSchema(AuthoredInputDeclaration value) { return put("item_schema", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag itemSchemaNull() { return put("item_schema", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag omitItemSchema() { return omit("item_schema"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag labels(AuthoredLabels value) { return put("labels", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag labelsNull() { return put("labels", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag omitLabels() { return omit("labels"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag name(String value) { return put("name", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag nameNull() { return put("name", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag omitName() { return omit("name"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag on(AuthoredPointers value) { return put("on", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag onNull() { return put("on", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag omitOn() { return omit("on"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag tag(AuthoredQuestionText value) { return put("tag", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag tagNull() { return put("tag", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag omitTag() { return omit("tag"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag threshold(AuthoredCut value) { return put("threshold", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag thresholdNull() { return put("threshold", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag omitThreshold() { return omit("threshold"); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag wordingVersion(Number value) { return put("wording_version", value); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag wordingVersionNull() { return put("wording_version", null); }
public RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag omitWordingVersion() { return omit("wording_version"); }
}
public static final class RequestDefinitionFieldsChoose extends Values.Builder<RequestDefinitionFieldsChoose> implements RequestDefinition {
public RequestDefinitionFieldsChoose() {
}
public static RequestDefinitionFieldsChoose fromJson(Object value) { RequestDefinitionFieldsChoose result = new RequestDefinitionFieldsChoose(); result.omit("batch"); result.omit("choose"); result.omit("context_schema"); result.omit("item_schema"); result.omit("model"); result.omit("name"); result.omit("on"); result.omit("options"); result.omit("profile"); result.omit("threshold"); result.omit("wording_version"); Values.object(value).forEach(result::extension); return result; }
protected RequestDefinitionFieldsChoose self() { return this; }
public RequestDefinitionFieldsChoose batch(RequestDefinitionFieldsChoosePropertiesBatch value) { return put("batch", value); }
public RequestDefinitionFieldsChoose batchNull() { return put("batch", null); }
public RequestDefinitionFieldsChoose omitBatch() { return omit("batch"); }
public RequestDefinitionFieldsChoose choose(AuthoredQuestionText value) { return put("choose", value); }
public RequestDefinitionFieldsChoose chooseNull() { return put("choose", null); }
public RequestDefinitionFieldsChoose omitChoose() { return omit("choose"); }
public RequestDefinitionFieldsChoose contextSchema(AuthoredInputDeclaration value) { return put("context_schema", value); }
public RequestDefinitionFieldsChoose contextSchemaNull() { return put("context_schema", null); }
public RequestDefinitionFieldsChoose omitContextSchema() { return omit("context_schema"); }
public RequestDefinitionFieldsChoose itemSchema(AuthoredInputDeclaration value) { return put("item_schema", value); }
public RequestDefinitionFieldsChoose itemSchemaNull() { return put("item_schema", null); }
public RequestDefinitionFieldsChoose omitItemSchema() { return omit("item_schema"); }
public RequestDefinitionFieldsChoose model(String value) { return put("model", value); }
public RequestDefinitionFieldsChoose modelNull() { return put("model", null); }
public RequestDefinitionFieldsChoose omitModel() { return omit("model"); }
public RequestDefinitionFieldsChoose name(String value) { return put("name", value); }
public RequestDefinitionFieldsChoose nameNull() { return put("name", null); }
public RequestDefinitionFieldsChoose omitName() { return omit("name"); }
public RequestDefinitionFieldsChoose on(AuthoredPointers value) { return put("on", value); }
public RequestDefinitionFieldsChoose onNull() { return put("on", null); }
public RequestDefinitionFieldsChoose omitOn() { return omit("on"); }
public RequestDefinitionFieldsChoose options(AuthoredOptions value) { return put("options", value); }
public RequestDefinitionFieldsChoose optionsNull() { return put("options", null); }
public RequestDefinitionFieldsChoose omitOptions() { return omit("options"); }
public RequestDefinitionFieldsChoose profile(String value) { return put("profile", value); }
public RequestDefinitionFieldsChoose profileNull() { return put("profile", null); }
public RequestDefinitionFieldsChoose omitProfile() { return omit("profile"); }
public RequestDefinitionFieldsChoose threshold(AuthoredCut value) { return put("threshold", value); }
public RequestDefinitionFieldsChoose thresholdNull() { return put("threshold", null); }
public RequestDefinitionFieldsChoose omitThreshold() { return omit("threshold"); }
public RequestDefinitionFieldsChoose wordingVersion(Number value) { return put("wording_version", value); }
public RequestDefinitionFieldsChoose wordingVersionNull() { return put("wording_version", null); }
public RequestDefinitionFieldsChoose omitWordingVersion() { return omit("wording_version"); }
}
public static final class RequestDefinitionFieldsChoosePropertiesBatch implements Values.Value {
private final Object value;
public RequestDefinitionFieldsChoosePropertiesBatch(String value) { this.value = Values.freeze(value); }
public RequestDefinitionFieldsChoosePropertiesBatch(Number value) { this.value = Values.freeze(value); }
public Object json() { return value; }
}
public static final class RequestDefinitionFieldsDecide extends Values.Builder<RequestDefinitionFieldsDecide> implements RequestDefinition {
public RequestDefinitionFieldsDecide() {
}
public static RequestDefinitionFieldsDecide fromJson(Object value) { RequestDefinitionFieldsDecide result = new RequestDefinitionFieldsDecide(); result.omit("batch"); result.omit("context_schema"); result.omit("decide"); result.omit("false"); result.omit("item_schema"); result.omit("model"); result.omit("name"); result.omit("on"); result.omit("profile"); result.omit("threshold"); result.omit("true"); result.omit("wording_version"); Values.object(value).forEach(result::extension); return result; }
protected RequestDefinitionFieldsDecide self() { return this; }
public RequestDefinitionFieldsDecide batch(RequestDefinitionFieldsDecidePropertiesBatch value) { return put("batch", value); }
public RequestDefinitionFieldsDecide batchNull() { return put("batch", null); }
public RequestDefinitionFieldsDecide omitBatch() { return omit("batch"); }
public RequestDefinitionFieldsDecide contextSchema(AuthoredInputDeclaration value) { return put("context_schema", value); }
public RequestDefinitionFieldsDecide contextSchemaNull() { return put("context_schema", null); }
public RequestDefinitionFieldsDecide omitContextSchema() { return omit("context_schema"); }
public RequestDefinitionFieldsDecide decide(AuthoredQuestionText value) { return put("decide", value); }
public RequestDefinitionFieldsDecide decideNull() { return put("decide", null); }
public RequestDefinitionFieldsDecide omitDecide() { return omit("decide"); }
public RequestDefinitionFieldsDecide falseValue(AuthoredCriterion value) { return put("false", value); }
public RequestDefinitionFieldsDecide falseValueNull() { return put("false", null); }
public RequestDefinitionFieldsDecide omitFalseValue() { return omit("false"); }
public RequestDefinitionFieldsDecide itemSchema(AuthoredInputDeclaration value) { return put("item_schema", value); }
public RequestDefinitionFieldsDecide itemSchemaNull() { return put("item_schema", null); }
public RequestDefinitionFieldsDecide omitItemSchema() { return omit("item_schema"); }
public RequestDefinitionFieldsDecide model(String value) { return put("model", value); }
public RequestDefinitionFieldsDecide modelNull() { return put("model", null); }
public RequestDefinitionFieldsDecide omitModel() { return omit("model"); }
public RequestDefinitionFieldsDecide name(String value) { return put("name", value); }
public RequestDefinitionFieldsDecide nameNull() { return put("name", null); }
public RequestDefinitionFieldsDecide omitName() { return omit("name"); }
public RequestDefinitionFieldsDecide on(AuthoredPointers value) { return put("on", value); }
public RequestDefinitionFieldsDecide onNull() { return put("on", null); }
public RequestDefinitionFieldsDecide omitOn() { return omit("on"); }
public RequestDefinitionFieldsDecide profile(String value) { return put("profile", value); }
public RequestDefinitionFieldsDecide profileNull() { return put("profile", null); }
public RequestDefinitionFieldsDecide omitProfile() { return omit("profile"); }
public RequestDefinitionFieldsDecide threshold(AuthoredThreshold value) { return put("threshold", value); }
public RequestDefinitionFieldsDecide thresholdNull() { return put("threshold", null); }
public RequestDefinitionFieldsDecide omitThreshold() { return omit("threshold"); }
public RequestDefinitionFieldsDecide trueValue(AuthoredCriterion value) { return put("true", value); }
public RequestDefinitionFieldsDecide trueValueNull() { return put("true", null); }
public RequestDefinitionFieldsDecide omitTrueValue() { return omit("true"); }
public RequestDefinitionFieldsDecide wordingVersion(Number value) { return put("wording_version", value); }
public RequestDefinitionFieldsDecide wordingVersionNull() { return put("wording_version", null); }
public RequestDefinitionFieldsDecide omitWordingVersion() { return omit("wording_version"); }
}
public static final class RequestDefinitionFieldsDecidePropertiesBatch implements Values.Value {
private final Object value;
public RequestDefinitionFieldsDecidePropertiesBatch(String value) { this.value = Values.freeze(value); }
public RequestDefinitionFieldsDecidePropertiesBatch(Number value) { this.value = Values.freeze(value); }
public Object json() { return value; }
}
public static final class RequestDefinitionFieldsFind extends Values.Builder<RequestDefinitionFieldsFind> implements RequestDefinition {
public RequestDefinitionFieldsFind() {
}
public static RequestDefinitionFieldsFind fromJson(Object value) { RequestDefinitionFieldsFind result = new RequestDefinitionFieldsFind(); result.omit("context_schema"); result.omit("find"); result.omit("item_schema"); result.omit("model"); result.omit("name"); result.omit("on"); result.omit("profile"); result.omit("wording_version"); Values.object(value).forEach(result::extension); return result; }
protected RequestDefinitionFieldsFind self() { return this; }
public RequestDefinitionFieldsFind contextSchema(AuthoredInputDeclaration value) { return put("context_schema", value); }
public RequestDefinitionFieldsFind contextSchemaNull() { return put("context_schema", null); }
public RequestDefinitionFieldsFind omitContextSchema() { return omit("context_schema"); }
public RequestDefinitionFieldsFind find(AuthoredQuestionText value) { return put("find", value); }
public RequestDefinitionFieldsFind findNull() { return put("find", null); }
public RequestDefinitionFieldsFind omitFind() { return omit("find"); }
public RequestDefinitionFieldsFind itemSchema(AuthoredInputDeclaration value) { return put("item_schema", value); }
public RequestDefinitionFieldsFind itemSchemaNull() { return put("item_schema", null); }
public RequestDefinitionFieldsFind omitItemSchema() { return omit("item_schema"); }
public RequestDefinitionFieldsFind model(String value) { return put("model", value); }
public RequestDefinitionFieldsFind modelNull() { return put("model", null); }
public RequestDefinitionFieldsFind omitModel() { return omit("model"); }
public RequestDefinitionFieldsFind name(String value) { return put("name", value); }
public RequestDefinitionFieldsFind nameNull() { return put("name", null); }
public RequestDefinitionFieldsFind omitName() { return omit("name"); }
public RequestDefinitionFieldsFind on(AuthoredPointers value) { return put("on", value); }
public RequestDefinitionFieldsFind onNull() { return put("on", null); }
public RequestDefinitionFieldsFind omitOn() { return omit("on"); }
public RequestDefinitionFieldsFind profile(String value) { return put("profile", value); }
public RequestDefinitionFieldsFind profileNull() { return put("profile", null); }
public RequestDefinitionFieldsFind omitProfile() { return omit("profile"); }
public RequestDefinitionFieldsFind wordingVersion(Number value) { return put("wording_version", value); }
public RequestDefinitionFieldsFind wordingVersionNull() { return put("wording_version", null); }
public RequestDefinitionFieldsFind omitWordingVersion() { return omit("wording_version"); }
}
public static final class RequestDefinitionFieldsQuestionsVersion extends Values.Builder<RequestDefinitionFieldsQuestionsVersion> implements RequestDefinition {
public RequestDefinitionFieldsQuestionsVersion() {
put("version", 1);
}
public static RequestDefinitionFieldsQuestionsVersion fromJson(Object value) { RequestDefinitionFieldsQuestionsVersion result = new RequestDefinitionFieldsQuestionsVersion(); result.omit("batch"); result.omit("profile"); result.omit("questions"); result.omit("threshold"); result.omit("version"); Values.object(value).forEach(result::extension); return result; }
protected RequestDefinitionFieldsQuestionsVersion self() { return this; }
public RequestDefinitionFieldsQuestionsVersion batch(Object value) { return put("batch", value); }
public RequestDefinitionFieldsQuestionsVersion batchNull() { return put("batch", null); }
public RequestDefinitionFieldsQuestionsVersion omitBatch() { return omit("batch"); }
public RequestDefinitionFieldsQuestionsVersion profile(String value) { return put("profile", value); }
public RequestDefinitionFieldsQuestionsVersion profileNull() { return put("profile", null); }
public RequestDefinitionFieldsQuestionsVersion omitProfile() { return omit("profile"); }
public RequestDefinitionFieldsQuestionsVersion questions(Map<String,? extends RequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties> value) { return put("questions", value); }
public RequestDefinitionFieldsQuestionsVersion questionsNull() { return put("questions", null); }
public RequestDefinitionFieldsQuestionsVersion omitQuestions() { return omit("questions"); }
public RequestDefinitionFieldsQuestionsVersion threshold(AuthoredThreshold value) { return put("threshold", value); }
public RequestDefinitionFieldsQuestionsVersion thresholdNull() { return put("threshold", null); }
public RequestDefinitionFieldsQuestionsVersion omitThreshold() { return omit("threshold"); }
}
public static final class RequestDefinitionFieldsRecognizeVersion extends Values.Builder<RequestDefinitionFieldsRecognizeVersion> implements RequestDefinition {
public RequestDefinitionFieldsRecognizeVersion() {
put("version", 1);
}
public static RequestDefinitionFieldsRecognizeVersion fromJson(Object value) { RequestDefinitionFieldsRecognizeVersion result = new RequestDefinitionFieldsRecognizeVersion(); result.omit("context_schema"); result.omit("item_schema"); result.omit("model"); result.omit("name"); result.omit("on"); result.omit("profile"); result.omit("recognize"); result.omit("relation_threshold"); result.omit("threshold"); result.omit("version"); result.omit("wording_version"); Values.object(value).forEach(result::extension); return result; }
protected RequestDefinitionFieldsRecognizeVersion self() { return this; }
public RequestDefinitionFieldsRecognizeVersion contextSchema(AuthoredInputDeclaration value) { return put("context_schema", value); }
public RequestDefinitionFieldsRecognizeVersion contextSchemaNull() { return put("context_schema", null); }
public RequestDefinitionFieldsRecognizeVersion omitContextSchema() { return omit("context_schema"); }
public RequestDefinitionFieldsRecognizeVersion itemSchema(AuthoredInputDeclaration value) { return put("item_schema", value); }
public RequestDefinitionFieldsRecognizeVersion itemSchemaNull() { return put("item_schema", null); }
public RequestDefinitionFieldsRecognizeVersion omitItemSchema() { return omit("item_schema"); }
public RequestDefinitionFieldsRecognizeVersion model(String value) { return put("model", value); }
public RequestDefinitionFieldsRecognizeVersion modelNull() { return put("model", null); }
public RequestDefinitionFieldsRecognizeVersion omitModel() { return omit("model"); }
public RequestDefinitionFieldsRecognizeVersion name(String value) { return put("name", value); }
public RequestDefinitionFieldsRecognizeVersion nameNull() { return put("name", null); }
public RequestDefinitionFieldsRecognizeVersion omitName() { return omit("name"); }
public RequestDefinitionFieldsRecognizeVersion on(AuthoredPointers value) { return put("on", value); }
public RequestDefinitionFieldsRecognizeVersion onNull() { return put("on", null); }
public RequestDefinitionFieldsRecognizeVersion omitOn() { return omit("on"); }
public RequestDefinitionFieldsRecognizeVersion profile(String value) { return put("profile", value); }
public RequestDefinitionFieldsRecognizeVersion profileNull() { return put("profile", null); }
public RequestDefinitionFieldsRecognizeVersion omitProfile() { return omit("profile"); }
public RequestDefinitionFieldsRecognizeVersion recognize(RequestDefinitionFieldsRecognizeVersionPropertiesRecognize value) { return put("recognize", value); }
public RequestDefinitionFieldsRecognizeVersion recognizeNull() { return put("recognize", null); }
public RequestDefinitionFieldsRecognizeVersion omitRecognize() { return omit("recognize"); }
public RequestDefinitionFieldsRecognizeVersion relationThreshold(AuthoredCut value) { return put("relation_threshold", value); }
public RequestDefinitionFieldsRecognizeVersion relationThresholdNull() { return put("relation_threshold", null); }
public RequestDefinitionFieldsRecognizeVersion omitRelationThreshold() { return omit("relation_threshold"); }
public RequestDefinitionFieldsRecognizeVersion threshold(AuthoredCut value) { return put("threshold", value); }
public RequestDefinitionFieldsRecognizeVersion thresholdNull() { return put("threshold", null); }
public RequestDefinitionFieldsRecognizeVersion omitThreshold() { return omit("threshold"); }
public RequestDefinitionFieldsRecognizeVersion wordingVersion(Number value) { return put("wording_version", value); }
public RequestDefinitionFieldsRecognizeVersion wordingVersionNull() { return put("wording_version", null); }
public RequestDefinitionFieldsRecognizeVersion omitWordingVersion() { return omit("wording_version"); }
}
public static final class RequestDefinitionFieldsRecognizeVersionPropertiesRecognize extends Values.Builder<RequestDefinitionFieldsRecognizeVersionPropertiesRecognize> {
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize() {
}
public static RequestDefinitionFieldsRecognizeVersionPropertiesRecognize fromJson(Object value) { RequestDefinitionFieldsRecognizeVersionPropertiesRecognize result = new RequestDefinitionFieldsRecognizeVersionPropertiesRecognize(); result.omit("entity_definition"); result.omit("instructions"); result.omit("kinds"); result.omit("mode"); result.omit("relations"); result.omit("snippet_pieces"); result.omit("stage_context"); Values.object(value).forEach(result::extension); return result; }
protected RequestDefinitionFieldsRecognizeVersionPropertiesRecognize self() { return this; }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize entityDefinition(AuthoredQuestionText value) { return put("entity_definition", value); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize entityDefinitionNull() { return put("entity_definition", null); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize omitEntityDefinition() { return omit("entity_definition"); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize instructions(AuthoredQuestionText value) { return put("instructions", value); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize instructionsNull() { return put("instructions", null); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize omitInstructions() { return omit("instructions"); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize kinds(Map<String,? extends AuthoredDescription> value) { return put("kinds", value); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize kindsNull() { return put("kinds", null); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize omitKinds() { return omit("kinds"); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize mode(RecognitionMode value) { return put("mode", value); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize modeNull() { return put("mode", null); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize omitMode() { return omit("mode"); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize relations(List<? extends AuthoredRelation> value) { return put("relations", value); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize relationsNull() { return put("relations", null); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize omitRelations() { return omit("relations"); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize snippetPieces(Number value) { return put("snippet_pieces", value); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize snippetPiecesNull() { return put("snippet_pieces", null); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize omitSnippetPieces() { return omit("snippet_pieces"); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize stageContext(RecognitionStageContext value) { return put("stage_context", value); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize stageContextNull() { return put("stage_context", null); }
public RequestDefinitionFieldsRecognizeVersionPropertiesRecognize omitStageContext() { return omit("stage_context"); }
}
public static final class RequestDefinitionFieldsRelateVersion extends Values.Builder<RequestDefinitionFieldsRelateVersion> implements RequestDefinition {
public RequestDefinitionFieldsRelateVersion() {
put("version", 1);
}
public static RequestDefinitionFieldsRelateVersion fromJson(Object value) { RequestDefinitionFieldsRelateVersion result = new RequestDefinitionFieldsRelateVersion(); result.omit("context_schema"); result.omit("item_schema"); result.omit("model"); result.omit("name"); result.omit("profile"); result.omit("relate"); result.omit("threshold"); result.omit("version"); result.omit("wording_version"); Values.object(value).forEach(result::extension); return result; }
protected RequestDefinitionFieldsRelateVersion self() { return this; }
public RequestDefinitionFieldsRelateVersion contextSchema(AuthoredInputDeclaration value) { return put("context_schema", value); }
public RequestDefinitionFieldsRelateVersion contextSchemaNull() { return put("context_schema", null); }
public RequestDefinitionFieldsRelateVersion omitContextSchema() { return omit("context_schema"); }
public RequestDefinitionFieldsRelateVersion itemSchema(AuthoredInputDeclaration value) { return put("item_schema", value); }
public RequestDefinitionFieldsRelateVersion itemSchemaNull() { return put("item_schema", null); }
public RequestDefinitionFieldsRelateVersion omitItemSchema() { return omit("item_schema"); }
public RequestDefinitionFieldsRelateVersion model(String value) { return put("model", value); }
public RequestDefinitionFieldsRelateVersion modelNull() { return put("model", null); }
public RequestDefinitionFieldsRelateVersion omitModel() { return omit("model"); }
public RequestDefinitionFieldsRelateVersion name(String value) { return put("name", value); }
public RequestDefinitionFieldsRelateVersion nameNull() { return put("name", null); }
public RequestDefinitionFieldsRelateVersion omitName() { return omit("name"); }
public RequestDefinitionFieldsRelateVersion profile(String value) { return put("profile", value); }
public RequestDefinitionFieldsRelateVersion profileNull() { return put("profile", null); }
public RequestDefinitionFieldsRelateVersion omitProfile() { return omit("profile"); }
public RequestDefinitionFieldsRelateVersion relate(RequestDefinitionFieldsRelateVersionPropertiesRelate value) { return put("relate", value); }
public RequestDefinitionFieldsRelateVersion relateNull() { return put("relate", null); }
public RequestDefinitionFieldsRelateVersion omitRelate() { return omit("relate"); }
public RequestDefinitionFieldsRelateVersion threshold(AuthoredCut value) { return put("threshold", value); }
public RequestDefinitionFieldsRelateVersion thresholdNull() { return put("threshold", null); }
public RequestDefinitionFieldsRelateVersion omitThreshold() { return omit("threshold"); }
public RequestDefinitionFieldsRelateVersion wordingVersion(Number value) { return put("wording_version", value); }
public RequestDefinitionFieldsRelateVersion wordingVersionNull() { return put("wording_version", null); }
public RequestDefinitionFieldsRelateVersion omitWordingVersion() { return omit("wording_version"); }
}
public static final class RequestDefinitionFieldsRelateVersionPropertiesRelate extends Values.Builder<RequestDefinitionFieldsRelateVersionPropertiesRelate> {
public RequestDefinitionFieldsRelateVersionPropertiesRelate() {
}
public static RequestDefinitionFieldsRelateVersionPropertiesRelate fromJson(Object value) { RequestDefinitionFieldsRelateVersionPropertiesRelate result = new RequestDefinitionFieldsRelateVersionPropertiesRelate(); result.omit("fields"); result.omit("relations"); Values.object(value).forEach(result::extension); return result; }
protected RequestDefinitionFieldsRelateVersionPropertiesRelate self() { return this; }
public RequestDefinitionFieldsRelateVersionPropertiesRelate fields(RequestDefinitionFieldsRelateVersionPropertiesRelatePropertiesFields value) { return put("fields", value); }
public RequestDefinitionFieldsRelateVersionPropertiesRelate fieldsNull() { return put("fields", null); }
public RequestDefinitionFieldsRelateVersionPropertiesRelate omitFields() { return omit("fields"); }
public RequestDefinitionFieldsRelateVersionPropertiesRelate relations(List<? extends AuthoredRelation> value) { return put("relations", value); }
public RequestDefinitionFieldsRelateVersionPropertiesRelate relationsNull() { return put("relations", null); }
public RequestDefinitionFieldsRelateVersionPropertiesRelate omitRelations() { return omit("relations"); }
}
public static final class RequestDefinitionFieldsRelateVersionPropertiesRelatePropertiesFields extends Values.Builder<RequestDefinitionFieldsRelateVersionPropertiesRelatePropertiesFields> {
public RequestDefinitionFieldsRelateVersionPropertiesRelatePropertiesFields() {
}
public static RequestDefinitionFieldsRelateVersionPropertiesRelatePropertiesFields fromJson(Object value) { RequestDefinitionFieldsRelateVersionPropertiesRelatePropertiesFields result = new RequestDefinitionFieldsRelateVersionPropertiesRelatePropertiesFields(); result.omit("kind"); result.omit("name"); Values.object(value).forEach(result::extension); return result; }
protected RequestDefinitionFieldsRelateVersionPropertiesRelatePropertiesFields self() { return this; }
public RequestDefinitionFieldsRelateVersionPropertiesRelatePropertiesFields kind(String value) { return put("kind", value); }
public RequestDefinitionFieldsRelateVersionPropertiesRelatePropertiesFields kindNull() { return put("kind", null); }
public RequestDefinitionFieldsRelateVersionPropertiesRelatePropertiesFields omitKind() { return omit("kind"); }
public RequestDefinitionFieldsRelateVersionPropertiesRelatePropertiesFields name(String value) { return put("name", value); }
public RequestDefinitionFieldsRelateVersionPropertiesRelatePropertiesFields nameNull() { return put("name", null); }
public RequestDefinitionFieldsRelateVersionPropertiesRelatePropertiesFields omitName() { return omit("name"); }
}
public static final class RequestDefinitionFieldsScore extends Values.Builder<RequestDefinitionFieldsScore> implements RequestDefinition {
public RequestDefinitionFieldsScore() {
}
public static RequestDefinitionFieldsScore fromJson(Object value) { RequestDefinitionFieldsScore result = new RequestDefinitionFieldsScore(); result.omit("batch"); result.omit("context_schema"); result.omit("item_schema"); result.omit("levels"); result.omit("model"); result.omit("name"); result.omit("on"); result.omit("profile"); result.omit("score"); result.omit("wording_version"); Values.object(value).forEach(result::extension); return result; }
protected RequestDefinitionFieldsScore self() { return this; }
public RequestDefinitionFieldsScore batch(RequestDefinitionFieldsScorePropertiesBatch value) { return put("batch", value); }
public RequestDefinitionFieldsScore batchNull() { return put("batch", null); }
public RequestDefinitionFieldsScore omitBatch() { return omit("batch"); }
public RequestDefinitionFieldsScore contextSchema(AuthoredInputDeclaration value) { return put("context_schema", value); }
public RequestDefinitionFieldsScore contextSchemaNull() { return put("context_schema", null); }
public RequestDefinitionFieldsScore omitContextSchema() { return omit("context_schema"); }
public RequestDefinitionFieldsScore itemSchema(AuthoredInputDeclaration value) { return put("item_schema", value); }
public RequestDefinitionFieldsScore itemSchemaNull() { return put("item_schema", null); }
public RequestDefinitionFieldsScore omitItemSchema() { return omit("item_schema"); }
public RequestDefinitionFieldsScore levels(AuthoredLevels value) { return put("levels", value); }
public RequestDefinitionFieldsScore levelsNull() { return put("levels", null); }
public RequestDefinitionFieldsScore omitLevels() { return omit("levels"); }
public RequestDefinitionFieldsScore model(String value) { return put("model", value); }
public RequestDefinitionFieldsScore modelNull() { return put("model", null); }
public RequestDefinitionFieldsScore omitModel() { return omit("model"); }
public RequestDefinitionFieldsScore name(String value) { return put("name", value); }
public RequestDefinitionFieldsScore nameNull() { return put("name", null); }
public RequestDefinitionFieldsScore omitName() { return omit("name"); }
public RequestDefinitionFieldsScore on(AuthoredPointers value) { return put("on", value); }
public RequestDefinitionFieldsScore onNull() { return put("on", null); }
public RequestDefinitionFieldsScore omitOn() { return omit("on"); }
public RequestDefinitionFieldsScore profile(String value) { return put("profile", value); }
public RequestDefinitionFieldsScore profileNull() { return put("profile", null); }
public RequestDefinitionFieldsScore omitProfile() { return omit("profile"); }
public RequestDefinitionFieldsScore score(AuthoredQuestionText value) { return put("score", value); }
public RequestDefinitionFieldsScore scoreNull() { return put("score", null); }
public RequestDefinitionFieldsScore omitScore() { return omit("score"); }
public RequestDefinitionFieldsScore wordingVersion(Number value) { return put("wording_version", value); }
public RequestDefinitionFieldsScore wordingVersionNull() { return put("wording_version", null); }
public RequestDefinitionFieldsScore omitWordingVersion() { return omit("wording_version"); }
}
public static final class RequestDefinitionFieldsScorePropertiesBatch implements Values.Value {
private final Object value;
public RequestDefinitionFieldsScorePropertiesBatch(String value) { this.value = Values.freeze(value); }
public RequestDefinitionFieldsScorePropertiesBatch(Number value) { this.value = Values.freeze(value); }
public Object json() { return value; }
}
public static final class RequestDefinitionFieldsTag extends Values.Builder<RequestDefinitionFieldsTag> implements RequestDefinition {
public RequestDefinitionFieldsTag() {
}
public static RequestDefinitionFieldsTag fromJson(Object value) { RequestDefinitionFieldsTag result = new RequestDefinitionFieldsTag(); result.omit("batch"); result.omit("context_schema"); result.omit("item_schema"); result.omit("labels"); result.omit("model"); result.omit("name"); result.omit("on"); result.omit("profile"); result.omit("tag"); result.omit("threshold"); result.omit("wording_version"); Values.object(value).forEach(result::extension); return result; }
protected RequestDefinitionFieldsTag self() { return this; }
public RequestDefinitionFieldsTag batch(RequestDefinitionFieldsTagPropertiesBatch value) { return put("batch", value); }
public RequestDefinitionFieldsTag batchNull() { return put("batch", null); }
public RequestDefinitionFieldsTag omitBatch() { return omit("batch"); }
public RequestDefinitionFieldsTag contextSchema(AuthoredInputDeclaration value) { return put("context_schema", value); }
public RequestDefinitionFieldsTag contextSchemaNull() { return put("context_schema", null); }
public RequestDefinitionFieldsTag omitContextSchema() { return omit("context_schema"); }
public RequestDefinitionFieldsTag itemSchema(AuthoredInputDeclaration value) { return put("item_schema", value); }
public RequestDefinitionFieldsTag itemSchemaNull() { return put("item_schema", null); }
public RequestDefinitionFieldsTag omitItemSchema() { return omit("item_schema"); }
public RequestDefinitionFieldsTag labels(AuthoredLabels value) { return put("labels", value); }
public RequestDefinitionFieldsTag labelsNull() { return put("labels", null); }
public RequestDefinitionFieldsTag omitLabels() { return omit("labels"); }
public RequestDefinitionFieldsTag model(String value) { return put("model", value); }
public RequestDefinitionFieldsTag modelNull() { return put("model", null); }
public RequestDefinitionFieldsTag omitModel() { return omit("model"); }
public RequestDefinitionFieldsTag name(String value) { return put("name", value); }
public RequestDefinitionFieldsTag nameNull() { return put("name", null); }
public RequestDefinitionFieldsTag omitName() { return omit("name"); }
public RequestDefinitionFieldsTag on(AuthoredPointers value) { return put("on", value); }
public RequestDefinitionFieldsTag onNull() { return put("on", null); }
public RequestDefinitionFieldsTag omitOn() { return omit("on"); }
public RequestDefinitionFieldsTag profile(String value) { return put("profile", value); }
public RequestDefinitionFieldsTag profileNull() { return put("profile", null); }
public RequestDefinitionFieldsTag omitProfile() { return omit("profile"); }
public RequestDefinitionFieldsTag tag(AuthoredQuestionText value) { return put("tag", value); }
public RequestDefinitionFieldsTag tagNull() { return put("tag", null); }
public RequestDefinitionFieldsTag omitTag() { return omit("tag"); }
public RequestDefinitionFieldsTag threshold(AuthoredCut value) { return put("threshold", value); }
public RequestDefinitionFieldsTag thresholdNull() { return put("threshold", null); }
public RequestDefinitionFieldsTag omitThreshold() { return omit("threshold"); }
public RequestDefinitionFieldsTag wordingVersion(Number value) { return put("wording_version", value); }
public RequestDefinitionFieldsTag wordingVersionNull() { return put("wording_version", null); }
public RequestDefinitionFieldsTag omitWordingVersion() { return omit("wording_version"); }
}
public static final class RequestDefinitionFieldsTagPropertiesBatch implements Values.Value {
private final Object value;
public RequestDefinitionFieldsTagPropertiesBatch(String value) { this.value = Values.freeze(value); }
public RequestDefinitionFieldsTagPropertiesBatch(Number value) { this.value = Values.freeze(value); }
public Object json() { return value; }
}
public enum RequestFraming implements Values.Value { DOCUMENT("document"),LINES("lines"),JSONL("jsonl"),CSV("csv"),TSV("tsv");
private final String value; RequestFraming(String value) { this.value = value; }
public Object json() { return value; }
}
public sealed interface RequestImage extends Values.Value permits RequestImageFile,RequestImageBytes {
static RequestImage fromJson(Object value) { Map<String,Object> object = Values.object(value);
if ("file".equals(object.get("kind"))) return RequestImageFile.fromJson(value);
if ("bytes".equals(object.get("kind"))) return RequestImageBytes.fromJson(value);
return RequestImageFile.fromJson(value);
}
}
public static final class RequestImageBytes extends Values.Builder<RequestImageBytes> implements RequestImage {
public RequestImageBytes() {
put("kind", "bytes");
}
public static RequestImageBytes fromJson(Object value) { RequestImageBytes result = new RequestImageBytes(); result.omit("bytes"); result.omit("kind"); result.omit("media"); Values.object(value).forEach(result::extension); return result; }
protected RequestImageBytes self() { return this; }
public RequestImageBytes bytes(String value) { return put("bytes", value); }
public RequestImageBytes bytesNull() { return put("bytes", null); }
public RequestImageBytes omitBytes() { return omit("bytes"); }
public RequestImageBytes media(ImageMedia value) { return put("media", value); }
public RequestImageBytes mediaNull() { return put("media", null); }
public RequestImageBytes omitMedia() { return omit("media"); }
}
public static final class RequestImageFile extends Values.Builder<RequestImageFile> implements RequestImage {
public RequestImageFile() {
put("kind", "file");
}
public static RequestImageFile fromJson(Object value) { RequestImageFile result = new RequestImageFile(); result.omit("kind"); result.omit("media"); result.omit("path"); Values.object(value).forEach(result::extension); return result; }
protected RequestImageFile self() { return this; }
public RequestImageFile media(ImageMedia value) { return put("media", value); }
public RequestImageFile mediaNull() { return put("media", null); }
public RequestImageFile omitMedia() { return omit("media"); }
public RequestImageFile path(String value) { return put("path", value); }
public RequestImageFile pathNull() { return put("path", null); }
public RequestImageFile omitPath() { return omit("path"); }
}
public sealed interface RequestInput extends Values.Value permits RequestInputText,RequestInputJson,RequestInputRecords,RequestInputUnits,RequestInputEntities,RequestInputSource,RequestInputFeed {
static RequestInput fromJson(Object value) { Map<String,Object> object = Values.object(value);
if ("text".equals(object.get("kind"))) return RequestInputText.fromJson(value);
if ("json".equals(object.get("kind"))) return RequestInputJson.fromJson(value);
if ("records".equals(object.get("kind"))) return RequestInputRecords.fromJson(value);
if ("units".equals(object.get("kind"))) return RequestInputUnits.fromJson(value);
if ("entities".equals(object.get("kind"))) return RequestInputEntities.fromJson(value);
if ("source".equals(object.get("kind"))) return RequestInputSource.fromJson(value);
if ("feed".equals(object.get("kind"))) return RequestInputFeed.fromJson(value);
return RequestInputText.fromJson(value);
}
}
public static final class RequestInputEntities extends Values.Builder<RequestInputEntities> implements RequestInput {
public RequestInputEntities() {
put("kind", "entities");
}
public static RequestInputEntities fromJson(Object value) { RequestInputEntities result = new RequestInputEntities(); result.omit("items"); result.omit("kind"); Values.object(value).forEach(result::extension); return result; }
protected RequestInputEntities self() { return this; }
public RequestInputEntities items(List<? extends RequestItem> value) { return put("items", value); }
public RequestInputEntities itemsNull() { return put("items", null); }
public RequestInputEntities omitItems() { return omit("items"); }
}
public static final class RequestInputFeed extends Values.Builder<RequestInputFeed> implements RequestInput {
public RequestInputFeed() {
put("kind", "feed");
}
public static RequestInputFeed fromJson(Object value) { RequestInputFeed result = new RequestInputFeed(); result.omit("framing"); result.omit("images"); result.omit("kind"); result.omit("name"); result.omit("reading"); Values.object(value).forEach(result::extension); return result; }
protected RequestInputFeed self() { return this; }
public RequestInputFeed framing(RequestFraming value) { return put("framing", value); }
public RequestInputFeed framingNull() { return put("framing", null); }
public RequestInputFeed omitFraming() { return omit("framing"); }
public RequestInputFeed images(List<? extends RequestImage> value) { return put("images", value); }
public RequestInputFeed imagesNull() { return put("images", null); }
public RequestInputFeed omitImages() { return omit("images"); }
public RequestInputFeed name(String value) { return put("name", value); }
public RequestInputFeed nameNull() { return put("name", null); }
public RequestInputFeed omitName() { return omit("name"); }
public RequestInputFeed reading(RequestReader value) { return put("reading", value); }
public RequestInputFeed readingNull() { return put("reading", null); }
public RequestInputFeed omitReading() { return omit("reading"); }
}
public static final class RequestInputJson extends Values.Builder<RequestInputJson> implements RequestInput {
public RequestInputJson() {
put("kind", "json");
}
public static RequestInputJson fromJson(Object value) { RequestInputJson result = new RequestInputJson(); result.omit("images"); result.omit("kind"); result.omit("value"); Values.object(value).forEach(result::extension); return result; }
protected RequestInputJson self() { return this; }
public RequestInputJson images(List<? extends RequestImage> value) { return put("images", value); }
public RequestInputJson imagesNull() { return put("images", null); }
public RequestInputJson omitImages() { return omit("images"); }
public RequestInputJson value(Object value) { return put("value", value); }
public RequestInputJson valueNull() { return put("value", null); }
public RequestInputJson omitValue() { return omit("value"); }
}
public static final class RequestInputRecords extends Values.Builder<RequestInputRecords> implements RequestInput {
public RequestInputRecords() {
put("kind", "records");
}
public static RequestInputRecords fromJson(Object value) { RequestInputRecords result = new RequestInputRecords(); result.omit("items"); result.omit("kind"); Values.object(value).forEach(result::extension); return result; }
protected RequestInputRecords self() { return this; }
public RequestInputRecords items(List<? extends RequestItem> value) { return put("items", value); }
public RequestInputRecords itemsNull() { return put("items", null); }
public RequestInputRecords omitItems() { return omit("items"); }
}
public static final class RequestInputSource extends Values.Builder<RequestInputSource> implements RequestInput {
public RequestInputSource() {
put("kind", "source");
}
public static RequestInputSource fromJson(Object value) { RequestInputSource result = new RequestInputSource(); result.omit("kind"); result.omit("source"); Values.object(value).forEach(result::extension); return result; }
protected RequestInputSource self() { return this; }
public RequestInputSource source(RequestSource value) { return put("source", value); }
public RequestInputSource sourceNull() { return put("source", null); }
public RequestInputSource omitSource() { return omit("source"); }
}
public static final class RequestInputText extends Values.Builder<RequestInputText> implements RequestInput {
public RequestInputText() {
put("kind", "text");
}
public static RequestInputText fromJson(Object value) { RequestInputText result = new RequestInputText(); result.omit("images"); result.omit("kind"); result.omit("text"); Values.object(value).forEach(result::extension); return result; }
protected RequestInputText self() { return this; }
public RequestInputText images(List<? extends RequestImage> value) { return put("images", value); }
public RequestInputText imagesNull() { return put("images", null); }
public RequestInputText omitImages() { return omit("images"); }
public RequestInputText text(String value) { return put("text", value); }
public RequestInputText textNull() { return put("text", null); }
public RequestInputText omitText() { return omit("text"); }
}
public static final class RequestInputUnits extends Values.Builder<RequestInputUnits> implements RequestInput {
public RequestInputUnits() {
put("kind", "units");
}
public static RequestInputUnits fromJson(Object value) { RequestInputUnits result = new RequestInputUnits(); result.omit("items"); result.omit("kind"); Values.object(value).forEach(result::extension); return result; }
protected RequestInputUnits self() { return this; }
public RequestInputUnits items(List<? extends RequestItem> value) { return put("items", value); }
public RequestInputUnits itemsNull() { return put("items", null); }
public RequestInputUnits omitItems() { return omit("items"); }
}
public static final class RequestItem extends Values.Builder<RequestItem> {
public RequestItem() {
}
public static RequestItem fromJson(Object value) { RequestItem result = new RequestItem(); result.omit("context"); result.omit("examples"); result.omit("images"); result.omit("options"); result.omit("original"); result.omit("seed_spans"); Values.object(value).forEach(result::extension); return result; }
protected RequestItem self() { return this; }
public RequestItem context(ContextSchema value) { return put("context", value); }
public RequestItem contextNull() { return put("context", null); }
public RequestItem omitContext() { return omit("context"); }
public RequestItem examples(List<? extends RecognitionExample> value) { return put("examples", value); }
public RequestItem examplesNull() { return put("examples", null); }
public RequestItem omitExamples() { return omit("examples"); }
public RequestItem images(List<? extends RequestImage> value) { return put("images", value); }
public RequestItem imagesNull() { return put("images", null); }
public RequestItem omitImages() { return omit("images"); }
public RequestItem options(List<? extends OptionSchema> value) { return put("options", value); }
public RequestItem optionsNull() { return put("options", null); }
public RequestItem omitOptions() { return omit("options"); }
public RequestItem original(RequestOriginal value) { return put("original", value); }
public RequestItem originalNull() { return put("original", null); }
public RequestItem omitOriginal() { return omit("original"); }
public RequestItem seedSpans(List<? extends RecognitionSeedSpan> value) { return put("seed_spans", value); }
public RequestItem seedSpansNull() { return put("seed_spans", null); }
public RequestItem omitSeedSpans() { return omit("seed_spans"); }
}
public static final class RequestOptions extends Values.Builder<RequestOptions> {
public RequestOptions() {
}
public static RequestOptions fromJson(Object value) { RequestOptions result = new RequestOptions(); result.omit("attempts"); result.omit("batch"); result.omit("context"); result.omit("context_field"); result.omit("deadline_ms"); result.omit("details"); result.omit("examples"); result.omit("examples_field"); result.omit("field"); result.omit("files_only"); result.omit("max_requests_total"); result.omit("mode"); result.omit("model"); result.omit("none"); result.omit("options_field"); result.omit("relation_threshold"); result.omit("seed_spans"); result.omit("seed_spans_field"); result.omit("snippet_pieces"); result.omit("stage_context"); result.omit("threshold"); result.omit("top"); Values.object(value).forEach(result::extension); return result; }
protected RequestOptions self() { return this; }
public RequestOptions attempts(Boolean value) { return put("attempts", value); }
public RequestOptions attemptsNull() { return put("attempts", null); }
public RequestOptions omitAttempts() { return omit("attempts"); }
public RequestOptions batch(RequestBatch value) { return put("batch", value); }
public RequestOptions batchNull() { return put("batch", null); }
public RequestOptions omitBatch() { return omit("batch"); }
public RequestOptions context(String value) { return put("context", value); }
public RequestOptions contextNull() { return put("context", null); }
public RequestOptions omitContext() { return omit("context"); }
public RequestOptions contextField(String value) { return put("context_field", value); }
public RequestOptions contextFieldNull() { return put("context_field", null); }
public RequestOptions omitContextField() { return omit("context_field"); }
public RequestOptions deadlineMs(Number value) { return put("deadline_ms", value); }
public RequestOptions deadlineMsNull() { return put("deadline_ms", null); }
public RequestOptions omitDeadlineMs() { return omit("deadline_ms"); }
public RequestOptions details(Boolean value) { return put("details", value); }
public RequestOptions detailsNull() { return put("details", null); }
public RequestOptions omitDetails() { return omit("details"); }
public RequestOptions examples(List<? extends RecognitionExample> value) { return put("examples", value); }
public RequestOptions examplesNull() { return put("examples", null); }
public RequestOptions omitExamples() { return omit("examples"); }
public RequestOptions examplesField(String value) { return put("examples_field", value); }
public RequestOptions examplesFieldNull() { return put("examples_field", null); }
public RequestOptions omitExamplesField() { return omit("examples_field"); }
public RequestOptions field(List<? extends String> value) { return put("field", value); }
public RequestOptions fieldNull() { return put("field", null); }
public RequestOptions omitField() { return omit("field"); }
public RequestOptions filesOnly(Boolean value) { return put("files_only", value); }
public RequestOptions filesOnlyNull() { return put("files_only", null); }
public RequestOptions omitFilesOnly() { return omit("files_only"); }
public RequestOptions maxRequestsTotal(Number value) { return put("max_requests_total", value); }
public RequestOptions maxRequestsTotalNull() { return put("max_requests_total", null); }
public RequestOptions omitMaxRequestsTotal() { return omit("max_requests_total"); }
public RequestOptions mode(RecognitionMode value) { return put("mode", value); }
public RequestOptions modeNull() { return put("mode", null); }
public RequestOptions omitMode() { return omit("mode"); }
public RequestOptions model(String value) { return put("model", value); }
public RequestOptions modelNull() { return put("model", null); }
public RequestOptions omitModel() { return omit("model"); }
public RequestOptions none(Boolean value) { return put("none", value); }
public RequestOptions noneNull() { return put("none", null); }
public RequestOptions omitNone() { return omit("none"); }
public RequestOptions optionsField(String value) { return put("options_field", value); }
public RequestOptions optionsFieldNull() { return put("options_field", null); }
public RequestOptions omitOptionsField() { return omit("options_field"); }
public RequestOptions relationThreshold(RequestThreshold value) { return put("relation_threshold", value); }
public RequestOptions relationThresholdNull() { return put("relation_threshold", null); }
public RequestOptions omitRelationThreshold() { return omit("relation_threshold"); }
public RequestOptions seedSpans(List<? extends RecognitionSeedSpan> value) { return put("seed_spans", value); }
public RequestOptions seedSpansNull() { return put("seed_spans", null); }
public RequestOptions omitSeedSpans() { return omit("seed_spans"); }
public RequestOptions seedSpansField(String value) { return put("seed_spans_field", value); }
public RequestOptions seedSpansFieldNull() { return put("seed_spans_field", null); }
public RequestOptions omitSeedSpansField() { return omit("seed_spans_field"); }
public RequestOptions snippetPieces(Number value) { return put("snippet_pieces", value); }
public RequestOptions snippetPiecesNull() { return put("snippet_pieces", null); }
public RequestOptions omitSnippetPieces() { return omit("snippet_pieces"); }
public RequestOptions stageContext(RecognitionStageContext value) { return put("stage_context", value); }
public RequestOptions stageContextNull() { return put("stage_context", null); }
public RequestOptions omitStageContext() { return omit("stage_context"); }
public RequestOptions threshold(RequestThreshold value) { return put("threshold", value); }
public RequestOptions thresholdNull() { return put("threshold", null); }
public RequestOptions omitThreshold() { return omit("threshold"); }
public RequestOptions top(Number value) { return put("top", value); }
public RequestOptions topNull() { return put("top", null); }
public RequestOptions omitTop() { return omit("top"); }
}
public sealed interface RequestOriginal extends Values.Value permits RequestOriginalText,RequestOriginalJson {
static RequestOriginal fromJson(Object value) { Map<String,Object> object = Values.object(value);
if ("text".equals(object.get("kind"))) return RequestOriginalText.fromJson(value);
if ("json".equals(object.get("kind"))) return RequestOriginalJson.fromJson(value);
return RequestOriginalText.fromJson(value);
}
}
public static final class RequestOriginalJson extends Values.Builder<RequestOriginalJson> implements RequestOriginal {
public RequestOriginalJson() {
put("kind", "json");
}
public static RequestOriginalJson fromJson(Object value) { RequestOriginalJson result = new RequestOriginalJson(); result.omit("kind"); result.omit("value"); Values.object(value).forEach(result::extension); return result; }
protected RequestOriginalJson self() { return this; }
public RequestOriginalJson value(Object value) { return put("value", value); }
public RequestOriginalJson valueNull() { return put("value", null); }
public RequestOriginalJson omitValue() { return omit("value"); }
}
public static final class RequestOriginalText extends Values.Builder<RequestOriginalText> implements RequestOriginal {
public RequestOriginalText() {
put("kind", "text");
}
public static RequestOriginalText fromJson(Object value) { RequestOriginalText result = new RequestOriginalText(); result.omit("kind"); result.omit("text"); Values.object(value).forEach(result::extension); return result; }
protected RequestOriginalText self() { return this; }
public RequestOriginalText text(String value) { return put("text", value); }
public RequestOriginalText textNull() { return put("text", null); }
public RequestOriginalText omitText() { return omit("text"); }
}
public sealed interface RequestQuestion extends Values.Value permits RequestQuestionText,RequestQuestionDefinition,RequestQuestionFile,RequestQuestionName,RequestQuestionReference {
static RequestQuestion fromJson(Object value) { Map<String,Object> object = Values.object(value);
if ("text".equals(object.get("kind"))) return RequestQuestionText.fromJson(value);
if ("definition".equals(object.get("kind"))) return RequestQuestionDefinition.fromJson(value);
if ("file".equals(object.get("kind"))) return RequestQuestionFile.fromJson(value);
if ("name".equals(object.get("kind"))) return RequestQuestionName.fromJson(value);
if ("reference".equals(object.get("kind"))) return RequestQuestionReference.fromJson(value);
return RequestQuestionText.fromJson(value);
}
}
public static final class RequestQuestionDefinition extends Values.Builder<RequestQuestionDefinition> implements RequestQuestion {
public RequestQuestionDefinition() {
put("kind", "definition");
}
public static RequestQuestionDefinition fromJson(Object value) { RequestQuestionDefinition result = new RequestQuestionDefinition(); result.omit("kind"); result.omit("value"); Values.object(value).forEach(result::extension); return result; }
protected RequestQuestionDefinition self() { return this; }
public RequestQuestionDefinition value(RequestDefinition value) { return put("value", value); }
public RequestQuestionDefinition valueNull() { return put("value", null); }
public RequestQuestionDefinition omitValue() { return omit("value"); }
}
public static final class RequestQuestionFile extends Values.Builder<RequestQuestionFile> implements RequestQuestion {
public RequestQuestionFile() {
put("kind", "file");
}
public static RequestQuestionFile fromJson(Object value) { RequestQuestionFile result = new RequestQuestionFile(); result.omit("kind"); result.omit("path"); Values.object(value).forEach(result::extension); return result; }
protected RequestQuestionFile self() { return this; }
public RequestQuestionFile path(String value) { return put("path", value); }
public RequestQuestionFile pathNull() { return put("path", null); }
public RequestQuestionFile omitPath() { return omit("path"); }
}
public static final class RequestQuestionName extends Values.Builder<RequestQuestionName> implements RequestQuestion {
public RequestQuestionName() {
put("kind", "name");
}
public static RequestQuestionName fromJson(Object value) { RequestQuestionName result = new RequestQuestionName(); result.omit("kind"); result.omit("name"); Values.object(value).forEach(result::extension); return result; }
protected RequestQuestionName self() { return this; }
public RequestQuestionName name(String value) { return put("name", value); }
public RequestQuestionName nameNull() { return put("name", null); }
public RequestQuestionName omitName() { return omit("name"); }
}
public static final class RequestQuestionReference extends Values.Builder<RequestQuestionReference> implements RequestQuestion {
public RequestQuestionReference() {
put("kind", "reference");
}
public static RequestQuestionReference fromJson(Object value) { RequestQuestionReference result = new RequestQuestionReference(); result.omit("kind"); result.omit("reference"); Values.object(value).forEach(result::extension); return result; }
protected RequestQuestionReference self() { return this; }
public RequestQuestionReference reference(String value) { return put("reference", value); }
public RequestQuestionReference referenceNull() { return put("reference", null); }
public RequestQuestionReference omitReference() { return omit("reference"); }
}
public static final class RequestQuestionText extends Values.Builder<RequestQuestionText> implements RequestQuestion {
public RequestQuestionText() {
put("kind", "text");
}
public static RequestQuestionText fromJson(Object value) { RequestQuestionText result = new RequestQuestionText(); result.omit("kind"); result.omit("text"); Values.object(value).forEach(result::extension); return result; }
protected RequestQuestionText self() { return this; }
public RequestQuestionText text(String value) { return put("text", value); }
public RequestQuestionText textNull() { return put("text", null); }
public RequestQuestionText omitText() { return omit("text"); }
}
public static final class RequestReader extends Values.Builder<RequestReader> {
public RequestReader() {
}
public static RequestReader fromJson(Object value) { RequestReader result = new RequestReader(); result.omit("unit"); result.omit("window"); Values.object(value).forEach(result::extension); return result; }
protected RequestReader self() { return this; }
public RequestReader unit(SourceUnit value) { return put("unit", value); }
public RequestReader unitNull() { return put("unit", null); }
public RequestReader omitUnit() { return omit("unit"); }
public RequestReader window(Number value) { return put("window", value); }
public RequestReader windowNull() { return put("window", null); }
public RequestReader omitWindow() { return omit("window"); }
}
public sealed interface RequestReaderFailure extends Values.Value permits RequestReaderFailureIo,RequestReaderFailureUtf8,RequestReaderFailureInvalidInput {
static RequestReaderFailure fromJson(Object value) { Map<String,Object> object = Values.object(value);
if ("io".equals(object.get("kind"))) return RequestReaderFailureIo.fromJson(value);
if ("utf8".equals(object.get("kind"))) return RequestReaderFailureUtf8.fromJson(value);
if ("invalid_input".equals(object.get("kind"))) return RequestReaderFailureInvalidInput.fromJson(value);
return RequestReaderFailureIo.fromJson(value);
}
}
public static final class RequestReaderFailureInvalidInput extends Values.Builder<RequestReaderFailureInvalidInput> implements RequestReaderFailure {
public RequestReaderFailureInvalidInput() {
put("kind", "invalid_input");
}
public static RequestReaderFailureInvalidInput fromJson(Object value) { RequestReaderFailureInvalidInput result = new RequestReaderFailureInvalidInput(); result.omit("kind"); result.omit("location"); Values.object(value).forEach(result::extension); return result; }
protected RequestReaderFailureInvalidInput self() { return this; }
public RequestReaderFailureInvalidInput location(SessionSourceLocation value) { return put("location", value); }
public RequestReaderFailureInvalidInput locationNull() { return put("location", null); }
public RequestReaderFailureInvalidInput omitLocation() { return omit("location"); }
}
public static final class RequestReaderFailureIo extends Values.Builder<RequestReaderFailureIo> implements RequestReaderFailure {
public RequestReaderFailureIo() {
put("kind", "io");
}
public static RequestReaderFailureIo fromJson(Object value) { RequestReaderFailureIo result = new RequestReaderFailureIo(); result.omit("kind"); result.omit("location"); Values.object(value).forEach(result::extension); return result; }
protected RequestReaderFailureIo self() { return this; }
public RequestReaderFailureIo location(SessionSourceLocation value) { return put("location", value); }
public RequestReaderFailureIo locationNull() { return put("location", null); }
public RequestReaderFailureIo omitLocation() { return omit("location"); }
}
public static final class RequestReaderFailureUtf8 extends Values.Builder<RequestReaderFailureUtf8> implements RequestReaderFailure {
public RequestReaderFailureUtf8() {
put("kind", "utf8");
}
public static RequestReaderFailureUtf8 fromJson(Object value) { RequestReaderFailureUtf8 result = new RequestReaderFailureUtf8(); result.omit("kind"); result.omit("location"); Values.object(value).forEach(result::extension); return result; }
protected RequestReaderFailureUtf8 self() { return this; }
public RequestReaderFailureUtf8 location(SessionSourceLocation value) { return put("location", value); }
public RequestReaderFailureUtf8 locationNull() { return put("location", null); }
public RequestReaderFailureUtf8 omitLocation() { return omit("location"); }
}
public static final class RequestSessionDescriptor extends Values.Builder<RequestSessionDescriptor> {
public RequestSessionDescriptor() {
}
public static RequestSessionDescriptor fromJson(Object value) { RequestSessionDescriptor result = new RequestSessionDescriptor(); result.omit("item"); result.omit("location"); Values.object(value).forEach(result::extension); return result; }
protected RequestSessionDescriptor self() { return this; }
public RequestSessionDescriptor item(RequestItem value) { return put("item", value); }
public RequestSessionDescriptor itemNull() { return put("item", null); }
public RequestSessionDescriptor omitItem() { return omit("item"); }
public RequestSessionDescriptor location(SessionSourceLocation value) { return put("location", value); }
public RequestSessionDescriptor locationNull() { return put("location", null); }
public RequestSessionDescriptor omitLocation() { return omit("location"); }
}
public static final class RequestSource extends Values.Builder<RequestSource> {
public RequestSource() {
}
public static RequestSource fromJson(Object value) { RequestSource result = new RequestSource(); result.omit("framing"); result.omit("media"); result.omit("paths"); result.omit("reading"); Values.object(value).forEach(result::extension); return result; }
protected RequestSource self() { return this; }
public RequestSource framing(RequestFraming value) { return put("framing", value); }
public RequestSource framingNull() { return put("framing", null); }
public RequestSource omitFraming() { return omit("framing"); }
public RequestSource media(ReaderMedia value) { return put("media", value); }
public RequestSource mediaNull() { return put("media", null); }
public RequestSource omitMedia() { return omit("media"); }
public RequestSource paths(List<? extends String> value) { return put("paths", value); }
public RequestSource pathsNull() { return put("paths", null); }
public RequestSource omitPaths() { return omit("paths"); }
public RequestSource reading(RequestReader value) { return put("reading", value); }
public RequestSource readingNull() { return put("reading", null); }
public RequestSource omitReading() { return omit("reading"); }
}
public static final class RequestThreshold implements Values.Value {
private final Object value;
public RequestThreshold(Number value) { this.value = Values.freeze(value); }
public RequestThreshold(String value) { this.value = Values.freeze(value); }
public Object json() { return value; }
}
public enum RequestVersion implements Values.Value { THINKTHENREQUEST1("thinkthen.request/1");
private final String value; RequestVersion(String value) { this.value = value; }
public Object json() { return value; }
}
public static final class SessionSourceLocation extends Values.Builder<SessionSourceLocation> {
public SessionSourceLocation() {
}
public static SessionSourceLocation fromJson(Object value) { SessionSourceLocation result = new SessionSourceLocation(); result.omit("file"); result.omit("first_line"); result.omit("last_line"); Values.object(value).forEach(result::extension); return result; }
protected SessionSourceLocation self() { return this; }
public SessionSourceLocation file(String value) { return put("file", value); }
public SessionSourceLocation fileNull() { return put("file", null); }
public SessionSourceLocation omitFile() { return omit("file"); }
public SessionSourceLocation firstLine(Number value) { return put("first_line", value); }
public SessionSourceLocation firstLineNull() { return put("first_line", null); }
public SessionSourceLocation omitFirstLine() { return omit("first_line"); }
public SessionSourceLocation lastLine(Number value) { return put("last_line", value); }
public SessionSourceLocation lastLineNull() { return put("last_line", null); }
public SessionSourceLocation omitLastLine() { return omit("last_line"); }
}
public enum SourceUnit implements Values.Value { LINE("line"),WINDOW("window"),FILE("file");
private final String value; SourceUnit(String value) { this.value = value; }
public Object json() { return value; }
}
}
