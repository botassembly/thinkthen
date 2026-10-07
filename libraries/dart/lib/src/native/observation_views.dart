part of 'views.dart';

final class UsageView {
  final BigInt input_tokens;
  final BigInt output_tokens;
  const UsageView(this.input_tokens, this.output_tokens);
  factory UsageView.copy(CUsageView v) => UsageView(
      BigInt.from(v.input_tokens).toUnsigned(64),
      BigInt.from(v.output_tokens).toUnsigned(64));
}

final class OptionalUsageView {
  final int present;
  final UsageView? value;
  const OptionalUsageView(this.present, this.value);
  factory OptionalUsageView.copy(COptionalUsageView v) => OptionalUsageView(
      v.present, (v.present != 0 ? UsageView.copy(v.value) : null));
}

final class QuestionSourceView {
  final int origin;
  final StringView answered_by;
  const QuestionSourceView(this.origin, this.answered_by);
  factory QuestionSourceView.copy(CQuestionSourceView v) =>
      QuestionSourceView(v.origin, StringView.copy(v.answered_by));
}

final class QuestionSourcesView {
  final List<QuestionSourceView> data;
  final int len;
  const QuestionSourcesView(this.data, this.len);
  factory QuestionSourcesView.copy(CQuestionSourcesView v) =>
      QuestionSourcesView(
          List.unmodifiable(
              List.generate(v.len, (i) => QuestionSourceView.copy(v.data[i]))),
          v.len);
}

final class ObservationIdentityV1DataView {
  final StringView? observation_id;
  final StringView? failure_id;
  const ObservationIdentityV1DataView(this.observation_id, this.failure_id);
  factory ObservationIdentityV1DataView.copy(
          CObservationIdentityV1DataView v, String? active) =>
      ObservationIdentityV1DataView(
          (active == "observation_id"
              ? StringView.copy(v.observation_id)
              : null),
          (active == "failure_id" ? StringView.copy(v.failure_id) : null));
}

final class ObservationIdentityView {
  final int kind;
  final ObservationIdentityV1DataView data;
  const ObservationIdentityView(this.kind, this.data);
  factory ObservationIdentityView.copy(CObservationIdentityView v) =>
      ObservationIdentityView(
          v.kind,
          ObservationIdentityV1DataView.copy(
              v.data,
              switch (v.kind) {
                1 => "observation_id",
                2 => "failure_id",
                _ => throw StateError("native discriminator")
              }));
}

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

final class ObservationSuccessView {
  final StringView answer_id;
  final StringView observation_id;
  final MemberValueView value;
  final ObservedProbabilitiesView probabilities;
  final OptionalDoubleView confidence;
  const ObservationSuccessView(this.answer_id, this.observation_id, this.value,
      this.probabilities, this.confidence);
  factory ObservationSuccessView.copy(CObservationSuccessView v) =>
      ObservationSuccessView(
          StringView.copy(v.answer_id),
          StringView.copy(v.observation_id),
          MemberValueView.copy(v.value),
          ObservedProbabilitiesView.copy(v.probabilities),
          OptionalDoubleView.copy(v.confidence));
}

final class QuestionObservationV1DataView {
  final ObservationSuccessView? success;
  final MemberFailureView? failure;
  const QuestionObservationV1DataView(this.success, this.failure);
  factory QuestionObservationV1DataView.copy(
          CQuestionObservationV1DataView v, String? active) =>
      QuestionObservationV1DataView(
          (active == "success" ? ObservationSuccessView.copy(v.success) : null),
          (active == "failure" ? MemberFailureView.copy(v.failure) : null));
}

