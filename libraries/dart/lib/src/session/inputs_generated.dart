// Generated from the shared Rust graph. Do not edit.
part of thinkthen_session;

final class AnnotateDefinition extends _NativeValue {
  AnnotateDefinition._(super.fields);
  factory AnnotateDefinition.read(Object? value) =>
      AnnotateDefinition._(_readObject(value));
  AnnotateDefinition(
      {Presence<Object?> batch = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      required Map<String, AnnotationSpec> questions,
      Presence<DecisionThreshold> threshold = const Presence.absent()})
      : super({
          if (batch.isPresent) "batch": batch.value,
          if (profile.isPresent) "profile": profile.value,
          "questions": questions,
          if (threshold.isPresent) "threshold": threshold.value,
          "version": 1
        });
  Presence<Object?> get batch => _fields.containsKey("batch")
      ? Presence.present(_fields["batch"])
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  Map<String, AnnotationSpec> get questions =>
      (_fields["questions"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, AnnotationSpec.read(v)));
  Presence<DecisionThreshold> get threshold => _fields.containsKey("threshold")
      ? Presence.present(DecisionThreshold.read(_fields["threshold"]))
      : const Presence.absent();
  BigInt get version => _readInteger(_fields["version"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["batch", "profile", "questions", "threshold", "version"]
              .contains(entry.key))
            entry.key: entry.value
      });
}

sealed class AnnotationSpec extends _NativeValue {
  AnnotationSpec(super.fields);
  factory AnnotationSpec.read(Object? value) {
    final map = _readObject(value);
    if (map.containsKey("decide") &&
        !map.containsKey("choose") &&
        !map.containsKey("score") &&
        !map.containsKey("tag")) return AnnotateQuestionDecide.read(map);
    if (map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("score") &&
        !map.containsKey("tag")) return AnnotateQuestionChoose.read(map);
    if (map.containsKey("tag") &&
        !map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("score")) return AnnotateQuestionTag.read(map);
    if (map.containsKey("score") &&
        !map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("tag")) return AnnotateQuestionScore.read(map);
    throw FormatException("Unknown AnnotationSpec alternative");
  }
}

