import 'dart:io';

import 'package:thinkthen_dart/thinkthen_dart.dart';

void main() {
  final library = File('thinkthen-c/lib/libthinkthen.so');
  final tt = Door(library.absolute.path);
  final engine = tt.create();
  try {
    final lines = [
      'Returns need the original receipt.',
      'Refunds are issued within 30 days of purchase.',
      r'Shipping is free on orders over $50.',
      'Gift cards cannot be exchanged for cash.',
    ];
    final deadline = tt.ask(engine, {
      'find': 'Which line gives the refund deadline?',
      'units': lines,
    }) as Map;
    final line = deadline['value']['unit'];
    assert(line == lines[1]);
  } finally {
    tt.engineFree(engine);
  }
}