final class QuestionObservationView {
  final int index;
  final OptionalStringView member;
  final OptionalDiscriminatorView stage;
  final int position;
  final StringView question_sha256;
  final StringView model;
  final StringView url;
  final StringsView requests;
  final BigInt requests_sent;
  final int cached;
  final int failed_questions;
  final OptionalUsageView usage;
  final QuestionSourcesView question_sources;
  final int state;
  final QuestionObservationV1DataView data;
  const QuestionObservationView(
      this.index,
      this.member,
      this.stage,
      this.position,
      this.question_sha256,
      this.model,
      this.url,
      this.requests,
      this.requests_sent,
      this.cached,
      this.failed_questions,
      this.usage,
      this.question_sources,
      this.state,
      this.data);
  factory QuestionObservationView.copy(CQuestionObservationView v) =>
      QuestionObservationView(
          v.index,
          OptionalStringView.copy(v.member),
          OptionalDiscriminatorView.copy(v.stage),
          v.position,
          StringView.copy(v.question_sha256),
          StringView.copy(v.model),
          StringView.copy(v.url),
          StringsView.copy(v.requests),
          BigInt.from(v.requests_sent).toUnsigned(64),
          v.cached,
          v.failed_questions,
          OptionalUsageView.copy(v.usage),
          QuestionSourcesView.copy(v.question_sources),
          v.state,
          QuestionObservationV1DataView.copy(
              v.data,
              switch (v.state) {
                1 => "success",
                2 => "failure",
                _ => throw StateError("native discriminator")
              }));
}

final class RowObservationV1DataView {
  final DecideView? decide;
  final ChooseView? choose;
  final TagView? tag;
  final ScoreView? score;
  final FilterView? filter;
  final RankView? rank;
  final FindView? find;
  final AnnotateView? annotate;
  final RecognizeView? recognize;
  final RelateView? relate;
  const RowObservationV1DataView(
      this.decide,
      this.choose,
      this.tag,
      this.score,
      this.filter,
      this.rank,
      this.find,
      this.annotate,
      this.recognize,
      this.relate);
  factory RowObservationV1DataView.copy(
          CRowObservationV1DataView v, String? active) =>
      RowObservationV1DataView(
          (active == "decide" ? DecideView.copy(v.decide) : null),
          (active == "choose" ? ChooseView.copy(v.choose) : null),
          (active == "tag" ? TagView.copy(v.tag) : null),
          (active == "score" ? ScoreView.copy(v.score) : null),
          (active == "filter" ? FilterView.copy(v.filter) : null),
          (active == "rank" ? RankView.copy(v.rank) : null),
          (active == "find" ? FindView.copy(v.find) : null),
          (active == "annotate" ? AnnotateView.copy(v.annotate) : null),
          (active == "recognize" ? RecognizeView.copy(v.recognize) : null),
          (active == "relate" ? RelateView.copy(v.relate) : null));
}

final class RowObservationView {
  final int index;
  final int function;
  final RowObservationV1DataView data;
  const RowObservationView(this.index, this.function, this.data);
  factory RowObservationView.copy(CRowObservationView v) => RowObservationView(
      v.index,
      v.function,
      RowObservationV1DataView.copy(
          v.data,
          switch (v.function) {
            1 => "decide",
            2 => "choose",
            3 => "tag",
            4 => "score",
            5 => "filter",
            6 => "rank",
            7 => "find",
            8 => "annotate",
            9 => "recognize",
            10 => "relate",
            _ => throw StateError("native discriminator")
          }));
}

final class ObservationV1DataView {
  final QuestionObservationView? question;
  final RowObservationView? row;
  const ObservationV1DataView(this.question, this.row);
  factory ObservationV1DataView.copy(
          CObservationV1DataView v, String? active) =>
      ObservationV1DataView(
          (active == "question"
              ? QuestionObservationView.copy(v.question)
              : null),
          (active == "row" ? RowObservationView.copy(v.row) : null));
}

final class ObservationView {
  final int kind;
  final ObservationV1DataView data;
  const ObservationView(this.kind, this.data);
  factory ObservationView.copy(CObservationView v) => ObservationView(
      v.kind,
      ObservationV1DataView.copy(
          v.data,
          switch (v.kind) {
            1 => "question",
            2 => "row",
            _ => throw StateError("native discriminator")
          }));
}

