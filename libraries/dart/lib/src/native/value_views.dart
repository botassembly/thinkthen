part of 'views.dart';

final class LegacyAnswerView {
  final int outcome;
  final double probability;
  const LegacyAnswerView(this.outcome, this.probability);
  factory LegacyAnswerView.copy(CLegacyAnswerView v) =>
      LegacyAnswerView(v.outcome, v.probability);
}

final class StringView {
  final String data;
  final int len;
  const StringView(this.data, this.len);
  factory StringView.copy(CStringView v) => StringView(
      (v.len == 0 ? "" : utf8.decode(v.data.cast<Uint8>().asTypedList(v.len))),
      v.len);
}

final class StringsView {
  final List<StringView> data;
  final int len;
  const StringsView(this.data, this.len);
  factory StringsView.copy(CStringsView v) => StringsView(
      List.unmodifiable(
          List.generate(v.len, (i) => StringView.copy(v.data[i]))),
      v.len);
}

final class OptionalStringView {
  final int present;
  final StringView? value;
  const OptionalStringView(this.present, this.value);
  factory OptionalStringView.copy(COptionalStringView v) => OptionalStringView(
      v.present, (v.present != 0 ? StringView.copy(v.value) : null));
}

final class OptionalSizeView {
  final int present;
  final int? value;
  const OptionalSizeView(this.present, this.value);
  factory OptionalSizeView.copy(COptionalSizeView v) =>
      OptionalSizeView(v.present, (v.present != 0 ? v.value : null));
}

final class OptionalU64View {
  final int present;
  final BigInt? value;
  const OptionalU64View(this.present, this.value);
  factory OptionalU64View.copy(COptionalU64View v) => OptionalU64View(
      v.present, (v.present != 0 ? BigInt.from(v.value).toUnsigned(64) : null));
}

final class OptionalU16View {
  final int present;
  final int? value;
  const OptionalU16View(this.present, this.value);
  factory OptionalU16View.copy(COptionalU16View v) =>
      OptionalU16View(v.present, (v.present != 0 ? v.value : null));
}

final class OptionalDoubleView {
  final int present;
  final double? value;
  const OptionalDoubleView(this.present, this.value);
  factory OptionalDoubleView.copy(COptionalDoubleView v) =>
      OptionalDoubleView(v.present, (v.present != 0 ? v.value : null));
}

final class OptionalDiscriminatorView {
  final int present;
  final int? value;
  const OptionalDiscriminatorView(this.present, this.value);
  factory OptionalDiscriminatorView.copy(COptionalDiscriminatorView v) =>
      OptionalDiscriminatorView(v.present, (v.present != 0 ? v.value : null));
}

final class ContentView {
  final int kind;
  final StringView data;
  const ContentView(this.kind, this.data);
  factory ContentView.copy(CContentView v) =>
      ContentView(v.kind, StringView.copy(v.data));
}

final class OptionalContentView {
  final int present;
  final ContentView? value;
  const OptionalContentView(this.present, this.value);
  factory OptionalContentView.copy(COptionalContentView v) =>
      OptionalContentView(
          v.present, (v.present != 0 ? ContentView.copy(v.value) : null));
}

final class RuleView {
  final int kind;
  final double low;
  final double high;
  const RuleView(this.kind, this.low, this.high);
  factory RuleView.copy(CRuleView v) => RuleView(v.kind, v.low, v.high);
}

final class OptionalRuleView {
  final int present;
  final RuleView? value;
  const OptionalRuleView(this.present, this.value);
  factory OptionalRuleView.copy(COptionalRuleView v) => OptionalRuleView(
      v.present, (v.present != 0 ? RuleView.copy(v.value) : null));
}

final class ChoiceView {
  final StringView name;
  final OptionalContentView description;
  final OptionalDoubleView weight;
  const ChoiceView(this.name, this.description, this.weight);
  factory ChoiceView.copy(CChoiceView v) => ChoiceView(
      StringView.copy(v.name),
      OptionalContentView.copy(v.description),
      OptionalDoubleView.copy(v.weight));
}

final class ChoicesView {
  final List<ChoiceView> data;
  final int len;
  const ChoicesView(this.data, this.len);
  factory ChoicesView.copy(CChoicesView v) => ChoicesView(
      List.unmodifiable(
          List.generate(v.len, (i) => ChoiceView.copy(v.data[i]))),
      v.len);
}

final class RelationView {
  final StringView name;
  final StringView source;
  final StringView target;
  final OptionalStringView reads;
  final int either;
  final int single;
  const RelationView(this.name, this.source, this.target, this.reads,
      this.either, this.single);
  factory RelationView.copy(CRelationView v) => RelationView(
      StringView.copy(v.name),
      StringView.copy(v.source),
      StringView.copy(v.target),
      OptionalStringView.copy(v.reads),
      v.either,
      v.single);
}

final class RelationsView {
  final List<RelationView> data;
  final int len;
  const RelationsView(this.data, this.len);
  factory RelationsView.copy(CRelationsView v) => RelationsView(
      List.unmodifiable(
          List.generate(v.len, (i) => RelationView.copy(v.data[i]))),
      v.len);
}

