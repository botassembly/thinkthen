part of 'engine.dart';

/// Owned native batch; pulling, packing, cache and scheduling stay in Rust.
final class Batch<T> {
  final Engine _owner;
  final Pointer<Void> _pointer;
  final String _verb;
  final T Function(Pointer<Void>, int) _read;
  bool _closed = false;
  Batch._(this._owner, this._pointer, this._verb, this._read);
  void _live() {
    if (_closed) throw StateError('batch closed');
  }

  CompleteResult<T>? next() {
    _live();
    final out = calloc<Pointer<Void>>();
    try {
      _owner._check(_owner._api.thinkthen_batch_next(_pointer, out));
      return out.value.address == 0
          ? null
          : _owner._copy(out.value, _verb, _read);
    } finally {
      _owner._api.thinkthen_result_free(out.value);
      calloc.free(out);
    }
  }

  CompleteResult<T> facts() {
    _live();
    final out = calloc<Pointer<Void>>();
    try {
      if (_owner._api.thinkthen_batch_facts(_pointer, out) != 0)
        throw StateError('batch has not terminated');
      return _owner._copy(out.value, _verb, _read);
    } finally {
      _owner._api.thinkthen_result_free(out.value);
      calloc.free(out);
    }
  }

  void close() {
    if (!_closed) {
      _owner._api.thinkthen_batch_free(_pointer);
      _closed = true;
      --_owner._batches;
    }
  }
}