final class SummaryView {
  final int state;
  final StringView schema;
  final OptionalStringView answer_id;
  final OptionalDiscriminatorView function;
  final int count;
  final int observation_count;
  final OptionalMetaView meta;
  final OptionalFactsView facts;
  final OptionalAttemptsView attempts;
  final OptionalErrorView error;
  const SummaryView(
      this.state,
      this.schema,
      this.answer_id,
      this.function,
      this.count,
      this.observation_count,
      this.meta,
      this.facts,
      this.attempts,
      this.error);
  factory SummaryView.copy(CSummaryView v) => SummaryView(
      v.state,
      StringView.copy(v.schema),
      OptionalStringView.copy(v.answer_id),
      OptionalDiscriminatorView.copy(v.function),
      v.count,
      v.observation_count,
      OptionalMetaView.copy(v.meta),
      OptionalFactsView.copy(v.facts),
      OptionalAttemptsView.copy(v.attempts),
      OptionalErrorView.copy(v.error));
}

final class ReportedUsageView {
  final int present;
  final OptionalU64View input_tokens;
  final OptionalU64View output_tokens;
  const ReportedUsageView(this.present, this.input_tokens, this.output_tokens);
  factory ReportedUsageView.copy(CReportedUsageView v) => ReportedUsageView(
      v.present,
      OptionalU64View.copy(v.input_tokens),
      OptionalU64View.copy(v.output_tokens));
}

final class SourceDetailView {
  final int origin;
  final StringView answered_by;
  final OptionalSizeView batch_size;
  const SourceDetailView(this.origin, this.answered_by, this.batch_size);
  factory SourceDetailView.copy(CSourceDetailView v) => SourceDetailView(
      v.origin,
      StringView.copy(v.answered_by),
      OptionalSizeView.copy(v.batch_size));
}

final class SourceDetailsView {
  final List<SourceDetailView> data;
  final int len;
  const SourceDetailsView(this.data, this.len);
  factory SourceDetailsView.copy(CSourceDetailsView v) => SourceDetailsView(
      List.unmodifiable(
          List.generate(v.len, (i) => SourceDetailView.copy(v.data[i]))),
      v.len);
}

final class InputView {
  final OptionalContentView original;
  final OptionalLocationView position;
  final OptionalImageViewsView images;
  const InputView(this.original, this.position, this.images);
  factory InputView.copy(CInputView v) => InputView(
      OptionalContentView.copy(v.original),
      OptionalLocationView.copy(v.position),
      OptionalImageViewsView.copy(v.images));
}

final class InputViewsView {
  final List<InputView> data;
  final int len;
  const InputViewsView(this.data, this.len);
  factory InputViewsView.copy(CInputViewsView v) => InputViewsView(
      List.unmodifiable(List.generate(v.len, (i) => InputView.copy(v.data[i]))),
      v.len);
}

final class DetailsView {
  final OptionalQuestionView question;
  final OptionalRuleView threshold;
  final OptionalStringView raw_pick;
  final ReportedUsageView usage;
  final SourceDetailsView question_sources;
  final ObservationIdentitiesView observations;
  final InputViewsView inputs;
  const DetailsView(this.question, this.threshold, this.raw_pick, this.usage,
      this.question_sources, this.observations, this.inputs);
  factory DetailsView.copy(CDetailsView v) => DetailsView(
      OptionalQuestionView.copy(v.question),
      OptionalRuleView.copy(v.threshold),
      OptionalStringView.copy(v.raw_pick),
      ReportedUsageView.copy(v.usage),
      SourceDetailsView.copy(v.question_sources),
      ObservationIdentitiesView.copy(v.observations),
      InputViewsView.copy(v.inputs));
}

final class SourceEntityView {
  final EntityView entity;
  final OptionalLocationView position;
  const SourceEntityView(this.entity, this.position);
  factory SourceEntityView.copy(CSourceEntityView v) => SourceEntityView(
      EntityView.copy(v.entity), OptionalLocationView.copy(v.position));
}

