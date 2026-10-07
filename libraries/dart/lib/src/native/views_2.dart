part of 'views.dart';

final class ObservationIdentitiesView {
  final List<ObservationIdentityView> data;
  final int len;
  const ObservationIdentitiesView(this.data, this.len);
  factory ObservationIdentitiesView.copy(CObservationIdentitiesView v) =>
      ObservationIdentitiesView(
          List.unmodifiable(List.generate(
              v.len, (i) => ObservationIdentityView.copy(v.data[i]))),
          v.len);
}

final class ProfileWarningView {
  final StringView tuned_for;
  final StringView running;
  const ProfileWarningView(this.tuned_for, this.running);
  factory ProfileWarningView.copy(CProfileWarningView v) => ProfileWarningView(
      StringView.copy(v.tuned_for), StringView.copy(v.running));
}

final class OptionalProfileWarningView {
  final int present;
  final ProfileWarningView? value;
  const OptionalProfileWarningView(this.present, this.value);
  factory OptionalProfileWarningView.copy(COptionalProfileWarningView v) =>
      OptionalProfileWarningView(v.present,
          (v.present != 0 ? ProfileWarningView.copy(v.value) : null));
}

final class BatchView {
  final int kind;
  final int records;
  const BatchView(this.kind, this.records);
  factory BatchView.copy(CBatchView v) => BatchView(v.kind, v.records);
}

final class OptionalBatchView {
  final int present;
  final BatchView? value;
  const OptionalBatchView(this.present, this.value);
  factory OptionalBatchView.copy(COptionalBatchView v) => OptionalBatchView(
      v.present, (v.present != 0 ? BatchView.copy(v.value) : null));
}

final class BatchWarningView {
  final BatchView tuned_for;
  final BatchView running;
  const BatchWarningView(this.tuned_for, this.running);
  factory BatchWarningView.copy(CBatchWarningView v) =>
      BatchWarningView(BatchView.copy(v.tuned_for), BatchView.copy(v.running));
}

final class OptionalBatchWarningView {
  final int present;
  final BatchWarningView? value;
  const OptionalBatchWarningView(this.present, this.value);
  factory OptionalBatchWarningView.copy(COptionalBatchWarningView v) =>
      OptionalBatchWarningView(
          v.present, (v.present != 0 ? BatchWarningView.copy(v.value) : null));
}

final class AttemptView {
  final BigInt ordinal;
  final StringView request_sha256;
  final BigInt wall_ms;
  final int outcome;
  final StringView sdk_request_id;
  final OptionalU16View status;
  final OptionalU64View server_ms;
  final OptionalStringView request_id;
  const AttemptView(
      this.ordinal,
      this.request_sha256,
      this.wall_ms,
      this.outcome,
      this.sdk_request_id,
      this.status,
      this.server_ms,
      this.request_id);
  factory AttemptView.copy(CAttemptView v) => AttemptView(
      BigInt.from(v.ordinal).toUnsigned(64),
      StringView.copy(v.request_sha256),
      BigInt.from(v.wall_ms).toUnsigned(64),
      v.outcome,
      StringView.copy(v.sdk_request_id),
      OptionalU16View.copy(v.status),
      OptionalU64View.copy(v.server_ms),
      OptionalStringView.copy(v.request_id));
}

final class AttemptsView {
  final List<AttemptView> data;
  final int len;
  const AttemptsView(this.data, this.len);
  factory AttemptsView.copy(CAttemptsView v) => AttemptsView(
      List.unmodifiable(
          List.generate(v.len, (i) => AttemptView.copy(v.data[i]))),
      v.len);
}

final class OptionalAttemptsView {
  final int present;
  final AttemptsView? value;
  const OptionalAttemptsView(this.present, this.value);
  factory OptionalAttemptsView.copy(COptionalAttemptsView v) =>
      OptionalAttemptsView(
          v.present, (v.present != 0 ? AttemptsView.copy(v.value) : null));
}

