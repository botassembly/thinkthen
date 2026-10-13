/// Typed development caller with shared implementation and fixed surface.
library thinkthen_session;

import 'dart:async';
import 'dart:convert';
import 'dart:ffi';
import 'dart:isolate';
import 'dart:io' show IOException;
import 'dart:typed_data';
import 'package:ffi/ffi.dart';
part 'src/session/abi_generated.dart';
part 'src/session/inputs_generated.dart';
part 'src/session/results_generated.dart';
part 'src/session/native.dart';
part 'src/session/values.dart';
part 'src/session/client.dart';

part 'src/session/calls_generated.dart';

const _surfaceToken = 'dart';
