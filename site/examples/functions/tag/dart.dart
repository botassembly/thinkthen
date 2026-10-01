import 'dart:convert';
import 'dart:io';

import 'package:thinkthen_dart/thinkthen_dart.dart';

void main() {
  final library = File('thinkthen-c/lib/libthinkthen.so');
  final tt = Door(library.absolute.path);
  final engine = tt.create();
  try {
    final fitting = tt.ask(engine, {
      'tag': 'Which labels fit this message?',
      'labels': ['praise', 'bug', 'billing'],
      'evidence': 'Love the new dashboard, but export '
          'crashes the app,\nand I was charged twice.\n',
    }) as Map;
    final labels = jsonEncode(fitting['value']);
    assert(labels == '["praise","bug","billing"]');
  } finally {
    tt.engineFree(engine);
  }
}
