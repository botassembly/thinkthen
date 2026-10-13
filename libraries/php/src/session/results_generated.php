<?php
// Generated from the shared Rust result graph; do not edit.
declare(strict_types=1);
namespace ThinkThen\Results;
trait Answers
{
    /** @return bool|null */ public function decideAnswer(): mixed
    {
        $answers = [];
        foreach ($this->packets as $packet) {
            if ($packet->kind === 'observation' && $packet->value->kind === 'row' && $packet->value->value->kind === 'judgment' && $packet->value->value->value->kind === 'decision') $answers[] = $packet->value->value->value->value;
        }
        if (count($answers) !== 1) throw new \ThinkThen\UsageFailure('expected one answer');
        return $answers[0];
    }
    /** @return string|null */ public function chooseAnswer(): mixed
    {
        $answers = [];
        foreach ($this->packets as $packet) {
            if ($packet->kind === 'observation' && $packet->value->kind === 'row' && $packet->value->value->kind === 'judgment' && $packet->value->value->value->kind === 'choice') $answers[] = $packet->value->value->value->value;
        }
        if (count($answers) !== 1) throw new \ThinkThen\UsageFailure('expected one answer');
        return $answers[0];
    }
    /** @return int|float */ public function scoreAnswer(): mixed
    {
        $answers = [];
        foreach ($this->packets as $packet) {
            if ($packet->kind === 'observation' && $packet->value->kind === 'row' && $packet->value->value->kind === 'judgment' && $packet->value->value->value->kind === 'score') $answers[] = $packet->value->value->value->value;
        }
        if (count($answers) !== 1) throw new \ThinkThen\UsageFailure('expected one answer');
        return $answers[0];
    }
    /** @return list<string> */ public function tagAnswer(): mixed
    {
        $answers = [];
        foreach ($this->packets as $packet) {
            if ($packet->kind === 'observation' && $packet->value->kind === 'row' && $packet->value->value->kind === 'judgment' && $packet->value->value->value->kind === 'tags') $answers[] = $packet->value->value->value->value;
        }
        if (count($answers) !== 1) throw new \ThinkThen\UsageFailure('expected one answer');
        return $answers[0];
    }
    /** @return list<NativeAtomicBoolean> */ public function filterAnswer(): mixed
    {
        $answer = [];
        foreach ($this->packets as $packet) {
            if ($packet instanceof NativeSessionPacketFilterAggregate) array_push($answer, ...$packet->value);
            elseif ($packet instanceof NativeSessionPacketFilterRow) $answer[] = $packet->value;
        }
        return array_values(array_map(fn($row) => $row->input, array_filter($answer, fn($row) => $row->value === true)));
    }
    /** @return list<NativeAtomicNonZeroUsize> */ public function rankAnswer(): mixed
    {
        $answer = [];
        foreach ($this->packets as $packet) {
            if ($packet instanceof NativeSessionPacketRankAggregate) array_push($answer, ...$packet->value);
        }
        $answer = array_values(array_filter($answer, fn($row) => $row->value !== null));
        usort($answer, fn($a, $b) => $a->value <=> $b->value);
        return array_map(fn($row) => $row->input, $answer);
    }
    /** @return NativeFind */ public function findAnswer(): mixed
    {
        $values = array_values(array_filter($this->packets, fn($p) => $p instanceof NativeSessionPacketFindAggregate));
        if (count($values) !== 1) throw new \ThinkThen\UsageFailure('expected one answer');
        $answer = $values[0]->value;
        return $answer;
    }
    /** @return list<NativeAnnotation> */ public function annotateAnswer(): mixed
    {
        $answer = [];
        foreach ($this->packets as $packet) {
            if ($packet instanceof NativeSessionPacketAnnotateAggregate) array_push($answer, ...$packet->value);
            elseif ($packet instanceof NativeSessionPacketAnnotateRow) $answer[] = $packet->value;
        }
        return $answer;
    }
    /** @return list<NativeRecognition> */ public function recognizeAnswer(): mixed
    {
        $answer = [];
        foreach ($this->packets as $packet) {
            if ($packet instanceof NativeSessionPacketRecognizeAggregate) array_push($answer, ...$packet->value);
        }
        return $answer;
    }
    /** @return NativeRelation */ public function relateAnswer(): mixed
    {
        $values = array_values(array_filter($this->packets, fn($p) => $p instanceof NativeSessionPacketRelateAggregate));
        if (count($values) !== 1) throw new \ThinkThen\UsageFailure('expected one answer');
        $answer = $values[0]->value;
        return $answer;
    }
}
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
final class NativeInputDeclarationObjectRoot extends Node {
    /** @return Presence<\stdClass> */ public function properties(): Presence { return $this->field('properties'); }
    /** @return Presence<list<string>> */ public function required(): Presence { return $this->field('required'); }
    /** @return Presence<string> */ public function type(): Presence { return $this->field('type'); }
}
final class NativeInputDeclarationStringRoot extends Node {
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
    /** @return Presence<NativeInputDeclarationStringRoot|NativeInputDeclarationObjectRoot> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<NativeInputDeclarationStringRoot|NativeInputDeclarationObjectRoot> */ public function item_schema(): Presence { return $this->field('item_schema'); }
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
    /** @return Presence<NativeInputDeclarationStringRoot|NativeInputDeclarationObjectRoot> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<mixed> */ public function entity_definition(): Presence { return $this->field('entity_definition'); }
    /** @return Presence<mixed> */ public function instructions(): Presence { return $this->field('instructions'); }
    /** @return Presence<NativeInputDeclarationStringRoot|NativeInputDeclarationObjectRoot> */ public function item_schema(): Presence { return $this->field('item_schema'); }
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
    /** @return Presence<NativeInputDeclarationStringRoot|NativeInputDeclarationObjectRoot> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<NativeRelateFields|null> */ public function fields(): Presence { return $this->field('fields'); }
    /** @return Presence<NativeInputDeclarationStringRoot|NativeInputDeclarationObjectRoot> */ public function item_schema(): Presence { return $this->field('item_schema'); }
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
    /** @return Presence<NativeInputDeclarationStringRoot|NativeInputDeclarationObjectRoot> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<NativeInputDeclarationStringRoot|NativeInputDeclarationObjectRoot> */ public function item_schema(): Presence { return $this->field('item_schema'); }
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
    /** @return Presence<NativeInputDeclarationStringRoot|NativeInputDeclarationObjectRoot> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<NativeInputDeclarationStringRoot|NativeInputDeclarationObjectRoot> */ public function item_schema(): Presence { return $this->field('item_schema'); }
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
    /** @return Presence<NativeInputDeclarationStringRoot|NativeInputDeclarationObjectRoot> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<NativeInputDeclarationStringRoot|NativeInputDeclarationObjectRoot> */ public function item_schema(): Presence { return $this->field('item_schema'); }
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
    /** @return Presence<NativeInputDeclarationStringRoot|NativeInputDeclarationObjectRoot> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<NativeInputDeclarationStringRoot|NativeInputDeclarationObjectRoot> */ public function item_schema(): Presence { return $this->field('item_schema'); }
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
    /** @return Presence<NativeRecognitionOddsRecognizeAnswer|NativeRecognitionOddsBoundaryOdds> */ public function answer(): Presence { return $this->field('answer'); }
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
    /** @return Presence<NativeRecognizeEntities|NativeRecognizeModeProposals> */ public function value(): Presence { return $this->field('value'); }
}
final class NativeRecognitionEdgeDocument extends Node {
    /** @return Presence<bool> */ public function either(): Presence { return $this->field('either'); }
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
    /** @return Presence<string> */ public function relation(): Presence { return $this->field('relation'); }
    /** @return Presence<NativeEntity> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<NativeEntity> */ public function target(): Presence { return $this->field('target'); }
}
final class NativeRecognitionOddsBoundaryOdds extends Node {
    /** @return Presence<list<NativePieceOdds>> */ public function pieces(): Presence { return $this->field('pieces'); }
    /** @return Presence<list<NativeBoundaryProposal>> */ public function proposals(): Presence { return $this->field('proposals'); }
}
final class NativeRecognitionOddsRecognizeAnswer extends Node {
    /** @return Presence<list<NativeNameOdds>> */ public function names(): Presence { return $this->field('names'); }
    /** @return Presence<list<NativePairOdds>> */ public function pairs(): Presence { return $this->field('pairs'); }
    /** @return Presence<list<NativePieceOdds>> */ public function pieces(): Presence { return $this->field('pieces'); }
    /** @return Presence<list<NativeRecognitionProposal>> */ public function proposals(): Presence { return $this->field('proposals'); }
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
final class NativeRelationEndpointRelatedEntity extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
}
final class NativeRelationEndpointSource extends Node {
    /** @return Presence<string|null> */ public function file(): Presence { return $this->field('file'); }
    /** @return Presence<int|string> */ public function first_line(): Presence { return $this->field('first_line'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|string> */ public function last_line(): Presence { return $this->field('last_line'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<int|string> */ public function ordinal(): Presence { return $this->field('ordinal'); }
    /** @return Presence<mixed> */ public function record(): Presence { return $this->field('record'); }
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
    public function sourceName(): string { return $this->source->name; }
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
    public function sourceName(): string { return $this->source->name; }
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
final class NativeRecognizeEntities extends Node {
    /** @return Presence<list<NativeEntity>> */ public function entities(): Presence { return $this->field('entities'); }
    /** @return Presence<list<NativeEntityEdge>> */ public function relations(): Presence { return $this->field('relations'); }
}
final class NativeRecognizeModeProposals extends Node {
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
    /** @return Presence<NativeRelationEndpointRelatedEntity|NativeRelationEndpointSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<NativeRelationEndpointRelatedEntity|NativeRelationEndpointSource> */ public function target(): Presence { return $this->field('target'); }
    public function sourceName(): string { return $this->source->name; }
    public function targetName(): string { return $this->target->name; }
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
    public function sourceName(): string { return $this->source->name; }
    public function targetName(): string { return $this->target->name; }
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
final class Decoder
{
    public static function native(\FFI\CData $view): Node
    {
        return self::readNativeSessionPacket($view[0]);
    }
    private static function text(\FFI\CData $view): string
    {
        return $view->len === 0 ? '' : \FFI::string($view->data, $view->len);
    }
    private static function unsigned(int $value): int|string
    {
        return $value < 0 ? sprintf('%u', $value) : $value;
    }
    private static function sequence(\FFI\CData $view, callable $read): array
    {
        $values = [];
        for ($i = 0; $i < $view->len; ++$i) $values[] = $read($view->data[$i]);
        return $values;
    }
    private static function mapping(\FFI\CData $view, callable $read): \stdClass
    {
        $values = new \stdClass();
        for ($i = 0; $i < $view->len; ++$i) {
            $entry = $view->data[$i];
            $values->{self::text($entry->name)} = $read($entry->value);
        }
        return $values;
    }
    private static function number(string $value): int|float|string
    {
        if (strpbrk($value, '.eE') !== false) return (float)$value;
        $integer = filter_var($value, FILTER_VALIDATE_INT);
        return $integer === false ? $value : $integer;
    }
    private static function jsonValue(\FFI\CData $value): mixed
    {
        return match ($value->kind) {
            1 => null,
            2 => (bool)$value->data->boolean,
            3 => self::number(self::text($value->data->number)),
            4 => self::text($value->data->string),
            5 => self::sequence($value->data->array, fn($v) => self::jsonValue($v[0])),
            6 => self::mapping($value->data->object, fn($v) => self::jsonValue($v[0])),
            default => throw new \LogicException('invalid native JSON value'),
        };
    }
    private static function readNativeAnnotation(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer_id'] = self::readNativeAnswerId($native->answer_id[0]);
        $values['answers'] = self::mapping($native->answers, fn($v) => self::readNativeAnnotationMember($v[0]));
        if ($native->file->presence !== 0) $values['file'] = $native->file->presence === 1 ? null : (self::text($native->file->value));
        if ($native->first_line->presence !== 0) $values['first_line'] = $native->first_line->presence === 1 ? null : (self::unsigned($native->first_line->value));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->last_line->presence !== 0) $values['last_line'] = $native->last_line->presence === 1 ? null : (self::unsigned($native->last_line->value));
        $values['meta'] = self::readNativeMeta($native->meta[0]);
        if ($native->position->presence !== 0) $values['position'] = $native->position->presence === 1 ? null : (self::readNativePosition($native->position->value[0]));
        $values['schema'] = self::readNativeVersion($native->schema[0]);
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readNativePhysicalSource($native->source->value[0]));
        $values['value'] = self::readNativeAnnotatedRow($native->value[0]);
        return new NativeAnnotation((object)$values, $values);
    }
    private static function readNativeAnnotationMember(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readNativeAnnotationMemberAnswerId($native->data->answer_id[0]),
            2 => self::readNativeAnnotationMemberFailureId($native->data->failure_id[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readNativeAnnotationMemberAnswerId(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readNativeAnswer($native->answer[0]);
        $values['answer_id'] = self::readNativeAnswerId($native->answer_id[0]);
        $values['observations'] = self::sequence($native->observations, fn($v) => self::readNativeObservation($v[0]));
        $values['question'] = self::readNativeReadableQuestion($native->question[0]);
        $values['question_sources'] = self::sequence($native->question_sources, fn($v) => self::readNativeQuestionSource($v[0]));
        $values['request'] = self::text($native->request);
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::readNativeThreshold($native->threshold->value[0]));
        if ($native->usage->presence !== 0) $values['usage'] = $native->usage->presence === 1 ? null : (self::readNativeUsage($native->usage->value[0]));
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : (self::readNativeValue($native->value->value[0]));
        return new NativeAnnotationMemberAnswerId((object)$values, $values);
    }
    private static function readNativeAnnotationMemberFailureId(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['failure'] = self::readNativeFailure($native->failure[0]);
        $values['failure_id'] = self::readNativeFailureId($native->failure_id[0]);
        $values['observations'] = self::sequence($native->observations, fn($v) => self::readNativeObservation($v[0]));
        $values['question'] = self::readNativeReadableQuestion($native->question[0]);
        $values['question_sources'] = self::sequence($native->question_sources, fn($v) => self::readNativeQuestionSource($v[0]));
        $values['request'] = self::text($native->request);
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::readNativeThreshold($native->threshold->value[0]));
        if ($native->usage->presence !== 0) $values['usage'] = $native->usage->presence === 1 ? null : (self::readNativeUsage($native->usage->value[0]));
        return new NativeAnnotationMemberFailureId((object)$values, $values);
    }
    private static function readNativeAnnotationValue(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readNativeAnnotationValueDecision($native->data->decision[0]),
            2 => self::readNativeAnnotationValueChoice($native->data->choice[0]),
            3 => self::readNativeAnnotationValueScore($native->data->score[0]),
            4 => self::readNativeAnnotationValueTags($native->data->tags[0]),
            5 => self::readNativeAnnotationValueFailed($native->data->failed[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readNativeAnnotationValueChoice(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : (self::text($native->value->value));
        return new NativeAnnotationValueChoice((object)$values, $values);
    }
    private static function readNativeAnnotationValueDecision(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : ((bool)($native->value->value));
        return new NativeAnnotationValueDecision((object)$values, $values);
    }
    private static function readNativeAnnotationValueFailed(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readNativeFailure($native->value[0]);
        return new NativeAnnotationValueFailed((object)$values, $values);
    }
    private static function readNativeAnnotationValueScore(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = $native->value;
        return new NativeAnnotationValueScore((object)$values, $values);
    }
    private static function readNativeAnnotationValueTags(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::text($v));
        return new NativeAnnotationValueTags((object)$values, $values);
    }
    private static function readNativeAnswerId(\FFI\CData $native): mixed
    {
        return self::text($native->value);
    }
    private static function readNativeAnswers(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['questions'] = self::sequence($native->questions, fn($v) => self::readNativeRelationMember($v[0]));
        return new NativeAnswers((object)$values, $values);
    }
    private static function readNativeAtomicArrayOfString(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readNativeAnswer($native->answer[0]);
        $values['answer_id'] = self::readNativeAnswerId($native->answer_id[0]);
        if ($native->images->presence !== 0) $values['images'] = $native->images->presence === 1 ? null : (self::sequence($native->images->value, fn($v) => self::readNativeImage($v[0])));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->members->presence !== 0) $values['members'] = $native->members->presence === 1 ? null : (self::sequence($native->members->value, fn($v) => self::readNativeRankMember($v[0])));
        $values['meta'] = self::readNativeMeta($native->meta[0]);
        $values['question'] = self::readNativeReadableQuestion($native->question[0]);
        if ($native->question_name->presence !== 0) $values['question_name'] = $native->question_name->presence === 1 ? null : (self::text($native->question_name->value));
        $values['schema'] = self::readNativeVersion($native->schema[0]);
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readNativePhysicalSource($native->source->value[0]));
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::readNativeThreshold($native->threshold->value[0]));
        $values['value'] = self::sequence($native->value, fn($v) => self::text($v));
        return new NativeAtomicArrayOfString((object)$values, $values);
    }
    private static function readNativeAtomicDecideValue(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readNativeAnswer($native->answer[0]);
        $values['answer_id'] = self::readNativeAnswerId($native->answer_id[0]);
        if ($native->images->presence !== 0) $values['images'] = $native->images->presence === 1 ? null : (self::sequence($native->images->value, fn($v) => self::readNativeImage($v[0])));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->members->presence !== 0) $values['members'] = $native->members->presence === 1 ? null : (self::sequence($native->members->value, fn($v) => self::readNativeRankMember($v[0])));
        $values['meta'] = self::readNativeMeta($native->meta[0]);
        $values['question'] = self::readNativeReadableQuestion($native->question[0]);
        if ($native->question_name->presence !== 0) $values['question_name'] = $native->question_name->presence === 1 ? null : (self::text($native->question_name->value));
        $values['schema'] = self::readNativeVersion($native->schema[0]);
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readNativePhysicalSource($native->source->value[0]));
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::readNativeThreshold($native->threshold->value[0]));
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : (self::readNativeDecideValue($native->value->value[0]));
        return new NativeAtomicDecideValue((object)$values, $values);
    }
    private static function readNativeAtomicNonZeroUsize(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readNativeAnswer($native->answer[0]);
        $values['answer_id'] = self::readNativeAnswerId($native->answer_id[0]);
        if ($native->images->presence !== 0) $values['images'] = $native->images->presence === 1 ? null : (self::sequence($native->images->value, fn($v) => self::readNativeImage($v[0])));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->members->presence !== 0) $values['members'] = $native->members->presence === 1 ? null : (self::sequence($native->members->value, fn($v) => self::readNativeRankMember($v[0])));
        $values['meta'] = self::readNativeMeta($native->meta[0]);
        $values['question'] = self::readNativeReadableQuestion($native->question[0]);
        if ($native->question_name->presence !== 0) $values['question_name'] = $native->question_name->presence === 1 ? null : (self::text($native->question_name->value));
        $values['schema'] = self::readNativeVersion($native->schema[0]);
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readNativePhysicalSource($native->source->value[0]));
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::readNativeThreshold($native->threshold->value[0]));
        $values['value'] = self::unsigned($native->value);
        return new NativeAtomicNonZeroUsize((object)$values, $values);
    }
    private static function readNativeAtomicNullableString(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readNativeAnswer($native->answer[0]);
        $values['answer_id'] = self::readNativeAnswerId($native->answer_id[0]);
        if ($native->images->presence !== 0) $values['images'] = $native->images->presence === 1 ? null : (self::sequence($native->images->value, fn($v) => self::readNativeImage($v[0])));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->members->presence !== 0) $values['members'] = $native->members->presence === 1 ? null : (self::sequence($native->members->value, fn($v) => self::readNativeRankMember($v[0])));
        $values['meta'] = self::readNativeMeta($native->meta[0]);
        $values['question'] = self::readNativeReadableQuestion($native->question[0]);
        if ($native->question_name->presence !== 0) $values['question_name'] = $native->question_name->presence === 1 ? null : (self::text($native->question_name->value));
        $values['schema'] = self::readNativeVersion($native->schema[0]);
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readNativePhysicalSource($native->source->value[0]));
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::readNativeThreshold($native->threshold->value[0]));
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : (self::text($native->value->value));
        return new NativeAtomicNullableString((object)$values, $values);
    }
    private static function readNativeAtomicBoolean(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readNativeAnswer($native->answer[0]);
        $values['answer_id'] = self::readNativeAnswerId($native->answer_id[0]);
        if ($native->images->presence !== 0) $values['images'] = $native->images->presence === 1 ? null : (self::sequence($native->images->value, fn($v) => self::readNativeImage($v[0])));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->members->presence !== 0) $values['members'] = $native->members->presence === 1 ? null : (self::sequence($native->members->value, fn($v) => self::readNativeRankMember($v[0])));
        $values['meta'] = self::readNativeMeta($native->meta[0]);
        $values['question'] = self::readNativeReadableQuestion($native->question[0]);
        if ($native->question_name->presence !== 0) $values['question_name'] = $native->question_name->presence === 1 ? null : (self::text($native->question_name->value));
        $values['schema'] = self::readNativeVersion($native->schema[0]);
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readNativePhysicalSource($native->source->value[0]));
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::readNativeThreshold($native->threshold->value[0]));
        $values['value'] = (bool)($native->value);
        return new NativeAtomicBoolean((object)$values, $values);
    }
    private static function readNativeAtomicDouble(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readNativeAnswer($native->answer[0]);
        $values['answer_id'] = self::readNativeAnswerId($native->answer_id[0]);
        if ($native->images->presence !== 0) $values['images'] = $native->images->presence === 1 ? null : (self::sequence($native->images->value, fn($v) => self::readNativeImage($v[0])));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->members->presence !== 0) $values['members'] = $native->members->presence === 1 ? null : (self::sequence($native->members->value, fn($v) => self::readNativeRankMember($v[0])));
        $values['meta'] = self::readNativeMeta($native->meta[0]);
        $values['question'] = self::readNativeReadableQuestion($native->question[0]);
        if ($native->question_name->presence !== 0) $values['question_name'] = $native->question_name->presence === 1 ? null : (self::text($native->question_name->value));
        $values['schema'] = self::readNativeVersion($native->schema[0]);
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readNativePhysicalSource($native->source->value[0]));
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::readNativeThreshold($native->threshold->value[0]));
        $values['value'] = $native->value;
        return new NativeAtomicDouble((object)$values, $values);
    }
    private static function readNativeAttempt(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['ordinal'] = self::unsigned($native->ordinal);
        $values['outcome'] = self::readNativeAttemptOutcome($native->outcome[0]);
        if ($native->request_id->presence !== 0) $values['request_id'] = $native->request_id->presence === 1 ? null : (self::text($native->request_id->value));
        $values['request_sha256'] = self::text($native->request_sha256);
        $values['sdk_request_id'] = self::readNativeSdkRequestId($native->sdk_request_id[0]);
        if ($native->server_ms->presence !== 0) $values['server_ms'] = $native->server_ms->presence === 1 ? null : (self::unsigned($native->server_ms->value));
        if ($native->status->presence !== 0) $values['status'] = $native->status->presence === 1 ? null : (self::unsigned($native->status->value));
        $values['wall_ms'] = self::unsigned($native->wall_ms);
        return new NativeAttempt((object)$values, $values);
    }
    private static function readNativeBatch(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::unsigned($native->data->integer),
            2 => self::readNativeBatchString($native->data->string[0]),
            default => throw new \LogicException('invalid native value'),
        };
    }
    private static function readNativeBoundaryMode(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'boundary_only',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readNativeBoundaryOdds(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['pieces'] = self::sequence($native->pieces, fn($v) => self::readNativePieceOdds($v[0]));
        $values['proposals'] = self::sequence($native->proposals, fn($v) => self::readNativeBoundaryProposal($v[0]));
        return new NativeBoundaryOdds((object)$values, $values);
    }
    private static function readNativeBoundaryProposal(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['end'] = self::unsigned($native->end);
        $values['length'] = self::unsigned($native->length);
        $values['probability'] = $native->probability;
        $values['start'] = self::unsigned($native->start);
        $values['text'] = self::text($native->text);
        return new NativeBoundaryProposal((object)$values, $values);
    }
    private static function readNativeCallError(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['error'] = self::readNativeError($native->error[0]);
        if ($native->facts->presence !== 0) $values['facts'] = $native->facts->presence === 1 ? null : (self::readNativeFacts($native->facts->value[0]));
        return new NativeCallError((object)$values, $values);
    }
    private static function readNativeCallId(\FFI\CData $native): mixed
    {
        return self::text($native->value);
    }
    private static function readNativeDecideValue(\FFI\CData $native): mixed
    {
        return self::jsonValue($native->value[0]);
    }
    private static function readNativeEntityDocument(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['name'] = self::text($native->name);
        return new NativeEntityDocument((object)$values, $values);
    }
    private static function readNativeError(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->estimated_input_denial->presence !== 0) $values['estimated_input_denial'] = $native->estimated_input_denial->presence === 1 ? null : (self::readNativeEstimatedInputDenial($native->estimated_input_denial->value[0]));
        $values['kind'] = self::readNativeFailureKind($native->kind[0]);
        $values['message'] = self::text($native->message);
        $values['retryable'] = (bool)($native->retryable);
        if ($native->send_budget_denial->presence !== 0) $values['send_budget_denial'] = $native->send_budget_denial->presence === 1 ? null : (self::readNativeSendBudgetDenial($native->send_budget_denial->value[0]));
        $values['stopped'] = self::readNativeStopped($native->stopped[0]);
        return new NativeError((object)$values, $values);
    }
    private static function readNativeEstimatedInputDenial(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readNativeEstimatedInputDenialInitialRequest($native->data->initial_request[0]),
            2 => self::readNativeEstimatedInputDenialAdditionalRequest($native->data->additional_request[0]),
            3 => self::readNativeEstimatedInputDenialRetry($native->data->retry[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readNativeEstimatedInputDenialAdditionalRequest(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['limit'] = self::unsigned($native->limit);
        return new NativeEstimatedInputDenialAdditionalRequest((object)$values, $values);
    }
    private static function readNativeEstimatedInputDenialInitialRequest(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['limit'] = self::unsigned($native->limit);
        return new NativeEstimatedInputDenialInitialRequest((object)$values, $values);
    }
    private static function readNativeEstimatedInputDenialRetry(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['last_status'] = self::unsigned($native->last_status);
        $values['limit'] = self::unsigned($native->limit);
        return new NativeEstimatedInputDenialRetry((object)$values, $values);
    }
    private static function readNativeFacts(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->attempts->presence !== 0) $values['attempts'] = $native->attempts->presence === 1 ? null : (self::sequence($native->attempts->value, fn($v) => self::readNativeAttempt($v[0])));
        $values['cache_answers'] = self::unsigned($native->cache_answers);
        $values['call_id'] = self::readNativeCallId($native->call_id[0]);
        if ($native->estimated_cost_usd->presence !== 0) $values['estimated_cost_usd'] = $native->estimated_cost_usd->presence === 1 ? null : (self::text($native->estimated_cost_usd->value));
        if ($native->held_model_mismatch->presence !== 0) $values['held_model_mismatch'] = $native->held_model_mismatch->presence === 1 ? null : ((bool)($native->held_model_mismatch->value));
        if ($native->input_tokens->presence !== 0) $values['input_tokens'] = $native->input_tokens->presence === 1 ? null : (self::unsigned($native->input_tokens->value));
        $values['largest_request_bytes'] = self::unsigned($native->largest_request_bytes);
        if ($native->largest_request_estimated_input_tokens->presence !== 0) $values['largest_request_estimated_input_tokens'] = $native->largest_request_estimated_input_tokens->presence === 1 ? null : (self::unsigned($native->largest_request_estimated_input_tokens->value));
        if ($native->model->presence !== 0) $values['model'] = $native->model->presence === 1 ? null : (self::text($native->model->value));
        if ($native->output_tokens->presence !== 0) $values['output_tokens'] = $native->output_tokens->presence === 1 ? null : (self::unsigned($native->output_tokens->value));
        $values['records'] = self::unsigned($native->records);
        $values['requests_sent'] = self::unsigned($native->requests_sent);
        $values['seconds'] = $native->seconds;
        $values['token_estimate_method'] = self::text($native->token_estimate_method);
        if ($native->usage_persistence->presence !== 0) $values['usage_persistence'] = $native->usage_persistence->presence === 1 ? null : (self::readNativePersistenceObservation($native->usage_persistence->value[0]));
        return new NativeFacts((object)$values, $values);
    }
    private static function readNativeFailureId(\FFI\CData $native): mixed
    {
        return self::text($native->value);
    }
    private static function readNativeFind(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readNativeFindAnswer($native->answer[0]);
        $values['answer_id'] = self::readNativeAnswerId($native->answer_id[0]);
        if ($native->candidates->presence !== 0) $values['candidates'] = $native->candidates->presence === 1 ? null : (self::sequence($native->candidates->value, fn($v) => self::readNativeFindCandidate($v[0])));
        if ($native->file->presence !== 0) $values['file'] = $native->file->presence === 1 ? null : (self::text($native->file->value));
        if ($native->first_line->presence !== 0) $values['first_line'] = $native->first_line->presence === 1 ? null : (self::unsigned($native->first_line->value));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->last_line->presence !== 0) $values['last_line'] = $native->last_line->presence === 1 ? null : (self::unsigned($native->last_line->value));
        $values['meta'] = self::readNativeMeta($native->meta[0]);
        if ($native->position->presence !== 0) $values['position'] = $native->position->presence === 1 ? null : (self::readNativePosition($native->position->value[0]));
        $values['question'] = self::readNativeReadableQuestion2($native->question[0]);
        $values['schema'] = self::readNativeVersion($native->schema[0]);
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::jsonValue($native->threshold->value[0]));
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : (self::jsonValue($native->value->value[0]));
        return new NativeFind((object)$values, $values);
    }
    private static function readNativeFindCandidate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        $values['probability'] = $native->probability;
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readNativePhysicalSource($native->source->value[0]));
        return new NativeFindCandidate((object)$values, $values);
    }
    private static function readNativeImage(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['base64'] = self::text($native->base64);
        $values['height'] = self::unsigned($native->height);
        $values['media'] = self::readNativeImageMedia($native->media[0]);
        $values['width'] = self::unsigned($native->width);
        return new NativeImage((object)$values, $values);
    }
    private static function readNativeImageMedia(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'image/jpeg',
            2 => 'image/png',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readNativeInputDeclaration(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readNativeInputDeclarationStringRoot($native->data->string_root[0]),
            2 => self::readNativeInputDeclarationObjectRoot($native->data->object_root[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readNativeInputDeclarationObjectRoot(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['properties'] = self::mapping($native->properties, fn($v) => self::readNativeInputPropertyType($v[0]));
        if ($native->required->presence !== 0) $values['required'] = $native->required->presence === 1 ? null : (self::sequence($native->required->value, fn($v) => self::text($v)));
        $values['type'] = self::readNativeObjectType($native->type[0]);
        return new NativeInputDeclarationObjectRoot((object)$values, $values);
    }
    private static function readNativeInputDeclarationStringRoot(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['type'] = self::readNativeStringType($native->type[0]);
        return new NativeInputDeclarationStringRoot((object)$values, $values);
    }
    private static function readNativeInputPropertyType(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readNativeInputPropertyTypeString($native->data->string[0]),
            2 => self::readNativeInputPropertyTypeNumber($native->data->number[0]),
            3 => self::readNativeInputPropertyTypeBoolean($native->data->boolean[0]),
            4 => self::readNativeInputPropertyTypeArray($native->data->array[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readNativeInputPropertyTypeArray(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['items'] = self::readNativeStringRoot($native->items[0]);
        $values['type'] = self::text($native->type);
        return new NativeInputPropertyTypeArray((object)$values, $values);
    }
    private static function readNativeInputPropertyTypeBoolean(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['type'] = self::text($native->type);
        return new NativeInputPropertyTypeBoolean((object)$values, $values);
    }
    private static function readNativeInputPropertyTypeNumber(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['type'] = self::text($native->type);
        return new NativeInputPropertyTypeNumber((object)$values, $values);
    }
    private static function readNativeInputPropertyTypeString(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['type'] = self::text($native->type);
        return new NativeInputPropertyTypeString((object)$values, $values);
    }
    private static function readNativeLabel(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->description->presence !== 0) $values['description'] = $native->description->presence === 1 ? null : (self::jsonValue($native->description->value[0]));
        $values['name'] = self::text($native->name);
        return new NativeLabel((object)$values, $values);
    }
    private static function readNativeMeta(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->answered_by->presence !== 0) $values['answered_by'] = $native->answered_by->presence === 1 ? null : (self::text($native->answered_by->value));
        if ($native->attempts->presence !== 0) $values['attempts'] = $native->attempts->presence === 1 ? null : (self::sequence($native->attempts->value, fn($v) => self::readNativeAttempt($v[0])));
        if ($native->batch_setting->presence !== 0) $values['batch_setting'] = $native->batch_setting->presence === 1 ? null : (self::readNativeBatchSetting($native->batch_setting->value[0]));
        if ($native->batch_warning->presence !== 0) $values['batch_warning'] = $native->batch_warning->presence === 1 ? null : (self::readNativeBatchWarning($native->batch_warning->value[0]));
        $values['cached'] = (bool)($native->cached);
        if ($native->context_sha256->presence !== 0) $values['context_sha256'] = $native->context_sha256->presence === 1 ? null : (self::text($native->context_sha256->value));
        $values['failed_questions'] = self::unsigned($native->failed_questions);
        $values['model'] = self::text($native->model);
        $values['observations'] = self::sequence($native->observations, fn($v) => self::readNativeObservation($v[0]));
        if ($native->origin->presence !== 0) $values['origin'] = $native->origin->presence === 1 ? null : (self::readNativeOrigin($native->origin->value[0]));
        if ($native->profile_warning->presence !== 0) $values['profile_warning'] = $native->profile_warning->presence === 1 ? null : (self::readNativeProfileWarning($native->profile_warning->value[0]));
        if ($native->question_sha256->presence !== 0) $values['question_sha256'] = $native->question_sha256->presence === 1 ? null : (self::text($native->question_sha256->value));
        $values['question_sources'] = self::sequence($native->question_sources, fn($v) => self::readNativeQuestionSource($v[0]));
        if ($native->questions_sha256->presence !== 0) $values['questions_sha256'] = $native->questions_sha256->presence === 1 ? null : (self::text($native->questions_sha256->value));
        $values['requests'] = self::sequence($native->requests, fn($v) => self::text($v));
        $values['requests_sent'] = self::unsigned($native->requests_sent);
        $values['tool'] = self::text($native->tool);
        $values['url'] = self::text($native->url);
        if ($native->usage->presence !== 0) $values['usage'] = $native->usage->presence === 1 ? null : (self::readNativeUsage($native->usage->value[0]));
        return new NativeMeta((object)$values, $values);
    }
    private static function readNativeObjectRoot(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['properties'] = self::mapping($native->properties, fn($v) => self::readNativeInputPropertyType($v[0]));
        if ($native->required->presence !== 0) $values['required'] = $native->required->presence === 1 ? null : (self::sequence($native->required->value, fn($v) => self::text($v)));
        $values['type'] = self::readNativeObjectType($native->type[0]);
        return new NativeObjectRoot((object)$values, $values);
    }
    private static function readNativeObjectType(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'object',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readNativeObservation(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readNativeObservationObservationId($native->data->observation_id[0]),
            2 => self::readNativeObservationFailureId($native->data->failure_id[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readNativeObservationId(\FFI\CData $native): mixed
    {
        return self::text($native->value);
    }
    private static function readNativeObservationFailureId(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['failure_id'] = self::readNativeFailureId($native->failure_id[0]);
        return new NativeObservationFailureId((object)$values, $values);
    }
    private static function readNativeObservationObservationId(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['observation_id'] = self::readNativeObservationId($native->observation_id[0]);
        return new NativeObservationObservationId((object)$values, $values);
    }
    private static function readNativeOrigin(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'live',
            2 => 'cache',
            3 => 'replay',
            4 => 'proxy',
            5 => 'memory',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readNativePersistenceObservation(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->advice->presence !== 0) $values['advice'] = $native->advice->presence === 1 ? null : (self::text($native->advice->value));
        $values['observed_at'] = self::text($native->observed_at);
        $values['state'] = self::readNativeUsagePersistence($native->state[0]);
        return new NativePersistenceObservation((object)$values, $values);
    }
    private static function readNativePhysicalSource(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['file'] = self::text($native->file);
        if ($native->first_line->presence !== 0) $values['first_line'] = $native->first_line->presence === 1 ? null : (self::unsigned($native->first_line->value));
        if ($native->last_line->presence !== 0) $values['last_line'] = $native->last_line->presence === 1 ? null : (self::unsigned($native->last_line->value));
        return new NativePhysicalSource((object)$values, $values);
    }
    private static function readNativePosition(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->file->presence !== 0) $values['file'] = $native->file->presence === 1 ? null : (self::text($native->file->value));
        if ($native->first->presence !== 0) $values['first'] = $native->first->presence === 1 ? null : (self::unsigned($native->first->value));
        if ($native->images->presence !== 0) $values['images'] = $native->images->presence === 1 ? null : (self::sequence($native->images->value, fn($v) => self::text($v)));
        if ($native->last->presence !== 0) $values['last'] = $native->last->presence === 1 ? null : (self::unsigned($native->last->value));
        return new NativePosition((object)$values, $values);
    }
    private static function readNativeQuestionName(\FFI\CData $native): mixed
    {
        return self::text($native->value);
    }
    private static function readNativeQuestionSource(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answered_by'] = self::text($native->answered_by);
        if ($native->batch_size->presence !== 0) $values['batch_size'] = $native->batch_size->presence === 1 ? null : (self::unsigned($native->batch_size->value));
        $values['origin'] = self::readNativeOrigin($native->origin[0]);
        return new NativeQuestionSource((object)$values, $values);
    }
    private static function readNativeRankMember(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['name'] = self::text($native->name);
        $values['result'] = self::readNativeRankMemberResult($native->result[0]);
        return new NativeRankMember((object)$values, $values);
    }
    private static function readNativeRankMemberResult(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readNativeAnswer($native->answer[0]);
        $values['answer_id'] = self::readNativeAnswerId($native->answer_id[0]);
        if ($native->images->presence !== 0) $values['images'] = $native->images->presence === 1 ? null : (self::sequence($native->images->value, fn($v) => self::readNativeImage($v[0])));
        $values['meta'] = self::readNativeMeta($native->meta[0]);
        $values['question'] = self::readNativeReadableQuestion($native->question[0]);
        $values['schema'] = self::readNativeVersion($native->schema[0]);
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readNativePhysicalSource($native->source->value[0]));
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::jsonValue($native->threshold->value[0]));
        $values['value'] = self::unsigned($native->value);
        return new NativeRankMemberResult((object)$values, $values);
    }
    private static function readNativeReadableQuestion(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readNativeReadableQuestionDecide($native->data->decide[0]),
            2 => self::readNativeReadableQuestionChoose($native->data->choose[0]),
            3 => self::readNativeReadableQuestionTag($native->data->tag[0]),
            4 => self::readNativeReadableQuestionScore($native->data->score[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readNativeReadableQuestion2(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->batch->presence !== 0) $values['batch'] = $native->batch->presence === 1 ? null : (self::readNativeBatch($native->batch->value[0]));
        if ($native->context_schema->presence !== 0) $values['context_schema'] = $native->context_schema->presence === 1 ? null : (self::readNativeInputDeclaration($native->context_schema->value[0]));
        if ($native->item_schema->presence !== 0) $values['item_schema'] = $native->item_schema->presence === 1 ? null : (self::readNativeInputDeclaration($native->item_schema->value[0]));
        if ($native->label_details->presence !== 0) $values['label_details'] = $native->label_details->presence === 1 ? null : (self::sequence($native->label_details->value, fn($v) => self::readNativeLabel($v[0])));
        if ($native->model->presence !== 0) $values['model'] = $native->model->presence === 1 ? null : (self::text($native->model->value));
        if ($native->name->presence !== 0) $values['name'] = $native->name->presence === 1 ? null : (self::readNativeQuestionName($native->name->value[0]));
        $values['none'] = (bool)($native->none);
        if ($native->on->presence !== 0) $values['on'] = $native->on->presence === 1 ? null : (self::sequence($native->on->value, fn($v) => self::text($v)));
        if ($native->profile->presence !== 0) $values['profile'] = $native->profile->presence === 1 ? null : (self::text($native->profile->value));
        if ($native->text->presence !== 0) $values['text'] = $native->text->presence === 1 ? null : (self::jsonValue($native->text->value[0]));
        $values['verb'] = self::text($native->verb);
        if ($native->wording_version->presence !== 0) $values['wording_version'] = $native->wording_version->presence === 1 ? null : (self::readNativeWordingVersion($native->wording_version->value[0]));
        return new NativeReadableQuestion2((object)$values, $values);
    }
    private static function readNativeReadableQuestion3(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->batch->presence !== 0) $values['batch'] = $native->batch->presence === 1 ? null : (self::readNativeBatch($native->batch->value[0]));
        if ($native->context_schema->presence !== 0) $values['context_schema'] = $native->context_schema->presence === 1 ? null : (self::readNativeInputDeclaration($native->context_schema->value[0]));
        if ($native->entity_definition->presence !== 0) $values['entity_definition'] = $native->entity_definition->presence === 1 ? null : (self::jsonValue($native->entity_definition->value[0]));
        if ($native->instructions->presence !== 0) $values['instructions'] = $native->instructions->presence === 1 ? null : (self::jsonValue($native->instructions->value[0]));
        if ($native->item_schema->presence !== 0) $values['item_schema'] = $native->item_schema->presence === 1 ? null : (self::readNativeInputDeclaration($native->item_schema->value[0]));
        $values['kinds'] = self::mapping($native->kinds, fn($v) => self::jsonValue($v[0]));
        if ($native->label_details->presence !== 0) $values['label_details'] = $native->label_details->presence === 1 ? null : (self::sequence($native->label_details->value, fn($v) => self::readNativeLabel($v[0])));
        if ($native->mode->presence !== 0) $values['mode'] = $native->mode->presence === 1 ? null : (self::readNativeRecognitionMode($native->mode->value[0]));
        if ($native->model->presence !== 0) $values['model'] = $native->model->presence === 1 ? null : (self::text($native->model->value));
        if ($native->name->presence !== 0) $values['name'] = $native->name->presence === 1 ? null : (self::readNativeQuestionName($native->name->value[0]));
        if ($native->on->presence !== 0) $values['on'] = $native->on->presence === 1 ? null : (self::sequence($native->on->value, fn($v) => self::text($v)));
        if ($native->profile->presence !== 0) $values['profile'] = $native->profile->presence === 1 ? null : (self::text($native->profile->value));
        if ($native->relation_threshold->presence !== 0) $values['relation_threshold'] = $native->relation_threshold->presence === 1 ? null : (self::readNativeThreshold($native->relation_threshold->value[0]));
        if ($native->relations->presence !== 0) $values['relations'] = $native->relations->presence === 1 ? null : (self::sequence($native->relations->value, fn($v) => self::readNativeRelationRule($v[0])));
        if ($native->snippet_pieces->presence !== 0) $values['snippet_pieces'] = $native->snippet_pieces->presence === 1 ? null : (self::unsigned($native->snippet_pieces->value));
        if ($native->stage_context->presence !== 0) $values['stage_context'] = $native->stage_context->presence === 1 ? null : (self::readNativeRecognitionStageContext($native->stage_context->value[0]));
        $values['threshold'] = self::readNativeThreshold($native->threshold[0]);
        $values['verb'] = self::readNativeVerb($native->verb[0]);
        if ($native->wording_version->presence !== 0) $values['wording_version'] = $native->wording_version->presence === 1 ? null : (self::readNativeWordingVersion($native->wording_version->value[0]));
        return new NativeReadableQuestion3((object)$values, $values);
    }
    private static function readNativeReadableQuestion4(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->batch->presence !== 0) $values['batch'] = $native->batch->presence === 1 ? null : (self::readNativeBatch($native->batch->value[0]));
        if ($native->context_schema->presence !== 0) $values['context_schema'] = $native->context_schema->presence === 1 ? null : (self::readNativeInputDeclaration($native->context_schema->value[0]));
        if ($native->fields->presence !== 0) $values['fields'] = $native->fields->presence === 1 ? null : (self::readNativeRelateFields($native->fields->value[0]));
        if ($native->item_schema->presence !== 0) $values['item_schema'] = $native->item_schema->presence === 1 ? null : (self::readNativeInputDeclaration($native->item_schema->value[0]));
        if ($native->label_details->presence !== 0) $values['label_details'] = $native->label_details->presence === 1 ? null : (self::sequence($native->label_details->value, fn($v) => self::readNativeLabel($v[0])));
        if ($native->model->presence !== 0) $values['model'] = $native->model->presence === 1 ? null : (self::text($native->model->value));
        if ($native->name->presence !== 0) $values['name'] = $native->name->presence === 1 ? null : (self::readNativeQuestionName($native->name->value[0]));
        if ($native->on->presence !== 0) $values['on'] = $native->on->presence === 1 ? null : (self::sequence($native->on->value, fn($v) => self::text($v)));
        if ($native->profile->presence !== 0) $values['profile'] = $native->profile->presence === 1 ? null : (self::text($native->profile->value));
        $values['relations'] = self::sequence($native->relations, fn($v) => self::readNativeRelationRule($v[0]));
        $values['threshold'] = self::readNativeThreshold($native->threshold[0]);
        $values['verb'] = self::text($native->verb);
        if ($native->wording_version->presence !== 0) $values['wording_version'] = $native->wording_version->presence === 1 ? null : (self::readNativeWordingVersion($native->wording_version->value[0]));
        return new NativeReadableQuestion4((object)$values, $values);
    }
    private static function readNativeReadableQuestionChoose(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->batch->presence !== 0) $values['batch'] = $native->batch->presence === 1 ? null : (self::readNativeBatch($native->batch->value[0]));
        if ($native->context_schema->presence !== 0) $values['context_schema'] = $native->context_schema->presence === 1 ? null : (self::readNativeInputDeclaration($native->context_schema->value[0]));
        if ($native->item_schema->presence !== 0) $values['item_schema'] = $native->item_schema->presence === 1 ? null : (self::readNativeInputDeclaration($native->item_schema->value[0]));
        if ($native->label_details->presence !== 0) $values['label_details'] = $native->label_details->presence === 1 ? null : (self::sequence($native->label_details->value, fn($v) => self::readNativeLabel($v[0])));
        if ($native->model->presence !== 0) $values['model'] = $native->model->presence === 1 ? null : (self::text($native->model->value));
        if ($native->name->presence !== 0) $values['name'] = $native->name->presence === 1 ? null : (self::readNativeQuestionName($native->name->value[0]));
        if ($native->on->presence !== 0) $values['on'] = $native->on->presence === 1 ? null : (self::sequence($native->on->value, fn($v) => self::text($v)));
        if ($native->profile->presence !== 0) $values['profile'] = $native->profile->presence === 1 ? null : (self::text($native->profile->value));
        if ($native->wording_version->presence !== 0) $values['wording_version'] = $native->wording_version->presence === 1 ? null : (self::readNativeWordingVersion($native->wording_version->value[0]));
        $values['options'] = self::sequence($native->options, fn($v) => self::text($v));
        if ($native->text->presence !== 0) $values['text'] = $native->text->presence === 1 ? null : (self::jsonValue($native->text->value[0]));
        $values['verb'] = self::text($native->verb);
        return new NativeReadableQuestionChoose((object)$values, $values);
    }
    private static function readNativeReadableQuestionDecide(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->batch->presence !== 0) $values['batch'] = $native->batch->presence === 1 ? null : (self::readNativeBatch($native->batch->value[0]));
        if ($native->context_schema->presence !== 0) $values['context_schema'] = $native->context_schema->presence === 1 ? null : (self::readNativeInputDeclaration($native->context_schema->value[0]));
        if ($native->item_schema->presence !== 0) $values['item_schema'] = $native->item_schema->presence === 1 ? null : (self::readNativeInputDeclaration($native->item_schema->value[0]));
        if ($native->label_details->presence !== 0) $values['label_details'] = $native->label_details->presence === 1 ? null : (self::sequence($native->label_details->value, fn($v) => self::readNativeLabel($v[0])));
        if ($native->model->presence !== 0) $values['model'] = $native->model->presence === 1 ? null : (self::text($native->model->value));
        if ($native->name->presence !== 0) $values['name'] = $native->name->presence === 1 ? null : (self::readNativeQuestionName($native->name->value[0]));
        if ($native->on->presence !== 0) $values['on'] = $native->on->presence === 1 ? null : (self::sequence($native->on->value, fn($v) => self::text($v)));
        if ($native->profile->presence !== 0) $values['profile'] = $native->profile->presence === 1 ? null : (self::text($native->profile->value));
        if ($native->wording_version->presence !== 0) $values['wording_version'] = $native->wording_version->presence === 1 ? null : (self::readNativeWordingVersion($native->wording_version->value[0]));
        if ($native->false_->presence !== 0) $values['false'] = $native->false_->presence === 1 ? null : (self::jsonValue($native->false_->value[0]));
        if ($native->text->presence !== 0) $values['text'] = $native->text->presence === 1 ? null : (self::jsonValue($native->text->value[0]));
        if ($native->true_->presence !== 0) $values['true'] = $native->true_->presence === 1 ? null : (self::jsonValue($native->true_->value[0]));
        $values['verb'] = self::text($native->verb);
        return new NativeReadableQuestionDecide((object)$values, $values);
    }
    private static function readNativeReadableQuestionScore(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->batch->presence !== 0) $values['batch'] = $native->batch->presence === 1 ? null : (self::readNativeBatch($native->batch->value[0]));
        if ($native->context_schema->presence !== 0) $values['context_schema'] = $native->context_schema->presence === 1 ? null : (self::readNativeInputDeclaration($native->context_schema->value[0]));
        if ($native->item_schema->presence !== 0) $values['item_schema'] = $native->item_schema->presence === 1 ? null : (self::readNativeInputDeclaration($native->item_schema->value[0]));
        if ($native->label_details->presence !== 0) $values['label_details'] = $native->label_details->presence === 1 ? null : (self::sequence($native->label_details->value, fn($v) => self::readNativeLabel($v[0])));
        if ($native->model->presence !== 0) $values['model'] = $native->model->presence === 1 ? null : (self::text($native->model->value));
        if ($native->name->presence !== 0) $values['name'] = $native->name->presence === 1 ? null : (self::readNativeQuestionName($native->name->value[0]));
        if ($native->on->presence !== 0) $values['on'] = $native->on->presence === 1 ? null : (self::sequence($native->on->value, fn($v) => self::text($v)));
        if ($native->profile->presence !== 0) $values['profile'] = $native->profile->presence === 1 ? null : (self::text($native->profile->value));
        if ($native->wording_version->presence !== 0) $values['wording_version'] = $native->wording_version->presence === 1 ? null : (self::readNativeWordingVersion($native->wording_version->value[0]));
        $values['levels'] = self::sequence($native->levels, fn($v) => self::text($v));
        if ($native->text->presence !== 0) $values['text'] = $native->text->presence === 1 ? null : (self::jsonValue($native->text->value[0]));
        $values['verb'] = self::text($native->verb);
        return new NativeReadableQuestionScore((object)$values, $values);
    }
    private static function readNativeReadableQuestionTag(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->batch->presence !== 0) $values['batch'] = $native->batch->presence === 1 ? null : (self::readNativeBatch($native->batch->value[0]));
        if ($native->context_schema->presence !== 0) $values['context_schema'] = $native->context_schema->presence === 1 ? null : (self::readNativeInputDeclaration($native->context_schema->value[0]));
        if ($native->item_schema->presence !== 0) $values['item_schema'] = $native->item_schema->presence === 1 ? null : (self::readNativeInputDeclaration($native->item_schema->value[0]));
        if ($native->label_details->presence !== 0) $values['label_details'] = $native->label_details->presence === 1 ? null : (self::sequence($native->label_details->value, fn($v) => self::readNativeLabel($v[0])));
        if ($native->model->presence !== 0) $values['model'] = $native->model->presence === 1 ? null : (self::text($native->model->value));
        if ($native->name->presence !== 0) $values['name'] = $native->name->presence === 1 ? null : (self::readNativeQuestionName($native->name->value[0]));
        if ($native->on->presence !== 0) $values['on'] = $native->on->presence === 1 ? null : (self::sequence($native->on->value, fn($v) => self::text($v)));
        if ($native->profile->presence !== 0) $values['profile'] = $native->profile->presence === 1 ? null : (self::text($native->profile->value));
        if ($native->wording_version->presence !== 0) $values['wording_version'] = $native->wording_version->presence === 1 ? null : (self::readNativeWordingVersion($native->wording_version->value[0]));
        $values['labels'] = self::sequence($native->labels, fn($v) => self::text($v));
        if ($native->text->presence !== 0) $values['text'] = $native->text->presence === 1 ? null : (self::jsonValue($native->text->value[0]));
        $values['verb'] = self::text($native->verb);
        return new NativeReadableQuestionTag((object)$values, $values);
    }
    private static function readNativeRecognition(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readNativeRecognitionOdds($native->answer[0]);
        $values['answer_id'] = self::readNativeAnswerId($native->answer_id[0]);
        if ($native->file->presence !== 0) $values['file'] = $native->file->presence === 1 ? null : (self::text($native->file->value));
        if ($native->first_line->presence !== 0) $values['first_line'] = $native->first_line->presence === 1 ? null : (self::unsigned($native->first_line->value));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->last_line->presence !== 0) $values['last_line'] = $native->last_line->presence === 1 ? null : (self::unsigned($native->last_line->value));
        $values['meta'] = self::readNativeMeta($native->meta[0]);
        if ($native->position->presence !== 0) $values['position'] = $native->position->presence === 1 ? null : (self::readNativePosition($native->position->value[0]));
        $values['question'] = self::readNativeReadableQuestion3($native->question[0]);
        $values['schema'] = self::readNativeVersion($native->schema[0]);
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readNativePhysicalSource($native->source->value[0]));
        $values['value'] = self::readNativeRecognize($native->value[0]);
        return new NativeRecognition((object)$values, $values);
    }
    private static function readNativeRecognitionEdgeDocument(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['either'] = (bool)($native->either);
        $values['probability'] = $native->probability;
        $values['relation'] = self::text($native->relation);
        $values['source'] = self::readNativeEntity($native->source[0]);
        $values['target'] = self::readNativeEntity($native->target[0]);
        return new NativeRecognitionEdgeDocument((object)$values, $values);
    }
    private static function readNativeRecognitionMode(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'whole',
            2 => 'boundary_only',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readNativeRecognitionOdds(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readNativeRecognitionOddsRecognizeAnswer($native->data->recognize_answer[0]),
            2 => self::readNativeRecognitionOddsBoundaryOdds($native->data->boundary_odds[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readNativeRecognitionOddsBoundaryOdds(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['pieces'] = self::sequence($native->pieces, fn($v) => self::readNativePieceOdds($v[0]));
        $values['proposals'] = self::sequence($native->proposals, fn($v) => self::readNativeBoundaryProposal($v[0]));
        return new NativeRecognitionOddsBoundaryOdds((object)$values, $values);
    }
    private static function readNativeRecognitionOddsRecognizeAnswer(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['names'] = self::sequence($native->names, fn($v) => self::readNativeNameOdds($v[0]));
        $values['pairs'] = self::sequence($native->pairs, fn($v) => self::readNativePairOdds($v[0]));
        $values['pieces'] = self::sequence($native->pieces, fn($v) => self::readNativePieceOdds($v[0]));
        $values['proposals'] = self::sequence($native->proposals, fn($v) => self::readNativeRecognitionProposal($v[0]));
        return new NativeRecognitionOddsRecognizeAnswer((object)$values, $values);
    }
    private static function readNativeRecognitionProposal(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['end'] = self::unsigned($native->end);
        $values['kept'] = (bool)($native->kept);
        if ($native->kind->presence !== 0) $values['kind'] = $native->kind->presence === 1 ? null : (self::text($native->kind->value));
        if ($native->selected->presence !== 0) $values['selected'] = $native->selected->presence === 1 ? null : (self::readNativePlace($native->selected->value[0]));
        $values['span_probability'] = $native->span_probability;
        $values['start'] = self::unsigned($native->start);
        if ($native->strength->presence !== 0) $values['strength'] = $native->strength->presence === 1 ? null : ($native->strength->value);
        return new NativeRecognitionProposal((object)$values, $values);
    }
    private static function readNativeRecognitionStageContext(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->boundary->presence !== 0) $values['boundary'] = $native->boundary->presence === 1 ? null : (self::text($native->boundary->value));
        if ($native->kind_edge->presence !== 0) $values['kind_edge'] = $native->kind_edge->presence === 1 ? null : (self::text($native->kind_edge->value));
        if ($native->relation->presence !== 0) $values['relation'] = $native->relation->presence === 1 ? null : (self::text($native->relation->value));
        return new NativeRecognitionStageContext((object)$values, $values);
    }
    private static function readNativeRelation(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readNativeAnswers($native->answer[0]);
        $values['answer_id'] = self::readNativeAnswerId($native->answer_id[0]);
        if ($native->file->presence !== 0) $values['file'] = $native->file->presence === 1 ? null : (self::text($native->file->value));
        if ($native->first_line->presence !== 0) $values['first_line'] = $native->first_line->presence === 1 ? null : (self::unsigned($native->first_line->value));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->input_sources->presence !== 0) $values['input_sources'] = $native->input_sources->presence === 1 ? null : (self::sequence($native->input_sources->value, fn($v) => self::readNativeSessionInputSource($v[0])));
        if ($native->last_line->presence !== 0) $values['last_line'] = $native->last_line->presence === 1 ? null : (self::unsigned($native->last_line->value));
        $values['meta'] = self::readNativeMeta($native->meta[0]);
        if ($native->position->presence !== 0) $values['position'] = $native->position->presence === 1 ? null : (self::readNativePosition($native->position->value[0]));
        $values['question'] = self::readNativeReadableQuestion4($native->question[0]);
        $values['schema'] = self::readNativeVersion($native->schema[0]);
        $values['value'] = self::sequence($native->value, fn($v) => self::readNativeRelatedEntityEdge($v[0]));
        return new NativeRelation((object)$values, $values);
    }
    private static function readNativeRelationDirection(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'source_to_target',
            2 => 'either',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readNativeRelationEndpoint(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readNativeRelationEndpointRelatedEntity($native->data->related_entity[0]),
            2 => self::readNativeRelationEndpointSource($native->data->source[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readNativeRelationEndpointRelatedEntity(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['name'] = self::text($native->name);
        return new NativeRelationEndpointRelatedEntity((object)$values, $values);
    }
    private static function readNativeRelationEndpointSource(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->file->presence !== 0) $values['file'] = $native->file->presence === 1 ? null : (self::text($native->file->value));
        if ($native->first_line->presence !== 0) $values['first_line'] = $native->first_line->presence === 1 ? null : (self::unsigned($native->first_line->value));
        $values['kind'] = self::text($native->kind);
        if ($native->last_line->presence !== 0) $values['last_line'] = $native->last_line->presence === 1 ? null : (self::unsigned($native->last_line->value));
        $values['name'] = self::text($native->name);
        $values['ordinal'] = self::unsigned($native->ordinal);
        if ($native->record->presence !== 0) $values['record'] = $native->record->presence === 1 ? null : (self::jsonValue($native->record->value[0]));
        return new NativeRelationEndpointSource((object)$values, $values);
    }
    private static function readNativeRelationMember(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readNativeRelationMemberAnswerId($native->data->answer_id[0]),
            2 => self::readNativeRelationMemberFailureId($native->data->failure_id[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readNativeRelationMemberAnswerId(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['direction'] = self::readNativeRelationDirection($native->direction[0]);
        $values['method'] = self::readNativeRelationMethod($native->method[0]);
        $values['observations'] = self::sequence($native->observations, fn($v) => self::readNativeObservation($v[0]));
        $values['question'] = self::readNativeReadableQuestion($native->question[0]);
        $values['question_sources'] = self::sequence($native->question_sources, fn($v) => self::readNativeQuestionSource($v[0]));
        $values['reads'] = self::text($native->reads);
        $values['relation'] = self::text($native->relation);
        $values['request'] = self::text($native->request);
        $values['source'] = self::readNativeRelatedEntity($native->source[0]);
        if ($native->target->presence !== 0) $values['target'] = $native->target->presence === 1 ? null : (self::readNativeRelatedEntity($native->target->value[0]));
        $values['threshold'] = self::readNativeThreshold($native->threshold[0]);
        if ($native->usage->presence !== 0) $values['usage'] = $native->usage->presence === 1 ? null : (self::readNativeUsage($native->usage->value[0]));
        $values['accepted'] = (bool)($native->accepted);
        $values['answer'] = self::readNativeAnswer($native->answer[0]);
        $values['answer_id'] = self::readNativeAnswerId($native->answer_id[0]);
        $values['probability'] = $native->probability;
        return new NativeRelationMemberAnswerId((object)$values, $values);
    }
    private static function readNativeRelationMemberFailureId(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['direction'] = self::readNativeRelationDirection($native->direction[0]);
        $values['method'] = self::readNativeRelationMethod($native->method[0]);
        $values['observations'] = self::sequence($native->observations, fn($v) => self::readNativeObservation($v[0]));
        $values['question'] = self::readNativeReadableQuestion($native->question[0]);
        $values['question_sources'] = self::sequence($native->question_sources, fn($v) => self::readNativeQuestionSource($v[0]));
        $values['reads'] = self::text($native->reads);
        $values['relation'] = self::text($native->relation);
        $values['request'] = self::text($native->request);
        $values['source'] = self::readNativeRelatedEntity($native->source[0]);
        if ($native->target->presence !== 0) $values['target'] = $native->target->presence === 1 ? null : (self::readNativeRelatedEntity($native->target->value[0]));
        $values['threshold'] = self::readNativeThreshold($native->threshold[0]);
        if ($native->usage->presence !== 0) $values['usage'] = $native->usage->presence === 1 ? null : (self::readNativeUsage($native->usage->value[0]));
        $values['failure'] = self::readNativeFailure($native->failure[0]);
        $values['failure_id'] = self::readNativeFailureId($native->failure_id[0]);
        return new NativeRelationMemberFailureId((object)$values, $values);
    }
    private static function readNativeRelationMethod(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'yes_no',
            2 => 'choice',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readNativeRequestFunction(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'decide',
            2 => 'choose',
            3 => 'tag',
            4 => 'score',
            5 => 'filter',
            6 => 'rank',
            7 => 'find',
            8 => 'annotate',
            9 => 'recognize',
            10 => 'relate',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readNativeSdkRequestId(\FFI\CData $native): mixed
    {
        return self::text($native->value);
    }
    private static function readNativeSendBudgetDenial(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readNativeSendBudgetDenialBeforeFirstSend($native->data->before_first_send[0]),
            2 => self::readNativeSendBudgetDenialBeforeAdditionalSend($native->data->before_additional_send[0]),
            3 => self::readNativeSendBudgetDenialBeforeRetry($native->data->before_retry[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readNativeSendBudgetDenialBeforeAdditionalSend(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        return new NativeSendBudgetDenialBeforeAdditionalSend((object)$values, $values);
    }
    private static function readNativeSendBudgetDenialBeforeFirstSend(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        return new NativeSendBudgetDenialBeforeFirstSend((object)$values, $values);
    }
    private static function readNativeSendBudgetDenialBeforeRetry(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['last_status'] = self::unsigned($native->last_status);
        return new NativeSendBudgetDenialBeforeRetry((object)$values, $values);
    }
    private static function readNativeStopCause(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'usage',
            2 => 'local',
            3 => 'no_key',
            4 => 'transport',
            5 => 'status',
            6 => 'too_large',
            7 => 'reply',
            8 => 'backend',
            9 => 'cancelled',
            10 => 'deadline',
            11 => 'defect',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readNativeStopped(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->at->presence !== 0) $values['at'] = $native->at->presence === 1 ? null : (self::unsigned($native->at->value));
        $values['cause'] = self::readNativeStopCause($native->cause[0]);
        $values['retryable'] = (bool)($native->retryable);
        if ($native->status->presence !== 0) $values['status'] = $native->status->presence === 1 ? null : (self::unsigned($native->status->value));
        return new NativeStopped((object)$values, $values);
    }
    private static function readNativeStringRoot(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['type'] = self::readNativeStringType($native->type[0]);
        return new NativeStringRoot((object)$values, $values);
    }
    private static function readNativeStringType(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'string',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readNativeUsage(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->input_tokens->presence !== 0) $values['input_tokens'] = $native->input_tokens->presence === 1 ? null : (self::unsigned($native->input_tokens->value));
        if ($native->output_tokens->presence !== 0) $values['output_tokens'] = $native->output_tokens->presence === 1 ? null : (self::unsigned($native->output_tokens->value));
        return new NativeUsage((object)$values, $values);
    }
    private static function readNativeUsagePersistence(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'disabled',
            2 => 'pending',
            3 => 'written',
            4 => 'failed',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readNativeVerb(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'recognize',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readNativeVersion(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'thinkthen.result/2',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readNativeWordingVersion(\FFI\CData $native): mixed
    {
        return self::unsigned($native->value);
    }
    private static function readNativeAnnotatedField(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => (bool)($native->data->boolean),
            2 => null,
            3 => self::text($native->data->string),
            4 => self::sequence($native->data->array, fn($v) => self::text($v)),
            5 => $native->data->number,
            6 => self::readNativeFailed($native->data->object[0]),
            default => throw new \LogicException('invalid native value'),
        };
    }
    private static function readNativeAnnotatedRow(\FFI\CData $native): mixed
    {
        return self::mapping($native->value, fn($v) => self::readNativeAnnotatedField($v[0]));
    }
    private static function readNativeAnswer(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readNativeAnswerYesNo($native->data->yes_no[0]),
            2 => self::readNativeAnswerChoice($native->data->choice[0]),
            3 => self::readNativeAnswerTag($native->data->tag[0]),
            4 => self::readNativeAnswerScore($native->data->score[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readNativeAnswerChoice(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->confidence->presence !== 0) $values['confidence'] = $native->confidence->presence === 1 ? null : ($native->confidence->value);
        $values['kind'] = self::text($native->kind);
        $values['pick'] = self::text($native->pick);
        $values['probabilities'] = self::mapping($native->probabilities, fn($v) => $v);
        return new NativeAnswerChoice((object)$values, $values);
    }
    private static function readNativeAnswerScore(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->confidence->presence !== 0) $values['confidence'] = $native->confidence->presence === 1 ? null : ($native->confidence->value);
        $values['kind'] = self::text($native->kind);
        $values['level'] = self::text($native->level);
        $values['probabilities'] = self::mapping($native->probabilities, fn($v) => $v);
        return new NativeAnswerScore((object)$values, $values);
    }
    private static function readNativeAnswerTag(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['probabilities'] = self::mapping($native->probabilities, fn($v) => $v);
        return new NativeAnswerTag((object)$values, $values);
    }
    private static function readNativeAnswerYesNo(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['probability'] = $native->probability;
        return new NativeAnswerYesNo((object)$values, $values);
    }
    private static function readNativeAttemptOutcome(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'ok',
            2 => 'status',
            3 => 'transport',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readNativeBatchSetting(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::unsigned($native->data->integer),
            2 => self::text($native->data->string),
            default => throw new \LogicException('invalid native value'),
        };
    }
    private static function readNativeBatchWarning(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['running'] = self::readNativeBatchSetting($native->running[0]);
        $values['tuned_for'] = self::readNativeBatchSetting($native->tuned_for[0]);
        return new NativeBatchWarning((object)$values, $values);
    }
    private static function readNativeEntity(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['end'] = self::unsigned($native->end);
        if ($native->file->presence !== 0) $values['file'] = $native->file->presence === 1 ? null : (self::text($native->file->value));
        if ($native->first_line->presence !== 0) $values['first_line'] = $native->first_line->presence === 1 ? null : (self::unsigned($native->first_line->value));
        $values['kind'] = self::text($native->kind);
        if ($native->last_line->presence !== 0) $values['last_line'] = $native->last_line->presence === 1 ? null : (self::unsigned($native->last_line->value));
        $values['length'] = self::unsigned($native->length);
        $values['start'] = self::unsigned($native->start);
        $values['strength'] = $native->strength;
        $values['text'] = self::text($native->text);
        return new NativeEntity((object)$values, $values);
    }
    private static function readNativeEntityEdge(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->either->presence !== 0) $values['either'] = $native->either->presence === 1 ? null : ((bool)($native->either->value));
        $values['probability'] = $native->probability;
        $values['relation'] = self::text($native->relation);
        $values['source'] = self::readNativeEntity($native->source[0]);
        $values['target'] = self::readNativeEntity($native->target[0]);
        return new NativeEntityEdge((object)$values, $values);
    }
    private static function readNativeFailed(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['failed'] = self::readNativeFailure($native->failed[0]);
        return new NativeFailed((object)$values, $values);
    }
    private static function readNativeFailure(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['cause'] = self::readNativeFailureCause($native->cause[0]);
        $values['kind'] = self::readNativeFailureFieldKind($native->kind[0]);
        return new NativeFailure((object)$values, $values);
    }
    private static function readNativeFailureCause(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'missing_answer',
            2 => 'wrong_kind',
            3 => 'missing_probability',
            4 => 'invalid_probability',
            5 => 'invalid_distribution',
            6 => 'unexpected_probability',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readNativeFailureKind(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'usage',
            2 => 'backend',
            3 => 'local',
            4 => 'cancelled',
            5 => 'deadline',
            6 => 'defect',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readNativeFindAnswer(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->confidence->presence !== 0) $values['confidence'] = $native->confidence->presence === 1 ? null : ($native->confidence->value);
        $values['kind'] = self::text($native->kind);
        $values['pick'] = self::text($native->pick);
        $values['probabilities'] = self::mapping($native->probabilities, fn($v) => $v);
        return new NativeFindAnswer((object)$values, $values);
    }
    private static function readNativeNameOdds(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->edges->presence !== 0) $values['edges'] = $native->edges->presence === 1 ? null : (self::mapping($native->edges->value, fn($v) => $v));
        $values['end'] = self::unsigned($native->end);
        if ($native->kinds->presence !== 0) $values['kinds'] = $native->kinds->presence === 1 ? null : (self::mapping($native->kinds->value, fn($v) => $v));
        $values['start'] = self::unsigned($native->start);
        return new NativeNameOdds((object)$values, $values);
    }
    private static function readNativePairOdds(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['probability'] = $native->probability;
        $values['relation'] = self::text($native->relation);
        $values['source'] = self::readNativePlace($native->source[0]);
        $values['target'] = self::readNativePlace($native->target[0]);
        return new NativePairOdds((object)$values, $values);
    }
    private static function readNativePieceOdds(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['end'] = self::unsigned($native->end);
        $values['start'] = self::unsigned($native->start);
        $values['tags'] = self::mapping($native->tags, fn($v) => $v);
        return new NativePieceOdds((object)$values, $values);
    }
    private static function readNativePlace(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['end'] = self::unsigned($native->end);
        $values['start'] = self::unsigned($native->start);
        return new NativePlace((object)$values, $values);
    }
    private static function readNativeProfileWarning(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['running'] = self::text($native->running);
        $values['tuned_for'] = self::text($native->tuned_for);
        return new NativeProfileWarning((object)$values, $values);
    }
    private static function readNativeRecognize(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readNativeRecognizeEntities($native->data->entities[0]),
            2 => self::readNativeRecognizeModeProposals($native->data->mode_proposals[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readNativeRecognizeAnswer(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['names'] = self::sequence($native->names, fn($v) => self::readNativeNameOdds($v[0]));
        $values['pairs'] = self::sequence($native->pairs, fn($v) => self::readNativePairOdds($v[0]));
        $values['pieces'] = self::sequence($native->pieces, fn($v) => self::readNativePieceOdds($v[0]));
        $values['proposals'] = self::sequence($native->proposals, fn($v) => self::readNativeRecognitionProposal($v[0]));
        return new NativeRecognizeAnswer((object)$values, $values);
    }
    private static function readNativeRecognizeEntities(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['entities'] = self::sequence($native->entities, fn($v) => self::readNativeEntity($v[0]));
        if ($native->relations->presence !== 0) $values['relations'] = $native->relations->presence === 1 ? null : (self::sequence($native->relations->value, fn($v) => self::readNativeEntityEdge($v[0])));
        return new NativeRecognizeEntities((object)$values, $values);
    }
    private static function readNativeRecognizeModeProposals(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['mode'] = self::readNativeBoundaryMode($native->mode[0]);
        $values['proposals'] = self::sequence($native->proposals, fn($v) => self::readNativeBoundaryProposal($v[0]));
        return new NativeRecognizeModeProposals((object)$values, $values);
    }
    private static function readNativeRelateFields(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['name'] = self::text($native->name);
        return new NativeRelateFields((object)$values, $values);
    }
    private static function readNativeRelatedEntity(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['name'] = self::text($native->name);
        return new NativeRelatedEntity((object)$values, $values);
    }
    private static function readNativeRelatedEntityEdge(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->either->presence !== 0) $values['either'] = $native->either->presence === 1 ? null : ((bool)($native->either->value));
        $values['probability'] = $native->probability;
        $values['relation'] = self::text($native->relation);
        $values['source'] = self::readNativeRelationEndpoint($native->source[0]);
        $values['target'] = self::readNativeRelationEndpoint($native->target[0]);
        return new NativeRelatedEntityEdge((object)$values, $values);
    }
    private static function readNativeRelationRule(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['either'] = (bool)($native->either);
        $values['name'] = self::text($native->name);
        $values['reads'] = self::text($native->reads);
        if ($native->single->presence !== 0) $values['single'] = $native->single->presence === 1 ? null : ((bool)($native->single->value));
        $values['source'] = self::text($native->source);
        $values['target'] = self::text($native->target);
        return new NativeRelationRule((object)$values, $values);
    }
    private static function readNativeSessionAnnotation(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['name'] = self::text($native->name);
        $values['value'] = self::readNativeAnnotationValue($native->value[0]);
        return new NativeSessionAnnotation((object)$values, $values);
    }
    private static function readNativeSessionInputSource(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['index'] = self::unsigned($native->index);
        $values['source'] = self::readNativePhysicalSource($native->source[0]);
        return new NativeSessionInputSource((object)$values, $values);
    }
    private static function readNativeSessionJudgment(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readNativeSessionJudgmentDecision($native->data->decision[0]),
            2 => self::readNativeSessionJudgmentChoice($native->data->choice[0]),
            3 => self::readNativeSessionJudgmentScore($native->data->score[0]),
            4 => self::readNativeSessionJudgmentTags($native->data->tags[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readNativeSessionJudgmentChoice(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : (self::text($native->value->value));
        return new NativeSessionJudgmentChoice((object)$values, $values);
    }
    private static function readNativeSessionJudgmentDecision(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : ((bool)($native->value->value));
        return new NativeSessionJudgmentDecision((object)$values, $values);
    }
    private static function readNativeSessionJudgmentScore(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = $native->value;
        return new NativeSessionJudgmentScore((object)$values, $values);
    }
    private static function readNativeSessionJudgmentTags(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::text($v));
        return new NativeSessionJudgmentTags((object)$values, $values);
    }
    private static function readNativeSessionNamedProbability(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['name'] = self::text($native->name);
        $values['probability'] = $native->probability;
        return new NativeSessionNamedProbability((object)$values, $values);
    }
    private static function readNativeSessionObservation(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readNativeSessionObservationQuestion($native->data->question[0]),
            2 => self::readNativeSessionObservationRow($native->data->row[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readNativeSessionObservationQuestion(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['detail'] = self::readNativeSessionQuestionDetail($native->detail[0]);
        $values['index'] = self::unsigned($native->index);
        $values['kind'] = self::text($native->kind);
        if ($native->member->presence !== 0) $values['member'] = $native->member->presence === 1 ? null : (self::text($native->member->value));
        $values['position'] = self::unsigned($native->position);
        if ($native->stage->presence !== 0) $values['stage'] = $native->stage->presence === 1 ? null : (self::text($native->stage->value));
        return new NativeSessionObservationQuestion((object)$values, $values);
    }
    private static function readNativeSessionObservationRow(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['index'] = self::unsigned($native->index);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readNativeSessionObservedRow($native->value[0]);
        return new NativeSessionObservationRow((object)$values, $values);
    }
    private static function readNativeSessionObservedRow(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readNativeSessionObservedRowJudgment($native->data->judgment[0]),
            2 => self::readNativeSessionObservedRowAnnotated($native->data->annotated[0]),
            3 => self::readNativeSessionObservedRowRecognized($native->data->recognized[0]),
            4 => self::readNativeSessionObservedRowFind($native->data->find[0]),
            5 => self::readNativeSessionObservedRowRelations($native->data->relations[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readNativeSessionObservedRowAnnotated(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readNativeSessionAnnotation($v[0]));
        return new NativeSessionObservedRowAnnotated((object)$values, $values);
    }
    private static function readNativeSessionObservedRowFind(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : (self::unsigned($native->value->value));
        return new NativeSessionObservedRowFind((object)$values, $values);
    }
    private static function readNativeSessionObservedRowJudgment(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readNativeSessionJudgment($native->value[0]);
        return new NativeSessionObservedRowJudgment((object)$values, $values);
    }
    private static function readNativeSessionObservedRowRecognized(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readNativeSessionRecognition($native->value[0]);
        return new NativeSessionObservedRowRecognized((object)$values, $values);
    }
    private static function readNativeSessionObservedRowRelations(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readNativeSessionRelationEdge($v[0]));
        return new NativeSessionObservedRowRelations((object)$values, $values);
    }
    private static function readNativeSessionPacket(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readNativeSessionPacketDecideRow($native->data->decide_row[0]),
            2 => self::readNativeSessionPacketChooseRow($native->data->choose_row[0]),
            3 => self::readNativeSessionPacketTagRow($native->data->tag_row[0]),
            4 => self::readNativeSessionPacketScoreRow($native->data->score_row[0]),
            5 => self::readNativeSessionPacketFilterRow($native->data->filter_row[0]),
            6 => self::readNativeSessionPacketAnnotateRow($native->data->annotate_row[0]),
            7 => self::readNativeSessionPacketDecideAggregate($native->data->decide_aggregate[0]),
            8 => self::readNativeSessionPacketChooseAggregate($native->data->choose_aggregate[0]),
            9 => self::readNativeSessionPacketTagAggregate($native->data->tag_aggregate[0]),
            10 => self::readNativeSessionPacketScoreAggregate($native->data->score_aggregate[0]),
            11 => self::readNativeSessionPacketFilterAggregate($native->data->filter_aggregate[0]),
            12 => self::readNativeSessionPacketRankAggregate($native->data->rank_aggregate[0]),
            13 => self::readNativeSessionPacketFindAggregate($native->data->find_aggregate[0]),
            14 => self::readNativeSessionPacketAnnotateAggregate($native->data->annotate_aggregate[0]),
            15 => self::readNativeSessionPacketRecognizeAggregate($native->data->recognize_aggregate[0]),
            16 => self::readNativeSessionPacketRelateAggregate($native->data->relate_aggregate[0]),
            17 => self::readNativeSessionPacketObservation($native->data->observation[0]),
            18 => self::readNativeSessionPacketTerminal($native->data->terminal[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readNativeSessionPacketAnnotateAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readNativeAnnotation($v[0]));
        return new NativeSessionPacketAnnotateAggregate((object)$values, $values);
    }
    private static function readNativeSessionPacketAnnotateRow(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readNativeAnnotation($native->value[0]);
        return new NativeSessionPacketAnnotateRow((object)$values, $values);
    }
    private static function readNativeSessionPacketChooseAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readNativeAtomicNullableString($v[0]));
        return new NativeSessionPacketChooseAggregate((object)$values, $values);
    }
    private static function readNativeSessionPacketChooseRow(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readNativeAtomicNullableString($native->value[0]);
        return new NativeSessionPacketChooseRow((object)$values, $values);
    }
    private static function readNativeSessionPacketDecideAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readNativeAtomicDecideValue($v[0]));
        return new NativeSessionPacketDecideAggregate((object)$values, $values);
    }
    private static function readNativeSessionPacketDecideRow(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readNativeAtomicDecideValue($native->value[0]);
        return new NativeSessionPacketDecideRow((object)$values, $values);
    }
    private static function readNativeSessionPacketFilterAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readNativeAtomicBoolean($v[0]));
        return new NativeSessionPacketFilterAggregate((object)$values, $values);
    }
    private static function readNativeSessionPacketFilterRow(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readNativeAtomicBoolean($native->value[0]);
        return new NativeSessionPacketFilterRow((object)$values, $values);
    }
    private static function readNativeSessionPacketFindAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readNativeFind($native->value[0]);
        return new NativeSessionPacketFindAggregate((object)$values, $values);
    }
    private static function readNativeSessionPacketObservation(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::readNativeRequestFunction($native->function[0]);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readNativeSessionObservation($native->value[0]);
        return new NativeSessionPacketObservation((object)$values, $values);
    }
    private static function readNativeSessionPacketRankAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readNativeAtomicNonZeroUsize($v[0]));
        return new NativeSessionPacketRankAggregate((object)$values, $values);
    }
    private static function readNativeSessionPacketRecognizeAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readNativeRecognition($v[0]));
        return new NativeSessionPacketRecognizeAggregate((object)$values, $values);
    }
    private static function readNativeSessionPacketRelateAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readNativeRelation($native->value[0]);
        return new NativeSessionPacketRelateAggregate((object)$values, $values);
    }
    private static function readNativeSessionPacketScoreAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readNativeAtomicDouble($v[0]));
        return new NativeSessionPacketScoreAggregate((object)$values, $values);
    }
    private static function readNativeSessionPacketScoreRow(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readNativeAtomicDouble($native->value[0]);
        return new NativeSessionPacketScoreRow((object)$values, $values);
    }
    private static function readNativeSessionPacketTagAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readNativeAtomicArrayOfString($v[0]));
        return new NativeSessionPacketTagAggregate((object)$values, $values);
    }
    private static function readNativeSessionPacketTagRow(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readNativeAtomicArrayOfString($native->value[0]);
        return new NativeSessionPacketTagRow((object)$values, $values);
    }
    private static function readNativeSessionPacketTerminal(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->facts->presence !== 0) $values['facts'] = $native->facts->presence === 1 ? null : (self::readNativeFacts($native->facts->value[0]));
        if ($native->failure->presence !== 0) $values['failure'] = $native->failure->presence === 1 ? null : (self::readNativeCallError($native->failure->value[0]));
        $values['kind'] = self::text($native->kind);
        return new NativeSessionPacketTerminal((object)$values, $values);
    }
    private static function readNativeSessionProbabilities(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readNativeSessionProbabilitiesYesNo($native->data->yes_no[0]),
            2 => self::readNativeSessionProbabilitiesNamed($native->data->named[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readNativeSessionProbabilitiesNamed(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readNativeSessionNamedProbability($v[0]));
        return new NativeSessionProbabilitiesNamed((object)$values, $values);
    }
    private static function readNativeSessionProbabilitiesYesNo(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = $native->value;
        return new NativeSessionProbabilitiesYesNo((object)$values, $values);
    }
    private static function readNativeSessionQuestionDetail(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->answer_id->presence !== 0) $values['answer_id'] = $native->answer_id->presence === 1 ? null : (self::readNativeAnswerId($native->answer_id->value[0]));
        $values['cached'] = (bool)($native->cached);
        if ($native->confidence->presence !== 0) $values['confidence'] = $native->confidence->presence === 1 ? null : ($native->confidence->value);
        $values['failed_questions'] = self::unsigned($native->failed_questions);
        if ($native->failure->presence !== 0) $values['failure'] = $native->failure->presence === 1 ? null : (self::readNativeFailure($native->failure->value[0]));
        if ($native->failure_id->presence !== 0) $values['failure_id'] = $native->failure_id->presence === 1 ? null : (self::readNativeFailureId($native->failure_id->value[0]));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->input_source->presence !== 0) $values['input_source'] = $native->input_source->presence === 1 ? null : (self::readNativePhysicalSource($native->input_source->value[0]));
        $values['input_sources'] = self::sequence($native->input_sources, fn($v) => self::readNativeSessionInputSource($v[0]));
        $values['inputs'] = self::sequence($native->inputs, fn($v) => self::jsonValue($v[0]));
        $values['model'] = self::text($native->model);
        $values['observations'] = self::sequence($native->observations, fn($v) => self::readNativeObservation($v[0]));
        if ($native->probabilities->presence !== 0) $values['probabilities'] = $native->probabilities->presence === 1 ? null : (self::readNativeSessionProbabilities($native->probabilities->value[0]));
        $values['question'] = self::readNativeReadableQuestion($native->question[0]);
        $values['question_sha256'] = self::text($native->question_sha256);
        $values['question_sources'] = self::sequence($native->question_sources, fn($v) => self::readNativeQuestionSource($v[0]));
        if ($native->raw_pick->presence !== 0) $values['raw_pick'] = $native->raw_pick->presence === 1 ? null : (self::text($native->raw_pick->value));
        if ($native->reported_usage->presence !== 0) $values['reported_usage'] = $native->reported_usage->presence === 1 ? null : (self::readNativeUsage($native->reported_usage->value[0]));
        $values['requests'] = self::sequence($native->requests, fn($v) => self::text($v));
        $values['requests_sent'] = self::unsigned($native->requests_sent);
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::readNativeThreshold($native->threshold->value[0]));
        $values['url'] = self::text($native->url);
        if ($native->usage->presence !== 0) $values['usage'] = $native->usage->presence === 1 ? null : (self::readNativeTokenUsage($native->usage->value[0]));
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : (self::readNativeValue($native->value->value[0]));
        return new NativeSessionQuestionDetail((object)$values, $values);
    }
    private static function readNativeSessionRecognition(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['entities'] = self::sequence($native->entities, fn($v) => self::readNativeEntity($v[0]));
        $values['mode'] = self::readNativeRecognitionMode($native->mode[0]);
        if ($native->proposals->presence !== 0) $values['proposals'] = $native->proposals->presence === 1 ? null : (self::sequence($native->proposals->value, fn($v) => self::readNativeBoundaryProposal($v[0])));
        if ($native->relations->presence !== 0) $values['relations'] = $native->relations->presence === 1 ? null : (self::sequence($native->relations->value, fn($v) => self::readNativeRecognitionEdgeDocument($v[0])));
        return new NativeSessionRecognition((object)$values, $values);
    }
    private static function readNativeSessionRelationEdge(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['either'] = (bool)($native->either);
        $values['probability'] = $native->probability;
        $values['relation'] = self::text($native->relation);
        $values['source'] = self::readNativeEntityDocument($native->source[0]);
        $values['target'] = self::readNativeEntityDocument($native->target[0]);
        return new NativeSessionRelationEdge((object)$values, $values);
    }
    private static function readNativeSourceRelationEndpoint(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->file->presence !== 0) $values['file'] = $native->file->presence === 1 ? null : (self::text($native->file->value));
        if ($native->first_line->presence !== 0) $values['first_line'] = $native->first_line->presence === 1 ? null : (self::unsigned($native->first_line->value));
        $values['kind'] = self::text($native->kind);
        if ($native->last_line->presence !== 0) $values['last_line'] = $native->last_line->presence === 1 ? null : (self::unsigned($native->last_line->value));
        $values['name'] = self::text($native->name);
        $values['ordinal'] = self::unsigned($native->ordinal);
        if ($native->record->presence !== 0) $values['record'] = $native->record->presence === 1 ? null : (self::jsonValue($native->record->value[0]));
        return new NativeSourceRelationEndpoint((object)$values, $values);
    }
    private static function readNativeThreshold(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => $native->data->number,
            2 => self::text($native->data->string),
            default => throw new \LogicException('invalid native value'),
        };
    }
    private static function readNativeTokenUsage(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['input_tokens'] = self::unsigned($native->input_tokens);
        $values['output_tokens'] = self::unsigned($native->output_tokens);
        return new NativeTokenUsage((object)$values, $values);
    }
    private static function readNativeValue(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => (bool)($native->data->boolean),
            2 => null,
            3 => self::text($native->data->string),
            4 => self::sequence($native->data->array, fn($v) => self::text($v)),
            5 => $native->data->number,
            default => throw new \LogicException('invalid native value'),
        };
    }
    private static function readNativeBatchString(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'max',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readNativeFailureFieldKind(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'backend',
            default => throw new \LogicException('invalid native enum'),
        };
    }
}
