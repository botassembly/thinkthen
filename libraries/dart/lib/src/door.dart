import 'dart:convert';
import 'dart:ffi';

import 'allocator.dart' as memory;
import 'typed.dart';

final class Answer extends Struct {
  @Int32()
  external int outcome;
  @Double()
  external double probability;
}

final class _OwnedPointers {
  final List<Pointer> _items = [];
  Pointer<T> add<T extends NativeType>(Pointer<T> pointer) {
    _items.add(pointer);
    return pointer;
  }

  void releaseAll() {
    for (final pointer in _items.reversed) {
      memory.release(pointer);
    }
  }
}

final class DoorFailure implements Exception {
  final int _code;
  final String message;
  final bool retryable;
  final Map<String, Object?>? facts;
  DoorFailure(this._code, this.message, this.retryable, [this.facts]);
  ErrorKind get kind => ErrorKind.values[_code - 1];
  @override
  String toString() => 'DoorFailure($kind, $message, retryable=$retryable)';
}

class Door {
  final DynamicLibrary lib;
  Door(String path) : lib = DynamicLibrary.open(path);
  late final Pointer<Void> Function() engineNew =
      lib.lookupFunction<Pointer<Void> Function(), Pointer<Void> Function()>(
    'thinkthen_engine_new',
  );
  late final Pointer<Void> Function(Pointer<Uint8>) engineNewWith =
      lib.lookupFunction<Pointer<Void> Function(Pointer<Uint8>),
          Pointer<Void> Function(Pointer<Uint8>)>('thinkthen_engine_new_with');
  late final void Function(Pointer<Void>) engineFree = lib.lookupFunction<
      Void Function(Pointer<Void>),
      void Function(Pointer<Void>)>('thinkthen_engine_free');
  late final Pointer<Void> Function() tokenNew =
      lib.lookupFunction<Pointer<Void> Function(), Pointer<Void> Function()>(
    'thinkthen_cancel_token_new',
  );
  late final void Function(Pointer<Void>) cancel = lib.lookupFunction<
      Void Function(Pointer<Void>),
      void Function(Pointer<Void>)>('thinkthen_cancel');
  late final void Function(Pointer<Void>) tokenFree = lib.lookupFunction<
      Void Function(Pointer<Void>),
      void Function(Pointer<Void>)>('thinkthen_cancel_token_free');
  late final int Function(Pointer<Void>) errorCode = lib.lookupFunction<
      Int32 Function(Pointer<Void>),
      int Function(Pointer<Void>)>('thinkthen_error_code');
  late final int Function(Pointer<Void>) errorRetryable = lib.lookupFunction<
      Int32 Function(Pointer<Void>),
      int Function(Pointer<Void>)>('thinkthen_error_retryable');
  late final Pointer<Uint8> Function(Pointer<Void>) errorMessage =
      lib.lookupFunction<Pointer<Uint8> Function(Pointer<Void>),
          Pointer<Uint8> Function(Pointer<Void>)>('thinkthen_error_message');
  late final Pointer<Uint8> Function(Pointer<Void>) errorFacts =
      lib.lookupFunction<Pointer<Uint8> Function(Pointer<Void>),
          Pointer<Uint8> Function(Pointer<Void>)>('thinkthen_error_facts_json');
  late final int Function(
    Pointer<Void>,
    Pointer<Uint8>,
    Pointer<Uint8>,
    int,
    int,
    Pointer<Void>,
    Pointer<Answer>,
    Pointer<Pointer<Uint8>>,
    Pointer<IntPtr>,
  ) decideOpts = lib.lookupFunction<
      Int32 Function(
        Pointer<Void>,
        Pointer<Uint8>,
        Pointer<Uint8>,
        IntPtr,
        Int64,
        Pointer<Void>,
        Pointer<Answer>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
      ),
      int Function(
        Pointer<Void>,
        Pointer<Uint8>,
        Pointer<Uint8>,
        int,
        int,
        Pointer<Void>,
        Pointer<Answer>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
      )>('thinkthen_decide_with_facts_opts');
  late final int Function(
    Pointer<Void>,
    Pointer<Uint8>,
    Pointer<Pointer<Uint8>>,
    Pointer<IntPtr>,
    int,
    int,
    Pointer<Void>,
    Pointer<Answer>,
    Pointer<Pointer<Uint8>>,
    Pointer<IntPtr>,
  ) manyOpts = lib.lookupFunction<
      Int32 Function(
        Pointer<Void>,
        Pointer<Uint8>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
        IntPtr,
        Int64,
        Pointer<Void>,
        Pointer<Answer>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
      ),
      int Function(
        Pointer<Void>,
        Pointer<Uint8>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
        int,
        int,
        Pointer<Void>,
        Pointer<Answer>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
      )>('thinkthen_decide_many_with_facts_opts');
  late final Pointer<Uint8> Function(
    Pointer<Void>,
    Pointer<Uint8>,
    int,
    Pointer<Void>,
  ) callOpts = lib.lookupFunction<
      Pointer<Uint8> Function(
        Pointer<Void>,
        Pointer<Uint8>,
        Int64,
        Pointer<Void>,
      ),
      Pointer<Uint8> Function(
        Pointer<Void>,
        Pointer<Uint8>,
        int,
        Pointer<Void>,
      )>('thinkthen_call_opts');
  late final int Function(
    Pointer<Void>,
    Pointer<Uint8>,
    Pointer<Uint8>,
    int,
    int,
    Pointer<Void>,
    Pointer<Pointer<Uint8>>,
    Pointer<IntPtr>,
    Pointer<Pointer<Uint8>>,
    Pointer<IntPtr>,
  ) recognizeOpts = lib.lookupFunction<
      Int32 Function(
        Pointer<Void>,
        Pointer<Uint8>,
        Pointer<Uint8>,
        IntPtr,
        Int64,
        Pointer<Void>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
      ),
      int Function(
        Pointer<Void>,
        Pointer<Uint8>,
        Pointer<Uint8>,
        int,
        int,
        Pointer<Void>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
      )>('thinkthen_recognize_with_facts_opts');
  late final int Function(
    Pointer<Void>,
    Pointer<Uint8>,
    Pointer<Pointer<Uint8>>,
    Pointer<IntPtr>,
    int,
    int,
    Pointer<Void>,
    Pointer<Pointer<Uint8>>,
    Pointer<IntPtr>,
    Pointer<Pointer<Uint8>>,
    Pointer<IntPtr>,
  ) relateOpts = lib.lookupFunction<
      Int32 Function(
        Pointer<Void>,
        Pointer<Uint8>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
        IntPtr,
        Int64,
        Pointer<Void>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
      ),
      int Function(
        Pointer<Void>,
        Pointer<Uint8>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
        int,
        int,
        Pointer<Void>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
      )>('thinkthen_relate_with_facts_opts');
  late final void Function(Pointer<Uint8>) freeString = lib.lookupFunction<
      Void Function(Pointer<Uint8>),
      void Function(Pointer<Uint8>)>('thinkthen_free_string');
  late final int Function(
    Pointer<Void>,
    Pointer<Uint8>,
    Pointer<Uint8>,
    int,
    Pointer<Answer>,
  ) decidePlain = lib.lookupFunction<
      Int32 Function(
        Pointer<Void>,
        Pointer<Uint8>,
        Pointer<Uint8>,
        IntPtr,
        Pointer<Answer>,
      ),
      int Function(
        Pointer<Void>,
        Pointer<Uint8>,
        Pointer<Uint8>,
        int,
        Pointer<Answer>,
      )>('thinkthen_decide');
  late final int Function(
    Pointer<Void>,
    Pointer<Uint8>,
    Pointer<Pointer<Uint8>>,
    Pointer<IntPtr>,
    int,
    Pointer<Answer>,
  ) manyPlain = lib.lookupFunction<
      Int32 Function(
        Pointer<Void>,
        Pointer<Uint8>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
        IntPtr,
        Pointer<Answer>,
      ),
      int Function(
        Pointer<Void>,
        Pointer<Uint8>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
        int,
        Pointer<Answer>,
      )>('thinkthen_decide_many');
  late final Pointer<Uint8> Function(Pointer<Void>, Pointer<Uint8>) callPlain =
      lib.lookupFunction<
          Pointer<Uint8> Function(Pointer<Void>, Pointer<Uint8>),
          Pointer<Uint8> Function(
              Pointer<Void>, Pointer<Uint8>)>('thinkthen_call');
  late final int Function(
    Pointer<Void>,
    Pointer<Uint8>,
    Pointer<Uint8>,
    int,
    Pointer<Pointer<Uint8>>,
    Pointer<IntPtr>,
  ) recognizePlain = lib.lookupFunction<
      Int32 Function(
        Pointer<Void>,
        Pointer<Uint8>,
        Pointer<Uint8>,
        IntPtr,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
      ),
      int Function(
        Pointer<Void>,
        Pointer<Uint8>,
        Pointer<Uint8>,
        int,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
      )>('thinkthen_recognize');
  late final int Function(
    Pointer<Void>,
    Pointer<Uint8>,
    Pointer<Pointer<Uint8>>,
    Pointer<IntPtr>,
    int,
    Pointer<Pointer<Uint8>>,
    Pointer<IntPtr>,
  ) relatePlain = lib.lookupFunction<
      Int32 Function(
        Pointer<Void>,
        Pointer<Uint8>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
        IntPtr,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
      ),
      int Function(
        Pointer<Void>,
        Pointer<Uint8>,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
        int,
        Pointer<Pointer<Uint8>>,
        Pointer<IntPtr>,
      )>('thinkthen_relate');
  // Exercise all five non-_opts entry points on existing cache keys.
  void plainAliases(Pointer<Void> engine) {
    final question = memory.cString('Is it?'), text = memory.cString('yes');
    final answer = memory.allocate(sizeOf<Answer>()).cast<Answer>();
    final ptrs =
        memory.allocate(sizeOf<Pointer<Uint8>>()).cast<Pointer<Uint8>>();
    final lens = memory.allocate(sizeOf<IntPtr>()).cast<IntPtr>();
    final result =
        memory.allocate(sizeOf<Pointer<Uint8>>()).cast<Pointer<Uint8>>();
    final resultLen = memory.allocate(sizeOf<IntPtr>()).cast<IntPtr>();
    final usage = memory.cString('{"usage":true}');
    final spec = memory.cString(
      '{"version":1,"recognize":{"kinds":{"person":"A person name."}}}',
    );
    final person = memory.cString('John Smith');
    final relation = memory.cString(
      '{"version":1,"relate":{"relations":[{"name":"caused_by","source":"alert","target":"alert"}]} }',
    );
    final first = memory.cString('{"name":"First","kind":"alert"}');
    final second = memory.cString('{"name":"Second","kind":"alert"}');
    final both =
        memory.allocate(2 * sizeOf<Pointer<Uint8>>()).cast<Pointer<Uint8>>();
    final lengths = memory.allocate(2 * sizeOf<IntPtr>()).cast<IntPtr>();
    try {
      if (decidePlain(engine, question, text, 3, answer) != 0 ||
          answer.ref.outcome != 1 ||
          answer.ref.probability != .9)
        throw StateError('plain decide exact 1/.9');
      ptrs[0] = text;
      lens[0] = 3;
      if (manyPlain(engine, question, ptrs, lens, 1, answer) != 0 ||
          answer.ref.outcome != 1 ||
          answer.ref.probability != .9)
        throw StateError('plain many exact 1/.9');
      final expectedUsage = call(engine, '{"usage":true}') as Map;
      final json = callPlain(engine, usage);
      if (json.address == 0) throw StateError('plain call');
      try {
        final usageValue = jsonDecode(memory.decodeCString(json)) as Map;
        for (final key in [
          'requests_sent',
          'retries',
          'input_tokens',
          'output_tokens',
        ]) {
          if (usageValue[key] != expectedUsage[key])
            throw StateError('plain JSON usage identity $key');
        }
        if (usageValue['cache_answers'] is! int ||
            usageValue['cache_answers'] < expectedUsage['cache_answers'])
          throw StateError('plain JSON cache identity');
      } finally {
        freeString(json);
      }
      if (recognizePlain(engine, spec, person, 10, result, resultLen) != 0 ||
          result.value.address == 0) throw StateError('plain recognize');
      try {
        if (jsonEncode(
              jsonDecode(
                utf8.decode(result.value.asTypedList(resultLen.value)),
              ),
            ) !=
            '{"entities":[{"text":"John Smith","start":0,"end":10,"length":10,"kind":"person","strength":0.81}]}')
          throw StateError('plain recognize exact entity');
      } finally {
        freeString(result.value);
        result.value = Pointer<Uint8>.fromAddress(0);
      }
      both[0] = first;
      both[1] = second;
      lengths[0] = utf8.encode('{"name":"First","kind":"alert"}').length;
      lengths[1] = utf8.encode('{"name":"Second","kind":"alert"}').length;
      if (relatePlain(engine, relation, both, lengths, 2, result, resultLen) !=
              0 ||
          result.value.address == 0) throw StateError('plain relate');
      try {
        final edges =
            (jsonDecode(utf8.decode(result.value.asTypedList(resultLen.value)))
                as Map)['edges'] as List;
        if (edges.length != 2 ||
            jsonEncode(edges[0]) !=
                '{"relation":"caused_by","source":{"name":"First","kind":"alert"},"target":{"name":"Second","kind":"alert"},"probability":0.9}' ||
            jsonEncode(edges[1]) !=
                '{"relation":"caused_by","source":{"name":"Second","kind":"alert"},"target":{"name":"First","kind":"alert"},"probability":0.9}')
          throw StateError('plain relate exact edges');
      } finally {
        freeString(result.value);
        result.value = Pointer<Uint8>.fromAddress(0);
      }
    } finally {
      for (final pointer in [
        question,
        text,
        answer,
        ptrs,
        lens,
        result,
        resultLen,
        usage,
        spec,
        person,
        relation,
        first,
        second,
        both,
        lengths,
      ]) {
        memory.release(pointer);
      }
    }
  }

