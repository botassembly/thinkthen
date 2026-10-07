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
