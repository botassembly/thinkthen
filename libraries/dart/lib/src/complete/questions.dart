part of 'models.dart';

final class DecideQuestion extends Carrier implements AtomicQuestion {
  final String verb;
  final Authored text;
  final Optional<Authored> true_;
  final Optional<Authored> false_;
  const DecideQuestion({
    required this.verb,
    required this.text,
    this.true_ = const Optional.absent(),
    this.false_ = const Optional.absent(),
  });
  factory DecideQuestion.fromJson(Object? value) {
    final v = readObject(value);
    verify("DecideQuestion", v, ["verb", "text", "true", "false"],
        ["verb", "text"]);
    return DecideQuestion(
      verb: readLiteral(v["verb"], "decide"),
      text: readText(v["text"]),
      true_: v.containsKey("true")
          ? Optional.present(Authored.fromJson(v["true"]))
          : const Optional.absent(),
      false_: v.containsKey("false")
          ? Optional.present(Authored.fromJson(v["false"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "verb": project(verb),
        "text": project(text),
        if (true_.present) "true": project(true_.value),
        if (false_.present) "false": project(false_.value),
      };
}

final class ChooseQuestion extends Carrier implements AtomicQuestion {
  final String verb;
  final Authored text;
  final List<String> options;
  const ChooseQuestion({
    required this.verb,
    required this.text,
    required this.options,
  });
  factory ChooseQuestion.fromJson(Object? value) {
    final v = readObject(value);
    verify("ChooseQuestion", v, ["verb", "text", "options"],
        ["verb", "text", "options"]);
    return ChooseQuestion(
      verb: readLiteral(v["verb"], "choose"),
      text: readText(v["text"]),
      options: readList(v["options"], (v) => readString(v)),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "verb": project(verb),
        "text": project(text),
        "options": project(options),
      };
}

final class TagQuestion extends Carrier implements AtomicQuestion {
  final String verb;
  final Authored text;
  final List<String> labels;
  const TagQuestion({
    required this.verb,
    required this.text,
    required this.labels,
  });
  factory TagQuestion.fromJson(Object? value) {
    final v = readObject(value);
    verify("TagQuestion", v, ["verb", "text", "labels"],
        ["verb", "text", "labels"]);
    return TagQuestion(
      verb: readLiteral(v["verb"], "tag"),
      text: readText(v["text"]),
      labels: readList(v["labels"], (v) => readString(v)),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "verb": project(verb),
        "text": project(text),
        "labels": project(labels),
      };
}

final class ScoreQuestion extends Carrier implements AtomicQuestion {
  final String verb;
  final Authored text;
  final List<String> levels;
  const ScoreQuestion({
    required this.verb,
    required this.text,
    required this.levels,
  });
  factory ScoreQuestion.fromJson(Object? value) {
    final v = readObject(value);
    verify("ScoreQuestion", v, ["verb", "text", "levels"],
        ["verb", "text", "levels"]);
    return ScoreQuestion(
      verb: readLiteral(v["verb"], "score"),
      text: readText(v["text"]),
      levels: readList(v["levels"], (v) => readString(v)),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "verb": project(verb),
        "text": project(text),
        "levels": project(levels),
      };
}

final class FindQuestion extends Carrier implements AtomicQuestion {
  final String verb;
  final Authored text;
  final bool none;
  const FindQuestion({
    required this.verb,
    required this.text,
    required this.none,
  });
  factory FindQuestion.fromJson(Object? value) {
    final v = readObject(value);
    verify(
        "FindQuestion", v, ["verb", "text", "none"], ["verb", "text", "none"]);
    return FindQuestion(
      verb: readLiteral(v["verb"], "find"),
      text: readText(v["text"]),
      none: readBool(v["none"]),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "verb": project(verb),
        "text": project(text),
        "none": project(none),
      };
}

final class RelationRule extends Carrier {
  final String name;
  final String source;
  final String target;
  final String reads;
  final bool either;
  final Optional<bool> single;
  const RelationRule({
    required this.name,
    required this.source,
    required this.target,
    required this.reads,
    required this.either,
    this.single = const Optional.absent(),
  });
  factory RelationRule.fromJson(Object? value) {
    final v = readObject(value);
    verify(
        "RelationRule",
        v,
        ["name", "source", "target", "reads", "either", "single"],
        ["name", "source", "target", "reads", "either"]);
    return RelationRule(
      name: readString(v["name"]),
      source: readString(v["source"]),
      target: readString(v["target"]),
      reads: readString(v["reads"]),
      either: readBool(v["either"]),
      single: v.containsKey("single")
          ? Optional.present(readBool(v["single"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "name": project(name),
        "source": project(source),
        "target": project(target),
        "reads": project(reads),
        "either": project(either),
        if (single.present) "single": project(single.value),
      };
}

final class RelateFields extends Carrier {
  final String name;
  final String kind;
  const RelateFields({
    required this.name,
    required this.kind,
  });
  factory RelateFields.fromJson(Object? value) {
    final v = readObject(value);
    verify("RelateFields", v, ["name", "kind"], ["name", "kind"]);
    return RelateFields(
      name: readString(v["name"]),
      kind: readString(v["kind"]),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "name": project(name),
        "kind": project(kind),
      };
}

final class RelateQuestion extends Carrier {
  final String verb;
  final RelateFields? fields;
  final List<RelationRule> relations;
  final Threshold threshold;
  final Optional<String> profile;
  const RelateQuestion({
    required this.verb,
    required this.fields,
    required this.relations,
    required this.threshold,
    this.profile = const Optional.absent(),
  });
  factory RelateQuestion.fromJson(Object? value) {
    final v = readObject(value);
    verify(
        "RelateQuestion",
        v,
        ["verb", "fields", "relations", "threshold", "profile"],
        ["verb", "fields", "relations", "threshold"]);
    return RelateQuestion(
      verb: readLiteral(v["verb"], "relate"),
      fields: (v["fields"] == null ? null : RelateFields.fromJson(v["fields"])),
      relations: readList(v["relations"], (v) => RelationRule.fromJson(v)),
      threshold: Threshold.fromJson(v["threshold"]),
      profile: v.containsKey("profile")
          ? Optional.present(readString(v["profile"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "verb": project(verb),
        "fields": project(fields),
        "relations": project(relations),
        "threshold": project(threshold),
        if (profile.present) "profile": project(profile.value),
      };
}

final class RecognizeQuestion extends Carrier {
  final Optional<String> instructions;
  final Optional<String> entity_definition;
  final String verb;
  final Map<String, Authored> kinds;
  final Threshold threshold;
  final Threshold relation_threshold;
  final Optional<List<RelationRule>> relations;
  final Optional<Pointers> on;
  final Optional<String> profile;
  const RecognizeQuestion({
    this.instructions = const Optional.absent(),
    this.entity_definition = const Optional.absent(),
    required this.verb,
    required this.kinds,
    required this.threshold,
    required this.relation_threshold,
    this.relations = const Optional.absent(),
    this.on = const Optional.absent(),
    this.profile = const Optional.absent(),
  });
  factory RecognizeQuestion.fromJson(Object? value) {
    final v = readObject(value);
    verify("RecognizeQuestion", v, [
      "verb",
      "kinds",
      "relations",
      "threshold",
      "relation_threshold",
      "on",
      "profile",
      "instructions",
      "entity_definition"
    ], [
      "verb",
      "kinds",
      "threshold",
      "relation_threshold"
    ]);
    return RecognizeQuestion(
      instructions: v.containsKey("instructions")
          ? Optional.present(readString(v["instructions"]))
          : const Optional.absent(),
      entity_definition: v.containsKey("entity_definition")
          ? Optional.present(readString(v["entity_definition"]))
          : const Optional.absent(),
      verb: readLiteral(v["verb"], "recognize"),
      kinds: readMap(v["kinds"], (v) => Authored.fromJson(v)),
      threshold: Threshold.fromJson(v["threshold"]),
      relation_threshold: Threshold.fromJson(v["relation_threshold"]),
      relations: v.containsKey("relations")
          ? Optional.present(
              readList(v["relations"], (v) => RelationRule.fromJson(v)))
          : const Optional.absent(),
      on: v.containsKey("on")
          ? Optional.present(Pointers.fromJson(v["on"]))
          : const Optional.absent(),
      profile: v.containsKey("profile")
          ? Optional.present(readString(v["profile"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        if (instructions.present) "instructions": project(instructions.value),
        if (entity_definition.present)
          "entity_definition": project(entity_definition.value),
        "verb": project(verb),
        "kinds": project(kinds),
        if (relations.present) "relations": project(relations.value),
        "threshold": project(threshold),
        "relation_threshold": project(relation_threshold),
        if (on.present) "on": project(on.value),
        if (profile.present) "profile": project(profile.value),
      };
}
