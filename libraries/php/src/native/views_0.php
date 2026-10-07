<?php
declare(strict_types=1);
namespace ThinkThen\Native;
final class LegacyAnswerView
{
    public function __construct(public readonly int $outcome, public readonly float $probability) {}
    public static function copy(\FFI\CData $v): self { return new self($v->outcome, $v->probability); }
}

final class StringView
{
    public function __construct(public readonly string $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(($v->len ? \FFI::string($v->data,$v->len) : ""), $v->len); }
}

final class StringsView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => StringView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class OptionalStringView
{
    public function __construct(public readonly int $present, public readonly ?StringView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? StringView::copy($v->value) : null)); }
}

final class OptionalSizeView
{
    public function __construct(public readonly int $present, public readonly ?int $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? $v->value : null)); }
}

final class OptionalU64View
{
    public function __construct(public readonly int $present, public readonly ?string $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? sprintf("%u",$v->value) : null)); }
}

final class OptionalU16View
{
    public function __construct(public readonly int $present, public readonly ?int $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? $v->value : null)); }
}

final class OptionalDoubleView
{
    public function __construct(public readonly int $present, public readonly ?float $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? $v->value : null)); }
}

final class OptionalDiscriminatorView
{
    public function __construct(public readonly int $present, public readonly ?int $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? $v->value : null)); }
}

final class ContentView
{
    public function __construct(public readonly int $kind, public readonly StringView $data) {}
    public static function copy(\FFI\CData $v): self { return new self($v->kind, StringView::copy($v->data)); }
}

final class OptionalContentView
{
    public function __construct(public readonly int $present, public readonly ?ContentView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? ContentView::copy($v->value) : null)); }
}

final class RuleView
{
    public function __construct(public readonly int $kind, public readonly float $low, public readonly float $high) {}
    public static function copy(\FFI\CData $v): self { return new self($v->kind, $v->low, $v->high); }
}

final class OptionalRuleView
{
    public function __construct(public readonly int $present, public readonly ?RuleView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? RuleView::copy($v->value) : null)); }
}

final class ChoiceView
{
    public function __construct(public readonly StringView $name, public readonly OptionalContentView $description, public readonly OptionalDoubleView $weight) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->name), OptionalContentView::copy($v->description), OptionalDoubleView::copy($v->weight)); }
}

final class ChoicesView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => ChoiceView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class RelationView
{
    public function __construct(public readonly StringView $name, public readonly StringView $source, public readonly StringView $target, public readonly OptionalStringView $reads, public readonly int $either, public readonly int $single) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->name), StringView::copy($v->source), StringView::copy($v->target), OptionalStringView::copy($v->reads), $v->either, $v->single); }
}

final class RelationsView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => RelationView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class QuestionMemberView
{
    public function __construct(public readonly StringView $name, public readonly ?QuestionViewView $question) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->name), (\FFI::isNull($v->question) ? null : QuestionViewView::copy($v->question[0]))); }
}

final class QuestionMembersView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => QuestionMemberView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class QuestionViewView
{
    public function __construct(public readonly int $kind, public readonly ContentView $text, public readonly OptionalContentView $yes, public readonly OptionalContentView $no, public readonly ChoicesView $choices, public readonly RuleView $threshold, public readonly RuleView $relation_threshold, public readonly OptionalStringView $model, public readonly OptionalStringView $profile, public readonly OptionalSizeView $batch, public readonly int $batch_max, public readonly int $none, public readonly StringsView $on, public readonly QuestionMembersView $members, public readonly ChoicesView $kinds, public readonly RelationsView $relations, public readonly OptionalStringView $name_pointer, public readonly OptionalStringView $kind_pointer) {}
    public static function copy(\FFI\CData $v): self { return new self($v->kind, ContentView::copy($v->text), OptionalContentView::copy($v->yes), OptionalContentView::copy($v->no), ChoicesView::copy($v->choices), RuleView::copy($v->threshold), RuleView::copy($v->relation_threshold), OptionalStringView::copy($v->model), OptionalStringView::copy($v->profile), OptionalSizeView::copy($v->batch), $v->batch_max, $v->none, StringsView::copy($v->on), QuestionMembersView::copy($v->members), ChoicesView::copy($v->kinds), RelationsView::copy($v->relations), OptionalStringView::copy($v->name_pointer), OptionalStringView::copy($v->kind_pointer)); }
}

