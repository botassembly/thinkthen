import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:thinkthen_dart/thinkthen_session.dart';

void check(bool value, String message) {
  if (!value) throw StateError(message);
}

Future<void> main(List<String> args) async {
  final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
  var sends = 0;
  final arrived = Completer<void>(), release = Completer<void>();
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
  });
  final engine = Engine.open(
      settings: InputEngineSettings(
          backend: const Presence.present('typesafe'),
          baseUrl: Presence.present('http://127.0.0.1:${server.port}'),
          cache: const Presence.present(InputCacheDocument.alternative1(false)),
          maxRetries: Presence.present(BigInt.zero),
          maxRequests: const Presence.present(null),
          maxRequestsTotal: const Presence.present(null),
          maxEstimatedInputTokensTotal: const Presence.present(null)));
  final text = InputRequestInputText(text: 'owned');
  final question = InputRequestQuestionText(text: 'Is it?');
  try {
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
            options: InputRequestOptions(
                maxRequestsTotal: Presence.present(BigInt.zero)));
        throw StateError('budget accepted');
      } on SessionFailure catch (error) {
        check(error.call.terminal.facts.isPresent, 'failure retains facts');
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
    final authored = InputRequestQuestionDefinition(
        value: InputRequestDefinitionFieldsDecide(
            decide: InputAuthoredQuestionText.alternative0('Is it?'),
            trueValue: const Presence.present(null),
            falseValue: const Presence.present(
                InputAuthoredCriterion.alternative0('No.'))));
    final present = await engine.decide(
        authored,
        InputRequestInputJson(
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
    final extended = AtomicDecideValue.read({
      ...value.json,
      'future': {'wide': wide}
    });
    check((extended.json['future'] as Map)['wide'] == wide,
        'unknown output retained');
    final absent =
        ReadableQuestionDecide.read({'verb': 'decide', 'text': 'Is it?'});
    check(!absent.trueValue.isPresent, 'absent reading differs from null');
    final unresolved = await engine.decide(
        question, InputRequestInputText(text: 'uncertain'),
        options: InputRequestOptions(
            threshold: const Presence.present(
                InputRequestThreshold.alternative1('0.2:0.8'))));
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
          options: InputRequestOptions(
              maxRequestsTotal: Presence.present(BigInt.zero)));
      throw StateError('budget succeeded');
    } on SessionFailure catch (error) {
      check(
          error.call.terminal.failure.isPresent &&
              error.call.terminal.facts.isPresent,
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
    final questions = <InputRequestQuestion>[
      question,
      InputRequestQuestionDefinition(
          value: InputRequestDefinition.read({
        'choose': 'Which?',
        'options': ['a', 'b']
      })),
      InputRequestQuestionDefinition(
          value: InputRequestDefinition.read({
        'tag': 'Which?',
        'labels': ['a', 'b']
      })),
      InputRequestQuestionDefinition(
          value: InputRequestDefinition.read({
        'score': 'Which?',
        'levels': ['low', 'high']
      })),
      question,
      question,
      InputRequestQuestionText(text: 'Which text?'),
      InputRequestQuestionDefinition(
          value: InputRequestDefinition.read({
        'version': 1,
        'questions': {
          'refund': {'decide': 'Is it?'}
        }
      })),
      InputRequestQuestionDefinition(
          value: InputRequestDefinition.read({
        'version': 1,
        'recognize': {
          'kinds': {'person': 'Person.'}
        }
      })),
      InputRequestQuestionDefinition(
          value: InputRequestDefinition.read({
        'version': 1,
        'relate': {
          'fields': {'name': '/name', 'kind': '/kind'},
          'relations': [
            {'name': 'knows', 'source': 'person', 'target': 'person'}
          ]
        }
      })),
    ];
    final records = InputRequestInputRecords(items: [
      InputRequestItem(
          original: Presence.present(InputRequestOriginalText(text: 'owned')))
    ]);
    for (var i = 0; i < methods.length; i++) {
      final input = i == 6
          ? InputRequestInputUnits(items: [
              ...records.items,
              InputRequestItem(
                  original:
                      Presence.present(InputRequestOriginalText(text: "other")))
            ])
          : i == 9
              ? InputRequestInputEntities(items: [
                  InputRequestItem(
                      original: Presence.present(InputRequestOriginalJson(
                          value: {'name': 'Alice', 'kind': 'person'}))),
                  InputRequestItem(
                      original: Presence.present(InputRequestOriginalJson(
                          value: {'name': 'Bob', 'kind': 'person'})))
                ])
              : i >= 4 && i != 8
                  ? records
                  : text;
      final result = await methods[i](questions[i], input);
      check(!result.terminal.failure.isPresent, 'named call $i');
    }
    final streamed =
        await engine.decide(question, InputRequestInputFeed(name: 'feed'),
            feed: Stream.fromIterable([
              InputRequestSessionDescriptor(
                  item: InputRequestItem(
                      original: Presence.present(
                          InputRequestOriginalText(text: 'feed-one')))),
              InputRequestSessionDescriptor(
                  item: InputRequestItem(
                      original: Presence.present(
                          InputRequestOriginalText(text: 'feed-two')))),
            ]));
    check(streamed.packets.whereType<SessionPacketDecideRow>().length == 2,
        'bounded host stream feeds typed rows');
    final before = sends;
    try {
      await engine.decide(InputRequestQuestionText(text: ''), text);
      throw StateError('invalid accepted');
    } on NativeFailure catch (error) {
      check(error.kind == NativeErrorKind.usage, 'native typed usage error');
    }
    check(sends == before, 'invalid admission sends nothing');
    var cleanupCalls = 0;
    final cancellation = Cancellation();
    final heldFeed =
        StreamController<InputRequestSessionDescriptor>(onCancel: () {
      cleanupCalls++;
      throw StateError('feed cleanup failed');
    });
    var intake = 0;
    final held = engine.decide(question, InputRequestInputFeed(name: 'feed'),
        cancellation: cancellation, feed: heldFeed.stream.map((item) {
      intake++;
      return item;
    }));
    heldFeed.add(InputRequestSessionDescriptor(
        item: InputRequestItem(
            original:
                Presence.present(InputRequestOriginalText(text: 'held')))));
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
    heldFeed.add(InputRequestSessionDescriptor(
        item: InputRequestItem(
            original:
                Presence.present(InputRequestOriginalText(text: 'extra')))));
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
