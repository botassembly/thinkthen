part of thinkthen_session;

/// Convert Dart bytes to the generated wire descriptor; Rust admits format and size.
InputRequestImageBytes imageBytes(Uint8List bytes, {required String media}) =>
    InputRequestImageBytes(bytes: base64Encode(bytes), media: media);

/// Explicit cancellation stops intake immediately; terminal facts may settle later.
final class Cancellation {
  bool cancelled = false;
  final Set<void Function()> _callbacks = {};
  void cancel() {
    if (cancelled) return;
    cancelled = true;
    for (final callback in _callbacks.toList()) callback();
  }

  void _check() {
    if (cancelled) throw const CallCancelled();
  }
}

final class CallCancelled implements Exception {
  const CallCancelled();
}

final class StreamCleanupFailure implements Exception {
  final Object cause;
  StreamCleanupFailure(this.cause);
  @override
  String toString() => 'Input stream cleanup failed: $cause';
}

final class OwnedCall {
  final List<SessionPacket> packets;
  final SessionPacketTerminal terminal;
  OwnedCall(List<SessionPacket> packets, this.terminal)
      : packets = List.unmodifiable(packets);
}

final class SessionFailure implements Exception {
  final CallError failure;
  final OwnedCall call;
  SessionFailure(this.failure, this.call);
  @override
  String toString() => failure.error.message;
}

/// Polling uses the native nonblocking operations and yields to the Dart event loop.
final class OwnedSession implements Finalizable {
  final NativeAbi _abi;
  final NativeOwner _owner;
  bool _reading = false, _pushing = false;
  OwnedSession._(this._abi, Pointer<Void> pointer)
      : _owner = NativeOwner(_abi.thinkthen_session_freePointer, pointer) {
    _owner.setRelease(_abi.thinkthen_session_free);
  }
  void cancel() {
    if (_owner.pointer != nullptr)
      _abi.thinkthen_session_cancel(_owner.pointer);
  }

  void close() {
    cancel();
    _owner.close();
  }

  void finish() => checkSession(
      _abi, _abi.thinkthen_session_finish(_owner.live, nullptr, 0));
  Future<bool> push(InputRequestSessionDescriptor descriptor) async {
    if (_pushing) throw StateError('A session allows one producer');
    _pushing = true;
    try {
      while (true) {
        final status = calloc<Uint32>();
        try {
          withBytes(
              descriptor,
              (bytes, length) => checkSession(
                  _abi,
                  _abi.thinkthen_session_try_push(
                      _owner.live, bytes, length, status)));
          if (status.value == THINKTHEN_SESSION_ACCEPTED_V1) return true;
          if (status.value == THINKTHEN_SESSION_CLOSED_V1) return false;
          if (status.value != THINKTHEN_SESSION_FULL_V1)
            throw StateError('Invalid native push status');
        } finally {
          calloc.free(status);
        }
        await Future<void>.delayed(const Duration(milliseconds: 10));
      }
    } finally {
      _pushing = false;
    }
  }

  Future<SessionPacket?> read() async {
    if (_reading) throw StateError('A session allows one reader');
    _reading = true;
    try {
      while (true) {
        final status = calloc<Uint32>();
        final output = calloc<Pointer<Void>>();
        try {
          checkSession(_abi,
              _abi.thinkthen_session_try_read(_owner.live, status, output));
          if (status.value == THINKTHEN_SESSION_END_V1) return null;
          if (status.value == THINKTHEN_SESSION_RESULT_V1) {
            final packet = output.value;
            final json = calloc<Pointer<Void>>();
            final length = calloc<Size>();
            try {
              checkSession(_abi,
                  _abi.thinkthen_session_result_json(packet, json, length));
              final source = utf8
                  .decode(json.value.cast<Uint8>().asTypedList(length.value));
              return SessionPacket.read(decodeNative(source));
            } finally {
              _abi.thinkthen_session_result_free(packet);
              calloc.free(json);
              calloc.free(length);
            }
          }
          if (status.value != THINKTHEN_SESSION_PENDING_V1)
            throw StateError('Invalid native read status');
        } finally {
          calloc.free(status);
          calloc.free(output);
        }
        await Future<void>.delayed(const Duration(milliseconds: 10));
      }
    } finally {
      _reading = false;
    }
  }

