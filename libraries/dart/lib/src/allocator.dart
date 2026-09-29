import 'dart:convert';
import 'dart:ffi';

import 'package:ffi/ffi.dart';

// Borrowed engine strings are never passed here: only our own allocations are freed.
final Set<int> _owned = {};
Pointer<Uint8> allocate(int length) {
  if (length < 0) throw ArgumentError('negative allocation');
  final pointer = calloc<Uint8>(length == 0 ? 1 : length);
  if (pointer.address == 0) throw StateError('null allocation');
  _owned.add(pointer.address);
  return pointer;
}

void release(Pointer pointer) {
  if (pointer.address == 0 || !_owned.remove(pointer.address)) {
    throw StateError('unowned or double-free pointer');
  }
  calloc.free(pointer);
}

int get liveAllocations => _owned.length;
Pointer<Uint8> cString(String text) {
  final bytes = utf8.encode(text);
  if (bytes.contains(0)) throw ArgumentError('embedded NUL');
  final pointer = allocate(bytes.length + 1);
  final target = pointer.asTypedList(bytes.length + 1);
  target.setRange(0, bytes.length, bytes);
  target[bytes.length] = 0;
  return pointer;
}

String decodeCString(Pointer<Uint8> pointer) {
  if (pointer.address == 0) throw StateError('null string');
  final values = <int>[];
  for (var i = 0; pointer[i] != 0; i++) {
    if (i >= 10000000) throw StateError('unbounded string');
    values.add(pointer[i]);
  }
  return utf8.decode(values);
}

void allocatorNegatives() {
  try {
    allocate(-1);
    throw StateError('negative accepted');
  } on ArgumentError {}
  try {
    release(Pointer<Void>.fromAddress(0));
    throw StateError('null accepted');
  } on StateError catch (e) {
    if (e.message == 'null accepted') rethrow;
  }
  final unowned = Pointer<Void>.fromAddress(0x10);
  try {
    release(unowned);
    throw StateError('unowned accepted');
  } on StateError catch (e) {
    if (e.message != 'unowned or double-free pointer') rethrow;
  }
  final pointer = allocate(4);
  release(pointer);
  try {
    release(pointer);
    throw StateError('double free accepted');
  } on StateError catch (e) {
    if (e.message == 'double free accepted') rethrow;
  }
  if (liveAllocations != 0) throw StateError('allocator leaked');
}
