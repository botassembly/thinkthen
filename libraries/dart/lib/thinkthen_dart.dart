/// Typed development caller with shared implementation and fixed surface.
library thinkthen_session;

import 'dart:async';
import 'dart:convert';
import 'dart:ffi';
import 'dart:io' show IOException;
import 'dart:typed_data';
import 'src/session/abi_generated.dart';
import 'src/session/inputs_generated.dart';
import 'src/session/results_generated.dart';
import 'src/session/native.dart';
import 'src/session/values.dart';
export 'src/session/native.dart' show NativeFailure;
export 'src/session/values.dart' show Presence;
export 'src/session/inputs_generated.dart';
export 'src/session/results_generated.dart';
export 'src/session/abi_generated.dart'
    show NativeErrorKind, UsagePersistenceState;
part 'src/session/client.dart';

const _surfaceToken = 'dart';
