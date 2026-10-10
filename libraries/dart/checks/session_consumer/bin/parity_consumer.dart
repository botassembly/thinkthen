import 'dart:async';
import 'dart:io';
import 'package:thinkthen_dart/thinkthen_dart.dart';
import 'package:thinkthen_dart/src/session/values.dart'
    show decodeNative, encodeNative;

Future<void> main(List<String> args) async {
  final home = Platform.environment['TT_CASE_HOME'];
  if (home != null) Directory.current = home;
  final fixture = decodeNative(File(args[0]).readAsStringSync()) as Map;
  Engine? engine;
  Map<String, Object?> payload;
  try {
    engine =
        Engine.open(settings: InputEngineSettings.read(decodeNative(args[2])));
    payload = await consumeFixture(fixture, engine);
  } on NativeFailure catch (error) {
    payload = {
      'admission': {
        'code': error.kind.code,
        'message': error.message,
        if (error.facts != null) 'facts': error.facts
      }
    };
  } on CallCancelled {
    payload = {
      'admission': {
        'code': NativeErrorKind.cancelled.code,
        'message': 'the call was cancelled'
      }
    };
  } finally {
    engine?.close();
  }
  final output = encodeNative(payload);
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
  final question = loader != null
      ? switch (loader) {
          'file' ||
          'load' =>
            InputRequestQuestionFile(path: fixture['reference']),
          'named' ||
          'load_named' =>
            InputRequestQuestionName(name: fixture['reference']),
          'reference' ||
          'load_reference' =>
            InputRequestQuestionReference(reference: fixture['reference']),
          _ => throw StateError('unknown question loader')
        }
      : fixture['question_form'] == 'file'
          ? InputRequestQuestionFile(path: 'fixture-question.json')
          : fixture['raw'] != null
              ? InputRequestQuestionFile(
                  path: (File('raw-question.json')
                        ..writeAsStringSync(fixture['raw']))
                      .path)
              : InputRequestQuestionDefinition(
                  value: InputRequestDefinition.read(fixture['question']));
  final injection = fixture['operation']?['injection'];
  InputRequestInput input;
  if (fixture['paths'] != null || injection == 'recording_read_failure') {
    final unit = switch (fixture['source_unit'] ?? 3) {
      1 => 'line',
      2 => 'window',
      5 => 'line',
      _ => 'file'
    };
    input = InputRequestInputSource(
        source: InputRequestSource(
            paths: [
          for (final path in fixture['paths'] ?? ['target/missing-input'])
            fixture['owned_jsonl'] == true ? path : '$root/$path'
        ],
            framing: fixture['owned_jsonl'] == true
                ? const Presence.present('jsonl')
                : const Presence.absent(),
            media: fixture['image_reader'] == true
                ? const Presence.present('image')
                : const Presence.absent(),
            reading: Presence.present(InputRequestReader(
                unit: Presence.present(unit),
                window: fixture['window'] != null && fixture['window'] != 0
                    ? Presence.present(BigInt.from(fixture['window']))
                    : const Presence.absent()))));
  } else {
    final images = [
      for (final path in fixture['image_paths'] ?? [])
        InputRequestImageFile(
            path: '$root/$path',
            media: Presence.present(fixture['media'] ?? 'image/png'))
    ];
    final items = fixture['items'] as List;
    final records = [
      for (var i = 0; i < items.length; i++)
        InputRequestItem(
            original: fixture['image_only'] == true
                ? const Presence.absent()
                : Presence.present(fixture['caption_files'] == true
                    ? InputRequestOriginalText(
                        text: File('caption-$i.txt').readAsStringSync())
                    : fixture['text'] == true && items[i] is String
                        ? InputRequestOriginalText(text: items[i])
                        : InputRequestOriginalJson(value: items[i])),
            images: images.isEmpty
                ? const Presence.absent()
                : Presence.present(images),
            context: fixture['contexts'] != null
                ? Presence.present(
                    InputContextSchema.read(fixture['contexts'][i]))
                : fixture['context_present'] == true
                    ? Presence.present(
                        InputContextSchema.read(fixture['context']))
                    : const Presence.absent(),
            options: fixture['candidate_orders'] != null
                ? Presence.present([
                    for (final name in fixture['candidate_orders'][i])
                      InputOptionSchema.read(name)
                  ])
                : const Presence.absent())
    ];
    input = verb == 'find'
        ? InputRequestInputUnits(items: records)
        : verb == 'relate'
            ? InputRequestInputEntities(items: records)
            : InputRequestInputRecords(items: records);
  }
  final options = InputRequestOptions(
      attempts: const Presence.present(true),
      details: const Presence.present(true),
      deadlineMs: injection == 'expired_deadline'
          ? Presence.present(BigInt.zero)
          : const Presence.absent(),
      context: fixture['shared_context'] != null
          ? Presence.present(fixture['shared_context'])
          : const Presence.absent());
  final cancellation = Cancellation();
  if (injection == 'cancel_token') cancellation.cancel();
  List<SessionPacket> packets;
  if (fixture['held_cancel'] == true) {
    final session =
        engine.startSession(question, input, verb, options: options);
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
      _ => throw StateError('unknown function')
    };
    try {
      packets = (await method(question, input,
              options: options, cancellation: cancellation))
          .packets;
    } on SessionFailure catch (error) {
      packets = error.call.packets;
    }
  }
  return {
    'packets': [for (final packet in packets) packet.toJson()]
  };
}
