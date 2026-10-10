// Generated from the canonical C header ABI. Do not edit.
import 'dart:ffi';

@Native<Pointer<Void> Function(Pointer<Uint8>)>(
    symbol: 'thinkthen_engine_new_with',
    assetId: 'package:thinkthen_dart/thinkthen')
external Pointer<Void> _thinkthen_engine_new_with(Pointer<Uint8> arg0);
@Native<Void Function(Pointer<Void>)>(
    symbol: 'thinkthen_engine_free',
    assetId: 'package:thinkthen_dart/thinkthen')
external void _thinkthen_engine_free(Pointer<Void> arg0);
@Native<Int32 Function(Pointer<Void>)>(
    symbol: 'thinkthen_error_code', assetId: 'package:thinkthen_dart/thinkthen')
external int _thinkthen_error_code(Pointer<Void> arg0);
@Native<Int32 Function(Pointer<Void>)>(
    symbol: 'thinkthen_error_retryable',
    assetId: 'package:thinkthen_dart/thinkthen')
external int _thinkthen_error_retryable(Pointer<Void> arg0);
@Native<Pointer<Uint8> Function(Pointer<Void>)>(
    symbol: 'thinkthen_error_message',
    assetId: 'package:thinkthen_dart/thinkthen')
external Pointer<Uint8> _thinkthen_error_message(Pointer<Void> arg0);
@Native<Pointer<Uint8> Function(Pointer<Void>)>(
    symbol: 'thinkthen_error_facts_json',
    assetId: 'package:thinkthen_dart/thinkthen')
external Pointer<Uint8> _thinkthen_error_facts_json(Pointer<Void> arg0);
@Native<
        Int32 Function(Pointer<Void>, Pointer<Uint8>, Size, Pointer<Uint8>,
            Size, Pointer<Pointer<Void>>)>(
    symbol: 'thinkthen_session_new_with_surface',
    assetId: 'package:thinkthen_dart/thinkthen')
external int _thinkthen_session_new_with_surface(
    Pointer<Void> arg0,
    Pointer<Uint8> arg1,
    int arg2,
    Pointer<Uint8> arg3,
    int arg4,
    Pointer<Pointer<Void>> arg5);
@Native<Int32 Function(Pointer<Void>, Pointer<Uint8>, Size, Pointer<Uint32>)>(
    symbol: 'thinkthen_session_try_push',
    assetId: 'package:thinkthen_dart/thinkthen')
external int _thinkthen_session_try_push(
    Pointer<Void> arg0, Pointer<Uint8> arg1, int arg2, Pointer<Uint32> arg3);
@Native<Int32 Function(Pointer<Void>, Pointer<Uint32>, Pointer<Pointer<Void>>)>(
    symbol: 'thinkthen_session_try_read',
    assetId: 'package:thinkthen_dart/thinkthen')
external int _thinkthen_session_try_read(
    Pointer<Void> arg0, Pointer<Uint32> arg1, Pointer<Pointer<Void>> arg2);
@Native<Int32 Function(Pointer<Void>, Pointer<Uint8>, Size)>(
    symbol: 'thinkthen_session_finish',
    assetId: 'package:thinkthen_dart/thinkthen')
external int _thinkthen_session_finish(
    Pointer<Void> arg0, Pointer<Uint8> arg1, int arg2);
@Native<Void Function(Pointer<Void>)>(
    symbol: 'thinkthen_session_cancel',
    assetId: 'package:thinkthen_dart/thinkthen')
external void _thinkthen_session_cancel(Pointer<Void> arg0);
@Native<Void Function(Pointer<Void>)>(
    symbol: 'thinkthen_session_free',
    assetId: 'package:thinkthen_dart/thinkthen')
external void _thinkthen_session_free(Pointer<Void> arg0);
@Native<Void Function(Pointer<Void>)>(
    symbol: 'thinkthen_session_result_free',
    assetId: 'package:thinkthen_dart/thinkthen')
external void _thinkthen_session_result_free(Pointer<Void> arg0);
@Native<Int32 Function(Pointer<Void>, Pointer<Pointer<Void>>, Pointer<Size>)>(
    symbol: 'thinkthen_session_result_json',
    assetId: 'package:thinkthen_dart/thinkthen')
external int _thinkthen_session_result_json(
    Pointer<Void> arg0, Pointer<Pointer<Void>> arg1, Pointer<Size> arg2);
@Native<Pointer<Uint8> Function()>(
    symbol: 'thinkthen_session_error_message',
    assetId: 'package:thinkthen_dart/thinkthen')
external Pointer<Uint8> _thinkthen_session_error_message();

final class NativeAbi {
  final thinkthen_engine_new_with = _thinkthen_engine_new_with;
  final thinkthen_engine_free = _thinkthen_engine_free;
  final thinkthen_engine_freePointer =
      Native.addressOf<NativeFunction<Void Function(Pointer<Void>)>>(
          _thinkthen_engine_free);
  final thinkthen_error_code = _thinkthen_error_code;
  final thinkthen_error_retryable = _thinkthen_error_retryable;
  final thinkthen_error_message = _thinkthen_error_message;
  final thinkthen_error_facts_json = _thinkthen_error_facts_json;
  final thinkthen_session_new_with_surface =
      _thinkthen_session_new_with_surface;
  final thinkthen_session_try_push = _thinkthen_session_try_push;
  final thinkthen_session_try_read = _thinkthen_session_try_read;
  final thinkthen_session_finish = _thinkthen_session_finish;
  final thinkthen_session_cancel = _thinkthen_session_cancel;
  final thinkthen_session_free = _thinkthen_session_free;
  final thinkthen_session_freePointer =
      Native.addressOf<NativeFunction<Void Function(Pointer<Void>)>>(
          _thinkthen_session_free);
  final thinkthen_session_result_free = _thinkthen_session_result_free;
  final thinkthen_session_result_json = _thinkthen_session_result_json;
  final thinkthen_session_error_message = _thinkthen_session_error_message;
}

enum NativeErrorKind {
  backend(2),
  cancelled(5),
  deadline(3),
  defect(6),
  local(4),
  usage(1);

  final int code;
  const NativeErrorKind(this.code);
}

const THINKTHEN_EBACKEND = 2;
const THINKTHEN_ECANCELLED = 5;
const THINKTHEN_EDEADLINE = 3;
const THINKTHEN_EDEFECT = 6;
const THINKTHEN_ELOCAL = 4;
const THINKTHEN_EUSAGE = 1;
const THINKTHEN_SESSION_ACCEPTED_V1 = 0;
const THINKTHEN_SESSION_CLOSED_V1 = 2;
const THINKTHEN_SESSION_END_V1 = 2;
const THINKTHEN_SESSION_FULL_V1 = 1;
const THINKTHEN_SESSION_PENDING_V1 = 1;
const THINKTHEN_SESSION_RESULT_V1 = 0;
