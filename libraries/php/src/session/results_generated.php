<?php
// Generated from the shared Rust result graph; do not edit.
declare(strict_types=1);
namespace ThinkThen\Results;
trait AnswerAccessors
{
    private function requireSuccess(): void
    {
        if (!$this->terminal->has('failure')) return;
        $failure = $this->terminal->failure;
        $class = \ThinkThen\Internal\ErrorFactory::classFor($failure->error->kind);
        throw new $class($failure->error->message, $failure, $this->results, $this->terminal, $this->packets);
    }
    /** @return bool|null */ public function decideAnswer(): bool|null
    {
        $this->requireSuccess();
        $answers = [];
        foreach ($this->packets as $packet) {
            if ($packet->kind === 'observation' && $packet->value->kind === 'row' && $packet->value->value->kind === 'judgment' && $packet->value->value->value->kind === 'decision') $answers[] = $packet->value->value->value->value;
        }
        if (count($answers) !== 1) throw new \ThinkThen\UsageFailure('expected one answer');
        return $answers[0];
    }
    /** @return string|null */ public function chooseAnswer(): string|null
    {
        $this->requireSuccess();
        $answers = [];
        foreach ($this->packets as $packet) {
            if ($packet->kind === 'observation' && $packet->value->kind === 'row' && $packet->value->value->kind === 'judgment' && $packet->value->value->value->kind === 'choice') $answers[] = $packet->value->value->value->value;
        }
        if (count($answers) !== 1) throw new \ThinkThen\UsageFailure('expected one answer');
        return $answers[0];
    }
    /** @return int|float */ public function scoreAnswer(): int|float
    {
        $this->requireSuccess();
        $answers = [];
        foreach ($this->packets as $packet) {
            if ($packet->kind === 'observation' && $packet->value->kind === 'row' && $packet->value->value->kind === 'judgment' && $packet->value->value->value->kind === 'score') $answers[] = $packet->value->value->value->value;
        }
        if (count($answers) !== 1) throw new \ThinkThen\UsageFailure('expected one answer');
        return $answers[0];
    }
    /** @return list<string> */ public function tagAnswer(): array
    {
        $this->requireSuccess();
        $answers = [];
        foreach ($this->packets as $packet) {
            if ($packet->kind === 'observation' && $packet->value->kind === 'row' && $packet->value->value->kind === 'judgment' && $packet->value->value->value->kind === 'tags') $answers[] = $packet->value->value->value->value;
        }
        if (count($answers) !== 1) throw new \ThinkThen\UsageFailure('expected one answer');
        return $answers[0];
    }
    /** @return list<FilterResult> */ public function filterAnswer(): mixed
    {
        $this->requireSuccess();
        $answer = [];
        foreach ($this->packets as $packet) {
            if ($packet instanceof SessionPacketFilterAggregate) array_push($answer, ...$packet->value);
            elseif ($packet instanceof SessionPacketFilterRow) $answer[] = $packet->value;
        }
        return array_values(array_map(fn($row) => $row->input, array_filter($answer, fn($row) => $row->value === true)));
    }
    /** @return list<RankResult> */ public function rankAnswer(): mixed
    {
        $this->requireSuccess();
        $answer = [];
        foreach ($this->packets as $packet) {
            if ($packet instanceof SessionPacketRankAggregate) array_push($answer, ...$packet->value);
        }
        $answer = array_values(array_filter($answer, fn($row) => $row->value !== null));
        usort($answer, fn($a, $b) => $a->value <=> $b->value);
        return array_map(fn($row) => $row->input, $answer);
    }
    /** @return Find */ public function findAnswer(): mixed
    {
        $this->requireSuccess();
        $values = array_values(array_filter($this->packets, fn($p) => $p instanceof SessionPacketFindAggregate));
        if (count($values) !== 1) throw new \ThinkThen\UsageFailure('expected one answer');
        $answer = $values[0]->value;
        return $answer;
    }
    /** @return list<Annotation> */ public function annotateAnswer(): mixed
    {
        $this->requireSuccess();
        $answer = [];
        foreach ($this->packets as $packet) {
            if ($packet instanceof SessionPacketAnnotateAggregate) array_push($answer, ...$packet->value);
            elseif ($packet instanceof SessionPacketAnnotateRow) $answer[] = $packet->value;
        }
        return $answer;
    }
    /** @return list<Recognition> */ public function recognizeAnswer(): mixed
    {
        $this->requireSuccess();
        $answer = [];
        foreach ($this->packets as $packet) {
            if ($packet instanceof SessionPacketRecognizeAggregate) array_push($answer, ...$packet->value);
        }
        return $answer;
    }
    /** @return Relation */ public function relateAnswer(): mixed
    {
        $this->requireSuccess();
        $values = array_values(array_filter($this->packets, fn($p) => $p instanceof SessionPacketRelateAggregate));
        if (count($values) !== 1) throw new \ThinkThen\UsageFailure('expected one answer');
        $answer = $values[0]->value;
        return $answer;
    }
}
final class Annotation extends Node {
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<\stdClass> */ public function answers(): Presence { return $this->field('answers'); }
    /** @return Presence<string> */ public function file(): Presence { return $this->field('file'); }
    /** @return Presence<int|string> */ public function first_line(): Presence { return $this->field('first_line'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<int|string> */ public function last_line(): Presence { return $this->field('last_line'); }
    /** @return Presence<Meta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<Position> */ public function position(): Presence { return $this->field('position'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<PhysicalSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<\stdClass> */ public function value(): Presence { return $this->field('value'); }
}
final class AnnotationMemberAnswerId extends Node {
    /** @return Presence<AnswerYesNo|AnswerChoice|AnswerTag|AnswerScore> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<list<ObservationObservationId|ObservationFailureId>> */ public function observations(): Presence { return $this->field('observations'); }
    /** @return Presence<QuestionDocumentDecide|QuestionDocumentChoose|QuestionDocumentTag|QuestionDocumentScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<list<QuestionSource>> */ public function question_sources(): Presence { return $this->field('question_sources'); }
    /** @return Presence<string> */ public function request(): Presence { return $this->field('request'); }
    /** @return Presence<int|float|string|null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<Usage> */ public function usage(): Presence { return $this->field('usage'); }
    /** @return Presence<mixed> */ public function value(): Presence { return $this->field('value'); }
}
final class AnnotationMemberFailureId extends Node {
    /** @return Presence<Failure> */ public function failure(): Presence { return $this->field('failure'); }
    /** @return Presence<string> */ public function failure_id(): Presence { return $this->field('failure_id'); }
    /** @return Presence<list<ObservationObservationId|ObservationFailureId>> */ public function observations(): Presence { return $this->field('observations'); }
    /** @return Presence<QuestionDocumentDecide|QuestionDocumentChoose|QuestionDocumentTag|QuestionDocumentScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<list<QuestionSource>> */ public function question_sources(): Presence { return $this->field('question_sources'); }
    /** @return Presence<string> */ public function request(): Presence { return $this->field('request'); }
    /** @return Presence<int|float|string|null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<Usage> */ public function usage(): Presence { return $this->field('usage'); }
}
final class AnnotationValueChoice extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string|null> */ public function value(): Presence { return $this->field('value'); }
}
final class AnnotationValueDecision extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<bool|null> */ public function value(): Presence { return $this->field('value'); }
}
final class AnnotationValueFailed extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<Failure> */ public function value(): Presence { return $this->field('value'); }
}
final class AnnotationValueScore extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|float> */ public function value(): Presence { return $this->field('value'); }
}
final class AnnotationValueTags extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<string>> */ public function value(): Presence { return $this->field('value'); }
}
final class Answers extends Node {
    /** @return Presence<list<RelationMemberAnswerId|RelationMemberFailureId>> */ public function questions(): Presence { return $this->field('questions'); }
}
final class TagResult extends Node {
    /** @return Presence<AnswerYesNo|AnswerChoice|AnswerTag|AnswerScore> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<list<Image>> */ public function images(): Presence { return $this->field('images'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<list<RankMember>> */ public function members(): Presence { return $this->field('members'); }
    /** @return Presence<Meta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<QuestionDocumentDecide|QuestionDocumentChoose|QuestionDocumentTag|QuestionDocumentScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function question_name(): Presence { return $this->field('question_name'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<PhysicalSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<int|float|string|null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<list<string>> */ public function value(): Presence { return $this->field('value'); }
}
final class DecisionResult extends Node {
    /** @return Presence<AnswerYesNo|AnswerChoice|AnswerTag|AnswerScore> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<list<Image>> */ public function images(): Presence { return $this->field('images'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<list<RankMember>> */ public function members(): Presence { return $this->field('members'); }
    /** @return Presence<Meta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<QuestionDocumentDecide|QuestionDocumentChoose|QuestionDocumentTag|QuestionDocumentScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function question_name(): Presence { return $this->field('question_name'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<PhysicalSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<int|float|string|null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<bool|null|mixed> */ public function value(): Presence { return $this->field('value'); }
}
final class RankResult extends Node {
    /** @return Presence<AnswerYesNo|AnswerChoice|AnswerTag|AnswerScore> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<list<Image>> */ public function images(): Presence { return $this->field('images'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<list<RankMember>> */ public function members(): Presence { return $this->field('members'); }
    /** @return Presence<Meta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<QuestionDocumentDecide|QuestionDocumentChoose|QuestionDocumentTag|QuestionDocumentScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function question_name(): Presence { return $this->field('question_name'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<PhysicalSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<int|float|string|null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<int|string> */ public function value(): Presence { return $this->field('value'); }
}
final class ChoiceResult extends Node {
    /** @return Presence<AnswerYesNo|AnswerChoice|AnswerTag|AnswerScore> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<list<Image>> */ public function images(): Presence { return $this->field('images'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<list<RankMember>> */ public function members(): Presence { return $this->field('members'); }
    /** @return Presence<Meta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<QuestionDocumentDecide|QuestionDocumentChoose|QuestionDocumentTag|QuestionDocumentScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function question_name(): Presence { return $this->field('question_name'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<PhysicalSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<int|float|string|null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<string|null> */ public function value(): Presence { return $this->field('value'); }
}
final class FilterResult extends Node {
    /** @return Presence<AnswerYesNo|AnswerChoice|AnswerTag|AnswerScore> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<list<Image>> */ public function images(): Presence { return $this->field('images'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<list<RankMember>> */ public function members(): Presence { return $this->field('members'); }
    /** @return Presence<Meta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<QuestionDocumentDecide|QuestionDocumentChoose|QuestionDocumentTag|QuestionDocumentScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function question_name(): Presence { return $this->field('question_name'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<PhysicalSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<int|float|string|null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<bool> */ public function value(): Presence { return $this->field('value'); }
}
final class ScoreResult extends Node {
    /** @return Presence<AnswerYesNo|AnswerChoice|AnswerTag|AnswerScore> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<list<Image>> */ public function images(): Presence { return $this->field('images'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<list<RankMember>> */ public function members(): Presence { return $this->field('members'); }
    /** @return Presence<Meta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<QuestionDocumentDecide|QuestionDocumentChoose|QuestionDocumentTag|QuestionDocumentScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function question_name(): Presence { return $this->field('question_name'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<PhysicalSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<int|float|string|null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<int|float> */ public function value(): Presence { return $this->field('value'); }
}
final class Attempt extends Node {
    /** @return Presence<int|string> */ public function ordinal(): Presence { return $this->field('ordinal'); }
    /** @return Presence<mixed> */ public function outcome(): Presence { return $this->field('outcome'); }
    /** @return Presence<string> */ public function request_id(): Presence { return $this->field('request_id'); }
    /** @return Presence<string> */ public function request_sha256(): Presence { return $this->field('request_sha256'); }
    /** @return Presence<string> */ public function sdk_request_id(): Presence { return $this->field('sdk_request_id'); }
    /** @return Presence<int|string> */ public function server_ms(): Presence { return $this->field('server_ms'); }
    /** @return Presence<int|string> */ public function status(): Presence { return $this->field('status'); }
    /** @return Presence<int|string> */ public function wall_ms(): Presence { return $this->field('wall_ms'); }
}
final class BoundaryOdds extends Node {
    /** @return Presence<list<PieceOdds>> */ public function pieces(): Presence { return $this->field('pieces'); }
    /** @return Presence<list<BoundaryProposal>> */ public function proposals(): Presence { return $this->field('proposals'); }
}
final class BoundaryProposal extends Node {
    /** @return Presence<int|string> */ public function end(): Presence { return $this->field('end'); }
    /** @return Presence<int|string> */ public function length(): Presence { return $this->field('length'); }
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
    /** @return Presence<int|string> */ public function start(): Presence { return $this->field('start'); }
    /** @return Presence<string> */ public function text(): Presence { return $this->field('text'); }
}
final class CallError extends Node {
    /** @return Presence<Error> */ public function error(): Presence { return $this->field('error'); }
    /** @return Presence<Facts> */ public function facts(): Presence { return $this->field('facts'); }
}
final class EntityDocument extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
}
final class Error extends Node {
    /** @return Presence<EstimatedInputDenialInitialRequest|EstimatedInputDenialAdditionalRequest|EstimatedInputDenialRetry> */ public function estimated_input_denial(): Presence { return $this->field('estimated_input_denial'); }
    /** @return Presence<mixed> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string> */ public function message(): Presence { return $this->field('message'); }
    /** @return Presence<bool> */ public function retryable(): Presence { return $this->field('retryable'); }
    /** @return Presence<SendBudgetDenialBeforeFirstSend|SendBudgetDenialBeforeAdditionalSend|SendBudgetDenialBeforeRetry> */ public function send_budget_denial(): Presence { return $this->field('send_budget_denial'); }
    /** @return Presence<Stopped> */ public function stopped(): Presence { return $this->field('stopped'); }
}
final class EstimatedInputDenialAdditionalRequest extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|string> */ public function limit(): Presence { return $this->field('limit'); }
}
final class EstimatedInputDenialInitialRequest extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|string> */ public function limit(): Presence { return $this->field('limit'); }
}
final class EstimatedInputDenialRetry extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|string> */ public function last_status(): Presence { return $this->field('last_status'); }
    /** @return Presence<int|string> */ public function limit(): Presence { return $this->field('limit'); }
}
final class Facts extends Node {
    /** @return Presence<list<Attempt>> */ public function attempts(): Presence { return $this->field('attempts'); }
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
    /** @return Presence<PersistenceObservation> */ public function usage_persistence(): Presence { return $this->field('usage_persistence'); }
}
final class Find extends Node {
    /** @return Presence<FindAnswer> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<list<FindCandidate>> */ public function candidates(): Presence { return $this->field('candidates'); }
    /** @return Presence<string> */ public function file(): Presence { return $this->field('file'); }
    /** @return Presence<int|string> */ public function first_line(): Presence { return $this->field('first_line'); }
    /** @return Presence<int|string|null> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<int|string> */ public function last_line(): Presence { return $this->field('last_line'); }
    /** @return Presence<Meta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<Position> */ public function position(): Presence { return $this->field('position'); }
    /** @return Presence<FindQuestionDocument> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<mixed> */ public function value(): Presence { return $this->field('value'); }
}
final class FindCandidate extends Node {
    /** @return Presence<int|string|null> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
    /** @return Presence<PhysicalSource> */ public function source(): Presence { return $this->field('source'); }
}
final class Image extends Node {
    /** @return Presence<string> */ public function base64(): Presence { return $this->field('base64'); }
    /** @return Presence<int|string> */ public function height(): Presence { return $this->field('height'); }
    /** @return Presence<mixed> */ public function media(): Presence { return $this->field('media'); }
    /** @return Presence<int|string> */ public function width(): Presence { return $this->field('width'); }
}
final class FailureFacts extends Node {
    /** @return Presence<list<Attempt>> */ public function attempts(): Presence { return $this->field('attempts'); }
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
    /** @return Presence<PersistenceObservation> */ public function usage_persistence(): Presence { return $this->field('usage_persistence'); }
}
final class InputDeclarationObjectRoot extends Node {
    /** @return Presence<\stdClass> */ public function properties(): Presence { return $this->field('properties'); }
    /** @return Presence<list<string>> */ public function required(): Presence { return $this->field('required'); }
    /** @return Presence<string> */ public function type(): Presence { return $this->field('type'); }
}
final class InputDeclarationStringRoot extends Node {
    /** @return Presence<string> */ public function type(): Presence { return $this->field('type'); }
}
final class InputPropertyTypeArray extends Node {
    /** @return Presence<StringRoot> */ public function items(): Presence { return $this->field('items'); }
    /** @return Presence<string> */ public function type(): Presence { return $this->field('type'); }
}
final class InputPropertyTypeBoolean extends Node {
    /** @return Presence<string> */ public function type(): Presence { return $this->field('type'); }
}
final class InputPropertyTypeNumber extends Node {
    /** @return Presence<string> */ public function type(): Presence { return $this->field('type'); }
}
final class InputPropertyTypeString extends Node {
    /** @return Presence<string> */ public function type(): Presence { return $this->field('type'); }
}
final class Label extends Node {
    /** @return Presence<mixed> */ public function description(): Presence { return $this->field('description'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
}
final class Meta extends Node {
    /** @return Presence<string> */ public function answered_by(): Presence { return $this->field('answered_by'); }
    /** @return Presence<list<Attempt>> */ public function attempts(): Presence { return $this->field('attempts'); }
    /** @return Presence<int|string|string> */ public function batch_setting(): Presence { return $this->field('batch_setting'); }
    /** @return Presence<BatchWarning> */ public function batch_warning(): Presence { return $this->field('batch_warning'); }
    /** @return Presence<bool> */ public function cached(): Presence { return $this->field('cached'); }
    /** @return Presence<string> */ public function context_sha256(): Presence { return $this->field('context_sha256'); }
    /** @return Presence<int|string> */ public function failed_questions(): Presence { return $this->field('failed_questions'); }
    /** @return Presence<string> */ public function model(): Presence { return $this->field('model'); }
    /** @return Presence<list<ObservationObservationId|ObservationFailureId>> */ public function observations(): Presence { return $this->field('observations'); }
    /** @return Presence<mixed|null> */ public function origin(): Presence { return $this->field('origin'); }
    /** @return Presence<ProfileWarning> */ public function profile_warning(): Presence { return $this->field('profile_warning'); }
    /** @return Presence<string> */ public function question_sha256(): Presence { return $this->field('question_sha256'); }
    /** @return Presence<list<QuestionSource>> */ public function question_sources(): Presence { return $this->field('question_sources'); }
    /** @return Presence<string> */ public function questions_sha256(): Presence { return $this->field('questions_sha256'); }
    /** @return Presence<list<string>> */ public function requests(): Presence { return $this->field('requests'); }
    /** @return Presence<int|string> */ public function requests_sent(): Presence { return $this->field('requests_sent'); }
    /** @return Presence<string> */ public function tool(): Presence { return $this->field('tool'); }
    /** @return Presence<string> */ public function url(): Presence { return $this->field('url'); }
    /** @return Presence<Usage> */ public function usage(): Presence { return $this->field('usage'); }
}
final class ObjectRoot extends Node {
    /** @return Presence<\stdClass> */ public function properties(): Presence { return $this->field('properties'); }
    /** @return Presence<list<string>> */ public function required(): Presence { return $this->field('required'); }
    /** @return Presence<string> */ public function type(): Presence { return $this->field('type'); }
}
final class ObservationFailureId extends Node {
    /** @return Presence<string> */ public function failure_id(): Presence { return $this->field('failure_id'); }
}
final class ObservationObservationId extends Node {
    /** @return Presence<string> */ public function observation_id(): Presence { return $this->field('observation_id'); }
}
final class PersistenceObservation extends Node {
    /** @return Presence<string> */ public function advice(): Presence { return $this->field('advice'); }
    /** @return Presence<string> */ public function observed_at(): Presence { return $this->field('observed_at'); }
    /** @return Presence<mixed> */ public function state(): Presence { return $this->field('state'); }
}
final class PhysicalSource extends Node {
    /** @return Presence<string> */ public function file(): Presence { return $this->field('file'); }
    /** @return Presence<int|string> */ public function first_line(): Presence { return $this->field('first_line'); }
    /** @return Presence<int|string> */ public function last_line(): Presence { return $this->field('last_line'); }
}
final class Position extends Node {
    /** @return Presence<string|null> */ public function file(): Presence { return $this->field('file'); }
    /** @return Presence<int|string> */ public function first(): Presence { return $this->field('first'); }
    /** @return Presence<list<string>> */ public function images(): Presence { return $this->field('images'); }
    /** @return Presence<int|string> */ public function last(): Presence { return $this->field('last'); }
}
final class QuestionSource extends Node {
    /** @return Presence<string> */ public function answered_by(): Presence { return $this->field('answered_by'); }
    /** @return Presence<int|string> */ public function batch_size(): Presence { return $this->field('batch_size'); }
    /** @return Presence<mixed> */ public function origin(): Presence { return $this->field('origin'); }
}
final class RankMember extends Node {
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<RankMemberResult> */ public function result(): Presence { return $this->field('result'); }
}
final class RankMemberResult extends Node {
    /** @return Presence<AnswerYesNo|AnswerChoice|AnswerTag|AnswerScore> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<list<Image>> */ public function images(): Presence { return $this->field('images'); }
    /** @return Presence<Meta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<QuestionDocumentDecide|QuestionDocumentChoose|QuestionDocumentTag|QuestionDocumentScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<PhysicalSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<null> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<int|string> */ public function value(): Presence { return $this->field('value'); }
}
final class Recognition extends Node {
    /** @return Presence<RecognitionOddsRecognizeAnswer|RecognitionOddsBoundaryOdds> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<string> */ public function file(): Presence { return $this->field('file'); }
    /** @return Presence<int|string> */ public function first_line(): Presence { return $this->field('first_line'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<int|string> */ public function last_line(): Presence { return $this->field('last_line'); }
    /** @return Presence<Meta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<Position> */ public function position(): Presence { return $this->field('position'); }
    /** @return Presence<RecognizeQuestionDocument> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<PhysicalSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<RecognizeEntities|RecognizeModeProposals> */ public function value(): Presence { return $this->field('value'); }
}
final class RecognitionEdgeDocument extends Node {
    /** @return Presence<bool> */ public function either(): Presence { return $this->field('either'); }
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
    /** @return Presence<string> */ public function relation(): Presence { return $this->field('relation'); }
    /** @return Presence<Entity> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<Entity> */ public function target(): Presence { return $this->field('target'); }
}
final class RecognitionOddsBoundaryOdds extends Node {
    /** @return Presence<list<PieceOdds>> */ public function pieces(): Presence { return $this->field('pieces'); }
    /** @return Presence<list<BoundaryProposal>> */ public function proposals(): Presence { return $this->field('proposals'); }
}
final class RecognitionOddsRecognizeAnswer extends Node {
    /** @return Presence<list<NameOdds>> */ public function names(): Presence { return $this->field('names'); }
    /** @return Presence<list<PairOdds>> */ public function pairs(): Presence { return $this->field('pairs'); }
    /** @return Presence<list<PieceOdds>> */ public function pieces(): Presence { return $this->field('pieces'); }
    /** @return Presence<list<RecognitionProposal>> */ public function proposals(): Presence { return $this->field('proposals'); }
}
final class RecognitionProposal extends Node {
    /** @return Presence<int|string> */ public function end(): Presence { return $this->field('end'); }
    /** @return Presence<bool> */ public function kept(): Presence { return $this->field('kept'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<Place> */ public function selected(): Presence { return $this->field('selected'); }
    /** @return Presence<int|float> */ public function span_probability(): Presence { return $this->field('span_probability'); }
    /** @return Presence<int|string> */ public function start(): Presence { return $this->field('start'); }
    /** @return Presence<int|float> */ public function strength(): Presence { return $this->field('strength'); }
}
final class RecognitionStageContext extends Node {
    /** @return Presence<string> */ public function boundary(): Presence { return $this->field('boundary'); }
    /** @return Presence<string> */ public function kind_edge(): Presence { return $this->field('kind_edge'); }
    /** @return Presence<string> */ public function relation(): Presence { return $this->field('relation'); }
}
final class Relation extends Node {
    /** @return Presence<Answers> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<string> */ public function file(): Presence { return $this->field('file'); }
    /** @return Presence<int|string> */ public function first_line(): Presence { return $this->field('first_line'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<list<SessionInputSource>> */ public function input_sources(): Presence { return $this->field('input_sources'); }
    /** @return Presence<int|string> */ public function last_line(): Presence { return $this->field('last_line'); }
    /** @return Presence<Meta> */ public function meta(): Presence { return $this->field('meta'); }
    /** @return Presence<Position> */ public function position(): Presence { return $this->field('position'); }
    /** @return Presence<RelateQuestionDocument> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function schema(): Presence { return $this->field('schema'); }
    /** @return Presence<list<RelatedEntityEdge>> */ public function value(): Presence { return $this->field('value'); }
}
final class RelationEndpointRelatedEntity extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
}
final class RelationEndpointSource extends Node {
    /** @return Presence<string|null> */ public function file(): Presence { return $this->field('file'); }
    /** @return Presence<int|string> */ public function first_line(): Presence { return $this->field('first_line'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|string> */ public function last_line(): Presence { return $this->field('last_line'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<int|string> */ public function ordinal(): Presence { return $this->field('ordinal'); }
    /** @return Presence<mixed> */ public function record(): Presence { return $this->field('record'); }
}
final class RelationMemberAnswerId extends Node {
    /** @return Presence<mixed> */ public function direction(): Presence { return $this->field('direction'); }
    /** @return Presence<mixed> */ public function method(): Presence { return $this->field('method'); }
    /** @return Presence<list<ObservationObservationId|ObservationFailureId>> */ public function observations(): Presence { return $this->field('observations'); }
    /** @return Presence<QuestionDocumentDecide|QuestionDocumentChoose|QuestionDocumentTag|QuestionDocumentScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<list<QuestionSource>> */ public function question_sources(): Presence { return $this->field('question_sources'); }
    /** @return Presence<string> */ public function reads(): Presence { return $this->field('reads'); }
    /** @return Presence<string> */ public function relation(): Presence { return $this->field('relation'); }
    /** @return Presence<string> */ public function request(): Presence { return $this->field('request'); }
    /** @return Presence<RelatedEntity> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<RelatedEntity|null> */ public function target(): Presence { return $this->field('target'); }
    /** @return Presence<int|float|string> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<Usage> */ public function usage(): Presence { return $this->field('usage'); }
    /** @return Presence<bool> */ public function accepted(): Presence { return $this->field('accepted'); }
    /** @return Presence<AnswerYesNo|AnswerChoice|AnswerTag|AnswerScore> */ public function answer(): Presence { return $this->field('answer'); }
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
    public function sourceName(): string { return $this->source->name; }
}
final class RelationMemberFailureId extends Node {
    /** @return Presence<mixed> */ public function direction(): Presence { return $this->field('direction'); }
    /** @return Presence<mixed> */ public function method(): Presence { return $this->field('method'); }
    /** @return Presence<list<ObservationObservationId|ObservationFailureId>> */ public function observations(): Presence { return $this->field('observations'); }
    /** @return Presence<QuestionDocumentDecide|QuestionDocumentChoose|QuestionDocumentTag|QuestionDocumentScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<list<QuestionSource>> */ public function question_sources(): Presence { return $this->field('question_sources'); }
    /** @return Presence<string> */ public function reads(): Presence { return $this->field('reads'); }
    /** @return Presence<string> */ public function relation(): Presence { return $this->field('relation'); }
    /** @return Presence<string> */ public function request(): Presence { return $this->field('request'); }
    /** @return Presence<RelatedEntity> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<RelatedEntity|null> */ public function target(): Presence { return $this->field('target'); }
    /** @return Presence<int|float|string> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<Usage> */ public function usage(): Presence { return $this->field('usage'); }
    /** @return Presence<Failure> */ public function failure(): Presence { return $this->field('failure'); }
    /** @return Presence<string> */ public function failure_id(): Presence { return $this->field('failure_id'); }
    public function sourceName(): string { return $this->source->name; }
}
final class SendBudgetDenialBeforeAdditionalSend extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
}
final class SendBudgetDenialBeforeFirstSend extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
}
final class SendBudgetDenialBeforeRetry extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|string> */ public function last_status(): Presence { return $this->field('last_status'); }
}
final class Stopped extends Node {
    /** @return Presence<int|string> */ public function at(): Presence { return $this->field('at'); }
    /** @return Presence<mixed> */ public function cause(): Presence { return $this->field('cause'); }
    /** @return Presence<bool> */ public function retryable(): Presence { return $this->field('retryable'); }
    /** @return Presence<int|string> */ public function status(): Presence { return $this->field('status'); }
}
final class StringRoot extends Node {
    /** @return Presence<string> */ public function type(): Presence { return $this->field('type'); }
}
final class Usage extends Node {
    /** @return Presence<int|string> */ public function input_tokens(): Presence { return $this->field('input_tokens'); }
    /** @return Presence<int|string> */ public function output_tokens(): Presence { return $this->field('output_tokens'); }
}
final class AnswerChoice extends Node {
    /** @return Presence<int|float> */ public function confidence(): Presence { return $this->field('confidence'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string> */ public function pick(): Presence { return $this->field('pick'); }
    /** @return Presence<\stdClass> */ public function probabilities(): Presence { return $this->field('probabilities'); }
}
final class AnswerScore extends Node {
    /** @return Presence<int|float> */ public function confidence(): Presence { return $this->field('confidence'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string> */ public function level(): Presence { return $this->field('level'); }
    /** @return Presence<\stdClass> */ public function probabilities(): Presence { return $this->field('probabilities'); }
}
final class AnswerTag extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<\stdClass> */ public function probabilities(): Presence { return $this->field('probabilities'); }
}
final class AnswerYesNo extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
}
final class BatchWarning extends Node {
    /** @return Presence<int|string|string> */ public function running(): Presence { return $this->field('running'); }
    /** @return Presence<int|string|string> */ public function tuned_for(): Presence { return $this->field('tuned_for'); }
}
final class Entity extends Node {
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
final class EntityEdge extends Node {
    /** @return Presence<bool> */ public function either(): Presence { return $this->field('either'); }
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
    /** @return Presence<string> */ public function relation(): Presence { return $this->field('relation'); }
    /** @return Presence<Entity> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<Entity> */ public function target(): Presence { return $this->field('target'); }
}
final class Failed extends Node {
    /** @return Presence<Failure> */ public function failed(): Presence { return $this->field('failed'); }
}
final class Failure extends Node {
    /** @return Presence<mixed> */ public function cause(): Presence { return $this->field('cause'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
}
final class FindAnswer extends Node {
    /** @return Presence<int|float> */ public function confidence(): Presence { return $this->field('confidence'); }
    /** @return Presence<mixed> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string> */ public function pick(): Presence { return $this->field('pick'); }
    /** @return Presence<\stdClass> */ public function probabilities(): Presence { return $this->field('probabilities'); }
}
final class FindQuestionDocument extends Node {
    /** @return Presence<int|string|string> */ public function batch(): Presence { return $this->field('batch'); }
    /** @return Presence<InputDeclarationStringRoot|InputDeclarationObjectRoot> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<InputDeclarationStringRoot|InputDeclarationObjectRoot> */ public function item_schema(): Presence { return $this->field('item_schema'); }
    /** @return Presence<list<Label>> */ public function label_details(): Presence { return $this->field('label_details'); }
    /** @return Presence<string> */ public function model(): Presence { return $this->field('model'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<bool> */ public function none(): Presence { return $this->field('none'); }
    /** @return Presence<list<string>> */ public function on(): Presence { return $this->field('on'); }
    /** @return Presence<string> */ public function profile(): Presence { return $this->field('profile'); }
    /** @return Presence<mixed> */ public function text(): Presence { return $this->field('text'); }
    /** @return Presence<mixed> */ public function verb(): Presence { return $this->field('verb'); }
    /** @return Presence<int|string> */ public function wording_version(): Presence { return $this->field('wording_version'); }
}
final class NameOdds extends Node {
    /** @return Presence<\stdClass|null> */ public function edges(): Presence { return $this->field('edges'); }
    /** @return Presence<int|string> */ public function end(): Presence { return $this->field('end'); }
    /** @return Presence<\stdClass|null> */ public function kinds(): Presence { return $this->field('kinds'); }
    /** @return Presence<int|string> */ public function start(): Presence { return $this->field('start'); }
}
final class PairOdds extends Node {
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
    /** @return Presence<string> */ public function relation(): Presence { return $this->field('relation'); }
    /** @return Presence<Place> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<Place> */ public function target(): Presence { return $this->field('target'); }
}
final class PieceOdds extends Node {
    /** @return Presence<int|string> */ public function end(): Presence { return $this->field('end'); }
    /** @return Presence<int|string> */ public function start(): Presence { return $this->field('start'); }
    /** @return Presence<\stdClass> */ public function tags(): Presence { return $this->field('tags'); }
}
final class Place extends Node {
    /** @return Presence<int|string> */ public function end(): Presence { return $this->field('end'); }
    /** @return Presence<int|string> */ public function start(): Presence { return $this->field('start'); }
}
final class ProfileWarning extends Node {
    /** @return Presence<string> */ public function running(): Presence { return $this->field('running'); }
    /** @return Presence<string> */ public function tuned_for(): Presence { return $this->field('tuned_for'); }
}
final class QuestionDocumentChoose extends Node {
    /** @return Presence<int|string|string> */ public function batch(): Presence { return $this->field('batch'); }
    /** @return Presence<InputDeclarationStringRoot|InputDeclarationObjectRoot> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<InputDeclarationStringRoot|InputDeclarationObjectRoot> */ public function item_schema(): Presence { return $this->field('item_schema'); }
    /** @return Presence<list<Label>> */ public function label_details(): Presence { return $this->field('label_details'); }
    /** @return Presence<string> */ public function model(): Presence { return $this->field('model'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<list<string>> */ public function on(): Presence { return $this->field('on'); }
    /** @return Presence<string> */ public function profile(): Presence { return $this->field('profile'); }
    /** @return Presence<int|string> */ public function wording_version(): Presence { return $this->field('wording_version'); }
    /** @return Presence<list<string>> */ public function options(): Presence { return $this->field('options'); }
    /** @return Presence<mixed> */ public function text(): Presence { return $this->field('text'); }
    /** @return Presence<string> */ public function verb(): Presence { return $this->field('verb'); }
}
final class QuestionDocumentDecide extends Node {
    /** @return Presence<int|string|string> */ public function batch(): Presence { return $this->field('batch'); }
    /** @return Presence<InputDeclarationStringRoot|InputDeclarationObjectRoot> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<InputDeclarationStringRoot|InputDeclarationObjectRoot> */ public function item_schema(): Presence { return $this->field('item_schema'); }
    /** @return Presence<list<Label>> */ public function label_details(): Presence { return $this->field('label_details'); }
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
final class QuestionDocumentScore extends Node {
    /** @return Presence<int|string|string> */ public function batch(): Presence { return $this->field('batch'); }
    /** @return Presence<InputDeclarationStringRoot|InputDeclarationObjectRoot> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<InputDeclarationStringRoot|InputDeclarationObjectRoot> */ public function item_schema(): Presence { return $this->field('item_schema'); }
    /** @return Presence<list<Label>> */ public function label_details(): Presence { return $this->field('label_details'); }
    /** @return Presence<string> */ public function model(): Presence { return $this->field('model'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<list<string>> */ public function on(): Presence { return $this->field('on'); }
    /** @return Presence<string> */ public function profile(): Presence { return $this->field('profile'); }
    /** @return Presence<int|string> */ public function wording_version(): Presence { return $this->field('wording_version'); }
    /** @return Presence<list<string>> */ public function levels(): Presence { return $this->field('levels'); }
    /** @return Presence<mixed> */ public function text(): Presence { return $this->field('text'); }
    /** @return Presence<string> */ public function verb(): Presence { return $this->field('verb'); }
}
final class QuestionDocumentTag extends Node {
    /** @return Presence<int|string|string> */ public function batch(): Presence { return $this->field('batch'); }
    /** @return Presence<InputDeclarationStringRoot|InputDeclarationObjectRoot> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<InputDeclarationStringRoot|InputDeclarationObjectRoot> */ public function item_schema(): Presence { return $this->field('item_schema'); }
    /** @return Presence<list<Label>> */ public function label_details(): Presence { return $this->field('label_details'); }
    /** @return Presence<string> */ public function model(): Presence { return $this->field('model'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<list<string>> */ public function on(): Presence { return $this->field('on'); }
    /** @return Presence<string> */ public function profile(): Presence { return $this->field('profile'); }
    /** @return Presence<int|string> */ public function wording_version(): Presence { return $this->field('wording_version'); }
    /** @return Presence<list<string>> */ public function labels(): Presence { return $this->field('labels'); }
    /** @return Presence<mixed> */ public function text(): Presence { return $this->field('text'); }
    /** @return Presence<string> */ public function verb(): Presence { return $this->field('verb'); }
}
final class RecognizeAnswer extends Node {
    /** @return Presence<list<NameOdds>> */ public function names(): Presence { return $this->field('names'); }
    /** @return Presence<list<PairOdds>> */ public function pairs(): Presence { return $this->field('pairs'); }
    /** @return Presence<list<PieceOdds>> */ public function pieces(): Presence { return $this->field('pieces'); }
    /** @return Presence<list<RecognitionProposal>> */ public function proposals(): Presence { return $this->field('proposals'); }
}
final class RecognizeQuestionDocument extends Node {
    /** @return Presence<int|string|string> */ public function batch(): Presence { return $this->field('batch'); }
    /** @return Presence<InputDeclarationStringRoot|InputDeclarationObjectRoot> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<mixed> */ public function entity_definition(): Presence { return $this->field('entity_definition'); }
    /** @return Presence<mixed> */ public function instructions(): Presence { return $this->field('instructions'); }
    /** @return Presence<InputDeclarationStringRoot|InputDeclarationObjectRoot> */ public function item_schema(): Presence { return $this->field('item_schema'); }
    /** @return Presence<\stdClass> */ public function kinds(): Presence { return $this->field('kinds'); }
    /** @return Presence<list<Label>> */ public function label_details(): Presence { return $this->field('label_details'); }
    /** @return Presence<mixed> */ public function mode(): Presence { return $this->field('mode'); }
    /** @return Presence<string> */ public function model(): Presence { return $this->field('model'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<list<string>> */ public function on(): Presence { return $this->field('on'); }
    /** @return Presence<string> */ public function profile(): Presence { return $this->field('profile'); }
    /** @return Presence<int|float|string> */ public function relation_threshold(): Presence { return $this->field('relation_threshold'); }
    /** @return Presence<list<RelationRule>> */ public function relations(): Presence { return $this->field('relations'); }
    /** @return Presence<int|string> */ public function snippet_pieces(): Presence { return $this->field('snippet_pieces'); }
    /** @return Presence<RecognitionStageContext> */ public function stage_context(): Presence { return $this->field('stage_context'); }
    /** @return Presence<int|float|string> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<string> */ public function verb(): Presence { return $this->field('verb'); }
    /** @return Presence<int|string> */ public function wording_version(): Presence { return $this->field('wording_version'); }
}
final class RecognizeEntities extends Node {
    /** @return Presence<list<Entity>> */ public function entities(): Presence { return $this->field('entities'); }
    /** @return Presence<list<EntityEdge>> */ public function relations(): Presence { return $this->field('relations'); }
}
final class RecognizeModeProposals extends Node {
    /** @return Presence<string> */ public function mode(): Presence { return $this->field('mode'); }
    /** @return Presence<list<BoundaryProposal>> */ public function proposals(): Presence { return $this->field('proposals'); }
}
final class RelateFields extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
}
final class RelateQuestionDocument extends Node {
    /** @return Presence<int|string|string> */ public function batch(): Presence { return $this->field('batch'); }
    /** @return Presence<InputDeclarationStringRoot|InputDeclarationObjectRoot> */ public function context_schema(): Presence { return $this->field('context_schema'); }
    /** @return Presence<RelateFields|null> */ public function fields(): Presence { return $this->field('fields'); }
    /** @return Presence<InputDeclarationStringRoot|InputDeclarationObjectRoot> */ public function item_schema(): Presence { return $this->field('item_schema'); }
    /** @return Presence<list<Label>> */ public function label_details(): Presence { return $this->field('label_details'); }
    /** @return Presence<string> */ public function model(): Presence { return $this->field('model'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<list<string>> */ public function on(): Presence { return $this->field('on'); }
    /** @return Presence<string> */ public function profile(): Presence { return $this->field('profile'); }
    /** @return Presence<list<RelationRule>> */ public function relations(): Presence { return $this->field('relations'); }
    /** @return Presence<int|float|string> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<mixed> */ public function verb(): Presence { return $this->field('verb'); }
    /** @return Presence<int|string> */ public function wording_version(): Presence { return $this->field('wording_version'); }
}
final class RelatedEntity extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
}
final class RelatedEntityEdge extends Node {
    /** @return Presence<bool> */ public function either(): Presence { return $this->field('either'); }
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
    /** @return Presence<string> */ public function relation(): Presence { return $this->field('relation'); }
    /** @return Presence<RelationEndpointRelatedEntity|RelationEndpointSource> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<RelationEndpointRelatedEntity|RelationEndpointSource> */ public function target(): Presence { return $this->field('target'); }
    public function sourceName(): string { return $this->source->name; }
    public function targetName(): string { return $this->target->name; }
}
final class RelationRule extends Node {
    /** @return Presence<bool> */ public function either(): Presence { return $this->field('either'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<string> */ public function reads(): Presence { return $this->field('reads'); }
    /** @return Presence<bool> */ public function single(): Presence { return $this->field('single'); }
    /** @return Presence<string> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<string> */ public function target(): Presence { return $this->field('target'); }
}
final class SessionAnnotation extends Node {
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<AnnotationValueDecision|AnnotationValueChoice|AnnotationValueScore|AnnotationValueTags|AnnotationValueFailed> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionInputSource extends Node {
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<PhysicalSource> */ public function source(): Presence { return $this->field('source'); }
}
final class SessionJudgmentChoice extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string|null> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionJudgmentDecision extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<bool|null> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionJudgmentScore extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|float> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionJudgmentTags extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<string>> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionNamedProbability extends Node {
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
}
final class SessionObservationQuestion extends Node {
    /** @return Presence<SessionQuestionDetail> */ public function detail(): Presence { return $this->field('detail'); }
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<string> */ public function member(): Presence { return $this->field('member'); }
    /** @return Presence<int|string> */ public function position(): Presence { return $this->field('position'); }
    /** @return Presence<string> */ public function stage(): Presence { return $this->field('stage'); }
}
final class SessionObservationRow extends Node {
    /** @return Presence<int|string> */ public function index(): Presence { return $this->field('index'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<SessionObservedRowJudgment|SessionObservedRowAnnotated|SessionObservedRowRecognized|SessionObservedRowFind|SessionObservedRowRelations> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionObservedRowAnnotated extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<SessionAnnotation>> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionObservedRowFind extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|string|null> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionObservedRowJudgment extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<SessionJudgmentDecision|SessionJudgmentChoice|SessionJudgmentScore|SessionJudgmentTags> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionObservedRowRecognized extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<SessionRecognition> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionObservedRowRelations extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<SessionRelationEdge>> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionPacketAnnotateAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<Annotation>> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionPacketAnnotateRow extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<Annotation> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionPacketChooseAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<ChoiceResult>> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionPacketChooseRow extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<ChoiceResult> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionPacketDecideAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<DecisionResult>> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionPacketDecideRow extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<DecisionResult> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionPacketFilterAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<FilterResult>> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionPacketFilterRow extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<FilterResult> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionPacketFindAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<Find> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionPacketObservation extends Node {
    /** @return Presence<mixed> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<SessionObservationQuestion|SessionObservationRow> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionPacketRankAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<RankResult>> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionPacketRecognizeAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<Recognition>> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionPacketRelateAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<Relation> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionPacketScoreAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<ScoreResult>> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionPacketScoreRow extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<ScoreResult> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionPacketTagAggregate extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<TagResult>> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionPacketTagRow extends Node {
    /** @return Presence<string> */ public function function(): Presence { return $this->field('function'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<TagResult> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionPacketTerminal extends Node {
    /** @return Presence<Facts> */ public function facts(): Presence { return $this->field('facts'); }
    /** @return Presence<CallError> */ public function failure(): Presence { return $this->field('failure'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
}
final class SessionProbabilitiesNamed extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<list<SessionNamedProbability>> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionProbabilitiesYesNo extends Node {
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|float> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionQuestionDetail extends Node {
    /** @return Presence<string> */ public function answer_id(): Presence { return $this->field('answer_id'); }
    /** @return Presence<bool> */ public function cached(): Presence { return $this->field('cached'); }
    /** @return Presence<int|float> */ public function confidence(): Presence { return $this->field('confidence'); }
    /** @return Presence<int|string> */ public function failed_questions(): Presence { return $this->field('failed_questions'); }
    /** @return Presence<Failure> */ public function failure(): Presence { return $this->field('failure'); }
    /** @return Presence<string> */ public function failure_id(): Presence { return $this->field('failure_id'); }
    /** @return Presence<mixed> */ public function input(): Presence { return $this->field('input'); }
    /** @return Presence<PhysicalSource> */ public function input_source(): Presence { return $this->field('input_source'); }
    /** @return Presence<list<SessionInputSource>> */ public function input_sources(): Presence { return $this->field('input_sources'); }
    /** @return Presence<list<mixed>> */ public function inputs(): Presence { return $this->field('inputs'); }
    /** @return Presence<string> */ public function model(): Presence { return $this->field('model'); }
    /** @return Presence<list<ObservationObservationId|ObservationFailureId>> */ public function observations(): Presence { return $this->field('observations'); }
    /** @return Presence<SessionProbabilitiesYesNo|SessionProbabilitiesNamed> */ public function probabilities(): Presence { return $this->field('probabilities'); }
    /** @return Presence<QuestionDocumentDecide|QuestionDocumentChoose|QuestionDocumentTag|QuestionDocumentScore> */ public function question(): Presence { return $this->field('question'); }
    /** @return Presence<string> */ public function question_sha256(): Presence { return $this->field('question_sha256'); }
    /** @return Presence<list<QuestionSource>> */ public function question_sources(): Presence { return $this->field('question_sources'); }
    /** @return Presence<string> */ public function raw_pick(): Presence { return $this->field('raw_pick'); }
    /** @return Presence<Usage> */ public function reported_usage(): Presence { return $this->field('reported_usage'); }
    /** @return Presence<list<string>> */ public function requests(): Presence { return $this->field('requests'); }
    /** @return Presence<int|string> */ public function requests_sent(): Presence { return $this->field('requests_sent'); }
    /** @return Presence<int|float|string> */ public function threshold(): Presence { return $this->field('threshold'); }
    /** @return Presence<string> */ public function url(): Presence { return $this->field('url'); }
    /** @return Presence<TokenUsage> */ public function usage(): Presence { return $this->field('usage'); }
    /** @return Presence<mixed> */ public function value(): Presence { return $this->field('value'); }
}
final class SessionRecognition extends Node {
    /** @return Presence<list<Entity>> */ public function entities(): Presence { return $this->field('entities'); }
    /** @return Presence<mixed> */ public function mode(): Presence { return $this->field('mode'); }
    /** @return Presence<list<BoundaryProposal>> */ public function proposals(): Presence { return $this->field('proposals'); }
    /** @return Presence<list<RecognitionEdgeDocument>> */ public function relations(): Presence { return $this->field('relations'); }
}
final class SessionRelationEdge extends Node {
    /** @return Presence<bool> */ public function either(): Presence { return $this->field('either'); }
    /** @return Presence<int|float> */ public function probability(): Presence { return $this->field('probability'); }
    /** @return Presence<string> */ public function relation(): Presence { return $this->field('relation'); }
    /** @return Presence<EntityDocument> */ public function source(): Presence { return $this->field('source'); }
    /** @return Presence<EntityDocument> */ public function target(): Presence { return $this->field('target'); }
    public function sourceName(): string { return $this->source->name; }
    public function targetName(): string { return $this->target->name; }
}
final class SourceRelationEndpoint extends Node {
    /** @return Presence<string|null> */ public function file(): Presence { return $this->field('file'); }
    /** @return Presence<int|string> */ public function first_line(): Presence { return $this->field('first_line'); }
    /** @return Presence<string> */ public function kind(): Presence { return $this->field('kind'); }
    /** @return Presence<int|string> */ public function last_line(): Presence { return $this->field('last_line'); }
    /** @return Presence<string> */ public function name(): Presence { return $this->field('name'); }
    /** @return Presence<int|string> */ public function ordinal(): Presence { return $this->field('ordinal'); }
    /** @return Presence<mixed> */ public function record(): Presence { return $this->field('record'); }
}
final class TokenUsage extends Node {
    /** @return Presence<int|string> */ public function input_tokens(): Presence { return $this->field('input_tokens'); }
    /** @return Presence<int|string> */ public function output_tokens(): Presence { return $this->field('output_tokens'); }
}
namespace ThinkThen\Internal;
final class Decoder
{
    public static function failureFacts(\FFI\CData $view): \ThinkThen\Results\FailureFacts
    {
        return self::readFailureFacts($view[0]);
    }
    public static function native(\FFI\CData $view): \ThinkThen\Results\Node
    {
        return self::readSessionPacket($view[0]);
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
    private static function readAnnotation(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer_id'] = self::readAnswerId($native->answer_id[0]);
        $values['answers'] = self::mapping($native->answers, fn($v) => self::readAnnotationMember($v[0]));
        if ($native->file->presence !== 0) $values['file'] = $native->file->presence === 1 ? null : (self::text($native->file->value));
        if ($native->first_line->presence !== 0) $values['first_line'] = $native->first_line->presence === 1 ? null : (self::unsigned($native->first_line->value));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->last_line->presence !== 0) $values['last_line'] = $native->last_line->presence === 1 ? null : (self::unsigned($native->last_line->value));
        $values['meta'] = self::readMeta($native->meta[0]);
        if ($native->position->presence !== 0) $values['position'] = $native->position->presence === 1 ? null : (self::readPosition($native->position->value[0]));
        $values['schema'] = self::readVersion($native->schema[0]);
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readPhysicalSource($native->source->value[0]));
        $values['value'] = self::readAnnotatedRow($native->value[0]);
        return new \ThinkThen\Results\Annotation((object)$values, $values);
    }
    private static function readAnnotationMember(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readAnnotationMemberAnswerId($native->data->answer_id[0]),
            2 => self::readAnnotationMemberFailureId($native->data->failure_id[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readAnnotationMemberAnswerId(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readAnswer($native->answer[0]);
        $values['answer_id'] = self::readAnswerId($native->answer_id[0]);
        $values['observations'] = self::sequence($native->observations, fn($v) => self::readObservation($v[0]));
        $values['question'] = self::readQuestionDocument($native->question[0]);
        $values['question_sources'] = self::sequence($native->question_sources, fn($v) => self::readQuestionSource($v[0]));
        $values['request'] = self::text($native->request);
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::readThreshold($native->threshold->value[0]));
        if ($native->usage->presence !== 0) $values['usage'] = $native->usage->presence === 1 ? null : (self::readUsage($native->usage->value[0]));
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : (self::readValue($native->value->value[0]));
        return new \ThinkThen\Results\AnnotationMemberAnswerId((object)$values, $values);
    }
    private static function readAnnotationMemberFailureId(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['failure'] = self::readFailure($native->failure[0]);
        $values['failure_id'] = self::readFailureId($native->failure_id[0]);
        $values['observations'] = self::sequence($native->observations, fn($v) => self::readObservation($v[0]));
        $values['question'] = self::readQuestionDocument($native->question[0]);
        $values['question_sources'] = self::sequence($native->question_sources, fn($v) => self::readQuestionSource($v[0]));
        $values['request'] = self::text($native->request);
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::readThreshold($native->threshold->value[0]));
        if ($native->usage->presence !== 0) $values['usage'] = $native->usage->presence === 1 ? null : (self::readUsage($native->usage->value[0]));
        return new \ThinkThen\Results\AnnotationMemberFailureId((object)$values, $values);
    }
    private static function readAnnotationValue(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readAnnotationValueDecision($native->data->decision[0]),
            2 => self::readAnnotationValueChoice($native->data->choice[0]),
            3 => self::readAnnotationValueScore($native->data->score[0]),
            4 => self::readAnnotationValueTags($native->data->tags[0]),
            5 => self::readAnnotationValueFailed($native->data->failed[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readAnnotationValueChoice(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : (self::text($native->value->value));
        return new \ThinkThen\Results\AnnotationValueChoice((object)$values, $values);
    }
    private static function readAnnotationValueDecision(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : ((bool)($native->value->value));
        return new \ThinkThen\Results\AnnotationValueDecision((object)$values, $values);
    }
    private static function readAnnotationValueFailed(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readFailure($native->value[0]);
        return new \ThinkThen\Results\AnnotationValueFailed((object)$values, $values);
    }
    private static function readAnnotationValueScore(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = $native->value;
        return new \ThinkThen\Results\AnnotationValueScore((object)$values, $values);
    }
    private static function readAnnotationValueTags(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::text($v));
        return new \ThinkThen\Results\AnnotationValueTags((object)$values, $values);
    }
    private static function readAnswerId(\FFI\CData $native): mixed
    {
        return self::text($native->value);
    }
    private static function readAnswers(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['questions'] = self::sequence($native->questions, fn($v) => self::readRelationMember($v[0]));
        return new \ThinkThen\Results\Answers((object)$values, $values);
    }
    private static function readTagResult(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readAnswer($native->answer[0]);
        $values['answer_id'] = self::readAnswerId($native->answer_id[0]);
        if ($native->images->presence !== 0) $values['images'] = $native->images->presence === 1 ? null : (self::sequence($native->images->value, fn($v) => self::readImage($v[0])));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->members->presence !== 0) $values['members'] = $native->members->presence === 1 ? null : (self::sequence($native->members->value, fn($v) => self::readRankMember($v[0])));
        $values['meta'] = self::readMeta($native->meta[0]);
        $values['question'] = self::readQuestionDocument($native->question[0]);
        if ($native->question_name->presence !== 0) $values['question_name'] = $native->question_name->presence === 1 ? null : (self::text($native->question_name->value));
        $values['schema'] = self::readVersion($native->schema[0]);
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readPhysicalSource($native->source->value[0]));
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::readThreshold($native->threshold->value[0]));
        $values['value'] = self::sequence($native->value, fn($v) => self::text($v));
        return new \ThinkThen\Results\TagResult((object)$values, $values);
    }
    private static function readDecisionResult(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readAnswer($native->answer[0]);
        $values['answer_id'] = self::readAnswerId($native->answer_id[0]);
        if ($native->images->presence !== 0) $values['images'] = $native->images->presence === 1 ? null : (self::sequence($native->images->value, fn($v) => self::readImage($v[0])));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->members->presence !== 0) $values['members'] = $native->members->presence === 1 ? null : (self::sequence($native->members->value, fn($v) => self::readRankMember($v[0])));
        $values['meta'] = self::readMeta($native->meta[0]);
        $values['question'] = self::readQuestionDocument($native->question[0]);
        if ($native->question_name->presence !== 0) $values['question_name'] = $native->question_name->presence === 1 ? null : (self::text($native->question_name->value));
        $values['schema'] = self::readVersion($native->schema[0]);
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readPhysicalSource($native->source->value[0]));
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::readThreshold($native->threshold->value[0]));
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : (self::readDecideValue($native->value->value[0]));
        return new \ThinkThen\Results\DecisionResult((object)$values, $values);
    }
    private static function readRankResult(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readAnswer($native->answer[0]);
        $values['answer_id'] = self::readAnswerId($native->answer_id[0]);
        if ($native->images->presence !== 0) $values['images'] = $native->images->presence === 1 ? null : (self::sequence($native->images->value, fn($v) => self::readImage($v[0])));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->members->presence !== 0) $values['members'] = $native->members->presence === 1 ? null : (self::sequence($native->members->value, fn($v) => self::readRankMember($v[0])));
        $values['meta'] = self::readMeta($native->meta[0]);
        $values['question'] = self::readQuestionDocument($native->question[0]);
        if ($native->question_name->presence !== 0) $values['question_name'] = $native->question_name->presence === 1 ? null : (self::text($native->question_name->value));
        $values['schema'] = self::readVersion($native->schema[0]);
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readPhysicalSource($native->source->value[0]));
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::readThreshold($native->threshold->value[0]));
        $values['value'] = self::unsigned($native->value);
        return new \ThinkThen\Results\RankResult((object)$values, $values);
    }
    private static function readChoiceResult(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readAnswer($native->answer[0]);
        $values['answer_id'] = self::readAnswerId($native->answer_id[0]);
        if ($native->images->presence !== 0) $values['images'] = $native->images->presence === 1 ? null : (self::sequence($native->images->value, fn($v) => self::readImage($v[0])));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->members->presence !== 0) $values['members'] = $native->members->presence === 1 ? null : (self::sequence($native->members->value, fn($v) => self::readRankMember($v[0])));
        $values['meta'] = self::readMeta($native->meta[0]);
        $values['question'] = self::readQuestionDocument($native->question[0]);
        if ($native->question_name->presence !== 0) $values['question_name'] = $native->question_name->presence === 1 ? null : (self::text($native->question_name->value));
        $values['schema'] = self::readVersion($native->schema[0]);
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readPhysicalSource($native->source->value[0]));
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::readThreshold($native->threshold->value[0]));
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : (self::text($native->value->value));
        return new \ThinkThen\Results\ChoiceResult((object)$values, $values);
    }
    private static function readFilterResult(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readAnswer($native->answer[0]);
        $values['answer_id'] = self::readAnswerId($native->answer_id[0]);
        if ($native->images->presence !== 0) $values['images'] = $native->images->presence === 1 ? null : (self::sequence($native->images->value, fn($v) => self::readImage($v[0])));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->members->presence !== 0) $values['members'] = $native->members->presence === 1 ? null : (self::sequence($native->members->value, fn($v) => self::readRankMember($v[0])));
        $values['meta'] = self::readMeta($native->meta[0]);
        $values['question'] = self::readQuestionDocument($native->question[0]);
        if ($native->question_name->presence !== 0) $values['question_name'] = $native->question_name->presence === 1 ? null : (self::text($native->question_name->value));
        $values['schema'] = self::readVersion($native->schema[0]);
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readPhysicalSource($native->source->value[0]));
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::readThreshold($native->threshold->value[0]));
        $values['value'] = (bool)($native->value);
        return new \ThinkThen\Results\FilterResult((object)$values, $values);
    }
    private static function readScoreResult(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readAnswer($native->answer[0]);
        $values['answer_id'] = self::readAnswerId($native->answer_id[0]);
        if ($native->images->presence !== 0) $values['images'] = $native->images->presence === 1 ? null : (self::sequence($native->images->value, fn($v) => self::readImage($v[0])));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->members->presence !== 0) $values['members'] = $native->members->presence === 1 ? null : (self::sequence($native->members->value, fn($v) => self::readRankMember($v[0])));
        $values['meta'] = self::readMeta($native->meta[0]);
        $values['question'] = self::readQuestionDocument($native->question[0]);
        if ($native->question_name->presence !== 0) $values['question_name'] = $native->question_name->presence === 1 ? null : (self::text($native->question_name->value));
        $values['schema'] = self::readVersion($native->schema[0]);
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readPhysicalSource($native->source->value[0]));
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::readThreshold($native->threshold->value[0]));
        $values['value'] = $native->value;
        return new \ThinkThen\Results\ScoreResult((object)$values, $values);
    }
    private static function readAttempt(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['ordinal'] = self::unsigned($native->ordinal);
        $values['outcome'] = self::readAttemptOutcome($native->outcome[0]);
        if ($native->request_id->presence !== 0) $values['request_id'] = $native->request_id->presence === 1 ? null : (self::text($native->request_id->value));
        $values['request_sha256'] = self::text($native->request_sha256);
        $values['sdk_request_id'] = self::readSdkRequestId($native->sdk_request_id[0]);
        if ($native->server_ms->presence !== 0) $values['server_ms'] = $native->server_ms->presence === 1 ? null : (self::unsigned($native->server_ms->value));
        if ($native->status->presence !== 0) $values['status'] = $native->status->presence === 1 ? null : (self::unsigned($native->status->value));
        $values['wall_ms'] = self::unsigned($native->wall_ms);
        return new \ThinkThen\Results\Attempt((object)$values, $values);
    }
    private static function readBatch(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::unsigned($native->data->integer),
            2 => self::readBatchString($native->data->string[0]),
            default => throw new \LogicException('invalid native value'),
        };
    }
    private static function readBoundaryMode(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'boundary_only',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readBoundaryOdds(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['pieces'] = self::sequence($native->pieces, fn($v) => self::readPieceOdds($v[0]));
        $values['proposals'] = self::sequence($native->proposals, fn($v) => self::readBoundaryProposal($v[0]));
        return new \ThinkThen\Results\BoundaryOdds((object)$values, $values);
    }
    private static function readBoundaryProposal(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['end'] = self::unsigned($native->end);
        $values['length'] = self::unsigned($native->length);
        $values['probability'] = $native->probability;
        $values['start'] = self::unsigned($native->start);
        $values['text'] = self::text($native->text);
        return new \ThinkThen\Results\BoundaryProposal((object)$values, $values);
    }
    private static function readCallError(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['error'] = self::readError($native->error[0]);
        if ($native->facts->presence !== 0) $values['facts'] = $native->facts->presence === 1 ? null : (self::readFacts($native->facts->value[0]));
        return new \ThinkThen\Results\CallError((object)$values, $values);
    }
    private static function readCallId(\FFI\CData $native): mixed
    {
        return self::text($native->value);
    }
    private static function readDecideValue(\FFI\CData $native): mixed
    {
        return self::jsonValue($native->value[0]);
    }
    private static function readEntityDocument(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['name'] = self::text($native->name);
        return new \ThinkThen\Results\EntityDocument((object)$values, $values);
    }
    private static function readError(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->estimated_input_denial->presence !== 0) $values['estimated_input_denial'] = $native->estimated_input_denial->presence === 1 ? null : (self::readEstimatedInputDenial($native->estimated_input_denial->value[0]));
        $values['kind'] = self::readFailureKind($native->kind[0]);
        $values['message'] = self::text($native->message);
        $values['retryable'] = (bool)($native->retryable);
        if ($native->send_budget_denial->presence !== 0) $values['send_budget_denial'] = $native->send_budget_denial->presence === 1 ? null : (self::readSendBudgetDenial($native->send_budget_denial->value[0]));
        $values['stopped'] = self::readStopped($native->stopped[0]);
        return new \ThinkThen\Results\Error((object)$values, $values);
    }
    private static function readEstimatedInputDenial(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readEstimatedInputDenialInitialRequest($native->data->initial_request[0]),
            2 => self::readEstimatedInputDenialAdditionalRequest($native->data->additional_request[0]),
            3 => self::readEstimatedInputDenialRetry($native->data->retry[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readEstimatedInputDenialAdditionalRequest(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['limit'] = self::unsigned($native->limit);
        return new \ThinkThen\Results\EstimatedInputDenialAdditionalRequest((object)$values, $values);
    }
    private static function readEstimatedInputDenialInitialRequest(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['limit'] = self::unsigned($native->limit);
        return new \ThinkThen\Results\EstimatedInputDenialInitialRequest((object)$values, $values);
    }
    private static function readEstimatedInputDenialRetry(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['last_status'] = self::unsigned($native->last_status);
        $values['limit'] = self::unsigned($native->limit);
        return new \ThinkThen\Results\EstimatedInputDenialRetry((object)$values, $values);
    }
    private static function readFacts(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->attempts->presence !== 0) $values['attempts'] = $native->attempts->presence === 1 ? null : (self::sequence($native->attempts->value, fn($v) => self::readAttempt($v[0])));
        $values['cache_answers'] = self::unsigned($native->cache_answers);
        $values['call_id'] = self::readCallId($native->call_id[0]);
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
        if ($native->usage_persistence->presence !== 0) $values['usage_persistence'] = $native->usage_persistence->presence === 1 ? null : (self::readPersistenceObservation($native->usage_persistence->value[0]));
        return new \ThinkThen\Results\Facts((object)$values, $values);
    }
    private static function readFailureId(\FFI\CData $native): mixed
    {
        return self::text($native->value);
    }
    private static function readFind(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readFindAnswer($native->answer[0]);
        $values['answer_id'] = self::readAnswerId($native->answer_id[0]);
        if ($native->candidates->presence !== 0) $values['candidates'] = $native->candidates->presence === 1 ? null : (self::sequence($native->candidates->value, fn($v) => self::readFindCandidate($v[0])));
        if ($native->file->presence !== 0) $values['file'] = $native->file->presence === 1 ? null : (self::text($native->file->value));
        if ($native->first_line->presence !== 0) $values['first_line'] = $native->first_line->presence === 1 ? null : (self::unsigned($native->first_line->value));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->last_line->presence !== 0) $values['last_line'] = $native->last_line->presence === 1 ? null : (self::unsigned($native->last_line->value));
        $values['meta'] = self::readMeta($native->meta[0]);
        if ($native->position->presence !== 0) $values['position'] = $native->position->presence === 1 ? null : (self::readPosition($native->position->value[0]));
        $values['question'] = self::readFindQuestionDocument($native->question[0]);
        $values['schema'] = self::readVersion($native->schema[0]);
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::jsonValue($native->threshold->value[0]));
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : (self::jsonValue($native->value->value[0]));
        return new \ThinkThen\Results\Find((object)$values, $values);
    }
    private static function readFindCandidate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        $values['probability'] = $native->probability;
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readPhysicalSource($native->source->value[0]));
        return new \ThinkThen\Results\FindCandidate((object)$values, $values);
    }
    private static function readImage(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['base64'] = self::text($native->base64);
        $values['height'] = self::unsigned($native->height);
        $values['media'] = self::readImageMedia($native->media[0]);
        $values['width'] = self::unsigned($native->width);
        return new \ThinkThen\Results\Image((object)$values, $values);
    }
    private static function readImageMedia(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'image/jpeg',
            2 => 'image/png',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readFailureFacts(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->attempts->presence !== 0) $values['attempts'] = $native->attempts->presence === 1 ? null : (self::sequence($native->attempts->value, fn($v) => self::readAttempt($v[0])));
        $values['cache_answers'] = self::unsigned($native->cache_answers);
        if ($native->call_id->presence !== 0) $values['call_id'] = $native->call_id->presence === 1 ? null : (self::readCallId($native->call_id->value[0]));
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
        if ($native->usage_persistence->presence !== 0) $values['usage_persistence'] = $native->usage_persistence->presence === 1 ? null : (self::readPersistenceObservation($native->usage_persistence->value[0]));
        return new \ThinkThen\Results\FailureFacts((object)$values, $values);
    }
    private static function readInputDeclaration(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readInputDeclarationStringRoot($native->data->string_root[0]),
            2 => self::readInputDeclarationObjectRoot($native->data->object_root[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readInputDeclarationObjectRoot(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['properties'] = self::mapping($native->properties, fn($v) => self::readInputPropertyType($v[0]));
        if ($native->required->presence !== 0) $values['required'] = $native->required->presence === 1 ? null : (self::sequence($native->required->value, fn($v) => self::text($v)));
        $values['type'] = self::readObjectType($native->type[0]);
        return new \ThinkThen\Results\InputDeclarationObjectRoot((object)$values, $values);
    }
    private static function readInputDeclarationStringRoot(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['type'] = self::readStringType($native->type[0]);
        return new \ThinkThen\Results\InputDeclarationStringRoot((object)$values, $values);
    }
    private static function readInputPropertyType(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readInputPropertyTypeString($native->data->string[0]),
            2 => self::readInputPropertyTypeNumber($native->data->number[0]),
            3 => self::readInputPropertyTypeBoolean($native->data->boolean[0]),
            4 => self::readInputPropertyTypeArray($native->data->array[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readInputPropertyTypeArray(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['items'] = self::readStringRoot($native->items[0]);
        $values['type'] = self::text($native->type);
        return new \ThinkThen\Results\InputPropertyTypeArray((object)$values, $values);
    }
    private static function readInputPropertyTypeBoolean(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['type'] = self::text($native->type);
        return new \ThinkThen\Results\InputPropertyTypeBoolean((object)$values, $values);
    }
    private static function readInputPropertyTypeNumber(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['type'] = self::text($native->type);
        return new \ThinkThen\Results\InputPropertyTypeNumber((object)$values, $values);
    }
    private static function readInputPropertyTypeString(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['type'] = self::text($native->type);
        return new \ThinkThen\Results\InputPropertyTypeString((object)$values, $values);
    }
    private static function readLabel(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->description->presence !== 0) $values['description'] = $native->description->presence === 1 ? null : (self::jsonValue($native->description->value[0]));
        $values['name'] = self::text($native->name);
        return new \ThinkThen\Results\Label((object)$values, $values);
    }
    private static function readMeta(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->answered_by->presence !== 0) $values['answered_by'] = $native->answered_by->presence === 1 ? null : (self::text($native->answered_by->value));
        if ($native->attempts->presence !== 0) $values['attempts'] = $native->attempts->presence === 1 ? null : (self::sequence($native->attempts->value, fn($v) => self::readAttempt($v[0])));
        if ($native->batch_setting->presence !== 0) $values['batch_setting'] = $native->batch_setting->presence === 1 ? null : (self::readBatchSetting($native->batch_setting->value[0]));
        if ($native->batch_warning->presence !== 0) $values['batch_warning'] = $native->batch_warning->presence === 1 ? null : (self::readBatchWarning($native->batch_warning->value[0]));
        $values['cached'] = (bool)($native->cached);
        if ($native->context_sha256->presence !== 0) $values['context_sha256'] = $native->context_sha256->presence === 1 ? null : (self::text($native->context_sha256->value));
        $values['failed_questions'] = self::unsigned($native->failed_questions);
        $values['model'] = self::text($native->model);
        $values['observations'] = self::sequence($native->observations, fn($v) => self::readObservation($v[0]));
        if ($native->origin->presence !== 0) $values['origin'] = $native->origin->presence === 1 ? null : (self::readOrigin($native->origin->value[0]));
        if ($native->profile_warning->presence !== 0) $values['profile_warning'] = $native->profile_warning->presence === 1 ? null : (self::readProfileWarning($native->profile_warning->value[0]));
        if ($native->question_sha256->presence !== 0) $values['question_sha256'] = $native->question_sha256->presence === 1 ? null : (self::text($native->question_sha256->value));
        $values['question_sources'] = self::sequence($native->question_sources, fn($v) => self::readQuestionSource($v[0]));
        if ($native->questions_sha256->presence !== 0) $values['questions_sha256'] = $native->questions_sha256->presence === 1 ? null : (self::text($native->questions_sha256->value));
        $values['requests'] = self::sequence($native->requests, fn($v) => self::text($v));
        $values['requests_sent'] = self::unsigned($native->requests_sent);
        $values['tool'] = self::text($native->tool);
        $values['url'] = self::text($native->url);
        if ($native->usage->presence !== 0) $values['usage'] = $native->usage->presence === 1 ? null : (self::readUsage($native->usage->value[0]));
        return new \ThinkThen\Results\Meta((object)$values, $values);
    }
    private static function readObjectRoot(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['properties'] = self::mapping($native->properties, fn($v) => self::readInputPropertyType($v[0]));
        if ($native->required->presence !== 0) $values['required'] = $native->required->presence === 1 ? null : (self::sequence($native->required->value, fn($v) => self::text($v)));
        $values['type'] = self::readObjectType($native->type[0]);
        return new \ThinkThen\Results\ObjectRoot((object)$values, $values);
    }
    private static function readObjectType(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'object',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readObservation(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readObservationObservationId($native->data->observation_id[0]),
            2 => self::readObservationFailureId($native->data->failure_id[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readObservationId(\FFI\CData $native): mixed
    {
        return self::text($native->value);
    }
    private static function readObservationFailureId(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['failure_id'] = self::readFailureId($native->failure_id[0]);
        return new \ThinkThen\Results\ObservationFailureId((object)$values, $values);
    }
    private static function readObservationObservationId(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['observation_id'] = self::readObservationId($native->observation_id[0]);
        return new \ThinkThen\Results\ObservationObservationId((object)$values, $values);
    }
    private static function readOrigin(\FFI\CData $native): mixed
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
    private static function readPersistenceObservation(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->advice->presence !== 0) $values['advice'] = $native->advice->presence === 1 ? null : (self::text($native->advice->value));
        $values['observed_at'] = self::text($native->observed_at);
        $values['state'] = self::readUsagePersistence($native->state[0]);
        return new \ThinkThen\Results\PersistenceObservation((object)$values, $values);
    }
    private static function readPhysicalSource(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['file'] = self::text($native->file);
        if ($native->first_line->presence !== 0) $values['first_line'] = $native->first_line->presence === 1 ? null : (self::unsigned($native->first_line->value));
        if ($native->last_line->presence !== 0) $values['last_line'] = $native->last_line->presence === 1 ? null : (self::unsigned($native->last_line->value));
        return new \ThinkThen\Results\PhysicalSource((object)$values, $values);
    }
    private static function readPosition(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->file->presence !== 0) $values['file'] = $native->file->presence === 1 ? null : (self::text($native->file->value));
        if ($native->first->presence !== 0) $values['first'] = $native->first->presence === 1 ? null : (self::unsigned($native->first->value));
        if ($native->images->presence !== 0) $values['images'] = $native->images->presence === 1 ? null : (self::sequence($native->images->value, fn($v) => self::text($v)));
        if ($native->last->presence !== 0) $values['last'] = $native->last->presence === 1 ? null : (self::unsigned($native->last->value));
        return new \ThinkThen\Results\Position((object)$values, $values);
    }
    private static function readQuestionName(\FFI\CData $native): mixed
    {
        return self::text($native->value);
    }
    private static function readQuestionSource(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answered_by'] = self::text($native->answered_by);
        if ($native->batch_size->presence !== 0) $values['batch_size'] = $native->batch_size->presence === 1 ? null : (self::unsigned($native->batch_size->value));
        $values['origin'] = self::readOrigin($native->origin[0]);
        return new \ThinkThen\Results\QuestionSource((object)$values, $values);
    }
    private static function readRankMember(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['name'] = self::text($native->name);
        $values['result'] = self::readRankMemberResult($native->result[0]);
        return new \ThinkThen\Results\RankMember((object)$values, $values);
    }
    private static function readRankMemberResult(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readAnswer($native->answer[0]);
        $values['answer_id'] = self::readAnswerId($native->answer_id[0]);
        if ($native->images->presence !== 0) $values['images'] = $native->images->presence === 1 ? null : (self::sequence($native->images->value, fn($v) => self::readImage($v[0])));
        $values['meta'] = self::readMeta($native->meta[0]);
        $values['question'] = self::readQuestionDocument($native->question[0]);
        $values['schema'] = self::readVersion($native->schema[0]);
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readPhysicalSource($native->source->value[0]));
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::jsonValue($native->threshold->value[0]));
        $values['value'] = self::unsigned($native->value);
        return new \ThinkThen\Results\RankMemberResult((object)$values, $values);
    }
    private static function readRecognition(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readRecognitionOdds($native->answer[0]);
        $values['answer_id'] = self::readAnswerId($native->answer_id[0]);
        if ($native->file->presence !== 0) $values['file'] = $native->file->presence === 1 ? null : (self::text($native->file->value));
        if ($native->first_line->presence !== 0) $values['first_line'] = $native->first_line->presence === 1 ? null : (self::unsigned($native->first_line->value));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->last_line->presence !== 0) $values['last_line'] = $native->last_line->presence === 1 ? null : (self::unsigned($native->last_line->value));
        $values['meta'] = self::readMeta($native->meta[0]);
        if ($native->position->presence !== 0) $values['position'] = $native->position->presence === 1 ? null : (self::readPosition($native->position->value[0]));
        $values['question'] = self::readRecognizeQuestionDocument($native->question[0]);
        $values['schema'] = self::readVersion($native->schema[0]);
        if ($native->source->presence !== 0) $values['source'] = $native->source->presence === 1 ? null : (self::readPhysicalSource($native->source->value[0]));
        $values['value'] = self::readRecognize($native->value[0]);
        return new \ThinkThen\Results\Recognition((object)$values, $values);
    }
    private static function readRecognitionEdgeDocument(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['either'] = (bool)($native->either);
        $values['probability'] = $native->probability;
        $values['relation'] = self::text($native->relation);
        $values['source'] = self::readEntity($native->source[0]);
        $values['target'] = self::readEntity($native->target[0]);
        return new \ThinkThen\Results\RecognitionEdgeDocument((object)$values, $values);
    }
    private static function readRecognitionMode(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'whole',
            2 => 'boundary_only',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readRecognitionOdds(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readRecognitionOddsRecognizeAnswer($native->data->recognize_answer[0]),
            2 => self::readRecognitionOddsBoundaryOdds($native->data->boundary_odds[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readRecognitionOddsBoundaryOdds(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['pieces'] = self::sequence($native->pieces, fn($v) => self::readPieceOdds($v[0]));
        $values['proposals'] = self::sequence($native->proposals, fn($v) => self::readBoundaryProposal($v[0]));
        return new \ThinkThen\Results\RecognitionOddsBoundaryOdds((object)$values, $values);
    }
    private static function readRecognitionOddsRecognizeAnswer(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['names'] = self::sequence($native->names, fn($v) => self::readNameOdds($v[0]));
        $values['pairs'] = self::sequence($native->pairs, fn($v) => self::readPairOdds($v[0]));
        $values['pieces'] = self::sequence($native->pieces, fn($v) => self::readPieceOdds($v[0]));
        $values['proposals'] = self::sequence($native->proposals, fn($v) => self::readRecognitionProposal($v[0]));
        return new \ThinkThen\Results\RecognitionOddsRecognizeAnswer((object)$values, $values);
    }
    private static function readRecognitionProposal(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['end'] = self::unsigned($native->end);
        $values['kept'] = (bool)($native->kept);
        if ($native->kind->presence !== 0) $values['kind'] = $native->kind->presence === 1 ? null : (self::text($native->kind->value));
        if ($native->selected->presence !== 0) $values['selected'] = $native->selected->presence === 1 ? null : (self::readPlace($native->selected->value[0]));
        $values['span_probability'] = $native->span_probability;
        $values['start'] = self::unsigned($native->start);
        if ($native->strength->presence !== 0) $values['strength'] = $native->strength->presence === 1 ? null : ($native->strength->value);
        return new \ThinkThen\Results\RecognitionProposal((object)$values, $values);
    }
    private static function readRecognitionStageContext(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->boundary->presence !== 0) $values['boundary'] = $native->boundary->presence === 1 ? null : (self::text($native->boundary->value));
        if ($native->kind_edge->presence !== 0) $values['kind_edge'] = $native->kind_edge->presence === 1 ? null : (self::text($native->kind_edge->value));
        if ($native->relation->presence !== 0) $values['relation'] = $native->relation->presence === 1 ? null : (self::text($native->relation->value));
        return new \ThinkThen\Results\RecognitionStageContext((object)$values, $values);
    }
    private static function readRelation(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['answer'] = self::readAnswers($native->answer[0]);
        $values['answer_id'] = self::readAnswerId($native->answer_id[0]);
        if ($native->file->presence !== 0) $values['file'] = $native->file->presence === 1 ? null : (self::text($native->file->value));
        if ($native->first_line->presence !== 0) $values['first_line'] = $native->first_line->presence === 1 ? null : (self::unsigned($native->first_line->value));
        if ($native->index->presence !== 0) $values['index'] = $native->index->presence === 1 ? null : (self::unsigned($native->index->value));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->input_sources->presence !== 0) $values['input_sources'] = $native->input_sources->presence === 1 ? null : (self::sequence($native->input_sources->value, fn($v) => self::readSessionInputSource($v[0])));
        if ($native->last_line->presence !== 0) $values['last_line'] = $native->last_line->presence === 1 ? null : (self::unsigned($native->last_line->value));
        $values['meta'] = self::readMeta($native->meta[0]);
        if ($native->position->presence !== 0) $values['position'] = $native->position->presence === 1 ? null : (self::readPosition($native->position->value[0]));
        $values['question'] = self::readRelateQuestionDocument($native->question[0]);
        $values['schema'] = self::readVersion($native->schema[0]);
        $values['value'] = self::sequence($native->value, fn($v) => self::readRelatedEntityEdge($v[0]));
        return new \ThinkThen\Results\Relation((object)$values, $values);
    }
    private static function readRelationDirection(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'source_to_target',
            2 => 'either',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readRelationEndpoint(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readRelationEndpointRelatedEntity($native->data->related_entity[0]),
            2 => self::readRelationEndpointSource($native->data->source[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readRelationEndpointRelatedEntity(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['name'] = self::text($native->name);
        return new \ThinkThen\Results\RelationEndpointRelatedEntity((object)$values, $values);
    }
    private static function readRelationEndpointSource(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->file->presence !== 0) $values['file'] = $native->file->presence === 1 ? null : (self::text($native->file->value));
        if ($native->first_line->presence !== 0) $values['first_line'] = $native->first_line->presence === 1 ? null : (self::unsigned($native->first_line->value));
        $values['kind'] = self::text($native->kind);
        if ($native->last_line->presence !== 0) $values['last_line'] = $native->last_line->presence === 1 ? null : (self::unsigned($native->last_line->value));
        $values['name'] = self::text($native->name);
        $values['ordinal'] = self::unsigned($native->ordinal);
        if ($native->record->presence !== 0) $values['record'] = $native->record->presence === 1 ? null : (self::jsonValue($native->record->value[0]));
        return new \ThinkThen\Results\RelationEndpointSource((object)$values, $values);
    }
    private static function readRelationMember(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readRelationMemberAnswerId($native->data->answer_id[0]),
            2 => self::readRelationMemberFailureId($native->data->failure_id[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readRelationMemberAnswerId(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['direction'] = self::readRelationDirection($native->direction[0]);
        $values['method'] = self::readRelationMethod($native->method[0]);
        $values['observations'] = self::sequence($native->observations, fn($v) => self::readObservation($v[0]));
        $values['question'] = self::readQuestionDocument($native->question[0]);
        $values['question_sources'] = self::sequence($native->question_sources, fn($v) => self::readQuestionSource($v[0]));
        $values['reads'] = self::text($native->reads);
        $values['relation'] = self::text($native->relation);
        $values['request'] = self::text($native->request);
        $values['source'] = self::readRelatedEntity($native->source[0]);
        if ($native->target->presence !== 0) $values['target'] = $native->target->presence === 1 ? null : (self::readRelatedEntity($native->target->value[0]));
        $values['threshold'] = self::readThreshold($native->threshold[0]);
        if ($native->usage->presence !== 0) $values['usage'] = $native->usage->presence === 1 ? null : (self::readUsage($native->usage->value[0]));
        $values['accepted'] = (bool)($native->accepted);
        $values['answer'] = self::readAnswer($native->answer[0]);
        $values['answer_id'] = self::readAnswerId($native->answer_id[0]);
        $values['probability'] = $native->probability;
        return new \ThinkThen\Results\RelationMemberAnswerId((object)$values, $values);
    }
    private static function readRelationMemberFailureId(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['direction'] = self::readRelationDirection($native->direction[0]);
        $values['method'] = self::readRelationMethod($native->method[0]);
        $values['observations'] = self::sequence($native->observations, fn($v) => self::readObservation($v[0]));
        $values['question'] = self::readQuestionDocument($native->question[0]);
        $values['question_sources'] = self::sequence($native->question_sources, fn($v) => self::readQuestionSource($v[0]));
        $values['reads'] = self::text($native->reads);
        $values['relation'] = self::text($native->relation);
        $values['request'] = self::text($native->request);
        $values['source'] = self::readRelatedEntity($native->source[0]);
        if ($native->target->presence !== 0) $values['target'] = $native->target->presence === 1 ? null : (self::readRelatedEntity($native->target->value[0]));
        $values['threshold'] = self::readThreshold($native->threshold[0]);
        if ($native->usage->presence !== 0) $values['usage'] = $native->usage->presence === 1 ? null : (self::readUsage($native->usage->value[0]));
        $values['failure'] = self::readFailure($native->failure[0]);
        $values['failure_id'] = self::readFailureId($native->failure_id[0]);
        return new \ThinkThen\Results\RelationMemberFailureId((object)$values, $values);
    }
    private static function readRelationMethod(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'yes_no',
            2 => 'choice',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readRequestFunction(\FFI\CData $native): mixed
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
    private static function readSdkRequestId(\FFI\CData $native): mixed
    {
        return self::text($native->value);
    }
    private static function readSendBudgetDenial(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readSendBudgetDenialBeforeFirstSend($native->data->before_first_send[0]),
            2 => self::readSendBudgetDenialBeforeAdditionalSend($native->data->before_additional_send[0]),
            3 => self::readSendBudgetDenialBeforeRetry($native->data->before_retry[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readSendBudgetDenialBeforeAdditionalSend(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        return new \ThinkThen\Results\SendBudgetDenialBeforeAdditionalSend((object)$values, $values);
    }
    private static function readSendBudgetDenialBeforeFirstSend(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        return new \ThinkThen\Results\SendBudgetDenialBeforeFirstSend((object)$values, $values);
    }
    private static function readSendBudgetDenialBeforeRetry(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['last_status'] = self::unsigned($native->last_status);
        return new \ThinkThen\Results\SendBudgetDenialBeforeRetry((object)$values, $values);
    }
    private static function readStopCause(\FFI\CData $native): mixed
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
    private static function readStopped(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->at->presence !== 0) $values['at'] = $native->at->presence === 1 ? null : (self::unsigned($native->at->value));
        $values['cause'] = self::readStopCause($native->cause[0]);
        $values['retryable'] = (bool)($native->retryable);
        if ($native->status->presence !== 0) $values['status'] = $native->status->presence === 1 ? null : (self::unsigned($native->status->value));
        return new \ThinkThen\Results\Stopped((object)$values, $values);
    }
    private static function readStringRoot(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['type'] = self::readStringType($native->type[0]);
        return new \ThinkThen\Results\StringRoot((object)$values, $values);
    }
    private static function readStringType(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'string',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readUsage(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->input_tokens->presence !== 0) $values['input_tokens'] = $native->input_tokens->presence === 1 ? null : (self::unsigned($native->input_tokens->value));
        if ($native->output_tokens->presence !== 0) $values['output_tokens'] = $native->output_tokens->presence === 1 ? null : (self::unsigned($native->output_tokens->value));
        return new \ThinkThen\Results\Usage((object)$values, $values);
    }
    private static function readUsagePersistence(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'disabled',
            2 => 'pending',
            3 => 'written',
            4 => 'failed',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readVerb(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'recognize',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readVersion(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'thinkthen.result/2',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readWordingVersion(\FFI\CData $native): mixed
    {
        return self::unsigned($native->value);
    }
    private static function readAnnotatedField(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => (bool)($native->data->boolean),
            2 => null,
            3 => self::text($native->data->string),
            4 => self::sequence($native->data->array, fn($v) => self::text($v)),
            5 => $native->data->number,
            6 => self::readFailed($native->data->object[0]),
            default => throw new \LogicException('invalid native value'),
        };
    }
    private static function readAnnotatedRow(\FFI\CData $native): mixed
    {
        return self::mapping($native->value, fn($v) => self::readAnnotatedField($v[0]));
    }
    private static function readAnswer(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readAnswerYesNo($native->data->yes_no[0]),
            2 => self::readAnswerChoice($native->data->choice[0]),
            3 => self::readAnswerTag($native->data->tag[0]),
            4 => self::readAnswerScore($native->data->score[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readAnswerChoice(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->confidence->presence !== 0) $values['confidence'] = $native->confidence->presence === 1 ? null : ($native->confidence->value);
        $values['kind'] = self::text($native->kind);
        $values['pick'] = self::text($native->pick);
        $values['probabilities'] = self::mapping($native->probabilities, fn($v) => $v);
        return new \ThinkThen\Results\AnswerChoice((object)$values, $values);
    }
    private static function readAnswerScore(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->confidence->presence !== 0) $values['confidence'] = $native->confidence->presence === 1 ? null : ($native->confidence->value);
        $values['kind'] = self::text($native->kind);
        $values['level'] = self::text($native->level);
        $values['probabilities'] = self::mapping($native->probabilities, fn($v) => $v);
        return new \ThinkThen\Results\AnswerScore((object)$values, $values);
    }
    private static function readAnswerTag(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['probabilities'] = self::mapping($native->probabilities, fn($v) => $v);
        return new \ThinkThen\Results\AnswerTag((object)$values, $values);
    }
    private static function readAnswerYesNo(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['probability'] = $native->probability;
        return new \ThinkThen\Results\AnswerYesNo((object)$values, $values);
    }
    private static function readAttemptOutcome(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'ok',
            2 => 'status',
            3 => 'transport',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readBatchSetting(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::unsigned($native->data->integer),
            2 => self::text($native->data->string),
            default => throw new \LogicException('invalid native value'),
        };
    }
    private static function readBatchWarning(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['running'] = self::readBatchSetting($native->running[0]);
        $values['tuned_for'] = self::readBatchSetting($native->tuned_for[0]);
        return new \ThinkThen\Results\BatchWarning((object)$values, $values);
    }
    private static function readEntity(\FFI\CData $native): mixed
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
        return new \ThinkThen\Results\Entity((object)$values, $values);
    }
    private static function readEntityEdge(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->either->presence !== 0) $values['either'] = $native->either->presence === 1 ? null : ((bool)($native->either->value));
        $values['probability'] = $native->probability;
        $values['relation'] = self::text($native->relation);
        $values['source'] = self::readEntity($native->source[0]);
        $values['target'] = self::readEntity($native->target[0]);
        return new \ThinkThen\Results\EntityEdge((object)$values, $values);
    }
    private static function readFailed(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['failed'] = self::readFailure($native->failed[0]);
        return new \ThinkThen\Results\Failed((object)$values, $values);
    }
    private static function readFailure(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['cause'] = self::readFailureCause($native->cause[0]);
        $values['kind'] = self::readFailureFieldKind($native->kind[0]);
        return new \ThinkThen\Results\Failure((object)$values, $values);
    }
    private static function readFailureCause(\FFI\CData $native): mixed
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
    private static function readFailureKind(\FFI\CData $native): mixed
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
    private static function readFindAnswer(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->confidence->presence !== 0) $values['confidence'] = $native->confidence->presence === 1 ? null : ($native->confidence->value);
        $values['kind'] = self::text($native->kind);
        $values['pick'] = self::text($native->pick);
        $values['probabilities'] = self::mapping($native->probabilities, fn($v) => $v);
        return new \ThinkThen\Results\FindAnswer((object)$values, $values);
    }
    private static function readFindQuestionDocument(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->batch->presence !== 0) $values['batch'] = $native->batch->presence === 1 ? null : (self::readBatch($native->batch->value[0]));
        if ($native->context_schema->presence !== 0) $values['context_schema'] = $native->context_schema->presence === 1 ? null : (self::readInputDeclaration($native->context_schema->value[0]));
        if ($native->item_schema->presence !== 0) $values['item_schema'] = $native->item_schema->presence === 1 ? null : (self::readInputDeclaration($native->item_schema->value[0]));
        if ($native->label_details->presence !== 0) $values['label_details'] = $native->label_details->presence === 1 ? null : (self::sequence($native->label_details->value, fn($v) => self::readLabel($v[0])));
        if ($native->model->presence !== 0) $values['model'] = $native->model->presence === 1 ? null : (self::text($native->model->value));
        if ($native->name->presence !== 0) $values['name'] = $native->name->presence === 1 ? null : (self::readQuestionName($native->name->value[0]));
        $values['none'] = (bool)($native->none);
        if ($native->on->presence !== 0) $values['on'] = $native->on->presence === 1 ? null : (self::sequence($native->on->value, fn($v) => self::text($v)));
        if ($native->profile->presence !== 0) $values['profile'] = $native->profile->presence === 1 ? null : (self::text($native->profile->value));
        if ($native->text->presence !== 0) $values['text'] = $native->text->presence === 1 ? null : (self::jsonValue($native->text->value[0]));
        $values['verb'] = self::text($native->verb);
        if ($native->wording_version->presence !== 0) $values['wording_version'] = $native->wording_version->presence === 1 ? null : (self::readWordingVersion($native->wording_version->value[0]));
        return new \ThinkThen\Results\FindQuestionDocument((object)$values, $values);
    }
    private static function readNameOdds(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->edges->presence !== 0) $values['edges'] = $native->edges->presence === 1 ? null : (self::mapping($native->edges->value, fn($v) => $v));
        $values['end'] = self::unsigned($native->end);
        if ($native->kinds->presence !== 0) $values['kinds'] = $native->kinds->presence === 1 ? null : (self::mapping($native->kinds->value, fn($v) => $v));
        $values['start'] = self::unsigned($native->start);
        return new \ThinkThen\Results\NameOdds((object)$values, $values);
    }
    private static function readPairOdds(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['probability'] = $native->probability;
        $values['relation'] = self::text($native->relation);
        $values['source'] = self::readPlace($native->source[0]);
        $values['target'] = self::readPlace($native->target[0]);
        return new \ThinkThen\Results\PairOdds((object)$values, $values);
    }
    private static function readPieceOdds(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['end'] = self::unsigned($native->end);
        $values['start'] = self::unsigned($native->start);
        $values['tags'] = self::mapping($native->tags, fn($v) => $v);
        return new \ThinkThen\Results\PieceOdds((object)$values, $values);
    }
    private static function readPlace(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['end'] = self::unsigned($native->end);
        $values['start'] = self::unsigned($native->start);
        return new \ThinkThen\Results\Place((object)$values, $values);
    }
    private static function readProfileWarning(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['running'] = self::text($native->running);
        $values['tuned_for'] = self::text($native->tuned_for);
        return new \ThinkThen\Results\ProfileWarning((object)$values, $values);
    }
    private static function readQuestionDocument(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readQuestionDocumentDecide($native->data->decide[0]),
            2 => self::readQuestionDocumentChoose($native->data->choose[0]),
            3 => self::readQuestionDocumentTag($native->data->tag[0]),
            4 => self::readQuestionDocumentScore($native->data->score[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readQuestionDocumentChoose(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->batch->presence !== 0) $values['batch'] = $native->batch->presence === 1 ? null : (self::readBatch($native->batch->value[0]));
        if ($native->context_schema->presence !== 0) $values['context_schema'] = $native->context_schema->presence === 1 ? null : (self::readInputDeclaration($native->context_schema->value[0]));
        if ($native->item_schema->presence !== 0) $values['item_schema'] = $native->item_schema->presence === 1 ? null : (self::readInputDeclaration($native->item_schema->value[0]));
        if ($native->label_details->presence !== 0) $values['label_details'] = $native->label_details->presence === 1 ? null : (self::sequence($native->label_details->value, fn($v) => self::readLabel($v[0])));
        if ($native->model->presence !== 0) $values['model'] = $native->model->presence === 1 ? null : (self::text($native->model->value));
        if ($native->name->presence !== 0) $values['name'] = $native->name->presence === 1 ? null : (self::readQuestionName($native->name->value[0]));
        if ($native->on->presence !== 0) $values['on'] = $native->on->presence === 1 ? null : (self::sequence($native->on->value, fn($v) => self::text($v)));
        if ($native->profile->presence !== 0) $values['profile'] = $native->profile->presence === 1 ? null : (self::text($native->profile->value));
        if ($native->wording_version->presence !== 0) $values['wording_version'] = $native->wording_version->presence === 1 ? null : (self::readWordingVersion($native->wording_version->value[0]));
        $values['options'] = self::sequence($native->options, fn($v) => self::text($v));
        if ($native->text->presence !== 0) $values['text'] = $native->text->presence === 1 ? null : (self::jsonValue($native->text->value[0]));
        $values['verb'] = self::text($native->verb);
        return new \ThinkThen\Results\QuestionDocumentChoose((object)$values, $values);
    }
    private static function readQuestionDocumentDecide(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->batch->presence !== 0) $values['batch'] = $native->batch->presence === 1 ? null : (self::readBatch($native->batch->value[0]));
        if ($native->context_schema->presence !== 0) $values['context_schema'] = $native->context_schema->presence === 1 ? null : (self::readInputDeclaration($native->context_schema->value[0]));
        if ($native->item_schema->presence !== 0) $values['item_schema'] = $native->item_schema->presence === 1 ? null : (self::readInputDeclaration($native->item_schema->value[0]));
        if ($native->label_details->presence !== 0) $values['label_details'] = $native->label_details->presence === 1 ? null : (self::sequence($native->label_details->value, fn($v) => self::readLabel($v[0])));
        if ($native->model->presence !== 0) $values['model'] = $native->model->presence === 1 ? null : (self::text($native->model->value));
        if ($native->name->presence !== 0) $values['name'] = $native->name->presence === 1 ? null : (self::readQuestionName($native->name->value[0]));
        if ($native->on->presence !== 0) $values['on'] = $native->on->presence === 1 ? null : (self::sequence($native->on->value, fn($v) => self::text($v)));
        if ($native->profile->presence !== 0) $values['profile'] = $native->profile->presence === 1 ? null : (self::text($native->profile->value));
        if ($native->wording_version->presence !== 0) $values['wording_version'] = $native->wording_version->presence === 1 ? null : (self::readWordingVersion($native->wording_version->value[0]));
        if ($native->false_->presence !== 0) $values['false'] = $native->false_->presence === 1 ? null : (self::jsonValue($native->false_->value[0]));
        if ($native->text->presence !== 0) $values['text'] = $native->text->presence === 1 ? null : (self::jsonValue($native->text->value[0]));
        if ($native->true_->presence !== 0) $values['true'] = $native->true_->presence === 1 ? null : (self::jsonValue($native->true_->value[0]));
        $values['verb'] = self::text($native->verb);
        return new \ThinkThen\Results\QuestionDocumentDecide((object)$values, $values);
    }
    private static function readQuestionDocumentScore(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->batch->presence !== 0) $values['batch'] = $native->batch->presence === 1 ? null : (self::readBatch($native->batch->value[0]));
        if ($native->context_schema->presence !== 0) $values['context_schema'] = $native->context_schema->presence === 1 ? null : (self::readInputDeclaration($native->context_schema->value[0]));
        if ($native->item_schema->presence !== 0) $values['item_schema'] = $native->item_schema->presence === 1 ? null : (self::readInputDeclaration($native->item_schema->value[0]));
        if ($native->label_details->presence !== 0) $values['label_details'] = $native->label_details->presence === 1 ? null : (self::sequence($native->label_details->value, fn($v) => self::readLabel($v[0])));
        if ($native->model->presence !== 0) $values['model'] = $native->model->presence === 1 ? null : (self::text($native->model->value));
        if ($native->name->presence !== 0) $values['name'] = $native->name->presence === 1 ? null : (self::readQuestionName($native->name->value[0]));
        if ($native->on->presence !== 0) $values['on'] = $native->on->presence === 1 ? null : (self::sequence($native->on->value, fn($v) => self::text($v)));
        if ($native->profile->presence !== 0) $values['profile'] = $native->profile->presence === 1 ? null : (self::text($native->profile->value));
        if ($native->wording_version->presence !== 0) $values['wording_version'] = $native->wording_version->presence === 1 ? null : (self::readWordingVersion($native->wording_version->value[0]));
        $values['levels'] = self::sequence($native->levels, fn($v) => self::text($v));
        if ($native->text->presence !== 0) $values['text'] = $native->text->presence === 1 ? null : (self::jsonValue($native->text->value[0]));
        $values['verb'] = self::text($native->verb);
        return new \ThinkThen\Results\QuestionDocumentScore((object)$values, $values);
    }
    private static function readQuestionDocumentTag(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->batch->presence !== 0) $values['batch'] = $native->batch->presence === 1 ? null : (self::readBatch($native->batch->value[0]));
        if ($native->context_schema->presence !== 0) $values['context_schema'] = $native->context_schema->presence === 1 ? null : (self::readInputDeclaration($native->context_schema->value[0]));
        if ($native->item_schema->presence !== 0) $values['item_schema'] = $native->item_schema->presence === 1 ? null : (self::readInputDeclaration($native->item_schema->value[0]));
        if ($native->label_details->presence !== 0) $values['label_details'] = $native->label_details->presence === 1 ? null : (self::sequence($native->label_details->value, fn($v) => self::readLabel($v[0])));
        if ($native->model->presence !== 0) $values['model'] = $native->model->presence === 1 ? null : (self::text($native->model->value));
        if ($native->name->presence !== 0) $values['name'] = $native->name->presence === 1 ? null : (self::readQuestionName($native->name->value[0]));
        if ($native->on->presence !== 0) $values['on'] = $native->on->presence === 1 ? null : (self::sequence($native->on->value, fn($v) => self::text($v)));
        if ($native->profile->presence !== 0) $values['profile'] = $native->profile->presence === 1 ? null : (self::text($native->profile->value));
        if ($native->wording_version->presence !== 0) $values['wording_version'] = $native->wording_version->presence === 1 ? null : (self::readWordingVersion($native->wording_version->value[0]));
        $values['labels'] = self::sequence($native->labels, fn($v) => self::text($v));
        if ($native->text->presence !== 0) $values['text'] = $native->text->presence === 1 ? null : (self::jsonValue($native->text->value[0]));
        $values['verb'] = self::text($native->verb);
        return new \ThinkThen\Results\QuestionDocumentTag((object)$values, $values);
    }
    private static function readRecognize(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readRecognizeEntities($native->data->entities[0]),
            2 => self::readRecognizeModeProposals($native->data->mode_proposals[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readRecognizeAnswer(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['names'] = self::sequence($native->names, fn($v) => self::readNameOdds($v[0]));
        $values['pairs'] = self::sequence($native->pairs, fn($v) => self::readPairOdds($v[0]));
        $values['pieces'] = self::sequence($native->pieces, fn($v) => self::readPieceOdds($v[0]));
        $values['proposals'] = self::sequence($native->proposals, fn($v) => self::readRecognitionProposal($v[0]));
        return new \ThinkThen\Results\RecognizeAnswer((object)$values, $values);
    }
    private static function readRecognizeQuestionDocument(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->batch->presence !== 0) $values['batch'] = $native->batch->presence === 1 ? null : (self::readBatch($native->batch->value[0]));
        if ($native->context_schema->presence !== 0) $values['context_schema'] = $native->context_schema->presence === 1 ? null : (self::readInputDeclaration($native->context_schema->value[0]));
        if ($native->entity_definition->presence !== 0) $values['entity_definition'] = $native->entity_definition->presence === 1 ? null : (self::jsonValue($native->entity_definition->value[0]));
        if ($native->instructions->presence !== 0) $values['instructions'] = $native->instructions->presence === 1 ? null : (self::jsonValue($native->instructions->value[0]));
        if ($native->item_schema->presence !== 0) $values['item_schema'] = $native->item_schema->presence === 1 ? null : (self::readInputDeclaration($native->item_schema->value[0]));
        $values['kinds'] = self::mapping($native->kinds, fn($v) => self::jsonValue($v[0]));
        if ($native->label_details->presence !== 0) $values['label_details'] = $native->label_details->presence === 1 ? null : (self::sequence($native->label_details->value, fn($v) => self::readLabel($v[0])));
        if ($native->mode->presence !== 0) $values['mode'] = $native->mode->presence === 1 ? null : (self::readRecognitionMode($native->mode->value[0]));
        if ($native->model->presence !== 0) $values['model'] = $native->model->presence === 1 ? null : (self::text($native->model->value));
        if ($native->name->presence !== 0) $values['name'] = $native->name->presence === 1 ? null : (self::readQuestionName($native->name->value[0]));
        if ($native->on->presence !== 0) $values['on'] = $native->on->presence === 1 ? null : (self::sequence($native->on->value, fn($v) => self::text($v)));
        if ($native->profile->presence !== 0) $values['profile'] = $native->profile->presence === 1 ? null : (self::text($native->profile->value));
        if ($native->relation_threshold->presence !== 0) $values['relation_threshold'] = $native->relation_threshold->presence === 1 ? null : (self::readThreshold($native->relation_threshold->value[0]));
        if ($native->relations->presence !== 0) $values['relations'] = $native->relations->presence === 1 ? null : (self::sequence($native->relations->value, fn($v) => self::readRelationRule($v[0])));
        if ($native->snippet_pieces->presence !== 0) $values['snippet_pieces'] = $native->snippet_pieces->presence === 1 ? null : (self::unsigned($native->snippet_pieces->value));
        if ($native->stage_context->presence !== 0) $values['stage_context'] = $native->stage_context->presence === 1 ? null : (self::readRecognitionStageContext($native->stage_context->value[0]));
        $values['threshold'] = self::readThreshold($native->threshold[0]);
        $values['verb'] = self::readVerb($native->verb[0]);
        if ($native->wording_version->presence !== 0) $values['wording_version'] = $native->wording_version->presence === 1 ? null : (self::readWordingVersion($native->wording_version->value[0]));
        return new \ThinkThen\Results\RecognizeQuestionDocument((object)$values, $values);
    }
    private static function readRecognizeEntities(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['entities'] = self::sequence($native->entities, fn($v) => self::readEntity($v[0]));
        if ($native->relations->presence !== 0) $values['relations'] = $native->relations->presence === 1 ? null : (self::sequence($native->relations->value, fn($v) => self::readEntityEdge($v[0])));
        return new \ThinkThen\Results\RecognizeEntities((object)$values, $values);
    }
    private static function readRecognizeModeProposals(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['mode'] = self::readBoundaryMode($native->mode[0]);
        $values['proposals'] = self::sequence($native->proposals, fn($v) => self::readBoundaryProposal($v[0]));
        return new \ThinkThen\Results\RecognizeModeProposals((object)$values, $values);
    }
    private static function readRelateFields(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['name'] = self::text($native->name);
        return new \ThinkThen\Results\RelateFields((object)$values, $values);
    }
    private static function readRelateQuestionDocument(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->batch->presence !== 0) $values['batch'] = $native->batch->presence === 1 ? null : (self::readBatch($native->batch->value[0]));
        if ($native->context_schema->presence !== 0) $values['context_schema'] = $native->context_schema->presence === 1 ? null : (self::readInputDeclaration($native->context_schema->value[0]));
        if ($native->fields->presence !== 0) $values['fields'] = $native->fields->presence === 1 ? null : (self::readRelateFields($native->fields->value[0]));
        if ($native->item_schema->presence !== 0) $values['item_schema'] = $native->item_schema->presence === 1 ? null : (self::readInputDeclaration($native->item_schema->value[0]));
        if ($native->label_details->presence !== 0) $values['label_details'] = $native->label_details->presence === 1 ? null : (self::sequence($native->label_details->value, fn($v) => self::readLabel($v[0])));
        if ($native->model->presence !== 0) $values['model'] = $native->model->presence === 1 ? null : (self::text($native->model->value));
        if ($native->name->presence !== 0) $values['name'] = $native->name->presence === 1 ? null : (self::readQuestionName($native->name->value[0]));
        if ($native->on->presence !== 0) $values['on'] = $native->on->presence === 1 ? null : (self::sequence($native->on->value, fn($v) => self::text($v)));
        if ($native->profile->presence !== 0) $values['profile'] = $native->profile->presence === 1 ? null : (self::text($native->profile->value));
        $values['relations'] = self::sequence($native->relations, fn($v) => self::readRelationRule($v[0]));
        $values['threshold'] = self::readThreshold($native->threshold[0]);
        $values['verb'] = self::text($native->verb);
        if ($native->wording_version->presence !== 0) $values['wording_version'] = $native->wording_version->presence === 1 ? null : (self::readWordingVersion($native->wording_version->value[0]));
        return new \ThinkThen\Results\RelateQuestionDocument((object)$values, $values);
    }
    private static function readRelatedEntity(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['name'] = self::text($native->name);
        return new \ThinkThen\Results\RelatedEntity((object)$values, $values);
    }
    private static function readRelatedEntityEdge(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->either->presence !== 0) $values['either'] = $native->either->presence === 1 ? null : ((bool)($native->either->value));
        $values['probability'] = $native->probability;
        $values['relation'] = self::text($native->relation);
        $values['source'] = self::readRelationEndpoint($native->source[0]);
        $values['target'] = self::readRelationEndpoint($native->target[0]);
        return new \ThinkThen\Results\RelatedEntityEdge((object)$values, $values);
    }
    private static function readRelationRule(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['either'] = (bool)($native->either);
        $values['name'] = self::text($native->name);
        $values['reads'] = self::text($native->reads);
        if ($native->single->presence !== 0) $values['single'] = $native->single->presence === 1 ? null : ((bool)($native->single->value));
        $values['source'] = self::text($native->source);
        $values['target'] = self::text($native->target);
        return new \ThinkThen\Results\RelationRule((object)$values, $values);
    }
    private static function readSessionAnnotation(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['name'] = self::text($native->name);
        $values['value'] = self::readAnnotationValue($native->value[0]);
        return new \ThinkThen\Results\SessionAnnotation((object)$values, $values);
    }
    private static function readSessionInputSource(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['index'] = self::unsigned($native->index);
        $values['source'] = self::readPhysicalSource($native->source[0]);
        return new \ThinkThen\Results\SessionInputSource((object)$values, $values);
    }
    private static function readSessionJudgment(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readSessionJudgmentDecision($native->data->decision[0]),
            2 => self::readSessionJudgmentChoice($native->data->choice[0]),
            3 => self::readSessionJudgmentScore($native->data->score[0]),
            4 => self::readSessionJudgmentTags($native->data->tags[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readSessionJudgmentChoice(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : (self::text($native->value->value));
        return new \ThinkThen\Results\SessionJudgmentChoice((object)$values, $values);
    }
    private static function readSessionJudgmentDecision(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : ((bool)($native->value->value));
        return new \ThinkThen\Results\SessionJudgmentDecision((object)$values, $values);
    }
    private static function readSessionJudgmentScore(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = $native->value;
        return new \ThinkThen\Results\SessionJudgmentScore((object)$values, $values);
    }
    private static function readSessionJudgmentTags(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::text($v));
        return new \ThinkThen\Results\SessionJudgmentTags((object)$values, $values);
    }
    private static function readSessionNamedProbability(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['name'] = self::text($native->name);
        $values['probability'] = $native->probability;
        return new \ThinkThen\Results\SessionNamedProbability((object)$values, $values);
    }
    private static function readSessionObservation(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readSessionObservationQuestion($native->data->question[0]),
            2 => self::readSessionObservationRow($native->data->row[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readSessionObservationQuestion(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['detail'] = self::readSessionQuestionDetail($native->detail[0]);
        $values['index'] = self::unsigned($native->index);
        $values['kind'] = self::text($native->kind);
        if ($native->member->presence !== 0) $values['member'] = $native->member->presence === 1 ? null : (self::text($native->member->value));
        $values['position'] = self::unsigned($native->position);
        if ($native->stage->presence !== 0) $values['stage'] = $native->stage->presence === 1 ? null : (self::text($native->stage->value));
        return new \ThinkThen\Results\SessionObservationQuestion((object)$values, $values);
    }
    private static function readSessionObservationRow(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['index'] = self::unsigned($native->index);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readSessionObservedRow($native->value[0]);
        return new \ThinkThen\Results\SessionObservationRow((object)$values, $values);
    }
    private static function readSessionObservedRow(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readSessionObservedRowJudgment($native->data->judgment[0]),
            2 => self::readSessionObservedRowAnnotated($native->data->annotated[0]),
            3 => self::readSessionObservedRowRecognized($native->data->recognized[0]),
            4 => self::readSessionObservedRowFind($native->data->find[0]),
            5 => self::readSessionObservedRowRelations($native->data->relations[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readSessionObservedRowAnnotated(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readSessionAnnotation($v[0]));
        return new \ThinkThen\Results\SessionObservedRowAnnotated((object)$values, $values);
    }
    private static function readSessionObservedRowFind(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : (self::unsigned($native->value->value));
        return new \ThinkThen\Results\SessionObservedRowFind((object)$values, $values);
    }
    private static function readSessionObservedRowJudgment(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readSessionJudgment($native->value[0]);
        return new \ThinkThen\Results\SessionObservedRowJudgment((object)$values, $values);
    }
    private static function readSessionObservedRowRecognized(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readSessionRecognition($native->value[0]);
        return new \ThinkThen\Results\SessionObservedRowRecognized((object)$values, $values);
    }
    private static function readSessionObservedRowRelations(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readSessionRelationEdge($v[0]));
        return new \ThinkThen\Results\SessionObservedRowRelations((object)$values, $values);
    }
    private static function readSessionPacket(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readSessionPacketDecideRow($native->data->decide_row[0]),
            2 => self::readSessionPacketChooseRow($native->data->choose_row[0]),
            3 => self::readSessionPacketTagRow($native->data->tag_row[0]),
            4 => self::readSessionPacketScoreRow($native->data->score_row[0]),
            5 => self::readSessionPacketFilterRow($native->data->filter_row[0]),
            6 => self::readSessionPacketAnnotateRow($native->data->annotate_row[0]),
            7 => self::readSessionPacketDecideAggregate($native->data->decide_aggregate[0]),
            8 => self::readSessionPacketChooseAggregate($native->data->choose_aggregate[0]),
            9 => self::readSessionPacketTagAggregate($native->data->tag_aggregate[0]),
            10 => self::readSessionPacketScoreAggregate($native->data->score_aggregate[0]),
            11 => self::readSessionPacketFilterAggregate($native->data->filter_aggregate[0]),
            12 => self::readSessionPacketRankAggregate($native->data->rank_aggregate[0]),
            13 => self::readSessionPacketFindAggregate($native->data->find_aggregate[0]),
            14 => self::readSessionPacketAnnotateAggregate($native->data->annotate_aggregate[0]),
            15 => self::readSessionPacketRecognizeAggregate($native->data->recognize_aggregate[0]),
            16 => self::readSessionPacketRelateAggregate($native->data->relate_aggregate[0]),
            17 => self::readSessionPacketObservation($native->data->observation[0]),
            18 => self::readSessionPacketTerminal($native->data->terminal[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readSessionPacketAnnotateAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readAnnotation($v[0]));
        return new \ThinkThen\Results\SessionPacketAnnotateAggregate((object)$values, $values);
    }
    private static function readSessionPacketAnnotateRow(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readAnnotation($native->value[0]);
        return new \ThinkThen\Results\SessionPacketAnnotateRow((object)$values, $values);
    }
    private static function readSessionPacketChooseAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readChoiceResult($v[0]));
        return new \ThinkThen\Results\SessionPacketChooseAggregate((object)$values, $values);
    }
    private static function readSessionPacketChooseRow(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readChoiceResult($native->value[0]);
        return new \ThinkThen\Results\SessionPacketChooseRow((object)$values, $values);
    }
    private static function readSessionPacketDecideAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readDecisionResult($v[0]));
        return new \ThinkThen\Results\SessionPacketDecideAggregate((object)$values, $values);
    }
    private static function readSessionPacketDecideRow(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readDecisionResult($native->value[0]);
        return new \ThinkThen\Results\SessionPacketDecideRow((object)$values, $values);
    }
    private static function readSessionPacketFilterAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readFilterResult($v[0]));
        return new \ThinkThen\Results\SessionPacketFilterAggregate((object)$values, $values);
    }
    private static function readSessionPacketFilterRow(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readFilterResult($native->value[0]);
        return new \ThinkThen\Results\SessionPacketFilterRow((object)$values, $values);
    }
    private static function readSessionPacketFindAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readFind($native->value[0]);
        return new \ThinkThen\Results\SessionPacketFindAggregate((object)$values, $values);
    }
    private static function readSessionPacketObservation(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::readRequestFunction($native->function[0]);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readSessionObservation($native->value[0]);
        return new \ThinkThen\Results\SessionPacketObservation((object)$values, $values);
    }
    private static function readSessionPacketRankAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readRankResult($v[0]));
        return new \ThinkThen\Results\SessionPacketRankAggregate((object)$values, $values);
    }
    private static function readSessionPacketRecognizeAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readRecognition($v[0]));
        return new \ThinkThen\Results\SessionPacketRecognizeAggregate((object)$values, $values);
    }
    private static function readSessionPacketRelateAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readRelation($native->value[0]);
        return new \ThinkThen\Results\SessionPacketRelateAggregate((object)$values, $values);
    }
    private static function readSessionPacketScoreAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readScoreResult($v[0]));
        return new \ThinkThen\Results\SessionPacketScoreAggregate((object)$values, $values);
    }
    private static function readSessionPacketScoreRow(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readScoreResult($native->value[0]);
        return new \ThinkThen\Results\SessionPacketScoreRow((object)$values, $values);
    }
    private static function readSessionPacketTagAggregate(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readTagResult($v[0]));
        return new \ThinkThen\Results\SessionPacketTagAggregate((object)$values, $values);
    }
    private static function readSessionPacketTagRow(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['function'] = self::text($native->function);
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::readTagResult($native->value[0]);
        return new \ThinkThen\Results\SessionPacketTagRow((object)$values, $values);
    }
    private static function readSessionPacketTerminal(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->facts->presence !== 0) $values['facts'] = $native->facts->presence === 1 ? null : (self::readFacts($native->facts->value[0]));
        if ($native->failure->presence !== 0) $values['failure'] = $native->failure->presence === 1 ? null : (self::readCallError($native->failure->value[0]));
        $values['kind'] = self::text($native->kind);
        return new \ThinkThen\Results\SessionPacketTerminal((object)$values, $values);
    }
    private static function readSessionProbabilities(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => self::readSessionProbabilitiesYesNo($native->data->yes_no[0]),
            2 => self::readSessionProbabilitiesNamed($native->data->named[0]),
            default => throw new \LogicException('invalid native discriminator'),
        };
    }
    private static function readSessionProbabilitiesNamed(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = self::sequence($native->value, fn($v) => self::readSessionNamedProbability($v[0]));
        return new \ThinkThen\Results\SessionProbabilitiesNamed((object)$values, $values);
    }
    private static function readSessionProbabilitiesYesNo(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['kind'] = self::text($native->kind);
        $values['value'] = $native->value;
        return new \ThinkThen\Results\SessionProbabilitiesYesNo((object)$values, $values);
    }
    private static function readSessionQuestionDetail(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->answer_id->presence !== 0) $values['answer_id'] = $native->answer_id->presence === 1 ? null : (self::readAnswerId($native->answer_id->value[0]));
        $values['cached'] = (bool)($native->cached);
        if ($native->confidence->presence !== 0) $values['confidence'] = $native->confidence->presence === 1 ? null : ($native->confidence->value);
        $values['failed_questions'] = self::unsigned($native->failed_questions);
        if ($native->failure->presence !== 0) $values['failure'] = $native->failure->presence === 1 ? null : (self::readFailure($native->failure->value[0]));
        if ($native->failure_id->presence !== 0) $values['failure_id'] = $native->failure_id->presence === 1 ? null : (self::readFailureId($native->failure_id->value[0]));
        if ($native->input->presence !== 0) $values['input'] = $native->input->presence === 1 ? null : (self::jsonValue($native->input->value[0]));
        if ($native->input_source->presence !== 0) $values['input_source'] = $native->input_source->presence === 1 ? null : (self::readPhysicalSource($native->input_source->value[0]));
        $values['input_sources'] = self::sequence($native->input_sources, fn($v) => self::readSessionInputSource($v[0]));
        $values['inputs'] = self::sequence($native->inputs, fn($v) => self::jsonValue($v[0]));
        $values['model'] = self::text($native->model);
        $values['observations'] = self::sequence($native->observations, fn($v) => self::readObservation($v[0]));
        if ($native->probabilities->presence !== 0) $values['probabilities'] = $native->probabilities->presence === 1 ? null : (self::readSessionProbabilities($native->probabilities->value[0]));
        $values['question'] = self::readQuestionDocument($native->question[0]);
        $values['question_sha256'] = self::text($native->question_sha256);
        $values['question_sources'] = self::sequence($native->question_sources, fn($v) => self::readQuestionSource($v[0]));
        if ($native->raw_pick->presence !== 0) $values['raw_pick'] = $native->raw_pick->presence === 1 ? null : (self::text($native->raw_pick->value));
        if ($native->reported_usage->presence !== 0) $values['reported_usage'] = $native->reported_usage->presence === 1 ? null : (self::readUsage($native->reported_usage->value[0]));
        $values['requests'] = self::sequence($native->requests, fn($v) => self::text($v));
        $values['requests_sent'] = self::unsigned($native->requests_sent);
        if ($native->threshold->presence !== 0) $values['threshold'] = $native->threshold->presence === 1 ? null : (self::readThreshold($native->threshold->value[0]));
        $values['url'] = self::text($native->url);
        if ($native->usage->presence !== 0) $values['usage'] = $native->usage->presence === 1 ? null : (self::readTokenUsage($native->usage->value[0]));
        if ($native->value->presence !== 0) $values['value'] = $native->value->presence === 1 ? null : (self::readValue($native->value->value[0]));
        return new \ThinkThen\Results\SessionQuestionDetail((object)$values, $values);
    }
    private static function readSessionRecognition(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['entities'] = self::sequence($native->entities, fn($v) => self::readEntity($v[0]));
        $values['mode'] = self::readRecognitionMode($native->mode[0]);
        if ($native->proposals->presence !== 0) $values['proposals'] = $native->proposals->presence === 1 ? null : (self::sequence($native->proposals->value, fn($v) => self::readBoundaryProposal($v[0])));
        if ($native->relations->presence !== 0) $values['relations'] = $native->relations->presence === 1 ? null : (self::sequence($native->relations->value, fn($v) => self::readRecognitionEdgeDocument($v[0])));
        return new \ThinkThen\Results\SessionRecognition((object)$values, $values);
    }
    private static function readSessionRelationEdge(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['either'] = (bool)($native->either);
        $values['probability'] = $native->probability;
        $values['relation'] = self::text($native->relation);
        $values['source'] = self::readEntityDocument($native->source[0]);
        $values['target'] = self::readEntityDocument($native->target[0]);
        return new \ThinkThen\Results\SessionRelationEdge((object)$values, $values);
    }
    private static function readSourceRelationEndpoint(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        if ($native->file->presence !== 0) $values['file'] = $native->file->presence === 1 ? null : (self::text($native->file->value));
        if ($native->first_line->presence !== 0) $values['first_line'] = $native->first_line->presence === 1 ? null : (self::unsigned($native->first_line->value));
        $values['kind'] = self::text($native->kind);
        if ($native->last_line->presence !== 0) $values['last_line'] = $native->last_line->presence === 1 ? null : (self::unsigned($native->last_line->value));
        $values['name'] = self::text($native->name);
        $values['ordinal'] = self::unsigned($native->ordinal);
        if ($native->record->presence !== 0) $values['record'] = $native->record->presence === 1 ? null : (self::jsonValue($native->record->value[0]));
        return new \ThinkThen\Results\SourceRelationEndpoint((object)$values, $values);
    }
    private static function readThreshold(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => $native->data->number,
            2 => self::text($native->data->string),
            default => throw new \LogicException('invalid native value'),
        };
    }
    private static function readTokenUsage(\FFI\CData $native): mixed
    {
        $values = (array)self::mapping($native->extensions, fn($v) => self::jsonValue($v[0]));
        $values['input_tokens'] = self::unsigned($native->input_tokens);
        $values['output_tokens'] = self::unsigned($native->output_tokens);
        return new \ThinkThen\Results\TokenUsage((object)$values, $values);
    }
    private static function readValue(\FFI\CData $native): mixed
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
    private static function readBatchString(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'max',
            default => throw new \LogicException('invalid native enum'),
        };
    }
    private static function readFailureFieldKind(\FFI\CData $native): mixed
    {
        return match ($native->kind) {
            1 => 'backend',
            default => throw new \LogicException('invalid native enum'),
        };
    }
}