  Stream<SessionPacket> get packets async* {
    while (true) {
      final packet = await read();
      if (packet == null) return;
      yield packet;
    }
  }
}

/// An owned snapshot of this engine's count persistence.
final class UsagePersistenceStatus {
  final UsagePersistenceState state;
  final String? advice;
  const UsagePersistenceStatus(this.state, this.advice);
}

final class Engine implements Finalizable {
  final NativeAbi _abi;
  final NativeOwner _owner;
  Engine._(this._abi, Pointer<Void> pointer)
      : _owner = NativeOwner(_abi.thinkthen_engine_freePointer, pointer) {
    _owner.setRelease(_abi.thinkthen_engine_free);
  }

  /// Load the SDK-bundled native asset.
  factory Engine.open({InputEngineSettings? settings}) {
    final abi = NativeAbi();
    final pointer = withBytes(settings ?? InputEngineSettings(),
        (bytes, _) => abi.thinkthen_engine_new_with(bytes));
    if (pointer == nullptr) {
      throw NativeFailure(
          abi.thinkthen_error_code(pointer),
          abi.thinkthen_error_retryable(pointer) != 0,
          nativeText(abi.thinkthen_error_message(pointer)),
          abi.thinkthen_error_facts_json(pointer) == nullptr
              ? null
              : decodeNative(
                  nativeText(abi.thinkthen_error_facts_json(pointer))));
    }
    return Engine._(abi, pointer);
  }
  void close() => _owner.close();
  /// Observe without waiting for the usage writer.
  UsagePersistenceStatus usagePersistence() => _usageStatus(false);

  /// Finish current deltas. Only usage-lock acquisition has a deadline.
  UsagePersistenceStatus finishUsageStatus() => _usageStatus(true);

  UsagePersistenceStatus _usageStatus(bool finish) {
    final engine = _owner.live;
    final state = calloc<NativeUsageState>();
    final advice = calloc<NativeUsageAdvice>();
    try {
      final operation = finish
          ? _abi.thinkthen_engine_finish_usage_status_v1
          : _abi.thinkthen_engine_usage_persistence_v1;
      checkSession(_abi, operation(engine, state, advice));
      return UsagePersistenceStatus(
        UsagePersistenceState.values.firstWhere(
          (value) => value.code == state.ref.kind,
        ),
        advice.ref.data == nullptr
            ? null
            : utf8.decode(advice.ref.data.asTypedList(advice.ref.len)),
      );
    } finally {
      calloc.free(state);
      calloc.free(advice);
    }
  }

  OwnedSession startSession(
      InputRequestQuestion question, InputRequestInput input, String function,
      {InputRequestOptions? options}) {
    final output = calloc<Pointer<Void>>();
    final surface = _surfaceToken.toNativeUtf8();
    try {
      withBytes({
        'schema': requestVersion,
        'call': {
          'function': function,
          'question': question,
          'input': input,
          if (options != null) 'options': options
        }
      }, (bytes, length) {
        checkSession(
            _abi,
            _abi.thinkthen_session_new_with_surface(_owner.live, bytes, length,
                surface.cast<Uint8>(), _surfaceToken.length, output));
      });
      return OwnedSession._(_abi, output.value);
    } finally {
      calloc.free(output);
      calloc.free(surface);
    }
  }

