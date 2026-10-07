part of 'models.dart';

final class RecognitionPlan extends Carrier {
  final Optional<Labels> kinds;
  final Optional<List<PlanRule>> relations;
  const RecognitionPlan({
    this.kinds = const Optional.absent(),
    this.relations = const Optional.absent(),
  });
  factory RecognitionPlan.fromJson(Object? value) {
    final v = readObject(value);
    verify("RecognitionPlan", v, ["kinds", "relations"], []);
    return RecognitionPlan(
      kinds: v.containsKey("kinds")
          ? Optional.present(Labels.fromJson(v["kinds"]))
          : const Optional.absent(),
      relations: v.containsKey("relations")
          ? Optional.present(
              readList(v["relations"], (v) => PlanRule.fromJson(v)))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        if (kinds.present) "kinds": project(kinds.value),
        if (relations.present) "relations": project(relations.value),
      };
}

final class PlanRule extends Carrier {
  final String name;
  final Optional<String> source;
  final Optional<String> target;
  final Optional<String> reads;
  final Optional<bool> either;
  final Optional<bool> single;
  const PlanRule({
    required this.name,
    this.source = const Optional.absent(),
    this.target = const Optional.absent(),
    this.reads = const Optional.absent(),
    this.either = const Optional.absent(),
    this.single = const Optional.absent(),
  });
  factory PlanRule.fromJson(Object? value) {
    final v = readObject(value);
    verify("PlanRule", v,
        ["name", "source", "target", "reads", "either", "single"], ["name"]);
    return PlanRule(
      name: readString(v["name"]),
      source: v.containsKey("source")
          ? Optional.present(readString(v["source"]))
          : const Optional.absent(),
      target: v.containsKey("target")
          ? Optional.present(readString(v["target"]))
          : const Optional.absent(),
      reads: v.containsKey("reads")
          ? Optional.present(readString(v["reads"]))
          : const Optional.absent(),
      either: v.containsKey("either")
          ? Optional.present(readBool(v["either"]))
          : const Optional.absent(),
      single: v.containsKey("single")
          ? Optional.present(readBool(v["single"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "name": project(name),
        if (source.present) "source": project(source.value),
        if (target.present) "target": project(target.value),
        if (reads.present) "reads": project(reads.value),
        if (either.present) "either": project(either.value),
        if (single.present) "single": project(single.value),
      };
}

final class RecognitionSpec extends Carrier {
  final int version;
  final RecognitionPlan recognize;
  final Optional<double> threshold;
  final Optional<double> relation_threshold;
  final Optional<String> model;
  final Optional<String> profile;
  final Optional<Pointers> on;
  const RecognitionSpec({
    required this.version,
    required this.recognize,
    this.threshold = const Optional.absent(),
    this.relation_threshold = const Optional.absent(),
    this.model = const Optional.absent(),
    this.profile = const Optional.absent(),
    this.on = const Optional.absent(),
  });
  factory RecognitionSpec.fromJson(Object? value) {
    final v = readObject(value);
    verify("RecognitionSpec", v, [
      "version",
      "recognize",
      "threshold",
      "relation_threshold",
      "model",
      "profile",
      "on"
    ], [
      "version",
      "recognize"
    ]);
    return RecognitionSpec(
      version: readInt(v["version"], 1, 1),
      recognize: RecognitionPlan.fromJson(v["recognize"]),
      threshold: v.containsKey("threshold")
          ? Optional.present(readNumber(v["threshold"], probability: true))
          : const Optional.absent(),
      relation_threshold: v.containsKey("relation_threshold")
          ? Optional.present(
              readNumber(v["relation_threshold"], probability: true))
          : const Optional.absent(),
      model: v.containsKey("model")
          ? Optional.present(readString(v["model"]))
          : const Optional.absent(),
      profile: v.containsKey("profile")
          ? Optional.present(readString(v["profile"]))
          : const Optional.absent(),
      on: v.containsKey("on")
          ? Optional.present(Pointers.fromJson(v["on"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "version": project(version),
        "recognize": project(recognize),
        if (threshold.present) "threshold": project(threshold.value),
        if (relation_threshold.present)
          "relation_threshold": project(relation_threshold.value),
        if (model.present) "model": project(model.value),
        if (profile.present) "profile": project(profile.value),
        if (on.present) "on": project(on.value),
      };
}

final class RelationPlan extends Carrier {
  final List<PlanRule> relations;
  final Optional<RelateFields> fields;
  const RelationPlan({
    required this.relations,
    this.fields = const Optional.absent(),
  });
  factory RelationPlan.fromJson(Object? value) {
    final v = readObject(value);
    verify("RelationPlan", v, ["relations", "fields"], ["relations"]);
    return RelationPlan(
      relations: readList(v["relations"], (v) => PlanRule.fromJson(v)),
      fields: v.containsKey("fields")
          ? Optional.present(RelateFields.fromJson(v["fields"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "relations": project(relations),
        if (fields.present) "fields": project(fields.value),
      };
}

final class RelationSpec extends Carrier {
  final int version;
  final RelationPlan relate;
  final Optional<double> threshold;
  final Optional<String> model;
  final Optional<String> profile;
  const RelationSpec({
    required this.version,
    required this.relate,
    this.threshold = const Optional.absent(),
    this.model = const Optional.absent(),
    this.profile = const Optional.absent(),
  });
  factory RelationSpec.fromJson(Object? value) {
    final v = readObject(value);
    verify(
        "RelationSpec",
        v,
        ["version", "relate", "threshold", "model", "profile"],
        ["version", "relate"]);
    return RelationSpec(
      version: readInt(v["version"], 1, 1),
      relate: RelationPlan.fromJson(v["relate"]),
      threshold: v.containsKey("threshold")
          ? Optional.present(readNumber(v["threshold"], probability: true))
          : const Optional.absent(),
      model: v.containsKey("model")
          ? Optional.present(readString(v["model"]))
          : const Optional.absent(),
      profile: v.containsKey("profile")
          ? Optional.present(readString(v["profile"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "version": project(version),
        "relate": project(relate),
        if (threshold.present) "threshold": project(threshold.value),
        if (model.present) "model": project(model.value),
        if (profile.present) "profile": project(profile.value),
      };
}
