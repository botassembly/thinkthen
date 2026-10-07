<?php
declare(strict_types=1);
namespace ThinkThen\Native;
final class StoppedView
{
    public function __construct(public readonly OptionalSizeView $at, public readonly int $cause, public readonly OptionalU16View $status, public readonly int $retryable) {}
    public static function copy(\FFI\CData $v): self { return new self(OptionalSizeView::copy($v->at), $v->cause, OptionalU16View::copy($v->status), $v->retryable); }
}

final class OptionalStoppedView
{
    public function __construct(public readonly int $present, public readonly ?StoppedView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? StoppedView::copy($v->value) : null)); }
}

final class ErrorView
{
    public function __construct(public readonly int $code, public readonly StringView $message, public readonly int $retryable, public readonly OptionalStoppedView $stopped) {}
    public static function copy(\FFI\CData $v): self { return new self($v->code, StringView::copy($v->message), $v->retryable, OptionalStoppedView::copy($v->stopped)); }
}

final class OptionalErrorView
{
    public function __construct(public readonly int $present, public readonly ?ErrorView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? ErrorView::copy($v->value) : null)); }
}

final class RowView
{
    public function __construct(public readonly StringView $answer_id, public readonly OptionalContentView $input, public readonly OptionalQuestionView $question, public readonly OptionalAnswerView $answer, public readonly OptionalRuleView $threshold, public readonly OptionalLocationView $position, public readonly OptionalStringView $input_file, public readonly MetaView $meta, public readonly OptionalImageViewsView $images) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->answer_id), OptionalContentView::copy($v->input), OptionalQuestionView::copy($v->question), OptionalAnswerView::copy($v->answer), OptionalRuleView::copy($v->threshold), OptionalLocationView::copy($v->position), OptionalStringView::copy($v->input_file), MetaView::copy($v->meta), OptionalImageViewsView::copy($v->images)); }
}

final class DecideViewView
{
    public function __construct(public readonly RowView $common, public readonly DecideValueView $value) {}
    public static function copy(\FFI\CData $v): self { return new self(RowView::copy($v->common), DecideValueView::copy($v->value)); }
}

final class ChooseViewView
{
    public function __construct(public readonly RowView $common, public readonly OptionalStringView $value) {}
    public static function copy(\FFI\CData $v): self { return new self(RowView::copy($v->common), OptionalStringView::copy($v->value)); }
}

final class TagViewView
{
    public function __construct(public readonly RowView $common, public readonly StringsView $value) {}
    public static function copy(\FFI\CData $v): self { return new self(RowView::copy($v->common), StringsView::copy($v->value)); }
}

final class ScoreViewView
{
    public function __construct(public readonly RowView $common, public readonly float $value) {}
    public static function copy(\FFI\CData $v): self { return new self(RowView::copy($v->common), $v->value); }
}

final class FilterViewView
{
    public function __construct(public readonly RowView $common, public readonly int $value) {}
    public static function copy(\FFI\CData $v): self { return new self(RowView::copy($v->common), $v->value); }
}

final class RankViewView
{
    public function __construct(public readonly RowView $common, public readonly OptionalSizeView $value, public readonly OptionalStringView $question_name) {}
    public static function copy(\FFI\CData $v): self { return new self(RowView::copy($v->common), OptionalSizeView::copy($v->value), OptionalStringView::copy($v->question_name)); }
}

final class FindViewView
{
    public function __construct(public readonly RowView $common, public readonly OptionalContentView $value, public readonly OptionalSizeView $index) {}
    public static function copy(\FFI\CData $v): self { return new self(RowView::copy($v->common), OptionalContentView::copy($v->value), OptionalSizeView::copy($v->index)); }
}

final class AnnotateViewView
{
    public function __construct(public readonly RowView $common, public readonly MembersView $answers) {}
    public static function copy(\FFI\CData $v): self { return new self(RowView::copy($v->common), MembersView::copy($v->answers)); }
}

