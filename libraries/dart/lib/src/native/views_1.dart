part of 'views.dart';

final class OptionalLocationView {
  final int present;
  final LocationView? value;
  const OptionalLocationView(this.present, this.value);
  factory OptionalLocationView.copy(COptionalLocationView v) =>
      OptionalLocationView(
          v.present, (v.present != 0 ? LocationView.copy(v.value) : null));
}

final class MemberValueV1DataView {
  final DecideValueView? decide;
  final OptionalStringView? choose;
  final StringsView? tag;
  final double? score;
  const MemberValueV1DataView(this.decide, this.choose, this.tag, this.score);
  factory MemberValueV1DataView.copy(
          CMemberValueV1DataView v, String? active) =>
      MemberValueV1DataView(
          (active == "decide" ? DecideValueView.copy(v.decide) : null),
          (active == "choose" ? OptionalStringView.copy(v.choose) : null),
          (active == "tag" ? StringsView.copy(v.tag) : null),
          (active == "score" ? v.score : null));
}

final class MemberValueView {
  final int kind;
  final MemberValueV1DataView data;
  const MemberValueView(this.kind, this.data);
  factory MemberValueView.copy(CMemberValueView v) => MemberValueView(
      v.kind,
      MemberValueV1DataView.copy(
          v.data,
          switch (v.kind) {
            1 => "decide",
            2 => "choose",
            3 => "tag",
            4 => "score",
            _ => throw StateError("native discriminator")
          }));
}

final class MemberFailureView {
  final StringView failure_id;
  final int cause;
  const MemberFailureView(this.failure_id, this.cause);
  factory MemberFailureView.copy(CMemberFailureView v) =>
      MemberFailureView(StringView.copy(v.failure_id), v.cause);
}

final class MemberSuccessView {
  final StringView answer_id;
  final MemberValueView value;
  final AnswerView answer;
  final RuleView threshold;
  const MemberSuccessView(
      this.answer_id, this.value, this.answer, this.threshold);
  factory MemberSuccessView.copy(CMemberSuccessView v) => MemberSuccessView(
      StringView.copy(v.answer_id),
      MemberValueView.copy(v.value),
      AnswerView.copy(v.answer),
      RuleView.copy(v.threshold));
}

final class MemberV1DataView {
  final MemberSuccessView? success;
  final MemberFailureView? failure;
  const MemberV1DataView(this.success, this.failure);
  factory MemberV1DataView.copy(CMemberV1DataView v, String? active) =>
      MemberV1DataView(
          (active == "success" ? MemberSuccessView.copy(v.success) : null),
          (active == "failure" ? MemberFailureView.copy(v.failure) : null));
}

final class MemberView {
  final StringView name;
  final StringView request;
  final QuestionViewView question;
  final int state;
  final MemberV1DataView data;
  const MemberView(
      this.name, this.request, this.question, this.state, this.data);
  factory MemberView.copy(CMemberView v) => MemberView(
      StringView.copy(v.name),
      StringView.copy(v.request),
      QuestionViewView.copy(v.question),
      v.state,
      MemberV1DataView.copy(
          v.data,
          switch (v.state) {
            1 => "success",
            2 => "failure",
            _ => throw StateError("native discriminator")
          }));
}

final class MembersView {
  final List<MemberView> data;
  final int len;
  const MembersView(this.data, this.len);
  factory MembersView.copy(CMembersView v) => MembersView(
      List.unmodifiable(
          List.generate(v.len, (i) => MemberView.copy(v.data[i]))),
      v.len);
}

final class EntityView {
  final StringView text;
  final int start;
  final int end;
  final int length;
  final StringView kind;
  final double strength;
  const EntityView(
      this.text, this.start, this.end, this.length, this.kind, this.strength);
  factory EntityView.copy(CEntityView v) => EntityView(StringView.copy(v.text),
      v.start, v.end, v.length, StringView.copy(v.kind), v.strength);
}

