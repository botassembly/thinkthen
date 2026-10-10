/// Typed development caller with shared implementation and fixed surface.
library thinkthen_session;

import 'dart:async';
import 'dart:convert';
import 'dart:ffi';
import 'dart:typed_data';
import 'package:thinkthen_dart/src/session/abi_generated.dart';
import 'package:thinkthen_dart/src/session/inputs_generated.dart';
import 'package:thinkthen_dart/src/session/results_generated.dart';
import 'package:thinkthen_dart/src/session/native.dart';
import 'package:thinkthen_dart/src/session/values.dart';
export 'package:thinkthen_dart/src/session/native.dart' show NativeFailure;
export 'package:thinkthen_dart/src/session/values.dart' show Presence;
export 'package:thinkthen_dart/src/session/inputs_generated.dart';
export 'package:thinkthen_dart/src/session/results_generated.dart';
export 'package:thinkthen_dart/src/session/abi_generated.dart'
    show NativeErrorKind, UsagePersistenceState;
part 'package:thinkthen_dart/src/session/client.dart';

const _surfaceToken = 'flutter';
