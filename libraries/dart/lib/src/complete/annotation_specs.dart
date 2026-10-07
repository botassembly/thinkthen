part of 'models.dart';

final class DecideMember extends Carrier implements AnnotationSpec {
  final Authored decide;
  final Optional<Authored> true_;
  final Optional<Authored> false_;
  final Optional<Threshold> threshold;
  final Optional<Pointers> on;
  const DecideMember({
    required this.decide,
    this.true_ = const Optional.absent(),
    this.false_ = const Optional.absent(),
    this.threshold = const Optional.absent(),
    this.on = const Optional.absent(),
  });
  factory DecideMember.fromJson(Object? value) {
    final v = readObject(value);
    verify("DecideMember", v, ["decide", "true", "false", "threshold", "on"],
        ["decide"]);
    return DecideMember(
      decide: readText(v["decide"]),
      true_: v.containsKey("true")
          ? Optional.present(Authored.fromJson(v["true"]))
          : const Optional.absent(),
      false_: v.containsKey("false")
          ? Optional.present(Authored.fromJson(v["false"]))
          : const Optional.absent(),
      threshold: v.containsKey("threshold")
          ? Optional.present(Threshold.fromJson(v["threshold"]))
          : const Optional.absent(),
      on: v.containsKey("on")
          ? Optional.present(Pointers.fromJson(v["on"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "decide": project(decide),
        if (true_.present) "true": project(true_.value),
        if (false_.present) "false": project(false_.value),
        if (threshold.present) "threshold": project(threshold.value),
        if (on.present) "on": project(on.value),
      };
}

final class ChooseMember extends Carrier implements AnnotationSpec {
  final Authored choose;
  final Labels options;
  final Optional<double> threshold;
  final Optional<Pointers> on;
  const ChooseMember({
    required this.choose,
    required this.options,
    this.threshold = const Optional.absent(),
    this.on = const Optional.absent(),
  });
  factory ChooseMember.fromJson(Object? value) {
    final v = readObject(value);
    verify("ChooseMember", v, ["choose", "options", "threshold", "on"],
        ["choose", "options"]);
    return ChooseMember(
      choose: readText(v["choose"]),
      options: Labels.fromJson(v["options"]),
      threshold: v.containsKey("threshold")
          ? Optional.present(readNumber(v["threshold"], probability: true))
          : const Optional.absent(),
      on: v.containsKey("on")
          ? Optional.present(Pointers.fromJson(v["on"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "choose": project(choose),
        "options": project(options),
        if (threshold.present) "threshold": project(threshold.value),
        if (on.present) "on": project(on.value),
      };
}

final class TagMember extends Carrier implements AnnotationSpec {
  final Authored tag;
  final Labels labels;
  final Optional<double> threshold;
  final Optional<Pointers> on;
  const TagMember({
    required this.tag,
    required this.labels,
    this.threshold = const Optional.absent(),
    this.on = const Optional.absent(),
  });
  factory TagMember.fromJson(Object? value) {
    final v = readObject(value);
    verify("TagMember", v, ["tag", "labels", "threshold", "on"],
        ["tag", "labels"]);
    return TagMember(
      tag: readText(v["tag"]),
      labels: Labels.fromJson(v["labels"]),
      threshold: v.containsKey("threshold")
          ? Optional.present(readNumber(v["threshold"], probability: true))
          : const Optional.absent(),
      on: v.containsKey("on")
          ? Optional.present(Pointers.fromJson(v["on"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "tag": project(tag),
        "labels": project(labels),
        if (threshold.present) "threshold": project(threshold.value),
        if (on.present) "on": project(on.value),
      };
}

final class ScoreMember extends Carrier implements AnnotationSpec {
  final Authored score;
  final Labels levels;
  final Optional<Pointers> on;
  const ScoreMember({
    required this.score,
    required this.levels,
    this.on = const Optional.absent(),
  });
  factory ScoreMember.fromJson(Object? value) {
    final v = readObject(value);
    verify("ScoreMember", v, ["score", "levels", "on"], ["score", "levels"]);
    return ScoreMember(
      score: readText(v["score"]),
      levels: Labels.fromJson(v["levels"]),
      on: v.containsKey("on")
          ? Optional.present(Pointers.fromJson(v["on"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "score": project(score),
        "levels": project(levels),
        if (on.present) "on": project(on.value),
      };
}