  Pointer<Void> create([String? settings]) {
    if (settings == null) return _created(engineNew());
    final s = memory.cString(settings);
    try {
      return _created(engineNewWith(s));
    } finally {
      memory.release(s);
    }
  }

  Pointer<Void> _created(Pointer<Void> engine) {
    if (engine.address == 0) throw failure(engine, 1);
    return engine;
  }

  DoorFailure failure(Pointer<Void> engine, int code) {
    final recorded = errorCode(engine);
    if (recorded != code)
      throw StateError('error slot $recorded != return $code');
    final factsPointer = errorFacts(engine);
    final facts = factsPointer.address == 0
        ? null
        : jsonDecode(memory.decodeCString(factsPointer))
            as Map<String, dynamic>;
    return DoorFailure(
      code,
      memory.decodeCString(errorMessage(engine)),
      errorRetryable(engine) != 0,
      facts,
    );
  }

  CallFacts _readFacts(Pointer<Uint8> pointer, int length) {
    if (pointer.address == 0)
      throw StateError('successful call returned null facts');
    return CallFacts.parse(
      jsonDecode(utf8.decode(pointer.asTypedList(length))),
    );
  }

  CallResult<AnswerValue> decide(
    Pointer<Void> engine,
    String question,
    String text, {
    int deadline = -1,
    Pointer<Void>? token,
  }) {
    final owned = _OwnedPointers();
    try {
      final q = owned.add(memory.cString(question));
      final t = owned.add(memory.cString(text));
      final out = owned.add(memory.allocate(sizeOf<Answer>()).cast<Answer>());
      final facts = owned.add(
        memory.allocate(sizeOf<Pointer<Uint8>>()).cast<Pointer<Uint8>>(),
      );
      final factsLen = owned.add(
        memory.allocate(sizeOf<IntPtr>()).cast<IntPtr>(),
      );
      out.ref.outcome = 123;
      out.ref.probability = -1;
      facts.value = nullptr;
      factsLen.value = 999;
      final code = decideOpts(
        engine,
        q,
        t,
        utf8.encode(text).length,
        deadline,
        token ?? nullptr,
        out,
        facts,
        factsLen,
      );
      try {
        if (code != 0) {
          if (out.ref.outcome != 123 ||
              out.ref.probability != -1 ||
              facts.value.address != 0 ||
              factsLen.value != 999)
            throw StateError('failure modified scalar outputs');
          throw failure(engine, code);
        }
        return CallResult(
          AnswerValue(out.ref.outcome, out.ref.probability),
          _readFacts(facts.value, factsLen.value),
        );
      } finally {
        if (facts.value.address != 0) freeString(facts.value);
      }
    } finally {
      owned.releaseAll();
    }
  }

