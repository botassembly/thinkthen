// C outcome codes stay inside the binding. A not-sure answer is not a failure.
enum Outcome { no, yes, notSure }

enum ErrorKind { usage, backend, deadline, local, cancelled, defect }

// C and Dart report zero-based, end-exclusive Unicode scalar offsets, not UTF-16.
final class Entity {
  final String text, kind;
  final int start, end, length;
  final double strength;
  Entity(
    this.text,
    this.kind,
    this.start,
    this.end,
    this.length,
    this.strength,
  );
  factory Entity.parse(Object? value) {
    final map = _map(
        value,
        {
          'text',
          'start',
          'end',
          'length',
          'kind',
          'strength',
        },
        exact: true);
    final text = _string(map['text']);
    final kind = _string(map['kind']);
    final start = _nonnegative(map['start']);
    final end = _nonnegative(map['end']);
    final length = _nonnegative(map['length']);
    if (end < start || length != end - start)
      throw FormatException('entity scalar span');
    final strength = _number(map['strength']);
    if (strength < 0) throw FormatException('negative strength');
    return Entity(text, kind, start, end, length, strength);
  }
}

final class Edge {
  final String relation;
  final Map<String, Object?> source, target;
  final double probability;
  Edge(this.relation, this.source, this.target, this.probability);
  factory Edge.parse(Object? value) {
    final map = _map(
        value,
        {
          'relation',
          'source',
          'target',
          'probability',
        },
        exact: true);
    final source = _map(map['source'], {'name', 'kind'});
    final target = _map(map['target'], {'name', 'kind'});
    for (final endpoint in [source, target]) {
      _string(endpoint['name']);
      _string(endpoint['kind']);
    }
    final probability = _number(map['probability']);
    if (probability < 0 || probability > 1)
      throw FormatException('edge probability');
    return Edge(_string(map['relation']), source, target, probability);
  }
}

final class Recognition {
  final List<Entity> entities;
  final List<Map<String, Object?>> relations;
  Recognition(this.entities, this.relations);
  factory Recognition.parse(Object? value) {
    final map = _map(value, {'entities'}, allowed: {'entities', 'relations'});
    final entities = _list(map['entities']).map(Entity.parse).toList();
    final relations = _list(map['relations'] ?? <Object?>[]).map((v) {
      final relation = _map(
          v,
          {
            'relation',
            'source',
            'target',
            'probability',
          },
          exact: true);
      _string(relation['relation']);
      Entity.parse(relation['source']);
      Entity.parse(relation['target']);
      final probability = _number(relation['probability']);
      if (probability < 0 || probability > 1)
        throw FormatException('relation probability');
      return relation;
    }).toList();
    return Recognition(entities, relations);
  }
}

final class Relations {
  final List<Edge> edges;
  Relations(this.edges);
  factory Relations.parse(Object? value) {
    final map = _map(value, {'edges'}, exact: true);
    return Relations(_list(map['edges']).map(Edge.parse).toList());
  }
}

sealed class AnnotatedField {
  const AnnotatedField();
}

final class AnswerField extends AnnotatedField {
  final Object? value; // null is unresolved, not failed.
  const AnswerField(this.value);
}

final class FailedField extends AnnotatedField {
  final String kind, cause;
  const FailedField(this.kind, this.cause);
}

final class Annotation {
  final List<Map<String, AnnotatedField>> rows;
  Annotation(this.rows);
  factory Annotation.parse(Object? value) {
    return Annotation(
      _list(value).map((row) {
        final fields = _map(row, {});
        return fields.map((name, field) {
          if (field is Map) {
            final marker = _map(field, {'failed'}, exact: true);
            final failed = _map(
                marker['failed'],
                {
                  'kind',
                  'cause',
                },
                exact: true);
            if (failed['kind'] != 'backend' ||
                !const {
                  'missing_answer',
                  'wrong_kind',
                  'missing_probability',
                  'invalid_probability',
                  'invalid_distribution',
                  'unexpected_probability',
                }.contains(failed['cause']))
              throw FormatException('failure marker');
            return MapEntry(
              name,
              FailedField(_string(failed['kind']), _string(failed['cause'])),
            );
          }
          if (field is List) {
            if (!field.every((v) => v is String))
              throw FormatException('annotate labels');
          } else if (field != null &&
              field is! bool &&
              field is! String &&
              field is! num) {
            throw FormatException('annotate answer');
          }
          return MapEntry(name, AnswerField(field));
        });
      }).toList(),
    );
  }
}

Map<String, Object?> _map(
  Object? value,
  Set<String> required, {
  Set<String>? allowed,
  bool exact = false,
}) {
  if (value is! Map || value.keys.any((k) => k is! String))
    throw FormatException('object required');
  final map = Map<String, Object?>.from(value);
  if (!map.keys.toSet().containsAll(required))
    throw FormatException('missing keys');
  if ((exact || allowed != null) &&
      map.keys.any((k) => !(allowed ?? required).contains(k)))
    throw FormatException('extra keys');
  return map;
}

List<Object?> _list(Object? value) => value is List
    ? List<Object?>.from(value)
    : throw FormatException('list required');
String _string(Object? value) =>
    value is String ? value : throw FormatException('string required');
int _nonnegative(Object? value) => value is int && value >= 0
    ? value
    : throw FormatException('offset required');
double _number(Object? value) => value is num && value.isFinite
    ? value.toDouble()
    : throw FormatException('number required');
