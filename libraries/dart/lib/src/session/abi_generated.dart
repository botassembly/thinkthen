// Generated from the canonical C header ABI. Do not edit.
part of thinkthen_session;

final class _NativeUsageState extends Struct {
  @Uint32()
  external int kind;
}

final class _NativeUsageAdvice extends Struct {
  external Pointer<Uint8> data;
  @Size()
  external int len;
}

final class _NativeNamedRequest extends Struct {
  external _NativeUsageAdvice function;
  external _NativeUsageAdvice question_json;
  external _NativeUsageAdvice input_json;
  external _NativeUsageAdvice options_json;
}

enum UsagePersistenceState {
  disabled(1),
  failed(4),
  pending(2),
  written(3);

  final int code;
  const UsagePersistenceState(this.code);
}

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
@Native<Pointer<Void> Function(Pointer<Void>)>(
    symbol: 'thinkthen_error_view', assetId: 'package:thinkthen_dart/thinkthen')
external Pointer<Void> _thinkthen_error_view(Pointer<Void> arg0);
@Native<
        Int32 Function(Pointer<Void>, Pointer<_NativeNamedRequest>,
            _NativeUsageAdvice, Pointer<Pointer<Void>>)>(
    symbol: 'thinkthen_session_new_named_with_surface',
    assetId: 'package:thinkthen_dart/thinkthen')
external int _thinkthen_session_new_named_with_surface(
    Pointer<Void> arg0,
    Pointer<_NativeNamedRequest> arg1,
    _NativeUsageAdvice arg2,
    Pointer<Pointer<Void>> arg3);
@Native<Int32 Function(Pointer<Void>, Pointer<Uint8>, Size, Pointer<Uint32>)>(
    symbol: 'thinkthen_session_push',
    assetId: 'package:thinkthen_dart/thinkthen')
external int _thinkthen_session_push(
    Pointer<Void> arg0, Pointer<Uint8> arg1, int arg2, Pointer<Uint32> arg3);
@Native<
        Int32 Function(
            Pointer<Void>, Uint32, Pointer<Uint32>, Pointer<Pointer<Void>>)>(
    symbol: 'thinkthen_session_next_packet',
    assetId: 'package:thinkthen_dart/thinkthen')
external int _thinkthen_session_next_packet(Pointer<Void> arg0, int arg1,
    Pointer<Uint32> arg2, Pointer<Pointer<Void>> arg3);
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
@Native<Int32 Function(Pointer<Void>, Pointer<Pointer<Void>>)>(
    symbol: 'thinkthen_session_result_view',
    assetId: 'package:thinkthen_dart/thinkthen')
external int _thinkthen_session_result_view(
    Pointer<Void> arg0, Pointer<Pointer<Void>> arg1);
@Native<Pointer<Void> Function()>(
    symbol: 'thinkthen_session_error_view',
    assetId: 'package:thinkthen_dart/thinkthen')
external Pointer<Void> _thinkthen_session_error_view();
@Native<
        Int32 Function(Pointer<Void>, Pointer<_NativeUsageState>,
            Pointer<_NativeUsageAdvice>)>(
    symbol: 'thinkthen_engine_usage_persistence_v1',
    assetId: 'package:thinkthen_dart/thinkthen')
external int _thinkthen_engine_usage_persistence_v1(Pointer<Void> arg0,
    Pointer<_NativeUsageState> arg1, Pointer<_NativeUsageAdvice> arg2);
@Native<
        Int32 Function(Pointer<Void>, Pointer<_NativeUsageState>,
            Pointer<_NativeUsageAdvice>)>(
    symbol: 'thinkthen_engine_finish_usage_status_v1',
    assetId: 'package:thinkthen_dart/thinkthen')
external int _thinkthen_engine_finish_usage_status_v1(Pointer<Void> arg0,
    Pointer<_NativeUsageState> arg1, Pointer<_NativeUsageAdvice> arg2);

final class _NativeAbi {
  final thinkthen_engine_new_with = _thinkthen_engine_new_with;
  final thinkthen_engine_free = _thinkthen_engine_free;
  final thinkthen_engine_freePointer =
      Native.addressOf<NativeFunction<Void Function(Pointer<Void>)>>(
          _thinkthen_engine_free);
  final thinkthen_error_code = _thinkthen_error_code;
  final thinkthen_error_view = _thinkthen_error_view;
  final thinkthen_session_new_named_with_surface =
      _thinkthen_session_new_named_with_surface;
  final thinkthen_session_push = _thinkthen_session_push;
  final thinkthen_session_next_packet = _thinkthen_session_next_packet;
  final thinkthen_session_finish = _thinkthen_session_finish;
  final thinkthen_session_cancel = _thinkthen_session_cancel;
  final thinkthen_session_free = _thinkthen_session_free;
  final thinkthen_session_freePointer =
      Native.addressOf<NativeFunction<Void Function(Pointer<Void>)>>(
          _thinkthen_session_free);
  final thinkthen_session_result_free = _thinkthen_session_result_free;
  final thinkthen_session_result_view = _thinkthen_session_result_view;
  final thinkthen_session_error_view = _thinkthen_session_error_view;
  final thinkthen_engine_usage_persistence_v1 =
      _thinkthen_engine_usage_persistence_v1;
  final thinkthen_engine_finish_usage_status_v1 =
      _thinkthen_engine_finish_usage_status_v1;
  Pointer<Void> startNamed(Pointer<Void> engine, String function,
      Question question, Input input, Options? options, String surface) {
    final output = calloc<Pointer<Void>>();
    final request = calloc<_NativeNamedRequest>();
    final bytes = <Pointer<Uint8>>[];
    final views = <Pointer<_NativeUsageAdvice>>[];
    _NativeUsageAdvice counted(String text) {
      final encoded = utf8.encode(text);
      final data = calloc<Uint8>(encoded.length + 1);
      bytes.add(data);
      data.asTypedList(encoded.length).setAll(0, encoded);
      final view = calloc<_NativeUsageAdvice>();
      views.add(view);
      view.ref.data = data;
      view.ref.len = encoded.length;
      return view.ref;
    }

    try {
      request.ref.function = counted(function);
      request.ref.question_json = counted(_encodeNative(question));
      request.ref.input_json = counted(_encodeNative(input));
      request.ref.options_json = counted(_encodeNative(options ?? Options()));
      _checkSession(
          this,
          thinkthen_session_new_named_with_surface(
              engine, request, counted(surface), output));
      return output.value;
    } finally {
      for (final value in bytes) calloc.free(value);
      for (final value in views) calloc.free(value);
      calloc.free(request);
      calloc.free(output);
    }
  }

  SessionPacket takePacket(Pointer<Void> packet) {
    final view = calloc<Pointer<Void>>();
    try {
      _checkSession(this, thinkthen_session_result_view(packet, view));
      return SessionPacket.read(
          _NativePacketReader.readSessionPacket(view.value));
    } finally {
      thinkthen_session_result_free(packet);
      calloc.free(view);
    }
  }

  UsagePersistenceStatus finishUsageStatus(Pointer<Void> engine) {
    final state = calloc<_NativeUsageState>();
    final advice = calloc<_NativeUsageAdvice>();
    try {
      _checkSession(
          this, thinkthen_engine_finish_usage_status_v1(engine, state, advice));
      return UsagePersistenceStatus(
          UsagePersistenceState.values
              .firstWhere((value) => value.code == state.ref.kind),
          advice.ref.data == nullptr
              ? null
              : utf8.decode(advice.ref.data.asTypedList(advice.ref.len)));
    } finally {
      calloc.free(state);
      calloc.free(advice);
    }
  }

  UsagePersistenceStatus usagePersistence(Pointer<Void> engine) {
    final state = calloc<_NativeUsageState>();
    final advice = calloc<_NativeUsageAdvice>();
    try {
      _checkSession(
          this, thinkthen_engine_usage_persistence_v1(engine, state, advice));
      return UsagePersistenceStatus(
          UsagePersistenceState.values
              .firstWhere((value) => value.code == state.ref.kind),
          advice.ref.data == nullptr
              ? null
              : utf8.decode(advice.ref.data.asTypedList(advice.ref.len)));
    } finally {
      calloc.free(state);
      calloc.free(advice);
    }
  }

  int waitPacket(Pointer<Void> session) {
    final status = calloc<Uint32>();
    final value = calloc<Pointer<Void>>();
    try {
      _checkSession(
          this,
          thinkthen_session_next_packet(
              session, THINKTHEN_SESSION_WAIT_SETTLEMENT_V1, status, value));
      if (status.value == THINKTHEN_SESSION_END_V1) return 0;
      if (status.value != THINKTHEN_SESSION_RESULT_V1)
        throw StateError("Invalid native waitPacket status");
      return value.value.address;
    } finally {
      calloc.free(status);
      calloc.free(value);
    }
  }

  int pushItem(Pointer<Void> session, String item) {
    final status = calloc<Uint32>();
    try {
      _withText(
          item,
          (bytes, length) => _checkSession(
              this, thinkthen_session_push(session, bytes, length, status)));
      if (status.value == THINKTHEN_SESSION_CLOSED_V1) return status.value;
      if (status.value != THINKTHEN_SESSION_ACCEPTED_V1)
        throw StateError("Invalid native pushItem status");
      return status.value;
    } finally {
      calloc.free(status);
    }
  }
}

enum ErrorKind {
  backend(2),
  cancelled(5),
  deadline(3),
  defect(6),
  local(4),
  usage(1);

  final int code;
  const ErrorKind(this.code);
}

const THINKTHEN_SESSION_ACCEPTED_V1 = 0;
const THINKTHEN_SESSION_CLOSED_V1 = 2;
const THINKTHEN_SESSION_END_V1 = 2;
const THINKTHEN_SESSION_FULL_V1 = 1;
const THINKTHEN_SESSION_PENDING_V1 = 1;
const THINKTHEN_SESSION_RESULT_V1 = 0;
const THINKTHEN_SESSION_WAIT_INPUT_V1 = 1;
const THINKTHEN_SESSION_WAIT_SETTLEMENT_V1 = 2;

final class _NativePacketReader {
  static Object? readFailureFacts(Pointer<Void> p) {
    final values = _extensions(_at(p, 216));
    if (_u32(_at(p, 0)) != 0)
      values["attempts"] = _u32(_at(p, 0)) == 1
          ? null
          : _list(_at(_at(p, 0), 8), 8, (p) => readAttempt(_pointer(p)));
    values["cache_answers"] = _unsigned(_at(p, 24));
    if (_u32(_at(p, 32)) != 0)
      values["call_id"] = _u32(_at(p, 32)) == 1
          ? null
          : readCallId(_pointer(_at(_at(p, 32), 8)));
    if (_u32(_at(p, 48)) != 0)
      values["estimated_cost_usd"] =
          _u32(_at(p, 48)) == 1 ? null : _text(_at(_at(p, 48), 8));
    if (_u32(_at(p, 72)) != 0)
      values["held_model_mismatch"] =
          _u32(_at(p, 72)) == 1 ? null : _u32(_at(_at(p, 72), 4)) != 0;
    if (_u32(_at(p, 80)) != 0)
      values["input_tokens"] =
          _u32(_at(p, 80)) == 1 ? null : _unsigned(_at(_at(p, 80), 8));
    values["largest_request_bytes"] = _unsigned(_at(p, 96));
    if (_u32(_at(p, 104)) != 0)
      values["largest_request_estimated_input_tokens"] =
          _u32(_at(p, 104)) == 1 ? null : _unsigned(_at(_at(p, 104), 8));
    if (_u32(_at(p, 120)) != 0)
      values["model"] =
          _u32(_at(p, 120)) == 1 ? null : _text(_at(_at(p, 120), 8));
    if (_u32(_at(p, 144)) != 0)
      values["output_tokens"] =
          _u32(_at(p, 144)) == 1 ? null : _unsigned(_at(_at(p, 144), 8));
    values["records"] = _unsigned(_at(p, 160));
    values["requests_sent"] = _unsigned(_at(p, 168));
    values["seconds"] = _at(p, 176).cast<Double>().value;
    values["token_estimate_method"] = _text(_at(p, 184));
    if (_u32(_at(p, 200)) != 0)
      values["usage_persistence"] = _u32(_at(p, 200)) == 1
          ? null
          : readPersistenceObservation(_pointer(_at(_at(p, 200), 8)));
    return values;
  }

  static Object? readPersistenceObservation(Pointer<Void> p) {
    final values = _extensions(_at(p, 48));
    if (_u32(_at(p, 0)) != 0)
      values["advice"] = _u32(_at(p, 0)) == 1 ? null : _text(_at(_at(p, 0), 8));
    values["observed_at"] = _text(_at(p, 24));
    values["state"] = readUsagePersistence(_pointer(_at(p, 40)));
    return values;
  }

