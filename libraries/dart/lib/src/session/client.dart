part of thinkthen_session;

/// Convert Dart bytes to the generated wire descriptor; Rust admits format and size.
ImageBytes imageBytes(Uint8List bytes, {required String media}) =>
    ImageBytes(bytes: base64Encode(bytes), media: media);

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

final class OwnedCall with CallAnswers {
  final List<SessionPacket> packets;
  final SessionPacketTerminal terminal;
  OwnedCall(List<SessionPacket> packets, this.terminal)
    : packets = List.unmodifiable(packets);
}

/// Native waits run in a helper isolate while the calling isolate stays responsive.
final class OwnedSession implements Finalizable {
  final _NativeAbi _abi;
  final _NativeOwner _owner;
  final _NativeWaitPool _pool;
  bool _reading = false, _pushing = false, _closed = false;
  int _waiters = 0;
  final Map<bool, Future<_NativeWaiter>> _helpers = {};
  OwnedSession._(this._abi, this._pool, Pointer<Void> pointer)
    : _owner = _NativeOwner(_abi.thinkthen_session_freePointer, pointer) {
    _owner.setRelease(_abi.thinkthen_session_free);
  }
  void cancel() {
    if (_owner.pointer != nullptr)
      _abi.thinkthen_session_cancel(_owner.pointer);
  }

  void close() {
    cancel();
    _closed = true;
    if (_waiters == 0) _release();
  }

  Pointer<Void> get _live {
    if (_closed) throw StateError('Native owner is closed');
    return _owner.live;
  }

  Future<int> _nativeRequest(bool input, Object? payload) async {
    final address = _live.address;
    _waiters++;
    try {
      final helper = await _helpers.putIfAbsent(
        input,
        () => _pool.acquire(input),
      );
      return await helper.request(address, input, payload);
    } finally {
      _waiters--;
      if (_closed && _waiters == 0) _release();
    }
  }

  void _release() {
    _owner.close();
    for (final helper in _helpers.values) {
      unawaited(
        helper.then((value) => _pool.release(value), onError: (Object _) {}),
      );
    }
    _helpers.clear();
  }

  void finish({ReaderFailure? failure}) {
    if (failure == null) {
      _checkSession(_abi, _abi.thinkthen_session_finish(_live, nullptr, 0));
    } else {
      _withBytes(
        failure,
        (bytes, length) => _checkSession(
          _abi,
          _abi.thinkthen_session_finish(_live, bytes, length),
        ),
      );
    }
  }

  Future<bool> push(FeedItem descriptor) async {
    if (_pushing) throw StateError('A session allows one producer');
    _pushing = true;
    try {
      return await _nativeRequest(true, _encodeNative(descriptor)) ==
          THINKTHEN_SESSION_ACCEPTED_V1;
    } finally {
      _pushing = false;
    }
  }

