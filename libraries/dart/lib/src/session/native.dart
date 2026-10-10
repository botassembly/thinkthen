import 'dart:convert';
import 'dart:ffi';
import 'package:ffi/ffi.dart';
export 'package:ffi/ffi.dart';
import 'abi_generated.dart';
import 'values.dart';

final class NativeFailure implements Exception {
  final NativeErrorKind kind;
  final bool retryable;
  final String message;
  final Object? facts;
  NativeFailure(int code, this.retryable, this.message, this.facts)
      : kind = NativeErrorKind.values.firstWhere((kind) => kind.code == code);
  @override
  String toString() => message;
}

final class NativeOwner implements Finalizable {
  final NativeFinalizer _finalizer;
  Pointer<Void> pointer;
  NativeOwner(DynamicLibrary library, String freeSymbol, this.pointer)
      : _finalizer = NativeFinalizer(library.lookup(freeSymbol)) {
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

T withBytes<T>(Object? value, T Function(Pointer<Uint8>, int) body) {
  final bytes = utf8.encode(encodeNative(value));
  final pointer = calloc<Uint8>(bytes.length + 1);
  try {
    pointer.asTypedList(bytes.length).setAll(0, bytes);
    return body(pointer, bytes.length);
  } finally {
    calloc.free(pointer);
  }
}

String nativeText(Pointer<Uint8> pointer) =>
    pointer == nullptr ? '' : pointer.cast<Utf8>().toDartString();
void checkSession(NativeAbi abi, int code) {
  if (code != 0)
    throw NativeFailure(
        code, false, nativeText(abi.thinkthen_session_error_message()), null);
}