final class AnnotateQuestionChoose extends AnnotationSpec {
  AnnotateQuestionChoose._(super.fields);
  factory AnnotateQuestionChoose.read(Object? value) =>
      AnnotateQuestionChoose._(_readObject(value));
  AnnotateQuestionChoose(
      {required Wording choose,
      Presence<QuestionInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<QuestionInputDeclaration> itemSchema = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<Pointers> on = const Presence.absent(),
      Presence<ChoiceOptions> options = const Presence.absent(),
      Presence<Cut> threshold = const Presence.absent(),
      Presence<BigInt> wordingVersion = const Presence.absent()})
      : super({
          "choose": choose,
          if (contextSchema.isPresent) "context_schema": contextSchema.value,
          if (itemSchema.isPresent) "item_schema": itemSchema.value,
          if (name.isPresent) "name": name.value,
          if (on.isPresent) "on": on.value,
          if (options.isPresent) "options": options.value,
          if (threshold.isPresent) "threshold": threshold.value,
          if (wordingVersion.isPresent) "wording_version": wordingVersion.value
        });
  Wording get choose => Wording.read(_fields["choose"]);
  Presence<QuestionInputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(
              QuestionInputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Presence<QuestionInputDeclaration> get itemSchema => _fields
          .containsKey("item_schema")
      ? Presence.present(QuestionInputDeclaration.read(_fields["item_schema"]))
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<Pointers> get on => _fields.containsKey("on")
      ? Presence.present(Pointers.read(_fields["on"]))
      : const Presence.absent();
  Presence<ChoiceOptions> get options => _fields.containsKey("options")
      ? Presence.present(ChoiceOptions.read(_fields["options"]))
      : const Presence.absent();
  Presence<Cut> get threshold => _fields.containsKey("threshold")
      ? Presence.present(Cut.read(_fields["threshold"]))
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "choose",
            "context_schema",
            "item_schema",
            "name",
            "on",
            "options",
            "threshold",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class AnnotateQuestionDecide extends AnnotationSpec {
  AnnotateQuestionDecide._(super.fields);
  factory AnnotateQuestionDecide.read(Object? value) =>
      AnnotateQuestionDecide._(_readObject(value));
  AnnotateQuestionDecide(
      {Presence<QuestionInputDeclaration> contextSchema =
          const Presence.absent(),
      required Wording decide,
      Presence<Reading> falseValue = const Presence.absent(),
      Presence<QuestionInputDeclaration> itemSchema = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<Pointers> on = const Presence.absent(),
      Presence<DecisionThreshold> threshold = const Presence.absent(),
      Presence<Reading> trueValue = const Presence.absent(),
      Presence<BigInt> wordingVersion = const Presence.absent()})
      : super({
          if (contextSchema.isPresent) "context_schema": contextSchema.value,
          "decide": decide,
          if (falseValue.isPresent) "false": falseValue.value,
          if (itemSchema.isPresent) "item_schema": itemSchema.value,
          if (name.isPresent) "name": name.value,
          if (on.isPresent) "on": on.value,
          if (threshold.isPresent) "threshold": threshold.value,
          if (trueValue.isPresent) "true": trueValue.value,
          if (wordingVersion.isPresent) "wording_version": wordingVersion.value
        });
  Presence<QuestionInputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(
              QuestionInputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Wording get decide => Wording.read(_fields["decide"]);
  Presence<Reading> get falseValue => _fields.containsKey("false")
      ? Presence.present(Reading.read(_fields["false"]))
      : const Presence.absent();
  Presence<QuestionInputDeclaration> get itemSchema => _fields
          .containsKey("item_schema")
      ? Presence.present(QuestionInputDeclaration.read(_fields["item_schema"]))
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<Pointers> get on => _fields.containsKey("on")
      ? Presence.present(Pointers.read(_fields["on"]))
      : const Presence.absent();
  Presence<DecisionThreshold> get threshold => _fields.containsKey("threshold")
      ? Presence.present(DecisionThreshold.read(_fields["threshold"]))
      : const Presence.absent();
  Presence<Reading> get trueValue => _fields.containsKey("true")
      ? Presence.present(Reading.read(_fields["true"]))
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "context_schema",
            "decide",
            "false",
            "item_schema",
            "name",
            "on",
            "threshold",
            "true",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class AnnotateQuestionScore extends AnnotationSpec {
  AnnotateQuestionScore._(super.fields);
  factory AnnotateQuestionScore.read(Object? value) =>
      AnnotateQuestionScore._(_readObject(value));
  AnnotateQuestionScore(
      {Presence<QuestionInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<QuestionInputDeclaration> itemSchema = const Presence.absent(),
      Presence<Levels> levels = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<Pointers> on = const Presence.absent(),
      required Wording score,
      Presence<BigInt> wordingVersion = const Presence.absent()})
      : super({
          if (contextSchema.isPresent) "context_schema": contextSchema.value,
          if (itemSchema.isPresent) "item_schema": itemSchema.value,
          if (levels.isPresent) "levels": levels.value,
          if (name.isPresent) "name": name.value,
          if (on.isPresent) "on": on.value,
          "score": score,
          if (wordingVersion.isPresent) "wording_version": wordingVersion.value
        });
  Presence<QuestionInputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(
              QuestionInputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Presence<QuestionInputDeclaration> get itemSchema => _fields
          .containsKey("item_schema")
      ? Presence.present(QuestionInputDeclaration.read(_fields["item_schema"]))
      : const Presence.absent();
  Presence<Levels> get levels => _fields.containsKey("levels")
      ? Presence.present(Levels.read(_fields["levels"]))
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<Pointers> get on => _fields.containsKey("on")
      ? Presence.present(Pointers.read(_fields["on"]))
      : const Presence.absent();
  Wording get score => Wording.read(_fields["score"]);
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "context_schema",
            "item_schema",
            "levels",
            "name",
            "on",
            "score",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class AnnotateQuestionTag extends AnnotationSpec {
  AnnotateQuestionTag._(super.fields);
  factory AnnotateQuestionTag.read(Object? value) =>
      AnnotateQuestionTag._(_readObject(value));
  AnnotateQuestionTag(
      {Presence<QuestionInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<QuestionInputDeclaration> itemSchema = const Presence.absent(),
      Presence<Labels> labels = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<Pointers> on = const Presence.absent(),
      required Wording tag,
      Presence<Cut> threshold = const Presence.absent(),
      Presence<BigInt> wordingVersion = const Presence.absent()})
      : super({
          if (contextSchema.isPresent) "context_schema": contextSchema.value,
          if (itemSchema.isPresent) "item_schema": itemSchema.value,
          if (labels.isPresent) "labels": labels.value,
          if (name.isPresent) "name": name.value,
          if (on.isPresent) "on": on.value,
          "tag": tag,
          if (threshold.isPresent) "threshold": threshold.value,
          if (wordingVersion.isPresent) "wording_version": wordingVersion.value
        });
  Presence<QuestionInputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(
              QuestionInputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Presence<QuestionInputDeclaration> get itemSchema => _fields
          .containsKey("item_schema")
      ? Presence.present(QuestionInputDeclaration.read(_fields["item_schema"]))
      : const Presence.absent();
  Presence<Labels> get labels => _fields.containsKey("labels")
      ? Presence.present(Labels.read(_fields["labels"]))
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<Pointers> get on => _fields.containsKey("on")
      ? Presence.present(Pointers.read(_fields["on"]))
      : const Presence.absent();
  Wording get tag => Wording.read(_fields["tag"]);
  Presence<Cut> get threshold => _fields.containsKey("threshold")
      ? Presence.present(Cut.read(_fields["threshold"]))
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "context_schema",
            "item_schema",
            "labels",
            "name",
            "on",
            "tag",
            "threshold",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class Choose extends _NativeValue {
  Choose._(super.fields);
  factory Choose.read(Object? value) => Choose._(_readObject(value));
  Choose(
      {Presence<QuestionBatch> batch = const Presence.absent(),
      required Wording choose,
      Presence<QuestionInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<QuestionInputDeclaration> itemSchema = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<Pointers> on = const Presence.absent(),
      Presence<ChoiceOptions> options = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      Presence<Cut> threshold = const Presence.absent(),
      Presence<BigInt> wordingVersion = const Presence.absent()})
      : super({
          if (batch.isPresent) "batch": batch.value,
          "choose": choose,
          if (contextSchema.isPresent) "context_schema": contextSchema.value,
          if (itemSchema.isPresent) "item_schema": itemSchema.value,
          if (model.isPresent) "model": model.value,
          if (name.isPresent) "name": name.value,
          if (on.isPresent) "on": on.value,
          if (options.isPresent) "options": options.value,
          if (profile.isPresent) "profile": profile.value,
          if (threshold.isPresent) "threshold": threshold.value,
          if (wordingVersion.isPresent) "wording_version": wordingVersion.value
        });
  Presence<QuestionBatch> get batch => _fields.containsKey("batch")
      ? Presence.present(QuestionBatch.read(_fields["batch"]))
      : const Presence.absent();
  Wording get choose => Wording.read(_fields["choose"]);
  Presence<QuestionInputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(
              QuestionInputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Presence<QuestionInputDeclaration> get itemSchema => _fields
          .containsKey("item_schema")
      ? Presence.present(QuestionInputDeclaration.read(_fields["item_schema"]))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<Pointers> get on => _fields.containsKey("on")
      ? Presence.present(Pointers.read(_fields["on"]))
      : const Presence.absent();
  Presence<ChoiceOptions> get options => _fields.containsKey("options")
      ? Presence.present(ChoiceOptions.read(_fields["options"]))
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  Presence<Cut> get threshold => _fields.containsKey("threshold")
      ? Presence.present(Cut.read(_fields["threshold"]))
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "batch",
            "choose",
            "context_schema",
            "item_schema",
            "model",
            "name",
            "on",
            "options",
            "profile",
            "threshold",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class Reading implements _EncodedValue {
  final Object? value;
  const Reading._(this.value);
  factory Reading.read(Object? value) =>
      value is Reading ? value : Reading._(value);
  Object? _toValue() => value;
  const Reading.text(String this.value);
  String get asText => value as String;
  const Reading.fields(Map<String, Object?> this.value);
  Map<String, Object?> get asFields =>
      (value as Map<String, Object?>).map((k, v) => MapEntry(k, v));
  const Reading.list(List<Object?> this.value);
  List<Object?> get asList =>
      (value as List).map((v) => v).toList(growable: false);
  const Reading.nullValue() : value = null;
}

final class Cut implements _EncodedValue {
  final Object? value;
  const Cut._(this.value);
  factory Cut.read(Object? value) => value is Cut ? value : Cut._(value);
  Object? _toValue() => value;
  const Cut.number(num this.value);
  num get asNumber => value as num;
  const Cut.text(String this.value);
  String get asText => value as String;
}

final class Decide extends _NativeValue {
  Decide._(super.fields);
  factory Decide.read(Object? value) => Decide._(_readObject(value));
  Decide(
      {Presence<QuestionBatch> batch = const Presence.absent(),
      Presence<QuestionInputDeclaration> contextSchema =
          const Presence.absent(),
      required Wording decide,
      Presence<Reading> falseValue = const Presence.absent(),
      Presence<QuestionInputDeclaration> itemSchema = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<Pointers> on = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      Presence<DecisionThreshold> threshold = const Presence.absent(),
      Presence<Reading> trueValue = const Presence.absent(),
      Presence<BigInt> wordingVersion = const Presence.absent()})
      : super({
          if (batch.isPresent) "batch": batch.value,
          if (contextSchema.isPresent) "context_schema": contextSchema.value,
          "decide": decide,
          if (falseValue.isPresent) "false": falseValue.value,
          if (itemSchema.isPresent) "item_schema": itemSchema.value,
          if (model.isPresent) "model": model.value,
          if (name.isPresent) "name": name.value,
          if (on.isPresent) "on": on.value,
          if (profile.isPresent) "profile": profile.value,
          if (threshold.isPresent) "threshold": threshold.value,
          if (trueValue.isPresent) "true": trueValue.value,
          if (wordingVersion.isPresent) "wording_version": wordingVersion.value
        });
  Presence<QuestionBatch> get batch => _fields.containsKey("batch")
      ? Presence.present(QuestionBatch.read(_fields["batch"]))
      : const Presence.absent();
  Presence<QuestionInputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(
              QuestionInputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Wording get decide => Wording.read(_fields["decide"]);
  Presence<Reading> get falseValue => _fields.containsKey("false")
      ? Presence.present(Reading.read(_fields["false"]))
      : const Presence.absent();
  Presence<QuestionInputDeclaration> get itemSchema => _fields
          .containsKey("item_schema")
      ? Presence.present(QuestionInputDeclaration.read(_fields["item_schema"]))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<Pointers> get on => _fields.containsKey("on")
      ? Presence.present(Pointers.read(_fields["on"]))
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  Presence<DecisionThreshold> get threshold => _fields.containsKey("threshold")
      ? Presence.present(DecisionThreshold.read(_fields["threshold"]))
      : const Presence.absent();
  Presence<Reading> get trueValue => _fields.containsKey("true")
      ? Presence.present(Reading.read(_fields["true"]))
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "batch",
            "context_schema",
            "decide",
            "false",
            "item_schema",
            "model",
            "name",
            "on",
            "profile",
            "threshold",
            "true",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class FindQuestionDefinition extends _NativeValue {
  FindQuestionDefinition._(super.fields);
  factory FindQuestionDefinition.read(Object? value) =>
      FindQuestionDefinition._(_readObject(value));
  FindQuestionDefinition(
      {Presence<QuestionInputDeclaration> contextSchema =
          const Presence.absent(),
      required Wording find,
      Presence<QuestionInputDeclaration> itemSchema = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<Pointers> on = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      Presence<BigInt> wordingVersion = const Presence.absent()})
      : super({
          if (contextSchema.isPresent) "context_schema": contextSchema.value,
          "find": find,
          if (itemSchema.isPresent) "item_schema": itemSchema.value,
          if (model.isPresent) "model": model.value,
          if (name.isPresent) "name": name.value,
          if (on.isPresent) "on": on.value,
          if (profile.isPresent) "profile": profile.value,
          if (wordingVersion.isPresent) "wording_version": wordingVersion.value
        });
  Presence<QuestionInputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(
              QuestionInputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Wording get find => Wording.read(_fields["find"]);
  Presence<QuestionInputDeclaration> get itemSchema => _fields
          .containsKey("item_schema")
      ? Presence.present(QuestionInputDeclaration.read(_fields["item_schema"]))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<Pointers> get on => _fields.containsKey("on")
      ? Presence.present(Pointers.read(_fields["on"]))
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "context_schema",
            "find",
            "item_schema",
            "model",
            "name",
            "on",
            "profile",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

sealed class QuestionInputDeclaration extends _NativeValue {
  QuestionInputDeclaration(super.fields);
  factory QuestionInputDeclaration.read(Object? value) {
    final map = _readObject(value);
    if (map["type"] == "string") return InputDeclarationString.read(map);
    if (map["type"] == "object") return InputDeclarationObject.read(map);
    throw FormatException("Unknown QuestionInputDeclaration alternative");
  }
}

final class InputDeclarationObject extends QuestionInputDeclaration {
  InputDeclarationObject._(super.fields);
  factory InputDeclarationObject.read(Object? value) =>
      InputDeclarationObject._(_readObject(value));
  InputDeclarationObject(
      {required Map<String, InputProperty> properties,
      Presence<List<String>> required = const Presence.absent()})
      : super({
          "properties": properties,
          if (required.isPresent) "required": required.value,
          "type": "object"
        });
  Map<String, InputProperty> get properties =>
      (_fields["properties"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, InputProperty.read(v)));
  Presence<List<String>> get required => _fields.containsKey("required")
      ? Presence.present((_fields["required"] as List)
          .map((v) => v as String)
          .toList(growable: false))
      : const Presence.absent();
  String get type => _fields["type"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["properties", "required", "type"].contains(entry.key))
            entry.key: entry.value
      });
}

final class InputDeclarationString extends QuestionInputDeclaration {
  InputDeclarationString._(super.fields);
  factory InputDeclarationString.read(Object? value) =>
      InputDeclarationString._(_readObject(value));
  InputDeclarationString() : super({"type": "string"});
  String get type => _fields["type"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["type"].contains(entry.key)) entry.key: entry.value
      });
}

sealed class InputProperty extends _NativeValue {
  InputProperty(super.fields);
  factory InputProperty.read(Object? value) {
    final map = _readObject(value);
    if (map["type"] == "string") return InputPropertyString.read(map);
    if (map["type"] == "number") return InputPropertyNumber.read(map);
    if (map["type"] == "boolean") return InputPropertyBoolean.read(map);
    if (map["type"] == "array") return InputPropertyArray.read(map);
    throw FormatException("Unknown InputProperty alternative");
  }
}

final class InputPropertyArray extends InputProperty {
  InputPropertyArray._(super.fields);
  factory InputPropertyArray.read(Object? value) =>
      InputPropertyArray._(_readObject(value));
  InputPropertyArray({required Object? items})
      : super({"items": items, "type": "array"});
  Object? get items => _fields["items"];
  String get type => _fields["type"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["items", "type"].contains(entry.key)) entry.key: entry.value
      });
}

final class InputPropertyBoolean extends InputProperty {
  InputPropertyBoolean._(super.fields);
  factory InputPropertyBoolean.read(Object? value) =>
      InputPropertyBoolean._(_readObject(value));
  InputPropertyBoolean() : super({"type": "boolean"});
  String get type => _fields["type"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["type"].contains(entry.key)) entry.key: entry.value
      });
}

final class InputPropertyNumber extends InputProperty {
  InputPropertyNumber._(super.fields);
  factory InputPropertyNumber.read(Object? value) =>
      InputPropertyNumber._(_readObject(value));
  InputPropertyNumber() : super({"type": "number"});
  String get type => _fields["type"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["type"].contains(entry.key)) entry.key: entry.value
      });
}

final class InputPropertyString extends InputProperty {
  InputPropertyString._(super.fields);
  factory InputPropertyString.read(Object? value) =>
      InputPropertyString._(_readObject(value));
  InputPropertyString() : super({"type": "string"});
  String get type => _fields["type"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["type"].contains(entry.key)) entry.key: entry.value
      });
}

final class Labels implements _EncodedValue {
  final Object? value;
  const Labels._(this.value);
  factory Labels.read(Object? value) =>
      value is Labels ? value : Labels._(value);
  Object? _toValue() => value;
  const Labels.list(List<String> this.value);
  List<String> get asList =>
      (value as List).map((v) => v as String).toList(growable: false);
  const Labels.fields(Map<String, Object?> this.value);
  Map<String, Object?> get asFields =>
      (value as Map<String, Object?>).map((k, v) => MapEntry(k, v));
}

final class Levels implements _EncodedValue {
  final Object? value;
  const Levels._(this.value);
  factory Levels.read(Object? value) =>
      value is Levels ? value : Levels._(value);
  Object? _toValue() => value;
  const Levels.list(List<String> this.value);
  List<String> get asList =>
      (value as List).map((v) => v as String).toList(growable: false);
  const Levels.fields(Map<String, Reading> this.value);
  Map<String, Reading> get asFields => (value as Map<String, Object?>)
      .map((k, v) => MapEntry(k, Reading.read(v)));
}

final class ChoiceOptions implements _EncodedValue {
  final Object? value;
  const ChoiceOptions._(this.value);
  factory ChoiceOptions.read(Object? value) =>
      value is ChoiceOptions ? value : ChoiceOptions._(value);
  Object? _toValue() => value;
  const ChoiceOptions.list(List<String> this.value);
  List<String> get asList =>
      (value as List).map((v) => v as String).toList(growable: false);
  const ChoiceOptions.fields(Map<String, Object?> this.value);
  Map<String, Object?> get asFields =>
      (value as Map<String, Object?>).map((k, v) => MapEntry(k, v));
}

final class Pointers implements _EncodedValue {
  final Object? value;
  const Pointers._(this.value);
  factory Pointers.read(Object? value) =>
      value is Pointers ? value : Pointers._(value);
  Object? _toValue() => value;
  const Pointers.text(String this.value);
  String get asText => value as String;
  const Pointers.list(List<String> this.value);
  List<String> get asList =>
      (value as List).map((v) => v as String).toList(growable: false);
}

final class QuestionBatch implements _EncodedValue {
  final Object? value;
  const QuestionBatch._(this.value);
  factory QuestionBatch.read(Object? value) =>
      value is QuestionBatch ? value : QuestionBatch._(value);
  Object? _toValue() => value;
  const QuestionBatch.reading(String this.value);
  String get asReading => value as String;
  const QuestionBatch.integer(BigInt this.value);
  BigInt get asInteger => _readInteger(value);
}

final class Wording implements _EncodedValue {
  final Object? value;
  const Wording._(this.value);
  factory Wording.read(Object? value) =>
      value is Wording ? value : Wording._(value);
  Object? _toValue() => value;
  const Wording.text(String this.value);
  String get asText => value as String;
  const Wording.fields(Map<String, Object?> this.value);
  Map<String, Object?> get asFields =>
      (value as Map<String, Object?>).map((k, v) => MapEntry(k, v));
  const Wording.list(List<Object?> this.value);
  List<Object?> get asList =>
      (value as List).map((v) => v).toList(growable: false);
}

final class Relate extends _NativeValue {
  Relate._(super.fields);
  factory Relate.read(Object? value) => Relate._(_readObject(value));
  Relate(
      {Presence<QuestionInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<QuestionInputDeclaration> itemSchema = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      required RelationSpec relate,
      Presence<Cut> threshold = const Presence.absent(),
      Presence<BigInt> wordingVersion = const Presence.absent()})
      : super({
          if (contextSchema.isPresent) "context_schema": contextSchema.value,
          if (itemSchema.isPresent) "item_schema": itemSchema.value,
          if (model.isPresent) "model": model.value,
          if (name.isPresent) "name": name.value,
          if (profile.isPresent) "profile": profile.value,
          "relate": relate,
          if (threshold.isPresent) "threshold": threshold.value,
          "version": 1,
          if (wordingVersion.isPresent) "wording_version": wordingVersion.value
        });
  Presence<QuestionInputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(
              QuestionInputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Presence<QuestionInputDeclaration> get itemSchema => _fields
          .containsKey("item_schema")
      ? Presence.present(QuestionInputDeclaration.read(_fields["item_schema"]))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  RelationSpec get relate => RelationSpec.read(_fields["relate"]);
  Presence<Cut> get threshold => _fields.containsKey("threshold")
      ? Presence.present(Cut.read(_fields["threshold"]))
      : const Presence.absent();
  BigInt get version => _readInteger(_fields["version"]);
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "context_schema",
            "item_schema",
            "model",
            "name",
            "profile",
            "relate",
            "threshold",
            "version",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class RelationDefinition extends _NativeValue {
  RelationDefinition._(super.fields);
  factory RelationDefinition.read(Object? value) =>
      RelationDefinition._(_readObject(value));
  RelationDefinition(
      {Presence<bool> either = const Presence.absent(),
      required String name,
      Presence<String> reads = const Presence.absent(),
      Presence<bool> single = const Presence.absent(),
      Presence<String> source = const Presence.absent(),
      Presence<String> target = const Presence.absent()})
      : super({
          if (either.isPresent) "either": either.value,
          "name": name,
          if (reads.isPresent) "reads": reads.value,
          if (single.isPresent) "single": single.value,
          if (source.isPresent) "source": source.value,
          if (target.isPresent) "target": target.value
        });
  Presence<bool> get either => _fields.containsKey("either")
      ? Presence.present(_fields["either"] as bool)
      : const Presence.absent();
  String get name => _fields["name"] as String;
  Presence<String> get reads => _fields.containsKey("reads")
      ? Presence.present(_fields["reads"] as String)
      : const Presence.absent();
  Presence<bool> get single => _fields.containsKey("single")
      ? Presence.present(_fields["single"] as bool)
      : const Presence.absent();
  Presence<String> get source => _fields.containsKey("source")
      ? Presence.present(_fields["source"] as String)
      : const Presence.absent();
  Presence<String> get target => _fields.containsKey("target")
      ? Presence.present(_fields["target"] as String)
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["either", "name", "reads", "single", "source", "target"]
              .contains(entry.key))
            entry.key: entry.value
      });
}

final class Score extends _NativeValue {
  Score._(super.fields);
  factory Score.read(Object? value) => Score._(_readObject(value));
  Score(
      {Presence<QuestionBatch> batch = const Presence.absent(),
      Presence<QuestionInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<QuestionInputDeclaration> itemSchema = const Presence.absent(),
      Presence<Levels> levels = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<Pointers> on = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      required Wording score,
      Presence<BigInt> wordingVersion = const Presence.absent()})
      : super({
          if (batch.isPresent) "batch": batch.value,
          if (contextSchema.isPresent) "context_schema": contextSchema.value,
          if (itemSchema.isPresent) "item_schema": itemSchema.value,
          if (levels.isPresent) "levels": levels.value,
          if (model.isPresent) "model": model.value,
          if (name.isPresent) "name": name.value,
          if (on.isPresent) "on": on.value,
          if (profile.isPresent) "profile": profile.value,
          "score": score,
          if (wordingVersion.isPresent) "wording_version": wordingVersion.value
        });
  Presence<QuestionBatch> get batch => _fields.containsKey("batch")
      ? Presence.present(QuestionBatch.read(_fields["batch"]))
      : const Presence.absent();
  Presence<QuestionInputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(
              QuestionInputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Presence<QuestionInputDeclaration> get itemSchema => _fields
          .containsKey("item_schema")
      ? Presence.present(QuestionInputDeclaration.read(_fields["item_schema"]))
      : const Presence.absent();
  Presence<Levels> get levels => _fields.containsKey("levels")
      ? Presence.present(Levels.read(_fields["levels"]))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<Pointers> get on => _fields.containsKey("on")
      ? Presence.present(Pointers.read(_fields["on"]))
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  Wording get score => Wording.read(_fields["score"]);
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "batch",
            "context_schema",
            "item_schema",
            "levels",
            "model",
            "name",
            "on",
            "profile",
            "score",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class Tag extends _NativeValue {
  Tag._(super.fields);
  factory Tag.read(Object? value) => Tag._(_readObject(value));
  Tag(
      {Presence<QuestionBatch> batch = const Presence.absent(),
      Presence<QuestionInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<QuestionInputDeclaration> itemSchema = const Presence.absent(),
      Presence<Labels> labels = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<Pointers> on = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      required Wording tag,
      Presence<Cut> threshold = const Presence.absent(),
      Presence<BigInt> wordingVersion = const Presence.absent()})
      : super({
          if (batch.isPresent) "batch": batch.value,
          if (contextSchema.isPresent) "context_schema": contextSchema.value,
          if (itemSchema.isPresent) "item_schema": itemSchema.value,
          if (labels.isPresent) "labels": labels.value,
          if (model.isPresent) "model": model.value,
          if (name.isPresent) "name": name.value,
          if (on.isPresent) "on": on.value,
          if (profile.isPresent) "profile": profile.value,
          "tag": tag,
          if (threshold.isPresent) "threshold": threshold.value,
          if (wordingVersion.isPresent) "wording_version": wordingVersion.value
        });
  Presence<QuestionBatch> get batch => _fields.containsKey("batch")
      ? Presence.present(QuestionBatch.read(_fields["batch"]))
      : const Presence.absent();
  Presence<QuestionInputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(
              QuestionInputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Presence<QuestionInputDeclaration> get itemSchema => _fields
          .containsKey("item_schema")
      ? Presence.present(QuestionInputDeclaration.read(_fields["item_schema"]))
      : const Presence.absent();
  Presence<Labels> get labels => _fields.containsKey("labels")
      ? Presence.present(Labels.read(_fields["labels"]))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<Pointers> get on => _fields.containsKey("on")
      ? Presence.present(Pointers.read(_fields["on"]))
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  Wording get tag => Wording.read(_fields["tag"]);
  Presence<Cut> get threshold => _fields.containsKey("threshold")
      ? Presence.present(Cut.read(_fields["threshold"]))
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "batch",
            "context_schema",
            "item_schema",
            "labels",
            "model",
            "name",
            "on",
            "profile",
            "tag",
            "threshold",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class DecisionThreshold implements _EncodedValue {
  final Object? value;
  const DecisionThreshold._(this.value);
  factory DecisionThreshold.read(Object? value) =>
      value is DecisionThreshold ? value : DecisionThreshold._(value);
  Object? _toValue() => value;
  const DecisionThreshold.number(num this.value);
  num get asNumber => value as num;
  const DecisionThreshold.text(String this.value);
  String get asText => value as String;
}

final class CacheDocument implements _EncodedValue {
  final Object? value;
  const CacheDocument._(this.value);
  factory CacheDocument.read(Object? value) =>
      value is CacheDocument ? value : CacheDocument._(value);
  Object? _toValue() => value;
  const CacheDocument.text(String this.value);
  String get asText => value as String;
  const CacheDocument.boolean(bool this.value);
  bool get asBoolean => value as bool;
}

final class ContextSchema implements _EncodedValue {
  final Object? value;
  const ContextSchema._(this.value);
  factory ContextSchema.read(Object? value) =>
      value is ContextSchema ? value : ContextSchema._(value);
  Object? _toValue() => value;
  const ContextSchema.text(String this.value);
  String get asText => value as String;
  const ContextSchema.fields(Map<String, Object?> this.value);
  Map<String, Object?> get asFields =>
      (value as Map<String, Object?>).map((k, v) => MapEntry(k, v));
}

final class EngineSettings extends _NativeValue {
  EngineSettings._(super.fields);
  factory EngineSettings.read(Object? value) =>
      EngineSettings._(_readObject(value));
  EngineSettings(
      {Presence<String> backend = const Presence.absent(),
      Presence<String> baseUrl = const Presence.absent(),
      Presence<BatchOptions> batch = const Presence.absent(),
      Presence<CacheDocument> cache = const Presence.absent(),
      Presence<BigInt?> maxEstimatedInputTokensTotal = const Presence.absent(),
      Presence<BigInt> maxRequestBytes = const Presence.absent(),
      Presence<BigInt?> maxRequests = const Presence.absent(),
      Presence<BigInt?> maxRequestsTotal = const Presence.absent(),
      Presence<BigInt> maxRetries = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      Presence<Object?> proxy = const Presence.absent(),
      Presence<String> record = const Presence.absent(),
      Presence<bool> refreshCache = const Presence.absent(),
      Presence<String> replay = const Presence.absent(),
      Presence<BigInt> throttle = const Presence.absent(),
      Presence<BigInt> timeout = const Presence.absent(),
      Presence<String> usdPerMillionInput = const Presence.absent(),
      Presence<String> usdPerMillionOutput = const Presence.absent()})
      : super({
          if (backend.isPresent) "backend": backend.value,
          if (baseUrl.isPresent) "base_url": baseUrl.value,
          if (batch.isPresent) "batch": batch.value,
          if (cache.isPresent) "cache": cache.value,
          if (maxEstimatedInputTokensTotal.isPresent)
            "max_estimated_input_tokens_total":
                maxEstimatedInputTokensTotal.value,
          if (maxRequestBytes.isPresent)
            "max_request_bytes": maxRequestBytes.value,
          if (maxRequests.isPresent) "max_requests": maxRequests.value,
          if (maxRequestsTotal.isPresent)
            "max_requests_total": maxRequestsTotal.value,
          if (maxRetries.isPresent) "max_retries": maxRetries.value,
          if (model.isPresent) "model": model.value,
          if (profile.isPresent) "profile": profile.value,
          if (proxy.isPresent) "proxy": proxy.value,
          if (record.isPresent) "record": record.value,
          if (refreshCache.isPresent) "refresh_cache": refreshCache.value,
          if (replay.isPresent) "replay": replay.value,
          if (throttle.isPresent) "throttle": throttle.value,
          if (timeout.isPresent) "timeout": timeout.value,
          if (usdPerMillionInput.isPresent)
            "usd_per_million_input": usdPerMillionInput.value,
          if (usdPerMillionOutput.isPresent)
            "usd_per_million_output": usdPerMillionOutput.value
        });
  Presence<String> get backend => _fields.containsKey("backend")
      ? Presence.present(_fields["backend"] as String)
      : const Presence.absent();
  Presence<String> get baseUrl => _fields.containsKey("base_url")
      ? Presence.present(_fields["base_url"] as String)
      : const Presence.absent();
  Presence<BatchOptions> get batch => _fields.containsKey("batch")
      ? Presence.present(BatchOptions.read(_fields["batch"]))
      : const Presence.absent();
  Presence<CacheDocument> get cache => _fields.containsKey("cache")
      ? Presence.present(CacheDocument.read(_fields["cache"]))
      : const Presence.absent();
  Presence<BigInt?> get maxEstimatedInputTokensTotal =>
      _fields.containsKey("max_estimated_input_tokens_total")
          ? Presence.present(_fields["max_estimated_input_tokens_total"] == null
              ? null
              : _readInteger(_fields["max_estimated_input_tokens_total"]))
          : const Presence.absent();
  Presence<BigInt> get maxRequestBytes =>
      _fields.containsKey("max_request_bytes")
          ? Presence.present(_readInteger(_fields["max_request_bytes"]))
          : const Presence.absent();
  Presence<BigInt?> get maxRequests => _fields.containsKey("max_requests")
      ? Presence.present(_fields["max_requests"] == null
          ? null
          : _readInteger(_fields["max_requests"]))
      : const Presence.absent();
  Presence<BigInt?> get maxRequestsTotal =>
      _fields.containsKey("max_requests_total")
          ? Presence.present(_fields["max_requests_total"] == null
              ? null
              : _readInteger(_fields["max_requests_total"]))
          : const Presence.absent();
  Presence<BigInt> get maxRetries => _fields.containsKey("max_retries")
      ? Presence.present(_readInteger(_fields["max_retries"]))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  Presence<Object?> get proxy => _fields.containsKey("proxy")
      ? Presence.present(_fields["proxy"])
      : const Presence.absent();
  Presence<String> get record => _fields.containsKey("record")
      ? Presence.present(_fields["record"] as String)
      : const Presence.absent();
  Presence<bool> get refreshCache => _fields.containsKey("refresh_cache")
      ? Presence.present(_fields["refresh_cache"] as bool)
      : const Presence.absent();
  Presence<String> get replay => _fields.containsKey("replay")
      ? Presence.present(_fields["replay"] as String)
      : const Presence.absent();
  Presence<BigInt> get throttle => _fields.containsKey("throttle")
      ? Presence.present(_readInteger(_fields["throttle"]))
      : const Presence.absent();
  Presence<BigInt> get timeout => _fields.containsKey("timeout")
      ? Presence.present(_readInteger(_fields["timeout"]))
      : const Presence.absent();
  Presence<String> get usdPerMillionInput =>
      _fields.containsKey("usd_per_million_input")
          ? Presence.present(_fields["usd_per_million_input"] as String)
          : const Presence.absent();
  Presence<String> get usdPerMillionOutput =>
      _fields.containsKey("usd_per_million_output")
          ? Presence.present(_fields["usd_per_million_output"] as String)
          : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "backend",
            "base_url",
            "batch",
            "cache",
            "max_estimated_input_tokens_total",
            "max_request_bytes",
            "max_requests",
            "max_requests_total",
            "max_retries",
            "model",
            "profile",
            "proxy",
            "record",
            "refresh_cache",
            "replay",
            "throttle",
            "timeout",
            "usd_per_million_input",
            "usd_per_million_output"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class OptionSchema extends _NativeValue {
  OptionSchema._(super.fields);
  factory OptionSchema.read(Object? value) =>
      OptionSchema._(_readObject(value));
  OptionSchema(
      {Presence<Object?> description = const Presence.absent(),
      required String name})
      : super({
          if (description.isPresent) "description": description.value,
          "name": name
        });
  Presence<Object?> get description => _fields.containsKey("description")
      ? Presence.present(_fields["description"])
      : const Presence.absent();
  String get name => _fields["name"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["description", "name"].contains(entry.key))
            entry.key: entry.value
      });
}

final class RecognitionExample implements _EncodedValue {
  final Object? value;
  const RecognitionExample._(this.value);
  factory RecognitionExample.read(Object? value) =>
      value is RecognitionExample ? value : RecognitionExample._(value);
  Object? _toValue() => value;
  const RecognitionExample.text(String this.value);
  String get asText => value as String;
  const RecognitionExample.fields(RecognitionExampleText this.value);
  RecognitionExampleText get asFields => RecognitionExampleText.read(value);
}

final class RecognitionExampleEntity extends _NativeValue {
  RecognitionExampleEntity._(super.fields);
  factory RecognitionExampleEntity.read(Object? value) =>
      RecognitionExampleEntity._(_readObject(value));
  RecognitionExampleEntity(
      {required BigInt end, required String kind, required BigInt start})
      : super({"end": end, "kind": kind, "start": start});
  BigInt get end => _readInteger(_fields["end"]);
  String get kind => _fields["kind"] as String;
  BigInt get start => _readInteger(_fields["start"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["end", "kind", "start"].contains(entry.key))
            entry.key: entry.value
      });
}

final class RecognitionExampleText extends _NativeValue {
  RecognitionExampleText._(super.fields);
  factory RecognitionExampleText.read(Object? value) =>
      RecognitionExampleText._(_readObject(value));
  RecognitionExampleText(
      {required List<RecognitionExampleEntity> entities,
      Presence<List<String>> kinds = const Presence.absent(),
      required String text})
      : super({
          "entities": entities,
          if (kinds.isPresent) "kinds": kinds.value,
          "text": text
        });
  List<RecognitionExampleEntity> get entities => (_fields["entities"] as List)
      .map((v) => RecognitionExampleEntity.read(v))
      .toList(growable: false);
  Presence<List<String>> get kinds => _fields.containsKey("kinds")
      ? Presence.present((_fields["kinds"] as List)
          .map((v) => v as String)
          .toList(growable: false))
      : const Presence.absent();
  String get text => _fields["text"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["entities", "kinds", "text"].contains(entry.key))
            entry.key: entry.value
      });
}

final class RecognitionSeedSpan extends _NativeValue {
  RecognitionSeedSpan._(super.fields);
  factory RecognitionSeedSpan.read(Object? value) =>
      RecognitionSeedSpan._(_readObject(value));
  RecognitionSeedSpan(
      {required BigInt end,
      Presence<String> kind = const Presence.absent(),
      required BigInt start})
      : super({
          "end": end,
          if (kind.isPresent) "kind": kind.value,
          "start": start
        });
  BigInt get end => _readInteger(_fields["end"]);
  Presence<String> get kind => _fields.containsKey("kind")
      ? Presence.present(_fields["kind"] as String)
      : const Presence.absent();
  BigInt get start => _readInteger(_fields["start"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["end", "kind", "start"].contains(entry.key))
            entry.key: entry.value
      });
}

