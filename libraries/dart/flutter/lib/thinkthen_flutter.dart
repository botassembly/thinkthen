/// Typed development caller with shared implementation and fixed surface.
library thinkthen_session;

import 'dart:async';
import 'dart:convert';
import 'dart:ffi';
import 'dart:isolate';
import 'dart:io' show IOException;
import 'dart:typed_data';
import 'package:ffi/ffi.dart';
part 'package:thinkthen_dart/src/session/abi_generated.dart';
part 'package:thinkthen_dart/src/session/inputs_generated.dart';
part 'package:thinkthen_dart/src/session/results_generated.dart';
part 'package:thinkthen_dart/src/session/native.dart';
part 'package:thinkthen_dart/src/session/values.dart';
part 'package:thinkthen_dart/src/session/client.dart';

part 'package:thinkthen_dart/src/session/calls_generated.dart';

const _surfaceToken = 'flutter';