final class RecognizeViewView
{
    public function __construct(public readonly RowView $common, public readonly RecognizeValueView $value, public readonly RecognizeAnswerView $answer) {}
    public static function copy(\FFI\CData $v): self { return new self(RowView::copy($v->common), RecognizeValueView::copy($v->value), RecognizeAnswerView::copy($v->answer)); }
}

final class RelateViewView
{
    public function __construct(public readonly RowView $common, public readonly EdgesView $value, public readonly RelationAnswersView $questions) {}
    public static function copy(\FFI\CData $v): self { return new self(RowView::copy($v->common), EdgesView::copy($v->value), RelationAnswersView::copy($v->questions)); }
}

final class ObservedProbabilitiesV1DataView
{
    public function __construct(public readonly ?float $yes, public readonly ?ProbabilitiesView $named) {}
    public static function copy(\FFI\CData $v, ?string $active): self { return new self(($active === "yes" ? $v->yes : null), ($active === "named" ? ProbabilitiesView::copy($v->named) : null)); }
}

final class ObservedProbabilitiesView
{
    public function __construct(public readonly int $kind, public readonly ObservedProbabilitiesV1DataView $data) {}
    public static function copy(\FFI\CData $v): self { return new self($v->kind, ObservedProbabilitiesV1DataView::copy($v->data,match($v->kind) {1 => "yes", 2 => "named", default => throw new \UnexpectedValueException("native discriminator") })); }
}

final class ObservationSuccessView
{
    public function __construct(public readonly StringView $answer_id, public readonly StringView $observation_id, public readonly MemberValueView $value, public readonly ObservedProbabilitiesView $probabilities, public readonly OptionalDoubleView $confidence) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->answer_id), StringView::copy($v->observation_id), MemberValueView::copy($v->value), ObservedProbabilitiesView::copy($v->probabilities), OptionalDoubleView::copy($v->confidence)); }
}

final class QuestionObservationV1DataView
{
    public function __construct(public readonly ?ObservationSuccessView $success, public readonly ?MemberFailureView $failure) {}
    public static function copy(\FFI\CData $v, ?string $active): self { return new self(($active === "success" ? ObservationSuccessView::copy($v->success) : null), ($active === "failure" ? MemberFailureView::copy($v->failure) : null)); }
}

final class QuestionObservationView
{
    public function __construct(public readonly int $index, public readonly OptionalStringView $member, public readonly OptionalDiscriminatorView $stage, public readonly int $position, public readonly StringView $question_sha256, public readonly StringView $model, public readonly StringView $url, public readonly StringsView $requests, public readonly string $requests_sent, public readonly int $cached, public readonly int $failed_questions, public readonly OptionalUsageView $usage, public readonly QuestionSourcesView $question_sources, public readonly int $state, public readonly QuestionObservationV1DataView $data) {}
    public static function copy(\FFI\CData $v): self { return new self($v->index, OptionalStringView::copy($v->member), OptionalDiscriminatorView::copy($v->stage), $v->position, StringView::copy($v->question_sha256), StringView::copy($v->model), StringView::copy($v->url), StringsView::copy($v->requests), sprintf("%u",$v->requests_sent), $v->cached, $v->failed_questions, OptionalUsageView::copy($v->usage), QuestionSourcesView::copy($v->question_sources), $v->state, QuestionObservationV1DataView::copy($v->data,match($v->state) {1 => "success", 2 => "failure", default => throw new \UnexpectedValueException("native discriminator") })); }
}

final class RowObservationV1DataView
{
    public function __construct(public readonly ?DecideViewView $decide, public readonly ?ChooseViewView $choose, public readonly ?TagViewView $tag, public readonly ?ScoreViewView $score, public readonly ?FilterViewView $filter, public readonly ?RankViewView $rank, public readonly ?FindViewView $find, public readonly ?AnnotateViewView $annotate, public readonly ?RecognizeViewView $recognize, public readonly ?RelateViewView $relate) {}
    public static function copy(\FFI\CData $v, ?string $active): self { return new self(($active === "decide" ? DecideViewView::copy($v->decide) : null), ($active === "choose" ? ChooseViewView::copy($v->choose) : null), ($active === "tag" ? TagViewView::copy($v->tag) : null), ($active === "score" ? ScoreViewView::copy($v->score) : null), ($active === "filter" ? FilterViewView::copy($v->filter) : null), ($active === "rank" ? RankViewView::copy($v->rank) : null), ($active === "find" ? FindViewView::copy($v->find) : null), ($active === "annotate" ? AnnotateViewView::copy($v->annotate) : null), ($active === "recognize" ? RecognizeViewView::copy($v->recognize) : null), ($active === "relate" ? RelateViewView::copy($v->relate) : null)); }
}