final class RecognitionContext extends _NativeValue {
  RecognitionContext._(super.fields);
  factory RecognitionContext.read(Object? value) =>
      RecognitionContext._(_readObject(value));
  RecognitionContext(
      {Presence<String> boundary = const Presence.absent(),
      Presence<String> kindEdge = const Presence.absent(),
      Presence<String> relation = const Presence.absent()})
      : super({
          if (boundary.isPresent) "boundary": boundary.value,
          if (kindEdge.isPresent) "kind_edge": kindEdge.value,
          if (relation.isPresent) "relation": relation.value
        });
  Presence<String> get boundary => _fields.containsKey("boundary")
      ? Presence.present(_fields["boundary"] as String)
      : const Presence.absent();
  Presence<String> get kindEdge => _fields.containsKey("kind_edge")
      ? Presence.present(_fields["kind_edge"] as String)
      : const Presence.absent();
  Presence<String> get relation => _fields.containsKey("relation")
      ? Presence.present(_fields["relation"] as String)
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["boundary", "kind_edge", "relation"].contains(entry.key))
            entry.key: entry.value
      });
}

final class RecognizeDefinition extends _NativeValue {
  RecognizeDefinition._(super.fields);
  factory RecognizeDefinition.read(Object? value) =>
      RecognizeDefinition._(_readObject(value));
  RecognizeDefinition(
      {Presence<QuestionInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<QuestionInputDeclaration> itemSchema = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<Pointers> on = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      required RecognitionSpec recognize,
      Presence<Cut> relationThreshold = const Presence.absent(),
      Presence<Cut> threshold = const Presence.absent(),
      Presence<BigInt> wordingVersion = const Presence.absent()})
      : super({
          if (contextSchema.isPresent) "context_schema": contextSchema.value,
          if (itemSchema.isPresent) "item_schema": itemSchema.value,
          if (model.isPresent) "model": model.value,
          if (name.isPresent) "name": name.value,
          if (on.isPresent) "on": on.value,
          if (profile.isPresent) "profile": profile.value,
          "recognize": recognize,
          if (relationThreshold.isPresent)
            "relation_threshold": relationThreshold.value,
          if (threshold.isPresent) "threshold": threshold.value,
          "version": 1,
          if (wordingVersion.isPresent) "wording_version": wordingVersion.value
        });
  Presence<QuestionInputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(
              QuestionInputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Presence<QuestionInputDeclaration> get itemSchema => _fields
          .containsKey("item_schema")
      ? Presence.present(QuestionInputDeclaration.read(_fields["item_schema"]))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<Pointers> get on => _fields.containsKey("on")
      ? Presence.present(Pointers.read(_fields["on"]))
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  RecognitionSpec get recognize => RecognitionSpec.read(_fields["recognize"]);
  Presence<Cut> get relationThreshold =>
      _fields.containsKey("relation_threshold")
          ? Presence.present(Cut.read(_fields["relation_threshold"]))
          : const Presence.absent();
  Presence<Cut> get threshold => _fields.containsKey("threshold")
      ? Presence.present(Cut.read(_fields["threshold"]))
      : const Presence.absent();
  BigInt get version => _readInteger(_fields["version"]);
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "context_schema",
            "item_schema",
            "model",
            "name",
            "on",
            "profile",
            "recognize",
            "relation_threshold",
            "threshold",
            "version",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class RecognitionSpec extends _NativeValue {
  RecognitionSpec._(super.fields);
  factory RecognitionSpec.read(Object? value) =>
      RecognitionSpec._(_readObject(value));
  RecognitionSpec(
      {Presence<Wording> entityDefinition = const Presence.absent(),
      Presence<Wording> instructions = const Presence.absent(),
      Presence<Map<String, Object?>> kinds = const Presence.absent(),
      Presence<String> mode = const Presence.absent(),
      Presence<List<RelationDefinition>> relations = const Presence.absent(),
      Presence<BigInt> snippetPieces = const Presence.absent(),
      Presence<RecognitionContext> stageContext = const Presence.absent()})
      : super({
          if (entityDefinition.isPresent)
            "entity_definition": entityDefinition.value,
          if (instructions.isPresent) "instructions": instructions.value,
          if (kinds.isPresent) "kinds": kinds.value,
          if (mode.isPresent) "mode": mode.value,
          if (relations.isPresent) "relations": relations.value,
          if (snippetPieces.isPresent) "snippet_pieces": snippetPieces.value,
          if (stageContext.isPresent) "stage_context": stageContext.value
        });
  Presence<Wording> get entityDefinition =>
      _fields.containsKey("entity_definition")
          ? Presence.present(Wording.read(_fields["entity_definition"]))
          : const Presence.absent();
  Presence<Wording> get instructions => _fields.containsKey("instructions")
      ? Presence.present(Wording.read(_fields["instructions"]))
      : const Presence.absent();
  Presence<Map<String, Object?>> get kinds => _fields.containsKey("kinds")
      ? Presence.present((_fields["kinds"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, v)))
      : const Presence.absent();
  Presence<String> get mode => _fields.containsKey("mode")
      ? Presence.present(_fields["mode"] as String)
      : const Presence.absent();
  Presence<List<RelationDefinition>> get relations =>
      _fields.containsKey("relations")
          ? Presence.present((_fields["relations"] as List)
              .map((v) => RelationDefinition.read(v))
              .toList(growable: false))
          : const Presence.absent();
  Presence<BigInt> get snippetPieces => _fields.containsKey("snippet_pieces")
      ? Presence.present(_readInteger(_fields["snippet_pieces"]))
      : const Presence.absent();
  Presence<RecognitionContext> get stageContext =>
      _fields.containsKey("stage_context")
          ? Presence.present(RecognitionContext.read(_fields["stage_context"]))
          : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "entity_definition",
            "instructions",
            "kinds",
            "mode",
            "relations",
            "snippet_pieces",
            "stage_context"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class EntityFields extends _NativeValue {
  EntityFields._(super.fields);
  factory EntityFields.read(Object? value) =>
      EntityFields._(_readObject(value));
  EntityFields({required String kind, required String name})
      : super({"kind": kind, "name": name});
  String get kind => _fields["kind"] as String;
  String get name => _fields["name"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "name"].contains(entry.key)) entry.key: entry.value
      });
}