final class QuestionMemberView {
  final StringView name;
  final QuestionView? question;
  const QuestionMemberView(this.name, this.question);
  factory QuestionMemberView.copy(CQuestionMemberView v) => QuestionMemberView(
      StringView.copy(v.name),
      (v.question.address == 0 ? null : QuestionView.copy(v.question.ref)));
}

final class QuestionMembersView {
  final List<QuestionMemberView> data;
  final int len;
  const QuestionMembersView(this.data, this.len);
  factory QuestionMembersView.copy(CQuestionMembersView v) =>
      QuestionMembersView(
          List.unmodifiable(
              List.generate(v.len, (i) => QuestionMemberView.copy(v.data[i]))),
          v.len);
}

final class QuestionView {
  final int kind;
  final ContentView text;
  final OptionalContentView yes;
  final OptionalContentView no;
  final ChoicesView choices;
  final RuleView threshold;
  final RuleView relation_threshold;
  final OptionalStringView model;
  final OptionalStringView profile;
  final OptionalSizeView batch;
  final int batch_max;
  final int none;
  final StringsView on;
  final QuestionMembersView members;
  final ChoicesView kinds;
  final RelationsView relations;
  final OptionalStringView name_pointer;
  final OptionalStringView kind_pointer;
  const QuestionView(
      this.kind,
      this.text,
      this.yes,
      this.no,
      this.choices,
      this.threshold,
      this.relation_threshold,
      this.model,
      this.profile,
      this.batch,
      this.batch_max,
      this.none,
      this.on,
      this.members,
      this.kinds,
      this.relations,
      this.name_pointer,
      this.kind_pointer);
  factory QuestionView.copy(CQuestionView v) => QuestionView(
      v.kind,
      ContentView.copy(v.text),
      OptionalContentView.copy(v.yes),
      OptionalContentView.copy(v.no),
      ChoicesView.copy(v.choices),
      RuleView.copy(v.threshold),
      RuleView.copy(v.relation_threshold),
      OptionalStringView.copy(v.model),
      OptionalStringView.copy(v.profile),
      OptionalSizeView.copy(v.batch),
      v.batch_max,
      v.none,
      StringsView.copy(v.on),
      QuestionMembersView.copy(v.members),
      ChoicesView.copy(v.kinds),
      RelationsView.copy(v.relations),
      OptionalStringView.copy(v.name_pointer),
      OptionalStringView.copy(v.kind_pointer));
}

final class OptionalQuestionView {
  final int present;
  final QuestionView? value;
  const OptionalQuestionView(this.present, this.value);
  factory OptionalQuestionView.copy(COptionalQuestionView v) =>
      OptionalQuestionView(
          v.present, (v.present != 0 ? QuestionView.copy(v.value) : null));
}

final class ImageView {
  final int media;
  final List<int> bytes;
  final int bytes_len;
  final int width;
  final int height;
  final OptionalStringView filename;
  const ImageView(this.media, this.bytes, this.bytes_len, this.width,
      this.height, this.filename);
  factory ImageView.copy(CImageView v) => ImageView(
      v.media,
      List.unmodifiable(v.bytes.asTypedList(v.bytes_len)),
      v.bytes_len,
      v.width,
      v.height,
      OptionalStringView.copy(v.filename));
}

final class ImageViewsView {
  final List<ImageView> data;
  final int len;
  const ImageViewsView(this.data, this.len);
  factory ImageViewsView.copy(CImageViewsView v) => ImageViewsView(
      List.unmodifiable(List.generate(v.len, (i) => ImageView.copy(v.data[i]))),
      v.len);
}

final class OptionalImageViewsView {
  final int present;
  final ImageViewsView? value;
  const OptionalImageViewsView(this.present, this.value);
  factory OptionalImageViewsView.copy(COptionalImageViewsView v) =>
      OptionalImageViewsView(
          v.present, (v.present != 0 ? ImageViewsView.copy(v.value) : null));
}

final class SourceSpecView {
  final StringsView paths;
  final int unit;
  final int window;
  const SourceSpecView(this.paths, this.unit, this.window);
  factory SourceSpecView.copy(CSourceSpecView v) =>
      SourceSpecView(StringsView.copy(v.paths), v.unit, v.window);
}

final class DecideValueV1DataView {
  final int? boolean;
  final ContentView? authored;
  const DecideValueV1DataView(this.boolean, this.authored);
  factory DecideValueV1DataView.copy(
          CDecideValueV1DataView v, String? active) =>
      DecideValueV1DataView((active == "boolean" ? v.boolean : null),
          (active == "authored" ? ContentView.copy(v.authored) : null));
}

final class DecideValueView {
  final int kind;
  final DecideValueV1DataView data;
  const DecideValueView(this.kind, this.data);
  factory DecideValueView.copy(CDecideValueView v) => DecideValueView(
      v.kind,
      DecideValueV1DataView.copy(
          v.data,
          switch (v.kind) {
            0 => null,
            1 => "boolean",
            2 => "authored",
            _ => throw StateError("native discriminator")
          }));
}