final class RowObservationView
{
    public function __construct(public readonly int $index, public readonly int $function, public readonly RowObservationV1DataView $data) {}
    public static function copy(\FFI\CData $v): self { return new self($v->index, $v->function, RowObservationV1DataView::copy($v->data,match($v->function) {1 => "decide", 2 => "choose", 3 => "tag", 4 => "score", 5 => "filter", 6 => "rank", 7 => "find", 8 => "annotate", 9 => "recognize", 10 => "relate", default => throw new \UnexpectedValueException("native discriminator") })); }
}

final class ObservationV1DataView
{
    public function __construct(public readonly ?QuestionObservationView $question, public readonly ?RowObservationView $row) {}
    public static function copy(\FFI\CData $v, ?string $active): self { return new self(($active === "question" ? QuestionObservationView::copy($v->question) : null), ($active === "row" ? RowObservationView::copy($v->row) : null)); }
}

final class ObservationView
{
    public function __construct(public readonly int $kind, public readonly ObservationV1DataView $data) {}
    public static function copy(\FFI\CData $v): self { return new self($v->kind, ObservationV1DataView::copy($v->data,match($v->kind) {1 => "question", 2 => "row", default => throw new \UnexpectedValueException("native discriminator") })); }
}

final class SummaryView
{
    public function __construct(public readonly int $state, public readonly StringView $schema, public readonly OptionalStringView $answer_id, public readonly OptionalDiscriminatorView $function, public readonly int $count, public readonly int $observation_count, public readonly OptionalMetaView $meta, public readonly OptionalFactsView $facts, public readonly OptionalAttemptsView $attempts, public readonly OptionalErrorView $error) {}
    public static function copy(\FFI\CData $v): self { return new self($v->state, StringView::copy($v->schema), OptionalStringView::copy($v->answer_id), OptionalDiscriminatorView::copy($v->function), $v->count, $v->observation_count, OptionalMetaView::copy($v->meta), OptionalFactsView::copy($v->facts), OptionalAttemptsView::copy($v->attempts), OptionalErrorView::copy($v->error)); }
}

final class ReportedUsageView
{
    public function __construct(public readonly int $present, public readonly OptionalU64View $input_tokens, public readonly OptionalU64View $output_tokens) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, OptionalU64View::copy($v->input_tokens), OptionalU64View::copy($v->output_tokens)); }
}

final class SourceDetailView
{
    public function __construct(public readonly int $origin, public readonly StringView $answered_by, public readonly OptionalSizeView $batch_size) {}
    public static function copy(\FFI\CData $v): self { return new self($v->origin, StringView::copy($v->answered_by), OptionalSizeView::copy($v->batch_size)); }
}

final class SourceDetailsView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => SourceDetailView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class InputViewView
{
    public function __construct(public readonly OptionalContentView $original, public readonly OptionalLocationView $position, public readonly OptionalImageViewsView $images) {}
    public static function copy(\FFI\CData $v): self { return new self(OptionalContentView::copy($v->original), OptionalLocationView::copy($v->position), OptionalImageViewsView::copy($v->images)); }
}