final class RelationSpec extends _NativeValue {
  RelationSpec._(super.fields);
  factory RelationSpec.read(Object? value) =>
      RelationSpec._(_readObject(value));
  RelationSpec(
      {Presence<EntityFields> fields = const Presence.absent(),
      required List<RelationDefinition> relations})
      : super({
          if (fields.isPresent) "fields": fields.value,
          "relations": relations
        });
  Presence<EntityFields> get fields => _fields.containsKey("fields")
      ? Presence.present(EntityFields.read(_fields["fields"]))
      : const Presence.absent();
  List<RelationDefinition> get relations => (_fields["relations"] as List)
      .map((v) => RelationDefinition.read(v))
      .toList(growable: false);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["fields", "relations"].contains(entry.key))
            entry.key: entry.value
      });
}

final class BatchOptions implements _EncodedValue {
  final Object? value;
  const BatchOptions._(this.value);
  factory BatchOptions.read(Object? value) =>
      value is BatchOptions ? value : BatchOptions._(value);
  Object? _toValue() => value;
  const BatchOptions.integer(BigInt this.value);
  BigInt get asInteger => _readInteger(value);
  const BatchOptions.text(String this.value);
  String get asText => value as String;
}

sealed class Definition extends _NativeValue {
  Definition(super.fields);
  factory Definition.read(Object? value) {
    final map = _readObject(value);
    if (map.containsKey("decide") &&
        !map.containsKey("choose") &&
        !map.containsKey("find") &&
        !map.containsKey("questions") &&
        !map.containsKey("recognize") &&
        !map.containsKey("relate") &&
        !map.containsKey("score") &&
        !map.containsKey("tag") &&
        !map.containsKey("version")) return DecideQuestion.read(map);
    if (map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("find") &&
        !map.containsKey("questions") &&
        !map.containsKey("recognize") &&
        !map.containsKey("relate") &&
        !map.containsKey("score") &&
        !map.containsKey("tag") &&
        !map.containsKey("version")) return ChooseQuestion.read(map);
    if (map.containsKey("tag") &&
        !map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("find") &&
        !map.containsKey("questions") &&
        !map.containsKey("recognize") &&
        !map.containsKey("relate") &&
        !map.containsKey("score") &&
        !map.containsKey("version")) return TagQuestion.read(map);
    if (map.containsKey("score") &&
        !map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("find") &&
        !map.containsKey("questions") &&
        !map.containsKey("recognize") &&
        !map.containsKey("relate") &&
        !map.containsKey("tag") &&
        !map.containsKey("version")) return ScoreQuestion.read(map);
    if (map.containsKey("relate") &&
        map.containsKey("version") &&
        !map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("find") &&
        !map.containsKey("questions") &&
        !map.containsKey("recognize") &&
        !map.containsKey("score") &&
        !map.containsKey("tag")) return RelateQuestion.read(map);
    if (map.containsKey("find") &&
        !map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("questions") &&
        !map.containsKey("recognize") &&
        !map.containsKey("relate") &&
        !map.containsKey("score") &&
        !map.containsKey("tag") &&
        !map.containsKey("version")) return FindQuestion.read(map);
    if (map.containsKey("recognize") &&
        map.containsKey("version") &&
        !map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("find") &&
        !map.containsKey("questions") &&
        !map.containsKey("relate") &&
        !map.containsKey("score") &&
        !map.containsKey("tag")) return RecognizeQuestion.read(map);
    if (map.containsKey("questions") &&
        map.containsKey("version") &&
        !map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("find") &&
        !map.containsKey("recognize") &&
        !map.containsKey("relate") &&
        !map.containsKey("score") &&
        !map.containsKey("tag")) return AnnotateQuestion.read(map);
    throw FormatException("Unknown Definition alternative");
  }
}

