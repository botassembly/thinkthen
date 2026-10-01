import 'dart:io';

import 'package:thinkthen_dart/thinkthen_dart.dart';

void main() {
  final library = File('thinkthen-c/lib/libthinkthen.so');
  final tt = Door(library.absolute.path);
  final engine = tt.create();
  try {
    const question = 'Does the customer ask for a refund?';
    final brokenIsRefund = tt.decide(
      engine,
      question,
      'Please refund my order. It arrived broken.',
    );
    final thanksIsRefund = tt.decide(
      engine,
      question,
      'Thanks for the quick help yesterday!',
    );
    final broken = brokenIsRefund.value.outcome;
    final thanks = thanksIsRefund.value.outcome;
    assert(broken == Outcome.yes);
    assert(thanks == Outcome.no);
  } finally {
    tt.engineFree(engine);
  }
}