  Future<OwnedCall> _call(
      String function,
      InputRequestQuestion question,
      InputRequestInput input,
      InputRequestOptions? options,
      Cancellation? cancellation,
      Stream<InputRequestSessionDescriptor>? feed) async {
    cancellation?._check();
    final session = startSession(question, input, function, options: options);
    StreamSubscription<InputRequestSessionDescriptor>? subscription;
    Object? readerFailure;
    var failed = false;
    void stop() {
      session.cancel();
      session.close();
    }

    cancellation?._callbacks.add(stop);
    try {
      if (feed == null) {
        session.finish();
      } else {
        subscription = feed.listen((descriptor) async {
          subscription!.pause();
          try {
            if (!await session.push(descriptor))
              await subscription?.cancel();
            else
              subscription?.resume();
          } catch (error) {
            readerFailure = error;
            session.close();
          }
        }, onDone: () {
          if (session._owner.pointer != nullptr) session.finish();
        }, onError: (Object error) {
          readerFailure = error;
          session.close();
        });
      }
      final packets = <SessionPacket>[];
      SessionPacketTerminal? terminal;
      while (true) {
        cancellation?._check();
        final packet = await session.read();
        if (packet == null) break;
        packets.add(packet);
        if (packet is SessionPacketTerminal) terminal = packet;
      }
      if (readerFailure != null) throw readerFailure!;
      cancellation?._check();
      final call = OwnedCall(packets,
          terminal ?? (throw StateError('Native End has no terminal')));
      if (call.terminal.failure.value case final CallError error)
        throw SessionFailure(error, call);
      return call;
    } catch (_) {
      failed = true;
      cancellation?._check();
      if (readerFailure != null) throw readerFailure!;
      rethrow;
    } finally {
      cancellation?._callbacks.remove(stop);
      try {
        await subscription?.cancel();
      } catch (error) {
        if (!failed) throw StreamCleanupFailure(error);
      } finally {
        session.close();
      }
    }
  }

  Future<OwnedCall> decide(
          InputRequestQuestion question, InputRequestInput input,
          {InputRequestOptions? options,
          Cancellation? cancellation,
          Stream<InputRequestSessionDescriptor>? feed}) =>
      _call("decide", question, input, options, cancellation, feed);
  Future<OwnedCall> choose(
          InputRequestQuestion question, InputRequestInput input,
          {InputRequestOptions? options,
          Cancellation? cancellation,
          Stream<InputRequestSessionDescriptor>? feed}) =>
      _call("choose", question, input, options, cancellation, feed);
  Future<OwnedCall> tag(InputRequestQuestion question, InputRequestInput input,
          {InputRequestOptions? options,
          Cancellation? cancellation,
          Stream<InputRequestSessionDescriptor>? feed}) =>
      _call("tag", question, input, options, cancellation, feed);
  Future<OwnedCall> score(
          InputRequestQuestion question, InputRequestInput input,
          {InputRequestOptions? options,
          Cancellation? cancellation,
          Stream<InputRequestSessionDescriptor>? feed}) =>
      _call("score", question, input, options, cancellation, feed);
  Future<OwnedCall> filter(
          InputRequestQuestion question, InputRequestInput input,
          {InputRequestOptions? options,
          Cancellation? cancellation,
          Stream<InputRequestSessionDescriptor>? feed}) =>
      _call("filter", question, input, options, cancellation, feed);
  Future<OwnedCall> rank(InputRequestQuestion question, InputRequestInput input,
          {InputRequestOptions? options,
          Cancellation? cancellation,
          Stream<InputRequestSessionDescriptor>? feed}) =>
      _call("rank", question, input, options, cancellation, feed);
  Future<OwnedCall> find(InputRequestQuestion question, InputRequestInput input,
          {InputRequestOptions? options,
          Cancellation? cancellation,
          Stream<InputRequestSessionDescriptor>? feed}) =>
      _call("find", question, input, options, cancellation, feed);
  Future<OwnedCall> annotate(
          InputRequestQuestion question, InputRequestInput input,
          {InputRequestOptions? options,
          Cancellation? cancellation,
          Stream<InputRequestSessionDescriptor>? feed}) =>
      _call("annotate", question, input, options, cancellation, feed);
  Future<OwnedCall> recognize(
          InputRequestQuestion question, InputRequestInput input,
          {InputRequestOptions? options,
          Cancellation? cancellation,
          Stream<InputRequestSessionDescriptor>? feed}) =>
      _call("recognize", question, input, options, cancellation, feed);
  Future<OwnedCall> relate(
          InputRequestQuestion question, InputRequestInput input,
          {InputRequestOptions? options,
          Cancellation? cancellation,
          Stream<InputRequestSessionDescriptor>? feed}) =>
      _call("relate", question, input, options, cancellation, feed);
}