final class AnnotateQuestion extends Definition {
  AnnotateQuestion._(super.fields);
  factory AnnotateQuestion.read(Object? value) =>
      AnnotateQuestion._(_readObject(value));
  AnnotateQuestion(
      {Presence<Object?> batch = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      required Map<String, AnnotationSpec> questions,
      Presence<DecisionThreshold> threshold = const Presence.absent()})
      : super({
          if (batch.isPresent) "batch": batch.value,
          if (profile.isPresent) "profile": profile.value,
          "questions": questions,
          if (threshold.isPresent) "threshold": threshold.value,
          "version": 1
        });
  Presence<Object?> get batch => _fields.containsKey("batch")
      ? Presence.present(_fields["batch"])
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  Map<String, AnnotationSpec> get questions =>
      (_fields["questions"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, AnnotationSpec.read(v)));
  Presence<DecisionThreshold> get threshold => _fields.containsKey("threshold")
      ? Presence.present(DecisionThreshold.read(_fields["threshold"]))
      : const Presence.absent();
  BigInt get version => _readInteger(_fields["version"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["batch", "profile", "questions", "threshold", "version"]
              .contains(entry.key))
            entry.key: entry.value
      });
}

final class ChooseQuestion extends Definition {
  ChooseQuestion._(super.fields);
  factory ChooseQuestion.read(Object? value) =>
      ChooseQuestion._(_readObject(value));
  ChooseQuestion(
      {Presence<QuestionBatch> batch = const Presence.absent(),
      required Wording choose,
      Presence<QuestionInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<QuestionInputDeclaration> itemSchema = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<Pointers> on = const Presence.absent(),
      Presence<ChoiceOptions> options = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      Presence<Cut> threshold = const Presence.absent(),
      Presence<BigInt> wordingVersion = const Presence.absent()})
      : super({
          if (batch.isPresent) "batch": batch.value,
          "choose": choose,
          if (contextSchema.isPresent) "context_schema": contextSchema.value,
          if (itemSchema.isPresent) "item_schema": itemSchema.value,
          if (model.isPresent) "model": model.value,
          if (name.isPresent) "name": name.value,
          if (on.isPresent) "on": on.value,
          if (options.isPresent) "options": options.value,
          if (profile.isPresent) "profile": profile.value,
          if (threshold.isPresent) "threshold": threshold.value,
          if (wordingVersion.isPresent) "wording_version": wordingVersion.value
        });
  Presence<QuestionBatch> get batch => _fields.containsKey("batch")
      ? Presence.present(QuestionBatch.read(_fields["batch"]))
      : const Presence.absent();
  Wording get choose => Wording.read(_fields["choose"]);
  Presence<QuestionInputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(
              QuestionInputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Presence<QuestionInputDeclaration> get itemSchema => _fields
          .containsKey("item_schema")
      ? Presence.present(QuestionInputDeclaration.read(_fields["item_schema"]))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<Pointers> get on => _fields.containsKey("on")
      ? Presence.present(Pointers.read(_fields["on"]))
      : const Presence.absent();
  Presence<ChoiceOptions> get options => _fields.containsKey("options")
      ? Presence.present(ChoiceOptions.read(_fields["options"]))
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  Presence<Cut> get threshold => _fields.containsKey("threshold")
      ? Presence.present(Cut.read(_fields["threshold"]))
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "batch",
            "choose",
            "context_schema",
            "item_schema",
            "model",
            "name",
            "on",
            "options",
            "profile",
            "threshold",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class DecideQuestion extends Definition {
  DecideQuestion._(super.fields);
  factory DecideQuestion.read(Object? value) =>
      DecideQuestion._(_readObject(value));
  DecideQuestion(
      {Presence<QuestionBatch> batch = const Presence.absent(),
      Presence<QuestionInputDeclaration> contextSchema =
          const Presence.absent(),
      required Wording decide,
      Presence<Reading> falseValue = const Presence.absent(),
      Presence<QuestionInputDeclaration> itemSchema = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<Pointers> on = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      Presence<DecisionThreshold> threshold = const Presence.absent(),
      Presence<Reading> trueValue = const Presence.absent(),
      Presence<BigInt> wordingVersion = const Presence.absent()})
      : super({
          if (batch.isPresent) "batch": batch.value,
          if (contextSchema.isPresent) "context_schema": contextSchema.value,
          "decide": decide,
          if (falseValue.isPresent) "false": falseValue.value,
          if (itemSchema.isPresent) "item_schema": itemSchema.value,
          if (model.isPresent) "model": model.value,
          if (name.isPresent) "name": name.value,
          if (on.isPresent) "on": on.value,
          if (profile.isPresent) "profile": profile.value,
          if (threshold.isPresent) "threshold": threshold.value,
          if (trueValue.isPresent) "true": trueValue.value,
          if (wordingVersion.isPresent) "wording_version": wordingVersion.value
        });
  Presence<QuestionBatch> get batch => _fields.containsKey("batch")
      ? Presence.present(QuestionBatch.read(_fields["batch"]))
      : const Presence.absent();
  Presence<QuestionInputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(
              QuestionInputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Wording get decide => Wording.read(_fields["decide"]);
  Presence<Reading> get falseValue => _fields.containsKey("false")
      ? Presence.present(Reading.read(_fields["false"]))
      : const Presence.absent();
  Presence<QuestionInputDeclaration> get itemSchema => _fields
          .containsKey("item_schema")
      ? Presence.present(QuestionInputDeclaration.read(_fields["item_schema"]))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<Pointers> get on => _fields.containsKey("on")
      ? Presence.present(Pointers.read(_fields["on"]))
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  Presence<DecisionThreshold> get threshold => _fields.containsKey("threshold")
      ? Presence.present(DecisionThreshold.read(_fields["threshold"]))
      : const Presence.absent();
  Presence<Reading> get trueValue => _fields.containsKey("true")
      ? Presence.present(Reading.read(_fields["true"]))
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "batch",
            "context_schema",
            "decide",
            "false",
            "item_schema",
            "model",
            "name",
            "on",
            "profile",
            "threshold",
            "true",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class FindQuestion extends Definition {
  FindQuestion._(super.fields);
  factory FindQuestion.read(Object? value) =>
      FindQuestion._(_readObject(value));
  FindQuestion(
      {Presence<QuestionInputDeclaration> contextSchema =
          const Presence.absent(),
      required Wording find,
      Presence<QuestionInputDeclaration> itemSchema = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<Pointers> on = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      Presence<BigInt> wordingVersion = const Presence.absent()})
      : super({
          if (contextSchema.isPresent) "context_schema": contextSchema.value,
          "find": find,
          if (itemSchema.isPresent) "item_schema": itemSchema.value,
          if (model.isPresent) "model": model.value,
          if (name.isPresent) "name": name.value,
          if (on.isPresent) "on": on.value,
          if (profile.isPresent) "profile": profile.value,
          if (wordingVersion.isPresent) "wording_version": wordingVersion.value
        });
  Presence<QuestionInputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(
              QuestionInputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Wording get find => Wording.read(_fields["find"]);
  Presence<QuestionInputDeclaration> get itemSchema => _fields
          .containsKey("item_schema")
      ? Presence.present(QuestionInputDeclaration.read(_fields["item_schema"]))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<Pointers> get on => _fields.containsKey("on")
      ? Presence.present(Pointers.read(_fields["on"]))
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "context_schema",
            "find",
            "item_schema",
            "model",
            "name",
            "on",
            "profile",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class RelateQuestion extends Definition {
  RelateQuestion._(super.fields);
  factory RelateQuestion.read(Object? value) =>
      RelateQuestion._(_readObject(value));
  RelateQuestion(
      {Presence<QuestionInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<QuestionInputDeclaration> itemSchema = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      required RelationSpec relate,
      Presence<Cut> threshold = const Presence.absent(),
      Presence<BigInt> wordingVersion = const Presence.absent()})
      : super({
          if (contextSchema.isPresent) "context_schema": contextSchema.value,
          if (itemSchema.isPresent) "item_schema": itemSchema.value,
          if (model.isPresent) "model": model.value,
          if (name.isPresent) "name": name.value,
          if (profile.isPresent) "profile": profile.value,
          "relate": relate,
          if (threshold.isPresent) "threshold": threshold.value,
          "version": 1,
          if (wordingVersion.isPresent) "wording_version": wordingVersion.value
        });
  Presence<QuestionInputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(
              QuestionInputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Presence<QuestionInputDeclaration> get itemSchema => _fields
          .containsKey("item_schema")
      ? Presence.present(QuestionInputDeclaration.read(_fields["item_schema"]))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  RelationSpec get relate => RelationSpec.read(_fields["relate"]);
  Presence<Cut> get threshold => _fields.containsKey("threshold")
      ? Presence.present(Cut.read(_fields["threshold"]))
      : const Presence.absent();
  BigInt get version => _readInteger(_fields["version"]);
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "context_schema",
            "item_schema",
            "model",
            "name",
            "profile",
            "relate",
            "threshold",
            "version",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class ScoreQuestion extends Definition {
  ScoreQuestion._(super.fields);
  factory ScoreQuestion.read(Object? value) =>
      ScoreQuestion._(_readObject(value));
  ScoreQuestion(
      {Presence<QuestionBatch> batch = const Presence.absent(),
      Presence<QuestionInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<QuestionInputDeclaration> itemSchema = const Presence.absent(),
      Presence<Levels> levels = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<Pointers> on = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      required Wording score,
      Presence<BigInt> wordingVersion = const Presence.absent()})
      : super({
          if (batch.isPresent) "batch": batch.value,
          if (contextSchema.isPresent) "context_schema": contextSchema.value,
          if (itemSchema.isPresent) "item_schema": itemSchema.value,
          if (levels.isPresent) "levels": levels.value,
          if (model.isPresent) "model": model.value,
          if (name.isPresent) "name": name.value,
          if (on.isPresent) "on": on.value,
          if (profile.isPresent) "profile": profile.value,
          "score": score,
          if (wordingVersion.isPresent) "wording_version": wordingVersion.value
        });
  Presence<QuestionBatch> get batch => _fields.containsKey("batch")
      ? Presence.present(QuestionBatch.read(_fields["batch"]))
      : const Presence.absent();
  Presence<QuestionInputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(
              QuestionInputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Presence<QuestionInputDeclaration> get itemSchema => _fields
          .containsKey("item_schema")
      ? Presence.present(QuestionInputDeclaration.read(_fields["item_schema"]))
      : const Presence.absent();
  Presence<Levels> get levels => _fields.containsKey("levels")
      ? Presence.present(Levels.read(_fields["levels"]))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<Pointers> get on => _fields.containsKey("on")
      ? Presence.present(Pointers.read(_fields["on"]))
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  Wording get score => Wording.read(_fields["score"]);
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "batch",
            "context_schema",
            "item_schema",
            "levels",
            "model",
            "name",
            "on",
            "profile",
            "score",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class TagQuestion extends Definition {
  TagQuestion._(super.fields);
  factory TagQuestion.read(Object? value) => TagQuestion._(_readObject(value));
  TagQuestion(
      {Presence<QuestionBatch> batch = const Presence.absent(),
      Presence<QuestionInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<QuestionInputDeclaration> itemSchema = const Presence.absent(),
      Presence<Labels> labels = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<Pointers> on = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      required Wording tag,
      Presence<Cut> threshold = const Presence.absent(),
      Presence<BigInt> wordingVersion = const Presence.absent()})
      : super({
          if (batch.isPresent) "batch": batch.value,
          if (contextSchema.isPresent) "context_schema": contextSchema.value,
          if (itemSchema.isPresent) "item_schema": itemSchema.value,
          if (labels.isPresent) "labels": labels.value,
          if (model.isPresent) "model": model.value,
          if (name.isPresent) "name": name.value,
          if (on.isPresent) "on": on.value,
          if (profile.isPresent) "profile": profile.value,
          "tag": tag,
          if (threshold.isPresent) "threshold": threshold.value,
          if (wordingVersion.isPresent) "wording_version": wordingVersion.value
        });
  Presence<QuestionBatch> get batch => _fields.containsKey("batch")
      ? Presence.present(QuestionBatch.read(_fields["batch"]))
      : const Presence.absent();
  Presence<QuestionInputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(
              QuestionInputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Presence<QuestionInputDeclaration> get itemSchema => _fields
          .containsKey("item_schema")
      ? Presence.present(QuestionInputDeclaration.read(_fields["item_schema"]))
      : const Presence.absent();
  Presence<Labels> get labels => _fields.containsKey("labels")
      ? Presence.present(Labels.read(_fields["labels"]))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<Pointers> get on => _fields.containsKey("on")
      ? Presence.present(Pointers.read(_fields["on"]))
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  Wording get tag => Wording.read(_fields["tag"]);
  Presence<Cut> get threshold => _fields.containsKey("threshold")
      ? Presence.present(Cut.read(_fields["threshold"]))
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "batch",
            "context_schema",
            "item_schema",
            "labels",
            "model",
            "name",
            "on",
            "profile",
            "tag",
            "threshold",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class RecognizeQuestion extends Definition {
  RecognizeQuestion._(super.fields);
  factory RecognizeQuestion.read(Object? value) =>
      RecognizeQuestion._(_readObject(value));
  RecognizeQuestion(
      {Presence<QuestionInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<QuestionInputDeclaration> itemSchema = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<Pointers> on = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      required RecognitionSpec recognize,
      Presence<Cut> relationThreshold = const Presence.absent(),
      Presence<Cut> threshold = const Presence.absent(),
      Presence<BigInt> wordingVersion = const Presence.absent()})
      : super({
          if (contextSchema.isPresent) "context_schema": contextSchema.value,
          if (itemSchema.isPresent) "item_schema": itemSchema.value,
          if (model.isPresent) "model": model.value,
          if (name.isPresent) "name": name.value,
          if (on.isPresent) "on": on.value,
          if (profile.isPresent) "profile": profile.value,
          "recognize": recognize,
          if (relationThreshold.isPresent)
            "relation_threshold": relationThreshold.value,
          if (threshold.isPresent) "threshold": threshold.value,
          "version": 1,
          if (wordingVersion.isPresent) "wording_version": wordingVersion.value
        });
  Presence<QuestionInputDeclaration> get contextSchema =>
      _fields.containsKey("context_schema")
          ? Presence.present(
              QuestionInputDeclaration.read(_fields["context_schema"]))
          : const Presence.absent();
  Presence<QuestionInputDeclaration> get itemSchema => _fields
          .containsKey("item_schema")
      ? Presence.present(QuestionInputDeclaration.read(_fields["item_schema"]))
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<String> get name => _fields.containsKey("name")
      ? Presence.present(_fields["name"] as String)
      : const Presence.absent();
  Presence<Pointers> get on => _fields.containsKey("on")
      ? Presence.present(Pointers.read(_fields["on"]))
      : const Presence.absent();
  Presence<String> get profile => _fields.containsKey("profile")
      ? Presence.present(_fields["profile"] as String)
      : const Presence.absent();
  RecognitionSpec get recognize => RecognitionSpec.read(_fields["recognize"]);
  Presence<Cut> get relationThreshold =>
      _fields.containsKey("relation_threshold")
          ? Presence.present(Cut.read(_fields["relation_threshold"]))
          : const Presence.absent();
  Presence<Cut> get threshold => _fields.containsKey("threshold")
      ? Presence.present(Cut.read(_fields["threshold"]))
      : const Presence.absent();
  BigInt get version => _readInteger(_fields["version"]);
  Presence<BigInt> get wordingVersion => _fields.containsKey("wording_version")
      ? Presence.present(_readInteger(_fields["wording_version"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "context_schema",
            "item_schema",
            "model",
            "name",
            "on",
            "profile",
            "recognize",
            "relation_threshold",
            "threshold",
            "version",
            "wording_version"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

sealed class ImageAttachment extends _NativeValue {
  ImageAttachment(super.fields);
  factory ImageAttachment.read(Object? value) {
    final map = _readObject(value);
    if (map["kind"] == "file") return ImageFile.read(map);
    if (map["kind"] == "bytes") return ImageBytes.read(map);
    throw FormatException("Unknown ImageAttachment alternative");
  }
}

final class ImageBytes extends ImageAttachment {
  ImageBytes._(super.fields);
  factory ImageBytes.read(Object? value) => ImageBytes._(_readObject(value));
  ImageBytes({required String bytes, required String media})
      : super({"bytes": bytes, "kind": "bytes", "media": media});
  String get bytes => _fields["bytes"] as String;
  String get kind => _fields["kind"] as String;
  String get media => _fields["media"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["bytes", "kind", "media"].contains(entry.key))
            entry.key: entry.value
      });
}

final class ImageFile extends ImageAttachment {
  ImageFile._(super.fields);
  factory ImageFile.read(Object? value) => ImageFile._(_readObject(value));
  ImageFile(
      {Presence<String> media = const Presence.absent(), required String path})
      : super({
          "kind": "file",
          if (media.isPresent) "media": media.value,
          "path": path
        });
  String get kind => _fields["kind"] as String;
  Presence<String> get media => _fields.containsKey("media")
      ? Presence.present(_fields["media"] as String)
      : const Presence.absent();
  String get path => _fields["path"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "media", "path"].contains(entry.key))
            entry.key: entry.value
      });
}

sealed class Input extends _NativeValue {
  Input(super.fields);
  factory Input.read(Object? value) {
    final map = _readObject(value);
    if (map["kind"] == "text") return InputText.read(map);
    if (map["kind"] == "json") return InputJson.read(map);
    if (map["kind"] == "records") return InputRecords.read(map);
    if (map["kind"] == "units") return InputUnits.read(map);
    if (map["kind"] == "entities") return InputEntities.read(map);
    if (map["kind"] == "source") return InputSource.read(map);
    if (map["kind"] == "feed") return InputFeed.read(map);
    throw FormatException("Unknown Input alternative");
  }
}

final class InputEntities extends Input {
  InputEntities._(super.fields);
  factory InputEntities.read(Object? value) =>
      InputEntities._(_readObject(value));
  InputEntities({required List<Item> items})
      : super({"items": items, "kind": "entities"});
  List<Item> get items => (_fields["items"] as List)
      .map((v) => Item.read(v))
      .toList(growable: false);
  String get kind => _fields["kind"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["items", "kind"].contains(entry.key)) entry.key: entry.value
      });
}

final class InputFeed extends Input {
  InputFeed._(super.fields);
  factory InputFeed.read(Object? value) => InputFeed._(_readObject(value));
  InputFeed(
      {Presence<String> framing = const Presence.absent(),
      Presence<List<ImageAttachment>> images = const Presence.absent(),
      required String name,
      Presence<Reader> reading = const Presence.absent()})
      : super({
          if (framing.isPresent) "framing": framing.value,
          if (images.isPresent) "images": images.value,
          "kind": "feed",
          "name": name,
          if (reading.isPresent) "reading": reading.value
        });
  Presence<String> get framing => _fields.containsKey("framing")
      ? Presence.present(_fields["framing"] as String)
      : const Presence.absent();
  Presence<List<ImageAttachment>> get images => _fields.containsKey("images")
      ? Presence.present((_fields["images"] as List)
          .map((v) => ImageAttachment.read(v))
          .toList(growable: false))
      : const Presence.absent();
  String get kind => _fields["kind"] as String;
  String get name => _fields["name"] as String;
  Presence<Reader> get reading => _fields.containsKey("reading")
      ? Presence.present(Reader.read(_fields["reading"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["framing", "images", "kind", "name", "reading"]
              .contains(entry.key))
            entry.key: entry.value
      });
}

final class InputJson extends Input {
  InputJson._(super.fields);
  factory InputJson.read(Object? value) => InputJson._(_readObject(value));
  InputJson(
      {Presence<List<ImageAttachment>> images = const Presence.absent(),
      required Object? value})
      : super({
          if (images.isPresent) "images": images.value,
          "kind": "json",
          "value": value
        });
  Presence<List<ImageAttachment>> get images => _fields.containsKey("images")
      ? Presence.present((_fields["images"] as List)
          .map((v) => ImageAttachment.read(v))
          .toList(growable: false))
      : const Presence.absent();
  String get kind => _fields["kind"] as String;
  Object? get value => _fields["value"];
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["images", "kind", "value"].contains(entry.key))
            entry.key: entry.value
      });
}

final class InputRecords extends Input {
  InputRecords._(super.fields);
  factory InputRecords.read(Object? value) =>
      InputRecords._(_readObject(value));
  InputRecords({required List<Item> items})
      : super({"items": items, "kind": "records"});
  List<Item> get items => (_fields["items"] as List)
      .map((v) => Item.read(v))
      .toList(growable: false);
  String get kind => _fields["kind"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["items", "kind"].contains(entry.key)) entry.key: entry.value
      });
}

final class InputSource extends Input {
  InputSource._(super.fields);
  factory InputSource.read(Object? value) => InputSource._(_readObject(value));
  InputSource({required Source source})
      : super({"kind": "source", "source": source});
  String get kind => _fields["kind"] as String;
  Source get source => Source.read(_fields["source"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "source"].contains(entry.key)) entry.key: entry.value
      });
}

final class InputText extends Input {
  InputText._(super.fields);
  factory InputText.read(Object? value) => InputText._(_readObject(value));
  InputText(
      {Presence<List<ImageAttachment>> images = const Presence.absent(),
      required String text})
      : super({
          if (images.isPresent) "images": images.value,
          "kind": "text",
          "text": text
        });
  Presence<List<ImageAttachment>> get images => _fields.containsKey("images")
      ? Presence.present((_fields["images"] as List)
          .map((v) => ImageAttachment.read(v))
          .toList(growable: false))
      : const Presence.absent();
  String get kind => _fields["kind"] as String;
  String get text => _fields["text"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["images", "kind", "text"].contains(entry.key))
            entry.key: entry.value
      });
}

