// Generated from the canonical C header ABI. Do not edit.
import 'dart:ffi';

final class NativeAbi {
  final DynamicLibrary library;
  NativeAbi(this.library);
  late final thinkthen_engine_new_with = library.lookupFunction<
      Pointer<Void> Function(Pointer<Uint8>),
      Pointer<Void> Function(Pointer<Uint8>)>("thinkthen_engine_new_with");
  late final thinkthen_engine_free = library.lookupFunction<
      Void Function(Pointer<Void>),
      void Function(Pointer<Void>)>("thinkthen_engine_free");
  late final thinkthen_error_code = library.lookupFunction<
      Int32 Function(Pointer<Void>),
      int Function(Pointer<Void>)>("thinkthen_error_code");
  late final thinkthen_error_retryable = library.lookupFunction<
      Int32 Function(Pointer<Void>),
      int Function(Pointer<Void>)>("thinkthen_error_retryable");
  late final thinkthen_error_message = library.lookupFunction<
      Pointer<Uint8> Function(Pointer<Void>),
      Pointer<Uint8> Function(Pointer<Void>)>("thinkthen_error_message");
  late final thinkthen_error_facts_json = library.lookupFunction<
      Pointer<Uint8> Function(Pointer<Void>),
      Pointer<Uint8> Function(Pointer<Void>)>("thinkthen_error_facts_json");
  late final thinkthen_session_new_with_surface = library.lookupFunction<
      Int32 Function(Pointer<Void>, Pointer<Uint8>, Size, Pointer<Uint8>, Size,
          Pointer<Pointer<Void>>),
      int Function(Pointer<Void>, Pointer<Uint8>, int, Pointer<Uint8>, int,
          Pointer<Pointer<Void>>)>("thinkthen_session_new_with_surface");
  late final thinkthen_session_try_push = library.lookupFunction<
      Int32 Function(Pointer<Void>, Pointer<Uint8>, Size, Pointer<Uint32>),
      int Function(Pointer<Void>, Pointer<Uint8>, int,
          Pointer<Uint32>)>("thinkthen_session_try_push");
  late final thinkthen_session_try_read = library.lookupFunction<
      Int32 Function(Pointer<Void>, Pointer<Uint32>, Pointer<Pointer<Void>>),
      int Function(Pointer<Void>, Pointer<Uint32>,
          Pointer<Pointer<Void>>)>("thinkthen_session_try_read");
  late final thinkthen_session_finish = library.lookupFunction<
      Int32 Function(Pointer<Void>, Pointer<Uint8>, Size),
      int Function(
          Pointer<Void>, Pointer<Uint8>, int)>("thinkthen_session_finish");
  late final thinkthen_session_cancel = library.lookupFunction<
      Void Function(Pointer<Void>),
      void Function(Pointer<Void>)>("thinkthen_session_cancel");
  late final thinkthen_session_free = library.lookupFunction<
      Void Function(Pointer<Void>),
      void Function(Pointer<Void>)>("thinkthen_session_free");
  late final thinkthen_session_result_free = library.lookupFunction<
      Void Function(Pointer<Void>),
      void Function(Pointer<Void>)>("thinkthen_session_result_free");
  late final thinkthen_session_result_json = library.lookupFunction<
      Int32 Function(Pointer<Void>, Pointer<Pointer<Void>>, Pointer<Size>),
      int Function(Pointer<Void>, Pointer<Pointer<Void>>,
          Pointer<Size>)>("thinkthen_session_result_json");
  late final thinkthen_session_error_message = library.lookupFunction<
      Pointer<Uint8> Function(),
      Pointer<Uint8> Function()>("thinkthen_session_error_message");
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
