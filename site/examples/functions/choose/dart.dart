import 'dart:io';

import 'package:thinkthen_dart/thinkthen_dart.dart';

void main() {
  final library = File('thinkthen-c/lib/libthinkthen.so');
  final tt = Door(library.absolute.path);
  final engine = tt.create();
  try {
    final owner = tt.ask(engine, {
      'choose': 'Which team owns this?',
      'options': {
        'billing': 'Invoices, fees, and refunds.',
        'shipping': 'Parcels and delivery.',
        'account': 'Logins and passwords.',
      },
      'records': [
        'Please refund the extra fee on my invoice.',
        'My parcel went to the wrong address.',
        'I cannot reset my password.',
      ],
    }) as Map;
    final teams = owner['value'] as List;
    assert(teams[0] == 'billing');
    assert(teams[1] == 'shipping');
    assert(teams[2] == 'account');
  } finally {
    tt.engineFree(engine);
  }
}