final class InputViewsView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => InputViewView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class DetailsView
{
    public function __construct(public readonly OptionalQuestionView $question, public readonly OptionalRuleView $threshold, public readonly OptionalStringView $raw_pick, public readonly ReportedUsageView $usage, public readonly SourceDetailsView $question_sources, public readonly ObservationIdentitiesView $observations, public readonly InputViewsView $inputs) {}
    public static function copy(\FFI\CData $v): self { return new self(OptionalQuestionView::copy($v->question), OptionalRuleView::copy($v->threshold), OptionalStringView::copy($v->raw_pick), ReportedUsageView::copy($v->usage), SourceDetailsView::copy($v->question_sources), ObservationIdentitiesView::copy($v->observations), InputViewsView::copy($v->inputs)); }
}

final class SourceEntityView
{
    public function __construct(public readonly EntityView $entity, public readonly OptionalLocationView $position) {}
    public static function copy(\FFI\CData $v): self { return new self(EntityView::copy($v->entity), OptionalLocationView::copy($v->position)); }
}

final class SourceEntitiesView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => SourceEntityView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class SourceEntityEdgeView
{
    public function __construct(public readonly StringView $relation, public readonly SourceEntityView $source, public readonly SourceEntityView $target, public readonly float $probability, public readonly int $either) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->relation), SourceEntityView::copy($v->source), SourceEntityView::copy($v->target), $v->probability, $v->either); }
}

final class SourceEntityEdgesView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => SourceEntityEdgeView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class OptionalSourceEntityEdgesView
{
    public function __construct(public readonly int $present, public readonly ?SourceEntityEdgesView $value) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, ($v->present ? SourceEntityEdgesView::copy($v->value) : null)); }
}

final class SourceRecognitionView
{
    public function __construct(public readonly int $present, public readonly SourceEntitiesView $entities, public readonly OptionalSourceEntityEdgesView $relations) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, SourceEntitiesView::copy($v->entities), OptionalSourceEntityEdgesView::copy($v->relations)); }
}

final class SourceEndpointView
{
    public function __construct(public readonly int $ordinal, public readonly EndpointView $endpoint, public readonly ContentView $record, public readonly OptionalLocationView $position) {}
    public static function copy(\FFI\CData $v): self { return new self($v->ordinal, EndpointView::copy($v->endpoint), ContentView::copy($v->record), OptionalLocationView::copy($v->position)); }
}

final class SourceEdgeView
{
    public function __construct(public readonly StringView $relation, public readonly SourceEndpointView $source, public readonly SourceEndpointView $target, public readonly float $probability, public readonly int $either) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->relation), SourceEndpointView::copy($v->source), SourceEndpointView::copy($v->target), $v->probability, $v->either); }
}

final class SourceEdgesView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => SourceEdgeView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class SourceRelationsView
{
    public function __construct(public readonly int $present, public readonly SourceEdgesView $edges) {}
    public static function copy(\FFI\CData $v): self { return new self($v->present, SourceEdgesView::copy($v->edges)); }
}

final class InputPropertyView
{
    public function __construct(public readonly StringView $name, public readonly int $kind) {}
    public static function copy(\FFI\CData $v): self { return new self(StringView::copy($v->name), $v->kind); }
}

final class InputPropertiesView
{
    public function __construct(public readonly array $data, public readonly int $len) {}
    public static function copy(\FFI\CData $v): self { return new self(array_map(fn($i) => InputPropertyView::copy($v->data[$i]), $v->len ? range(0,$v->len-1) : []), $v->len); }
}

final class InputDeclarationView
{
    public function __construct(public readonly int $kind, public readonly InputPropertiesView $properties, public readonly StringsView $required) {}
    public static function copy(\FFI\CData $v): self { return new self($v->kind, InputPropertiesView::copy($v->properties), StringsView::copy($v->required)); }
}

final class QuestionAuthorView
{
    public function __construct(public readonly OptionalStringView $name, public readonly OptionalU64View $wording_version, public readonly InputDeclarationView $item_schema, public readonly InputDeclarationView $context_schema) {}
    public static function copy(\FFI\CData $v): self { return new self(OptionalStringView::copy($v->name), OptionalU64View::copy($v->wording_version), InputDeclarationView::copy($v->item_schema), InputDeclarationView::copy($v->context_schema)); }
}
