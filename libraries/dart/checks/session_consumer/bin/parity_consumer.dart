import 'dart:async';
import 'dart:io';

import 'package:thinkthen_dart/thinkthen_dart.dart';

import 'fixture_values.dart';

Future<void> main(List<String> args) async {
  final home = Platform.environment['TT_CASE_HOME'];
  if (home != null) Directory.current = home;
  final fixture = decodeFixture(File(args[0]).readAsStringSync()) as Map;
  Engine? engine;
  Map<String, Object?> payload;
  try {
    engine = Engine.open(settings: EngineSettings.read(decodeFixture(args[2])));
    payload = await consumeFixture(fixture, engine);
  } on CallFailure catch (error) {
    payload = {
      'admission': {
        'code': error.kind.code,
        'message': error.message,
        if (error.facts != null) 'facts': error.facts,
      },
    };
  } on CallCancelled {
    payload = {
      'admission': {
        'code': ErrorKind.cancelled.code,
        'message': 'the call was cancelled',
      },
    };
  } finally {
    engine?.close();
  }
  final output = encodeFixture(payload);
  final file = Platform.environment['TT_OUTPUT'];
  if (file == null)
    stdout.writeln(output);
  else
    File(file).writeAsStringSync(output);
}

Future<Map<String, Object?>> consumeFixture(Map fixture, Engine engine) async {
  final verb = fixture['verb'] as String;
  final root = Platform.environment['TT_REPO']!;
  final loader = fixture['loader'];
  final definition = Map<String, Object?>.from(fixture['question']);
  final none = verb == 'find' && definition.remove('none') == true;
  if (fixture['metadata'] != null && !definition.containsKey('questions'))
    definition.addAll(Map<String, Object?>.from(fixture['metadata']));
  final question = loader != null
      ? switch (loader) {
          'file' || 'load' => QuestionFile(path: fixture['reference']),
          'named' || 'load_named' => QuestionName(name: fixture['reference']),
          'reference' || 'load_reference' => QuestionReference(
            reference: fixture['reference'],
          ),
          _ => throw StateError('unknown question loader'),
        }
      : fixture['question_form'] == 'file'
      ? QuestionFile(path: 'fixture-question.json')
      : fixture['raw'] != null
      ? QuestionFile(
          path: (File(
            'raw-question.json',
          )..writeAsStringSync(fixture['raw'])).path,
        )
      : DefinedQuestion(value: Definition.read(definition));
  final injection = fixture['operation']?['injection'];
  Input input;
  Stream<FeedItem>? feed;
  if (fixture['paths'] != null || injection == 'recording_read_failure') {
    final unit = switch (fixture['source_unit'] ?? 3) {
      1 => 'line',
      2 => 'window',
      5 => 'line',
      _ => 'file',
    };
    input = InputSource(
      source: Source(
        paths: [
          for (final path in fixture['paths'] ?? ['target/missing-input'])
            fixture['owned_jsonl'] == true ? path : '$root/$path',
        ],
        framing: fixture['owned_jsonl'] == true || fixture['source_unit'] == 5
            ? const Presence.present('jsonl')
            : const Presence.absent(),
        media: fixture['image_reader'] == true || fixture['source_unit'] == 4
            ? const Presence.present('image')
            : const Presence.absent(),
        reading: Presence.present(
          Reader(
            unit: Presence.present(unit),
            window: fixture['window'] != null && fixture['window'] != 0
                ? Presence.present(BigInt.from(fixture['window']))
                : const Presence.absent(),
          ),
        ),
      ),
    );
  } else {
    final images = [
      for (final path in fixture['image_paths'] ?? [])
        ImageFile(
          path: '$root/$path',
          media: Presence.present(fixture['media'] ?? 'image/png'),
        ),
    ];
    final items = fixture['items'] as List;
    final records = [
      for (var i = 0; i < items.length; i++)
        Item(
          original: fixture['image_only'] == true
              ? const Presence.absent()
              : Presence.present(
                  fixture['caption_files'] == true
                      ? OriginalText(
                          text: File('caption-$i.txt').readAsStringSync(),
                        )
                      : fixture['text'] == true && items[i] is String
                      ? OriginalText(text: items[i])
                      : OriginalJson(value: items[i]),
                ),
          images: images.isEmpty
              ? const Presence.absent()
              : Presence.present(images),
          context: fixture['contexts'] != null
              ? Presence.present(ContextSchema.read(fixture['contexts'][i]))
              : fixture['context_present'] == true
              ? Presence.present(ContextSchema.read(fixture['context']))
              : const Presence.absent(),
          options: fixture['candidate_orders'] != null
              ? Presence.present([
                  for (final name in fixture['candidate_orders'][i])
                    OptionSchema(name: name),
                ])
              : const Presence.absent(),
        ),
    ];
    if (fixture['incremental'] == true) {
      feed = Stream.fromIterable([
        for (final record in records) FeedItem(item: record),
      ]);
    }
    input = feed != null
        ? InputFeed(name: 'owned')
        : verb == 'find'
        ? InputUnits(items: records)
        : verb == 'relate'
        ? InputEntities(items: records)
        : InputRecords(items: records);
  }
  final options = Options(
    attempts: const Presence.present(true),
    none: verb == 'find' ? Presence.present(none) : const Presence.absent(),
    deadlineMs: injection == 'expired_deadline'
        ? Presence.present(BigInt.zero)
        : const Presence.absent(),
    context: fixture['shared_context'] != null
        ? Presence.present(fixture['shared_context'])
        : const Presence.absent(),
  );
  final cancellation = Cancellation();
  if (injection == 'cancel_token') cancellation.cancel();
  List<SessionPacket> packets;
  CallResult? callResult;
  if (feed != null && fixture['settings']?['max_requests'] == 2) {
    final session = startFixture(
      engine,
      verb,
      question,
      input,
      options: options,
    );
    try {
      await for (final item in feed) {
        if (!await session.push(item)) break;
      }
      session.finish();
      packets = await session.packets.toList();
      final rows = packets.whereType<SessionPacketDecideRow>().toList();
      final indices = packets
          .whereType<SessionPacketObservation>()
          .map((packet) => packet.value)
          .whereType<SessionObservationRow>()
          .map((row) => row.index)
          .toList();
      if (rows.length != 2 ||
          rows[0].value.input.value != 'refund one' ||
          rows[1].value.input.value != 'refund two' ||
          indices.length != 2 ||
          indices[0] != BigInt.zero ||
          indices[1] != BigInt.one)
        throw StateError('push-all ordered prefix');
    } finally {
      session.close();
    }
  } else if (fixture['held_cancel'] == true) {
    final session = startFixture(
      engine,
      verb,
      question,
      input,
      options: options,
    );
    final signal = File('${Platform.environment['HOME']}/cancel');
    final watcher = Timer.periodic(const Duration(milliseconds: 10), (_) {
      if (signal.existsSync()) {
        session.cancel();
        File('${signal.path}.fired').writeAsStringSync('');
      }
    });
    try {
      session.finish();
      packets = await session.packets.toList();
    } finally {
      watcher.cancel();
      session.close();
    }
  } else {
    if (fixture['call_result_contract'] == true ||
        fixture['expect']?['call_result'] != null) {
      input = InputText(text: (fixture['items'] as List).single as String);
    }
    final method = switch (verb) {
      'decide' => engine.decide,
      'choose' => engine.choose,
      'tag' => engine.tag,
      'score' => engine.score,
      'filter' => engine.filter,
      'rank' => engine.rank,
      'find' => engine.find,
      'annotate' => engine.annotate,
      'recognize' => engine.recognize,
      'relate' => engine.relate,
      _ => throw StateError('unknown function'),
    };
    try {
      callResult = await method(
        question,
        input,
        options: options,
        cancellation: cancellation,
        feed: feed,
      );
      packets = callResult.packets;
    } on CallFailure catch (error) {
      packets = error.call!.packets;
    }
  }
  return {
    'packets': [for (final packet in packets) fixtureValue(packet)],
    if (callResult != null)
      'call_result': {
        'value': fixtureValue(callResult.value),
        'probability': switch (callResult.probability) {
          ScalarProbability(value: final value) => value,
          ListProbability(value: final value) => value,
          GroupProbability(value: final value) => value,
          null => null,
        },
        'results': [
          for (final result in callResult.results) fixtureValue(result),
        ],
        'facts': fixtureValue(callResult.facts),
      },
  };
}