final class SourceEntitiesView {
  final List<SourceEntityView> data;
  final int len;
  const SourceEntitiesView(this.data, this.len);
  factory SourceEntitiesView.copy(CSourceEntitiesView v) => SourceEntitiesView(
      List.unmodifiable(
          List.generate(v.len, (i) => SourceEntityView.copy(v.data[i]))),
      v.len);
}

final class SourceEntityEdgeView {
  final StringView relation;
  final SourceEntityView source;
  final SourceEntityView target;
  final double probability;
  final int either;
  const SourceEntityEdgeView(
      this.relation, this.source, this.target, this.probability, this.either);
  factory SourceEntityEdgeView.copy(CSourceEntityEdgeView v) =>
      SourceEntityEdgeView(
          StringView.copy(v.relation),
          SourceEntityView.copy(v.source),
          SourceEntityView.copy(v.target),
          v.probability,
          v.either);
}

final class SourceEntityEdgesView {
  final List<SourceEntityEdgeView> data;
  final int len;
  const SourceEntityEdgesView(this.data, this.len);
  factory SourceEntityEdgesView.copy(CSourceEntityEdgesView v) =>
      SourceEntityEdgesView(
          List.unmodifiable(List.generate(
              v.len, (i) => SourceEntityEdgeView.copy(v.data[i]))),
          v.len);
}

final class OptionalSourceEntityEdgesView {
  final int present;
  final SourceEntityEdgesView? value;
  const OptionalSourceEntityEdgesView(this.present, this.value);
  factory OptionalSourceEntityEdgesView.copy(
          COptionalSourceEntityEdgesView v) =>
      OptionalSourceEntityEdgesView(v.present,
          (v.present != 0 ? SourceEntityEdgesView.copy(v.value) : null));
}

final class SourceRecognitionView {
  final int present;
  final SourceEntitiesView entities;
  final OptionalSourceEntityEdgesView relations;
  const SourceRecognitionView(this.present, this.entities, this.relations);
  factory SourceRecognitionView.copy(CSourceRecognitionView v) =>
      SourceRecognitionView(v.present, SourceEntitiesView.copy(v.entities),
          OptionalSourceEntityEdgesView.copy(v.relations));
}

final class SourceEndpointView {
  final int ordinal;
  final EndpointView endpoint;
  final ContentView record;
  final OptionalLocationView position;
  const SourceEndpointView(
      this.ordinal, this.endpoint, this.record, this.position);
  factory SourceEndpointView.copy(CSourceEndpointView v) => SourceEndpointView(
      v.ordinal,
      EndpointView.copy(v.endpoint),
      ContentView.copy(v.record),
      OptionalLocationView.copy(v.position));
}

final class SourceEdgeView {
  final StringView relation;
  final SourceEndpointView source;
  final SourceEndpointView target;
  final double probability;
  final int either;
  const SourceEdgeView(
      this.relation, this.source, this.target, this.probability, this.either);
  factory SourceEdgeView.copy(CSourceEdgeView v) => SourceEdgeView(
      StringView.copy(v.relation),
      SourceEndpointView.copy(v.source),
      SourceEndpointView.copy(v.target),
      v.probability,
      v.either);
}

final class SourceEdgesView {
  final List<SourceEdgeView> data;
  final int len;
  const SourceEdgesView(this.data, this.len);
  factory SourceEdgesView.copy(CSourceEdgesView v) => SourceEdgesView(
      List.unmodifiable(
          List.generate(v.len, (i) => SourceEdgeView.copy(v.data[i]))),
      v.len);
}

final class SourceRelationsView {
  final int present;
  final SourceEdgesView edges;
  const SourceRelationsView(this.present, this.edges);
  factory SourceRelationsView.copy(CSourceRelationsView v) =>
      SourceRelationsView(v.present, SourceEdgesView.copy(v.edges));
}