  CallResult<List<AnswerValue>> many(
    Pointer<Void> engine,
    String question,
    List<String> texts, {
    int deadline = -1,
    Pointer<Void>? token,
  }) {
    final owned = _OwnedPointers();
    try {
      final q = owned.add(memory.cString(question));
      final encoded = <Pointer<Uint8>>[];
      for (final text in texts) {
        encoded.add(owned.add(memory.cString(text)));
      }
      final ptrs = owned.add(
        memory
            .allocate(sizeOf<Pointer<Uint8>>() * texts.length)
            .cast<Pointer<Uint8>>(),
      );
      final lens = owned.add(
        memory.allocate(sizeOf<IntPtr>() * texts.length).cast<IntPtr>(),
      );
      final out = owned.add(
        memory.allocate(sizeOf<Answer>() * texts.length).cast<Answer>(),
      );
      final facts = owned.add(
        memory.allocate(sizeOf<Pointer<Uint8>>()).cast<Pointer<Uint8>>(),
      );
      final factsLen = owned.add(
        memory.allocate(sizeOf<IntPtr>()).cast<IntPtr>(),
      );
      facts.value = nullptr;
      factsLen.value = 999;
      for (var i = 0; i < texts.length; i++) {
        ptrs[i] = encoded[i];
        lens[i] = utf8.encode(texts[i]).length;
        out[i].outcome = 123;
        out[i].probability = -1;
      }
      final code = manyOpts(
        engine,
        q,
        ptrs,
        lens,
        texts.length,
        deadline,
        token ?? nullptr,
        out,
        facts,
        factsLen,
      );
      try {
        if (code != 0) {
          for (var i = 0; i < texts.length; i++) {
            if (out[i].outcome != 123 || out[i].probability != -1)
              throw StateError('failure modified bulk output');
          }
          if (facts.value.address != 0 || factsLen.value != 999)
            throw StateError('failure modified bulk facts output');
          throw failure(engine, code);
        }
        return CallResult([
          for (var i = 0; i < texts.length; i++)
            AnswerValue(out[i].outcome, out[i].probability),
        ], _readFacts(facts.value, factsLen.value));
      } finally {
        if (facts.value.address != 0) freeString(facts.value);
      }
    } finally {
      owned.releaseAll();
    }
  }