final class MetaView {
  final StringView tool;
  final OptionalStringView question_sha256;
  final OptionalStringView questions_sha256;
  final StringView url;
  final StringView model;
  final OptionalUsageView usage;
  final BigInt requests_sent;
  final int cached;
  final StringsView requests;
  final int failed_questions;
  final OptionalProfileWarningView profile_warning;
  final OptionalBatchView batch_setting;
  final OptionalBatchWarningView batch_warning;
  final OptionalStringView context_sha256;
  final OptionalAttemptsView attempts;
  final OptionalDiscriminatorView origin;
  final QuestionSourcesView question_sources;
  final ObservationIdentitiesView observations;
  final OptionalStringView answered_by;
  const MetaView(
      this.tool,
      this.question_sha256,
      this.questions_sha256,
      this.url,
      this.model,
      this.usage,
      this.requests_sent,
      this.cached,
      this.requests,
      this.failed_questions,
      this.profile_warning,
      this.batch_setting,
      this.batch_warning,
      this.context_sha256,
      this.attempts,
      this.origin,
      this.question_sources,
      this.observations,
      this.answered_by);
  factory MetaView.copy(CMetaView v) => MetaView(
      StringView.copy(v.tool),
      OptionalStringView.copy(v.question_sha256),
      OptionalStringView.copy(v.questions_sha256),
      StringView.copy(v.url),
      StringView.copy(v.model),
      OptionalUsageView.copy(v.usage),
      BigInt.from(v.requests_sent).toUnsigned(64),
      v.cached,
      StringsView.copy(v.requests),
      v.failed_questions,
      OptionalProfileWarningView.copy(v.profile_warning),
      OptionalBatchView.copy(v.batch_setting),
      OptionalBatchWarningView.copy(v.batch_warning),
      OptionalStringView.copy(v.context_sha256),
      OptionalAttemptsView.copy(v.attempts),
      OptionalDiscriminatorView.copy(v.origin),
      QuestionSourcesView.copy(v.question_sources),
      ObservationIdentitiesView.copy(v.observations),
      OptionalStringView.copy(v.answered_by));
}

final class OptionalMetaView {
  final int present;
  final MetaView? value;
  const OptionalMetaView(this.present, this.value);
  factory OptionalMetaView.copy(COptionalMetaView v) => OptionalMetaView(
      v.present, (v.present != 0 ? MetaView.copy(v.value) : null));
}

final class FactsView {
  final StringView call_id;
  final BigInt cache_answers;
  final OptionalStringView estimated_cost_usd;
  final OptionalU64View input_tokens;
  final OptionalStringView model;
  final OptionalU64View output_tokens;
  final BigInt records;
  final BigInt requests_sent;
  final double seconds;
  final OptionalU64View command_ms;
  const FactsView(
      this.call_id,
      this.cache_answers,
      this.estimated_cost_usd,
      this.input_tokens,
      this.model,
      this.output_tokens,
      this.records,
      this.requests_sent,
      this.seconds,
      this.command_ms);
  factory FactsView.copy(CFactsView v) => FactsView(
      StringView.copy(v.call_id),
      BigInt.from(v.cache_answers).toUnsigned(64),
      OptionalStringView.copy(v.estimated_cost_usd),
      OptionalU64View.copy(v.input_tokens),
      OptionalStringView.copy(v.model),
      OptionalU64View.copy(v.output_tokens),
      BigInt.from(v.records).toUnsigned(64),
      BigInt.from(v.requests_sent).toUnsigned(64),
      v.seconds,
      OptionalU64View.copy(v.command_ms));
}

final class OptionalFactsView {
  final int present;
  final FactsView? value;
  const OptionalFactsView(this.present, this.value);
  factory OptionalFactsView.copy(COptionalFactsView v) => OptionalFactsView(
      v.present, (v.present != 0 ? FactsView.copy(v.value) : null));
}

final class StoppedView {
  final OptionalSizeView at;
  final int cause;
  final OptionalU16View status;
  final int retryable;
  const StoppedView(this.at, this.cause, this.status, this.retryable);
  factory StoppedView.copy(CStoppedView v) => StoppedView(
      OptionalSizeView.copy(v.at),
      v.cause,
      OptionalU16View.copy(v.status),
      v.retryable);
}

final class OptionalStoppedView {
  final int present;
  final StoppedView? value;
  const OptionalStoppedView(this.present, this.value);
  factory OptionalStoppedView.copy(COptionalStoppedView v) =>
      OptionalStoppedView(
          v.present, (v.present != 0 ? StoppedView.copy(v.value) : null));
}

final class ErrorView {
  final int code;
  final StringView message;
  final int retryable;
  final OptionalStoppedView stopped;
  const ErrorView(this.code, this.message, this.retryable, this.stopped);
  factory ErrorView.copy(CErrorView v) => ErrorView(
      v.code,
      StringView.copy(v.message),
      v.retryable,
      OptionalStoppedView.copy(v.stopped));
}

final class OptionalErrorView {
  final int present;
  final ErrorView? value;
  const OptionalErrorView(this.present, this.value);
  factory OptionalErrorView.copy(COptionalErrorView v) => OptionalErrorView(
      v.present, (v.present != 0 ? ErrorView.copy(v.value) : null));
}

