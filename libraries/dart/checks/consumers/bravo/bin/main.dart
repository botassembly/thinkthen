import 'dart:async';
import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:isolate';

import 'package:thinkthen_dart/src/allocator.dart' as memory;
import 'package:thinkthen_dart/thinkthen_dart.dart';

void require(bool ok, String label) {
  if (!ok) throw StateError(label);
  print('PASS $label');
}

Future<void> arrival(String name) async {
  final file = File('${Platform.environment['TT_BARRIER_DIR']}/arrived-$name');
  for (var i = 0; i < 6000; i++) {
    if (await file.exists()) return;
    await Future<void>.delayed(const Duration(milliseconds: 5));
  }
  throw StateError('missing arrival $name');
}

void release(String name) =>
    File('${Platform.environment['TT_BARRIER_DIR']}/release-$name')
        .writeAsStringSync('go');
void worker(List<Object> args) {
  final send = args[0] as SendPort;
  final door = Door(args[1] as String);
  final engine = Pointer<Void>.fromAddress(args[2] as int);
  final token = Pointer<Void>.fromAddress(args[3] as int);
  final state = args[4] as String;
  final mode = args[5] as String;
  try {
    if (mode == 'bulk')
      door.many(engine, 'Is it?', [state, 'hold-bulk-second'], token: token);
    else
      door.decide(
        engine,
        'Is it?',
        state,
        deadline: mode == 'deadline' ? 1000 : -1,
        token: token,
      );
    send.send(['wrong-success', 0, '']);
  } on DoorFailure catch (e) {
    send.send([
      'failure',
      door.errorCode(engine),
      e.kind.name,
      e.message,
      e.retryable,
    ]);
  } catch (e) {
    send.send(['unexpected', -1, '$e']);
  }
}

void fire(List<Object> args) {
  final door = Door(args[1] as String);
  final token = Pointer<Void>.fromAddress(args[2] as int);
  door.cancel(token);
  door.cancel(token);
  (args[0] as SendPort).send('fired twice');
}

void invalidInputAllocationRegression(Door door, Pointer<Void> engine) {
  final spec =
      '{"version":1,"recognize":{"kinds":{"person":"A person name."}}}';
  final relation =
      '{"version":1,"relate":{"relations":[{"name":"caused_by","source":"alert","target":"alert"}]}}';
  final cases = <(String, void Function(), bool Function(Object?))>[
    (
      'decide-question-nul',
      () => door.decide(engine, 'Is\u0000 it?', 'yes'),
      (e) => e is ArgumentError && '$e'.contains('embedded NUL'),
    ),
    (
      'decide-text-nul',
      () => door.decide(engine, 'Is it?', 'yes\u0000'),
      (e) => e is ArgumentError && '$e'.contains('embedded NUL'),
    ),
    (
      'many-question-nul',
      () => door.many(engine, 'Is\u0000 it?', ['yes']),
      (e) => e is ArgumentError && '$e'.contains('embedded NUL'),
    ),
    (
      'many-partial-records',
      () => door.many(engine, 'Is it?', ['first', 'bad\u0000', 'not-reached']),
      (e) => e is ArgumentError && '$e'.contains('embedded NUL'),
    ),
    (
      'structured-spec-nul',
      () => door.structured(engine, '$spec\u0000', ['first'], recognize: true),
      (e) => e is ArgumentError && '$e'.contains('embedded NUL'),
    ),
    (
      'structured-partial-records',
      () => door.structured(
          engine,
          relation,
          [
            'first',
            'bad\u0000',
            'not-reached',
          ],
          recognize: false),
      (e) => e is ArgumentError && '$e'.contains('embedded NUL'),
    ),
    (
      'structured-non-nul-single',
      () => door.structured(engine, spec, ['first', 'second'], recognize: true),
      (e) => e is StateError && '$e'.contains('Too many elements'),
    ),
  ];
  for (final (name, action, expected) in cases) {
    for (var attempt = 0; attempt < 3; attempt++) {
      final before = memory.liveAllocations;
      Object? caught;
      try {
        action();
      } catch (e) {
        caught = e;
      }
      if (!expected(caught))
        throw StateError(
          'invalid input $name: expected rejection, got $caught',
        );
      if (memory.liveAllocations != before)
        throw StateError(
          'invalid input $name: allocated before=$before after=${memory.liveAllocations}',
        );
    }
    print('INVALID_INPUT_NO_LEAK_PASS $name repeated=3');
  }
}

