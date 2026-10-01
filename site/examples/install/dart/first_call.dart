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
    );
    assert(isRefund.value.outcome == Outcome.yes);

    const refund = '{"decide": "$question", '
        '"threshold": "0.2:0.8"}';
    final backIsRefund = tt.decide(
      engine,
      refund,
      'I want to send this back.',
    );
    final outcome = backIsRefund.value.outcome;
    assert(outcome == Outcome.notSure);
  } finally {
    tt.engineFree(engine);
  }
}