  static Object? readUsagePersistence(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => "disabled",
      2 => "pending",
      3 => "written",
      4 => "failed",
      _ => throw StateError('Invalid native enum'),
    };
  }

  static Object? readCallId(Pointer<Void> p) {
    return _text(_at(p, 0));
  }

  static Object? readAttempt(Pointer<Void> p) {
    final values = _extensions(_at(p, 104));
    values["ordinal"] = _unsigned(_at(p, 0));
    values["outcome"] = readAttemptOutcome(_pointer(_at(p, 8)));
    if (_u32(_at(p, 16)) != 0)
      values["request_id"] =
          _u32(_at(p, 16)) == 1 ? null : _text(_at(_at(p, 16), 8));
    values["request_sha256"] = _text(_at(p, 40));
    values["sdk_request_id"] = readSdkRequestId(_pointer(_at(p, 56)));
    if (_u32(_at(p, 64)) != 0)
      values["server_ms"] =
          _u32(_at(p, 64)) == 1 ? null : _unsigned(_at(_at(p, 64), 8));
    if (_u32(_at(p, 80)) != 0)
      values["status"] =
          _u32(_at(p, 80)) == 1 ? null : _unsigned(_at(_at(p, 80), 8));
    values["wall_ms"] = _unsigned(_at(p, 96));
    return values;
  }

  static Object? readSdkRequestId(Pointer<Void> p) {
    return _text(_at(p, 0));
  }

  static Object? readAttemptOutcome(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => "ok",
      2 => "status",
      3 => "transport",
      _ => throw StateError('Invalid native enum'),
    };
  }

  static Object? readSessionPacket(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => readSessionPacketDecideRow(_pointer(_at(p, 8))),
      2 => readSessionPacketChooseRow(_pointer(_at(p, 8))),
      3 => readSessionPacketTagRow(_pointer(_at(p, 8))),
      4 => readSessionPacketScoreRow(_pointer(_at(p, 8))),
      5 => readSessionPacketFilterRow(_pointer(_at(p, 8))),
      6 => readSessionPacketAnnotateRow(_pointer(_at(p, 8))),
      7 => readSessionPacketDecideAggregate(_pointer(_at(p, 8))),
      8 => readSessionPacketChooseAggregate(_pointer(_at(p, 8))),
      9 => readSessionPacketTagAggregate(_pointer(_at(p, 8))),
      10 => readSessionPacketScoreAggregate(_pointer(_at(p, 8))),
      11 => readSessionPacketFilterAggregate(_pointer(_at(p, 8))),
      12 => readSessionPacketRankAggregate(_pointer(_at(p, 8))),
      13 => readSessionPacketFindAggregate(_pointer(_at(p, 8))),
      14 => readSessionPacketAnnotateAggregate(_pointer(_at(p, 8))),
      15 => readSessionPacketRecognizeAggregate(_pointer(_at(p, 8))),
      16 => readSessionPacketRelateAggregate(_pointer(_at(p, 8))),
      17 => readSessionPacketObservation(_pointer(_at(p, 8))),
      18 => readSessionPacketTerminal(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native discriminator'),
    };
  }

  static Object? readSessionPacketTerminal(Pointer<Void> p) {
    final values = _extensions(_at(p, 48));
    if (_u32(_at(p, 0)) != 0)
      values["facts"] =
          _u32(_at(p, 0)) == 1 ? null : readFacts(_pointer(_at(_at(p, 0), 8)));
    if (_u32(_at(p, 16)) != 0)
      values["failure"] = _u32(_at(p, 16)) == 1
          ? null
          : readCallError(_pointer(_at(_at(p, 16), 8)));
    values["kind"] = _text(_at(p, 32));
    return values;
  }

  static Object? readCallError(Pointer<Void> p) {
    final values = _extensions(_at(p, 24));
    values["error"] = readError(_pointer(_at(p, 0)));
    if (_u32(_at(p, 8)) != 0)
      values["facts"] =
          _u32(_at(p, 8)) == 1 ? null : readFacts(_pointer(_at(_at(p, 8), 8)));
    return values;
  }

  static Object? readFacts(Pointer<Void> p) {
    final values = _extensions(_at(p, 208));
    if (_u32(_at(p, 0)) != 0)
      values["attempts"] = _u32(_at(p, 0)) == 1
          ? null
          : _list(_at(_at(p, 0), 8), 8, (p) => readAttempt(_pointer(p)));
    values["cache_answers"] = _unsigned(_at(p, 24));
    values["call_id"] = readCallId(_pointer(_at(p, 32)));
    if (_u32(_at(p, 40)) != 0)
      values["estimated_cost_usd"] =
          _u32(_at(p, 40)) == 1 ? null : _text(_at(_at(p, 40), 8));
    if (_u32(_at(p, 64)) != 0)
      values["held_model_mismatch"] =
          _u32(_at(p, 64)) == 1 ? null : _u32(_at(_at(p, 64), 4)) != 0;
    if (_u32(_at(p, 72)) != 0)
      values["input_tokens"] =
          _u32(_at(p, 72)) == 1 ? null : _unsigned(_at(_at(p, 72), 8));
    values["largest_request_bytes"] = _unsigned(_at(p, 88));
    if (_u32(_at(p, 96)) != 0)
      values["largest_request_estimated_input_tokens"] =
          _u32(_at(p, 96)) == 1 ? null : _unsigned(_at(_at(p, 96), 8));
    if (_u32(_at(p, 112)) != 0)
      values["model"] =
          _u32(_at(p, 112)) == 1 ? null : _text(_at(_at(p, 112), 8));
    if (_u32(_at(p, 136)) != 0)
      values["output_tokens"] =
          _u32(_at(p, 136)) == 1 ? null : _unsigned(_at(_at(p, 136), 8));
    values["records"] = _unsigned(_at(p, 152));
    values["requests_sent"] = _unsigned(_at(p, 160));
    values["seconds"] = _at(p, 168).cast<Double>().value;
    values["token_estimate_method"] = _text(_at(p, 176));
    if (_u32(_at(p, 192)) != 0)
      values["usage_persistence"] = _u32(_at(p, 192)) == 1
          ? null
          : readPersistenceObservation(_pointer(_at(_at(p, 192), 8)));
    return values;
  }

  static Object? readError(Pointer<Void> p) {
    final values = _extensions(_at(p, 72));
    if (_u32(_at(p, 0)) != 0)
      values["estimated_input_denial"] = _u32(_at(p, 0)) == 1
          ? null
          : readEstimatedInputDenial(_pointer(_at(_at(p, 0), 8)));
    values["kind"] = readFailureKind(_pointer(_at(p, 16)));
    values["message"] = _text(_at(p, 24));
    values["retryable"] = _u32(_at(p, 40)) != 0;
    if (_u32(_at(p, 48)) != 0)
      values["send_budget_denial"] = _u32(_at(p, 48)) == 1
          ? null
          : readSendBudgetDenial(_pointer(_at(_at(p, 48), 8)));
    values["stopped"] = readStopped(_pointer(_at(p, 64)));
    return values;
  }

  static Object? readStopped(Pointer<Void> p) {
    final values = _extensions(_at(p, 48));
    if (_u32(_at(p, 0)) != 0)
      values["at"] = _u32(_at(p, 0)) == 1 ? null : _unsigned(_at(_at(p, 0), 8));
    values["cause"] = readStopCause(_pointer(_at(p, 16)));
    values["retryable"] = _u32(_at(p, 24)) != 0;
    if (_u32(_at(p, 32)) != 0)
      values["status"] =
          _u32(_at(p, 32)) == 1 ? null : _unsigned(_at(_at(p, 32), 8));
    return values;
  }

  static Object? readStopCause(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => "usage",
      2 => "local",
      3 => "no_key",
      4 => "transport",
      5 => "status",
      6 => "too_large",
      7 => "reply",
      8 => "backend",
      9 => "cancelled",
      10 => "deadline",
      11 => "defect",
      _ => throw StateError('Invalid native enum'),
    };
  }

  static Object? readSendBudgetDenial(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => readSendBudgetDenialBeforeFirstSend(_pointer(_at(p, 8))),
      2 => readSendBudgetDenialBeforeAdditionalSend(_pointer(_at(p, 8))),
      3 => readSendBudgetDenialBeforeRetry(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native discriminator'),
    };
  }

  static Object? readSendBudgetDenialBeforeRetry(Pointer<Void> p) {
    final values = _extensions(_at(p, 24));
    values["kind"] = _text(_at(p, 0));
    values["last_status"] = _unsigned(_at(p, 16));
    return values;
  }

  static Object? readSendBudgetDenialBeforeAdditionalSend(Pointer<Void> p) {
    final values = _extensions(_at(p, 16));
    values["kind"] = _text(_at(p, 0));
    return values;
  }

  static Object? readSendBudgetDenialBeforeFirstSend(Pointer<Void> p) {
    final values = _extensions(_at(p, 16));
    values["kind"] = _text(_at(p, 0));
    return values;
  }

  static Object? readFailureKind(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => "usage",
      2 => "backend",
      3 => "local",
      4 => "cancelled",
      5 => "deadline",
      6 => "defect",
      _ => throw StateError('Invalid native enum'),
    };
  }

  static Object? readEstimatedInputDenial(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => readEstimatedInputDenialInitialRequest(_pointer(_at(p, 8))),
      2 => readEstimatedInputDenialAdditionalRequest(_pointer(_at(p, 8))),
      3 => readEstimatedInputDenialRetry(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native discriminator'),
    };
  }

  static Object? readEstimatedInputDenialRetry(Pointer<Void> p) {
    final values = _extensions(_at(p, 32));
    values["kind"] = _text(_at(p, 0));
    values["last_status"] = _unsigned(_at(p, 16));
    values["limit"] = _unsigned(_at(p, 24));
    return values;
  }

  static Object? readEstimatedInputDenialAdditionalRequest(Pointer<Void> p) {
    final values = _extensions(_at(p, 24));
    values["kind"] = _text(_at(p, 0));
    values["limit"] = _unsigned(_at(p, 16));
    return values;
  }

  static Object? readEstimatedInputDenialInitialRequest(Pointer<Void> p) {
    final values = _extensions(_at(p, 24));
    values["kind"] = _text(_at(p, 0));
    values["limit"] = _unsigned(_at(p, 16));
    return values;
  }

  static Object? readSessionPacketObservation(Pointer<Void> p) {
    final values = _extensions(_at(p, 32));
    values["function"] = readRequestFunction(_pointer(_at(p, 0)));
    values["kind"] = _text(_at(p, 8));
    values["value"] = readSessionObservation(_pointer(_at(p, 24)));
    return values;
  }

  static Object? readSessionObservation(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => readSessionObservationQuestion(_pointer(_at(p, 8))),
      2 => readSessionObservationRow(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native discriminator'),
    };
  }

  static Object? readSessionObservationRow(Pointer<Void> p) {
    final values = _extensions(_at(p, 32));
    values["index"] = _unsigned(_at(p, 0));
    values["kind"] = _text(_at(p, 8));
    values["value"] = readSessionObservedRow(_pointer(_at(p, 24)));
    return values;
  }

  static Object? readSessionObservedRow(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => readSessionObservedRowJudgment(_pointer(_at(p, 8))),
      2 => readSessionObservedRowAnnotated(_pointer(_at(p, 8))),
      3 => readSessionObservedRowRecognized(_pointer(_at(p, 8))),
      4 => readSessionObservedRowFind(_pointer(_at(p, 8))),
      5 => readSessionObservedRowRelations(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native discriminator'),
    };
  }

  static Object? readSessionObservedRowRelations(Pointer<Void> p) {
    final values = _extensions(_at(p, 32));
    values["kind"] = _text(_at(p, 0));
    values["value"] =
        _list(_at(p, 16), 8, (p) => readSessionRelationEdge(_pointer(p)));
    return values;
  }

  static Object? readSessionRelationEdge(Pointer<Void> p) {
    final values = _extensions(_at(p, 48));
    values["either"] = _u32(_at(p, 0)) != 0;
    values["probability"] = _at(p, 8).cast<Double>().value;
    values["relation"] = _text(_at(p, 16));
    values["source"] = readEntityDocument(_pointer(_at(p, 32)));
    values["target"] = readEntityDocument(_pointer(_at(p, 40)));
    return values;
  }

  static Object? readEntityDocument(Pointer<Void> p) {
    final values = _extensions(_at(p, 32));
    values["kind"] = _text(_at(p, 0));
    values["name"] = _text(_at(p, 16));
    return values;
  }

  static Object? readSessionObservedRowFind(Pointer<Void> p) {
    final values = _extensions(_at(p, 32));
    values["kind"] = _text(_at(p, 0));
    if (_u32(_at(p, 16)) != 0)
      values["value"] =
          _u32(_at(p, 16)) == 1 ? null : _unsigned(_at(_at(p, 16), 8));
    return values;
  }

  static Object? readSessionObservedRowRecognized(Pointer<Void> p) {
    final values = _extensions(_at(p, 24));
    values["kind"] = _text(_at(p, 0));
    values["value"] = readSessionRecognition(_pointer(_at(p, 16)));
    return values;
  }

  static Object? readSessionRecognition(Pointer<Void> p) {
    final values = _extensions(_at(p, 72));
    values["entities"] = _list(_at(p, 0), 8, (p) => readEntity(_pointer(p)));
    values["mode"] = readRecognitionMode(_pointer(_at(p, 16)));
    if (_u32(_at(p, 24)) != 0)
      values["proposals"] = _u32(_at(p, 24)) == 1
          ? null
          : _list(
              _at(_at(p, 24), 8), 8, (p) => readBoundaryProposal(_pointer(p)));
    if (_u32(_at(p, 48)) != 0)
      values["relations"] = _u32(_at(p, 48)) == 1
          ? null
          : _list(_at(_at(p, 48), 8), 8,
              (p) => readRecognitionEdgeDocument(_pointer(p)));
    return values;
  }

  static Object? readRecognitionEdgeDocument(Pointer<Void> p) {
    final values = _extensions(_at(p, 48));
    values["either"] = _u32(_at(p, 0)) != 0;
    values["probability"] = _at(p, 8).cast<Double>().value;
    values["relation"] = _text(_at(p, 16));
    values["source"] = readEntity(_pointer(_at(p, 32)));
    values["target"] = readEntity(_pointer(_at(p, 40)));
    return values;
  }

  static Object? readEntity(Pointer<Void> p) {
    final values = _extensions(_at(p, 120));
    values["end"] = _unsigned(_at(p, 0));
    if (_u32(_at(p, 8)) != 0)
      values["file"] = _u32(_at(p, 8)) == 1 ? null : _text(_at(_at(p, 8), 8));
    if (_u32(_at(p, 32)) != 0)
      values["first_line"] =
          _u32(_at(p, 32)) == 1 ? null : _unsigned(_at(_at(p, 32), 8));
    values["kind"] = _text(_at(p, 48));
    if (_u32(_at(p, 64)) != 0)
      values["last_line"] =
          _u32(_at(p, 64)) == 1 ? null : _unsigned(_at(_at(p, 64), 8));
    values["length"] = _unsigned(_at(p, 80));
    values["start"] = _unsigned(_at(p, 88));
    values["strength"] = _at(p, 96).cast<Double>().value;
    values["text"] = _text(_at(p, 104));
    return values;
  }

  static Object? readBoundaryProposal(Pointer<Void> p) {
    final values = _extensions(_at(p, 48));
    values["end"] = _unsigned(_at(p, 0));
    values["length"] = _unsigned(_at(p, 8));
    values["probability"] = _at(p, 16).cast<Double>().value;
    values["start"] = _unsigned(_at(p, 24));
    values["text"] = _text(_at(p, 32));
    return values;
  }

  static Object? readRecognitionMode(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => "whole",
      2 => "boundary_only",
      _ => throw StateError('Invalid native enum'),
    };
  }

  static Object? readSessionObservedRowAnnotated(Pointer<Void> p) {
    final values = _extensions(_at(p, 32));
    values["kind"] = _text(_at(p, 0));
    values["value"] =
        _list(_at(p, 16), 8, (p) => readSessionAnnotation(_pointer(p)));
    return values;
  }

  static Object? readSessionAnnotation(Pointer<Void> p) {
    final values = _extensions(_at(p, 24));
    values["name"] = _text(_at(p, 0));
    values["value"] = readAnnotationValue(_pointer(_at(p, 16)));
    return values;
  }

  static Object? readAnnotationValue(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => readAnnotationValueDecision(_pointer(_at(p, 8))),
      2 => readAnnotationValueChoice(_pointer(_at(p, 8))),
      3 => readAnnotationValueScore(_pointer(_at(p, 8))),
      4 => readAnnotationValueTags(_pointer(_at(p, 8))),
      5 => readAnnotationValueFailed(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native discriminator'),
    };
  }

  static Object? readAnnotationValueFailed(Pointer<Void> p) {
    final values = _extensions(_at(p, 24));
    values["kind"] = _text(_at(p, 0));
    values["value"] = readFailure(_pointer(_at(p, 16)));
    return values;
  }

  static Object? readFailure(Pointer<Void> p) {
    final values = _extensions(_at(p, 16));
    values["cause"] = readFailureCause(_pointer(_at(p, 0)));
    values["kind"] = readFailureFieldKind(_pointer(_at(p, 8)));
    return values;
  }

  static Object? readFailureCause(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => "missing_answer",
      2 => "wrong_kind",
      3 => "missing_probability",
      4 => "invalid_probability",
      5 => "invalid_distribution",
      6 => "unexpected_probability",
      _ => throw StateError('Invalid native enum'),
    };
  }

  static Object? readAnnotationValueTags(Pointer<Void> p) {
    final values = _extensions(_at(p, 32));
    values["kind"] = _text(_at(p, 0));
    values["value"] = _list(_at(p, 16), 16, (p) => _text(p));
    return values;
  }

  static Object? readAnnotationValueScore(Pointer<Void> p) {
    final values = _extensions(_at(p, 24));
    values["kind"] = _text(_at(p, 0));
    values["value"] = _at(p, 16).cast<Double>().value;
    return values;
  }

  static Object? readAnnotationValueChoice(Pointer<Void> p) {
    final values = _extensions(_at(p, 40));
    values["kind"] = _text(_at(p, 0));
    if (_u32(_at(p, 16)) != 0)
      values["value"] =
          _u32(_at(p, 16)) == 1 ? null : _text(_at(_at(p, 16), 8));
    return values;
  }

  static Object? readAnnotationValueDecision(Pointer<Void> p) {
    final values = _extensions(_at(p, 24));
    values["kind"] = _text(_at(p, 0));
    if (_u32(_at(p, 16)) != 0)
      values["value"] =
          _u32(_at(p, 16)) == 1 ? null : _u32(_at(_at(p, 16), 4)) != 0;
    return values;
  }

  static Object? readSessionObservedRowJudgment(Pointer<Void> p) {
    final values = _extensions(_at(p, 24));
    values["kind"] = _text(_at(p, 0));
    values["value"] = readSessionJudgment(_pointer(_at(p, 16)));
    return values;
  }

  static Object? readSessionJudgment(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => readSessionJudgmentDecision(_pointer(_at(p, 8))),
      2 => readSessionJudgmentChoice(_pointer(_at(p, 8))),
      3 => readSessionJudgmentScore(_pointer(_at(p, 8))),
      4 => readSessionJudgmentTags(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native discriminator'),
    };
  }

  static Object? readSessionJudgmentTags(Pointer<Void> p) {
    final values = _extensions(_at(p, 32));
    values["kind"] = _text(_at(p, 0));
    values["value"] = _list(_at(p, 16), 16, (p) => _text(p));
    return values;
  }

  static Object? readSessionJudgmentScore(Pointer<Void> p) {
    final values = _extensions(_at(p, 24));
    values["kind"] = _text(_at(p, 0));
    values["value"] = _at(p, 16).cast<Double>().value;
    return values;
  }

  static Object? readSessionJudgmentChoice(Pointer<Void> p) {
    final values = _extensions(_at(p, 40));
    values["kind"] = _text(_at(p, 0));
    if (_u32(_at(p, 16)) != 0)
      values["value"] =
          _u32(_at(p, 16)) == 1 ? null : _text(_at(_at(p, 16), 8));
    return values;
  }

  static Object? readSessionJudgmentDecision(Pointer<Void> p) {
    final values = _extensions(_at(p, 24));
    values["kind"] = _text(_at(p, 0));
    if (_u32(_at(p, 16)) != 0)
      values["value"] =
          _u32(_at(p, 16)) == 1 ? null : _u32(_at(_at(p, 16), 4)) != 0;
    return values;
  }

  static Object? readSessionObservationQuestion(Pointer<Void> p) {
    final values = _extensions(_at(p, 88));
    values["detail"] = readSessionQuestionDetail(_pointer(_at(p, 0)));
    values["index"] = _unsigned(_at(p, 8));
    values["kind"] = _text(_at(p, 16));
    if (_u32(_at(p, 32)) != 0)
      values["member"] =
          _u32(_at(p, 32)) == 1 ? null : _text(_at(_at(p, 32), 8));
    values["position"] = _unsigned(_at(p, 56));
    if (_u32(_at(p, 64)) != 0)
      values["stage"] =
          _u32(_at(p, 64)) == 1 ? null : _text(_at(_at(p, 64), 8));
    return values;
  }

  static Object? readSessionQuestionDetail(Pointer<Void> p) {
    final values = _extensions(_at(p, 360));
    if (_u32(_at(p, 0)) != 0)
      values["answer_id"] = _u32(_at(p, 0)) == 1
          ? null
          : readAnswerId(_pointer(_at(_at(p, 0), 8)));
    values["cached"] = _u32(_at(p, 16)) != 0;
    if (_u32(_at(p, 24)) != 0)
      values["confidence"] = _u32(_at(p, 24)) == 1
          ? null
          : _at(_at(p, 24), 8).cast<Double>().value;
    values["failed_questions"] = _unsigned(_at(p, 40));
    if (_u32(_at(p, 48)) != 0)
      values["failure"] = _u32(_at(p, 48)) == 1
          ? null
          : readFailure(_pointer(_at(_at(p, 48), 8)));
    if (_u32(_at(p, 64)) != 0)
      values["failure_id"] = _u32(_at(p, 64)) == 1
          ? null
          : readFailureId(_pointer(_at(_at(p, 64), 8)));
    if (_u32(_at(p, 80)) != 0)
      values["input"] =
          _u32(_at(p, 80)) == 1 ? null : _value(_pointer(_at(_at(p, 80), 8)));
    if (_u32(_at(p, 96)) != 0)
      values["input_source"] = _u32(_at(p, 96)) == 1
          ? null
          : readPhysicalSource(_pointer(_at(_at(p, 96), 8)));
    values["input_sources"] =
        _list(_at(p, 112), 8, (p) => readSessionInputSource(_pointer(p)));
    values["inputs"] = _list(_at(p, 128), 8, (p) => _value(_pointer(p)));
    values["model"] = _text(_at(p, 144));
    values["observations"] =
        _list(_at(p, 160), 8, (p) => readObservation(_pointer(p)));
    if (_u32(_at(p, 176)) != 0)
      values["probabilities"] = _u32(_at(p, 176)) == 1
          ? null
          : readSessionProbabilities(_pointer(_at(_at(p, 176), 8)));
    values["question"] = readReadableQuestion(_pointer(_at(p, 192)));
    values["question_sha256"] = _text(_at(p, 200));
    values["question_sources"] =
        _list(_at(p, 216), 8, (p) => readQuestionSource(_pointer(p)));
    if (_u32(_at(p, 232)) != 0)
      values["raw_pick"] =
          _u32(_at(p, 232)) == 1 ? null : _text(_at(_at(p, 232), 8));
    if (_u32(_at(p, 256)) != 0)
      values["reported_usage"] = _u32(_at(p, 256)) == 1
          ? null
          : readUsage(_pointer(_at(_at(p, 256), 8)));
    values["requests"] = _list(_at(p, 272), 16, (p) => _text(p));
    values["requests_sent"] = _unsigned(_at(p, 288));
    if (_u32(_at(p, 296)) != 0)
      values["threshold"] = _u32(_at(p, 296)) == 1
          ? null
          : readThreshold(_pointer(_at(_at(p, 296), 8)));
    values["url"] = _text(_at(p, 312));
    if (_u32(_at(p, 328)) != 0)
      values["usage"] = _u32(_at(p, 328)) == 1
          ? null
          : readTokenUsage(_pointer(_at(_at(p, 328), 8)));
    if (_u32(_at(p, 344)) != 0)
      values["value"] = _u32(_at(p, 344)) == 1
          ? null
          : readValue(_pointer(_at(_at(p, 344), 8)));
    return values;
  }

  static Object? readValue(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => _u32(_at(p, 8)) != 0,
      2 => null,
      3 => _text(_at(p, 8)),
      4 => _list(_at(p, 8), 16, (p) => _text(p)),
      5 => _at(p, 8).cast<Double>().value,
      _ => throw StateError('Invalid native value'),
    };
  }

  static Object? readTokenUsage(Pointer<Void> p) {
    final values = _extensions(_at(p, 16));
    values["input_tokens"] = _unsigned(_at(p, 0));
    values["output_tokens"] = _unsigned(_at(p, 8));
    return values;
  }

  static Object? readThreshold(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => _at(p, 8).cast<Double>().value,
      2 => _text(_at(p, 8)),
      _ => throw StateError('Invalid native value'),
    };
  }

  static Object? readUsage(Pointer<Void> p) {
    final values = _extensions(_at(p, 32));
    if (_u32(_at(p, 0)) != 0)
      values["input_tokens"] =
          _u32(_at(p, 0)) == 1 ? null : _unsigned(_at(_at(p, 0), 8));
    if (_u32(_at(p, 16)) != 0)
      values["output_tokens"] =
          _u32(_at(p, 16)) == 1 ? null : _unsigned(_at(_at(p, 16), 8));
    return values;
  }

  static Object? readQuestionSource(Pointer<Void> p) {
    final values = _extensions(_at(p, 40));
    values["answered_by"] = _text(_at(p, 0));
    if (_u32(_at(p, 16)) != 0)
      values["batch_size"] =
          _u32(_at(p, 16)) == 1 ? null : _unsigned(_at(_at(p, 16), 8));
    values["origin"] = readOrigin(_pointer(_at(p, 32)));
    return values;
  }

  static Object? readOrigin(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => "live",
      2 => "cache",
      3 => "replay",
      4 => "proxy",
      5 => "memory",
      _ => throw StateError('Invalid native enum'),
    };
  }

  static Object? readReadableQuestion(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => readReadableQuestionDecide(_pointer(_at(p, 8))),
      2 => readReadableQuestionChoose(_pointer(_at(p, 8))),
      3 => readReadableQuestionTag(_pointer(_at(p, 8))),
      4 => readReadableQuestionScore(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native discriminator'),
    };
  }

  static Object? readReadableQuestionScore(Pointer<Void> p) {
    final values = _extensions(_at(p, 224));
    if (_u32(_at(p, 0)) != 0)
      values["batch"] =
          _u32(_at(p, 0)) == 1 ? null : readBatch(_pointer(_at(_at(p, 0), 8)));
    if (_u32(_at(p, 16)) != 0)
      values["context_schema"] = _u32(_at(p, 16)) == 1
          ? null
          : readInputDeclaration(_pointer(_at(_at(p, 16), 8)));
    if (_u32(_at(p, 32)) != 0)
      values["item_schema"] = _u32(_at(p, 32)) == 1
          ? null
          : readInputDeclaration(_pointer(_at(_at(p, 32), 8)));
    if (_u32(_at(p, 48)) != 0)
      values["label_details"] = _u32(_at(p, 48)) == 1
          ? null
          : _list(_at(_at(p, 48), 8), 8, (p) => readLabel(_pointer(p)));
    if (_u32(_at(p, 72)) != 0)
      values["model"] =
          _u32(_at(p, 72)) == 1 ? null : _text(_at(_at(p, 72), 8));
    if (_u32(_at(p, 96)) != 0)
      values["name"] = _u32(_at(p, 96)) == 1
          ? null
          : readQuestionName(_pointer(_at(_at(p, 96), 8)));
    if (_u32(_at(p, 112)) != 0)
      values["on"] = _u32(_at(p, 112)) == 1
          ? null
          : _list(_at(_at(p, 112), 8), 16, (p) => _text(p));
    if (_u32(_at(p, 136)) != 0)
      values["profile"] =
          _u32(_at(p, 136)) == 1 ? null : _text(_at(_at(p, 136), 8));
    if (_u32(_at(p, 160)) != 0)
      values["wording_version"] = _u32(_at(p, 160)) == 1
          ? null
          : readWordingVersion(_pointer(_at(_at(p, 160), 8)));
    values["levels"] = _list(_at(p, 176), 16, (p) => _text(p));
    if (_u32(_at(p, 192)) != 0)
      values["text"] =
          _u32(_at(p, 192)) == 1 ? null : _value(_pointer(_at(_at(p, 192), 8)));
    values["verb"] = _text(_at(p, 208));
    return values;
  }

  static Object? readWordingVersion(Pointer<Void> p) {
    return _unsigned(_at(p, 0));
  }

  static Object? readQuestionName(Pointer<Void> p) {
    return _text(_at(p, 0));
  }

  static Object? readLabel(Pointer<Void> p) {
    final values = _extensions(_at(p, 32));
    if (_u32(_at(p, 0)) != 0)
      values["description"] =
          _u32(_at(p, 0)) == 1 ? null : _value(_pointer(_at(_at(p, 0), 8)));
    values["name"] = _text(_at(p, 16));
    return values;
  }

  static Object? readInputDeclaration(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => readInputDeclarationStringRoot(_pointer(_at(p, 8))),
      2 => readInputDeclarationObjectRoot(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native discriminator'),
    };
  }

  static Object? readInputDeclarationObjectRoot(Pointer<Void> p) {
    final values = _extensions(_at(p, 48));
    values["properties"] =
        _map(_at(p, 0), 24, 16, (p) => readInputPropertyType(_pointer(p)));
    if (_u32(_at(p, 16)) != 0)
      values["required"] = _u32(_at(p, 16)) == 1
          ? null
          : _list(_at(_at(p, 16), 8), 16, (p) => _text(p));
    values["type"] = readObjectType(_pointer(_at(p, 40)));
    return values;
  }

  static Object? readObjectType(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => "object",
      _ => throw StateError('Invalid native enum'),
    };
  }

  static Object? readInputPropertyType(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => readInputPropertyTypeString(_pointer(_at(p, 8))),
      2 => readInputPropertyTypeNumber(_pointer(_at(p, 8))),
      3 => readInputPropertyTypeBoolean(_pointer(_at(p, 8))),
      4 => readInputPropertyTypeArray(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native discriminator'),
    };
  }

  static Object? readInputPropertyTypeArray(Pointer<Void> p) {
    final values = _extensions(_at(p, 24));
    values["items"] = readStringRoot(_pointer(_at(p, 0)));
    values["type"] = _text(_at(p, 8));
    return values;
  }

  static Object? readStringRoot(Pointer<Void> p) {
    final values = _extensions(_at(p, 8));
    values["type"] = readStringType(_pointer(_at(p, 0)));
    return values;
  }

  static Object? readStringType(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => "string",
      _ => throw StateError('Invalid native enum'),
    };
  }

  static Object? readInputPropertyTypeBoolean(Pointer<Void> p) {
    final values = _extensions(_at(p, 16));
    values["type"] = _text(_at(p, 0));
    return values;
  }

  static Object? readInputPropertyTypeNumber(Pointer<Void> p) {
    final values = _extensions(_at(p, 16));
    values["type"] = _text(_at(p, 0));
    return values;
  }

  static Object? readInputPropertyTypeString(Pointer<Void> p) {
    final values = _extensions(_at(p, 16));
    values["type"] = _text(_at(p, 0));
    return values;
  }

  static Object? readInputDeclarationStringRoot(Pointer<Void> p) {
    final values = _extensions(_at(p, 8));
    values["type"] = readStringType(_pointer(_at(p, 0)));
    return values;
  }

  static Object? readBatch(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => _unsigned(_at(p, 8)),
      2 => readBatchString(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native value'),
    };
  }

  static Object? readReadableQuestionTag(Pointer<Void> p) {
    final values = _extensions(_at(p, 224));
    if (_u32(_at(p, 0)) != 0)
      values["batch"] =
          _u32(_at(p, 0)) == 1 ? null : readBatch(_pointer(_at(_at(p, 0), 8)));
    if (_u32(_at(p, 16)) != 0)
      values["context_schema"] = _u32(_at(p, 16)) == 1
          ? null
          : readInputDeclaration(_pointer(_at(_at(p, 16), 8)));
    if (_u32(_at(p, 32)) != 0)
      values["item_schema"] = _u32(_at(p, 32)) == 1
          ? null
          : readInputDeclaration(_pointer(_at(_at(p, 32), 8)));
    if (_u32(_at(p, 48)) != 0)
      values["label_details"] = _u32(_at(p, 48)) == 1
          ? null
          : _list(_at(_at(p, 48), 8), 8, (p) => readLabel(_pointer(p)));
    if (_u32(_at(p, 72)) != 0)
      values["model"] =
          _u32(_at(p, 72)) == 1 ? null : _text(_at(_at(p, 72), 8));
    if (_u32(_at(p, 96)) != 0)
      values["name"] = _u32(_at(p, 96)) == 1
          ? null
          : readQuestionName(_pointer(_at(_at(p, 96), 8)));
    if (_u32(_at(p, 112)) != 0)
      values["on"] = _u32(_at(p, 112)) == 1
          ? null
          : _list(_at(_at(p, 112), 8), 16, (p) => _text(p));
    if (_u32(_at(p, 136)) != 0)
      values["profile"] =
          _u32(_at(p, 136)) == 1 ? null : _text(_at(_at(p, 136), 8));
    if (_u32(_at(p, 160)) != 0)
      values["wording_version"] = _u32(_at(p, 160)) == 1
          ? null
          : readWordingVersion(_pointer(_at(_at(p, 160), 8)));
    values["labels"] = _list(_at(p, 176), 16, (p) => _text(p));
    if (_u32(_at(p, 192)) != 0)
      values["text"] =
          _u32(_at(p, 192)) == 1 ? null : _value(_pointer(_at(_at(p, 192), 8)));
    values["verb"] = _text(_at(p, 208));
    return values;
  }

  static Object? readReadableQuestionChoose(Pointer<Void> p) {
    final values = _extensions(_at(p, 224));
    if (_u32(_at(p, 0)) != 0)
      values["batch"] =
          _u32(_at(p, 0)) == 1 ? null : readBatch(_pointer(_at(_at(p, 0), 8)));
    if (_u32(_at(p, 16)) != 0)
      values["context_schema"] = _u32(_at(p, 16)) == 1
          ? null
          : readInputDeclaration(_pointer(_at(_at(p, 16), 8)));
    if (_u32(_at(p, 32)) != 0)
      values["item_schema"] = _u32(_at(p, 32)) == 1
          ? null
          : readInputDeclaration(_pointer(_at(_at(p, 32), 8)));
    if (_u32(_at(p, 48)) != 0)
      values["label_details"] = _u32(_at(p, 48)) == 1
          ? null
          : _list(_at(_at(p, 48), 8), 8, (p) => readLabel(_pointer(p)));
    if (_u32(_at(p, 72)) != 0)
      values["model"] =
          _u32(_at(p, 72)) == 1 ? null : _text(_at(_at(p, 72), 8));
    if (_u32(_at(p, 96)) != 0)
      values["name"] = _u32(_at(p, 96)) == 1
          ? null
          : readQuestionName(_pointer(_at(_at(p, 96), 8)));
    if (_u32(_at(p, 112)) != 0)
      values["on"] = _u32(_at(p, 112)) == 1
          ? null
          : _list(_at(_at(p, 112), 8), 16, (p) => _text(p));
    if (_u32(_at(p, 136)) != 0)
      values["profile"] =
          _u32(_at(p, 136)) == 1 ? null : _text(_at(_at(p, 136), 8));
    if (_u32(_at(p, 160)) != 0)
      values["wording_version"] = _u32(_at(p, 160)) == 1
          ? null
          : readWordingVersion(_pointer(_at(_at(p, 160), 8)));
    values["options"] = _list(_at(p, 176), 16, (p) => _text(p));
    if (_u32(_at(p, 192)) != 0)
      values["text"] =
          _u32(_at(p, 192)) == 1 ? null : _value(_pointer(_at(_at(p, 192), 8)));
    values["verb"] = _text(_at(p, 208));
    return values;
  }

  static Object? readReadableQuestionDecide(Pointer<Void> p) {
    final values = _extensions(_at(p, 240));
    if (_u32(_at(p, 0)) != 0)
      values["batch"] =
          _u32(_at(p, 0)) == 1 ? null : readBatch(_pointer(_at(_at(p, 0), 8)));
    if (_u32(_at(p, 16)) != 0)
      values["context_schema"] = _u32(_at(p, 16)) == 1
          ? null
          : readInputDeclaration(_pointer(_at(_at(p, 16), 8)));
    if (_u32(_at(p, 32)) != 0)
      values["item_schema"] = _u32(_at(p, 32)) == 1
          ? null
          : readInputDeclaration(_pointer(_at(_at(p, 32), 8)));
    if (_u32(_at(p, 48)) != 0)
      values["label_details"] = _u32(_at(p, 48)) == 1
          ? null
          : _list(_at(_at(p, 48), 8), 8, (p) => readLabel(_pointer(p)));
    if (_u32(_at(p, 72)) != 0)
      values["model"] =
          _u32(_at(p, 72)) == 1 ? null : _text(_at(_at(p, 72), 8));
    if (_u32(_at(p, 96)) != 0)
      values["name"] = _u32(_at(p, 96)) == 1
          ? null
          : readQuestionName(_pointer(_at(_at(p, 96), 8)));
    if (_u32(_at(p, 112)) != 0)
      values["on"] = _u32(_at(p, 112)) == 1
          ? null
          : _list(_at(_at(p, 112), 8), 16, (p) => _text(p));
    if (_u32(_at(p, 136)) != 0)
      values["profile"] =
          _u32(_at(p, 136)) == 1 ? null : _text(_at(_at(p, 136), 8));
    if (_u32(_at(p, 160)) != 0)
      values["wording_version"] = _u32(_at(p, 160)) == 1
          ? null
          : readWordingVersion(_pointer(_at(_at(p, 160), 8)));
    if (_u32(_at(p, 176)) != 0)
      values["false"] =
          _u32(_at(p, 176)) == 1 ? null : _value(_pointer(_at(_at(p, 176), 8)));
    if (_u32(_at(p, 192)) != 0)
      values["text"] =
          _u32(_at(p, 192)) == 1 ? null : _value(_pointer(_at(_at(p, 192), 8)));
    if (_u32(_at(p, 208)) != 0)
      values["true"] =
          _u32(_at(p, 208)) == 1 ? null : _value(_pointer(_at(_at(p, 208), 8)));
    values["verb"] = _text(_at(p, 224));
    return values;
  }

  static Object? readSessionProbabilities(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => readSessionProbabilitiesYesNo(_pointer(_at(p, 8))),
      2 => readSessionProbabilitiesNamed(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native discriminator'),
    };
  }

  static Object? readSessionProbabilitiesNamed(Pointer<Void> p) {
    final values = _extensions(_at(p, 32));
    values["kind"] = _text(_at(p, 0));
    values["value"] =
        _list(_at(p, 16), 8, (p) => readSessionNamedProbability(_pointer(p)));
    return values;
  }

  static Object? readSessionNamedProbability(Pointer<Void> p) {
    final values = _extensions(_at(p, 24));
    values["name"] = _text(_at(p, 0));
    values["probability"] = _at(p, 16).cast<Double>().value;
    return values;
  }

  static Object? readSessionProbabilitiesYesNo(Pointer<Void> p) {
    final values = _extensions(_at(p, 24));
    values["kind"] = _text(_at(p, 0));
    values["value"] = _at(p, 16).cast<Double>().value;
    return values;
  }

  static Object? readObservation(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => readObservationObservationId(_pointer(_at(p, 8))),
      2 => readObservationFailureId(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native discriminator'),
    };
  }

  static Object? readObservationFailureId(Pointer<Void> p) {
    final values = _extensions(_at(p, 8));
    values["failure_id"] = readFailureId(_pointer(_at(p, 0)));
    return values;
  }

  static Object? readFailureId(Pointer<Void> p) {
    return _text(_at(p, 0));
  }

  static Object? readObservationObservationId(Pointer<Void> p) {
    final values = _extensions(_at(p, 8));
    values["observation_id"] = readObservationId(_pointer(_at(p, 0)));
    return values;
  }

  static Object? readObservationId(Pointer<Void> p) {
    return _text(_at(p, 0));
  }

  static Object? readSessionInputSource(Pointer<Void> p) {
    final values = _extensions(_at(p, 16));
    values["index"] = _unsigned(_at(p, 0));
    values["source"] = readPhysicalSource(_pointer(_at(p, 8)));
    return values;
  }

  static Object? readPhysicalSource(Pointer<Void> p) {
    final values = _extensions(_at(p, 48));
    values["file"] = _text(_at(p, 0));
    if (_u32(_at(p, 16)) != 0)
      values["first_line"] =
          _u32(_at(p, 16)) == 1 ? null : _unsigned(_at(_at(p, 16), 8));
    if (_u32(_at(p, 32)) != 0)
      values["last_line"] =
          _u32(_at(p, 32)) == 1 ? null : _unsigned(_at(_at(p, 32), 8));
    return values;
  }

  static Object? readAnswerId(Pointer<Void> p) {
    return _text(_at(p, 0));
  }

  static Object? readRequestFunction(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => "decide",
      2 => "choose",
      3 => "tag",
      4 => "score",
      5 => "filter",
      6 => "rank",
      7 => "find",
      8 => "annotate",
      9 => "recognize",
      10 => "relate",
      _ => throw StateError('Invalid native enum'),
    };
  }

  static Object? readSessionPacketRelateAggregate(Pointer<Void> p) {
    final values = _extensions(_at(p, 40));
    values["function"] = _text(_at(p, 0));
    values["kind"] = _text(_at(p, 16));
    values["value"] = readRelation(_pointer(_at(p, 32)));
    return values;
  }

  static Object? readRelation(Pointer<Void> p) {
    final values = _extensions(_at(p, 184));
    values["answer"] = readAnswers(_pointer(_at(p, 0)));
    values["answer_id"] = readAnswerId(_pointer(_at(p, 8)));
    if (_u32(_at(p, 16)) != 0)
      values["file"] = _u32(_at(p, 16)) == 1 ? null : _text(_at(_at(p, 16), 8));
    if (_u32(_at(p, 40)) != 0)
      values["first_line"] =
          _u32(_at(p, 40)) == 1 ? null : _unsigned(_at(_at(p, 40), 8));
    if (_u32(_at(p, 56)) != 0)
      values["index"] =
          _u32(_at(p, 56)) == 1 ? null : _unsigned(_at(_at(p, 56), 8));
    if (_u32(_at(p, 72)) != 0)
      values["input"] =
          _u32(_at(p, 72)) == 1 ? null : _value(_pointer(_at(_at(p, 72), 8)));
    if (_u32(_at(p, 88)) != 0)
      values["input_sources"] = _u32(_at(p, 88)) == 1
          ? null
          : _list(_at(_at(p, 88), 8), 8,
              (p) => readSessionInputSource(_pointer(p)));
    if (_u32(_at(p, 112)) != 0)
      values["last_line"] =
          _u32(_at(p, 112)) == 1 ? null : _unsigned(_at(_at(p, 112), 8));
    values["meta"] = readMeta(_pointer(_at(p, 128)));
    if (_u32(_at(p, 136)) != 0)
      values["position"] = _u32(_at(p, 136)) == 1
          ? null
          : readPosition(_pointer(_at(_at(p, 136), 8)));
    values["question"] = readRelationQuestionInfo(_pointer(_at(p, 152)));
    values["schema"] = readVersion(_pointer(_at(p, 160)));
    values["value"] =
        _list(_at(p, 168), 8, (p) => readRelatedEntityEdge(_pointer(p)));
    return values;
  }

  static Object? readRelatedEntityEdge(Pointer<Void> p) {
    final values = _extensions(_at(p, 48));
    if (_u32(_at(p, 0)) != 0)
      values["either"] =
          _u32(_at(p, 0)) == 1 ? null : _u32(_at(_at(p, 0), 4)) != 0;
    values["probability"] = _at(p, 8).cast<Double>().value;
    values["relation"] = _text(_at(p, 16));
    values["source"] = readRelationEndpoint(_pointer(_at(p, 32)));
    values["target"] = readRelationEndpoint(_pointer(_at(p, 40)));
    return values;
  }

  static Object? readRelationEndpoint(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => readRelationEndpointRelatedEntity(_pointer(_at(p, 8))),
      2 => readRelationEndpointSource(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native discriminator'),
    };
  }

  static Object? readRelationEndpointSource(Pointer<Void> p) {
    final values = _extensions(_at(p, 112));
    if (_u32(_at(p, 0)) != 0)
      values["file"] = _u32(_at(p, 0)) == 1 ? null : _text(_at(_at(p, 0), 8));
    if (_u32(_at(p, 24)) != 0)
      values["first_line"] =
          _u32(_at(p, 24)) == 1 ? null : _unsigned(_at(_at(p, 24), 8));
    values["kind"] = _text(_at(p, 40));
    if (_u32(_at(p, 56)) != 0)
      values["last_line"] =
          _u32(_at(p, 56)) == 1 ? null : _unsigned(_at(_at(p, 56), 8));
    values["name"] = _text(_at(p, 72));
    values["ordinal"] = _unsigned(_at(p, 88));
    if (_u32(_at(p, 96)) != 0)
      values["record"] =
          _u32(_at(p, 96)) == 1 ? null : _value(_pointer(_at(_at(p, 96), 8)));
    return values;
  }

  static Object? readRelationEndpointRelatedEntity(Pointer<Void> p) {
    final values = _extensions(_at(p, 32));
    values["kind"] = _text(_at(p, 0));
    values["name"] = _text(_at(p, 16));
    return values;
  }

  static Object? readVersion(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => "thinkthen.result/2",
      _ => throw StateError('Invalid native enum'),
    };
  }

  static Object? readRelationQuestionInfo(Pointer<Void> p) {
    final values = _extensions(_at(p, 232));
    if (_u32(_at(p, 0)) != 0)
      values["batch"] =
          _u32(_at(p, 0)) == 1 ? null : readBatch(_pointer(_at(_at(p, 0), 8)));
    if (_u32(_at(p, 16)) != 0)
      values["context_schema"] = _u32(_at(p, 16)) == 1
          ? null
          : readInputDeclaration(_pointer(_at(_at(p, 16), 8)));
    if (_u32(_at(p, 32)) != 0)
      values["fields"] = _u32(_at(p, 32)) == 1
          ? null
          : readRelateFields(_pointer(_at(_at(p, 32), 8)));
    if (_u32(_at(p, 48)) != 0)
      values["item_schema"] = _u32(_at(p, 48)) == 1
          ? null
          : readInputDeclaration(_pointer(_at(_at(p, 48), 8)));
    if (_u32(_at(p, 64)) != 0)
      values["label_details"] = _u32(_at(p, 64)) == 1
          ? null
          : _list(_at(_at(p, 64), 8), 8, (p) => readLabel(_pointer(p)));
    if (_u32(_at(p, 88)) != 0)
      values["model"] =
          _u32(_at(p, 88)) == 1 ? null : _text(_at(_at(p, 88), 8));
    if (_u32(_at(p, 112)) != 0)
      values["name"] = _u32(_at(p, 112)) == 1
          ? null
          : readQuestionName(_pointer(_at(_at(p, 112), 8)));
    if (_u32(_at(p, 128)) != 0)
      values["on"] = _u32(_at(p, 128)) == 1
          ? null
          : _list(_at(_at(p, 128), 8), 16, (p) => _text(p));
    if (_u32(_at(p, 152)) != 0)
      values["profile"] =
          _u32(_at(p, 152)) == 1 ? null : _text(_at(_at(p, 152), 8));
    values["relations"] =
        _list(_at(p, 176), 8, (p) => readRelationRule(_pointer(p)));
    values["threshold"] = readThreshold(_pointer(_at(p, 192)));
    values["verb"] = _text(_at(p, 200));
    if (_u32(_at(p, 216)) != 0)
      values["wording_version"] = _u32(_at(p, 216)) == 1
          ? null
          : readWordingVersion(_pointer(_at(_at(p, 216), 8)));
    return values;
  }

  static Object? readRelationRule(Pointer<Void> p) {
    final values = _extensions(_at(p, 80));
    values["either"] = _u32(_at(p, 0)) != 0;
    values["name"] = _text(_at(p, 8));
    values["reads"] = _text(_at(p, 24));
    if (_u32(_at(p, 40)) != 0)
      values["single"] =
          _u32(_at(p, 40)) == 1 ? null : _u32(_at(_at(p, 40), 4)) != 0;
    values["source"] = _text(_at(p, 48));
    values["target"] = _text(_at(p, 64));
    return values;
  }

  static Object? readRelateFields(Pointer<Void> p) {
    final values = _extensions(_at(p, 32));
    values["kind"] = _text(_at(p, 0));
    values["name"] = _text(_at(p, 16));
    return values;
  }

  static Object? readPosition(Pointer<Void> p) {
    final values = _extensions(_at(p, 80));
    if (_u32(_at(p, 0)) != 0)
      values["file"] = _u32(_at(p, 0)) == 1 ? null : _text(_at(_at(p, 0), 8));
    if (_u32(_at(p, 24)) != 0)
      values["first"] =
          _u32(_at(p, 24)) == 1 ? null : _unsigned(_at(_at(p, 24), 8));
    if (_u32(_at(p, 40)) != 0)
      values["images"] = _u32(_at(p, 40)) == 1
          ? null
          : _list(_at(_at(p, 40), 8), 16, (p) => _text(p));
    if (_u32(_at(p, 64)) != 0)
      values["last"] =
          _u32(_at(p, 64)) == 1 ? null : _unsigned(_at(_at(p, 64), 8));
    return values;
  }

  static Object? readMeta(Pointer<Void> p) {
    final values = _extensions(_at(p, 320));
    if (_u32(_at(p, 0)) != 0)
      values["answered_by"] =
          _u32(_at(p, 0)) == 1 ? null : _text(_at(_at(p, 0), 8));
    if (_u32(_at(p, 24)) != 0)
      values["attempts"] = _u32(_at(p, 24)) == 1
          ? null
          : _list(_at(_at(p, 24), 8), 8, (p) => readAttempt(_pointer(p)));
    if (_u32(_at(p, 48)) != 0)
      values["batch_setting"] = _u32(_at(p, 48)) == 1
          ? null
          : readBatchSetting(_pointer(_at(_at(p, 48), 8)));
    if (_u32(_at(p, 64)) != 0)
      values["batch_warning"] = _u32(_at(p, 64)) == 1
          ? null
          : readBatchWarning(_pointer(_at(_at(p, 64), 8)));
    values["cached"] = _u32(_at(p, 80)) != 0;
    if (_u32(_at(p, 88)) != 0)
      values["context_sha256"] =
          _u32(_at(p, 88)) == 1 ? null : _text(_at(_at(p, 88), 8));
    values["failed_questions"] = _unsigned(_at(p, 112));
    values["model"] = _text(_at(p, 120));
    values["observations"] =
        _list(_at(p, 136), 8, (p) => readObservation(_pointer(p)));
    if (_u32(_at(p, 152)) != 0)
      values["origin"] = _u32(_at(p, 152)) == 1
          ? null
          : readOrigin(_pointer(_at(_at(p, 152), 8)));
    if (_u32(_at(p, 168)) != 0)
      values["profile_warning"] = _u32(_at(p, 168)) == 1
          ? null
          : readProfileWarning(_pointer(_at(_at(p, 168), 8)));
    if (_u32(_at(p, 184)) != 0)
      values["question_sha256"] =
          _u32(_at(p, 184)) == 1 ? null : _text(_at(_at(p, 184), 8));
    values["question_sources"] =
        _list(_at(p, 208), 8, (p) => readQuestionSource(_pointer(p)));
    if (_u32(_at(p, 224)) != 0)
      values["questions_sha256"] =
          _u32(_at(p, 224)) == 1 ? null : _text(_at(_at(p, 224), 8));
    values["requests"] = _list(_at(p, 248), 16, (p) => _text(p));
    values["requests_sent"] = _unsigned(_at(p, 264));
    values["tool"] = _text(_at(p, 272));
    values["url"] = _text(_at(p, 288));
    if (_u32(_at(p, 304)) != 0)
      values["usage"] = _u32(_at(p, 304)) == 1
          ? null
          : readUsage(_pointer(_at(_at(p, 304), 8)));
    return values;
  }

  static Object? readProfileWarning(Pointer<Void> p) {
    final values = _extensions(_at(p, 32));
    values["running"] = _text(_at(p, 0));
    values["tuned_for"] = _text(_at(p, 16));
    return values;
  }

  static Object? readBatchWarning(Pointer<Void> p) {
    final values = _extensions(_at(p, 16));
    values["running"] = readBatchSetting(_pointer(_at(p, 0)));
    values["tuned_for"] = readBatchSetting(_pointer(_at(p, 8)));
    return values;
  }

  static Object? readBatchSetting(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => _unsigned(_at(p, 8)),
      2 => _text(_at(p, 8)),
      _ => throw StateError('Invalid native value'),
    };
  }

  static Object? readAnswers(Pointer<Void> p) {
    final values = _extensions(_at(p, 16));
    values["questions"] =
        _list(_at(p, 0), 8, (p) => readRelationMember(_pointer(p)));
    return values;
  }

  static Object? readRelationMember(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => readRelationMemberAnswerId(_pointer(_at(p, 8))),
      2 => readRelationMemberFailureId(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native discriminator'),
    };
  }

  static Object? readRelationMemberFailureId(Pointer<Void> p) {
    final values = _extensions(_at(p, 168));
    values["direction"] = readRelationDirection(_pointer(_at(p, 0)));
    values["method"] = readRelationMethod(_pointer(_at(p, 8)));
    values["observations"] =
        _list(_at(p, 16), 8, (p) => readObservation(_pointer(p)));
    values["question"] = readReadableQuestion(_pointer(_at(p, 32)));
    values["question_sources"] =
        _list(_at(p, 40), 8, (p) => readQuestionSource(_pointer(p)));
    values["reads"] = _text(_at(p, 56));
    values["relation"] = _text(_at(p, 72));
    values["request"] = _text(_at(p, 88));
    values["source"] = readRelatedEntity(_pointer(_at(p, 104)));
    if (_u32(_at(p, 112)) != 0)
      values["target"] = _u32(_at(p, 112)) == 1
          ? null
          : readRelatedEntity(_pointer(_at(_at(p, 112), 8)));
    values["threshold"] = readThreshold(_pointer(_at(p, 128)));
    if (_u32(_at(p, 136)) != 0)
      values["usage"] = _u32(_at(p, 136)) == 1
          ? null
          : readUsage(_pointer(_at(_at(p, 136), 8)));
    values["failure"] = readFailure(_pointer(_at(p, 152)));
    values["failure_id"] = readFailureId(_pointer(_at(p, 160)));
    return values;
  }

  static Object? readRelatedEntity(Pointer<Void> p) {
    final values = _extensions(_at(p, 32));
    values["kind"] = _text(_at(p, 0));
    values["name"] = _text(_at(p, 16));
    return values;
  }

  static Object? readRelationMethod(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => "yes_no",
      2 => "choice",
      _ => throw StateError('Invalid native enum'),
    };
  }

  static Object? readRelationDirection(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => "source_to_target",
      2 => "either",
      _ => throw StateError('Invalid native enum'),
    };
  }

  static Object? readRelationMemberAnswerId(Pointer<Void> p) {
    final values = _extensions(_at(p, 184));
    values["direction"] = readRelationDirection(_pointer(_at(p, 0)));
    values["method"] = readRelationMethod(_pointer(_at(p, 8)));
    values["observations"] =
        _list(_at(p, 16), 8, (p) => readObservation(_pointer(p)));
    values["question"] = readReadableQuestion(_pointer(_at(p, 32)));
    values["question_sources"] =
        _list(_at(p, 40), 8, (p) => readQuestionSource(_pointer(p)));
    values["reads"] = _text(_at(p, 56));
    values["relation"] = _text(_at(p, 72));
    values["request"] = _text(_at(p, 88));
    values["source"] = readRelatedEntity(_pointer(_at(p, 104)));
    if (_u32(_at(p, 112)) != 0)
      values["target"] = _u32(_at(p, 112)) == 1
          ? null
          : readRelatedEntity(_pointer(_at(_at(p, 112), 8)));
    values["threshold"] = readThreshold(_pointer(_at(p, 128)));
    if (_u32(_at(p, 136)) != 0)
      values["usage"] = _u32(_at(p, 136)) == 1
          ? null
          : readUsage(_pointer(_at(_at(p, 136), 8)));
    values["accepted"] = _u32(_at(p, 152)) != 0;
    values["answer"] = readAnswer(_pointer(_at(p, 160)));
    values["answer_id"] = readAnswerId(_pointer(_at(p, 168)));
    values["probability"] = _at(p, 176).cast<Double>().value;
    return values;
  }

  static Object? readAnswer(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => readAnswerYesNo(_pointer(_at(p, 8))),
      2 => readAnswerChoice(_pointer(_at(p, 8))),
      3 => readAnswerTag(_pointer(_at(p, 8))),
      4 => readAnswerScore(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native discriminator'),
    };
  }

  static Object? readAnswerScore(Pointer<Void> p) {
    final values = _extensions(_at(p, 64));
    if (_u32(_at(p, 0)) != 0)
      values["confidence"] =
          _u32(_at(p, 0)) == 1 ? null : _at(_at(p, 0), 8).cast<Double>().value;
    values["kind"] = _text(_at(p, 16));
    values["level"] = _text(_at(p, 32));
    values["probabilities"] =
        _map(_at(p, 48), 24, 16, (p) => p.cast<Double>().value);
    return values;
  }

  static Object? readAnswerTag(Pointer<Void> p) {
    final values = _extensions(_at(p, 32));
    values["kind"] = _text(_at(p, 0));
    values["probabilities"] =
        _map(_at(p, 16), 24, 16, (p) => p.cast<Double>().value);
    return values;
  }

  static Object? readAnswerChoice(Pointer<Void> p) {
    final values = _extensions(_at(p, 64));
    if (_u32(_at(p, 0)) != 0)
      values["confidence"] =
          _u32(_at(p, 0)) == 1 ? null : _at(_at(p, 0), 8).cast<Double>().value;
    values["kind"] = _text(_at(p, 16));
    values["pick"] = _text(_at(p, 32));
    values["probabilities"] =
        _map(_at(p, 48), 24, 16, (p) => p.cast<Double>().value);
    return values;
  }

  static Object? readAnswerYesNo(Pointer<Void> p) {
    final values = _extensions(_at(p, 24));
    values["kind"] = _text(_at(p, 0));
    values["probability"] = _at(p, 16).cast<Double>().value;
    return values;
  }

  static Object? readSessionPacketRecognizeAggregate(Pointer<Void> p) {
    final values = _extensions(_at(p, 48));
    values["function"] = _text(_at(p, 0));
    values["kind"] = _text(_at(p, 16));
    values["value"] = _list(_at(p, 32), 8, (p) => readRecognition(_pointer(p)));
    return values;
  }

  static Object? readRecognition(Pointer<Void> p) {
    final values = _extensions(_at(p, 168));
    values["answer"] = readRecognitionOdds(_pointer(_at(p, 0)));
    values["answer_id"] = readAnswerId(_pointer(_at(p, 8)));
    if (_u32(_at(p, 16)) != 0)
      values["file"] = _u32(_at(p, 16)) == 1 ? null : _text(_at(_at(p, 16), 8));
    if (_u32(_at(p, 40)) != 0)
      values["first_line"] =
          _u32(_at(p, 40)) == 1 ? null : _unsigned(_at(_at(p, 40), 8));
    if (_u32(_at(p, 56)) != 0)
      values["index"] =
          _u32(_at(p, 56)) == 1 ? null : _unsigned(_at(_at(p, 56), 8));
    if (_u32(_at(p, 72)) != 0)
      values["input"] =
          _u32(_at(p, 72)) == 1 ? null : _value(_pointer(_at(_at(p, 72), 8)));
    if (_u32(_at(p, 88)) != 0)
      values["last_line"] =
          _u32(_at(p, 88)) == 1 ? null : _unsigned(_at(_at(p, 88), 8));
    values["meta"] = readMeta(_pointer(_at(p, 104)));
    if (_u32(_at(p, 112)) != 0)
      values["position"] = _u32(_at(p, 112)) == 1
          ? null
          : readPosition(_pointer(_at(_at(p, 112), 8)));
    values["question"] = readRecognitionQuestionInfo(_pointer(_at(p, 128)));
    values["schema"] = readVersion(_pointer(_at(p, 136)));
    if (_u32(_at(p, 144)) != 0)
      values["source"] = _u32(_at(p, 144)) == 1
          ? null
          : readPhysicalSource(_pointer(_at(_at(p, 144), 8)));
    values["value"] = readRecognize(_pointer(_at(p, 160)));
    return values;
  }

  static Object? readRecognize(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => readRecognizeEntities(_pointer(_at(p, 8))),
      2 => readRecognizeModeProposals(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native discriminator'),
    };
  }

  static Object? readRecognizeModeProposals(Pointer<Void> p) {
    final values = _extensions(_at(p, 24));
    values["mode"] = readBoundaryMode(_pointer(_at(p, 0)));
    values["proposals"] =
        _list(_at(p, 8), 8, (p) => readBoundaryProposal(_pointer(p)));
    return values;
  }

  static Object? readBoundaryMode(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => "boundary_only",
      _ => throw StateError('Invalid native enum'),
    };
  }

  static Object? readRecognizeEntities(Pointer<Void> p) {
    final values = _extensions(_at(p, 40));
    values["entities"] = _list(_at(p, 0), 8, (p) => readEntity(_pointer(p)));
    if (_u32(_at(p, 16)) != 0)
      values["relations"] = _u32(_at(p, 16)) == 1
          ? null
          : _list(_at(_at(p, 16), 8), 8, (p) => readEntityEdge(_pointer(p)));
    return values;
  }

  static Object? readEntityEdge(Pointer<Void> p) {
    final values = _extensions(_at(p, 48));
    if (_u32(_at(p, 0)) != 0)
      values["either"] =
          _u32(_at(p, 0)) == 1 ? null : _u32(_at(_at(p, 0), 4)) != 0;
    values["probability"] = _at(p, 8).cast<Double>().value;
    values["relation"] = _text(_at(p, 16));
    values["source"] = readEntity(_pointer(_at(p, 32)));
    values["target"] = readEntity(_pointer(_at(p, 40)));
    return values;
  }

  static Object? readRecognitionQuestionInfo(Pointer<Void> p) {
    final values = _extensions(_at(p, 328));
    if (_u32(_at(p, 0)) != 0)
      values["batch"] =
          _u32(_at(p, 0)) == 1 ? null : readBatch(_pointer(_at(_at(p, 0), 8)));
    if (_u32(_at(p, 16)) != 0)
      values["context_schema"] = _u32(_at(p, 16)) == 1
          ? null
          : readInputDeclaration(_pointer(_at(_at(p, 16), 8)));
    if (_u32(_at(p, 32)) != 0)
      values["entity_definition"] =
          _u32(_at(p, 32)) == 1 ? null : _value(_pointer(_at(_at(p, 32), 8)));
    if (_u32(_at(p, 48)) != 0)
      values["instructions"] =
          _u32(_at(p, 48)) == 1 ? null : _value(_pointer(_at(_at(p, 48), 8)));
    if (_u32(_at(p, 64)) != 0)
      values["item_schema"] = _u32(_at(p, 64)) == 1
          ? null
          : readInputDeclaration(_pointer(_at(_at(p, 64), 8)));
    values["kinds"] = _map(_at(p, 80), 24, 16, (p) => _value(_pointer(p)));
    if (_u32(_at(p, 96)) != 0)
      values["label_details"] = _u32(_at(p, 96)) == 1
          ? null
          : _list(_at(_at(p, 96), 8), 8, (p) => readLabel(_pointer(p)));
    if (_u32(_at(p, 120)) != 0)
      values["mode"] = _u32(_at(p, 120)) == 1
          ? null
          : readRecognitionMode(_pointer(_at(_at(p, 120), 8)));
    if (_u32(_at(p, 136)) != 0)
      values["model"] =
          _u32(_at(p, 136)) == 1 ? null : _text(_at(_at(p, 136), 8));
    if (_u32(_at(p, 160)) != 0)
      values["name"] = _u32(_at(p, 160)) == 1
          ? null
          : readQuestionName(_pointer(_at(_at(p, 160), 8)));
    if (_u32(_at(p, 176)) != 0)
      values["on"] = _u32(_at(p, 176)) == 1
          ? null
          : _list(_at(_at(p, 176), 8), 16, (p) => _text(p));
    if (_u32(_at(p, 200)) != 0)
      values["profile"] =
          _u32(_at(p, 200)) == 1 ? null : _text(_at(_at(p, 200), 8));
    if (_u32(_at(p, 224)) != 0)
      values["relation_threshold"] = _u32(_at(p, 224)) == 1
          ? null
          : readThreshold(_pointer(_at(_at(p, 224), 8)));
    if (_u32(_at(p, 240)) != 0)
      values["relations"] = _u32(_at(p, 240)) == 1
          ? null
          : _list(_at(_at(p, 240), 8), 8, (p) => readRelationRule(_pointer(p)));
    if (_u32(_at(p, 264)) != 0)
      values["snippet_pieces"] =
          _u32(_at(p, 264)) == 1 ? null : _unsigned(_at(_at(p, 264), 8));
    if (_u32(_at(p, 280)) != 0)
      values["stage_context"] = _u32(_at(p, 280)) == 1
          ? null
          : readRecognitionStageContext(_pointer(_at(_at(p, 280), 8)));
    values["threshold"] = readThreshold(_pointer(_at(p, 296)));
    values["verb"] = readVerb(_pointer(_at(p, 304)));
    if (_u32(_at(p, 312)) != 0)
      values["wording_version"] = _u32(_at(p, 312)) == 1
          ? null
          : readWordingVersion(_pointer(_at(_at(p, 312), 8)));
    return values;
  }

  static Object? readVerb(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => "recognize",
      _ => throw StateError('Invalid native enum'),
    };
  }

  static Object? readRecognitionStageContext(Pointer<Void> p) {
    final values = _extensions(_at(p, 72));
    if (_u32(_at(p, 0)) != 0)
      values["boundary"] =
          _u32(_at(p, 0)) == 1 ? null : _text(_at(_at(p, 0), 8));
    if (_u32(_at(p, 24)) != 0)
      values["kind_edge"] =
          _u32(_at(p, 24)) == 1 ? null : _text(_at(_at(p, 24), 8));
    if (_u32(_at(p, 48)) != 0)
      values["relation"] =
          _u32(_at(p, 48)) == 1 ? null : _text(_at(_at(p, 48), 8));
    return values;
  }

  static Object? readRecognitionOdds(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => readRecognitionOddsRecognizeAnswer(_pointer(_at(p, 8))),
      2 => readRecognitionOddsBoundaryOdds(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native discriminator'),
    };
  }

  static Object? readRecognitionOddsBoundaryOdds(Pointer<Void> p) {
    final values = _extensions(_at(p, 32));
    values["pieces"] = _list(_at(p, 0), 8, (p) => readPieceOdds(_pointer(p)));
    values["proposals"] =
        _list(_at(p, 16), 8, (p) => readBoundaryProposal(_pointer(p)));
    return values;
  }

  static Object? readPieceOdds(Pointer<Void> p) {
    final values = _extensions(_at(p, 32));
    values["end"] = _unsigned(_at(p, 0));
    values["start"] = _unsigned(_at(p, 8));
    values["tags"] = _map(_at(p, 16), 24, 16, (p) => p.cast<Double>().value);
    return values;
  }

  static Object? readRecognitionOddsRecognizeAnswer(Pointer<Void> p) {
    final values = _extensions(_at(p, 64));
    values["names"] = _list(_at(p, 0), 8, (p) => readNameOdds(_pointer(p)));
    values["pairs"] = _list(_at(p, 16), 8, (p) => readPairOdds(_pointer(p)));
    values["pieces"] = _list(_at(p, 32), 8, (p) => readPieceOdds(_pointer(p)));
    values["proposals"] =
        _list(_at(p, 48), 8, (p) => readRecognitionProposal(_pointer(p)));
    return values;
  }

  static Object? readRecognitionProposal(Pointer<Void> p) {
    final values = _extensions(_at(p, 88));
    values["end"] = _unsigned(_at(p, 0));
    values["kept"] = _u32(_at(p, 8)) != 0;
    if (_u32(_at(p, 16)) != 0)
      values["kind"] = _u32(_at(p, 16)) == 1 ? null : _text(_at(_at(p, 16), 8));
    if (_u32(_at(p, 40)) != 0)
      values["selected"] = _u32(_at(p, 40)) == 1
          ? null
          : readPlace(_pointer(_at(_at(p, 40), 8)));
    values["span_probability"] = _at(p, 56).cast<Double>().value;
    values["start"] = _unsigned(_at(p, 64));
    if (_u32(_at(p, 72)) != 0)
      values["strength"] = _u32(_at(p, 72)) == 1
          ? null
          : _at(_at(p, 72), 8).cast<Double>().value;
    return values;
  }

  static Object? readPlace(Pointer<Void> p) {
    final values = _extensions(_at(p, 16));
    values["end"] = _unsigned(_at(p, 0));
    values["start"] = _unsigned(_at(p, 8));
    return values;
  }

  static Object? readPairOdds(Pointer<Void> p) {
    final values = _extensions(_at(p, 40));
    values["probability"] = _at(p, 0).cast<Double>().value;
    values["relation"] = _text(_at(p, 8));
    values["source"] = readPlace(_pointer(_at(p, 24)));
    values["target"] = readPlace(_pointer(_at(p, 32)));
    return values;
  }

  static Object? readNameOdds(Pointer<Void> p) {
    final values = _extensions(_at(p, 64));
    if (_u32(_at(p, 0)) != 0)
      values["edges"] = _u32(_at(p, 0)) == 1
          ? null
          : _map(_at(_at(p, 0), 8), 24, 16, (p) => p.cast<Double>().value);
    values["end"] = _unsigned(_at(p, 24));
    if (_u32(_at(p, 32)) != 0)
      values["kinds"] = _u32(_at(p, 32)) == 1
          ? null
          : _map(_at(_at(p, 32), 8), 24, 16, (p) => p.cast<Double>().value);
    values["start"] = _unsigned(_at(p, 56));
    return values;
  }

  static Object? readSessionPacketAnnotateAggregate(Pointer<Void> p) {
    final values = _extensions(_at(p, 48));
    values["function"] = _text(_at(p, 0));
    values["kind"] = _text(_at(p, 16));
    values["value"] = _list(_at(p, 32), 8, (p) => readAnnotation(_pointer(p)));
    return values;
  }

  static Object? readAnnotation(Pointer<Void> p) {
    final values = _extensions(_at(p, 168));
    values["answer_id"] = readAnswerId(_pointer(_at(p, 0)));
    values["answers"] =
        _map(_at(p, 8), 24, 16, (p) => readAnnotationMember(_pointer(p)));
    if (_u32(_at(p, 24)) != 0)
      values["file"] = _u32(_at(p, 24)) == 1 ? null : _text(_at(_at(p, 24), 8));
    if (_u32(_at(p, 48)) != 0)
      values["first_line"] =
          _u32(_at(p, 48)) == 1 ? null : _unsigned(_at(_at(p, 48), 8));
    if (_u32(_at(p, 64)) != 0)
      values["index"] =
          _u32(_at(p, 64)) == 1 ? null : _unsigned(_at(_at(p, 64), 8));
    if (_u32(_at(p, 80)) != 0)
      values["input"] =
          _u32(_at(p, 80)) == 1 ? null : _value(_pointer(_at(_at(p, 80), 8)));
    if (_u32(_at(p, 96)) != 0)
      values["last_line"] =
          _u32(_at(p, 96)) == 1 ? null : _unsigned(_at(_at(p, 96), 8));
    values["meta"] = readMeta(_pointer(_at(p, 112)));
    if (_u32(_at(p, 120)) != 0)
      values["position"] = _u32(_at(p, 120)) == 1
          ? null
          : readPosition(_pointer(_at(_at(p, 120), 8)));
    values["schema"] = readVersion(_pointer(_at(p, 136)));
    if (_u32(_at(p, 144)) != 0)
      values["source"] = _u32(_at(p, 144)) == 1
          ? null
          : readPhysicalSource(_pointer(_at(_at(p, 144), 8)));
    values["value"] = readAnnotatedRow(_pointer(_at(p, 160)));
    return values;
  }

  static Object? readAnnotatedRow(Pointer<Void> p) {
    return _map(_at(p, 0), 24, 16, (p) => readAnnotatedField(_pointer(p)));
  }

  static Object? readAnnotatedField(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => _u32(_at(p, 8)) != 0,
      2 => null,
      3 => _text(_at(p, 8)),
      4 => _list(_at(p, 8), 16, (p) => _text(p)),
      5 => _at(p, 8).cast<Double>().value,
      6 => readFailed(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native value'),
    };
  }

  static Object? readFailed(Pointer<Void> p) {
    final values = _extensions(_at(p, 8));
    values["failed"] = readFailure(_pointer(_at(p, 0)));
    return values;
  }

  static Object? readAnnotationMember(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => readAnnotationMemberAnswerId(_pointer(_at(p, 8))),
      2 => readAnnotationMemberFailureId(_pointer(_at(p, 8))),
      _ => throw StateError('Invalid native discriminator'),
    };
  }

  static Object? readAnnotationMemberFailureId(Pointer<Void> p) {
    final values = _extensions(_at(p, 104));
    values["failure"] = readFailure(_pointer(_at(p, 0)));
    values["failure_id"] = readFailureId(_pointer(_at(p, 8)));
    values["observations"] =
        _list(_at(p, 16), 8, (p) => readObservation(_pointer(p)));
    values["question"] = readReadableQuestion(_pointer(_at(p, 32)));
    values["question_sources"] =
        _list(_at(p, 40), 8, (p) => readQuestionSource(_pointer(p)));
    values["request"] = _text(_at(p, 56));
    if (_u32(_at(p, 72)) != 0)
      values["threshold"] = _u32(_at(p, 72)) == 1
          ? null
          : readThreshold(_pointer(_at(_at(p, 72), 8)));
    if (_u32(_at(p, 88)) != 0)
      values["usage"] = _u32(_at(p, 88)) == 1
          ? null
          : readUsage(_pointer(_at(_at(p, 88), 8)));
    return values;
  }

  static Object? readAnnotationMemberAnswerId(Pointer<Void> p) {
    final values = _extensions(_at(p, 120));
    values["answer"] = readAnswer(_pointer(_at(p, 0)));
    values["answer_id"] = readAnswerId(_pointer(_at(p, 8)));
    values["observations"] =
        _list(_at(p, 16), 8, (p) => readObservation(_pointer(p)));
    values["question"] = readReadableQuestion(_pointer(_at(p, 32)));
    values["question_sources"] =
        _list(_at(p, 40), 8, (p) => readQuestionSource(_pointer(p)));
    values["request"] = _text(_at(p, 56));
    if (_u32(_at(p, 72)) != 0)
      values["threshold"] = _u32(_at(p, 72)) == 1
          ? null
          : readThreshold(_pointer(_at(_at(p, 72), 8)));
    if (_u32(_at(p, 88)) != 0)
      values["usage"] = _u32(_at(p, 88)) == 1
          ? null
          : readUsage(_pointer(_at(_at(p, 88), 8)));
    if (_u32(_at(p, 104)) != 0)
      values["value"] = _u32(_at(p, 104)) == 1
          ? null
          : readValue(_pointer(_at(_at(p, 104), 8)));
    return values;
  }

  static Object? readSessionPacketFindAggregate(Pointer<Void> p) {
    final values = _extensions(_at(p, 40));
    values["function"] = _text(_at(p, 0));
    values["kind"] = _text(_at(p, 16));
    values["value"] = readFind(_pointer(_at(p, 32)));
    return values;
  }

  static Object? readFind(Pointer<Void> p) {
    final values = _extensions(_at(p, 184));
    values["answer"] = readFindAnswer(_pointer(_at(p, 0)));
    values["answer_id"] = readAnswerId(_pointer(_at(p, 8)));
    if (_u32(_at(p, 16)) != 0)
      values["candidates"] = _u32(_at(p, 16)) == 1
          ? null
          : _list(_at(_at(p, 16), 8), 8, (p) => readFindCandidate(_pointer(p)));
    if (_u32(_at(p, 40)) != 0)
      values["file"] = _u32(_at(p, 40)) == 1 ? null : _text(_at(_at(p, 40), 8));
    if (_u32(_at(p, 64)) != 0)
      values["first_line"] =
          _u32(_at(p, 64)) == 1 ? null : _unsigned(_at(_at(p, 64), 8));
    if (_u32(_at(p, 80)) != 0)
      values["index"] =
          _u32(_at(p, 80)) == 1 ? null : _unsigned(_at(_at(p, 80), 8));
    if (_u32(_at(p, 96)) != 0)
      values["last_line"] =
          _u32(_at(p, 96)) == 1 ? null : _unsigned(_at(_at(p, 96), 8));
    values["meta"] = readMeta(_pointer(_at(p, 112)));
    if (_u32(_at(p, 120)) != 0)
      values["position"] = _u32(_at(p, 120)) == 1
          ? null
          : readPosition(_pointer(_at(_at(p, 120), 8)));
    values["question"] = readFindQuestionInfo(_pointer(_at(p, 136)));
    values["schema"] = readVersion(_pointer(_at(p, 144)));
    if (_u32(_at(p, 152)) != 0)
      values["threshold"] =
          _u32(_at(p, 152)) == 1 ? null : _value(_pointer(_at(_at(p, 152), 8)));
    if (_u32(_at(p, 168)) != 0)
      values["value"] =
          _u32(_at(p, 168)) == 1 ? null : _value(_pointer(_at(_at(p, 168), 8)));
    return values;
  }

  static Object? readFindQuestionInfo(Pointer<Void> p) {
    final values = _extensions(_at(p, 216));
    if (_u32(_at(p, 0)) != 0)
      values["batch"] =
          _u32(_at(p, 0)) == 1 ? null : readBatch(_pointer(_at(_at(p, 0), 8)));
    if (_u32(_at(p, 16)) != 0)
      values["context_schema"] = _u32(_at(p, 16)) == 1
          ? null
          : readInputDeclaration(_pointer(_at(_at(p, 16), 8)));
    if (_u32(_at(p, 32)) != 0)
      values["item_schema"] = _u32(_at(p, 32)) == 1
          ? null
          : readInputDeclaration(_pointer(_at(_at(p, 32), 8)));
    if (_u32(_at(p, 48)) != 0)
      values["label_details"] = _u32(_at(p, 48)) == 1
          ? null
          : _list(_at(_at(p, 48), 8), 8, (p) => readLabel(_pointer(p)));
    if (_u32(_at(p, 72)) != 0)
      values["model"] =
          _u32(_at(p, 72)) == 1 ? null : _text(_at(_at(p, 72), 8));
    if (_u32(_at(p, 96)) != 0)
      values["name"] = _u32(_at(p, 96)) == 1
          ? null
          : readQuestionName(_pointer(_at(_at(p, 96), 8)));
    values["none"] = _u32(_at(p, 112)) != 0;
    if (_u32(_at(p, 120)) != 0)
      values["on"] = _u32(_at(p, 120)) == 1
          ? null
          : _list(_at(_at(p, 120), 8), 16, (p) => _text(p));
    if (_u32(_at(p, 144)) != 0)
      values["profile"] =
          _u32(_at(p, 144)) == 1 ? null : _text(_at(_at(p, 144), 8));
    if (_u32(_at(p, 168)) != 0)
      values["text"] =
          _u32(_at(p, 168)) == 1 ? null : _value(_pointer(_at(_at(p, 168), 8)));
    values["verb"] = _text(_at(p, 184));
    if (_u32(_at(p, 200)) != 0)
      values["wording_version"] = _u32(_at(p, 200)) == 1
          ? null
          : readWordingVersion(_pointer(_at(_at(p, 200), 8)));
    return values;
  }

  static Object? readFindCandidate(Pointer<Void> p) {
    final values = _extensions(_at(p, 56));
    if (_u32(_at(p, 0)) != 0)
      values["index"] =
          _u32(_at(p, 0)) == 1 ? null : _unsigned(_at(_at(p, 0), 8));
    if (_u32(_at(p, 16)) != 0)
      values["input"] =
          _u32(_at(p, 16)) == 1 ? null : _value(_pointer(_at(_at(p, 16), 8)));
    values["probability"] = _at(p, 32).cast<Double>().value;
    if (_u32(_at(p, 40)) != 0)
      values["source"] = _u32(_at(p, 40)) == 1
          ? null
          : readPhysicalSource(_pointer(_at(_at(p, 40), 8)));
    return values;
  }

  static Object? readFindAnswer(Pointer<Void> p) {
    final values = _extensions(_at(p, 64));
    if (_u32(_at(p, 0)) != 0)
      values["confidence"] =
          _u32(_at(p, 0)) == 1 ? null : _at(_at(p, 0), 8).cast<Double>().value;
    values["kind"] = _text(_at(p, 16));
    values["pick"] = _text(_at(p, 32));
    values["probabilities"] =
        _map(_at(p, 48), 24, 16, (p) => p.cast<Double>().value);
    return values;
  }

  static Object? readSessionPacketRankAggregate(Pointer<Void> p) {
    final values = _extensions(_at(p, 48));
    values["function"] = _text(_at(p, 0));
    values["kind"] = _text(_at(p, 16));
    values["value"] = _list(_at(p, 32), 8, (p) => readRankResult(_pointer(p)));
    return values;
  }

  static Object? readRankResult(Pointer<Void> p) {
    final values = _extensions(_at(p, 184));
    values["answer"] = readAnswer(_pointer(_at(p, 0)));
    values["answer_id"] = readAnswerId(_pointer(_at(p, 8)));
    if (_u32(_at(p, 16)) != 0)
      values["images"] = _u32(_at(p, 16)) == 1
          ? null
          : _list(_at(_at(p, 16), 8), 8, (p) => readImage(_pointer(p)));
    if (_u32(_at(p, 40)) != 0)
      values["index"] =
          _u32(_at(p, 40)) == 1 ? null : _unsigned(_at(_at(p, 40), 8));
    if (_u32(_at(p, 56)) != 0)
      values["input"] =
          _u32(_at(p, 56)) == 1 ? null : _value(_pointer(_at(_at(p, 56), 8)));
    if (_u32(_at(p, 72)) != 0)
      values["members"] = _u32(_at(p, 72)) == 1
          ? null
          : _list(_at(_at(p, 72), 8), 8, (p) => readRankMember(_pointer(p)));
    values["meta"] = readMeta(_pointer(_at(p, 96)));
    values["question"] = readReadableQuestion(_pointer(_at(p, 104)));
    if (_u32(_at(p, 112)) != 0)
      values["question_name"] =
          _u32(_at(p, 112)) == 1 ? null : _text(_at(_at(p, 112), 8));
    values["schema"] = readVersion(_pointer(_at(p, 136)));
    if (_u32(_at(p, 144)) != 0)
      values["source"] = _u32(_at(p, 144)) == 1
          ? null
          : readPhysicalSource(_pointer(_at(_at(p, 144), 8)));
    if (_u32(_at(p, 160)) != 0)
      values["threshold"] = _u32(_at(p, 160)) == 1
          ? null
          : readThreshold(_pointer(_at(_at(p, 160), 8)));
    values["value"] = _unsigned(_at(p, 176));
    return values;
  }

  static Object? readRankMember(Pointer<Void> p) {
    final values = _extensions(_at(p, 24));
    values["name"] = _text(_at(p, 0));
    values["result"] = readRankMemberResult(_pointer(_at(p, 16)));
    return values;
  }

  static Object? readRankMemberResult(Pointer<Void> p) {
    final values = _extensions(_at(p, 104));
    values["answer"] = readAnswer(_pointer(_at(p, 0)));
    values["answer_id"] = readAnswerId(_pointer(_at(p, 8)));
    if (_u32(_at(p, 16)) != 0)
      values["images"] = _u32(_at(p, 16)) == 1
          ? null
          : _list(_at(_at(p, 16), 8), 8, (p) => readImage(_pointer(p)));
    values["meta"] = readMeta(_pointer(_at(p, 40)));
    values["question"] = readReadableQuestion(_pointer(_at(p, 48)));
    values["schema"] = readVersion(_pointer(_at(p, 56)));
    if (_u32(_at(p, 64)) != 0)
      values["source"] = _u32(_at(p, 64)) == 1
          ? null
          : readPhysicalSource(_pointer(_at(_at(p, 64), 8)));
    if (_u32(_at(p, 80)) != 0)
      values["threshold"] =
          _u32(_at(p, 80)) == 1 ? null : _value(_pointer(_at(_at(p, 80), 8)));
    values["value"] = _unsigned(_at(p, 96));
    return values;
  }

  static Object? readImage(Pointer<Void> p) {
    final values = _extensions(_at(p, 40));
    values["base64"] = _text(_at(p, 0));
    values["height"] = _unsigned(_at(p, 16));
    values["media"] = readImageMedia(_pointer(_at(p, 24)));
    values["width"] = _unsigned(_at(p, 32));
    return values;
  }

  static Object? readImageMedia(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => "image/jpeg",
      2 => "image/png",
      _ => throw StateError('Invalid native enum'),
    };
  }

  static Object? readSessionPacketFilterAggregate(Pointer<Void> p) {
    final values = _extensions(_at(p, 48));
    values["function"] = _text(_at(p, 0));
    values["kind"] = _text(_at(p, 16));
    values["value"] =
        _list(_at(p, 32), 8, (p) => readFilterResult(_pointer(p)));
    return values;
  }

  static Object? readFilterResult(Pointer<Void> p) {
    final values = _extensions(_at(p, 184));
    values["answer"] = readAnswer(_pointer(_at(p, 0)));
    values["answer_id"] = readAnswerId(_pointer(_at(p, 8)));
    if (_u32(_at(p, 16)) != 0)
      values["images"] = _u32(_at(p, 16)) == 1
          ? null
          : _list(_at(_at(p, 16), 8), 8, (p) => readImage(_pointer(p)));
    if (_u32(_at(p, 40)) != 0)
      values["index"] =
          _u32(_at(p, 40)) == 1 ? null : _unsigned(_at(_at(p, 40), 8));
    if (_u32(_at(p, 56)) != 0)
      values["input"] =
          _u32(_at(p, 56)) == 1 ? null : _value(_pointer(_at(_at(p, 56), 8)));
    if (_u32(_at(p, 72)) != 0)
      values["members"] = _u32(_at(p, 72)) == 1
          ? null
          : _list(_at(_at(p, 72), 8), 8, (p) => readRankMember(_pointer(p)));
    values["meta"] = readMeta(_pointer(_at(p, 96)));
    values["question"] = readReadableQuestion(_pointer(_at(p, 104)));
    if (_u32(_at(p, 112)) != 0)
      values["question_name"] =
          _u32(_at(p, 112)) == 1 ? null : _text(_at(_at(p, 112), 8));
    values["schema"] = readVersion(_pointer(_at(p, 136)));
    if (_u32(_at(p, 144)) != 0)
      values["source"] = _u32(_at(p, 144)) == 1
          ? null
          : readPhysicalSource(_pointer(_at(_at(p, 144), 8)));
    if (_u32(_at(p, 160)) != 0)
      values["threshold"] = _u32(_at(p, 160)) == 1
          ? null
          : readThreshold(_pointer(_at(_at(p, 160), 8)));
    values["value"] = _u32(_at(p, 176)) != 0;
    return values;
  }

  static Object? readSessionPacketScoreAggregate(Pointer<Void> p) {
    final values = _extensions(_at(p, 48));
    values["function"] = _text(_at(p, 0));
    values["kind"] = _text(_at(p, 16));
    values["value"] = _list(_at(p, 32), 8, (p) => readScoreResult(_pointer(p)));
    return values;
  }

  static Object? readScoreResult(Pointer<Void> p) {
    final values = _extensions(_at(p, 184));
    values["answer"] = readAnswer(_pointer(_at(p, 0)));
    values["answer_id"] = readAnswerId(_pointer(_at(p, 8)));
    if (_u32(_at(p, 16)) != 0)
      values["images"] = _u32(_at(p, 16)) == 1
          ? null
          : _list(_at(_at(p, 16), 8), 8, (p) => readImage(_pointer(p)));
    if (_u32(_at(p, 40)) != 0)
      values["index"] =
          _u32(_at(p, 40)) == 1 ? null : _unsigned(_at(_at(p, 40), 8));
    if (_u32(_at(p, 56)) != 0)
      values["input"] =
          _u32(_at(p, 56)) == 1 ? null : _value(_pointer(_at(_at(p, 56), 8)));
    if (_u32(_at(p, 72)) != 0)
      values["members"] = _u32(_at(p, 72)) == 1
          ? null
          : _list(_at(_at(p, 72), 8), 8, (p) => readRankMember(_pointer(p)));
    values["meta"] = readMeta(_pointer(_at(p, 96)));
    values["question"] = readReadableQuestion(_pointer(_at(p, 104)));
    if (_u32(_at(p, 112)) != 0)
      values["question_name"] =
          _u32(_at(p, 112)) == 1 ? null : _text(_at(_at(p, 112), 8));
    values["schema"] = readVersion(_pointer(_at(p, 136)));
    if (_u32(_at(p, 144)) != 0)
      values["source"] = _u32(_at(p, 144)) == 1
          ? null
          : readPhysicalSource(_pointer(_at(_at(p, 144), 8)));
    if (_u32(_at(p, 160)) != 0)
      values["threshold"] = _u32(_at(p, 160)) == 1
          ? null
          : readThreshold(_pointer(_at(_at(p, 160), 8)));
    values["value"] = _at(p, 176).cast<Double>().value;
    return values;
  }

  static Object? readSessionPacketTagAggregate(Pointer<Void> p) {
    final values = _extensions(_at(p, 48));
    values["function"] = _text(_at(p, 0));
    values["kind"] = _text(_at(p, 16));
    values["value"] = _list(_at(p, 32), 8, (p) => readTagResult(_pointer(p)));
    return values;
  }

  static Object? readTagResult(Pointer<Void> p) {
    final values = _extensions(_at(p, 192));
    values["answer"] = readAnswer(_pointer(_at(p, 0)));
    values["answer_id"] = readAnswerId(_pointer(_at(p, 8)));
    if (_u32(_at(p, 16)) != 0)
      values["images"] = _u32(_at(p, 16)) == 1
          ? null
          : _list(_at(_at(p, 16), 8), 8, (p) => readImage(_pointer(p)));
    if (_u32(_at(p, 40)) != 0)
      values["index"] =
          _u32(_at(p, 40)) == 1 ? null : _unsigned(_at(_at(p, 40), 8));
    if (_u32(_at(p, 56)) != 0)
      values["input"] =
          _u32(_at(p, 56)) == 1 ? null : _value(_pointer(_at(_at(p, 56), 8)));
    if (_u32(_at(p, 72)) != 0)
      values["members"] = _u32(_at(p, 72)) == 1
          ? null
          : _list(_at(_at(p, 72), 8), 8, (p) => readRankMember(_pointer(p)));
    values["meta"] = readMeta(_pointer(_at(p, 96)));
    values["question"] = readReadableQuestion(_pointer(_at(p, 104)));
    if (_u32(_at(p, 112)) != 0)
      values["question_name"] =
          _u32(_at(p, 112)) == 1 ? null : _text(_at(_at(p, 112), 8));
    values["schema"] = readVersion(_pointer(_at(p, 136)));
    if (_u32(_at(p, 144)) != 0)
      values["source"] = _u32(_at(p, 144)) == 1
          ? null
          : readPhysicalSource(_pointer(_at(_at(p, 144), 8)));
    if (_u32(_at(p, 160)) != 0)
      values["threshold"] = _u32(_at(p, 160)) == 1
          ? null
          : readThreshold(_pointer(_at(_at(p, 160), 8)));
    values["value"] = _list(_at(p, 176), 16, (p) => _text(p));
    return values;
  }

  static Object? readSessionPacketChooseAggregate(Pointer<Void> p) {
    final values = _extensions(_at(p, 48));
    values["function"] = _text(_at(p, 0));
    values["kind"] = _text(_at(p, 16));
    values["value"] =
        _list(_at(p, 32), 8, (p) => readChoiceResult(_pointer(p)));
    return values;
  }

  static Object? readChoiceResult(Pointer<Void> p) {
    final values = _extensions(_at(p, 200));
    values["answer"] = readAnswer(_pointer(_at(p, 0)));
    values["answer_id"] = readAnswerId(_pointer(_at(p, 8)));
    if (_u32(_at(p, 16)) != 0)
      values["images"] = _u32(_at(p, 16)) == 1
          ? null
          : _list(_at(_at(p, 16), 8), 8, (p) => readImage(_pointer(p)));
    if (_u32(_at(p, 40)) != 0)
      values["index"] =
          _u32(_at(p, 40)) == 1 ? null : _unsigned(_at(_at(p, 40), 8));
    if (_u32(_at(p, 56)) != 0)
      values["input"] =
          _u32(_at(p, 56)) == 1 ? null : _value(_pointer(_at(_at(p, 56), 8)));
    if (_u32(_at(p, 72)) != 0)
      values["members"] = _u32(_at(p, 72)) == 1
          ? null
          : _list(_at(_at(p, 72), 8), 8, (p) => readRankMember(_pointer(p)));
    values["meta"] = readMeta(_pointer(_at(p, 96)));
    values["question"] = readReadableQuestion(_pointer(_at(p, 104)));
    if (_u32(_at(p, 112)) != 0)
      values["question_name"] =
          _u32(_at(p, 112)) == 1 ? null : _text(_at(_at(p, 112), 8));
    values["schema"] = readVersion(_pointer(_at(p, 136)));
    if (_u32(_at(p, 144)) != 0)
      values["source"] = _u32(_at(p, 144)) == 1
          ? null
          : readPhysicalSource(_pointer(_at(_at(p, 144), 8)));
    if (_u32(_at(p, 160)) != 0)
      values["threshold"] = _u32(_at(p, 160)) == 1
          ? null
          : readThreshold(_pointer(_at(_at(p, 160), 8)));
    if (_u32(_at(p, 176)) != 0)
      values["value"] =
          _u32(_at(p, 176)) == 1 ? null : _text(_at(_at(p, 176), 8));
    return values;
  }

  static Object? readSessionPacketDecideAggregate(Pointer<Void> p) {
    final values = _extensions(_at(p, 48));
    values["function"] = _text(_at(p, 0));
    values["kind"] = _text(_at(p, 16));
    values["value"] =
        _list(_at(p, 32), 8, (p) => readDecisionResult(_pointer(p)));
    return values;
  }

  static Object? readDecisionResult(Pointer<Void> p) {
    final values = _extensions(_at(p, 192));
    values["answer"] = readAnswer(_pointer(_at(p, 0)));
    values["answer_id"] = readAnswerId(_pointer(_at(p, 8)));
    if (_u32(_at(p, 16)) != 0)
      values["images"] = _u32(_at(p, 16)) == 1
          ? null
          : _list(_at(_at(p, 16), 8), 8, (p) => readImage(_pointer(p)));
    if (_u32(_at(p, 40)) != 0)
      values["index"] =
          _u32(_at(p, 40)) == 1 ? null : _unsigned(_at(_at(p, 40), 8));
    if (_u32(_at(p, 56)) != 0)
      values["input"] =
          _u32(_at(p, 56)) == 1 ? null : _value(_pointer(_at(_at(p, 56), 8)));
    if (_u32(_at(p, 72)) != 0)
      values["members"] = _u32(_at(p, 72)) == 1
          ? null
          : _list(_at(_at(p, 72), 8), 8, (p) => readRankMember(_pointer(p)));
    values["meta"] = readMeta(_pointer(_at(p, 96)));
    values["question"] = readReadableQuestion(_pointer(_at(p, 104)));
    if (_u32(_at(p, 112)) != 0)
      values["question_name"] =
          _u32(_at(p, 112)) == 1 ? null : _text(_at(_at(p, 112), 8));
    values["schema"] = readVersion(_pointer(_at(p, 136)));
    if (_u32(_at(p, 144)) != 0)
      values["source"] = _u32(_at(p, 144)) == 1
          ? null
          : readPhysicalSource(_pointer(_at(_at(p, 144), 8)));
    if (_u32(_at(p, 160)) != 0)
      values["threshold"] = _u32(_at(p, 160)) == 1
          ? null
          : readThreshold(_pointer(_at(_at(p, 160), 8)));
    if (_u32(_at(p, 176)) != 0)
      values["value"] = _u32(_at(p, 176)) == 1
          ? null
          : readDecideValue(_pointer(_at(_at(p, 176), 8)));
    return values;
  }

  static Object? readDecideValue(Pointer<Void> p) {
    return _value(_pointer(_at(p, 0)));
  }

  static Object? readSessionPacketAnnotateRow(Pointer<Void> p) {
    final values = _extensions(_at(p, 40));
    values["function"] = _text(_at(p, 0));
    values["kind"] = _text(_at(p, 16));
    values["value"] = readAnnotation(_pointer(_at(p, 32)));
    return values;
  }

  static Object? readSessionPacketFilterRow(Pointer<Void> p) {
    final values = _extensions(_at(p, 40));
    values["function"] = _text(_at(p, 0));
    values["kind"] = _text(_at(p, 16));
    values["value"] = readFilterResult(_pointer(_at(p, 32)));
    return values;
  }

  static Object? readSessionPacketScoreRow(Pointer<Void> p) {
    final values = _extensions(_at(p, 40));
    values["function"] = _text(_at(p, 0));
    values["kind"] = _text(_at(p, 16));
    values["value"] = readScoreResult(_pointer(_at(p, 32)));
    return values;
  }

  static Object? readSessionPacketTagRow(Pointer<Void> p) {
    final values = _extensions(_at(p, 40));
    values["function"] = _text(_at(p, 0));
    values["kind"] = _text(_at(p, 16));
    values["value"] = readTagResult(_pointer(_at(p, 32)));
    return values;
  }

  static Object? readSessionPacketChooseRow(Pointer<Void> p) {
    final values = _extensions(_at(p, 40));
    values["function"] = _text(_at(p, 0));
    values["kind"] = _text(_at(p, 16));
    values["value"] = readChoiceResult(_pointer(_at(p, 32)));
    return values;
  }

  static Object? readSessionPacketDecideRow(Pointer<Void> p) {
    final values = _extensions(_at(p, 40));
    values["function"] = _text(_at(p, 0));
    values["kind"] = _text(_at(p, 16));
    values["value"] = readDecisionResult(_pointer(_at(p, 32)));
    return values;
  }

  static Object? readFailureFieldKind(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => "backend",
      _ => throw StateError('Invalid native enum'),
    };
  }

  static Object? readBatchString(Pointer<Void> p) {
    return switch (_u32(_at(p, 0))) {
      1 => "max",
      _ => throw StateError('Invalid native enum'),
    };
  }

  static Map<String, Object?> _extensions(Pointer<Void> p) =>
      _map(p, 40, 32, (v) => _value(_pointer(v)));
  static Object? _value(Pointer<Void> p) => switch (_u32(p)) {
        1 => null,
        2 => _u32(_at(p, 8)) != 0,
        3 => _number(_text(_at(p, 8))),
        4 => _text(_at(p, 8)),
        5 => _list(_at(p, 8), 8, (v) => _value(_pointer(v))),
        6 => _map(_at(p, 8), 24, 16, (v) => _value(_pointer(v))),
        _ => throw StateError('Invalid native value'),
      };
  static CallFailure failure(Pointer<Void> p, int fallback) {
    if (p == nullptr)
      return CallFailure._native(
          fallback, false, 'Native operation failed', null);
    final facts = _pointer(_at(p, 24));
    return CallFailure._native(
        _at(p, 0).cast<Int32>().value,
        _at(p, 4).cast<Int32>().value != 0,
        _text(_at(p, 8)),
        facts == nullptr ? null : FailureFacts.read(readFailureFacts(facts)));
  }

  static Pointer<Void> _at(Pointer<Void> p, int offset) =>
      (p.cast<Uint8>() + offset).cast<Void>();
  static Pointer<Void> _pointer(Pointer<Void> p) =>
      p.cast<Pointer<Void>>().value;
  static int _u32(Pointer<Void> p) => p.cast<Uint32>().value;
  static BigInt _unsigned(Pointer<Void> p) {
    final value = p.cast<Uint64>().value;
    return value < 0
        ? BigInt.from(value) + (BigInt.one << 64)
        : BigInt.from(value);
  }

  static String _text(Pointer<Void> p) {
    final data = p.cast<Pointer<Uint8>>().value;
    final length = _at(p, sizeOf<Pointer<Void>>()).cast<Size>().value;
    return length == 0 ? '' : utf8.decode(data.asTypedList(length));
  }

  static Object _number(String source) =>
      BigInt.tryParse(source) ?? double.parse(source);
  static List<Object?> _list(
      Pointer<Void> p, int stride, Object? Function(Pointer<Void>) read) {
    final data = _pointer(p);
    final length = _at(p, sizeOf<Pointer<Void>>()).cast<Size>().value;
    return List.generate(length, (i) => read(_at(data, i * stride)),
        growable: false);
  }

  static Map<String, Object?> _map(Pointer<Void> p, int stride, int valueOffset,
      Object? Function(Pointer<Void>) read) {
    final data = _pointer(p);
    final length = _at(p, sizeOf<Pointer<Void>>()).cast<Size>().value;
    return {
      for (var i = 0; i < length; i++)
        _text(_at(data, i * stride)): read(_at(data, i * stride + valueOffset))
    };
  }
}
