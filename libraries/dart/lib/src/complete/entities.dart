part of 'models.dart';

final class Entity extends Carrier {
  final String text;
  final int start;
  final int end;
  final int length;
  final String kind;
  final double strength;
  final Optional<String> file;
  final Optional<int> first_line;
  final Optional<int> last_line;
  const Entity({
    required this.text,
    required this.start,
    required this.end,
    required this.length,
    required this.kind,
    required this.strength,
    this.file = const Optional.absent(),
    this.first_line = const Optional.absent(),
    this.last_line = const Optional.absent(),
  });
  factory Entity.fromJson(Object? value) {
    final v = readObject(value);
    verify("Entity", v, [
      "text",
      "start",
      "end",
      "length",
      "kind",
      "strength",
      "file",
      "first_line",
      "last_line"
    ], [
      "text",
      "start",
      "end",
      "length",
      "kind",
      "strength"
    ]);
    return Entity(
      text: readString(v["text"]),
      start: readInt(v["start"], 0, maxInteger),
      end: readInt(v["end"], 0, maxInteger),
      length: readInt(v["length"], 0, maxInteger),
      kind: readString(v["kind"]),
      strength: readNumber(v["strength"], probability: true),
      file: v.containsKey("file")
          ? Optional.present(readString(v["file"]))
          : const Optional.absent(),
      first_line: v.containsKey("first_line")
          ? Optional.present(readInt(v["first_line"], 1, maxInteger))
          : const Optional.absent(),
      last_line: v.containsKey("last_line")
          ? Optional.present(readInt(v["last_line"], 1, maxInteger))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "text": project(text),
        "start": project(start),
        "end": project(end),
        "length": project(length),
        "kind": project(kind),
        "strength": project(strength),
        if (file.present) "file": project(file.value),
        if (first_line.present) "first_line": project(first_line.value),
        if (last_line.present) "last_line": project(last_line.value),
      };
}

final class Endpoint extends Carrier {
  final String name;
  final String kind;
  final Optional<Object?> record;
  final Optional<String> file;
  final Optional<int> first_line;
  final Optional<int> last_line;
  const Endpoint({
    required this.name,
    required this.kind,
    this.record = const Optional.absent(),
    this.file = const Optional.absent(),
    this.first_line = const Optional.absent(),
    this.last_line = const Optional.absent(),
  });
  factory Endpoint.fromJson(Object? value) {
    final v = readObject(value);
    verify(
        "Endpoint",
        v,
        ["name", "kind", "record", "file", "first_line", "last_line"],
        ["name", "kind"]);
    return Endpoint(
      name: readString(v["name"]),
      kind: readString(v["kind"]),
      record: v.containsKey("record")
          ? Optional.present(readJson(v["record"]))
          : const Optional.absent(),
      file: v.containsKey("file")
          ? Optional.present(readString(v["file"]))
          : const Optional.absent(),
      first_line: v.containsKey("first_line")
          ? Optional.present(readInt(v["first_line"], 1, maxInteger))
          : const Optional.absent(),
      last_line: v.containsKey("last_line")
          ? Optional.present(readInt(v["last_line"], 1, maxInteger))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "name": project(name),
        "kind": project(kind),
        if (record.present) "record": project(record.value),
        if (file.present) "file": project(file.value),
        if (first_line.present) "first_line": project(first_line.value),
        if (last_line.present) "last_line": project(last_line.value),
      };
}

final class Edge extends Carrier {
  final String relation;
  final Endpoint source;
  final Endpoint target;
  final double probability;
  final Optional<bool> either;
  const Edge({
    required this.relation,
    required this.source,
    required this.target,
    required this.probability,
    this.either = const Optional.absent(),
  });
  factory Edge.fromJson(Object? value) {
    final v = readObject(value);
    verify("Edge", v, ["relation", "source", "target", "probability", "either"],
        ["relation", "source", "target", "probability"]);
    return Edge(
      relation: readString(v["relation"]),
      source: Endpoint.fromJson(v["source"]),
      target: Endpoint.fromJson(v["target"]),
      probability: readNumber(v["probability"], probability: true),
      either: v.containsKey("either")
          ? Optional.present(readLiteral(v["either"], true))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "relation": project(relation),
        "source": project(source),
        "target": project(target),
        "probability": project(probability),
        if (either.present) "either": project(either.value),
      };
}

final class EntityEdge extends Carrier {
  final String relation;
  final Entity source;
  final Entity target;
  final double probability;
  final Optional<bool> either;
  const EntityEdge({
    required this.relation,
    required this.source,
    required this.target,
    required this.probability,
    this.either = const Optional.absent(),
  });
  factory EntityEdge.fromJson(Object? value) {
    final v = readObject(value);
    verify(
        "EntityEdge",
        v,
        ["relation", "source", "target", "probability", "either"],
        ["relation", "source", "target", "probability"]);
    return EntityEdge(
      relation: readString(v["relation"]),
      source: Entity.fromJson(v["source"]),
      target: Entity.fromJson(v["target"]),
      probability: readNumber(v["probability"], probability: true),
      either: v.containsKey("either")
          ? Optional.present(readLiteral(v["either"], true))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "relation": project(relation),
        "source": project(source),
        "target": project(target),
        "probability": project(probability),
        if (either.present) "either": project(either.value),
      };
}

final class Recognition extends Carrier {
  final List<Entity> entities;
  final Optional<List<EntityEdge>> relations;
  const Recognition({
    required this.entities,
    this.relations = const Optional.absent(),
  });
  factory Recognition.fromJson(Object? value) {
    final v = readObject(value);
    verify("Recognition", v, ["entities", "relations"], ["entities"]);
    return Recognition(
      entities: readList(v["entities"], (v) => Entity.fromJson(v)),
      relations: v.containsKey("relations")
          ? Optional.present(
              readList(v["relations"], (v) => EntityEdge.fromJson(v)))
          : const Optional.absent(),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "entities": project(entities),
        if (relations.present) "relations": project(relations.value),
      };
}

final class PieceOdds extends Carrier {
  final int start;
  final int end;
  final Map<String, double> tags;
  const PieceOdds({
    required this.start,
    required this.end,
    required this.tags,
  });
  factory PieceOdds.fromJson(Object? value) {
    final v = readObject(value);
    verify("PieceOdds", v, ["start", "end", "tags"], ["start", "end", "tags"]);
    return PieceOdds(
      start: readInt(v["start"], 0, maxInteger),
      end: readInt(v["end"], 0, maxInteger),
      tags: readMap(v["tags"], (v) => readNumber(v, probability: true)),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "start": project(start),
        "end": project(end),
        "tags": project(tags),
      };
}

final class NameOdds extends Carrier {
  final int start;
  final int end;
  final Map<String, double>? kinds;
  final Map<String, double>? edges;
  const NameOdds({
    required this.start,
    required this.end,
    required this.kinds,
    required this.edges,
  });
  factory NameOdds.fromJson(Object? value) {
    final v = readObject(value);
    verify("NameOdds", v, ["start", "end", "kinds", "edges"],
        ["start", "end", "kinds", "edges"]);
    return NameOdds(
      start: readInt(v["start"], 0, maxInteger),
      end: readInt(v["end"], 0, maxInteger),
      kinds: (v["kinds"] == null
          ? null
          : readMap(v["kinds"], (v) => readNumber(v, probability: true))),
      edges: (v["edges"] == null
          ? null
          : readMap(v["edges"], (v) => readNumber(v, probability: true))),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "start": project(start),
        "end": project(end),
        "kinds": project(kinds),
        "edges": project(edges),
      };
}

final class Span extends Carrier {
  final int start;
  final int end;
  const Span({
    required this.start,
    required this.end,
  });
  factory Span.fromJson(Object? value) {
    final v = readObject(value);
    verify("Span", v, ["start", "end"], ["start", "end"]);
    return Span(
      start: readInt(v["start"], 0, maxInteger),
      end: readInt(v["end"], 0, maxInteger),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "start": project(start),
        "end": project(end),
      };
}

final class PairOdds extends Carrier {
  final String relation;
  final Span source;
  final Span target;
  final double probability;
  const PairOdds({
    required this.relation,
    required this.source,
    required this.target,
    required this.probability,
  });
  factory PairOdds.fromJson(Object? value) {
    final v = readObject(value);
    verify("PairOdds", v, ["relation", "source", "target", "probability"],
        ["relation", "source", "target", "probability"]);
    return PairOdds(
      relation: readString(v["relation"]),
      source: Span.fromJson(v["source"]),
      target: Span.fromJson(v["target"]),
      probability: readNumber(v["probability"], probability: true),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "relation": project(relation),
        "source": project(source),
        "target": project(target),
        "probability": project(probability),
      };
}

final class RecognitionAnswer extends Carrier {
  final List<PieceOdds> pieces;
  final List<NameOdds> names;
  final List<PairOdds> pairs;
  const RecognitionAnswer({
    required this.pieces,
    required this.names,
    required this.pairs,
  });
  factory RecognitionAnswer.fromJson(Object? value) {
    final v = readObject(value);
    verify("RecognitionAnswer", v, ["pieces", "names", "pairs"],
        ["pieces", "names", "pairs"]);
    return RecognitionAnswer(
      pieces: readList(v["pieces"], (v) => PieceOdds.fromJson(v)),
      names: readList(v["names"], (v) => NameOdds.fromJson(v)),
      pairs: readList(v["pairs"], (v) => PairOdds.fromJson(v)),
    );
  }
  @override
  Map<String, Object?> toJson() => {
        "pieces": project(pieces),
        "names": project(names),
        "pairs": project(pairs),
      };
}