final class OptionalQuestionView
{
    public function __construct(public readonly int $present, public readonly ?QuestionViewView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? QuestionViewView::copy($v->value) : null)); }
}

final class ImageViewView
{
    public function __construct(public readonly int $media, public readonly string $bytes, public readonly int $bytes_len, public readonly int $width, public readonly int $height, public readonly OptionalStringView $filename) {}
    public static function copy(\FFI\CData $v): self { return new self($v->media, ($v->bytes_len ? \FFI::string($v->bytes,$v->bytes_len) : ""), $v->bytes_len, $v->width, $v->height, OptionalStringView::copy($v->filename)); }
}

final class ImageViewsView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => ImageViewView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class OptionalImageViewsView
{
    public function __construct(public readonly int $present, public readonly ?ImageViewsView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? ImageViewsView::copy($v->value) : null)); }
}

final class SourceSpecView
{
    public function __construct(public readonly StringsView $paths, public readonly int $unit, public readonly int $window) {}
    public static function copy(\FFI\CData $v): self { return new self(StringsView::copy($v->paths), $v->unit, $v->window); }
}

final class DecideValueV1DataView
{
    public function __construct(public readonly ?int $boolean, public readonly ?ContentView $authored) {}
    public static function copy(\FFI\CData $v, ?string $active): self { return new self(($active === "boolean" ? $v->boolean : null), ($active === "authored" ? ContentView::copy($v->authored) : null)); }
}

final class DecideValueView
{
    public function __construct(public readonly int $kind, public readonly DecideValueV1DataView $data) {}
    public static function copy(\FFI\CData $v): self { return new self($v->kind, DecideValueV1DataView::copy($v->data,match($v->kind) {0 => null, 1 => "boolean", 2 => "authored", default => throw new \UnexpectedValueException("native discriminator") })); }
}

final class ProbabilityView
{
    public function __construct(public readonly StringView $name, public readonly float $probability) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->name), $v->probability); }
}

final class ProbabilitiesView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => ProbabilityView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class OptionalProbabilitiesView
{
    public function __construct(public readonly int $present, public readonly ?ProbabilitiesView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? ProbabilitiesView::copy($v->value) : null)); }
}

final class NamedAnswerView
{
    public function __construct(public readonly StringView $pick, public readonly ProbabilitiesView $probabilities, public readonly OptionalDoubleView $confidence) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->pick), ProbabilitiesView::copy($v->probabilities), OptionalDoubleView::copy($v->confidence)); }
}

final class ScoreAnswerView
{
    public function __construct(public readonly StringView $level, public readonly ProbabilitiesView $probabilities, public readonly OptionalDoubleView $confidence) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->level), ProbabilitiesView::copy($v->probabilities), OptionalDoubleView::copy($v->confidence)); }
}

final class AnswerV1DataView
{
    public function __construct(public readonly ?float $probability, public readonly ?NamedAnswerView $choice, public readonly ?ProbabilitiesView $tag, public readonly ?ScoreAnswerView $score, public readonly ?NamedAnswerView $find) {}
    public static function copy(\FFI\CData $v, ?string $active): self { return new self(($active === "probability" ? $v->probability : null), ($active === "choice" ? NamedAnswerView::copy($v->choice) : null), ($active === "tag" ? ProbabilitiesView::copy($v->tag) : null), ($active === "score" ? ScoreAnswerView::copy($v->score) : null), ($active === "find" ? NamedAnswerView::copy($v->find) : null)); }
}

final class AnswerView
{
    public function __construct(public readonly int $kind, public readonly AnswerV1DataView $data) {}
    public static function copy(\FFI\CData $v): self { return new self($v->kind, AnswerV1DataView::copy($v->data,match($v->kind) {1 => "probability", 2 => "choice", 3 => "tag", 4 => "score", 5, 7 => "find", default => throw new \UnexpectedValueException("native discriminator") })); }
}

final class OptionalAnswerView
{
    public function __construct(public readonly int $present, public readonly ?AnswerView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? AnswerView::copy($v->value) : null)); }
}

final class LocationView
{
    public function __construct(public readonly OptionalStringView $file, public readonly OptionalSizeView $first_line, public readonly OptionalSizeView $last_line) {}
    public static function copy(\FFI\CData $v): self { return new self(OptionalStringView::copy($v->file), OptionalSizeView::copy($v->first_line), OptionalSizeView::copy($v->last_line)); }
}

