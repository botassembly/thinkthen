import 'dart:io';

import 'package:thinkthen_dart/thinkthen_dart.dart';

void main() {
  final library = File('thinkthen-c/lib/libthinkthen.so');
  final tt = Door(library.absolute.path);
  final engine = tt.create();
  try {
    const question = 'Which team owns this?';
    const teams = {
      'billing': 'Invoices, fees, and refunds.',
      'shipping': 'Parcels and delivery.',
      'account': 'Logins and passwords.',
    };
    const parcel = 'My parcel went to the wrong address.';
    final owner = tt.ask(engine, {
      'choose': question,
      'options': teams,
      'evidence': parcel,
    }) as Map;
    final team = owner['value'] as String;
    assert(team == 'shipping');
  } finally {
    tt.engineFree(engine);
  }
}
