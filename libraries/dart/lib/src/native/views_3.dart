part of 'views.dart';

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
  final DecideViewView? decide;
  final ChooseViewView? choose;
  final TagViewView? tag;
  final ScoreViewView? score;
  final FilterViewView? filter;
  final RankViewView? rank;
  final FindViewView? find;
  final AnnotateViewView? annotate;
  final RecognizeViewView? recognize;
  final RelateViewView? relate;
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
          (active == "decide" ? DecideViewView.copy(v.decide) : null),
          (active == "choose" ? ChooseViewView.copy(v.choose) : null),
          (active == "tag" ? TagViewView.copy(v.tag) : null),
          (active == "score" ? ScoreViewView.copy(v.score) : null),
          (active == "filter" ? FilterViewView.copy(v.filter) : null),
          (active == "rank" ? RankViewView.copy(v.rank) : null),
          (active == "find" ? FindViewView.copy(v.find) : null),
          (active == "annotate" ? AnnotateViewView.copy(v.annotate) : null),
          (active == "recognize" ? RecognizeViewView.copy(v.recognize) : null),
          (active == "relate" ? RelateViewView.copy(v.relate) : null));
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

final class InputViewView {
  final OptionalContentView original;
  final OptionalLocationView position;
  final OptionalImageViewsView images;
  const InputViewView(this.original, this.position, this.images);
  factory InputViewView.copy(CInputViewView v) => InputViewView(
      OptionalContentView.copy(v.original),
      OptionalLocationView.copy(v.position),
      OptionalImageViewsView.copy(v.images));
}

final class InputViewsView {
  final List<InputViewView> data;
  final int len;
  const InputViewsView(this.data, this.len);
  factory InputViewsView.copy(CInputViewsView v) => InputViewsView(
      List.unmodifiable(
          List.generate(v.len, (i) => InputViewView.copy(v.data[i]))),
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

final class InputPropertyView {
  final StringView name;
  final int kind;
  const InputPropertyView(this.name, this.kind);
  factory InputPropertyView.copy(CInputPropertyView v) =>
      InputPropertyView(StringView.copy(v.name), v.kind);
}

final class InputPropertiesView {
  final List<InputPropertyView> data;
  final int len;
  const InputPropertiesView(this.data, this.len);
  factory InputPropertiesView.copy(CInputPropertiesView v) =>
      InputPropertiesView(
          List.unmodifiable(
              List.generate(v.len, (i) => InputPropertyView.copy(v.data[i]))),
          v.len);
}