final class OptionalLocationView
{
    public function __construct(public readonly int $present, public readonly ?LocationView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? LocationView::copy($v->value) : null)); }
}

final class MemberValueV1DataView
{
    public function __construct(public readonly ?DecideValueView $decide, public readonly ?OptionalStringView $choose, public readonly ?StringsView $tag, public readonly ?float $score) {}
    public static function copy(\FFI\CData $v, ?string $active): self { return new self(($active === "decide" ? DecideValueView::copy($v->decide) : null), ($active === "choose" ? OptionalStringView::copy($v->choose) : null), ($active === "tag" ? StringsView::copy($v->tag) : null), ($active === "score" ? $v->score : null)); }
}

final class MemberValueView
{
    public function __construct(public readonly int $kind, public readonly MemberValueV1DataView $data) {}
    public static function copy(\FFI\CData $v): self { return new self($v->kind, MemberValueV1DataView::copy($v->data,match($v->kind) {1 => "decide", 2 => "choose", 3 => "tag", 4 => "score", default => throw new \UnexpectedValueException("native discriminator") })); }
}

final class MemberFailureView
{
    public function __construct(public readonly StringView $failure_id, public readonly int $cause) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->failure_id), $v->cause); }
}

final class MemberSuccessView
{
    public function __construct(public readonly StringView $answer_id, public readonly MemberValueView $value, public readonly AnswerView $answer, public readonly RuleView $threshold) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->answer_id), MemberValueView::copy($v->value), AnswerView::copy($v->answer), RuleView::copy($v->threshold)); }
}

final class MemberV1DataView
{
    public function __construct(public readonly ?MemberSuccessView $success, public readonly ?MemberFailureView $failure) {}
    public static function copy(\FFI\CData $v, ?string $active): self { return new self(($active === "success" ? MemberSuccessView::copy($v->success) : null), ($active === "failure" ? MemberFailureView::copy($v->failure) : null)); }
}

final class MemberView
{
    public function __construct(public readonly StringView $name, public readonly StringView $request, public readonly QuestionViewView $question, public readonly int $state, public readonly MemberV1DataView $data) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->name), StringView::copy($v->request), QuestionViewView::copy($v->question), $v->state, MemberV1DataView::copy($v->data,match($v->state) {1 => "success", 2 => "failure", default => throw new \UnexpectedValueException("native discriminator") })); }
}

final class MembersView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => MemberView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class EntityView
{
    public function __construct(public readonly StringView $text, public readonly int $start, public readonly int $end, public readonly int $length, public readonly StringView $kind, public readonly float $strength) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->text), $v->start, $v->end, $v->length, StringView::copy($v->kind), $v->strength); }
}

final class EntitiesView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => EntityView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class EntityEdgeView
{
    public function __construct(public readonly StringView $relation, public readonly EntityView $source, public readonly EntityView $target, public readonly float $probability, public readonly int $either) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->relation), EntityView::copy($v->source), EntityView::copy($v->target), $v->probability, $v->either); }
}

final class EntityEdgesView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => EntityEdgeView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class OptionalEntityEdgesView
{
    public function __construct(public readonly int $present, public readonly ?EntityEdgesView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? EntityEdgesView::copy($v->value) : null)); }
}

final class PlaceView
{
    public function __construct(public readonly int $start, public readonly int $end) {}
    public static function copy(\FFI\CData $v): self { return new self($v->start, $v->end); }
}

final class PieceView
{
    public function __construct(public readonly int $start, public readonly int $end, public readonly ProbabilitiesView $tags) {}
    public static function copy(\FFI\CData $v): self { return new self($v->start, $v->end, ProbabilitiesView::copy($v->tags)); }
}

final class PiecesView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => PieceView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class NameView
{
    public function __construct(public readonly int $start, public readonly int $end, public readonly OptionalProbabilitiesView $kinds, public readonly OptionalProbabilitiesView $edges) {}
    public static function copy(\FFI\CData $v): self { return new self($v->start, $v->end, OptionalProbabilitiesView::copy($v->kinds), OptionalProbabilitiesView::copy($v->edges)); }
}

final class NamesView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => NameView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class PairView
{
    public function __construct(public readonly StringView $relation, public readonly PlaceView $source, public readonly PlaceView $target, public readonly float $probability) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->relation), PlaceView::copy($v->source), PlaceView::copy($v->target), $v->probability); }
}

