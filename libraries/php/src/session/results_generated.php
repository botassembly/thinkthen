<?php
// Generated from the shared Rust result graph; do not edit.
declare(strict_types=1);
namespace ThinkThen\Results;
final class NativeAnnotation extends Node {
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<\stdClass> */ public function answers(): Presence { return $this->field('answers'); }
    /** @return Presence<string> */ public function file(): Presence { return $this->field('file'); }
    /** @return Presence<int|string> */ public function first_line(): Presence { return $this->field('first_line'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<int|string> */ public function last_line(): Presence { return $this->field('last_line'); }
    /** @return Presence<NativeMeta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<NativePosition> */ public function position(): Presence { return $this->field('position'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<NativePhysicalSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<\stdClass> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeAnnotationMemberAnswerId extends Node {
    /** @return Presence<NativeAnswerYesNo|NativeAnswerChoice|NativeAnswerTag|NativeAnswerScore> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<list<NativeObservationObservationId|NativeObservationFailureId>> */ public function observations(): Presence { return $this->field('observations'); }
    /** @return Presence<NativeReadableQuestionDecide|NativeReadableQuestionChoose|NativeReadableQuestionTag|NativeReadableQuestionScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<list<NativeQuestionSource>> */ public function question_sources(): Presence { return $this->field('question_sources'); }
    /** @return Presence<string> */ public function request(): Presence { return $this->field('request'); }
    /** @return Presence<int|float|string|null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<NativeUsage> */ public function usage(): Presence { return $this->field('usage'); }
    /** @return Presence<mixed> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeAnnotationMemberFailureId extends Node {
    /** @return Presence<NativeFailure> */ public function failure(): Presence { return $this->field('failure'); }
    /** @return Presence<string> */ public function failure_id(): Presence { return $this->field('failure_id'); }
    /** @return Presence<list<NativeObservationObservationId|NativeObservationFailureId>> */ public function observations(): Presence { return $this->field('observations'); }
    /** @return Presence<NativeReadableQuestionDecide|NativeReadableQuestionChoose|NativeReadableQuestionTag|NativeReadableQuestionScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<list<NativeQuestionSource>> */ public function question_sources(): Presence { return $this->field('question_sources'); }
    /** @return Presence<string> */ public function request(): Presence { return $this->field('request'); }
    /** @return Presence<int|float|string|null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<NativeUsage> */ public function usage(): Presence { return $this->field('usage'); }
}
final class NativeAnnotationValueChoice extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string|null> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeAnnotationValueDecision extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<bool|null> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeAnnotationValueFailed extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<NativeFailure> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeAnnotationValueScore extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|float> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeAnnotationValueTags extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<string>> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeAnswers extends Node {
    /** @return Presence<list<NativeRelationMemberAnswerId|NativeRelationMemberFailureId>> */ public function questions(): Presence { return $this->field('questions'); }
}
final class NativeAtomicArrayOfString extends Node {
    /** @return Presence<NativeAnswerYesNo|NativeAnswerChoice|NativeAnswerTag|NativeAnswerScore> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<list<NativeImage>> */ public function images(): Presence { return $this->field('images'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<list<NativeRankMember>> */ public function members(): Presence { return $this->field('members'); }
    /** @return Presence<NativeMeta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<NativeReadableQuestionDecide|NativeReadableQuestionChoose|NativeReadableQuestionTag|NativeReadableQuestionScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function question_name(): Presence { return $this->field('question_name'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<NativePhysicalSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<int|float|string|null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<list<string>> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeAtomicDecideValue extends Node {
    /** @return Presence<NativeAnswerYesNo|NativeAnswerChoice|NativeAnswerTag|NativeAnswerScore> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<list<NativeImage>> */ public function images(): Presence { return $this->field('images'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<list<NativeRankMember>> */ public function members(): Presence { return $this->field('members'); }
    /** @return Presence<NativeMeta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<NativeReadableQuestionDecide|NativeReadableQuestionChoose|NativeReadableQuestionTag|NativeReadableQuestionScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function question_name(): Presence { return $this->field('question_name'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<NativePhysicalSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<int|float|string|null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<bool|null|mixed> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeAtomicNonZeroUsize extends Node {
    /** @return Presence<NativeAnswerYesNo|NativeAnswerChoice|NativeAnswerTag|NativeAnswerScore> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<list<NativeImage>> */ public function images(): Presence { return $this->field('images'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<list<NativeRankMember>> */ public function members(): Presence { return $this->field('members'); }
    /** @return Presence<NativeMeta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<NativeReadableQuestionDecide|NativeReadableQuestionChoose|NativeReadableQuestionTag|NativeReadableQuestionScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function question_name(): Presence { return $this->field('question_name'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<NativePhysicalSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<int|float|string|null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<int|string> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeAtomicNullableString extends Node {
    /** @return Presence<NativeAnswerYesNo|NativeAnswerChoice|NativeAnswerTag|NativeAnswerScore> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<list<NativeImage>> */ public function images(): Presence { return $this->field('images'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<list<NativeRankMember>> */ public function members(): Presence { return $this->field('members'); }
    /** @return Presence<NativeMeta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<NativeReadableQuestionDecide|NativeReadableQuestionChoose|NativeReadableQuestionTag|NativeReadableQuestionScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function question_name(): Presence { return $this->field('question_name'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<NativePhysicalSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<int|float|string|null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<string|null> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeAtomicBoolean extends Node {
    /** @return Presence<NativeAnswerYesNo|NativeAnswerChoice|NativeAnswerTag|NativeAnswerScore> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<list<NativeImage>> */ public function images(): Presence { return $this->field('images'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<list<NativeRankMember>> */ public function members(): Presence { return $this->field('members'); }
    /** @return Presence<NativeMeta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<NativeReadableQuestionDecide|NativeReadableQuestionChoose|NativeReadableQuestionTag|NativeReadableQuestionScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function question_name(): Presence { return $this->field('question_name'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<NativePhysicalSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<int|float|string|null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<bool> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeAtomicDouble extends Node {
    /** @return Presence<NativeAnswerYesNo|NativeAnswerChoice|NativeAnswerTag|NativeAnswerScore> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<list<NativeImage>> */ public function images(): Presence { return $this->field('images'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<list<NativeRankMember>> */ public function members(): Presence { return $this->field('members'); }
    /** @return Presence<NativeMeta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<NativeReadableQuestionDecide|NativeReadableQuestionChoose|NativeReadableQuestionTag|NativeReadableQuestionScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function question_name(): Presence { return $this->field('question_name'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<NativePhysicalSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<int|float|string|null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<int|float> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeAttempt extends Node {
    /** @return Presence<int|string> */ public function ordinal(): Presence { return $this->field('ordinal'); }
    /** @return Presence<mixed> */ public function outcome(): Presence { return $this->field('outcome'); }
    /** @return Presence<string> */ public function request_id(): Presence { return $this->field('request_id'); }
    /** @return Presence<string> */ public function request_sha256(): Presence { return $this->field('request_sha256'); }
    /** @return Presence<string> */ public function sdk_request_id(): Presence { return $this->field('sdk_request_id'); }
    /** @return Presence<int|string> */ public function server_ms(): Presence { return $this->field('server_ms'); }
    /** @return Presence<int|string> */ public function status(): Presence { return $this->field('status'); }
    /** @return Presence<int|string> */ public function wall_ms(): Presence { return $this->field('wall_ms'); }
}
final class NativeBoundaryOdds extends Node {
    /** @return Presence<list<NativePieceOdds>> */ public function pieces(): Presence { return $this->field('pieces'); }
    /** @return Presence<list<NativeBoundaryProposal>> */ public function proposals(): Presence { return $this->field('proposals'); }
}
final class NativeBoundaryProposal extends Node {
    /** @return Presence<int|string> */ public function end(): Presence { return $this->field('end'); }
    /** @return Presence<int|string> */ public function length(): Presence { return $this->field('length'); }
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
    /** @return Presence<int|string> */ public function start(): Presence { return $this->field('start'); }
    /** @return Presence<string> */ public function text(): Presence { return $this->field('text'); }
}
final class NativeCallError extends Node {
    /** @return Presence<NativeError> */ public function error(): Presence { return $this->field('error'); }
    /** @return Presence<NativeFacts> */ public function facts(): Presence { return $this->field('facts'); }
}
final class NativeEntityDocument extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
}
final class NativeError extends Node {
    /** @return Presence<NativeEstimatedInputDenialInitialRequest|NativeEstimatedInputDenialAdditionalRequest|NativeEstimatedInputDenialRetry> */ public function estimated_input_denial(): Presence { return $this->field('estimated_input_denial'); }
    /** @return Presence<mixed> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string> */ public function message(): Presence { return $this->field('message'); }
    /** @return Presence<bool> */ public function retryable(): Presence { return $this->field('retryable'); }
    /** @return Presence<NativeSendBudgetDenialBeforeFirstSend|NativeSendBudgetDenialBeforeAdditionalSend|NativeSendBudgetDenialBeforeRetry> */ public function send_budget_denial(): Presence { return $this->field('send_budget_denial'); }
    /** @return Presence<NativeStopped> */ public function stopped(): Presence { return $this->field('stopped'); }
}
final class NativeEstimatedInputDenialAdditionalRequest extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|string> */ public function limit(): Presence { return $this->field('limit'); }
}
final class NativeEstimatedInputDenialInitialRequest extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|string> */ public function limit(): Presence { return $this->field('limit'); }
}
final class NativeEstimatedInputDenialRetry extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|string> */ public function last_status(): Presence { return $this->field('last_status'); }
    /** @return Presence<int|string> */ public function limit(): Presence { return $this->field('limit'); }
}
final class NativeFacts extends Node {
    /** @return Presence<list<NativeAttempt>> */ public function attempts(): Presence { return $this->field('attempts'); }
    /** @return Presence<int|string> */ public function cache_answers(): Presence { return $this->field('cache_answers'); }
    /** @return Presence<string> */ public function call_id(): Presence { return $this->field('call_id'); }
    /** @return Presence<string> */ public function estimated_cost_usd(): Presence { return $this->field('estimated_cost_usd'); }
    /** @return Presence<bool> */ public function held_model_mismatch(): Presence { return $this->field('held_model_mismatch'); }
    /** @return Presence<int|string> */ public function input_tokens(): Presence { return $this->field('input_tokens'); }
    /** @return Presence<int|string> */ public function largest_request_bytes(): Presence { return $this->field('largest_request_bytes'); }
    /** @return Presence<int|string|null> */ public function largest_request_estimated_input_tokens(): Presence { return $this->field('largest_request_estimated_input_tokens'); }
    /** @return Presence<string> */ public function model(): Presence { return $this->field('model'); }
    /** @return Presence<int|string> */ public function output_tokens(): Presence { return $this->field('output_tokens'); }
    /** @return Presence<int|string> */ public function records(): Presence { return $this->field('records'); }
    /** @return Presence<int|string> */ public function requests_sent(): Presence { return $this->field('requests_sent'); }
    /** @return Presence<int|float> */ public function seconds(): Presence { return $this->field('seconds'); }
    /** @return Presence<string> */ public function token_estimate_method(): Presence { return $this->field('token_estimate_method'); }
    /** @return Presence<NativePersistenceObservation> */ public function usage_persistence(): Presence { return $this->field('usage_persistence'); }
}
final class NativeFind extends Node {
    /** @return Presence<NativeFindAnswer> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<list<NativeFindCandidate>> */ public function candidates(): Presence { return $this->field('candidates'); }
    /** @return Presence<string> */ public function file(): Presence { return $this->field('file'); }
    /** @return Presence<int|string> */ public function first_line(): Presence { return $this->field('first_line'); }
    /** @return Presence<int|string|null> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<int|string> */ public function last_line(): Presence { return $this->field('last_line'); }
    /** @return Presence<NativeMeta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<NativePosition> */ public function position(): Presence { return $this->field('position'); }
    /** @return Presence<NativeReadableQuestion2> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<mixed> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeFindCandidate extends Node {
    /** @return Presence<int|string|null> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
    /** @return Presence<NativePhysicalSource> */ public function source(): Presence { return $this->field('source'); }
}
final class NativeImage extends Node {
    /** @return Presence<string> */ public function base64(): Presence { return $this->field('base64'); }
    /** @return Presence<int|string> */ public function height(): Presence { return $this->field('height'); }
    /** @return Presence<mixed> */ public function media(): Presence { return $this->field('media'); }
    /** @return Presence<int|string> */ public function width(): Presence { return $this->field('width'); }
}
final class NativeInputDeclarationObject extends Node {
    /** @return Presence<\stdClass> */ public function properties(): Presence { return $this->field('properties'); }
    /** @return Presence<list<string>> */ public function required(): Presence { return $this->field('required'); }
    /** @return Presence<string> */ public function type(): Presence { return $this->field('type'); }
}
final class NativeInputDeclarationString extends Node {
    /** @return Presence<string> */ public function type(): Presence { return $this->field('type'); }
}
final class NativeInputPropertyTypeArray extends Node {
    /** @return Presence<NativeStringRoot> */ public function items(): Presence { return $this->field('items'); }
    /** @return Presence<string> */ public function type(): Presence { return $this->field('type'); }
}
final class NativeInputPropertyTypeBoolean extends Node {
    /** @return Presence<string> */ public function type(): Presence { return $this->field('type'); }
}
final class NativeInputPropertyTypeNumber extends Node {
    /** @return Presence<string> */ public function type(): Presence { return $this->field('type'); }
}
final class NativeInputPropertyTypeString extends Node {
    /** @return Presence<string> */ public function type(): Presence { return $this->field('type'); }
}
final class NativeLabel extends Node {
    /** @return Presence<mixed> */ public function description(): Presence { return $this->field('description'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
}
final class NativeMeta extends Node {
    /** @return Presence<string> */ public function answered_by(): Presence { return $this->field('answered_by'); }
    /** @return Presence<list<NativeAttempt>> */ public function attempts(): Presence { return $this->field('attempts'); }
    /** @return Presence<int|string|string> */ public function batch_setting(): Presence { return $this->field('batch_setting'); }
    /** @return Presence<NativeBatchWarning> */ public function batch_warning(): Presence { return $this->field('batch_warning'); }
    /** @return Presence<bool> */ public function cached(): Presence { return $this->field('cached'); }
    /** @return Presence<string> */ public function context_sha256(): Presence { return $this->field('context_sha256'); }
    /** @return Presence<int|string> */ public function failed_questions(): Presence { return $this->field('failed_questions'); }
    /** @return Presence<string> */ public function model(): Presence { return $this->field('model'); }
    /** @return Presence<list<NativeObservationObservationId|NativeObservationFailureId>> */ public function observations(): Presence { return $this->field('observations'); }
    /** @return Presence<mixed|null> */ public function origin(): Presence { return $this->field('origin'); }
    /** @return Presence<NativeProfileWarning> */ public function profile_warning(): Presence { return $this->field('profile_warning'); }
    /** @return Presence<string> */ public function question_sha256(): Presence { return $this->field('question_sha256'); }
    /** @return Presence<list<NativeQuestionSource>> */ public function question_sources(): Presence { return $this->field('question_sources'); }
    /** @return Presence<string> */ public function questions_sha256(): Presence { return $this->field('questions_sha256'); }
    /** @return Presence<list<string>> */ public function requests(): Presence { return $this->field('requests'); }
    /** @return Presence<int|string> */ public function requests_sent(): Presence { return $this->field('requests_sent'); }
    /** @return Presence<string> */ public function tool(): Presence { return $this->field('tool'); }
    /** @return Presence<string> */ public function url(): Presence { return $this->field('url'); }
    /** @return Presence<NativeUsage> */ public function usage(): Presence { return $this->field('usage'); }
}
final class NativeObjectRoot extends Node {
    /** @return Presence<\stdClass> */ public function properties(): Presence { return $this->field('properties'); }
    /** @return Presence<list<string>> */ public function required(): Presence { return $this->field('required'); }
    /** @return Presence<string> */ public function type(): Presence { return $this->field('type'); }
}
final class NativeObservationFailureId extends Node {
    /** @return Presence<string> */ public function failure_id(): Presence { return $this->field('failure_id'); }
}
final class NativeObservationObservationId extends Node {
    /** @return Presence<string> */ public function observation_id(): Presence { return $this->field('observation_id'); }
}
final class NativePersistenceObservation extends Node {
    /** @return Presence<string> */ public function advice(): Presence { return $this->field('advice'); }
    /** @return Presence<string> */ public function observed_at(): Presence { return $this->field('observed_at'); }
    /** @return Presence<mixed> */ public function state(): Presence { return $this->field('state'); }
}
final class NativePhysicalSource extends Node {
    /** @return Presence<string> */ public function file(): Presence { return $this->field('file'); }
    /** @return Presence<int|string> */ public function first_line(): Presence { return $this->field('first_line'); }
    /** @return Presence<int|string> */ public function last_line(): Presence { return $this->field('last_line'); }
}
final class NativePosition extends Node {
    /** @return Presence<string|null> */ public function file(): Presence { return $this->field('file'); }
    /** @return Presence<int|string> */ public function first(): Presence { return $this->field('first'); }
    /** @return Presence<list<string>> */ public function images(): Presence { return $this->field('images'); }
    /** @return Presence<int|string> */ public function last(): Presence { return $this->field('last'); }
}
final class NativeQuestionSource extends Node {
    /** @return Presence<string> */ public function answered_by(): Presence { return $this->field('answered_by'); }
    /** @return Presence<int|string> */ public function batch_size(): Presence { return $this->field('batch_size'); }
    /** @return Presence<mixed> */ public function origin(): Presence { return $this->field('origin'); }
}
final class NativeRankMember extends Node {
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<NativeRankMemberResult> */ public function result(): Presence { return $this->field('result'); }
}
final class NativeRankMemberResult extends Node {
    /** @return Presence<NativeAnswerYesNo|NativeAnswerChoice|NativeAnswerTag|NativeAnswerScore> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<list<NativeImage>> */ public function images(): Presence { return $this->field('images'); }
    /** @return Presence<NativeMeta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<NativeReadableQuestionDecide|NativeReadableQuestionChoose|NativeReadableQuestionTag|NativeReadableQuestionScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<NativePhysicalSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<int|string> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeReadableQuestion2 extends Node {
    /** @return Presence<int|string|string> */ public function batch(): Presence { return $this->field('batch'); }
    /** @return Presence<NativeInputDeclarationString|NativeInputDeclarationObject> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<NativeInputDeclarationString|NativeInputDeclarationObject> */ public function item_schema(): Presence { return $this->field('item_schema'); }
    /** @return Presence<list<NativeLabel>> */ public function label_details(): Presence { return $this->field('label_details'); }
    /** @return Presence<string> */ public function model(): Presence { return $this->field('model'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<bool> */ public function none(): Presence { return $this->field('none'); }
    /** @return Presence<list<string>> */ public function on(): Presence { return $this->field('on'); }
    /** @return Presence<string> */ public function profile(): Presence { return $this->field('profile'); }
    /** @return Presence<mixed> */ public function text(): Presence { return $this->field('text'); }
    /** @return Presence<mixed> */ public function verb(): Presence { return $this->field('verb'); }
    /** @return Presence<int|string> */ public function wording_version(): Presence { return $this->field('wording_version'); }
}
final class NativeReadableQuestion3 extends Node {
    /** @return Presence<int|string|string> */ public function batch(): Presence { return $this->field('batch'); }
    /** @return Presence<NativeInputDeclarationString|NativeInputDeclarationObject> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<mixed> */ public function entity_definition(): Presence { return $this->field('entity_definition'); }
    /** @return Presence<mixed> */ public function instructions(): Presence { return $this->field('instructions'); }
    /** @return Presence<NativeInputDeclarationString|NativeInputDeclarationObject> */ public function item_schema(): Presence { return $this->field('item_schema'); }
    /** @return Presence<\stdClass> */ public function kinds(): Presence { return $this->field('kinds'); }
    /** @return Presence<list<NativeLabel>> */ public function label_details(): Presence { return $this->field('label_details'); }
    /** @return Presence<mixed> */ public function mode(): Presence { return $this->field('mode'); }
    /** @return Presence<string> */ public function model(): Presence { return $this->field('model'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<list<string>> */ public function on(): Presence { return $this->field('on'); }
    /** @return Presence<string> */ public function profile(): Presence { return $this->field('profile'); }
    /** @return Presence<int|float|string> */ public function relation_threshold(): Presence { return $this->field('relation_threshold'); }
    /** @return Presence<list<NativeRelationRule>> */ public function relations(): Presence { return $this->field('relations'); }
    /** @return Presence<int|string> */ public function snippet_pieces(): Presence { return $this->field('snippet_pieces'); }
    /** @return Presence<NativeRecognitionStageContext> */ public function stage_context(): Presence { return $this->field('stage_context'); }
    /** @return Presence<int|float|string> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<string> */ public function verb(): Presence { return $this->field('verb'); }
    /** @return Presence<int|string> */ public function wording_version(): Presence { return $this->field('wording_version'); }
}
final class NativeReadableQuestion4 extends Node {
    /** @return Presence<int|string|string> */ public function batch(): Presence { return $this->field('batch'); }
    /** @return Presence<NativeInputDeclarationString|NativeInputDeclarationObject> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<NativeRelateFields|null> */ public function fields(): Presence { return $this->field('fields'); }
    /** @return Presence<NativeInputDeclarationString|NativeInputDeclarationObject> */ public function item_schema(): Presence { return $this->field('item_schema'); }
    /** @return Presence<list<NativeLabel>> */ public function label_details(): Presence { return $this->field('label_details'); }
    /** @return Presence<string> */ public function model(): Presence { return $this->field('model'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<list<string>> */ public function on(): Presence { return $this->field('on'); }
    /** @return Presence<string> */ public function profile(): Presence { return $this->field('profile'); }
    /** @return Presence<list<NativeRelationRule>> */ public function relations(): Presence { return $this->field('relations'); }
    /** @return Presence<int|float|string> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<mixed> */ public function verb(): Presence { return $this->field('verb'); }
    /** @return Presence<int|string> */ public function wording_version(): Presence { return $this->field('wording_version'); }
}
final class NativeReadableQuestionChoose extends Node {
    /** @return Presence<int|string|string> */ public function batch(): Presence { return $this->field('batch'); }
    /** @return Presence<NativeInputDeclarationString|NativeInputDeclarationObject> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<NativeInputDeclarationString|NativeInputDeclarationObject> */ public function item_schema(): Presence { return $this->field('item_schema'); }
    /** @return Presence<list<NativeLabel>> */ public function label_details(): Presence { return $this->field('label_details'); }
    /** @return Presence<string> */ public function model(): Presence { return $this->field('model'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<list<string>> */ public function on(): Presence { return $this->field('on'); }
    /** @return Presence<string> */ public function profile(): Presence { return $this->field('profile'); }
    /** @return Presence<int|string> */ public function wording_version(): Presence { return $this->field('wording_version'); }
    /** @return Presence<list<string>> */ public function options(): Presence { return $this->field('options'); }
    /** @return Presence<mixed> */ public function text(): Presence { return $this->field('text'); }
    /** @return Presence<string> */ public function verb(): Presence { return $this->field('verb'); }
}
final class NativeReadableQuestionDecide extends Node {
    /** @return Presence<int|string|string> */ public function batch(): Presence { return $this->field('batch'); }
    /** @return Presence<NativeInputDeclarationString|NativeInputDeclarationObject> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<NativeInputDeclarationString|NativeInputDeclarationObject> */ public function item_schema(): Presence { return $this->field('item_schema'); }
    /** @return Presence<list<NativeLabel>> */ public function label_details(): Presence { return $this->field('label_details'); }
    /** @return Presence<string> */ public function model(): Presence { return $this->field('model'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<list<string>> */ public function on(): Presence { return $this->field('on'); }
    /** @return Presence<string> */ public function profile(): Presence { return $this->field('profile'); }
    /** @return Presence<int|string> */ public function wording_version(): Presence { return $this->field('wording_version'); }
    /** @return Presence<mixed> */ public function false(): Presence { return $this->field('false'); }
    /** @return Presence<mixed> */ public function text(): Presence { return $this->field('text'); }
    /** @return Presence<mixed> */ public function true(): Presence { return $this->field('true'); }
    /** @return Presence<string> */ public function verb(): Presence { return $this->field('verb'); }
}
final class NativeReadableQuestionScore extends Node {
    /** @return Presence<int|string|string> */ public function batch(): Presence { return $this->field('batch'); }
    /** @return Presence<NativeInputDeclarationString|NativeInputDeclarationObject> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<NativeInputDeclarationString|NativeInputDeclarationObject> */ public function item_schema(): Presence { return $this->field('item_schema'); }
    /** @return Presence<list<NativeLabel>> */ public function label_details(): Presence { return $this->field('label_details'); }
    /** @return Presence<string> */ public function model(): Presence { return $this->field('model'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<list<string>> */ public function on(): Presence { return $this->field('on'); }
    /** @return Presence<string> */ public function profile(): Presence { return $this->field('profile'); }
    /** @return Presence<int|string> */ public function wording_version(): Presence { return $this->field('wording_version'); }
    /** @return Presence<list<string>> */ public function levels(): Presence { return $this->field('levels'); }
    /** @return Presence<mixed> */ public function text(): Presence { return $this->field('text'); }
    /** @return Presence<string> */ public function verb(): Presence { return $this->field('verb'); }
}
final class NativeReadableQuestionTag extends Node {
    /** @return Presence<int|string|string> */ public function batch(): Presence { return $this->field('batch'); }
    /** @return Presence<NativeInputDeclarationString|NativeInputDeclarationObject> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<NativeInputDeclarationString|NativeInputDeclarationObject> */ public function item_schema(): Presence { return $this->field('item_schema'); }
    /** @return Presence<list<NativeLabel>> */ public function label_details(): Presence { return $this->field('label_details'); }
    /** @return Presence<string> */ public function model(): Presence { return $this->field('model'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<list<string>> */ public function on(): Presence { return $this->field('on'); }
    /** @return Presence<string> */ public function profile(): Presence { return $this->field('profile'); }
    /** @return Presence<int|string> */ public function wording_version(): Presence { return $this->field('wording_version'); }
    /** @return Presence<list<string>> */ public function labels(): Presence { return $this->field('labels'); }
    /** @return Presence<mixed> */ public function text(): Presence { return $this->field('text'); }
    /** @return Presence<string> */ public function verb(): Presence { return $this->field('verb'); }
}
final class NativeRecognition extends Node {
    /** @return Presence<NativeRecognitionOddsFieldsNamesPairsPiecesProposals|NativeRecognitionOddsFieldsPiecesProposals> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<string> */ public function file(): Presence { return $this->field('file'); }
    /** @return Presence<int|string> */ public function first_line(): Presence { return $this->field('first_line'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<int|string> */ public function last_line(): Presence { return $this->field('last_line'); }
    /** @return Presence<NativeMeta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<NativePosition> */ public function position(): Presence { return $this->field('position'); }
    /** @return Presence<NativeReadableQuestion3> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<NativePhysicalSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<NativeRecognizeFieldsEntities|NativeRecognizeFieldsModeProposals> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeRecognitionEdgeDocument extends Node {
    /** @return Presence<bool> */ public function either(): Presence { return $this->field('either'); }
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
    /** @return Presence<string> */ public function relation(): Presence { return $this->field('relation'); }
    /** @return Presence<NativeEntity> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<NativeEntity> */ public function target(): Presence { return $this->field('target'); }
}
final class NativeRecognitionOddsFieldsNamesPairsPiecesProposals extends Node {
    /** @return Presence<list<NativeNameOdds>> */ public function names(): Presence { return $this->field('names'); }
    /** @return Presence<list<NativePairOdds>> */ public function pairs(): Presence { return $this->field('pairs'); }
    /** @return Presence<list<NativePieceOdds>> */ public function pieces(): Presence { return $this->field('pieces'); }
    /** @return Presence<list<NativeRecognitionProposal>> */ public function proposals(): Presence { return $this->field('proposals'); }
}
final class NativeRecognitionOddsFieldsPiecesProposals extends Node {
    /** @return Presence<list<NativePieceOdds>> */ public function pieces(): Presence { return $this->field('pieces'); }
    /** @return Presence<list<NativeBoundaryProposal>> */ public function proposals(): Presence { return $this->field('proposals'); }
}
final class NativeRecognitionProposal extends Node {
    /** @return Presence<int|string> */ public function end(): Presence { return $this->field('end'); }
    /** @return Presence<bool> */ public function kept(): Presence { return $this->field('kept'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<NativePlace> */ public function selected(): Presence { return $this->field('selected'); }
    /** @return Presence<int|float> */ public function span_probability(): Presence { return $this->field('span_probability'); }
    /** @return Presence<int|string> */ public function start(): Presence { return $this->field('start'); }
    /** @return Presence<int|float> */ public function strength(): Presence { return $this->field('strength'); }
}
final class NativeRecognitionStageContext extends Node {
    /** @return Presence<string> */ public function boundary(): Presence { return $this->field('boundary'); }
    /** @return Presence<string> */ public function kind_edge(): Presence { return $this->field('kind_edge'); }
    /** @return Presence<string> */ public function relation(): Presence { return $this->field('relation'); }
}
final class NativeRelation extends Node {
    /** @return Presence<NativeAnswers> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<string> */ public function file(): Presence { return $this->field('file'); }
    /** @return Presence<int|string> */ public function first_line(): Presence { return $this->field('first_line'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<list<NativeSessionInputSource>> */ public function input_sources(): Presence { return $this->field('input_sources'); }
    /** @return Presence<int|string> */ public function last_line(): Presence { return $this->field('last_line'); }
    /** @return Presence<NativeMeta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<NativePosition> */ public function position(): Presence { return $this->field('position'); }
    /** @return Presence<NativeReadableQuestion4> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<list<NativeRelatedEntityEdge>> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeRelationMemberAnswerId extends Node {
    /** @return Presence<mixed> */ public function direction(): Presence { return $this->field('direction'); }
    /** @return Presence<mixed> */ public function method(): Presence { return $this->field('method'); }
    /** @return Presence<list<NativeObservationObservationId|NativeObservationFailureId>> */ public function observations(): Presence { return $this->field('observations'); }
    /** @return Presence<NativeReadableQuestionDecide|NativeReadableQuestionChoose|NativeReadableQuestionTag|NativeReadableQuestionScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<list<NativeQuestionSource>> */ public function question_sources(): Presence { return $this->field('question_sources'); }
    /** @return Presence<string> */ public function reads(): Presence { return $this->field('reads'); }
    /** @return Presence<string> */ public function relation(): Presence { return $this->field('relation'); }
    /** @return Presence<string> */ public function request(): Presence { return $this->field('request'); }
    /** @return Presence<NativeRelatedEntity> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<NativeRelatedEntity|null> */ public function target(): Presence { return $this->field('target'); }
    /** @return Presence<int|float|string> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<NativeUsage> */ public function usage(): Presence { return $this->field('usage'); }
    /** @return Presence<bool> */ public function accepted(): Presence { return $this->field('accepted'); }
    /** @return Presence<NativeAnswerYesNo|NativeAnswerChoice|NativeAnswerTag|NativeAnswerScore> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
}
final class NativeRelationMemberFailureId extends Node {
    /** @return Presence<mixed> */ public function direction(): Presence { return $this->field('direction'); }
    /** @return Presence<mixed> */ public function method(): Presence { return $this->field('method'); }
    /** @return Presence<list<NativeObservationObservationId|NativeObservationFailureId>> */ public function observations(): Presence { return $this->field('observations'); }
    /** @return Presence<NativeReadableQuestionDecide|NativeReadableQuestionChoose|NativeReadableQuestionTag|NativeReadableQuestionScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<list<NativeQuestionSource>> */ public function question_sources(): Presence { return $this->field('question_sources'); }
    /** @return Presence<string> */ public function reads(): Presence { return $this->field('reads'); }
    /** @return Presence<string> */ public function relation(): Presence { return $this->field('relation'); }
    /** @return Presence<string> */ public function request(): Presence { return $this->field('request'); }
    /** @return Presence<NativeRelatedEntity> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<NativeRelatedEntity|null> */ public function target(): Presence { return $this->field('target'); }
    /** @return Presence<int|float|string> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<NativeUsage> */ public function usage(): Presence { return $this->field('usage'); }
    /** @return Presence<NativeFailure> */ public function failure(): Presence { return $this->field('failure'); }
    /** @return Presence<string> */ public function failure_id(): Presence { return $this->field('failure_id'); }
}
final class NativeSendBudgetDenialBeforeAdditionalSend extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
}
final class NativeSendBudgetDenialBeforeFirstSend extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
}
final class NativeSendBudgetDenialBeforeRetry extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|string> */ public function last_status(): Presence { return $this->field('last_status'); }
}
final class NativeStopped extends Node {
    /** @return Presence<int|string> */ public function at(): Presence { return $this->field('at'); }
    /** @return Presence<mixed> */ public function cause(): Presence { return $this->field('cause'); }
    /** @return Presence<bool> */ public function retryable(): Presence { return $this->field('retryable'); }
    /** @return Presence<int|string> */ public function status(): Presence { return $this->field('status'); }
}
final class NativeStringRoot extends Node {
    /** @return Presence<string> */ public function type(): Presence { return $this->field('type'); }
}
final class NativeUsage extends Node {
    /** @return Presence<int|string> */ public function input_tokens(): Presence { return $this->field('input_tokens'); }
    /** @return Presence<int|string> */ public function output_tokens(): Presence { return $this->field('output_tokens'); }
}
final class NativeAnswerChoice extends Node {
    /** @return Presence<int|float> */ public function confidence(): Presence { return $this->field('confidence'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string> */ public function pick(): Presence { return $this->field('pick'); }
    /** @return Presence<\stdClass> */ public function probabilities(): Presence { return $this->field('probabilities'); }
}
final class NativeAnswerScore extends Node {
    /** @return Presence<int|float> */ public function confidence(): Presence { return $this->field('confidence'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string> */ public function level(): Presence { return $this->field('level'); }
    /** @return Presence<\stdClass> */ public function probabilities(): Presence { return $this->field('probabilities'); }
}
final class NativeAnswerTag extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<\stdClass> */ public function probabilities(): Presence { return $this->field('probabilities'); }
}
final class NativeAnswerYesNo extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
}
final class NativeBatchWarning extends Node {
    /** @return Presence<int|string|string> */ public function running(): Presence { return $this->field('running'); }
    /** @return Presence<int|string|string> */ public function tuned_for(): Presence { return $this->field('tuned_for'); }
}
final class NativeEntity extends Node {
    /** @return Presence<int|string> */ public function end(): Presence { return $this->field('end'); }
    /** @return Presence<string> */ public function file(): Presence { return $this->field('file'); }
    /** @return Presence<int|string> */ public function first_line(): Presence { return $this->field('first_line'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|string> */ public function last_line(): Presence { return $this->field('last_line'); }
    /** @return Presence<int|string> */ public function length(): Presence { return $this->field('length'); }
    /** @return Presence<int|string> */ public function start(): Presence { return $this->field('start'); }
    /** @return Presence<int|float> */ public function strength(): Presence { return $this->field('strength'); }
    /** @return Presence<string> */ public function text(): Presence { return $this->field('text'); }
}
final class NativeEntityEdge extends Node {
    /** @return Presence<bool> */ public function either(): Presence { return $this->field('either'); }
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
    /** @return Presence<string> */ public function relation(): Presence { return $this->field('relation'); }
    /** @return Presence<NativeEntity> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<NativeEntity> */ public function target(): Presence { return $this->field('target'); }
}
final class NativeFailed extends Node {
    /** @return Presence<NativeFailure> */ public function failed(): Presence { return $this->field('failed'); }
}
final class NativeFailure extends Node {
    /** @return Presence<mixed> */ public function cause(): Presence { return $this->field('cause'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
}
final class NativeFindAnswer extends Node {
    /** @return Presence<int|float> */ public function confidence(): Presence { return $this->field('confidence'); }
    /** @return Presence<mixed> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string> */ public function pick(): Presence { return $this->field('pick'); }
    /** @return Presence<\stdClass> */ public function probabilities(): Presence { return $this->field('probabilities'); }
}
final class NativeNameOdds extends Node {
    /** @return Presence<\stdClass|null> */ public function edges(): Presence { return $this->field('edges'); }
    /** @return Presence<int|string> */ public function end(): Presence { return $this->field('end'); }
    /** @return Presence<\stdClass|null> */ public function kinds(): Presence { return $this->field('kinds'); }
    /** @return Presence<int|string> */ public function start(): Presence { return $this->field('start'); }
}
final class NativePairOdds extends Node {
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
    /** @return Presence<string> */ public function relation(): Presence { return $this->field('relation'); }
    /** @return Presence<NativePlace> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<NativePlace> */ public function target(): Presence { return $this->field('target'); }
}
final class NativePieceOdds extends Node {
    /** @return Presence<int|string> */ public function end(): Presence { return $this->field('end'); }
    /** @return Presence<int|string> */ public function start(): Presence { return $this->field('start'); }
    /** @return Presence<\stdClass> */ public function tags(): Presence { return $this->field('tags'); }
}
final class NativePlace extends Node {
    /** @return Presence<int|string> */ public function end(): Presence { return $this->field('end'); }
    /** @return Presence<int|string> */ public function start(): Presence { return $this->field('start'); }
}
final class NativeProfileWarning extends Node {
    /** @return Presence<string> */ public function running(): Presence { return $this->field('running'); }
    /** @return Presence<string> */ public function tuned_for(): Presence { return $this->field('tuned_for'); }
}
final class NativeRecognizeAnswer extends Node {
    /** @return Presence<list<NativeNameOdds>> */ public function names(): Presence { return $this->field('names'); }
    /** @return Presence<list<NativePairOdds>> */ public function pairs(): Presence { return $this->field('pairs'); }
    /** @return Presence<list<NativePieceOdds>> */ public function pieces(): Presence { return $this->field('pieces'); }
    /** @return Presence<list<NativeRecognitionProposal>> */ public function proposals(): Presence { return $this->field('proposals'); }
}
final class NativeRecognizeFieldsEntities extends Node {
    /** @return Presence<list<NativeEntity>> */ public function entities(): Presence { return $this->field('entities'); }
    /** @return Presence<list<NativeEntityEdge>> */ public function relations(): Presence { return $this->field('relations'); }
}
final class NativeRecognizeFieldsModeProposals extends Node {
    /** @return Presence<string> */ public function mode(): Presence { return $this->field('mode'); }
    /** @return Presence<list<NativeBoundaryProposal>> */ public function proposals(): Presence { return $this->field('proposals'); }
}
final class NativeRelateFields extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
}
final class NativeRelatedEntity extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
}
final class NativeRelatedEntityEdge extends Node {
    /** @return Presence<bool> */ public function either(): Presence { return $this->field('either'); }
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
    /** @return Presence<string> */ public function relation(): Presence { return $this->field('relation'); }
    /** @return Presence<NativeRelatedEntityEdgePropertiesSourceFieldsKindName|NativeRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<NativeRelatedEntityEdgePropertiesSourceFieldsKindName|NativeRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord> */ public function target(): Presence { return $this->field('target'); }
}
final class NativeRelatedEntityEdgePropertiesSourceFieldsFileKindNameOrdinalRecord extends Node {
    /** @return Presence<string|null> */ public function file(): Presence { return $this->field('file'); }
    /** @return Presence<int|string> */ public function first_line(): Presence { return $this->field('first_line'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|string> */ public function last_line(): Presence { return $this->field('last_line'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<int|string> */ public function ordinal(): Presence { return $this->field('ordinal'); }
    /** @return Presence<mixed> */ public function record(): Presence { return $this->field('record'); }
}
final class NativeRelatedEntityEdgePropertiesSourceFieldsKindName extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
}
final class NativeRelationRule extends Node {
    /** @return Presence<bool> */ public function either(): Presence { return $this->field('either'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<string> */ public function reads(): Presence { return $this->field('reads'); }
    /** @return Presence<bool> */ public function single(): Presence { return $this->field('single'); }
    /** @return Presence<string> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<string> */ public function target(): Presence { return $this->field('target'); }
}
final class NativeSessionAnnotation extends Node {
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<NativeAnnotationValueDecision|NativeAnnotationValueChoice|NativeAnnotationValueScore|NativeAnnotationValueTags|NativeAnnotationValueFailed> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionInputSource extends Node {
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<NativePhysicalSource> */ public function source(): Presence { return $this->field('source'); }
}
final class NativeSessionJudgmentChoice extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string|null> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionJudgmentDecision extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<bool|null> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionJudgmentScore extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|float> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionJudgmentTags extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<string>> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionNamedProbability extends Node {
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
}
final class NativeSessionObservationQuestion extends Node {
    /** @return Presence<NativeSessionQuestionDetail> */ public function detail(): Presence { return $this->field('detail'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string> */ public function member(): Presence { return $this->field('member'); }
    /** @return Presence<int|string> */ public function position(): Presence { return $this->field('position'); }
    /** @return Presence<string> */ public function stage(): Presence { return $this->field('stage'); }
}
final class NativeSessionObservationRow extends Node {
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<NativeSessionObservedRowJudgment|NativeSessionObservedRowAnnotated|NativeSessionObservedRowRecognized|NativeSessionObservedRowFind|NativeSessionObservedRowRelations> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionObservedRowAnnotated extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<NativeSessionAnnotation>> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionObservedRowFind extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|string|null> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionObservedRowJudgment extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<NativeSessionJudgmentDecision|NativeSessionJudgmentChoice|NativeSessionJudgmentScore|NativeSessionJudgmentTags> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionObservedRowRecognized extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<NativeSessionRecognition> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionObservedRowRelations extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<NativeSessionRelationEdge>> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionPacketAnnotateAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<NativeAnnotation>> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionPacketAnnotateRow extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<NativeAnnotation> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionPacketChooseAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<NativeAtomicNullableString>> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionPacketChooseRow extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<NativeAtomicNullableString> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionPacketDecideAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<NativeAtomicDecideValue>> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionPacketDecideRow extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<NativeAtomicDecideValue> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionPacketFilterAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<NativeAtomicBoolean>> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionPacketFilterRow extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<NativeAtomicBoolean> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionPacketFindAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<NativeFind> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionPacketObservation extends Node {
    /** @return Presence<mixed> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<NativeSessionObservationQuestion|NativeSessionObservationRow> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionPacketRankAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<NativeAtomicNonZeroUsize>> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionPacketRecognizeAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<NativeRecognition>> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionPacketRelateAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<NativeRelation> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionPacketScoreAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<NativeAtomicDouble>> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionPacketScoreRow extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<NativeAtomicDouble> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionPacketTagAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<NativeAtomicArrayOfString>> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionPacketTagRow extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<NativeAtomicArrayOfString> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionPacketTerminal extends Node {
    /** @return Presence<NativeFacts> */ public function facts(): Presence { return $this->field('facts'); }
    /** @return Presence<NativeCallError> */ public function failure(): Presence { return $this->field('failure'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
}
final class NativeSessionProbabilitiesNamed extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<NativeSessionNamedProbability>> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionProbabilitiesYesNo extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|float> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionQuestionDetail extends Node {
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<bool> */ public function cached(): Presence { return $this->field('cached'); }
    /** @return Presence<int|float> */ public function confidence(): Presence { return $this->field('confidence'); }
    /** @return Presence<int|string> */ public function failed_questions(): Presence { return $this->field('failed_questions'); }
    /** @return Presence<NativeFailure> */ public function failure(): Presence { return $this->field('failure'); }
    /** @return Presence<string> */ public function failure_id(): Presence { return $this->field('failure_id'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<NativePhysicalSource> */ public function input_source(): Presence { return $this->field('input_source'); }
    /** @return Presence<list<NativeSessionInputSource>> */ public function input_sources(): Presence { return $this->field('input_sources'); }
    /** @return Presence<list<mixed>> */ public function inputs(): Presence { return $this->field('inputs'); }
    /** @return Presence<string> */ public function model(): Presence { return $this->field('model'); }
    /** @return Presence<list<NativeObservationObservationId|NativeObservationFailureId>> */ public function observations(): Presence { return $this->field('observations'); }
    /** @return Presence<NativeSessionProbabilitiesYesNo|NativeSessionProbabilitiesNamed> */ public function probabilities(): Presence { return $this->field('probabilities'); }
    /** @return Presence<NativeReadableQuestionDecide|NativeReadableQuestionChoose|NativeReadableQuestionTag|NativeReadableQuestionScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function question_sha256(): Presence { return $this->field('question_sha256'); }
    /** @return Presence<list<NativeQuestionSource>> */ public function question_sources(): Presence { return $this->field('question_sources'); }
    /** @return Presence<string> */ public function raw_pick(): Presence { return $this->field('raw_pick'); }
    /** @return Presence<NativeUsage> */ public function reported_usage(): Presence { return $this->field('reported_usage'); }
    /** @return Presence<list<string>> */ public function requests(): Presence { return $this->field('requests'); }
    /** @return Presence<int|string> */ public function requests_sent(): Presence { return $this->field('requests_sent'); }
    /** @return Presence<int|float|string> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<string> */ public function url(): Presence { return $this->field('url'); }
    /** @return Presence<NativeTokenUsage> */ public function usage(): Presence { return $this->field('usage'); }
    /** @return Presence<mixed> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeSessionRecognition extends Node {
    /** @return Presence<list<NativeEntity>> */ public function entities(): Presence { return $this->field('entities'); }
    /** @return Presence<mixed> */ public function mode(): Presence { return $this->field('mode'); }
    /** @return Presence<list<NativeBoundaryProposal>> */ public function proposals(): Presence { return $this->field('proposals'); }
    /** @return Presence<list<NativeRecognitionEdgeDocument>> */ public function relations(): Presence { return $this->field('relations'); }
}
final class NativeSessionRelationEdge extends Node {
    /** @return Presence<bool> */ public function either(): Presence { return $this->field('either'); }
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
    /** @return Presence<string> */ public function relation(): Presence { return $this->field('relation'); }
    /** @return Presence<NativeEntityDocument> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<NativeEntityDocument> */ public function target(): Presence { return $this->field('target'); }
}
final class NativeSourceRelationEndpoint extends Node {
    /** @return Presence<string|null> */ public function file(): Presence { return $this->field('file'); }
    /** @return Presence<int|string> */ public function first_line(): Presence { return $this->field('first_line'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|string> */ public function last_line(): Presence { return $this->field('last_line'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<int|string> */ public function ordinal(): Presence { return $this->field('ordinal'); }
    /** @return Presence<mixed> */ public function record(): Presence { return $this->field('record'); }
}
final class NativeTokenUsage extends Node {
    /** @return Presence<int|string> */ public function input_tokens(): Presence { return $this->field('input_tokens'); }
    /** @return Presence<int|string> */ public function output_tokens(): Presence { return $this->field('output_tokens'); }
}
