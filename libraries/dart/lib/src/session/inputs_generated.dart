// Generated from the shared Rust graph. Do not edit.
import 'values.dart';

final class InputAuthoredChoose extends NativeObject {
  InputAuthoredChoose._(super.json);
  factory InputAuthoredChoose.read(Object? value) =>
      InputAuthoredChoose._(readObject(value));
  InputAuthoredChoose(
      {Presence<Object?> batch = const Presence.absent(),
      required InputAuthoredQuestionText choose,
      Presence<InputAuthoredInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> itemSchema =
          const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<InputAuthoredPointers> on = const Presence.absent(),
      Presence<InputAuthoredOptions> options = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      Presence<InputAuthoredCut> threshold = const Presence.absent(),
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
  Presence<Object?> get batch => json.containsKey("batch")
      ? Presence.present(json["batch"])
      : const Presence.absent();
  InputAuthoredQuestionText get choose =>
      InputAuthoredQuestionText.read(json["choose"]);
  Presence<InputAuthoredInputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get itemSchema =>
      json.containsKey("item_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["item_schema"]))
          : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<InputAuthoredPointers> get on => json.containsKey("on")
      ? Presence.present(InputAuthoredPointers.read(json["on"]))
      : const Presence.absent();
  Presence<InputAuthoredOptions> get options => json.containsKey("options")
      ? Presence.present(InputAuthoredOptions.read(json["options"]))
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  Presence<InputAuthoredCut> get threshold => json.containsKey("threshold")
      ? Presence.present(InputAuthoredCut.read(json["threshold"]))
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

final class InputAuthoredCriterion {
  final Object? value;
  const InputAuthoredCriterion._(this.value);
  factory InputAuthoredCriterion.read(Object? value) =>
      value is InputAuthoredCriterion ? value : InputAuthoredCriterion._(value);
  Object? toJson() => value;
  const InputAuthoredCriterion.alternative0(String this.value);
  String get asAlternative0 => value as String;
  const InputAuthoredCriterion.alternative1(Map<String, Object?> this.value);
  Map<String, Object?> get asAlternative1 =>
      (value as Map<String, Object?>).map((k, v) => MapEntry(k, v));
  const InputAuthoredCriterion.alternative2(List<Object?> this.value);
  List<Object?> get asAlternative2 =>
      (value as List).map((v) => v).toList(growable: false);
  const InputAuthoredCriterion.nullValue() : value = null;
}

final class InputAuthoredCut {
  final Object? value;
  const InputAuthoredCut._(this.value);
  factory InputAuthoredCut.read(Object? value) =>
      value is InputAuthoredCut ? value : InputAuthoredCut._(value);
  Object? toJson() => value;
  const InputAuthoredCut.alternative0(num this.value);
  num get asAlternative0 => value as num;
  const InputAuthoredCut.alternative1(String this.value);
  String get asAlternative1 => value as String;
}

final class InputAuthoredDecide extends NativeObject {
  InputAuthoredDecide._(super.json);
  factory InputAuthoredDecide.read(Object? value) =>
      InputAuthoredDecide._(readObject(value));
  InputAuthoredDecide(
      {Presence<Object?> batch = const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> contextSchema =
          const Presence.absent(),
      required InputAuthoredQuestionText decide,
      Presence<InputAuthoredCriterion> falseValue = const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> itemSchema =
          const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<InputAuthoredPointers> on = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      Presence<InputAuthoredThreshold> threshold = const Presence.absent(),
      Presence<InputAuthoredCriterion> trueValue = const Presence.absent(),
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
  Presence<Object?> get batch => json.containsKey("batch")
      ? Presence.present(json["batch"])
      : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  InputAuthoredQuestionText get decide =>
      InputAuthoredQuestionText.read(json["decide"]);
  Presence<InputAuthoredCriterion> get falseValue => json.containsKey("false")
      ? Presence.present(InputAuthoredCriterion.read(json["false"]))
      : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get itemSchema =>
      json.containsKey("item_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["item_schema"]))
          : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<InputAuthoredPointers> get on => json.containsKey("on")
      ? Presence.present(InputAuthoredPointers.read(json["on"]))
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  Presence<InputAuthoredThreshold> get threshold =>
      json.containsKey("threshold")
          ? Presence.present(InputAuthoredThreshold.read(json["threshold"]))
          : const Presence.absent();
  Presence<InputAuthoredCriterion> get trueValue => json.containsKey("true")
      ? Presence.present(InputAuthoredCriterion.read(json["true"]))
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

final class InputAuthoredFind extends NativeObject {
  InputAuthoredFind._(super.json);
  factory InputAuthoredFind.read(Object? value) =>
      InputAuthoredFind._(readObject(value));
  InputAuthoredFind(
      {Presence<InputAuthoredInputDeclaration> contextSchema =
          const Presence.absent(),
      required InputAuthoredQuestionText find,
      Presence<InputAuthoredInputDeclaration> itemSchema =
          const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<InputAuthoredPointers> on = const Presence.absent(),
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
  Presence<InputAuthoredInputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  InputAuthoredQuestionText get find =>
      InputAuthoredQuestionText.read(json["find"]);
  Presence<InputAuthoredInputDeclaration> get itemSchema =>
      json.containsKey("item_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["item_schema"]))
          : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<InputAuthoredPointers> get on => json.containsKey("on")
      ? Presence.present(InputAuthoredPointers.read(json["on"]))
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

sealed class InputAuthoredInputDeclaration extends NativeObject {
  InputAuthoredInputDeclaration(super.json);
  factory InputAuthoredInputDeclaration.read(Object? value) {
    final map = readObject(value);
    if (map["type"] == "string")
      return InputAuthoredInputDeclarationString.read(map);
    if (map["type"] == "object")
      return InputAuthoredInputDeclarationObject.read(map);
    throw FormatException("Unknown InputAuthoredInputDeclaration alternative");
  }
}

final class InputAuthoredInputDeclarationObject
    extends InputAuthoredInputDeclaration {
  InputAuthoredInputDeclarationObject._(super.json);
  factory InputAuthoredInputDeclarationObject.read(Object? value) =>
      InputAuthoredInputDeclarationObject._(readObject(value));
  InputAuthoredInputDeclarationObject(
      {required Map<String, InputAuthoredInputProperty> properties,
      Presence<List<String>> required = const Presence.absent()})
      : super({
          "properties": properties,
          if (required.isPresent) "required": required.value,
          "type": "object"
        });
  Map<String, InputAuthoredInputProperty> get properties =>
      (json["properties"] as Map<String, Object?>)
          .map((k, v) => MapEntry(k, InputAuthoredInputProperty.read(v)));
  Presence<List<String>> get required => json.containsKey("required")
      ? Presence.present((json["required"] as List)
          .map((v) => v as String)
          .toList(growable: false))
      : const Presence.absent();
  String get type => json["type"] as String;
}

final class InputAuthoredInputDeclarationString
    extends InputAuthoredInputDeclaration {
  InputAuthoredInputDeclarationString._(super.json);
  factory InputAuthoredInputDeclarationString.read(Object? value) =>
      InputAuthoredInputDeclarationString._(readObject(value));
  InputAuthoredInputDeclarationString() : super({"type": "string"});
  String get type => json["type"] as String;
}

sealed class InputAuthoredInputProperty extends NativeObject {
  InputAuthoredInputProperty(super.json);
  factory InputAuthoredInputProperty.read(Object? value) {
    final map = readObject(value);
    if (map["type"] == "string")
      return InputAuthoredInputPropertyString.read(map);
    if (map["type"] == "number")
      return InputAuthoredInputPropertyNumber.read(map);
    if (map["type"] == "boolean")
      return InputAuthoredInputPropertyBoolean.read(map);
    if (map["type"] == "array")
      return InputAuthoredInputPropertyArray.read(map);
    throw FormatException("Unknown InputAuthoredInputProperty alternative");
  }
}

final class InputAuthoredInputPropertyArray extends InputAuthoredInputProperty {
  InputAuthoredInputPropertyArray._(super.json);
  factory InputAuthoredInputPropertyArray.read(Object? value) =>
      InputAuthoredInputPropertyArray._(readObject(value));
  InputAuthoredInputPropertyArray({required Object? items})
      : super({"items": items, "type": "array"});
  Object? get items => json["items"];
  String get type => json["type"] as String;
}

final class InputAuthoredInputPropertyBoolean
    extends InputAuthoredInputProperty {
  InputAuthoredInputPropertyBoolean._(super.json);
  factory InputAuthoredInputPropertyBoolean.read(Object? value) =>
      InputAuthoredInputPropertyBoolean._(readObject(value));
  InputAuthoredInputPropertyBoolean() : super({"type": "boolean"});
  String get type => json["type"] as String;
}

final class InputAuthoredInputPropertyNumber
    extends InputAuthoredInputProperty {
  InputAuthoredInputPropertyNumber._(super.json);
  factory InputAuthoredInputPropertyNumber.read(Object? value) =>
      InputAuthoredInputPropertyNumber._(readObject(value));
  InputAuthoredInputPropertyNumber() : super({"type": "number"});
  String get type => json["type"] as String;
}

final class InputAuthoredInputPropertyString
    extends InputAuthoredInputProperty {
  InputAuthoredInputPropertyString._(super.json);
  factory InputAuthoredInputPropertyString.read(Object? value) =>
      InputAuthoredInputPropertyString._(readObject(value));
  InputAuthoredInputPropertyString() : super({"type": "string"});
  String get type => json["type"] as String;
}

final class InputAuthoredLabels {
  final Object? value;
  const InputAuthoredLabels._(this.value);
  factory InputAuthoredLabels.read(Object? value) =>
      value is InputAuthoredLabels ? value : InputAuthoredLabels._(value);
  Object? toJson() => value;
  const InputAuthoredLabels.alternative0(List<String> this.value);
  List<String> get asAlternative0 =>
      (value as List).map((v) => v as String).toList(growable: false);
  const InputAuthoredLabels.alternative1(Map<String, Object?> this.value);
  Map<String, Object?> get asAlternative1 =>
      (value as Map<String, Object?>).map((k, v) => MapEntry(k, v));
}

final class InputAuthoredLevels {
  final Object? value;
  const InputAuthoredLevels._(this.value);
  factory InputAuthoredLevels.read(Object? value) =>
      value is InputAuthoredLevels ? value : InputAuthoredLevels._(value);
  Object? toJson() => value;
  const InputAuthoredLevels.alternative0(List<String> this.value);
  List<String> get asAlternative0 =>
      (value as List).map((v) => v as String).toList(growable: false);
  const InputAuthoredLevels.alternative1(
      Map<String, InputAuthoredCriterion> this.value);
  Map<String, InputAuthoredCriterion> get asAlternative1 =>
      (value as Map<String, Object?>)
          .map((k, v) => MapEntry(k, InputAuthoredCriterion.read(v)));
}

final class InputAuthoredOptions {
  final Object? value;
  const InputAuthoredOptions._(this.value);
  factory InputAuthoredOptions.read(Object? value) =>
      value is InputAuthoredOptions ? value : InputAuthoredOptions._(value);
  Object? toJson() => value;
  const InputAuthoredOptions.alternative0(List<String> this.value);
  List<String> get asAlternative0 =>
      (value as List).map((v) => v as String).toList(growable: false);
  const InputAuthoredOptions.alternative1(Map<String, Object?> this.value);
  Map<String, Object?> get asAlternative1 =>
      (value as Map<String, Object?>).map((k, v) => MapEntry(k, v));
}

final class InputAuthoredPointers {
  final Object? value;
  const InputAuthoredPointers._(this.value);
  factory InputAuthoredPointers.read(Object? value) =>
      value is InputAuthoredPointers ? value : InputAuthoredPointers._(value);
  Object? toJson() => value;
  const InputAuthoredPointers.alternative0(String this.value);
  String get asAlternative0 => value as String;
  const InputAuthoredPointers.alternative1(List<String> this.value);
  List<String> get asAlternative1 =>
      (value as List).map((v) => v as String).toList(growable: false);
}

final class InputAuthoredQuestionText {
  final Object? value;
  const InputAuthoredQuestionText._(this.value);
  factory InputAuthoredQuestionText.read(Object? value) =>
      value is InputAuthoredQuestionText
          ? value
          : InputAuthoredQuestionText._(value);
  Object? toJson() => value;
  const InputAuthoredQuestionText.alternative0(String this.value);
  String get asAlternative0 => value as String;
  const InputAuthoredQuestionText.alternative1(Map<String, Object?> this.value);
  Map<String, Object?> get asAlternative1 =>
      (value as Map<String, Object?>).map((k, v) => MapEntry(k, v));
  const InputAuthoredQuestionText.alternative2(List<Object?> this.value);
  List<Object?> get asAlternative2 =>
      (value as List).map((v) => v).toList(growable: false);
}

final class InputAuthoredRelate extends NativeObject {
  InputAuthoredRelate._(super.json);
  factory InputAuthoredRelate.read(Object? value) =>
      InputAuthoredRelate._(readObject(value));
  InputAuthoredRelate(
      {Presence<InputAuthoredInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> itemSchema =
          const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      required Object? relate,
      Presence<InputAuthoredCut> threshold = const Presence.absent(),
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
  Presence<InputAuthoredInputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get itemSchema =>
      json.containsKey("item_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["item_schema"]))
          : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  Object? get relate => json["relate"];
  Presence<InputAuthoredCut> get threshold => json.containsKey("threshold")
      ? Presence.present(InputAuthoredCut.read(json["threshold"]))
      : const Presence.absent();
  BigInt get version => readInteger(json["version"]);
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

final class InputAuthoredRelation extends NativeObject {
  InputAuthoredRelation._(super.json);
  factory InputAuthoredRelation.read(Object? value) =>
      InputAuthoredRelation._(readObject(value));
  InputAuthoredRelation(
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
  Presence<bool> get either => json.containsKey("either")
      ? Presence.present(json["either"] as bool)
      : const Presence.absent();
  String get name => json["name"] as String;
  Presence<String> get reads => json.containsKey("reads")
      ? Presence.present(json["reads"] as String)
      : const Presence.absent();
  Presence<bool> get single => json.containsKey("single")
      ? Presence.present(json["single"] as bool)
      : const Presence.absent();
  Presence<String> get source => json.containsKey("source")
      ? Presence.present(json["source"] as String)
      : const Presence.absent();
  Presence<String> get target => json.containsKey("target")
      ? Presence.present(json["target"] as String)
      : const Presence.absent();
}

final class InputAuthoredScore extends NativeObject {
  InputAuthoredScore._(super.json);
  factory InputAuthoredScore.read(Object? value) =>
      InputAuthoredScore._(readObject(value));
  InputAuthoredScore(
      {Presence<Object?> batch = const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> itemSchema =
          const Presence.absent(),
      Presence<InputAuthoredLevels> levels = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<InputAuthoredPointers> on = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      required InputAuthoredQuestionText score,
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
  Presence<Object?> get batch => json.containsKey("batch")
      ? Presence.present(json["batch"])
      : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get itemSchema =>
      json.containsKey("item_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["item_schema"]))
          : const Presence.absent();
  Presence<InputAuthoredLevels> get levels => json.containsKey("levels")
      ? Presence.present(InputAuthoredLevels.read(json["levels"]))
      : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<InputAuthoredPointers> get on => json.containsKey("on")
      ? Presence.present(InputAuthoredPointers.read(json["on"]))
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  InputAuthoredQuestionText get score =>
      InputAuthoredQuestionText.read(json["score"]);
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

final class InputAuthoredTag extends NativeObject {
  InputAuthoredTag._(super.json);
  factory InputAuthoredTag.read(Object? value) =>
      InputAuthoredTag._(readObject(value));
  InputAuthoredTag(
      {Presence<Object?> batch = const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> itemSchema =
          const Presence.absent(),
      Presence<InputAuthoredLabels> labels = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<InputAuthoredPointers> on = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      required InputAuthoredQuestionText tag,
      Presence<InputAuthoredCut> threshold = const Presence.absent(),
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
  Presence<Object?> get batch => json.containsKey("batch")
      ? Presence.present(json["batch"])
      : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get itemSchema =>
      json.containsKey("item_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["item_schema"]))
          : const Presence.absent();
  Presence<InputAuthoredLabels> get labels => json.containsKey("labels")
      ? Presence.present(InputAuthoredLabels.read(json["labels"]))
      : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<InputAuthoredPointers> get on => json.containsKey("on")
      ? Presence.present(InputAuthoredPointers.read(json["on"]))
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  InputAuthoredQuestionText get tag =>
      InputAuthoredQuestionText.read(json["tag"]);
  Presence<InputAuthoredCut> get threshold => json.containsKey("threshold")
      ? Presence.present(InputAuthoredCut.read(json["threshold"]))
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

final class InputAuthoredThreshold {
  final Object? value;
  const InputAuthoredThreshold._(this.value);
  factory InputAuthoredThreshold.read(Object? value) =>
      value is InputAuthoredThreshold ? value : InputAuthoredThreshold._(value);
  Object? toJson() => value;
  const InputAuthoredThreshold.alternative0(num this.value);
  num get asAlternative0 => value as num;
  const InputAuthoredThreshold.alternative1(String this.value);
  String get asAlternative1 => value as String;
}

final class InputCacheDocument {
  final Object? value;
  const InputCacheDocument._(this.value);
  factory InputCacheDocument.read(Object? value) =>
      value is InputCacheDocument ? value : InputCacheDocument._(value);
  Object? toJson() => value;
  const InputCacheDocument.alternative0(String this.value);
  String get asAlternative0 => value as String;
  const InputCacheDocument.alternative1(bool this.value);
  bool get asAlternative1 => value as bool;
}

final class InputContextSchema {
  final Object? value;
  const InputContextSchema._(this.value);
  factory InputContextSchema.read(Object? value) =>
      value is InputContextSchema ? value : InputContextSchema._(value);
  Object? toJson() => value;
  const InputContextSchema.alternative0(String this.value);
  String get asAlternative0 => value as String;
  const InputContextSchema.alternative1(Map<String, Object?> this.value);
  Map<String, Object?> get asAlternative1 =>
      (value as Map<String, Object?>).map((k, v) => MapEntry(k, v));
}

final class InputEngineSettings extends NativeObject {
  InputEngineSettings._(super.json);
  factory InputEngineSettings.read(Object? value) =>
      InputEngineSettings._(readObject(value));
  InputEngineSettings(
      {Presence<String> backend = const Presence.absent(),
      Presence<String> baseUrl = const Presence.absent(),
      Presence<InputRequestBatch> batch = const Presence.absent(),
      Presence<InputCacheDocument> cache = const Presence.absent(),
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
  Presence<String> get backend => json.containsKey("backend")
      ? Presence.present(json["backend"] as String)
      : const Presence.absent();
  Presence<String> get baseUrl => json.containsKey("base_url")
      ? Presence.present(json["base_url"] as String)
      : const Presence.absent();
  Presence<InputRequestBatch> get batch => json.containsKey("batch")
      ? Presence.present(InputRequestBatch.read(json["batch"]))
      : const Presence.absent();
  Presence<InputCacheDocument> get cache => json.containsKey("cache")
      ? Presence.present(InputCacheDocument.read(json["cache"]))
      : const Presence.absent();
  Presence<BigInt?> get maxEstimatedInputTokensTotal =>
      json.containsKey("max_estimated_input_tokens_total")
          ? Presence.present(json["max_estimated_input_tokens_total"] == null
              ? null
              : readInteger(json["max_estimated_input_tokens_total"]))
          : const Presence.absent();
  Presence<BigInt> get maxRequestBytes => json.containsKey("max_request_bytes")
      ? Presence.present(readInteger(json["max_request_bytes"]))
      : const Presence.absent();
  Presence<BigInt?> get maxRequests => json.containsKey("max_requests")
      ? Presence.present(json["max_requests"] == null
          ? null
          : readInteger(json["max_requests"]))
      : const Presence.absent();
  Presence<BigInt?> get maxRequestsTotal =>
      json.containsKey("max_requests_total")
          ? Presence.present(json["max_requests_total"] == null
              ? null
              : readInteger(json["max_requests_total"]))
          : const Presence.absent();
  Presence<BigInt> get maxRetries => json.containsKey("max_retries")
      ? Presence.present(readInteger(json["max_retries"]))
      : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  Presence<Object?> get proxy => json.containsKey("proxy")
      ? Presence.present(json["proxy"])
      : const Presence.absent();
  Presence<String> get record => json.containsKey("record")
      ? Presence.present(json["record"] as String)
      : const Presence.absent();
  Presence<bool> get refreshCache => json.containsKey("refresh_cache")
      ? Presence.present(json["refresh_cache"] as bool)
      : const Presence.absent();
  Presence<String> get replay => json.containsKey("replay")
      ? Presence.present(json["replay"] as String)
      : const Presence.absent();
  Presence<BigInt> get throttle => json.containsKey("throttle")
      ? Presence.present(readInteger(json["throttle"]))
      : const Presence.absent();
  Presence<BigInt> get timeout => json.containsKey("timeout")
      ? Presence.present(readInteger(json["timeout"]))
      : const Presence.absent();
  Presence<String> get usdPerMillionInput =>
      json.containsKey("usd_per_million_input")
          ? Presence.present(json["usd_per_million_input"] as String)
          : const Presence.absent();
  Presence<String> get usdPerMillionOutput =>
      json.containsKey("usd_per_million_output")
          ? Presence.present(json["usd_per_million_output"] as String)
          : const Presence.absent();
}

final class InputOptionSchema extends NativeObject {
  InputOptionSchema._(super.json);
  factory InputOptionSchema.read(Object? value) =>
      InputOptionSchema._(readObject(value));
  InputOptionSchema(
      {Presence<Object?> description = const Presence.absent(),
      required String name})
      : super({
          if (description.isPresent) "description": description.value,
          "name": name
        });
  Presence<Object?> get description => json.containsKey("description")
      ? Presence.present(json["description"])
      : const Presence.absent();
  String get name => json["name"] as String;
}

final class InputRecognitionExample {
  final Object? value;
  const InputRecognitionExample._(this.value);
  factory InputRecognitionExample.read(Object? value) =>
      value is InputRecognitionExample
          ? value
          : InputRecognitionExample._(value);
  Object? toJson() => value;
  const InputRecognitionExample.alternative0(String this.value);
  String get asAlternative0 => value as String;
  const InputRecognitionExample.alternative1(
      InputRecognitionExampleText this.value);
  InputRecognitionExampleText get asAlternative1 =>
      InputRecognitionExampleText.read(value);
}

final class InputRecognitionExampleEntity extends NativeObject {
  InputRecognitionExampleEntity._(super.json);
  factory InputRecognitionExampleEntity.read(Object? value) =>
      InputRecognitionExampleEntity._(readObject(value));
  InputRecognitionExampleEntity(
      {required BigInt end, required String kind, required BigInt start})
      : super({"end": end, "kind": kind, "start": start});
  BigInt get end => readInteger(json["end"]);
  String get kind => json["kind"] as String;
  BigInt get start => readInteger(json["start"]);
}

final class InputRecognitionExampleText extends NativeObject {
  InputRecognitionExampleText._(super.json);
  factory InputRecognitionExampleText.read(Object? value) =>
      InputRecognitionExampleText._(readObject(value));
  InputRecognitionExampleText(
      {required List<InputRecognitionExampleEntity> entities,
      Presence<List<String>> kinds = const Presence.absent(),
      required String text})
      : super({
          "entities": entities,
          if (kinds.isPresent) "kinds": kinds.value,
          "text": text
        });
  List<InputRecognitionExampleEntity> get entities => (json["entities"] as List)
      .map((v) => InputRecognitionExampleEntity.read(v))
      .toList(growable: false);
  Presence<List<String>> get kinds => json.containsKey("kinds")
      ? Presence.present((json["kinds"] as List)
          .map((v) => v as String)
          .toList(growable: false))
      : const Presence.absent();
  String get text => json["text"] as String;
}

final class InputRecognitionSeedSpan extends NativeObject {
  InputRecognitionSeedSpan._(super.json);
  factory InputRecognitionSeedSpan.read(Object? value) =>
      InputRecognitionSeedSpan._(readObject(value));
  InputRecognitionSeedSpan(
      {required BigInt end,
      Presence<String> kind = const Presence.absent(),
      required BigInt start})
      : super({
          "end": end,
          if (kind.isPresent) "kind": kind.value,
          "start": start
        });
  BigInt get end => readInteger(json["end"]);
  Presence<String> get kind => json.containsKey("kind")
      ? Presence.present(json["kind"] as String)
      : const Presence.absent();
  BigInt get start => readInteger(json["start"]);
}

final class InputRecognitionStageContext extends NativeObject {
  InputRecognitionStageContext._(super.json);
  factory InputRecognitionStageContext.read(Object? value) =>
      InputRecognitionStageContext._(readObject(value));
  InputRecognitionStageContext(
      {Presence<String> boundary = const Presence.absent(),
      Presence<String> kindEdge = const Presence.absent(),
      Presence<String> relation = const Presence.absent()})
      : super({
          if (boundary.isPresent) "boundary": boundary.value,
          if (kindEdge.isPresent) "kind_edge": kindEdge.value,
          if (relation.isPresent) "relation": relation.value
        });
  Presence<String> get boundary => json.containsKey("boundary")
      ? Presence.present(json["boundary"] as String)
      : const Presence.absent();
  Presence<String> get kindEdge => json.containsKey("kind_edge")
      ? Presence.present(json["kind_edge"] as String)
      : const Presence.absent();
  Presence<String> get relation => json.containsKey("relation")
      ? Presence.present(json["relation"] as String)
      : const Presence.absent();
}

final class InputRequestBatch {
  final Object? value;
  const InputRequestBatch._(this.value);
  factory InputRequestBatch.read(Object? value) =>
      value is InputRequestBatch ? value : InputRequestBatch._(value);
  Object? toJson() => value;
  const InputRequestBatch.alternative0(BigInt this.value);
  BigInt get asAlternative0 => readInteger(value);
  const InputRequestBatch.alternative1(String this.value);
  String get asAlternative1 => value as String;
}

sealed class InputRequestDefinition extends NativeObject {
  InputRequestDefinition(super.json);
  factory InputRequestDefinition.read(Object? value) {
    final map = readObject(value);
    if (map.containsKey("decide") &&
        !map.containsKey("choose") &&
        !map.containsKey("find") &&
        !map.containsKey("questions") &&
        !map.containsKey("recognize") &&
        !map.containsKey("relate") &&
        !map.containsKey("score") &&
        !map.containsKey("tag") &&
        !map.containsKey("version"))
      return InputRequestDefinitionFieldsDecide.read(map);
    if (map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("find") &&
        !map.containsKey("questions") &&
        !map.containsKey("recognize") &&
        !map.containsKey("relate") &&
        !map.containsKey("score") &&
        !map.containsKey("tag") &&
        !map.containsKey("version"))
      return InputRequestDefinitionFieldsChoose.read(map);
    if (map.containsKey("tag") &&
        !map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("find") &&
        !map.containsKey("questions") &&
        !map.containsKey("recognize") &&
        !map.containsKey("relate") &&
        !map.containsKey("score") &&
        !map.containsKey("version"))
      return InputRequestDefinitionFieldsTag.read(map);
    if (map.containsKey("score") &&
        !map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("find") &&
        !map.containsKey("questions") &&
        !map.containsKey("recognize") &&
        !map.containsKey("relate") &&
        !map.containsKey("tag") &&
        !map.containsKey("version"))
      return InputRequestDefinitionFieldsScore.read(map);
    if (map.containsKey("relate") &&
        map.containsKey("version") &&
        !map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("find") &&
        !map.containsKey("questions") &&
        !map.containsKey("recognize") &&
        !map.containsKey("score") &&
        !map.containsKey("tag"))
      return InputRequestDefinitionFieldsRelateVersion.read(map);
    if (map.containsKey("find") &&
        !map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("questions") &&
        !map.containsKey("recognize") &&
        !map.containsKey("relate") &&
        !map.containsKey("score") &&
        !map.containsKey("tag") &&
        !map.containsKey("version"))
      return InputRequestDefinitionFieldsFind.read(map);
    if (map.containsKey("recognize") &&
        map.containsKey("version") &&
        !map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("find") &&
        !map.containsKey("questions") &&
        !map.containsKey("relate") &&
        !map.containsKey("score") &&
        !map.containsKey("tag"))
      return InputRequestDefinitionFieldsRecognizeVersion.read(map);
    if (map.containsKey("questions") &&
        map.containsKey("version") &&
        !map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("find") &&
        !map.containsKey("recognize") &&
        !map.containsKey("relate") &&
        !map.containsKey("score") &&
        !map.containsKey("tag"))
      return InputRequestDefinitionFieldsQuestionsVersion.read(map);
    throw FormatException("Unknown InputRequestDefinition alternative");
  }
}

sealed class InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties
    extends NativeObject {
  InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties(
      super.json);
  factory InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties.read(
      Object? value) {
    final map = readObject(value);
    if (map.containsKey("decide") &&
        !map.containsKey("choose") &&
        !map.containsKey("score") &&
        !map.containsKey("tag"))
      return InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide
          .read(map);
    if (map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("score") &&
        !map.containsKey("tag"))
      return InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose
          .read(map);
    if (map.containsKey("tag") &&
        !map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("score"))
      return InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag
          .read(map);
    if (map.containsKey("score") &&
        !map.containsKey("choose") &&
        !map.containsKey("decide") &&
        !map.containsKey("tag"))
      return InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore
          .read(map);
    throw FormatException(
        "Unknown InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties alternative");
  }
}

final class InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose
    extends InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties {
  InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose._(
      super.json);
  factory InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose.read(
          Object? value) =>
      InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose
          ._(readObject(value));
  InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsChoose(
      {required InputAuthoredQuestionText choose,
      Presence<InputAuthoredInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> itemSchema =
          const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<InputAuthoredPointers> on = const Presence.absent(),
      Presence<InputAuthoredOptions> options = const Presence.absent(),
      Presence<InputAuthoredCut> threshold = const Presence.absent(),
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
  InputAuthoredQuestionText get choose =>
      InputAuthoredQuestionText.read(json["choose"]);
  Presence<InputAuthoredInputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get itemSchema =>
      json.containsKey("item_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["item_schema"]))
          : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<InputAuthoredPointers> get on => json.containsKey("on")
      ? Presence.present(InputAuthoredPointers.read(json["on"]))
      : const Presence.absent();
  Presence<InputAuthoredOptions> get options => json.containsKey("options")
      ? Presence.present(InputAuthoredOptions.read(json["options"]))
      : const Presence.absent();
  Presence<InputAuthoredCut> get threshold => json.containsKey("threshold")
      ? Presence.present(InputAuthoredCut.read(json["threshold"]))
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

final class InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide
    extends InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties {
  InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide._(
      super.json);
  factory InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide.read(
          Object? value) =>
      InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide
          ._(readObject(value));
  InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide(
      {Presence<InputAuthoredInputDeclaration> contextSchema =
          const Presence.absent(),
      required InputAuthoredQuestionText decide,
      Presence<InputAuthoredCriterion> falseValue = const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> itemSchema =
          const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<InputAuthoredPointers> on = const Presence.absent(),
      Presence<InputAuthoredThreshold> threshold = const Presence.absent(),
      Presence<InputAuthoredCriterion> trueValue = const Presence.absent(),
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
  Presence<InputAuthoredInputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  InputAuthoredQuestionText get decide =>
      InputAuthoredQuestionText.read(json["decide"]);
  Presence<InputAuthoredCriterion> get falseValue => json.containsKey("false")
      ? Presence.present(InputAuthoredCriterion.read(json["false"]))
      : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get itemSchema =>
      json.containsKey("item_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["item_schema"]))
          : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<InputAuthoredPointers> get on => json.containsKey("on")
      ? Presence.present(InputAuthoredPointers.read(json["on"]))
      : const Presence.absent();
  Presence<InputAuthoredThreshold> get threshold =>
      json.containsKey("threshold")
          ? Presence.present(InputAuthoredThreshold.read(json["threshold"]))
          : const Presence.absent();
  Presence<InputAuthoredCriterion> get trueValue => json.containsKey("true")
      ? Presence.present(InputAuthoredCriterion.read(json["true"]))
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

final class InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore
    extends InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties {
  InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore._(
      super.json);
  factory InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore.read(
          Object? value) =>
      InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore
          ._(readObject(value));
  InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsScore(
      {Presence<InputAuthoredInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> itemSchema =
          const Presence.absent(),
      Presence<InputAuthoredLevels> levels = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<InputAuthoredPointers> on = const Presence.absent(),
      required InputAuthoredQuestionText score,
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
  Presence<InputAuthoredInputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get itemSchema =>
      json.containsKey("item_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["item_schema"]))
          : const Presence.absent();
  Presence<InputAuthoredLevels> get levels => json.containsKey("levels")
      ? Presence.present(InputAuthoredLevels.read(json["levels"]))
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<InputAuthoredPointers> get on => json.containsKey("on")
      ? Presence.present(InputAuthoredPointers.read(json["on"]))
      : const Presence.absent();
  InputAuthoredQuestionText get score =>
      InputAuthoredQuestionText.read(json["score"]);
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

final class InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag
    extends InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties {
  InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag._(
      super.json);
  factory InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag.read(
          Object? value) =>
      InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag
          ._(readObject(value));
  InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsTag(
      {Presence<InputAuthoredInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> itemSchema =
          const Presence.absent(),
      Presence<InputAuthoredLabels> labels = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<InputAuthoredPointers> on = const Presence.absent(),
      required InputAuthoredQuestionText tag,
      Presence<InputAuthoredCut> threshold = const Presence.absent(),
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
  Presence<InputAuthoredInputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get itemSchema =>
      json.containsKey("item_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["item_schema"]))
          : const Presence.absent();
  Presence<InputAuthoredLabels> get labels => json.containsKey("labels")
      ? Presence.present(InputAuthoredLabels.read(json["labels"]))
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<InputAuthoredPointers> get on => json.containsKey("on")
      ? Presence.present(InputAuthoredPointers.read(json["on"]))
      : const Presence.absent();
  InputAuthoredQuestionText get tag =>
      InputAuthoredQuestionText.read(json["tag"]);
  Presence<InputAuthoredCut> get threshold => json.containsKey("threshold")
      ? Presence.present(InputAuthoredCut.read(json["threshold"]))
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

final class InputRequestDefinitionFieldsChoose extends InputRequestDefinition {
  InputRequestDefinitionFieldsChoose._(super.json);
  factory InputRequestDefinitionFieldsChoose.read(Object? value) =>
      InputRequestDefinitionFieldsChoose._(readObject(value));
  InputRequestDefinitionFieldsChoose(
      {Presence<Object?> batch = const Presence.absent(),
      required InputAuthoredQuestionText choose,
      Presence<InputAuthoredInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> itemSchema =
          const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<InputAuthoredPointers> on = const Presence.absent(),
      Presence<InputAuthoredOptions> options = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      Presence<InputAuthoredCut> threshold = const Presence.absent(),
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
  Presence<Object?> get batch => json.containsKey("batch")
      ? Presence.present(json["batch"])
      : const Presence.absent();
  InputAuthoredQuestionText get choose =>
      InputAuthoredQuestionText.read(json["choose"]);
  Presence<InputAuthoredInputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get itemSchema =>
      json.containsKey("item_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["item_schema"]))
          : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<InputAuthoredPointers> get on => json.containsKey("on")
      ? Presence.present(InputAuthoredPointers.read(json["on"]))
      : const Presence.absent();
  Presence<InputAuthoredOptions> get options => json.containsKey("options")
      ? Presence.present(InputAuthoredOptions.read(json["options"]))
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  Presence<InputAuthoredCut> get threshold => json.containsKey("threshold")
      ? Presence.present(InputAuthoredCut.read(json["threshold"]))
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

final class InputRequestDefinitionFieldsDecide extends InputRequestDefinition {
  InputRequestDefinitionFieldsDecide._(super.json);
  factory InputRequestDefinitionFieldsDecide.read(Object? value) =>
      InputRequestDefinitionFieldsDecide._(readObject(value));
  InputRequestDefinitionFieldsDecide(
      {Presence<Object?> batch = const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> contextSchema =
          const Presence.absent(),
      required InputAuthoredQuestionText decide,
      Presence<InputAuthoredCriterion> falseValue = const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> itemSchema =
          const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<InputAuthoredPointers> on = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      Presence<InputAuthoredThreshold> threshold = const Presence.absent(),
      Presence<InputAuthoredCriterion> trueValue = const Presence.absent(),
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
  Presence<Object?> get batch => json.containsKey("batch")
      ? Presence.present(json["batch"])
      : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  InputAuthoredQuestionText get decide =>
      InputAuthoredQuestionText.read(json["decide"]);
  Presence<InputAuthoredCriterion> get falseValue => json.containsKey("false")
      ? Presence.present(InputAuthoredCriterion.read(json["false"]))
      : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get itemSchema =>
      json.containsKey("item_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["item_schema"]))
          : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<InputAuthoredPointers> get on => json.containsKey("on")
      ? Presence.present(InputAuthoredPointers.read(json["on"]))
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  Presence<InputAuthoredThreshold> get threshold =>
      json.containsKey("threshold")
          ? Presence.present(InputAuthoredThreshold.read(json["threshold"]))
          : const Presence.absent();
  Presence<InputAuthoredCriterion> get trueValue => json.containsKey("true")
      ? Presence.present(InputAuthoredCriterion.read(json["true"]))
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

final class InputRequestDefinitionFieldsFind extends InputRequestDefinition {
  InputRequestDefinitionFieldsFind._(super.json);
  factory InputRequestDefinitionFieldsFind.read(Object? value) =>
      InputRequestDefinitionFieldsFind._(readObject(value));
  InputRequestDefinitionFieldsFind(
      {Presence<InputAuthoredInputDeclaration> contextSchema =
          const Presence.absent(),
      required InputAuthoredQuestionText find,
      Presence<InputAuthoredInputDeclaration> itemSchema =
          const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<InputAuthoredPointers> on = const Presence.absent(),
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
  Presence<InputAuthoredInputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  InputAuthoredQuestionText get find =>
      InputAuthoredQuestionText.read(json["find"]);
  Presence<InputAuthoredInputDeclaration> get itemSchema =>
      json.containsKey("item_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["item_schema"]))
          : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<InputAuthoredPointers> get on => json.containsKey("on")
      ? Presence.present(InputAuthoredPointers.read(json["on"]))
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

final class InputRequestDefinitionFieldsQuestionsVersion
    extends InputRequestDefinition {
  InputRequestDefinitionFieldsQuestionsVersion._(super.json);
  factory InputRequestDefinitionFieldsQuestionsVersion.read(Object? value) =>
      InputRequestDefinitionFieldsQuestionsVersion._(readObject(value));
  InputRequestDefinitionFieldsQuestionsVersion(
      {Presence<Object?> batch = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      required Map<String,
              InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties>
          questions,
      Presence<InputAuthoredThreshold> threshold = const Presence.absent()})
      : super({
          if (batch.isPresent) "batch": batch.value,
          if (profile.isPresent) "profile": profile.value,
          "questions": questions,
          if (threshold.isPresent) "threshold": threshold.value,
          "version": 1
        });
  Presence<Object?> get batch => json.containsKey("batch")
      ? Presence.present(json["batch"])
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  Map<String,
          InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties>
      get questions =>
          (json["questions"] as Map<String, Object?>).map((k, v) => MapEntry(
              k,
              InputRequestDefinitionAnyOf7PropertiesQuestionsAdditionalProperties
                  .read(v)));
  Presence<InputAuthoredThreshold> get threshold =>
      json.containsKey("threshold")
          ? Presence.present(InputAuthoredThreshold.read(json["threshold"]))
          : const Presence.absent();
  BigInt get version => readInteger(json["version"]);
}

final class InputRequestDefinitionFieldsRecognizeVersion
    extends InputRequestDefinition {
  InputRequestDefinitionFieldsRecognizeVersion._(super.json);
  factory InputRequestDefinitionFieldsRecognizeVersion.read(Object? value) =>
      InputRequestDefinitionFieldsRecognizeVersion._(readObject(value));
  InputRequestDefinitionFieldsRecognizeVersion(
      {Presence<InputAuthoredInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> itemSchema =
          const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<InputAuthoredPointers> on = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      required Object? recognize,
      Presence<InputAuthoredCut> relationThreshold = const Presence.absent(),
      Presence<InputAuthoredCut> threshold = const Presence.absent(),
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
  Presence<InputAuthoredInputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get itemSchema =>
      json.containsKey("item_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["item_schema"]))
          : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<InputAuthoredPointers> get on => json.containsKey("on")
      ? Presence.present(InputAuthoredPointers.read(json["on"]))
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  Object? get recognize => json["recognize"];
  Presence<InputAuthoredCut> get relationThreshold =>
      json.containsKey("relation_threshold")
          ? Presence.present(InputAuthoredCut.read(json["relation_threshold"]))
          : const Presence.absent();
  Presence<InputAuthoredCut> get threshold => json.containsKey("threshold")
      ? Presence.present(InputAuthoredCut.read(json["threshold"]))
      : const Presence.absent();
  BigInt get version => readInteger(json["version"]);
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

final class InputRequestDefinitionFieldsRelateVersion
    extends InputRequestDefinition {
  InputRequestDefinitionFieldsRelateVersion._(super.json);
  factory InputRequestDefinitionFieldsRelateVersion.read(Object? value) =>
      InputRequestDefinitionFieldsRelateVersion._(readObject(value));
  InputRequestDefinitionFieldsRelateVersion(
      {Presence<InputAuthoredInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> itemSchema =
          const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      required Object? relate,
      Presence<InputAuthoredCut> threshold = const Presence.absent(),
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
  Presence<InputAuthoredInputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get itemSchema =>
      json.containsKey("item_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["item_schema"]))
          : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  Object? get relate => json["relate"];
  Presence<InputAuthoredCut> get threshold => json.containsKey("threshold")
      ? Presence.present(InputAuthoredCut.read(json["threshold"]))
      : const Presence.absent();
  BigInt get version => readInteger(json["version"]);
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

final class InputRequestDefinitionFieldsScore extends InputRequestDefinition {
  InputRequestDefinitionFieldsScore._(super.json);
  factory InputRequestDefinitionFieldsScore.read(Object? value) =>
      InputRequestDefinitionFieldsScore._(readObject(value));
  InputRequestDefinitionFieldsScore(
      {Presence<Object?> batch = const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> itemSchema =
          const Presence.absent(),
      Presence<InputAuthoredLevels> levels = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<InputAuthoredPointers> on = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      required InputAuthoredQuestionText score,
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
  Presence<Object?> get batch => json.containsKey("batch")
      ? Presence.present(json["batch"])
      : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get itemSchema =>
      json.containsKey("item_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["item_schema"]))
          : const Presence.absent();
  Presence<InputAuthoredLevels> get levels => json.containsKey("levels")
      ? Presence.present(InputAuthoredLevels.read(json["levels"]))
      : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<InputAuthoredPointers> get on => json.containsKey("on")
      ? Presence.present(InputAuthoredPointers.read(json["on"]))
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  InputAuthoredQuestionText get score =>
      InputAuthoredQuestionText.read(json["score"]);
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

final class InputRequestDefinitionFieldsTag extends InputRequestDefinition {
  InputRequestDefinitionFieldsTag._(super.json);
  factory InputRequestDefinitionFieldsTag.read(Object? value) =>
      InputRequestDefinitionFieldsTag._(readObject(value));
  InputRequestDefinitionFieldsTag(
      {Presence<Object?> batch = const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> contextSchema =
          const Presence.absent(),
      Presence<InputAuthoredInputDeclaration> itemSchema =
          const Presence.absent(),
      Presence<InputAuthoredLabels> labels = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<String> name = const Presence.absent(),
      Presence<InputAuthoredPointers> on = const Presence.absent(),
      Presence<String> profile = const Presence.absent(),
      required InputAuthoredQuestionText tag,
      Presence<InputAuthoredCut> threshold = const Presence.absent(),
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
  Presence<Object?> get batch => json.containsKey("batch")
      ? Presence.present(json["batch"])
      : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get contextSchema =>
      json.containsKey("context_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["context_schema"]))
          : const Presence.absent();
  Presence<InputAuthoredInputDeclaration> get itemSchema =>
      json.containsKey("item_schema")
          ? Presence.present(
              InputAuthoredInputDeclaration.read(json["item_schema"]))
          : const Presence.absent();
  Presence<InputAuthoredLabels> get labels => json.containsKey("labels")
      ? Presence.present(InputAuthoredLabels.read(json["labels"]))
      : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<String> get name => json.containsKey("name")
      ? Presence.present(json["name"] as String)
      : const Presence.absent();
  Presence<InputAuthoredPointers> get on => json.containsKey("on")
      ? Presence.present(InputAuthoredPointers.read(json["on"]))
      : const Presence.absent();
  Presence<String> get profile => json.containsKey("profile")
      ? Presence.present(json["profile"] as String)
      : const Presence.absent();
  InputAuthoredQuestionText get tag =>
      InputAuthoredQuestionText.read(json["tag"]);
  Presence<InputAuthoredCut> get threshold => json.containsKey("threshold")
      ? Presence.present(InputAuthoredCut.read(json["threshold"]))
      : const Presence.absent();
  Presence<BigInt> get wordingVersion => json.containsKey("wording_version")
      ? Presence.present(readInteger(json["wording_version"]))
      : const Presence.absent();
}

sealed class InputRequestImage extends NativeObject {
  InputRequestImage(super.json);
  factory InputRequestImage.read(Object? value) {
    final map = readObject(value);
    if (map["kind"] == "file") return InputRequestImageFile.read(map);
    if (map["kind"] == "bytes") return InputRequestImageBytes.read(map);
    throw FormatException("Unknown InputRequestImage alternative");
  }
}

final class InputRequestImageBytes extends InputRequestImage {
  InputRequestImageBytes._(super.json);
  factory InputRequestImageBytes.read(Object? value) =>
      InputRequestImageBytes._(readObject(value));
  InputRequestImageBytes({required String bytes, required String media})
      : super({"bytes": bytes, "kind": "bytes", "media": media});
  String get bytes => json["bytes"] as String;
  String get kind => json["kind"] as String;
  String get media => json["media"] as String;
}

final class InputRequestImageFile extends InputRequestImage {
  InputRequestImageFile._(super.json);
  factory InputRequestImageFile.read(Object? value) =>
      InputRequestImageFile._(readObject(value));
  InputRequestImageFile(
      {Presence<String> media = const Presence.absent(), required String path})
      : super({
          "kind": "file",
          if (media.isPresent) "media": media.value,
          "path": path
        });
  String get kind => json["kind"] as String;
  Presence<String> get media => json.containsKey("media")
      ? Presence.present(json["media"] as String)
      : const Presence.absent();
  String get path => json["path"] as String;
}

sealed class InputRequestInput extends NativeObject {
  InputRequestInput(super.json);
  factory InputRequestInput.read(Object? value) {
    final map = readObject(value);
    if (map["kind"] == "text") return InputRequestInputText.read(map);
    if (map["kind"] == "json") return InputRequestInputJson.read(map);
    if (map["kind"] == "records") return InputRequestInputRecords.read(map);
    if (map["kind"] == "units") return InputRequestInputUnits.read(map);
    if (map["kind"] == "entities") return InputRequestInputEntities.read(map);
    if (map["kind"] == "source") return InputRequestInputSource.read(map);
    if (map["kind"] == "feed") return InputRequestInputFeed.read(map);
    throw FormatException("Unknown InputRequestInput alternative");
  }
}

final class InputRequestInputEntities extends InputRequestInput {
  InputRequestInputEntities._(super.json);
  factory InputRequestInputEntities.read(Object? value) =>
      InputRequestInputEntities._(readObject(value));
  InputRequestInputEntities({required List<InputRequestItem> items})
      : super({"items": items, "kind": "entities"});
  List<InputRequestItem> get items => (json["items"] as List)
      .map((v) => InputRequestItem.read(v))
      .toList(growable: false);
  String get kind => json["kind"] as String;
}

final class InputRequestInputFeed extends InputRequestInput {
  InputRequestInputFeed._(super.json);
  factory InputRequestInputFeed.read(Object? value) =>
      InputRequestInputFeed._(readObject(value));
  InputRequestInputFeed(
      {Presence<String> framing = const Presence.absent(),
      Presence<List<InputRequestImage>> images = const Presence.absent(),
      required String name,
      Presence<InputRequestReader> reading = const Presence.absent()})
      : super({
          if (framing.isPresent) "framing": framing.value,
          if (images.isPresent) "images": images.value,
          "kind": "feed",
          "name": name,
          if (reading.isPresent) "reading": reading.value
        });
  Presence<String> get framing => json.containsKey("framing")
      ? Presence.present(json["framing"] as String)
      : const Presence.absent();
  Presence<List<InputRequestImage>> get images => json.containsKey("images")
      ? Presence.present((json["images"] as List)
          .map((v) => InputRequestImage.read(v))
          .toList(growable: false))
      : const Presence.absent();
  String get kind => json["kind"] as String;
  String get name => json["name"] as String;
  Presence<InputRequestReader> get reading => json.containsKey("reading")
      ? Presence.present(InputRequestReader.read(json["reading"]))
      : const Presence.absent();
}

final class InputRequestInputJson extends InputRequestInput {
  InputRequestInputJson._(super.json);
  factory InputRequestInputJson.read(Object? value) =>
      InputRequestInputJson._(readObject(value));
  InputRequestInputJson(
      {Presence<List<InputRequestImage>> images = const Presence.absent(),
      required Object? value})
      : super({
          if (images.isPresent) "images": images.value,
          "kind": "json",
          "value": value
        });
  Presence<List<InputRequestImage>> get images => json.containsKey("images")
      ? Presence.present((json["images"] as List)
          .map((v) => InputRequestImage.read(v))
          .toList(growable: false))
      : const Presence.absent();
  String get kind => json["kind"] as String;
  Object? get value => json["value"];
}

final class InputRequestInputRecords extends InputRequestInput {
  InputRequestInputRecords._(super.json);
  factory InputRequestInputRecords.read(Object? value) =>
      InputRequestInputRecords._(readObject(value));
  InputRequestInputRecords({required List<InputRequestItem> items})
      : super({"items": items, "kind": "records"});
  List<InputRequestItem> get items => (json["items"] as List)
      .map((v) => InputRequestItem.read(v))
      .toList(growable: false);
  String get kind => json["kind"] as String;
}

final class InputRequestInputSource extends InputRequestInput {
  InputRequestInputSource._(super.json);
  factory InputRequestInputSource.read(Object? value) =>
      InputRequestInputSource._(readObject(value));
  InputRequestInputSource({required InputRequestSource source})
      : super({"kind": "source", "source": source});
  String get kind => json["kind"] as String;
  InputRequestSource get source => InputRequestSource.read(json["source"]);
}

final class InputRequestInputText extends InputRequestInput {
  InputRequestInputText._(super.json);
  factory InputRequestInputText.read(Object? value) =>
      InputRequestInputText._(readObject(value));
  InputRequestInputText(
      {Presence<List<InputRequestImage>> images = const Presence.absent(),
      required String text})
      : super({
          if (images.isPresent) "images": images.value,
          "kind": "text",
          "text": text
        });
  Presence<List<InputRequestImage>> get images => json.containsKey("images")
      ? Presence.present((json["images"] as List)
          .map((v) => InputRequestImage.read(v))
          .toList(growable: false))
      : const Presence.absent();
  String get kind => json["kind"] as String;
  String get text => json["text"] as String;
}

final class InputRequestInputUnits extends InputRequestInput {
  InputRequestInputUnits._(super.json);
  factory InputRequestInputUnits.read(Object? value) =>
      InputRequestInputUnits._(readObject(value));
  InputRequestInputUnits({required List<InputRequestItem> items})
      : super({"items": items, "kind": "units"});
  List<InputRequestItem> get items => (json["items"] as List)
      .map((v) => InputRequestItem.read(v))
      .toList(growable: false);
  String get kind => json["kind"] as String;
}

final class InputRequestItem extends NativeObject {
  InputRequestItem._(super.json);
  factory InputRequestItem.read(Object? value) =>
      InputRequestItem._(readObject(value));
  InputRequestItem(
      {Presence<InputContextSchema> context = const Presence.absent(),
      Presence<List<InputRecognitionExample>> examples =
          const Presence.absent(),
      Presence<List<InputRequestImage>> images = const Presence.absent(),
      Presence<List<InputOptionSchema>> options = const Presence.absent(),
      Presence<InputRequestOriginal> original = const Presence.absent(),
      Presence<List<InputRecognitionSeedSpan>> seedSpans =
          const Presence.absent()})
      : super({
          if (context.isPresent) "context": context.value,
          if (examples.isPresent) "examples": examples.value,
          if (images.isPresent) "images": images.value,
          if (options.isPresent) "options": options.value,
          if (original.isPresent) "original": original.value,
          if (seedSpans.isPresent) "seed_spans": seedSpans.value
        });
  Presence<InputContextSchema> get context => json.containsKey("context")
      ? Presence.present(InputContextSchema.read(json["context"]))
      : const Presence.absent();
  Presence<List<InputRecognitionExample>> get examples =>
      json.containsKey("examples")
          ? Presence.present((json["examples"] as List)
              .map((v) => InputRecognitionExample.read(v))
              .toList(growable: false))
          : const Presence.absent();
  Presence<List<InputRequestImage>> get images => json.containsKey("images")
      ? Presence.present((json["images"] as List)
          .map((v) => InputRequestImage.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<List<InputOptionSchema>> get options => json.containsKey("options")
      ? Presence.present((json["options"] as List)
          .map((v) => InputOptionSchema.read(v))
          .toList(growable: false))
      : const Presence.absent();
  Presence<InputRequestOriginal> get original => json.containsKey("original")
      ? Presence.present(InputRequestOriginal.read(json["original"]))
      : const Presence.absent();
  Presence<List<InputRecognitionSeedSpan>> get seedSpans =>
      json.containsKey("seed_spans")
          ? Presence.present((json["seed_spans"] as List)
              .map((v) => InputRecognitionSeedSpan.read(v))
              .toList(growable: false))
          : const Presence.absent();
}

final class InputRequestOptions extends NativeObject {
  InputRequestOptions._(super.json);
  factory InputRequestOptions.read(Object? value) =>
      InputRequestOptions._(readObject(value));
  InputRequestOptions(
      {Presence<bool> attempts = const Presence.absent(),
      Presence<InputRequestBatch> batch = const Presence.absent(),
      Presence<String> context = const Presence.absent(),
      Presence<String> contextField = const Presence.absent(),
      Presence<BigInt> deadlineMs = const Presence.absent(),
      Presence<bool> details = const Presence.absent(),
      Presence<List<InputRecognitionExample>> examples =
          const Presence.absent(),
      Presence<String> examplesField = const Presence.absent(),
      Presence<List<String>> field = const Presence.absent(),
      Presence<bool> filesOnly = const Presence.absent(),
      Presence<BigInt> maxRequestsTotal = const Presence.absent(),
      Presence<String> mode = const Presence.absent(),
      Presence<String> model = const Presence.absent(),
      Presence<bool> none = const Presence.absent(),
      Presence<String> optionsField = const Presence.absent(),
      Presence<InputRequestThreshold> relationThreshold =
          const Presence.absent(),
      Presence<List<InputRecognitionSeedSpan>> seedSpans =
          const Presence.absent(),
      Presence<String> seedSpansField = const Presence.absent(),
      Presence<BigInt> snippetPieces = const Presence.absent(),
      Presence<InputRecognitionStageContext> stageContext =
          const Presence.absent(),
      Presence<InputRequestThreshold> threshold = const Presence.absent(),
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
  Presence<bool> get attempts => json.containsKey("attempts")
      ? Presence.present(json["attempts"] as bool)
      : const Presence.absent();
  Presence<InputRequestBatch> get batch => json.containsKey("batch")
      ? Presence.present(InputRequestBatch.read(json["batch"]))
      : const Presence.absent();
  Presence<String> get context => json.containsKey("context")
      ? Presence.present(json["context"] as String)
      : const Presence.absent();
  Presence<String> get contextField => json.containsKey("context_field")
      ? Presence.present(json["context_field"] as String)
      : const Presence.absent();
  Presence<BigInt> get deadlineMs => json.containsKey("deadline_ms")
      ? Presence.present(readInteger(json["deadline_ms"]))
      : const Presence.absent();
  Presence<bool> get details => json.containsKey("details")
      ? Presence.present(json["details"] as bool)
      : const Presence.absent();
  Presence<List<InputRecognitionExample>> get examples =>
      json.containsKey("examples")
          ? Presence.present((json["examples"] as List)
              .map((v) => InputRecognitionExample.read(v))
              .toList(growable: false))
          : const Presence.absent();
  Presence<String> get examplesField => json.containsKey("examples_field")
      ? Presence.present(json["examples_field"] as String)
      : const Presence.absent();
  Presence<List<String>> get field => json.containsKey("field")
      ? Presence.present((json["field"] as List)
          .map((v) => v as String)
          .toList(growable: false))
      : const Presence.absent();
  Presence<bool> get filesOnly => json.containsKey("files_only")
      ? Presence.present(json["files_only"] as bool)
      : const Presence.absent();
  Presence<BigInt> get maxRequestsTotal =>
      json.containsKey("max_requests_total")
          ? Presence.present(readInteger(json["max_requests_total"]))
          : const Presence.absent();
  Presence<String> get mode => json.containsKey("mode")
      ? Presence.present(json["mode"] as String)
      : const Presence.absent();
  Presence<String> get model => json.containsKey("model")
      ? Presence.present(json["model"] as String)
      : const Presence.absent();
  Presence<bool> get none => json.containsKey("none")
      ? Presence.present(json["none"] as bool)
      : const Presence.absent();
  Presence<String> get optionsField => json.containsKey("options_field")
      ? Presence.present(json["options_field"] as String)
      : const Presence.absent();
  Presence<InputRequestThreshold> get relationThreshold => json
          .containsKey("relation_threshold")
      ? Presence.present(InputRequestThreshold.read(json["relation_threshold"]))
      : const Presence.absent();
  Presence<List<InputRecognitionSeedSpan>> get seedSpans =>
      json.containsKey("seed_spans")
          ? Presence.present((json["seed_spans"] as List)
              .map((v) => InputRecognitionSeedSpan.read(v))
              .toList(growable: false))
          : const Presence.absent();
  Presence<String> get seedSpansField => json.containsKey("seed_spans_field")
      ? Presence.present(json["seed_spans_field"] as String)
      : const Presence.absent();
  Presence<BigInt> get snippetPieces => json.containsKey("snippet_pieces")
      ? Presence.present(readInteger(json["snippet_pieces"]))
      : const Presence.absent();
  Presence<InputRecognitionStageContext> get stageContext =>
      json.containsKey("stage_context")
          ? Presence.present(
              InputRecognitionStageContext.read(json["stage_context"]))
          : const Presence.absent();
  Presence<InputRequestThreshold> get threshold => json.containsKey("threshold")
      ? Presence.present(InputRequestThreshold.read(json["threshold"]))
      : const Presence.absent();
  Presence<BigInt> get top => json.containsKey("top")
      ? Presence.present(readInteger(json["top"]))
      : const Presence.absent();
}

sealed class InputRequestOriginal extends NativeObject {
  InputRequestOriginal(super.json);
  factory InputRequestOriginal.read(Object? value) {
    final map = readObject(value);
    if (map["kind"] == "text") return InputRequestOriginalText.read(map);
    if (map["kind"] == "json") return InputRequestOriginalJson.read(map);
    throw FormatException("Unknown InputRequestOriginal alternative");
  }
}

final class InputRequestOriginalJson extends InputRequestOriginal {
  InputRequestOriginalJson._(super.json);
  factory InputRequestOriginalJson.read(Object? value) =>
      InputRequestOriginalJson._(readObject(value));
  InputRequestOriginalJson({required Object? value})
      : super({"kind": "json", "value": value});
  String get kind => json["kind"] as String;
  Object? get value => json["value"];
}

final class InputRequestOriginalText extends InputRequestOriginal {
  InputRequestOriginalText._(super.json);
  factory InputRequestOriginalText.read(Object? value) =>
      InputRequestOriginalText._(readObject(value));
  InputRequestOriginalText({required String text})
      : super({"kind": "text", "text": text});
  String get kind => json["kind"] as String;
  String get text => json["text"] as String;
}

sealed class InputRequestQuestion extends NativeObject {
  InputRequestQuestion(super.json);
  factory InputRequestQuestion.read(Object? value) {
    final map = readObject(value);
    if (map["kind"] == "text") return InputRequestQuestionText.read(map);
    if (map["kind"] == "definition")
      return InputRequestQuestionDefinition.read(map);
    if (map["kind"] == "file") return InputRequestQuestionFile.read(map);
    if (map["kind"] == "name") return InputRequestQuestionName.read(map);
    if (map["kind"] == "reference")
      return InputRequestQuestionReference.read(map);
    throw FormatException("Unknown InputRequestQuestion alternative");
  }
}

final class InputRequestQuestionDefinition extends InputRequestQuestion {
  InputRequestQuestionDefinition._(super.json);
  factory InputRequestQuestionDefinition.read(Object? value) =>
      InputRequestQuestionDefinition._(readObject(value));
  InputRequestQuestionDefinition({required InputRequestDefinition value})
      : super({"kind": "definition", "value": value});
  String get kind => json["kind"] as String;
  InputRequestDefinition get value =>
      InputRequestDefinition.read(json["value"]);
}

final class InputRequestQuestionFile extends InputRequestQuestion {
  InputRequestQuestionFile._(super.json);
  factory InputRequestQuestionFile.read(Object? value) =>
      InputRequestQuestionFile._(readObject(value));
  InputRequestQuestionFile({required String path})
      : super({"kind": "file", "path": path});
  String get kind => json["kind"] as String;
  String get path => json["path"] as String;
}

final class InputRequestQuestionName extends InputRequestQuestion {
  InputRequestQuestionName._(super.json);
  factory InputRequestQuestionName.read(Object? value) =>
      InputRequestQuestionName._(readObject(value));
  InputRequestQuestionName({required String name})
      : super({"kind": "name", "name": name});
  String get kind => json["kind"] as String;
  String get name => json["name"] as String;
}

final class InputRequestQuestionReference extends InputRequestQuestion {
  InputRequestQuestionReference._(super.json);
  factory InputRequestQuestionReference.read(Object? value) =>
      InputRequestQuestionReference._(readObject(value));
  InputRequestQuestionReference({required String reference})
      : super({"kind": "reference", "reference": reference});
  String get kind => json["kind"] as String;
  String get reference => json["reference"] as String;
}

final class InputRequestQuestionText extends InputRequestQuestion {
  InputRequestQuestionText._(super.json);
  factory InputRequestQuestionText.read(Object? value) =>
      InputRequestQuestionText._(readObject(value));
  InputRequestQuestionText({required String text})
      : super({"kind": "text", "text": text});
  String get kind => json["kind"] as String;
  String get text => json["text"] as String;
}

final class InputRequestReader extends NativeObject {
  InputRequestReader._(super.json);
  factory InputRequestReader.read(Object? value) =>
      InputRequestReader._(readObject(value));
  InputRequestReader(
      {Presence<String> unit = const Presence.absent(),
      Presence<BigInt> window = const Presence.absent()})
      : super({
          if (unit.isPresent) "unit": unit.value,
          if (window.isPresent) "window": window.value
        });
  Presence<String> get unit => json.containsKey("unit")
      ? Presence.present(json["unit"] as String)
      : const Presence.absent();
  Presence<BigInt> get window => json.containsKey("window")
      ? Presence.present(readInteger(json["window"]))
      : const Presence.absent();
}

final class InputRequestSessionDescriptor extends NativeObject {
  InputRequestSessionDescriptor._(super.json);
  factory InputRequestSessionDescriptor.read(Object? value) =>
      InputRequestSessionDescriptor._(readObject(value));
  InputRequestSessionDescriptor(
      {required InputRequestItem item,
      Presence<InputSessionSourceLocation> location = const Presence.absent()})
      : super(
            {"item": item, if (location.isPresent) "location": location.value});
  InputRequestItem get item => InputRequestItem.read(json["item"]);
  Presence<InputSessionSourceLocation> get location =>
      json.containsKey("location")
          ? Presence.present(InputSessionSourceLocation.read(json["location"]))
          : const Presence.absent();
}

final class InputRequestSource extends NativeObject {
  InputRequestSource._(super.json);
  factory InputRequestSource.read(Object? value) =>
      InputRequestSource._(readObject(value));
  InputRequestSource(
      {Presence<String> framing = const Presence.absent(),
      Presence<String> media = const Presence.absent(),
      required List<String> paths,
      Presence<InputRequestReader> reading = const Presence.absent()})
      : super({
          if (framing.isPresent) "framing": framing.value,
          if (media.isPresent) "media": media.value,
          "paths": paths,
          if (reading.isPresent) "reading": reading.value
        });
  Presence<String> get framing => json.containsKey("framing")
      ? Presence.present(json["framing"] as String)
      : const Presence.absent();
  Presence<String> get media => json.containsKey("media")
      ? Presence.present(json["media"] as String)
      : const Presence.absent();
  List<String> get paths =>
      (json["paths"] as List).map((v) => v as String).toList(growable: false);
  Presence<InputRequestReader> get reading => json.containsKey("reading")
      ? Presence.present(InputRequestReader.read(json["reading"]))
      : const Presence.absent();
}

final class InputRequestThreshold {
  final Object? value;
  const InputRequestThreshold._(this.value);
  factory InputRequestThreshold.read(Object? value) =>
      value is InputRequestThreshold ? value : InputRequestThreshold._(value);
  Object? toJson() => value;
  const InputRequestThreshold.alternative0(num this.value);
  num get asAlternative0 => value as num;
  const InputRequestThreshold.alternative1(String this.value);
  String get asAlternative1 => value as String;
}

final class InputSessionSourceLocation extends NativeObject {
  InputSessionSourceLocation._(super.json);
  factory InputSessionSourceLocation.read(Object? value) =>
      InputSessionSourceLocation._(readObject(value));
  InputSessionSourceLocation(
      {required String file,
      Presence<BigInt> firstLine = const Presence.absent(),
      Presence<BigInt> lastLine = const Presence.absent()})
      : super({
          "file": file,
          if (firstLine.isPresent) "first_line": firstLine.value,
          if (lastLine.isPresent) "last_line": lastLine.value
        });
  String get file => json["file"] as String;
  Presence<BigInt> get firstLine => json.containsKey("first_line")
      ? Presence.present(readInteger(json["first_line"]))
      : const Presence.absent();
  Presence<BigInt> get lastLine => json.containsKey("last_line")
      ? Presence.present(readInteger(json["last_line"]))
      : const Presence.absent();
}

const requestVersion = "thinkthen.request/1";