final class PairsView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => PairView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class RecognizeValueView
{
    public function __construct(public readonly EntitiesView $entities, public readonly OptionalEntityEdgesView $relations) {}
    public static function copy(\FFI\CData $v): self { return new self(EntitiesView::copy($v->entities), OptionalEntityEdgesView::copy($v->relations)); }
}

final class RecognizeAnswerView
{
    public function __construct(public readonly PiecesView $pieces, public readonly NamesView $names, public readonly PairsView $pairs) {}
    public static function copy(\FFI\CData $v): self { return new self(PiecesView::copy($v->pieces), NamesView::copy($v->names), PairsView::copy($v->pairs)); }
}

final class EndpointView
{
    public function __construct(public readonly StringView $name, public readonly StringView $kind) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->name), StringView::copy($v->kind)); }
}

final class OptionalEndpointView
{
    public function __construct(public readonly int $present, public readonly ?EndpointView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? EndpointView::copy($v->value) : null)); }
}

final class EdgeView
{
    public function __construct(public readonly StringView $relation, public readonly EndpointView $source, public readonly EndpointView $target, public readonly float $probability, public readonly int $either) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->relation), EndpointView::copy($v->source), EndpointView::copy($v->target), $v->probability, $v->either); }
}

final class EdgesView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => EdgeView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class RelationSuccessView
{
    public function __construct(public readonly StringView $answer_id, public readonly float $probability, public readonly int $accepted) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->answer_id), $v->probability, $v->accepted); }
}

final class RelationAnswerV1DataView
{
    public function __construct(public readonly ?RelationSuccessView $success, public readonly ?MemberFailureView $failure) {}
    public static function copy(\FFI\CData $v, ?string $active): self { return new self(($active === "success" ? RelationSuccessView::copy($v->success) : null), ($active === "failure" ? MemberFailureView::copy($v->failure) : null)); }
}

final class RelationAnswerView
{
    public function __construct(public readonly StringView $relation, public readonly StringView $reads, public readonly int $method, public readonly int $direction, public readonly EndpointView $source, public readonly OptionalEndpointView $target, public readonly StringView $request, public readonly int $state, public readonly RelationAnswerV1DataView $data) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->relation), StringView::copy($v->reads), $v->method, $v->direction, EndpointView::copy($v->source), OptionalEndpointView::copy($v->target), StringView::copy($v->request), $v->state, RelationAnswerV1DataView::copy($v->data,match($v->state) {1 => "success", 2 => "failure", default => throw new \UnexpectedValueException("native discriminator") })); }
}

final class RelationAnswersView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => RelationAnswerView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class UsageView
{
    public function __construct(public readonly string $input_tokens, public readonly string $output_tokens) {}
    public static function copy(\FFI\CData $v): self { return new self(sprintf("%u",$v->input_tokens), sprintf("%u",$v->output_tokens)); }
}

final class OptionalUsageView
{
    public function __construct(public readonly int $present, public readonly ?UsageView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? UsageView::copy($v->value) : null)); }
}

final class QuestionSourceView
{
    public function __construct(public readonly int $origin, public readonly StringView $answered_by) {}
    public static function copy(\FFI\CData $v): self { return new self($v->origin, StringView::copy($v->answered_by)); }
}

final class QuestionSourcesView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => QuestionSourceView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class ObservationIdentityV1DataView
{
    public function __construct(public readonly ?StringView $observation_id, public readonly ?StringView $failure_id) {}
    public static function copy(\FFI\CData $v, ?string $active): self { return new self(($active === "observation_id" ? StringView::copy($v->observation_id) : null), ($active === "failure_id" ? StringView::copy($v->failure_id) : null)); }
}

final class ObservationIdentityView
{
    public function __construct(public readonly int $kind, public readonly ObservationIdentityV1DataView $data) {}
    public static function copy(\FFI\CData $v): self { return new self($v->kind, ObservationIdentityV1DataView::copy($v->data,match($v->kind) {1 => "observation_id", 2 => "failure_id", default => throw new \UnexpectedValueException("native discriminator") })); }
}

final class ObservationIdentitiesView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => ObservationIdentityView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class ProfileWarningView
{
    public function __construct(public readonly StringView $tuned_for, public readonly StringView $running) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->tuned_for), StringView::copy($v->running)); }
}

final class OptionalProfileWarningView
{
    public function __construct(public readonly int $present, public readonly ?ProfileWarningView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? ProfileWarningView::copy($v->value) : null)); }
}