  Future<SessionPacket?> read() async {
    if (_reading) throw StateError('A session allows one reader');
    _reading = true;
    try {
      final address = await _nativeRequest(false, null);
      if (address == 0) return null;
      return _abi.takePacket(Pointer<Void>.fromAddress(address));
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

final class Engine with NamedCalls implements Finalizable {
  final _NativeAbi _abi;
  final _NativeOwner _owner;
  final _NativeWaitPool _pool = _NativeWaitPool();
  static final Finalizer<_NativeWaitPool> _poolFinalizer = Finalizer(
    (pool) => pool.close(),
  );
  Engine._(this._abi, Pointer<Void> pointer)
    : _owner = _NativeOwner(_abi.thinkthen_engine_freePointer, pointer) {
    _owner.setRelease(_abi.thinkthen_engine_free);
    _poolFinalizer.attach(this, _pool, detach: this);
  }

  /// Load the SDK-bundled native asset.
  factory Engine.open({EngineSettings? settings}) {
    final abi = _NativeAbi();
    final pointer = _withBytes(
      settings ?? EngineSettings(),
      (bytes, _) => abi.thinkthen_engine_new_with(bytes),
    );
    if (pointer == nullptr) {
      throw _NativePacketReader.failure(
        abi.thinkthen_error_view(pointer),
        abi.thinkthen_error_code(pointer),
      );
    }
    return Engine._(abi, pointer);
  }
  void close() {
    _poolFinalizer.detach(this);
    _pool.close();
    _owner.close();
  }

  /// Observe without waiting for the usage writer.
  UsagePersistenceStatus usagePersistence() =>
      _abi.usagePersistence(_owner.live);

  /// Finish current deltas. Only usage-lock acquisition has a deadline.
  UsagePersistenceStatus finishUsageStatus() =>
      _abi.finishUsageStatus(_owner.live);

  OwnedSession _startSession(
    Question question,
    Input input,
    String function, {
    Options? options,
  }) => OwnedSession._(
    _abi,
    _pool,
    _abi.startNamed(
      _owner.live,
      function,
      question,
      input,
      options,
      _surfaceToken,
    ),
  );

  Future<OwnedCall> _call(
    String function,
    Question question,
    Input input,
    Options? options,
    Cancellation? cancellation,
    Stream<FeedItem>? feed,
  ) async {
    cancellation?._check();
    final session = _startSession(question, input, function, options: options);
    StreamSubscription<FeedItem>? subscription;
    Object? readerFailure;
    var failed = false, readerFinished = false;
    final interrupted = Completer<SessionPacket?>();
    void stop() {
      session.close();
      if (!interrupted.isCompleted)
        interrupted.completeError(const CallCancelled());
    }

    void failReader(Object error) {
      if (readerFinished || cancellation?.cancelled == true) return;
      readerFinished = true;
      final failure = error is ReaderFailure
          ? error
          : error is IOException
          ? ReaderFailureIo()
          : error is FormatException
          ? ReaderFailureInvalidInput()
          : null;
      if (failure != null) {
        try {
          session.finish(failure: failure);
        } catch (error) {
          readerFailure = error;
          session.close();
        }
      } else {
        readerFailure = error;
        session.close();
      }
    }

    cancellation?._callbacks.add(stop);
    try {
      if (feed == null) {
        session.finish();
      } else {
        subscription = feed.listen(
          (descriptor) async {
            if (readerFinished ||
                cancellation?.cancelled == true ||
                session._owner.pointer == nullptr)
              return;
            subscription!.pause();
            try {
              if (!await session.push(descriptor))
                await subscription?.cancel();
              else if (!readerFinished &&
                  cancellation?.cancelled != true &&
                  session._owner.pointer != nullptr)
                subscription?.resume();
            } catch (error) {
              failReader(error);
            }
          },
          onDone: () {
            if (!readerFinished && session._owner.pointer != nullptr) {
              try {
                session.finish();
                readerFinished = true;
              } catch (error) {
                failReader(error);
              }
            }
          },
          onError: (Object error) {
            failReader(error);
          },
        );
      }
      final packets = <SessionPacket>[];
      SessionPacketTerminal? terminal;
      while (true) {
        cancellation?._check();
        final packet = await Future.any([session.read(), interrupted.future]);
        if (packet == null) break;
        packets.add(packet);
        if (packet is SessionPacketTerminal) terminal = packet;
      }
      if (readerFailure != null) throw readerFailure!;
      cancellation?._check();
      final call = OwnedCall(
        packets,
        terminal ?? (throw StateError('Native End has no terminal')),
      );
      if (call.terminal.failure.value case final CallError error)
        throw CallFailure._result(error, call);
      return call;
    } catch (_) {
      failed = true;
      cancellation?._check();
      if (readerFailure != null) throw readerFailure!;
      rethrow;
    } finally {
      cancellation?._callbacks.remove(stop);
      session.close();
      if (cancellation?.cancelled == true) {
        // Host cleanup cannot delay cancellation or retain native ownership.
        try {
          final cleanup = subscription?.cancel();
          if (cleanup != null) unawaited(cleanup.catchError((Object _) {}));
        } catch (_) {}
      } else {
        try {
          await subscription?.cancel();
        } catch (error) {
          if (!failed) throw StreamCleanupFailure(error);
        }
      }
    }
  }
}

/// Helpers remain available across calls; concurrent sessions receive distinct leases.
final class _NativeWaitPool {
  final Map<bool, List<_NativeWaiter>> _idle = {false: [], true: []};
  bool _closed = false;
  Future<_NativeWaiter> acquire(bool input) async {
    final available = _idle[input]!;
    final helper = available.isEmpty
        ? await _NativeWaiter.open()
        : available.removeLast();
    helper.input = input;
    return helper;
  }

  void release(_NativeWaiter helper) {
    if (_closed)
      helper.close();
    else
      _idle[helper.input]!.add(helper);
  }

  void close() {
    _closed = true;
    for (final available in _idle.values) {
      for (final helper in available) helper.close();
      available.clear();
    }
  }
}

/// Each session retains a reader helper and, for feeds, a producer helper.
/// Separate helpers let a blocked producer and reader make progress together.
final class _NativeWaiter {
  final ReceivePort _replies;
  final SendPort _requests;
  final Map<int, Completer<int>> _pending = {};
  late final StreamSubscription<dynamic> _subscription;
  int _next = 0;
  bool input = false;
  _NativeWaiter._(this._replies, this._requests) {
    _subscription = _replies.listen((dynamic reply) {
      final message = reply as List<Object?>;
      final pending = _pending.remove(message[0] as int)!;
      if (message[1] == null) {
        pending.complete(message[2] as int);
      } else {
        pending.completeError(message[1]!);
      }
    });
  }
  static Future<_NativeWaiter> open() async {
    final handshake = ReceivePort();
    final replies = ReceivePort();
    try {
      await Isolate.spawn(_nativeWaitLoop, [
        handshake.sendPort,
        replies.sendPort,
      ]);
      final requests = await handshake.first as SendPort;
      return _NativeWaiter._(replies, requests);
    } catch (_) {
      replies.close();
      rethrow;
    } finally {
      handshake.close();
    }
  }

  Future<int> request(int address, bool input, Object? payload) {
    final id = _next++;
    final pending = Completer<int>();
    _pending[id] = pending;
    _requests.send([id, address, input, payload]);
    return pending.future;
  }

  void close() {
    _requests.send(null);
    unawaited(_subscription.cancel());
    _replies.close();
  }
}

void _nativeWaitLoop(List<SendPort> ports) async {
  final requests = ReceivePort();
  ports[0].send(requests.sendPort);
  final abi = _NativeAbi();
  try {
    await for (final dynamic request in requests) {
      if (request == null) break;
      final message = request as List<Object?>;
      Object? failure;
      var result = 0;
      try {
        final session = Pointer<Void>.fromAddress(message[1] as int);
        result = message[2] == true
            ? abi.pushItem(session, message[3] as String)
            : abi.waitPacket(session);
      } catch (error) {
        failure = error;
      }
      ports[1].send([message[0], failure, result]);
    }
  } finally {
    requests.close();
  }
}