bool engineCanBeFreed = true;
Future<bool> joinIsolate(
  Isolate? isolate,
  Future<dynamic>? exited,
  String label,
) async {
  if (isolate == null || exited == null) return true;
  try {
    await exited.timeout(const Duration(seconds: 30));
    return true;
  } on TimeoutException {
    isolate.kill(priority: Isolate.immediate);
    try {
      await exited.timeout(const Duration(seconds: 3));
      return true;
    } on TimeoutException {
      print('ISOLATE_JOIN_FAILED $label');
      return false;
    }
  }
}

Future<void> held(
  Door door,
  Pointer<Void> engine,
  String library,
  String state,
  int wanted, {
  String mode = 'scalar',
}) async {
  final token = door.tokenNew();
  require(token.address != 0, 'token allocation');
  final receiver = ReceivePort(),
      exited = ReceivePort(),
      fireReceiver = ReceivePort(),
      fireExited = ReceivePort();
  Future<dynamic>? workerExitFuture, fireExitFuture;
  Isolate? workerIsolate, cancelIsolate;
  bool completed = false;
  try {
    workerIsolate = await Isolate.spawn(
        worker,
        [
          receiver.sendPort,
          library,
          engine.address,
          token.address,
          state,
          mode,
        ],
        onExit: exited.sendPort);
    workerExitFuture = exited.first;
    await arrival(state);
    if (Platform.environment['TT_PLANT'] == 'fail-mid-case' &&
        state == 'hold-scalar') throw StateError('PLANTED_MID_CASE_FAILURE');
    if (wanted == 5) {
      cancelIsolate = await Isolate.spawn(
          fire,
          [
            fireReceiver.sendPort,
            library,
            token.address,
          ],
          onExit: fireExited.sendPort);
      fireExitFuture = fireExited.first;
      require(
        await fireReceiver.first.timeout(const Duration(seconds: 30)) ==
            'fired twice',
        'second isolate fired twice after counted arrival',
      );
      release(state);
      if (mode == 'bulk') release('hold-bulk-second');
    }
    // A fired token lets the sent request finish, so a cancelled call answers
    // after the release. A deadline ends the call while the reply is held, so
    // the release waits for that answer (ticket 0356).
    final answered = receiver.first;
    if (wanted == 3) {
      await answered.timeout(const Duration(seconds: 30));
      release(state);
    }
    final result = await answered.timeout(const Duration(seconds: 30)) as List;
    require(
      result[0] == 'failure' &&
          result[1] == wanted &&
          result[2] == (wanted == 5 ? 'cancelled' : 'deadline') &&
          result[3].toString().isNotEmpty &&
          result[4] == false,
      '$mode held $state code $wanted, outputs untouched, error copied in caller isolate',
    );
    if (wanted == 5) {
      try {
        door.decide(engine, 'Is it?', 'no-arrival-spent-$state', token: token);
        throw StateError('fired token reused');
      } on DoorFailure catch (e) {
        require(
          e.kind == ErrorKind.cancelled,
          'fired-token reuse refuses without sending',
        );
      }
    }
    completed = true;
  } finally {
    // Release a blocked HTTP reply on every path; never free token/engine while a native call may run.
    try {
      release(state);
      if (mode == 'bulk') release('hold-bulk-second');
    } catch (e) {
      print(
        'BARRIER_RELEASE_ERROR $e; retaining native lifetimes if isolates do not exit',
      );
    }
    final workerJoined = await joinIsolate(
      workerIsolate,
      workerIsolate == null ? null : workerExitFuture,
      'worker',
    );
    final cancellerJoined = await joinIsolate(
      cancelIsolate,
      cancelIsolate == null ? null : fireExitFuture,
      'canceller',
    );
    receiver.close();
    exited.close();
    fireReceiver.close();
    fireExited.close();
    if (!workerJoined || !cancellerJoined) {
      engineCanBeFreed = false;
      throw StateError(
        'unsafe isolate lifetime: native engine and token deliberately retained',
      );
    }
    print('ISOLATES_JOINED worker canceller($wanted)');
    door.tokenFree(token);
    print('TOKEN_FREED_AFTER_JOIN');
    if (!completed) print('CLEANUP_ORDER_PASS $state');
  }
}