final class BatchView
{
    public function __construct(public readonly int $kind, public readonly int $records) {}
    public static function copy(\FFI\CData $v): self { return new self($v->kind, $v->records); }
}

final class OptionalBatchView
{
    public function __construct(public readonly int $present, public readonly ?BatchView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? BatchView::copy($v->value) : null)); }
}

final class BatchWarningView
{
    public function __construct(public readonly BatchView $tuned_for, public readonly BatchView $running) {}
    public static function copy(\FFI\CData $v): self { return new self(BatchView::copy($v->tuned_for), BatchView::copy($v->running)); }
}

final class OptionalBatchWarningView
{
    public function __construct(public readonly int $present, public readonly ?BatchWarningView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? BatchWarningView::copy($v->value) : null)); }
}

final class AttemptView
{
    public function __construct(public readonly string $ordinal, public readonly StringView $request_sha256, public readonly string $wall_ms, public readonly int $outcome, public readonly StringView $sdk_request_id, public readonly OptionalU16View $status, public readonly OptionalU64View $server_ms, public readonly OptionalStringView $request_id) {}
    public static function copy(\FFI\CData $v): self { return new self(sprintf("%u",$v->ordinal), StringView::copy($v->request_sha256), sprintf("%u",$v->wall_ms), $v->outcome, StringView::copy($v->sdk_request_id), OptionalU16View::copy($v->status), OptionalU64View::copy($v->server_ms), OptionalStringView::copy($v->request_id)); }
}

final class AttemptsView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => AttemptView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class OptionalAttemptsView
{
    public function __construct(public readonly int $present, public readonly ?AttemptsView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? AttemptsView::copy($v->value) : null)); }
}

final class MetaView
{
    public function __construct(public readonly StringView $tool, public readonly OptionalStringView $question_sha256, public readonly OptionalStringView $questions_sha256, public readonly StringView $url, public readonly StringView $model, public readonly OptionalUsageView $usage, public readonly string $requests_sent, public readonly int $cached, public readonly StringsView $requests, public readonly int $failed_questions, public readonly OptionalProfileWarningView $profile_warning, public readonly OptionalBatchView $batch_setting, public readonly OptionalBatchWarningView $batch_warning, public readonly OptionalStringView $context_sha256, public readonly OptionalAttemptsView $attempts, public readonly OptionalDiscriminatorView $origin, public readonly QuestionSourcesView $question_sources, public readonly ObservationIdentitiesView $observations, public readonly OptionalStringView $answered_by) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->tool), OptionalStringView::copy($v->question_sha256), OptionalStringView::copy($v->questions_sha256), StringView::copy($v->url), StringView::copy($v->model), OptionalUsageView::copy($v->usage), sprintf("%u",$v->requests_sent), $v->cached, StringsView::copy($v->requests), $v->failed_questions, OptionalProfileWarningView::copy($v->profile_warning), OptionalBatchView::copy($v->batch_setting), OptionalBatchWarningView::copy($v->batch_warning), OptionalStringView::copy($v->context_sha256), OptionalAttemptsView::copy($v->attempts), OptionalDiscriminatorView::copy($v->origin), QuestionSourcesView::copy($v->question_sources), ObservationIdentitiesView::copy($v->observations), OptionalStringView::copy($v->answered_by)); }
}

final class OptionalMetaView
{
    public function __construct(public readonly int $present, public readonly ?MetaView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? MetaView::copy($v->value) : null)); }
}

final class FactsView
{
    public function __construct(public readonly StringView $call_id, public readonly string $cache_answers, public readonly OptionalStringView $estimated_cost_usd, public readonly OptionalU64View $input_tokens, public readonly OptionalStringView $model, public readonly OptionalU64View $output_tokens, public readonly string $records, public readonly string $requests_sent, public readonly float $seconds, public readonly OptionalU64View $command_ms) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->call_id), sprintf("%u",$v->cache_answers), OptionalStringView::copy($v->estimated_cost_usd), OptionalU64View::copy($v->input_tokens), OptionalStringView::copy($v->model), OptionalU64View::copy($v->output_tokens), sprintf("%u",$v->records), sprintf("%u",$v->requests_sent), $v->seconds, OptionalU64View::copy($v->command_ms)); }
}

final class OptionalFactsView
{
    public function __construct(public readonly int $present, public readonly ?FactsView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? FactsView::copy($v->value) : null)); }
}
