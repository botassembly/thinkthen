import 'dart:convert';
import 'dart:io';
import 'package:thinkthen_dart/src/complete/models.dart';
import 'package:thinkthen_dart/src/complete/values.dart';
import 'package:thinkthen_dart/src/complete/decision.dart';
import 'package:thinkthen_dart/src/complete/call.dart';
import 'package:thinkthen_dart/src/complete/read.dart';
import 'package:thinkthen_dart/src/complete/requests.dart';

typedef Builder = Request Function(Carrier, Selection, [Controls?]);
final dartBuilders = <String, Builder>{
  'decide': Requests.decide,
  'choose': Requests.choose,
  'tag': Requests.tag,
  'score': Requests.score,
  'filter': Requests.filter,
  'rank': Requests.rank,
  'find': Requests.find,
  'annotate': Requests.annotate,
  'recognize': Requests.recognize,
  'relate': Requests.relate
};
void check(bool ok, String behavior) {
  if (!ok) throw StateError(behavior);
}

bool equalJson(Object? a, Object? b) {
  if (a is List && b is List)
    return a.length == b.length &&
        List.generate(a.length, (i) => equalJson(a[i], b[i])).every((v) => v);
  if (a is Map && b is Map)
    return a.length == b.length &&
        a.keys.every((k) => b.containsKey(k) && equalJson(a[k], b[k]));
  return a == b;
}

void rejects(void Function() read, String behavior) {
  try {
    read();
  } on FormatException catch (e) {
    check(!e.toString().contains('secret-sentinel'),
        'failure diagnostic is redacted');
    return;
  } on ArgumentError {
    return;
  }
  throw StateError(behavior);
}

Object readType(String type, Object? json) => switch (type) {
      'DecideResult' => DecideResult.fromJson(json),
      'ChooseResult' => ChooseResult.fromJson(json),
      'TagResult' => TagResult.fromJson(json),
      'ScoreResult' => ScoreResult.fromJson(json),
      'FilterResult' => FilterResult.fromJson(json),
      'RankResult' => RankResult.fromJson(json),
      'FindResult' => FindResult.fromJson(json),
      'AnnotateResult' => AnnotateResult.fromJson(json),
      'RecognizeResult' => RecognizeResult.fromJson(json),
      'RelateResult' => RelateResult.fromJson(json),
      'Meta' => Meta.fromJson(json),
      'Facts' => Facts.fromJson(json),
      'Entity' => Entity.fromJson(json),
      'AnnotationSuccess' => AnnotationSuccess.fromJson(json),
      'AnnotationFailure' => AnnotationFailure.fromJson(json),
      'ChooseSpec' => ChooseSpec.fromJson(json),
      'Controls' => Controls.fromJson(json),
      _ => throw StateError('unknown test type'),
    };
Carrier readQuestion(String type, Object? json) {
  switch (type) {
    case 'DecideSpec':
      return DecideSpec.fromJson(json);
    case 'ChooseSpec':
      return ChooseSpec.fromJson(json);
    case 'TagSpec':
      return TagSpec.fromJson(json);
    case 'ScoreSpec':
      return ScoreSpec.fromJson(json);
    case 'FindSpec':
      return FindSpec.fromJson(json);
    case 'QuestionSet':
      return QuestionSet.fromJson(json);
    case 'RecognitionSpec':
      return RecognitionSpec.fromJson(json);
    case 'RelationSpec':
      return RelationSpec.fromJson(json);
    default:
      throw StateError('unknown test question');
  }
}

