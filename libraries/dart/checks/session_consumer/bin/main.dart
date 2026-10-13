import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:thinkthen_dart/thinkthen_dart.dart';
import 'stream_cases.dart';
import 'data_cases.dart';
import 'fixture_values.dart';
import 'shipped_example.dart' as shipped;

void check(bool value, String message) {
  if (!value) throw StateError(message);
}

Future<void> main(List<String> args) async {
  if (args.contains('smoke')) {
    final engine = Engine.open();
    try {
      final call = await engine.decide(
        QuestionText(
          text: Platform.environment['THINKTHEN_TEST_SMOKE_QUESTION']!,
        ),
        InputText(
          text: Platform.environment['THINKTHEN_TEST_SMOKE_TEXT']!,
        ),
      );
      final answer = call.packets
          .whereType<SessionPacketDecideRow>()
          .single
          .value
          .value
          .value;
      check(answer is bool, 'smoke answer is not boolean');
      print('\nsmoke: $answer');
    } finally {
      engine.close();
    }
    return;
  }
  final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
  var sends = 0;
  final arrived = Completer<void>(), release = Completer<void>();
  final prefixAnswered = Completer<void>();
  final serving = server.listen((request) async {
    sends++;
    check(request.headers.value('user-agent') == 'thinkthen/0.2.0 (dart)',
        'fixed Dart surface');
    final body = jsonDecode(await utf8.decoder.bind(request).join()) as Map;
    if (jsonEncode(body).contains('held')) {
      arrived.complete();
      await release.future;
    }
    final answers = <String, Object?>{};
    for (final entry in (body['questions'] as Map).entries) {
      final question = entry.value as Map;
      final kind = question['type'];
      if (kind == 'noul')
        answers[entry.key] = {
          'type': kind,
          'noul': jsonEncode(body).contains('uncertain') ? 0.5 : 0.9
        };
      else {
        final keys = kind == 'score'
            ? List.generate((question['criteria'] as List).length, (i) => '$i')
            : (question['criteria'] as Map).keys.toList();
        answers[entry.key] = {
          'type': kind,
          'probabilities': {
            for (var i = 0; i < keys.length; i++)
              keys[i]: i == 0 ? 0.9 : 0.1 / (keys.length - 1)
          }
        };
      }
    }
    request.response.headers.contentType = ContentType.json;
    request.response.write(jsonEncode({
      'model': body['model'],
      'answers': answers,
      'usage': {'input_tokens': 1, 'output_tokens': 1}
    }));
    await request.response.close();
    if (jsonEncode(body).contains('reader-prefix')) prefixAnswered.complete();
  });
  final engine = Engine.open(
      settings: EngineSettings(
          backend: const Presence.present('typesafe'),
          baseUrl: Presence.present('http://127.0.0.1:${server.port}'),
          cache: const Presence.present(CacheDocument.boolean(false)),
          maxRetries: Presence.present(BigInt.zero),
          maxRequests: const Presence.present(null),
          maxRequestsTotal: const Presence.present(null),
          maxEstimatedInputTokensTotal: const Presence.present(null)));
  final text = InputText(text: 'owned');
  final question = QuestionText(text: 'Is it?');
  try {
    if (args.contains('examples')) { await shipped.showAnswers(engine); print('PASS: ten shipped examples'); return; }
    if (args.contains('stream-regressions') ||
        args.contains('cancel-regression')) {
      await streamCases(engine, arrived.future, release, prefixAnswered.future,
          cancelOnly: args.contains('cancel-regression'));
      return;
    }
    if (args.contains('usage-disabled')) {
      final status = engine.finishUsageStatus();
      check(
          status.state == UsagePersistenceState.disabled &&
              status.advice == null,
          'disabled storage returns owned state');
      engine.close();
      check(status.state == UsagePersistenceState.disabled && sends == 0,
          'disabled inspection sends nothing and survives close');
      try {
        engine.usagePersistence();
        throw StateError('closed accepted');
      } on StateError catch (error) {
        check(error.message == 'Native owner is closed', 'closed guard');
      }
      print('PASS: disabled usage and closed guard');
      return;
    }
    final call = await engine.decide(question, text);
    check(!call.terminal.failure.isPresent && call.terminal.facts.isPresent,
        'typed terminal facts');
    final row = call.packets.whereType<SessionPacketDecideRow>().single;
    check(row.value.answer is AnswerYesNo, 'typed decide answer');
    if (args.any((arg) => arg.startsWith('usage-'))) {
      final held = args.contains('usage-failed');
      if (held)
        check(engine.usagePersistence().state == UsagePersistenceState.pending,
            'held writer remains pending');
      try {
        await engine.decide(question, text,
            options: Options(
                maxRequestsTotal: Presence.present(BigInt.zero)));
        throw StateError('budget accepted');
      } on CallFailure catch (error) {
        check(error.call!.terminal.facts.isPresent, 'failure retains facts');
      }
      final status = engine.finishUsageStatus();
      check(
          status.state ==
              (held
                  ? UsagePersistenceState.failed
                  : UsagePersistenceState.written),
          'finalization returns actual writer state after judgment failure');
      check(
          status.advice ==
              (held
                  ? 'check the usage folder permissions and free space'
                  : null),
          'native advice is safe');
      check(engine.usagePersistence().state == status.state && sends == 1,
          'latched state and no additional sends');
      engine.close();
      check(
          status.state ==
                  (held
                      ? UsagePersistenceState.failed
                      : UsagePersistenceState.written) &&
              call.terminal.facts.value != null &&
              row.value.answer is AnswerYesNo,
          'owned status facts and successful answer survive cleanup');
      for (final method in [
        engine.usagePersistence,
        engine.finishUsageStatus
      ]) {
        try {
          method();
          throw StateError('closed accepted');
        } on StateError catch (error) {
          check(error.message == 'Native owner is closed', 'closed guard');
        }
      }
      print(
          'PASS: usage state, copied advice, facts, one send and closed guards');
      return;
    }
    final wide = BigInt.parse('18446744073709551615');
    final authored = DefinedQuestion(
        value: DecideQuestion(
            decide: Wording.text('Is it?'),
            trueValue: const Presence.present(null),
            falseValue: const Presence.present(
                Reading.text('No.'))));
    final present = await engine.decide(
        authored,
        InputJson(
            value: {'wide': wide, 'false': false, 'null': null}));
    final value =
        present.packets.whereType<SessionPacketDecideRow>().single.value;
    final saved = value.question as ReadableQuestionDecide;
    check(
        saved.trueValue.isPresent &&
            saved.trueValue.value == null &&
            saved.falseValue.value == 'No.',
        'presence null and false');
    check((value.input.value as Map)['false'] == false,
        'false evidence retained');
    check((value.input.value as Map)['wide'] == wide,
        'wide original integer survives native round trip');
    final extended = DecisionResult.read({
      ...fixtureValue(value) as Map<String, Object?>,
      'future': {'wide': wide}
    });
    check((extended.extensions['future'] as Map)['wide'] == wide,
        'unknown output retained');
    final absent =
        ReadableQuestionDecide.read({'verb': 'decide', 'text': 'Is it?'});
    check(!absent.trueValue.isPresent, 'absent reading differs from null');
    final unresolved = await engine.decide(
        question, InputText(text: 'uncertain'),
        options: Options(
            threshold: const Presence.present(
                Threshold.text('0.2:0.8'))));
    check(
        unresolved.packets
                .whereType<SessionPacketDecideRow>()
                .single
                .value
                .value
                .value ==
            null,
        'unresolved value stays null');
    check(unresolved.terminal.failure.value == null,
        'unresolved answer is not failure');
    final beforeBudget = sends;
    try {
      await engine.decide(question, text,
          options: Options(
              maxRequestsTotal: Presence.present(BigInt.zero)));
      throw StateError('budget succeeded');
    } on CallFailure catch (error) {
      check(
          error.call!.terminal.failure.isPresent &&
              error.call!.terminal.facts.isPresent,
          'typed failure retains terminal facts');
    }
    check(sends == beforeBudget, 'budget refusal sends nothing');
    final methods = [
      engine.decide,
      engine.choose,
      engine.tag,
      engine.score,
      engine.filter,
      engine.rank,
      engine.find,
      engine.annotate,
      engine.recognize,
      engine.relate
    ];
    final questions = <Question>[
      question,
      DefinedQuestion(
          value: Definition.read({
        'choose': 'Which?',
        'options': ['a', 'b']
      })),
      DefinedQuestion(
          value: Definition.read({
        'tag': 'Which?',
        'labels': ['a', 'b']
      })),
      DefinedQuestion(
          value: Definition.read({
        'score': 'Which?',
        'levels': ['low', 'high']
      })),
      question,
      question,
      QuestionText(text: 'Which text?'),
      DefinedQuestion(
          value: Definition.read({
        'version': 1,
        'questions': {
          'refund': {'decide': 'Is it?'}
        }
      })),
      DefinedQuestion(
          value: Definition.read({
        'version': 1,
        'recognize': {
          'kinds': {'person': 'Person.'}
        }
      })),
      DefinedQuestion(
          value: Definition.read({
        'version': 1,
        'relate': {
          'fields': {'name': '/name', 'kind': '/kind'},
          'relations': [
            {'name': 'knows', 'source': 'person', 'target': 'person'}
          ]
        }
      })),
    ];
    final records = InputRecords(items: [
      Item(
          original: Presence.present(OriginalText(text: 'owned')))
    ]);
    for (var i = 0; i < methods.length; i++) {
      final input = i == 6
          ? InputUnits(items: [
              ...records.items,
              Item(
                  original:
                      Presence.present(OriginalText(text: "other")))
            ])
          : i == 9
              ? InputEntities(items: [
                  Item(
                      original: Presence.present(OriginalJson(
                          value: {'name': 'Alice', 'kind': 'person'}))),
                  Item(
                      original: Presence.present(OriginalJson(
                          value: {'name': 'Bob', 'kind': 'person'})))
                ])
              : i >= 4 && i != 8
                  ? records
                  : text;
      final result = await methods[i](questions[i], input);
      check(!result.terminal.failure.isPresent, 'named call $i');
    }
    await dataCases(engine, 'http://127.0.0.1:${server.port}', () => sends);
    final streamed =
        await engine.decide(question, InputFeed(name: 'feed'),
            feed: Stream.fromIterable([
              FeedItem(
                  item: Item(
                      original: Presence.present(
                          OriginalText(text: 'feed-one')))),
              FeedItem(
                  item: Item(
                      original: Presence.present(
                          OriginalText(text: 'feed-two')))),
            ]));
    check(streamed.packets.whereType<SessionPacketDecideRow>().length == 2,
        'bounded host stream feeds typed rows');
    final before = sends;
    try {
      await engine.decide(QuestionText(text: ''), text);
      throw StateError('invalid accepted');
    } on CallFailure catch (error) {
      check(error.kind == ErrorKind.usage, 'native typed usage error');
    }
    check(sends == before, 'invalid admission sends nothing');
    var cleanupCalls = 0;
    final cancellation = Cancellation();
    final heldFeed =
        StreamController<FeedItem>(onCancel: () {
      cleanupCalls++;
      throw StateError('feed cleanup failed');
    });
    var intake = 0;
    final held = engine.decide(question, InputFeed(name: 'feed'),
        cancellation: cancellation, feed: heldFeed.stream.map((item) {
      intake++;
      return item;
    }));
    heldFeed.add(FeedItem(
        item: Item(
            original:
                Presence.present(OriginalText(text: 'held')))));
    await arrived.future.timeout(const Duration(seconds: 5));
    var progressed = false;
    await Future<void>(() {
      progressed = true;
    });
    check(progressed && !release.isCompleted,
        'host progresses while provider held');
    cancellation.cancel();
    try {
      await held.timeout(const Duration(seconds: 1));
      throw StateError('cancel returned answer');
    } on CallCancelled {}
    heldFeed.add(FeedItem(
        item: Item(
            original:
                Presence.present(OriginalText(text: 'extra')))));
    await Future<void>(() {});
    check(intake == 1 && cleanupCalls == 1,
        'throwing cleanup cancels each feed without extra reads');
    engine.close();
    check(!release.isCompleted, 'cancel and cleanup precede provider release');
    check(
        call.terminal.facts.value != null, 'terminal survives engine cleanup');
    print(
        'PASS: ten named calls, presence/null/false/unknown/wide original, typed terminal failure, zero-send invalid, throwing stream cleanup preserves cancellation, held cancellation and no extra reads');
  } finally {
    engine.close();
    if (!release.isCompleted) release.complete();
    await serving.cancel();
    await server.close(force: true);
  }
}