final class InputUnits extends Input {
  InputUnits._(super.fields);
  factory InputUnits.read(Object? value) => InputUnits._(_readObject(value));
  InputUnits({required List<Item> items})
      : super({"items": items, "kind": "units"});
  List<Item> get items => (_fields["items"] as List)
      .map((v) => Item.read(v))
      .toList(growable: false);
  String get kind => _fields["kind"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["items", "kind"].contains(entry.key)) entry.key: entry.value
      });
}

final class Item extends _NativeValue {
  Item._(super.fields);
  factory Item.read(Object? value) => Item._(_readObject(value));
  Item(
      {Presence<ContextSchema> context = const Presence.absent(),
      Presence<List<RecognitionExample>> examples = const Presence.absent(),
      Presence<List<ImageAttachment>> images = const Presence.absent(),
      Presence<List<OptionSchema>> options = const Presence.absent(),
      Presence<Original> original = const Presence.absent(),
      Presence<List<RecognitionSeedSpan>> seedSpans = const Presence.absent()})
      : super({
          if (context.isPresent) "context": context.value,
          if (examples.isPresent) "examples": examples.value,
          if (images.isPresent) "images": images.value,
          if (options.isPresent) "options": options.value,
          if (original.isPresent) "original": original.value,
          if (seedSpans.isPresent) "seed_spans": seedSpans.value
        });
  Presence<ContextSchema> get context => _fields.containsKey("context")
      ? Presence.present(ContextSchema.read(_fields["context"]))
      : const Presence.absent();
  Presence<List<RecognitionExample>> get examples => _fields
          .containsKey("examples")
      ? Presence.present((_fields["examples"] as List)
          .map((v) => RecognitionExample.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<List<ImageAttachment>> get images => _fields.containsKey("images")
      ? Presence.present((_fields["images"] as List)
          .map((v) => ImageAttachment.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<List<OptionSchema>> get options => _fields.containsKey("options")
      ? Presence.present((_fields["options"] as List)
          .map((v) => OptionSchema.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<Original> get original => _fields.containsKey("original")
      ? Presence.present(Original.read(_fields["original"]))
      : const Presence.absent();
  Presence<List<RecognitionSeedSpan>> get seedSpans =>
      _fields.containsKey("seed_spans")
          ? Presence.present((_fields["seed_spans"] as List)
              .map((v) => RecognitionSeedSpan.read(v))
              .toList(growable: false))
          : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "context",
            "examples",
            "images",
            "options",
            "original",
            "seed_spans"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

final class Options extends _NativeValue {
  Options._(super.fields);
  factory Options.read(Object? value) => Options._(_readObject(value));
  Options(
      {Presence<bool> attempts = const Presence.absent(),
      Presence<BatchOptions> batch = const Presence.absent(),
      Presence<String> context = const Presence.absent(),
      Presence<String> contextField = const Presence.absent(),
      Presence<BigInt> deadlineMs = const Presence.absent(),
      Presence<bool> details = const Presence.absent(),
      Presence<List<RecognitionExample>> examples = const Presence.absent(),
      Presence<String> examplesField = const Presence.absent(),
      Presence<List<String>> field = const Presence.absent(),
      Presence<bool> filesOnly = const Presence.absent(),
      Presence<BigInt> maxRequestsTotal = const Presence.absent(),
      Presence<String> mode = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<bool> none = const Presence.absent(),
      Presence<String> optionsField = const Presence.absent(),
      Presence<Threshold> relationThreshold = const Presence.absent(),
      Presence<List<RecognitionSeedSpan>> seedSpans = const Presence.absent(),
      Presence<String> seedSpansField = const Presence.absent(),
      Presence<BigInt> snippetPieces = const Presence.absent(),
      Presence<RecognitionContext> stageContext = const Presence.absent(),
      Presence<Threshold> threshold = const Presence.absent(),
      Presence<BigInt> top = const Presence.absent()})
      : super({
          if (attempts.isPresent) "attempts": attempts.value,
          if (batch.isPresent) "batch": batch.value,
          if (context.isPresent) "context": context.value,
          if (contextField.isPresent) "context_field": contextField.value,
          if (deadlineMs.isPresent) "deadline_ms": deadlineMs.value,
          if (details.isPresent) "details": details.value,
          if (examples.isPresent) "examples": examples.value,
          if (examplesField.isPresent) "examples_field": examplesField.value,
          if (field.isPresent) "field": field.value,
          if (filesOnly.isPresent) "files_only": filesOnly.value,
          if (maxRequestsTotal.isPresent)
            "max_requests_total": maxRequestsTotal.value,
          if (mode.isPresent) "mode": mode.value,
          if (model.isPresent) "model": model.value,
          if (none.isPresent) "none": none.value,
          if (optionsField.isPresent) "options_field": optionsField.value,
          if (relationThreshold.isPresent)
            "relation_threshold": relationThreshold.value,
          if (seedSpans.isPresent) "seed_spans": seedSpans.value,
          if (seedSpansField.isPresent)
            "seed_spans_field": seedSpansField.value,
          if (snippetPieces.isPresent) "snippet_pieces": snippetPieces.value,
          if (stageContext.isPresent) "stage_context": stageContext.value,
          if (threshold.isPresent) "threshold": threshold.value,
          if (top.isPresent) "top": top.value
        });
  Presence<bool> get attempts => _fields.containsKey("attempts")
      ? Presence.present(_fields["attempts"] as bool)
      : const Presence.absent();
  Presence<BatchOptions> get batch => _fields.containsKey("batch")
      ? Presence.present(BatchOptions.read(_fields["batch"]))
      : const Presence.absent();
  Presence<String> get context => _fields.containsKey("context")
      ? Presence.present(_fields["context"] as String)
      : const Presence.absent();
  Presence<String> get contextField => _fields.containsKey("context_field")
      ? Presence.present(_fields["context_field"] as String)
      : const Presence.absent();
  Presence<BigInt> get deadlineMs => _fields.containsKey("deadline_ms")
      ? Presence.present(_readInteger(_fields["deadline_ms"]))
      : const Presence.absent();
  Presence<bool> get details => _fields.containsKey("details")
      ? Presence.present(_fields["details"] as bool)
      : const Presence.absent();
  Presence<List<RecognitionExample>> get examples => _fields
          .containsKey("examples")
      ? Presence.present((_fields["examples"] as List)
          .map((v) => RecognitionExample.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<String> get examplesField => _fields.containsKey("examples_field")
      ? Presence.present(_fields["examples_field"] as String)
      : const Presence.absent();
  Presence<List<String>> get field => _fields.containsKey("field")
      ? Presence.present((_fields["field"] as List)
          .map((v) => v as String)
          .toList(growable: false))
      : const Presence.absent();
  Presence<bool> get filesOnly => _fields.containsKey("files_only")
      ? Presence.present(_fields["files_only"] as bool)
      : const Presence.absent();
  Presence<BigInt> get maxRequestsTotal =>
      _fields.containsKey("max_requests_total")
          ? Presence.present(_readInteger(_fields["max_requests_total"]))
          : const Presence.absent();
  Presence<String> get mode => _fields.containsKey("mode")
      ? Presence.present(_fields["mode"] as String)
      : const Presence.absent();
  Presence<String> get model => _fields.containsKey("model")
      ? Presence.present(_fields["model"] as String)
      : const Presence.absent();
  Presence<bool> get none => _fields.containsKey("none")
      ? Presence.present(_fields["none"] as bool)
      : const Presence.absent();
  Presence<String> get optionsField => _fields.containsKey("options_field")
      ? Presence.present(_fields["options_field"] as String)
      : const Presence.absent();
  Presence<Threshold> get relationThreshold =>
      _fields.containsKey("relation_threshold")
          ? Presence.present(Threshold.read(_fields["relation_threshold"]))
          : const Presence.absent();
  Presence<List<RecognitionSeedSpan>> get seedSpans =>
      _fields.containsKey("seed_spans")
          ? Presence.present((_fields["seed_spans"] as List)
              .map((v) => RecognitionSeedSpan.read(v))
              .toList(growable: false))
          : const Presence.absent();
  Presence<String> get seedSpansField => _fields.containsKey("seed_spans_field")
      ? Presence.present(_fields["seed_spans_field"] as String)
      : const Presence.absent();
  Presence<BigInt> get snippetPieces => _fields.containsKey("snippet_pieces")
      ? Presence.present(_readInteger(_fields["snippet_pieces"]))
      : const Presence.absent();
  Presence<RecognitionContext> get stageContext =>
      _fields.containsKey("stage_context")
          ? Presence.present(RecognitionContext.read(_fields["stage_context"]))
          : const Presence.absent();
  Presence<Threshold> get threshold => _fields.containsKey("threshold")
      ? Presence.present(Threshold.read(_fields["threshold"]))
      : const Presence.absent();
  Presence<BigInt> get top => _fields.containsKey("top")
      ? Presence.present(_readInteger(_fields["top"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (![
            "attempts",
            "batch",
            "context",
            "context_field",
            "deadline_ms",
            "details",
            "examples",
            "examples_field",
            "field",
            "files_only",
            "max_requests_total",
            "mode",
            "model",
            "none",
            "options_field",
            "relation_threshold",
            "seed_spans",
            "seed_spans_field",
            "snippet_pieces",
            "stage_context",
            "threshold",
            "top"
          ].contains(entry.key))
            entry.key: entry.value
      });
}

sealed class Original extends _NativeValue {
  Original(super.fields);
  factory Original.read(Object? value) {
    final map = _readObject(value);
    if (map["kind"] == "text") return OriginalText.read(map);
    if (map["kind"] == "json") return OriginalJson.read(map);
    throw FormatException("Unknown Original alternative");
  }
}

final class OriginalJson extends Original {
  OriginalJson._(super.fields);
  factory OriginalJson.read(Object? value) =>
      OriginalJson._(_readObject(value));
  OriginalJson({required Object? value})
      : super({"kind": "json", "value": value});
  String get kind => _fields["kind"] as String;
  Object? get value => _fields["value"];
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "value"].contains(entry.key)) entry.key: entry.value
      });
}

final class OriginalText extends Original {
  OriginalText._(super.fields);
  factory OriginalText.read(Object? value) =>
      OriginalText._(_readObject(value));
  OriginalText({required String text}) : super({"kind": "text", "text": text});
  String get kind => _fields["kind"] as String;
  String get text => _fields["text"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "text"].contains(entry.key)) entry.key: entry.value
      });
}

sealed class Question extends _NativeValue {
  Question(super.fields);
  factory Question.read(Object? value) {
    final map = _readObject(value);
    if (map["kind"] == "text") return QuestionText.read(map);
    if (map["kind"] == "definition") return DefinedQuestion.read(map);
    if (map["kind"] == "file") return QuestionFile.read(map);
    if (map["kind"] == "name") return QuestionName.read(map);
    if (map["kind"] == "reference") return QuestionReference.read(map);
    throw FormatException("Unknown Question alternative");
  }
}

final class DefinedQuestion extends Question {
  DefinedQuestion._(super.fields);
  factory DefinedQuestion.read(Object? value) =>
      DefinedQuestion._(_readObject(value));
  DefinedQuestion({required Definition value})
      : super({"kind": "definition", "value": value});
  String get kind => _fields["kind"] as String;
  Definition get value => Definition.read(_fields["value"]);
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "value"].contains(entry.key)) entry.key: entry.value
      });
}