final class EntitiesView {
  final List<EntityView> data;
  final int len;
  const EntitiesView(this.data, this.len);
  factory EntitiesView.copy(CEntitiesView v) => EntitiesView(
      List.unmodifiable(
          List.generate(v.len, (i) => EntityView.copy(v.data[i]))),
      v.len);
}

final class EntityEdgeView {
  final StringView relation;
  final EntityView source;
  final EntityView target;
  final double probability;
  final int either;
  const EntityEdgeView(
      this.relation, this.source, this.target, this.probability, this.either);
  factory EntityEdgeView.copy(CEntityEdgeView v) => EntityEdgeView(
      StringView.copy(v.relation),
      EntityView.copy(v.source),
      EntityView.copy(v.target),
      v.probability,
      v.either);
}

final class EntityEdgesView {
  final List<EntityEdgeView> data;
  final int len;
  const EntityEdgesView(this.data, this.len);
  factory EntityEdgesView.copy(CEntityEdgesView v) => EntityEdgesView(
      List.unmodifiable(
          List.generate(v.len, (i) => EntityEdgeView.copy(v.data[i]))),
      v.len);
}

final class OptionalEntityEdgesView {
  final int present;
  final EntityEdgesView? value;
  const OptionalEntityEdgesView(this.present, this.value);
  factory OptionalEntityEdgesView.copy(COptionalEntityEdgesView v) =>
      OptionalEntityEdgesView(
          v.present, (v.present != 0 ? EntityEdgesView.copy(v.value) : null));
}

final class PlaceView {
  final int start;
  final int end;
  const PlaceView(this.start, this.end);
  factory PlaceView.copy(CPlaceView v) => PlaceView(v.start, v.end);
}

final class PieceView {
  final int start;
  final int end;
  final ProbabilitiesView tags;
  const PieceView(this.start, this.end, this.tags);
  factory PieceView.copy(CPieceView v) =>
      PieceView(v.start, v.end, ProbabilitiesView.copy(v.tags));
}

final class PiecesView {
  final List<PieceView> data;
  final int len;
  const PiecesView(this.data, this.len);
  factory PiecesView.copy(CPiecesView v) => PiecesView(
      List.unmodifiable(List.generate(v.len, (i) => PieceView.copy(v.data[i]))),
      v.len);
}

final class NameView {
  final int start;
  final int end;
  final OptionalProbabilitiesView kinds;
  final OptionalProbabilitiesView edges;
  const NameView(this.start, this.end, this.kinds, this.edges);
  factory NameView.copy(CNameView v) => NameView(
      v.start,
      v.end,
      OptionalProbabilitiesView.copy(v.kinds),
      OptionalProbabilitiesView.copy(v.edges));
}

final class NamesView {
  final List<NameView> data;
  final int len;
  const NamesView(this.data, this.len);
  factory NamesView.copy(CNamesView v) => NamesView(
      List.unmodifiable(List.generate(v.len, (i) => NameView.copy(v.data[i]))),
      v.len);
}

final class PairView {
  final StringView relation;
  final PlaceView source;
  final PlaceView target;
  final double probability;
  const PairView(this.relation, this.source, this.target, this.probability);
  factory PairView.copy(CPairView v) => PairView(StringView.copy(v.relation),
      PlaceView.copy(v.source), PlaceView.copy(v.target), v.probability);
}

final class PairsView {
  final List<PairView> data;
  final int len;
  const PairsView(this.data, this.len);
  factory PairsView.copy(CPairsView v) => PairsView(
      List.unmodifiable(List.generate(v.len, (i) => PairView.copy(v.data[i]))),
      v.len);
}

final class RecognizeValueView {
  final EntitiesView entities;
  final OptionalEntityEdgesView relations;
  const RecognizeValueView(this.entities, this.relations);
  factory RecognizeValueView.copy(CRecognizeValueView v) => RecognizeValueView(
      EntitiesView.copy(v.entities), OptionalEntityEdgesView.copy(v.relations));
}

