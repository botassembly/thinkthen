part of 'models.dart';

final class DecideSpec extends Carrier implements QuestionSpec, RankSpec {
  final Authored decide;
  final Optional<Authored> true_;
  final Optional<Authored> false_;
  final Optional<Threshold> threshold;
  final Optional<String> model;
  final Optional<String> profile;
  final Optional<Batch> batch;
  final Optional<Pointers> on;
  const DecideSpec({
    required this.decide,
    this.true_ = const Optional.absent(),
    this.false_ = const Optional.absent(),
    this.threshold = const Optional.absent(),
    this.model = const Optional.absent(),
    this.profile = const Optional.absent(),
    this.batch = const Optional.absent(),
    this.on = const Optional.absent(),
  });
  factory DecideSpec.fromJson(Object? value) {
    final v = readObject(value);
    verify("DecideSpec", v, [
      "decide",
      "true",
      "false",
      "threshold",
      "model",
      "profile",
      "batch",
      "on"
    ], [
      "decide"
    ]);
    return DecideSpec(
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
      model: v.containsKey("model")
          ? Optional.present(readString(v["model"]))
          : const Optional.absent(),
      profile: v.containsKey("profile")
          ? Optional.present(readString(v["profile"]))
          : const Optional.absent(),
      batch: v.containsKey("batch")
          ? Optional.present(Batch.fromJson(v["batch"]))
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
        if (model.present) "model": project(model.value),
        if (profile.present) "profile": project(profile.value),
        if (batch.present) "batch": project(batch.value),
        if (on.present) "on": project(on.value),
      };
}

final class ChooseSpec extends Carrier implements QuestionSpec {
  final Authored choose;
  final Labels options;
  final Optional<double> threshold;
  final Optional<String> model;
  final Optional<String> profile;
  final Optional<Batch> batch;
  final Optional<Pointers> on;
  const ChooseSpec({
    required this.choose,
    required this.options,
    this.threshold = const Optional.absent(),
    this.model = const Optional.absent(),
    this.profile = const Optional.absent(),
    this.batch = const Optional.absent(),
    this.on = const Optional.absent(),
  });
  factory ChooseSpec.fromJson(Object? value) {
    final v = readObject(value);
    verify(
        "ChooseSpec",
        v,
        ["choose", "options", "threshold", "model", "profile", "batch", "on"],
        ["choose", "options"]);
    return ChooseSpec(
      choose: readText(v["choose"]),
      options: Labels.fromJson(v["options"]),
      threshold: v.containsKey("threshold")
          ? Optional.present(readNumber(v["threshold"], probability: true))
          : const Optional.absent(),
      model: v.containsKey("model")
          ? Optional.present(readString(v["model"]))
          : const Optional.absent(),
      profile: v.containsKey("profile")
          ? Optional.present(readString(v["profile"]))
          : const Optional.absent(),
      batch: v.containsKey("batch")
          ? Optional.present(Batch.fromJson(v["batch"]))
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
        if (model.present) "model": project(model.value),
        if (profile.present) "profile": project(profile.value),
        if (batch.present) "batch": project(batch.value),
        if (on.present) "on": project(on.value),
      };
}

final class TagSpec extends Carrier implements QuestionSpec {
  final Authored tag;
  final Labels labels;
  final Optional<double> threshold;
  final Optional<String> model;
  final Optional<String> profile;
  final Optional<Batch> batch;
  final Optional<Pointers> on;
  const TagSpec({
    required this.tag,
    required this.labels,
    this.threshold = const Optional.absent(),
    this.model = const Optional.absent(),
    this.profile = const Optional.absent(),
    this.batch = const Optional.absent(),
    this.on = const Optional.absent(),
  });
  factory TagSpec.fromJson(Object? value) {
    final v = readObject(value);
    verify(
        "TagSpec",
        v,
        ["tag", "labels", "threshold", "model", "profile", "batch", "on"],
        ["tag", "labels"]);
    return TagSpec(
      tag: readText(v["tag"]),
      labels: Labels.fromJson(v["labels"]),
      threshold: v.containsKey("threshold")
          ? Optional.present(readNumber(v["threshold"], probability: true))
          : const Optional.absent(),
      model: v.containsKey("model")
          ? Optional.present(readString(v["model"]))
          : const Optional.absent(),
      profile: v.containsKey("profile")
          ? Optional.present(readString(v["profile"]))
          : const Optional.absent(),
      batch: v.containsKey("batch")
          ? Optional.present(Batch.fromJson(v["batch"]))
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
        if (model.present) "model": project(model.value),
        if (profile.present) "profile": project(profile.value),
        if (batch.present) "batch": project(batch.value),
        if (on.present) "on": project(on.value),
      };
}

final class ScoreSpec extends Carrier implements QuestionSpec, RankSpec {
  final Authored score;
  final Labels levels;
  final Optional<String> model;
  final Optional<String> profile;
  final Optional<Batch> batch;
  final Optional<Pointers> on;
  const ScoreSpec({
    required this.score,
    required this.levels,
    this.model = const Optional.absent(),
    this.profile = const Optional.absent(),
    this.batch = const Optional.absent(),
    this.on = const Optional.absent(),
  });
  factory ScoreSpec.fromJson(Object? value) {
    final v = readObject(value);
    verify(
        "ScoreSpec",
        v,
        ["score", "levels", "model", "profile", "batch", "on"],
        ["score", "levels"]);
    return ScoreSpec(
      score: readText(v["score"]),
      levels: Labels.fromJson(v["levels"]),
      model: v.containsKey("model")
          ? Optional.present(readString(v["model"]))
          : const Optional.absent(),
      profile: v.containsKey("profile")
          ? Optional.present(readString(v["profile"]))
          : const Optional.absent(),
      batch: v.containsKey("batch")
          ? Optional.present(Batch.fromJson(v["batch"]))
          : const Optional.absent(),
      on: v.containsKey("on")
          ? Optional.present(Pointers.fromJson(v["on"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "score": project(score),
        "levels": project(levels),
        if (model.present) "model": project(model.value),
        if (profile.present) "profile": project(profile.value),
        if (batch.present) "batch": project(batch.value),
        if (on.present) "on": project(on.value),
      };
}

final class FindSpec extends Carrier {
  final Authored find;
  final Optional<bool> none;
  final Optional<String> model;
  const FindSpec({
    required this.find,
    this.none = const Optional.absent(),
    this.model = const Optional.absent(),
  });
  factory FindSpec.fromJson(Object? value) {
    final v = readObject(value);
    verify("FindSpec", v, ["find", "none", "model"], ["find"]);
    return FindSpec(
      find: readText(v["find"]),
      none: v.containsKey("none")
          ? Optional.present(readBool(v["none"]))
          : const Optional.absent(),
      model: v.containsKey("model")
          ? Optional.present(readString(v["model"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "find": project(find),
        if (none.present) "none": project(none.value),
        if (model.present) "model": project(model.value),
      };
}

final class QuestionFile extends Carrier implements RankSpec {
  final String path;
  const QuestionFile({
    required this.path,
  });
  factory QuestionFile.fromJson(Object? value) {
    final v = readObject(value);
    verify("QuestionFile", v, ["path"], ["path"]);
    return QuestionFile(
      path: readString(v["path"]),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "path": project(path),
      };
}

final class QuestionSet extends Carrier implements RankSpec {
  final int version;
  final Map<String, AnnotationSpec> questions;
  final Optional<Batch> batch;
  final Optional<Threshold> threshold;
  final Optional<String> profile;
  const QuestionSet({
    required this.version,
    required this.questions,
    this.batch = const Optional.absent(),
    this.threshold = const Optional.absent(),
    this.profile = const Optional.absent(),
  });
  factory QuestionSet.fromJson(Object? value) {
    final v = readObject(value);
    verify(
        "QuestionSet",
        v,
        ["version", "questions", "batch", "threshold", "profile"],
        ["version", "questions"]);
    return QuestionSet(
      version: readInt(v["version"], 1, 1),
      questions: readMap(
          v["questions"],
          (v) => readVariant<AnnotationSpec>(v, [
                (v) => DecideMember.fromJson(v),
                (v) => ChooseMember.fromJson(v),
                (v) => TagMember.fromJson(v),
                (v) => ScoreMember.fromJson(v)
              ])),
      batch: v.containsKey("batch")
          ? Optional.present(Batch.fromJson(v["batch"]))
          : const Optional.absent(),
      threshold: v.containsKey("threshold")
          ? Optional.present(Threshold.fromJson(v["threshold"]))
          : const Optional.absent(),
      profile: v.containsKey("profile")
          ? Optional.present(readString(v["profile"]))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "version": project(version),
        "questions": project(questions),
        if (batch.present) "batch": project(batch.value),
        if (threshold.present) "threshold": project(threshold.value),
        if (profile.present) "profile": project(profile.value),
      };
}