  /// Carries description maps and structured {what,not_for,examples} unchanged.
  Object? ask(Pointer<Void> engine, Map<String, Object?> request) =>
      call(engine, jsonEncode(request));
  Annotation annotate(Pointer<Void> engine, Map<String, Object?> request) {
    final envelope = ask(engine, request) as Map;
    return Annotation.parse(envelope['value']);
  }

  CallResult<Recognition> recognize(
    Pointer<Void> engine,
    String spec,
    String text,
  ) {
    final result = structured(engine, spec, [text], recognize: true);
    return CallResult(Recognition.parse(result.value), result.facts);
  }

  CallResult<Relations> relate(
    Pointer<Void> engine,
    String spec,
    List<String> records,
  ) {
    final result = structured(engine, spec, records, recognize: false);
    return CallResult(Relations.parse(result.value), result.facts);
  }

  Object? call(Pointer<Void> engine, String jsonText) {
    final text = memory.cString(jsonText);
    try {
      final result = callOpts(engine, text, -1, nullptr);
      if (result.address == 0) throw failure(engine, errorCode(engine));
      try {
        return jsonDecode(memory.decodeCString(result));
      } finally {
        freeString(result);
      }
    } finally {
      memory.release(text);
    }
  }

  CallResult<Object?> structured(
    Pointer<Void> engine,
    String spec,
    List<String> texts, {
    required bool recognize,
  }) {
    final owned = _OwnedPointers();
    try {
      final s = owned.add(memory.cString(spec));
      final values = <Pointer<Uint8>>[];
      for (final text in texts) {
        values.add(owned.add(memory.cString(text)));
      }
      final ptrs = owned.add(
        memory
            .allocate(sizeOf<Pointer<Uint8>>() * values.length)
            .cast<Pointer<Uint8>>(),
      );
      final lens = owned.add(
        memory.allocate(sizeOf<IntPtr>() * values.length).cast<IntPtr>(),
      );
      final out = owned.add(
        memory.allocate(sizeOf<Pointer<Uint8>>()).cast<Pointer<Uint8>>(),
      );
      final outLen = owned.add(
        memory.allocate(sizeOf<IntPtr>()).cast<IntPtr>(),
      );
      final facts = owned.add(
        memory.allocate(sizeOf<Pointer<Uint8>>()).cast<Pointer<Uint8>>(),
      );
      final factsLen = owned.add(
        memory.allocate(sizeOf<IntPtr>()).cast<IntPtr>(),
      );
      for (var i = 0; i < values.length; i++) {
        ptrs[i] = values[i];
        lens[i] = utf8.encode(texts[i]).length;
      }
      out.value = Pointer<Uint8>.fromAddress(0);
      outLen.value = 999;
      facts.value = nullptr;
      factsLen.value = 999;
      final code = recognize
          ? recognizeOpts(
              engine,
              s,
              values.single,
              lens[0],
              -1,
              nullptr,
              out,
              outLen,
              facts,
              factsLen,
            )
          : relateOpts(
              engine,
              s,
              ptrs,
              lens,
              values.length,
              -1,
              nullptr,
              out,
              outLen,
              facts,
              factsLen,
            );
      try {
        if (code != 0) {
          if (out.value.address != 0 ||
              outLen.value != 999 ||
              facts.value.address != 0 ||
              factsLen.value != 999)
            throw StateError('failure changed JSON outputs');
          throw failure(engine, code);
        }
        if (out.value.address == 0)
          throw StateError('successful JSON call returned null');
        return CallResult(
          jsonDecode(utf8.decode(out.value.asTypedList(outLen.value))),
          _readFacts(facts.value, factsLen.value),
        );
      } finally {
        if (out.value.address != 0) freeString(out.value);
        if (facts.value.address != 0) freeString(facts.value);
      }
    } finally {
      owned.releaseAll();
    }
  }
}

final class AnswerValue {
  final int rawOutcome;
  final double probability;
  AnswerValue(this.rawOutcome, this.probability);
  Outcome get outcome => Outcome.values[rawOutcome];
  @override
  String toString() => '$outcome/$probability';
}