final class QuestionFile extends Question {
  QuestionFile._(super.fields);
  factory QuestionFile.read(Object? value) =>
      QuestionFile._(_readObject(value));
  QuestionFile({required String path}) : super({"kind": "file", "path": path});
  String get kind => _fields["kind"] as String;
  String get path => _fields["path"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "path"].contains(entry.key)) entry.key: entry.value
      });
}

final class QuestionName extends Question {
  QuestionName._(super.fields);
  factory QuestionName.read(Object? value) =>
      QuestionName._(_readObject(value));
  QuestionName({required String name}) : super({"kind": "name", "name": name});
  String get kind => _fields["kind"] as String;
  String get name => _fields["name"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "name"].contains(entry.key)) entry.key: entry.value
      });
}

final class QuestionReference extends Question {
  QuestionReference._(super.fields);
  factory QuestionReference.read(Object? value) =>
      QuestionReference._(_readObject(value));
  QuestionReference({required String reference})
      : super({"kind": "reference", "reference": reference});
  String get kind => _fields["kind"] as String;
  String get reference => _fields["reference"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "reference"].contains(entry.key)) entry.key: entry.value
      });
}

final class QuestionText extends Question {
  QuestionText._(super.fields);
  factory QuestionText.read(Object? value) =>
      QuestionText._(_readObject(value));
  QuestionText({required String text}) : super({"kind": "text", "text": text});
  String get kind => _fields["kind"] as String;
  String get text => _fields["text"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "text"].contains(entry.key)) entry.key: entry.value
      });
}

