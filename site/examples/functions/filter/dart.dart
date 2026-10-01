import 'dart:io';

import 'package:thinkthen_dart/thinkthen_dart.dart';

void main() {
  final library = File('thinkthen-c/lib/libthinkthen.so');
  final tt = Door(library.absolute.path);
  final engine = tt.create();
  try {
    final reviews = [
      'Arrived a day early. Thank you!',
      'The zipper broke the first time I used it.',
      'Does this come in blue?',
      'The strap snapped on day two.',
    ];
    final isComplaint = tt.ask(engine, {
      'filter': 'Is this a complaint?',
      'records': reviews,
    }) as Map;
    final complaints = isComplaint['value'] as List;
    assert(complaints.length == 2);
    assert(complaints[0] == reviews[1]);
    assert(complaints[1] == reviews[3]);
  } finally {
    tt.engineFree(engine);
  }
}
