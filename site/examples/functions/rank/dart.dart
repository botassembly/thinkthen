import 'dart:io';

import 'package:thinkthen_dart/thinkthen_dart.dart';

void main() {
  final library = File('thinkthen-c/lib/libthinkthen.so');
  final tt = Door(library.absolute.path);
  final engine = tt.create();
  try {
    final mostUrgent = tt.ask(engine, {
      'rank': 'Is this urgent?',
      'records': [
        'Newsletter: our autumn catalog is here. '
            'No reply needed.',
        'Our checkout page is down and customers '
            'cannot pay',
        'Reminder: your invoice is due in 30 days',
        'Please send the signed quote by 5 pm today',
      ],
    }) as Map;
    final order = [
      for (final one in mostUrgent['value']) one['index'],
    ];
    assert(order.join(',') == '1,3,2,0');
  } finally {
    tt.engineFree(engine);
  }
}