Future<void> main(List<String> args) async {
  if (args.length != 1) throw ArgumentError('library path');
  memory.allocatorNegatives();
  print('ALLOCATOR_NEGATIVES_PASS');
  final door = Door(args.single);
  final engine = door.create();
  try {
    invalidInputAllocationRegression(door, engine);
    for (final (text, outcome, probability) in [
      ('yes', 1, .9),
      ('no', 0, .1),
      ('unsure', 2, .5),
    ]) {
      final answer = door.decide(
        engine,
        text == 'unsure'
            ? '{"decide":"Is it?","threshold":"0.4:0.8"}'
            : 'Is it?',
        text,
      );
      require(
        answer.value.outcome == Outcome.values[outcome] &&
            answer.value.probability == probability,
        'scalar $text ${answer.value.outcome}/${answer.value.probability}',
      );
    }
    final described = door.ask(engine, {
      'decide': 'Is it?',
      'true': {
        'what': 'Affirmative answer.',
        'not_for': 'Unclear.',
        'examples': ['yes'],
      },
      'false': 'Negative answer.',
      'evidence': 'described',
    }) as Map;
    require(
      described['value'] == true,
      'structured description map across JSON door',
    );
    final before = door.call(engine, '{"usage":true}') as Map;
    final cached = door.decide(engine, 'Is it?', 'yes');
    final after = door.call(engine, '{"usage":true}') as Map;
    require(
      cached.value.outcome == Outcome.yes &&
          after['requests_sent'] == before['requests_sent'] &&
          after['cache_answers'] > before['cache_answers'],
      'cache coalescing and counters',
    );
    final batch = door.many(engine, 'Is it?', ['batch-one', 'batch-two']);
    require(
      batch.value.length == 2 &&
          batch.value[0].outcome == Outcome.yes &&
          batch.value[0].probability == .9 &&
          batch.value[1].outcome == Outcome.no &&
          batch.value[1].probability == .1,
      'bulk ordered answers .9 then .1',
    );
    final decide = door.call(
      engine,
      '{"decide":"Is it?","evidence":"json-decide"}',
    ) as Map;
    require(
      decide['value'] == true &&
          (decide['facts'] as Map)['records'] == 1 &&
          (decide['facts'] as Map)['requests_sent'] == 1,
      'JSON door exact decision and facts identity',
    );
    final annotate = door.call(
      engine,
      '{"annotate":{"version":1,"questions":{"check":{"decide":"Is it?"}}},"records":["annotate-one"]}',
    ) as Map;
    require(
      jsonEncode(annotate['value']) == '[{"check":true}]' &&
          (annotate['facts'] as Map)['records'] == 1,
      'annotate JSON exact field identity',
    );
    final checked =
        readField(((annotate['value'] as List).single as Map)['check']);
    require(
      checked is AnswerField && checked.value == true,
      'annotate field read through readField',
    );
    final spec =
        '{"version":1,"recognize":{"kinds":{"person":"A person name."}}}';
    final recognized = door
        .structured(engine, spec, ['John Smith'], recognize: true)
        .value as Map;
    require(
      jsonEncode(recognized) ==
          '{"entities":[{"text":"John Smith","start":0,"end":10,"length":10,"kind":"person","strength":0.81}]}',
      'recognize JSON exact entity identity',
    );
    final relSpec =
        '{"version":1,"relate":{"relations":[{"name":"caused_by","source":"alert","target":"alert"}]} }';
    final related = door
        .structured(
            engine,
            relSpec,
            [
              '{"name":"First","kind":"alert"}',
              '{"name":"Second","kind":"alert"}',
            ],
            recognize: false)
        .value as Map;
    final edges = related['edges'] as List;
    require(
      edges.length == 2 &&
          jsonEncode(edges[0]) ==
              '{"relation":"caused_by","source":{"name":"First","kind":"alert"},"target":{"name":"Second","kind":"alert"},"probability":0.9}' &&
          jsonEncode(edges[1]) ==
              '{"relation":"caused_by","source":{"name":"Second","kind":"alert"},"target":{"name":"First","kind":"alert"},"probability":0.9}',
      'relate JSON exact edges and order',
    );
    door.plainAliases(engine);
    print('PLAIN_ENTRYPOINTS_PASS five non-opts aliases');
    try {
      door.decide(engine, 'Is it?', 'backend-failure');
      throw StateError('backend failure succeeded');
    } on DoorFailure catch (e) {
      require(
        e.kind == ErrorKind.backend &&
            e.facts != null &&
            !e.retryable &&
            e.message.isNotEmpty,
        'backend error code/message/retryable',
      );
    }
    require(door.errorFacts(engine).address != 0, 'failure facts borrowed');
    final failureFacts =
        jsonDecode(memory.decodeCString(door.errorFacts(engine))) as Map;
    require(
      failureFacts['requests_sent'] == 1 && failureFacts['records'] == 0,
      'failure facts exact counted send',
    );
    try {
      door.decide(engine, 'Is it?', 'never-spent', deadline: 0);
      throw StateError('deadline succeeded');
    } on DoorFailure catch (e) {
      require(e.kind == ErrorKind.deadline, 'spent deadline refusal');
    }
    for (final settings in ['{"not_a_setting":1}', '{"timeout":"wrong"}']) {
      try {
        door.create(settings);
        throw StateError('bad settings accepted');
      } on DoorFailure catch (e) {
        require(
          e.kind == ErrorKind.usage,
          'constructor invalid settings refusal',
        );
      }
    }
    final configured = door.create(
      jsonEncode({
        'base_url': Platform.environment['THINKTHEN_BASE_URL'],
        'cache': Platform.environment['THINKTHEN_CACHE'],
      }),
    );
    try {
      require(
        door.decide(configured, 'Is it?', 'configured').value.outcome ==
            Outcome.yes,
        'settings constructor route',
      );
    } finally {
      door.engineFree(configured);
    }
    final empty = door.create('{}');
    try {
      require(
        door.decide(empty, 'Is it?', 'yes').value.outcome == Outcome.yes,
        '{} constructor equivalence',
      );
    } finally {
      door.engineFree(empty);
    }
    await held(door, engine, args.single, 'hold-scalar', 5);
    final previous = door.call(engine, '{"usage":true}') as Map;
    require(
      door.decide(engine, 'Is it?', 'hold-scalar').value.outcome == Outcome.yes,
      'cancelled reply retained in cache',
    );
    final present = door.call(engine, '{"usage":true}') as Map;
    require(
      previous['requests_sent'] == present['requests_sent'] &&
          present['cache_answers'] > previous['cache_answers'],
      'cancel drain cache/counters',
    );
    final fresh = door.tokenNew();
    try {
      require(
        door
                .decide(engine, 'Is it?', 'recovery-scalar', token: fresh)
                .value
                .outcome ==
            Outcome.yes,
        'fresh token recovery',
      );
    } finally {
      door.tokenFree(fresh);
    }
    await held(door, engine, args.single, 'hold-bulk-first', 5, mode: 'bulk');
    await held(door, engine, args.single, 'hold-deadline', 3, mode: 'deadline');
    require(
      memory.liveAllocations == 0,
      'allocator leak check after all calls',
    );
    print('DART_STAGE_TWO_PASS bravo');
  } finally {
    if (engineCanBeFreed) {
      door.engineFree(engine);
      print('ENGINE_FREED_AFTER_JOIN');
    }
  }
}