final class Reader extends _NativeValue {
  Reader._(super.fields);
  factory Reader.read(Object? value) => Reader._(_readObject(value));
  Reader(
      {Presence<String> unit = const Presence.absent(),
      Presence<BigInt> window = const Presence.absent()})
      : super({
          if (unit.isPresent) "unit": unit.value,
          if (window.isPresent) "window": window.value
        });
  Presence<String> get unit => _fields.containsKey("unit")
      ? Presence.present(_fields["unit"] as String)
      : const Presence.absent();
  Presence<BigInt> get window => _fields.containsKey("window")
      ? Presence.present(_readInteger(_fields["window"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["unit", "window"].contains(entry.key)) entry.key: entry.value
      });
}

sealed class ReaderFailure extends _NativeValue {
  ReaderFailure(super.fields);
  factory ReaderFailure.read(Object? value) {
    final map = _readObject(value);
    if (map["kind"] == "producer") return ReaderFailureProducer.read(map);
    if (map["kind"] == "io") return ReaderFailureIo.read(map);
    if (map["kind"] == "utf8") return ReaderFailureUtf8.read(map);
    if (map["kind"] == "invalid_input")
      return ReaderFailureInvalidInput.read(map);
    throw FormatException("Unknown ReaderFailure alternative");
  }
}

final class ReaderFailureInvalidInput extends ReaderFailure {
  ReaderFailureInvalidInput._(super.fields);
  factory ReaderFailureInvalidInput.read(Object? value) =>
      ReaderFailureInvalidInput._(_readObject(value));
  ReaderFailureInvalidInput(
      {Presence<SessionSourceLocation> location = const Presence.absent()})
      : super({
          "kind": "invalid_input",
          if (location.isPresent) "location": location.value
        });
  String get kind => _fields["kind"] as String;
  Presence<SessionSourceLocation> get location =>
      _fields.containsKey("location")
          ? Presence.present(SessionSourceLocation.read(_fields["location"]))
          : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "location"].contains(entry.key)) entry.key: entry.value
      });
}

final class ReaderFailureIo extends ReaderFailure {
  ReaderFailureIo._(super.fields);
  factory ReaderFailureIo.read(Object? value) =>
      ReaderFailureIo._(_readObject(value));
  ReaderFailureIo(
      {Presence<SessionSourceLocation> location = const Presence.absent()})
      : super(
            {"kind": "io", if (location.isPresent) "location": location.value});
  String get kind => _fields["kind"] as String;
  Presence<SessionSourceLocation> get location =>
      _fields.containsKey("location")
          ? Presence.present(SessionSourceLocation.read(_fields["location"]))
          : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "location"].contains(entry.key)) entry.key: entry.value
      });
}

final class ReaderFailureProducer extends ReaderFailure {
  ReaderFailureProducer._(super.fields);
  factory ReaderFailureProducer.read(Object? value) =>
      ReaderFailureProducer._(_readObject(value));
  ReaderFailureProducer() : super({"kind": "producer"});
  String get kind => _fields["kind"] as String;
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind"].contains(entry.key)) entry.key: entry.value
      });
}

final class ReaderFailureUtf8 extends ReaderFailure {
  ReaderFailureUtf8._(super.fields);
  factory ReaderFailureUtf8.read(Object? value) =>
      ReaderFailureUtf8._(_readObject(value));
  ReaderFailureUtf8(
      {Presence<SessionSourceLocation> location = const Presence.absent()})
      : super({
          "kind": "utf8",
          if (location.isPresent) "location": location.value
        });
  String get kind => _fields["kind"] as String;
  Presence<SessionSourceLocation> get location =>
      _fields.containsKey("location")
          ? Presence.present(SessionSourceLocation.read(_fields["location"]))
          : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["kind", "location"].contains(entry.key)) entry.key: entry.value
      });
}

final class FeedItem extends _NativeValue {
  FeedItem._(super.fields);
  factory FeedItem.read(Object? value) => FeedItem._(_readObject(value));
  FeedItem(
      {required Item item,
      Presence<SessionSourceLocation> location = const Presence.absent()})
      : super(
            {"item": item, if (location.isPresent) "location": location.value});
  Item get item => Item.read(_fields["item"]);
  Presence<SessionSourceLocation> get location =>
      _fields.containsKey("location")
          ? Presence.present(SessionSourceLocation.read(_fields["location"]))
          : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["item", "location"].contains(entry.key)) entry.key: entry.value
      });
}

final class Source extends _NativeValue {
  Source._(super.fields);
  factory Source.read(Object? value) => Source._(_readObject(value));
  Source(
      {Presence<String> framing = const Presence.absent(),
      Presence<String> media = const Presence.absent(),
      required List<String> paths,
      Presence<Reader> reading = const Presence.absent()})
      : super({
          if (framing.isPresent) "framing": framing.value,
          if (media.isPresent) "media": media.value,
          "paths": paths,
          if (reading.isPresent) "reading": reading.value
        });
  Presence<String> get framing => _fields.containsKey("framing")
      ? Presence.present(_fields["framing"] as String)
      : const Presence.absent();
  Presence<String> get media => _fields.containsKey("media")
      ? Presence.present(_fields["media"] as String)
      : const Presence.absent();
  List<String> get paths => (_fields["paths"] as List)
      .map((v) => v as String)
      .toList(growable: false);
  Presence<Reader> get reading => _fields.containsKey("reading")
      ? Presence.present(Reader.read(_fields["reading"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["framing", "media", "paths", "reading"].contains(entry.key))
            entry.key: entry.value
      });
}

final class Threshold implements _EncodedValue {
  final Object? value;
  const Threshold._(this.value);
  factory Threshold.read(Object? value) =>
      value is Threshold ? value : Threshold._(value);
  Object? _toValue() => value;
  const Threshold.number(num this.value);
  num get asNumber => value as num;
  const Threshold.text(String this.value);
  String get asText => value as String;
}

final class SessionSourceLocation extends _NativeValue {
  SessionSourceLocation._(super.fields);
  factory SessionSourceLocation.read(Object? value) =>
      SessionSourceLocation._(_readObject(value));
  SessionSourceLocation(
      {required String file,
      Presence<BigInt> firstLine = const Presence.absent(),
      Presence<BigInt> lastLine = const Presence.absent()})
      : super({
          "file": file,
          if (firstLine.isPresent) "first_line": firstLine.value,
          if (lastLine.isPresent) "last_line": lastLine.value
        });
  String get file => _fields["file"] as String;
  Presence<BigInt> get firstLine => _fields.containsKey("first_line")
      ? Presence.present(_readInteger(_fields["first_line"]))
      : const Presence.absent();
  Presence<BigInt> get lastLine => _fields.containsKey("last_line")
      ? Presence.present(_readInteger(_fields["last_line"]))
      : const Presence.absent();
  Map<String, Object?> get extensions => Map.unmodifiable({
        for (final entry in _fields.entries)
          if (!["file", "first_line", "last_line"].contains(entry.key))
            entry.key: entry.value
      });
}

const requestVersion = "thinkthen.request/1";