void runCarrierCases(String fixture, Map<String, Builder> builders) {
  final f = readObject(jsonDecode(File(fixture).readAsStringSync())),
      results = readObject(f['results']);
  for (final e in results.entries) {
    final name = '${e.key[0].toUpperCase()}${e.key.substring(1)}Result';
    final r = readType(name, e.value);
    check(equalJson(project(r), e.value),
        'all known fields round trip: ${e.key}');
  }
  final decide = DecideResult.fromJson(results['decide']);
  final BooleanDecision decision = decide.value as BooleanDecision;
  final AnswerId answerId = decide.answer_id;
  check(
      decision.value == false &&
          answerId.value == 'a' * 64 &&
          decide.input.present &&
          decide.input.value == null,
      'false, ID and present null');
  final authored = DecideResult.fromJson(f['authored_decision']);
  check(
      authored.value is AuthoredDecision &&
          ((authored.value as AuthoredDecision).meaning.value
                  as Map)['accepted'] ==
              false,
      'structured authored value');
  final authoredNull = readObject(f['authored_decision']);
  authoredNull['value'] = null;
  rejects(() => DecideResult.fromJson(authoredNull),
      'authored null requires actual C discriminator');
  check(AuthoredDecision(JsonContent.fromJson(null)).meaning.value == null,
      'native adapter preserves authored null');
  check(decide.threshold.low == 0.1 && decide.position.value.last.value == 4,
      'band and physical line window');
  final choose = ChooseResult.fromJson(results['choose']);
  final Map<String, double> probabilities = choose.answer.probabilities;
  check(
      choose.value == null &&
          probabilities.keys.first == '0' &&
          probabilities['0'] == 0.9 &&
          choose.answer.confidence.value == 0.8,
      'typed full probabilities and confidence');
  check(!FindResult.fromJson(results['find']).answer.confidence.present,
      'omitted confidence');
  final annotation = AnnotateResult.fromJson(results['annotate']);
  final Map<String, AnnotationEntry> members = annotation.answers;
  check(
      annotation.value['0'] is BooleanValue &&
          annotation.value['unresolved'] is Unresolved,
      'typed actionable annotation values');
  check(
      members['0'] is AnnotationSuccess &&
          (members['fail'] as AnnotationFailure).failure.cause ==
              CauseKind.missing_probability,
      'typed member failure');
  final recognition = RecognizeResult.fromJson(results['recognize']);
  final Entity entity = recognition.value.entities.first;
  final Span span = recognition.answer.pairs.first.source;
  check(entity.start == 1 && entity.length == 2 && span.end == 3,
      'native Unicode scalar spans');
  check(
      recognition.answer.names.first.kinds == null &&
          recognition.answer.names.first.edges!.isEmpty,
      'null map and empty map');
  final relations = RelateResult.fromJson(results['relate']);
  final Edge edge = relations.value.first;
  check(
      (edge.source.record.value as Map)['ok'] == false &&
          edge.target.first_line.value == 2,
      'located endpoint original payload');
  final RelationSuccess relation =
      relations.answer.questions.first as RelationSuccess;
  check(relation.target == null && !relation.accepted,
      'nullable relation target and false acceptance');
  final facts = Facts.fromJson(f['facts']);
  final CallId callId = facts.call_id;
  check(
      callId.value == 'a' * 64 &&
          facts.estimated_cost_usd.value == '12345678901234567890.000001' &&
          facts.command_ms.value == 0 &&
          !facts.input_tokens.present,
      'exact cost, zero timing and absent usage');
  final attempt = Attempt.fromJson(f['attempt']);
  final SdkRequestId requestId = attempt.sdk_request_id;
  check(requestId.value == 'b' * 64 && attempt.server_ms.value == 0,
      'typed attempt ID and reported zero');
  for (final e in f['errors'] as List) {
    check(CallError.fromJson(e).facts.value.call_id.value == 'a' * 64,
        'six started failures retain facts');
  }
  for (final e in f['negative'] as List) {
    final bad = readObject(e);
    rejects(() => readType(bad['type'] as String, bad['value']),
        'strict known fields');
  }
  for (final raw in f['requests'] as List) {
    final c = readObject(raw), verb = c['verb'] as String;
    final q = readQuestion(c['question_type'] as String, c['question']);
    final input = c['input_type'] == 'CandidateInput'
        ? CandidateInput.fromJson(c['input'])
        : RecordInput.fromJson(c['input']);
    final request = builders[verb]!(
        q, input as Selection, Controls.fromJson(c['controls']));
    check(equalJson(request.toJson(), c['expected']),
        'independent named builder expectation: $verb');
  }
  final defaults = RecognitionSpec.fromJson(
      jsonDecode('{"version":1,"recognize":{"relations":[{"name":"knows"}]}}'));
  final defaultRequest = builders['recognize']!(
      defaults, TextInput(text: Authored.fromJson('A knows B')));
  check(
      !(defaultRequest.question as RecognitionSpec)
          .recognize
          .relations
          .value
          .first
          .source
          .present,
      'native any-kind sides stay absent');
  check(Controls.fromJson({'deadline_ms': -1}).deadline_ms.value == -1,
      'explicit unlimited deadline');
  final imageView =
      ImageView(MediaType.png, [0, 255], 1, 1, const Optional.absent());
  check(imageView.bytes.last == 255 && !imageView.filename.present,
      'copied image view binary bytes and absent filename');
  final request = builders['decide']!(
      const QuestionFile(path: '/explicit/questions.json'),
      const Files(
          paths: ['same.txt', 'same.txt'],
          unit: UnitKind.window,
          window: Optional.present(2)));
  check(
      (request.input as Files).paths.length == 2, 'duplicate file occurrences');
  try {
    request.questionJson();
    throw StateError('host loaded a question file');
  } on StateError catch (e) {
    check(e.message == 'question files require the native loader',
        'explicit native question loader');
  }
  final images = ImageInput(images: [
    const ImageBytes(media: MediaType.png, data: [255, 0]),
    const ImageBytes(media: MediaType.png, data: [1]),
    const ImageBytes(media: MediaType.png, data: [255, 0])
  ]);
  for (final verb in ['decide', 'choose', 'score']) {
    final c = (f['requests'] as List)
            .map(readObject)
            .firstWhere((c) => c['verb'] == verb),
        q = readQuestion(c['question_type'] as String, c['question']);
    check(
        (builders[verb]!(q, images).input as ImageInput)
                .images
                .last
                .data
                .first ==
            255,
        'image order and duplicate bytes');
  }
  for (final verb in [
    'tag',
    'filter',
    'rank',
    'find',
    'annotate',
    'recognize',
    'relate'
  ]) {
    final c = (f['requests'] as List)
            .map(readObject)
            .firstWhere((c) => c['verb'] == verb),
        q = readQuestion(c['question_type'] as String, c['question']);
    rejects(() => builders[verb]!(q, images),
        'seven explicit image builder refusals');
    rejects(
        () => builders[verb]!(
            q,
            const Files(
                paths: ['a.png'],
                unit: UnitKind.file,
                media: Optional.present(MediaKind.image))),
        'seven explicit image-source builder refusals');
  }
  final max = readObject(jsonDecode(
      '{"call_id":"${'a' * 64}","records":9223372036854775807,"requests_sent":0,"cache_answers":0,"seconds":0}'));
  check(Facts.fromJson(max).records == maxInteger, 'signed 64-bit boundary');
  final overflow = jsonDecode(jsonEncode(max)
      .replaceFirst('9223372036854775807', '9223372036854775808'));
  rejects(() => Facts.fromJson(overflow),
      'no count truncation above signed 64-bit');
  final call = CompleteCall<DecideResult>.fromJson(
      {'value': results['decide'], 'facts': f['facts'], 'attempts': []},
      DecideResult.fromJson);
  check(
      (call.value.value as BooleanDecision).value == false &&
          call.attempts.present &&
          call.attempts.value.isEmpty,
      'typed envelope and requested zero attempts');
  check(
      !request.toString().contains('explicit/questions') &&
          !facts.toString().contains('12345678901234567890'),
      'carrier diagnostics withhold content');
}

void main(List<String> args) {
  runCarrierCases(args.single, dartBuilders);
  print(
      'Dart private carrier and builder consumers PASS; native complete parity pending');
}
