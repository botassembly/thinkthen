import 'dart:io';

import 'package:thinkthen_dart/thinkthen_dart.dart';

void main() {
  final library = File('thinkthen-c/lib/libthinkthen.so');
  final tt = Door(library.absolute.path);
  final engine = tt.create();
  try {
    const question = 'Does the customer ask for a refund?';
    final isRefund = tt.decide(
      engine,
      question,
      'Please refund my order. It arrived broken.',
    ).value.outcome;
    assert(isRefund == Outcome.yes);
  } finally {
    tt.engineFree(engine);
  }
}
