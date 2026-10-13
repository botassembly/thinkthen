import 'dart:io';
import 'dart:convert';
import 'dart:typed_data';
import 'package:thinkthen_dart/thinkthen_dart.dart';

void requireData(bool value, String message) {
  if (!value) throw StateError(message);
}

Future<void> dataCases(
    Engine engine, String endpoint, int Function() sends) async {
  final scratch = Directory.systemTemp.createTempSync('thinkthen-caller-');
  final question = QuestionText(text: 'Does it apply?');
  try {
    final file = File('${scratch.path}/evidence.txt')
      ..writeAsStringSync('first\nsecond\n');
    final located = await engine.decide(
        question,
        InputSource(
            source: Source(
                paths: [file.path],
                reading: Presence.present(Reader(
                    unit: const Presence.present('line'))))));
    requireData(located.packets.whereType<SessionPacketDecideRow>().length == 2,
        'native file reader retains both lines');
    final beforeMissing = sends();
    try {
      await engine.decide(
          question,
          InputSource(
              source: Source(paths: ['${scratch.path}/missing'])));
      throw StateError('missing file accepted');
    } on CallFailure catch (error) {
      if (error.call != null) {
        requireData(!error.call!.terminal.facts.isPresent && error.failure!.error.kind == 'local',
            'file failure before start keeps facts absent');
      } else {
        requireData(error.kind == ErrorKind.local && error.facts == null,
            'prestart file failure is local and keeps facts absent');
      }
    }
    requireData(sends() == beforeMissing, 'missing file sends nothing');
    final image = imageBytes(Uint8List.fromList([0, 1, 2]), media: 'image/png');
    final beforeImage = sends();
    for (final method in [engine.decide, engine.tag]) {
      try {
        await method(
            question,
            InputText(
                text: 'caption', images: Presence.present([image])));
        throw StateError('unadmitted image accepted');
      } on CallFailure catch (error) {
        requireData(error.kind == ErrorKind.usage,
            'image admission stays native');
      }
    }
    requireData(sends() == beforeImage, 'unadmitted image routes send nothing');
    final admittedBytes = base64Decode('__CANONICAL_IMAGE__');
    final admittedFile = File('${scratch.path}/image.png')
      ..writeAsBytesSync(admittedBytes);
    final admitted = Engine.open(
        settings: EngineSettings(
            backend: const Presence.present('liquid'),
            model: const Presence.present('d1'),
            baseUrl: Presence.present(endpoint),
            cache: const Presence.present(
                CacheDocument.boolean(false))));
    try {
      final beforeAdmitted = sends();
      final result = await admitted.decide(
          question,
          InputText(
              text: 'image-caption',
              images: Presence.present([
                imageBytes(admittedBytes, media: 'image/png'),
                ImageFile(path: admittedFile.path)
              ])));
      requireData(
          result.packets.whereType<SessionPacketDecideRow>().single.value.answer
                  is AnswerYesNo &&
              sends() == beforeAdmitted + 1,
          'admitted byte and file images produce one answer');
    } finally {
      admitted.close();
    }
    final recording = '${scratch.path}/recording';
    EngineSettings settings({bool replay = false}) => EngineSettings(
        backend: const Presence.present('typesafe'),
        baseUrl: Presence.present(endpoint),
        cache: const Presence.present(CacheDocument.boolean(false)),
        record: replay ? const Presence.absent() : Presence.present(recording),
        replay: replay ? Presence.present(recording) : const Presence.absent());
    final input = InputText(text: 'recording-evidence');
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
            question, InputText(text: 'cache-miss'));
        throw StateError('replay miss accepted');
      } on CallFailure catch (error) {
        requireData(error.call!.terminal.facts.isPresent,
            'replay miss retains failure facts');
      }
    } finally {
      replayer.close();
    }
    requireData(sends() == beforeReplay, 'replay and replay miss send nothing');
    final cached = Engine.open(
        settings: EngineSettings(
            backend: const Presence.present('typesafe'),
            baseUrl: Presence.present(endpoint),
            cache: Presence.present(
                CacheDocument.text('${scratch.path}/cache'))));
    try {
      await cached.decide(
          question, InputText(text: 'cached-evidence'));
      final beforeHit = sends();
      await cached.decide(
          question, InputText(text: 'cached-evidence'));
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
