part of thinkthen_session;

final class CallFailure implements Exception {
  final ErrorKind kind;
  final bool retryable;
  final String message;
  final FailureFacts? facts;
  final OwnedCall? call;
  final CallError? failure;
  CallFailure._native(int code, this.retryable, this.message, this.facts, {this.call, this.failure})
      : kind = ErrorKind.values.firstWhere((kind) => kind.code == code, orElse: () => ErrorKind.defect);
  factory CallFailure._result(CallError failure, OwnedCall call) {
    final kind = ErrorKind.values.firstWhere((kind) => kind.name == failure.error.kind, orElse: () => ErrorKind.defect);
    final facts = failure.facts.value;
    return CallFailure._native(kind.code, failure.error.retryable, failure.error.message,
        facts == null ? null : FailureFacts.read(facts._fields), call: call, failure: failure);
  }
  @override
  String toString() => message;
}

final class _NativeOwner implements Finalizable {
  final NativeFinalizer _finalizer;
  Pointer<Void> pointer;
  _NativeOwner(
    Pointer<NativeFunction<Void Function(Pointer<Void>)>> free,
    this.pointer,
  ) : _finalizer = NativeFinalizer(free) {
    _finalizer.attach(this, pointer, detach: this);
  }
  void close() {
    if (pointer == nullptr) return;
    _finalizer.detach(this);
    final pointerToFree = pointer;
    pointer = nullptr;
    _release(pointerToFree);
  }

  late final void Function(Pointer<Void>) _release;
  void setRelease(void Function(Pointer<Void>) release) => _release = release;
  Pointer<Void> get live {
    if (pointer == nullptr) throw StateError('Native owner is closed');
    return pointer;
  }
}

T _withBytes<T>(Object? value, T Function(Pointer<Uint8>, int) body) {
  return _withText(_encodeNative(value), body);
}

T _withText<T>(String text, T Function(Pointer<Uint8>, int) body) {
  final bytes = utf8.encode(text);
  final pointer = calloc<Uint8>(bytes.length + 1);
  try {
    pointer.asTypedList(bytes.length).setAll(0, bytes);
    return body(pointer, bytes.length);
  } finally {
    calloc.free(pointer);
  }
}

void _checkSession(_NativeAbi abi, int code) {
  if (code != 0) throw _NativePacketReader.failure(abi.thinkthen_session_error_view(), code);
}