final class RecognizeAnswerView {
  final PiecesView pieces;
  final NamesView names;
  final PairsView pairs;
  const RecognizeAnswerView(this.pieces, this.names, this.pairs);
  factory RecognizeAnswerView.copy(CRecognizeAnswerView v) =>
      RecognizeAnswerView(PiecesView.copy(v.pieces), NamesView.copy(v.names),
          PairsView.copy(v.pairs));
}

final class EndpointView {
  final StringView name;
  final StringView kind;
  const EndpointView(this.name, this.kind);
  factory EndpointView.copy(CEndpointView v) =>
      EndpointView(StringView.copy(v.name), StringView.copy(v.kind));
}

final class OptionalEndpointView {
  final int present;
  final EndpointView? value;
  const OptionalEndpointView(this.present, this.value);
  factory OptionalEndpointView.copy(COptionalEndpointView v) =>
      OptionalEndpointView(
          v.present, (v.present != 0 ? EndpointView.copy(v.value) : null));
}

final class EdgeView {
  final StringView relation;
  final EndpointView source;
  final EndpointView target;
  final double probability;
  final int either;
  const EdgeView(
      this.relation, this.source, this.target, this.probability, this.either);
  factory EdgeView.copy(CEdgeView v) => EdgeView(
      StringView.copy(v.relation),
      EndpointView.copy(v.source),
      EndpointView.copy(v.target),
      v.probability,
      v.either);
}

final class EdgesView {
  final List<EdgeView> data;
  final int len;
  const EdgesView(this.data, this.len);
  factory EdgesView.copy(CEdgesView v) => EdgesView(
      List.unmodifiable(List.generate(v.len, (i) => EdgeView.copy(v.data[i]))),
      v.len);
}

final class RelationSuccessView {
  final StringView answer_id;
  final double probability;
  final int accepted;
  const RelationSuccessView(this.answer_id, this.probability, this.accepted);
  factory RelationSuccessView.copy(CRelationSuccessView v) =>
      RelationSuccessView(
          StringView.copy(v.answer_id), v.probability, v.accepted);
}

final class RelationAnswerV1DataView {
  final RelationSuccessView? success;
  final MemberFailureView? failure;
  const RelationAnswerV1DataView(this.success, this.failure);
  factory RelationAnswerV1DataView.copy(
          CRelationAnswerV1DataView v, String? active) =>
      RelationAnswerV1DataView(
          (active == "success" ? RelationSuccessView.copy(v.success) : null),
          (active == "failure" ? MemberFailureView.copy(v.failure) : null));
}

final class RelationAnswerView {
  final StringView relation;
  final StringView reads;
  final int method;
  final int direction;
  final EndpointView source;
  final OptionalEndpointView target;
  final StringView request;
  final int state;
  final RelationAnswerV1DataView data;
  const RelationAnswerView(
      this.relation,
      this.reads,
      this.method,
      this.direction,
      this.source,
      this.target,
      this.request,
      this.state,
      this.data);
  factory RelationAnswerView.copy(CRelationAnswerView v) => RelationAnswerView(
      StringView.copy(v.relation),
      StringView.copy(v.reads),
      v.method,
      v.direction,
      EndpointView.copy(v.source),
      OptionalEndpointView.copy(v.target),
      StringView.copy(v.request),
      v.state,
      RelationAnswerV1DataView.copy(
          v.data,
          switch (v.state) {
            1 => "success",
            2 => "failure",
            _ => throw StateError("native discriminator")
          }));
}

final class RelationAnswersView {
  final List<RelationAnswerView> data;
  final int len;
  const RelationAnswersView(this.data, this.len);
  factory RelationAnswersView.copy(CRelationAnswersView v) =>
      RelationAnswersView(
          List.unmodifiable(
              List.generate(v.len, (i) => RelationAnswerView.copy(v.data[i]))),
          v.len);
}

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