final class ProbabilityView {
  final StringView name;
  final double probability;
  const ProbabilityView(this.name, this.probability);
  factory ProbabilityView.copy(CProbabilityView v) =>
      ProbabilityView(StringView.copy(v.name), v.probability);
}

final class ProbabilitiesView {
  final List<ProbabilityView> data;
  final int len;
  const ProbabilitiesView(this.data, this.len);
  factory ProbabilitiesView.copy(CProbabilitiesView v) => ProbabilitiesView(
      List.unmodifiable(
          List.generate(v.len, (i) => ProbabilityView.copy(v.data[i]))),
      v.len);
}

final class OptionalProbabilitiesView {
  final int present;
  final ProbabilitiesView? value;
  const OptionalProbabilitiesView(this.present, this.value);
  factory OptionalProbabilitiesView.copy(COptionalProbabilitiesView v) =>
      OptionalProbabilitiesView(
          v.present, (v.present != 0 ? ProbabilitiesView.copy(v.value) : null));
}

final class NamedAnswerView {
  final StringView pick;
  final ProbabilitiesView probabilities;
  final OptionalDoubleView confidence;
  const NamedAnswerView(this.pick, this.probabilities, this.confidence);
  factory NamedAnswerView.copy(CNamedAnswerView v) => NamedAnswerView(
      StringView.copy(v.pick),
      ProbabilitiesView.copy(v.probabilities),
      OptionalDoubleView.copy(v.confidence));
}

final class ScoreAnswerView {
  final StringView level;
  final ProbabilitiesView probabilities;
  final OptionalDoubleView confidence;
  const ScoreAnswerView(this.level, this.probabilities, this.confidence);
  factory ScoreAnswerView.copy(CScoreAnswerView v) => ScoreAnswerView(
      StringView.copy(v.level),
      ProbabilitiesView.copy(v.probabilities),
      OptionalDoubleView.copy(v.confidence));
}

final class AnswerV1DataView {
  final double? probability;
  final NamedAnswerView? choice;
  final ProbabilitiesView? tag;
  final ScoreAnswerView? score;
  final NamedAnswerView? find;
  const AnswerV1DataView(
      this.probability, this.choice, this.tag, this.score, this.find);
  factory AnswerV1DataView.copy(CAnswerV1DataView v, String? active) =>
      AnswerV1DataView(
          (active == "probability" ? v.probability : null),
          (active == "choice" ? NamedAnswerView.copy(v.choice) : null),
          (active == "tag" ? ProbabilitiesView.copy(v.tag) : null),
          (active == "score" ? ScoreAnswerView.copy(v.score) : null),
          (active == "find" ? NamedAnswerView.copy(v.find) : null));
}

final class AnswerView {
  final int kind;
  final AnswerV1DataView data;
  const AnswerView(this.kind, this.data);
  factory AnswerView.copy(CAnswerView v) => AnswerView(
      v.kind,
      AnswerV1DataView.copy(
          v.data,
          switch (v.kind) {
            1 => "probability",
            2 => "choice",
            3 => "tag",
            4 => "score",
            5 => "find",
            _ => throw StateError("native discriminator")
          }));
}

final class OptionalAnswerView {
  final int present;
  final AnswerView? value;
  const OptionalAnswerView(this.present, this.value);
  factory OptionalAnswerView.copy(COptionalAnswerView v) => OptionalAnswerView(
      v.present, (v.present != 0 ? AnswerView.copy(v.value) : null));
}

final class LocationView {
  final OptionalStringView file;
  final OptionalSizeView first_line;
  final OptionalSizeView last_line;
  const LocationView(this.file, this.first_line, this.last_line);
  factory LocationView.copy(CLocationView v) => LocationView(
      OptionalStringView.copy(v.file),
      OptionalSizeView.copy(v.first_line),
      OptionalSizeView.copy(v.last_line));
}

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
  final QuestionView question;
  final int state;
  final MemberV1DataView data;
  const MemberView(
      this.name, this.request, this.question, this.state, this.data);
  factory MemberView.copy(CMemberView v) => MemberView(
      StringView.copy(v.name),
      StringView.copy(v.request),
      QuestionView.copy(v.question),
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

final class InputDeclarationView {
  final int kind;
  final InputPropertiesView properties;
  final StringsView required;
  const InputDeclarationView(this.kind, this.properties, this.required);
  factory InputDeclarationView.copy(CInputDeclarationView v) =>
      InputDeclarationView(v.kind, InputPropertiesView.copy(v.properties),
          StringsView.copy(v.required));
}

final class QuestionAuthorView {
  final OptionalStringView name;
  final OptionalU64View wording_version;
  final InputDeclarationView item_schema;
  final InputDeclarationView context_schema;
  const QuestionAuthorView(
      this.name, this.wording_version, this.item_schema, this.context_schema);
  factory QuestionAuthorView.copy(CQuestionAuthorView v) => QuestionAuthorView(
      OptionalStringView.copy(v.name),
      OptionalU64View.copy(v.wording_version),
      InputDeclarationView.copy(v.item_schema),
      InputDeclarationView.copy(v.context_schema));
}
