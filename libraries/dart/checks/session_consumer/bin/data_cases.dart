import 'dart:io';
import 'dart:typed_data';
import 'package:thinkthen_dart/thinkthen_dart.dart';

void requireData(bool value, String message) {
  if (!value) throw StateError(message);
}

Future<void> dataCases(
    Engine engine, String endpoint, int Function() sends) async {
  final scratch = Directory.systemTemp.createTempSync('thinkthen-caller-');
  final question = InputRequestQuestionText(text: 'Does it apply?');
  try {
    final file = File('${scratch.path}/evidence.txt')
      ..writeAsStringSync('first\nsecond\n');
    final located = await engine.decide(
        question,
        InputRequestInputSource(
            source: InputRequestSource(
                paths: [file.path],
                reading: Presence.present(InputRequestReader(
                    unit: const Presence.present('line'))))));
    requireData(located.packets.whereType<SessionPacketDecideRow>().length == 2,
        'native file reader retains both lines');
    final beforeMissing = sends();
    try {
      await engine.decide(
          question,
          InputRequestInputSource(
              source: InputRequestSource(paths: ['${scratch.path}/missing'])));
      throw StateError('missing file accepted');
    } on SessionFailure catch (error) {
      requireData(
          !error.call.terminal.facts.isPresent &&
              error.failure.error.kind == 'local',
          'file failure before start keeps facts absent');
    } on NativeFailure catch (error) {
      requireData(error.kind == NativeErrorKind.local,
          'prestart file failure is local');
    }
    requireData(sends() == beforeMissing, 'missing file sends nothing');
    final image = imageBytes(Uint8List.fromList([0, 1, 2]), media: 'image/png');
    final beforeImage = sends();
    for (final method in [engine.decide, engine.tag]) {
      try {
        await method(
            question,
            InputRequestInputText(
                text: 'caption', images: Presence.present([image])));
        throw StateError('unadmitted image accepted');
      } on NativeFailure catch (error) {
        requireData(error.kind == NativeErrorKind.usage,
            'image admission stays native');
      }
    }
    requireData(sends() == beforeImage, 'unadmitted image routes send nothing');
    final recording = '${scratch.path}/recording';
    InputEngineSettings settings({bool replay = false}) => InputEngineSettings(
        backend: const Presence.present('typesafe'),
        baseUrl: Presence.present(endpoint),
        cache: const Presence.present(InputCacheDocument.alternative1(false)),
        record: replay ? const Presence.absent() : Presence.present(recording),
        replay: replay ? Presence.present(recording) : const Presence.absent());
    final input = InputRequestInputText(text: 'recording-evidence');
    final recorder = Engine.open(settings: settings());
    try {
      await recorder.decide(question, input);
    } finally {
      recorder.close();
    }
    final beforeReplay = sends();
    final replayer = Engine.open(settings: settings(replay: true));
    try {
      final replay = await replayer.decide(question, input);
      requireData(
          replay.terminal.facts.isPresent, 'replay retains final facts');
      try {
        await replayer.decide(
            question, InputRequestInputText(text: 'cache-miss'));
        throw StateError('replay miss accepted');
      } on SessionFailure catch (error) {
        requireData(error.call.terminal.facts.isPresent,
            'replay miss retains failure facts');
      }
    } finally {
      replayer.close();
    }
    requireData(sends() == beforeReplay, 'replay and replay miss send nothing');
    final cached = Engine.open(
        settings: InputEngineSettings(
            backend: const Presence.present('typesafe'),
            baseUrl: Presence.present(endpoint),
            cache:
                const Presence.present(InputCacheDocument.alternative1(true))));
    try {
      await cached.decide(
          question, InputRequestInputText(text: 'cached-evidence'));
      final beforeHit = sends();
      await cached.decide(
          question, InputRequestInputText(text: 'cached-evidence'));
      requireData(sends() == beforeHit, 'cache hit sends nothing');
    } finally {
      cached.close();
    }
    print(
        'PASS: native files, missing file, image admission, cache hit, offline replay and replay miss');
  } finally {
    scratch.deleteSync(recursive: true);
  }
}
