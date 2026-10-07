import 'values.dart';

Authored readText(Object? v) => v != null ? Authored.fromJson(v) : invalid();

const maxInteger = 9223372036854775807;
Never invalid() => throw const FormatException('invalid typed carrier');
Map<String, Object?> readObject(Object? v) {
  if (v is! Map || v.keys.any((key) => key is! String)) invalid();
  return Map<String, Object?>.from(v);
}

String readString(Object? v) => v is String ? v : invalid();
bool readBool(Object? v) => v is bool ? v : invalid();
Null readNull(Object? v) => v == null ? null : invalid();
T readLiteral<T>(Object? v, T expected) =>
    v == expected && v is T ? expected : invalid();
int readInt(Object? v, int low, int high) =>
    v is int && v >= low && v <= high ? v : invalid();
double readNumber(Object? v, {bool probability = false}) =>
    v is num && v.isFinite && (!probability || (v >= 0 && v <= 1))
        ? v.toDouble()
        : invalid();
String readCost(Object? v) =>
    v is String && RegExp(r'^[0-9]+\.[0-9]{6}$').stringMatch(v) == v
        ? v
        : invalid();
T readEnum<T extends Enum>(Object? v, List<T> choices) {
  for (final c in choices) {
    if (c.name == v) return c;
  }
  return invalid();
}

List<T> readList<T>(Object? v, T Function(Object?) reader) =>
    v is List ? List<T>.unmodifiable(v.map(reader)) : invalid();
Map<String, T> readMap<T>(Object? v, T Function(Object?) reader) =>
    Map<String, T>.unmodifiable(
        readObject(v).map((k, v) => MapEntry(k, reader(v))));
List<int> readBytes(Object? v) => readList(v, (b) => readInt(b, 0, 255));
T readVariant<T>(Object? v, List<T Function(Object?)> readers) {
  for (final read in readers) {
    try {
      return read(v);
    } on FormatException {/* Try the next closed variant. */}
  }
  return invalid();
}

Object? readJson(Object? v) {
  if (v == null || v is String || v is bool || v is int) return v;
  if (v is double && v.isFinite) return v;
  if (v is List) return readList(v, readJson);
  if (v is Map) return readMap(v, readJson);
  return invalid();
}

Object? project(Object? v) {
  if (v is Carrier) return project(v.toJson());
  if (v is Enum) return v.name;
  if (v is List) return v.map(project).toList();
  if (v is Map) return v.map((k, v) => MapEntry(k, project(v)));
  return v;
}

void verify(String type, Map<String, Object?> v, List<String> fields,
    List<String> required) {
  if (required.any((k) => !v.containsKey(k))) invalid();
  final input = type.endsWith('Spec') ||
      type.endsWith('Member') ||
      const [
        'Controls',
        'QuestionSet',
        'QuestionFile',
        'Files',
        'ImageInput',
        'ImageBytes',
        'TextInput',
        'RecordInput',
        'CandidateInput',
        'RecognitionPlan',
        'RelationPlan',
        'PlanRule'
      ].contains(type);
  if (input && v.keys.any((k) => !fields.contains(k))) invalid();
  if (v.containsKey('proxy')) invalid();
  if (const ['Observed', 'AnnotationSuccess', 'RelationSuccess']
          .contains(type) &&
      v.containsKey('failure_id')) invalid();
  if (const ['FailedObservation', 'AnnotationFailure', 'RelationFailure']
          .contains(type) &&
      v.containsKey('answer_id')) invalid();
  if (type == 'FailedObservation' && v.containsKey('observation_id')) invalid();
  if (type == 'Meta') verifyMeta(v);
  if (const ['Entity', 'Span', 'PieceOdds', 'NameOdds'].contains(type)) {
    final start = readInt(v['start'], 0, maxInteger),
        end = readInt(v['end'], 0, maxInteger);
    if (end < start || (type == 'Entity' && v['length'] != end - start))
      invalid();
  }
  for (final pair in [
    ['first', 'last'],
    ['first_line', 'last_line']
  ]) {
    if (v.containsKey(pair[0]) != v.containsKey(pair[1])) invalid();
    if (v.containsKey(pair[0]) &&
        (!v.containsKey('file') ||
            readInt(v[pair[1]], 1, maxInteger) <
                readInt(v[pair[0]], 1, maxInteger))) invalid();
  }
  if (const [
    'ChooseSpec',
    'TagSpec',
    'ChooseMember',
    'TagMember',
    'RecognitionSpec',
    'RelationSpec',
    'TagResult',
    'FilterResult',
    'RecognizeQuestion',
    'RelateQuestion'
  ].contains(type)) {
    for (final key in ['threshold', 'relation_threshold']) {
      if (v.containsKey(key)) {
        final cut = readNumber(v[key], probability: true);
        if (cut <= 0) invalid();
      }
    }
  }
  if (type == 'ChooseResult' &&
      v['threshold'] != null &&
      v['threshold'] is! num) invalid();
  if (type == 'DecideResult' && v['threshold'] == null) invalid();
  if (type == 'RankResult') {
    final a = readObject(v['answer']), q = readObject(v['question']);
    if (!['decide', 'score'].contains(q['verb']) ||
        !['yes_no', 'score'].contains(a['kind']) ||
        (a['kind'] == 'score') != (q['verb'] == 'score')) invalid();
  }
  if (type.endsWith('Result') &&
      ((type == 'AnnotateResult') !=
          readObject(v['meta']).containsKey('questions_sha256'))) invalid();
  if (type == 'Facts' && readNumber(v['seconds']) < 0) invalid();
}

void verifyMeta(Map<String, Object?> v) {
  final requests = readList(v['requests'], readString),
      sources = readList(v['question_sources'], readObject),
      observations = readList(v['observations'], readObject);
  if (requests.length != sources.length ||
      requests.length != observations.length) invalid();
  if (v.containsKey('question_sha256') == v.containsKey('questions_sha256'))
    invalid();
  if (sources.any((s) =>
      !['live', 'cache', 'replay'].contains(s['origin']) ||
      s['answered_by'] is! String)) invalid();
  if (v['failed_questions'] !=
      observations.where((o) => o.containsKey('failure_id')).length) invalid();
  if (sources.isEmpty) {
    if (v['origin'] != null ||
        v['cached'] != false ||
        v['requests_sent'] != 0 ||
        v.containsKey('answered_by')) invalid();
  } else {
    final origins = sources.map((s) => s['origin']);
    final origin = origins.contains('live')
        ? 'live'
        : origins.contains('replay')
            ? 'replay'
            : 'cache';
    if (v['origin'] != origin || v['cached'] != (origin != 'live')) invalid();
    final names = sources.map((s) => s['answered_by']).toSet();
    if (names.length == 1
        ? v['answered_by'] != names.single
        : v.containsKey('answered_by')) invalid();
  }
}
