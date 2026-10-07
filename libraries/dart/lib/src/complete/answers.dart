part of 'models.dart';

final class YesNo extends Carrier implements AtomicAnswer {
  final String kind;
  final double probability;
  const YesNo({
    required this.kind,
    required this.probability,
  });
  factory YesNo.fromJson(Object? value) {
    final v = readObject(value);
    verify("YesNo", v, ["kind", "probability"], ["kind", "probability"]);
    return YesNo(
      kind: readLiteral(v["kind"], "yes_no"),
      probability: readNumber(v["probability"], probability: true),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "kind": project(kind),
        "probability": project(probability),
      };
}

final class Choice extends Carrier implements AtomicAnswer {
  final String kind;
  final String pick;
  final Map<String, double> probabilities;
  final Optional<double> confidence;
  const Choice({
    required this.kind,
    required this.pick,
    required this.probabilities,
    this.confidence = const Optional.absent(),
  });
  factory Choice.fromJson(Object? value) {
    final v = readObject(value);
    verify("Choice", v, ["kind", "pick", "probabilities", "confidence"],
        ["kind", "pick", "probabilities"]);
    return Choice(
      kind: readLiteral(v["kind"], "choice"),
      pick: readString(v["pick"]),
      probabilities:
          readMap(v["probabilities"], (v) => readNumber(v, probability: true)),
      confidence: v.containsKey("confidence")
          ? Optional.present(readNumber(v["confidence"], probability: true))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "kind": project(kind),
        "pick": project(pick),
        "probabilities": project(probabilities),
        if (confidence.present) "confidence": project(confidence.value),
      };
}

final class Tags extends Carrier implements AtomicAnswer {
  final String kind;
  final Map<String, double> probabilities;
  const Tags({
    required this.kind,
    required this.probabilities,
  });
  factory Tags.fromJson(Object? value) {
    final v = readObject(value);
    verify("Tags", v, ["kind", "probabilities"], ["kind", "probabilities"]);
    return Tags(
      kind: readLiteral(v["kind"], "tag"),
      probabilities:
          readMap(v["probabilities"], (v) => readNumber(v, probability: true)),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "kind": project(kind),
        "probabilities": project(probabilities),
      };
}

final class Score extends Carrier implements AtomicAnswer {
  final String kind;
  final String level;
  final Map<String, double> probabilities;
  final Optional<double> confidence;
  const Score({
    required this.kind,
    required this.level,
    required this.probabilities,
    this.confidence = const Optional.absent(),
  });
  factory Score.fromJson(Object? value) {
    final v = readObject(value);
    verify("Score", v, ["kind", "level", "probabilities", "confidence"],
        ["kind", "level", "probabilities"]);
    return Score(
      kind: readLiteral(v["kind"], "score"),
      level: readString(v["level"]),
      probabilities:
          readMap(v["probabilities"], (v) => readNumber(v, probability: true)),
      confidence: v.containsKey("confidence")
          ? Optional.present(readNumber(v["confidence"], probability: true))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "kind": project(kind),
        "level": project(level),
        "probabilities": project(probabilities),
        if (confidence.present) "confidence": project(confidence.value),
      };
}

final class FindAnswer extends Carrier implements AtomicAnswer {
  final String kind;
  final String pick;
  final Map<String, double> probabilities;
  final Optional<double> confidence;
  const FindAnswer({
    required this.kind,
    required this.pick,
    required this.probabilities,
    this.confidence = const Optional.absent(),
  });
  factory FindAnswer.fromJson(Object? value) {
    final v = readObject(value);
    verify("FindAnswer", v, ["kind", "pick", "probabilities", "confidence"],
        ["kind", "pick", "probabilities"]);
    return FindAnswer(
      kind: readLiteral(v["kind"], "find"),
      pick: readString(v["pick"]),
      probabilities:
          readMap(v["probabilities"], (v) => readNumber(v, probability: true)),
      confidence: v.containsKey("confidence")
          ? Optional.present(readNumber(v["confidence"], probability: true))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "kind": project(kind),
        "pick": project(pick),
        "probabilities": project(probabilities),
        if (confidence.present) "confidence": project(confidence.value),
      };
}