final class RowView {
  final StringView answer_id;
  final OptionalContentView input;
  final OptionalQuestionView question;
  final OptionalAnswerView answer;
  final OptionalRuleView threshold;
  final OptionalLocationView position;
  final OptionalStringView input_file;
  final MetaView meta;
  final OptionalImageViewsView images;
  const RowView(this.answer_id, this.input, this.question, this.answer,
      this.threshold, this.position, this.input_file, this.meta, this.images);
  factory RowView.copy(CRowView v) => RowView(
      StringView.copy(v.answer_id),
      OptionalContentView.copy(v.input),
      OptionalQuestionView.copy(v.question),
      OptionalAnswerView.copy(v.answer),
      OptionalRuleView.copy(v.threshold),
      OptionalLocationView.copy(v.position),
      OptionalStringView.copy(v.input_file),
      MetaView.copy(v.meta),
      OptionalImageViewsView.copy(v.images));
}

final class DecideView {
  final RowView common;
  final DecideValueView value;
  const DecideView(this.common, this.value);
  factory DecideView.copy(CDecideView v) =>
      DecideView(RowView.copy(v.common), DecideValueView.copy(v.value));
}

final class ChooseView {
  final RowView common;
  final OptionalStringView value;
  const ChooseView(this.common, this.value);
  factory ChooseView.copy(CChooseView v) =>
      ChooseView(RowView.copy(v.common), OptionalStringView.copy(v.value));
}

final class TagView {
  final RowView common;
  final StringsView value;
  const TagView(this.common, this.value);
  factory TagView.copy(CTagView v) =>
      TagView(RowView.copy(v.common), StringsView.copy(v.value));
}

final class ScoreView {
  final RowView common;
  final double value;
  const ScoreView(this.common, this.value);
  factory ScoreView.copy(CScoreView v) =>
      ScoreView(RowView.copy(v.common), v.value);
}

final class FilterView {
  final RowView common;
  final int value;
  const FilterView(this.common, this.value);
  factory FilterView.copy(CFilterView v) =>
      FilterView(RowView.copy(v.common), v.value);
}

final class RankView {
  final RowView common;
  final OptionalSizeView value;
  final OptionalStringView question_name;
  const RankView(this.common, this.value, this.question_name);
  factory RankView.copy(CRankView v) => RankView(RowView.copy(v.common),
      OptionalSizeView.copy(v.value), OptionalStringView.copy(v.question_name));
}

final class FindView {
  final RowView common;
  final OptionalContentView value;
  final OptionalSizeView index;
  const FindView(this.common, this.value, this.index);
  factory FindView.copy(CFindView v) => FindView(RowView.copy(v.common),
      OptionalContentView.copy(v.value), OptionalSizeView.copy(v.index));
}

final class AnnotateView {
  final RowView common;
  final MembersView answers;
  const AnnotateView(this.common, this.answers);
  factory AnnotateView.copy(CAnnotateView v) =>
      AnnotateView(RowView.copy(v.common), MembersView.copy(v.answers));
}

final class RecognizeView {
  final RowView common;
  final RecognizeValueView value;
  final RecognizeAnswerView answer;
  const RecognizeView(this.common, this.value, this.answer);
  factory RecognizeView.copy(CRecognizeView v) => RecognizeView(
      RowView.copy(v.common),
      RecognizeValueView.copy(v.value),
      RecognizeAnswerView.copy(v.answer));
}

final class RelateView {
  final RowView common;
  final EdgesView value;
  final RelationAnswersView questions;
  const RelateView(this.common, this.value, this.questions);
  factory RelateView.copy(CRelateView v) => RelateView(RowView.copy(v.common),
      EdgesView.copy(v.value), RelationAnswersView.copy(v.questions));
}

final class ObservedProbabilitiesV1DataView {
  final double? yes;
  final ProbabilitiesView? named;
  const ObservedProbabilitiesV1DataView(this.yes, this.named);
  factory ObservedProbabilitiesV1DataView.copy(
          CObservedProbabilitiesV1DataView v, String? active) =>
      ObservedProbabilitiesV1DataView((active == "yes" ? v.yes : null),
          (active == "named" ? ProbabilitiesView.copy(v.named) : null));
}

final class ObservedProbabilitiesView {
  final int kind;
  final ObservedProbabilitiesV1DataView data;
  const ObservedProbabilitiesView(this.kind, this.data);
  factory ObservedProbabilitiesView.copy(CObservedProbabilitiesView v) =>
      ObservedProbabilitiesView(
          v.kind,
          ObservedProbabilitiesV1DataView.copy(
              v.data,
              switch (v.kind) {
                1 => "yes",
                2 => "named",
                _ => throw StateError("native discriminator")
              }));
}
