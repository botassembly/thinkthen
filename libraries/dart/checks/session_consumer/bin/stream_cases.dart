import 'dart:async';
import 'dart:io';
import 'package:thinkthen_dart/thinkthen_session.dart';

void require(bool value, String message) {
  if (!value) throw StateError(message);
}

InputRequestSessionDescriptor descriptor(String text) =>
    InputRequestSessionDescriptor(
        item: InputRequestItem(
            original: Presence.present(InputRequestOriginalText(text: text))));

Future<void> streamCases(Engine engine, Future<void> arrived,
    Completer<void> release, Future<void> prefixAnswered,
    {bool cancelOnly = false}) async {
  final question = InputRequestQuestionText(text: 'Is it?');
  final input = InputRequestInputFeed(name: 'feed');
  if (!cancelOnly) {
    final reader = StreamController<InputRequestSessionDescriptor>();
    final reading = engine.decide(question, input, feed: reader.stream);
    reader.add(descriptor('reader-prefix'));
    await prefixAnswered.timeout(const Duration(seconds: 5));
    reader
        .addError(const FileSystemException('secret host path and diagnostic'));
    await reader.close();
    try {
      await reading.timeout(const Duration(seconds: 5));
      throw StateError('reader failure returned an answer');
    } on SessionFailure catch (error) {
      require(
          error.failure.error.kind == 'local', 'typed native local failure');
      require(error.failure.error.message == 'session input could not be read',
          'safe native diagnostic');
      require(
          error.call.packets.whereType<SessionPacketDecideRow>().length == 1,
          'completed prefix retained');
      require(
          error.call.terminal.facts.isPresent &&
              error.call.terminal.facts.value != null,
          'native terminal facts retained');
      require(error.call.terminal.facts.value!.requestsSent == BigInt.one,
          'native facts retain the observed request');
    }
  }

  final cleanup = Completer<void>();
  var cleanupCalls = 0, intake = 0;
  final feed = StreamController<InputRequestSessionDescriptor>(onCancel: () {
    cleanupCalls++;
    return cleanup.future;
  });
  final cancellation = Cancellation();
  final held = engine.decide(question, input, cancellation: cancellation,
      feed: feed.stream.map((item) {
    intake++;
    return item;
  }));
  feed.add(descriptor('held'));
  await arrived.timeout(const Duration(seconds: 5));
  cancellation.cancel();
  try {
    await held.timeout(const Duration(seconds: 1));
    throw StateError('cancel returned an answer');
  } on CallCancelled {}
  require(!cleanup.isCompleted && !release.isCompleted,
      'cancellation settles while provider and cleanup remain held');
  feed.add(descriptor('unaccepted'));
  await Future<void>(() {});
  require(
      cleanupCalls == 1 && intake == 1, 'cleanup once and no further intake');
  cleanup.completeError(StateError('late cleanup failure'));
  await Future<void>(() {});
  engine.close();
  require(!release.isCompleted, 'native owners release before provider');
  print(
      'PASS: typed reader failure retains prefix and facts; cancellation settles before async cleanup');
}
