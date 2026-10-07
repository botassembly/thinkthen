import 'package:flutter_test/flutter_test.dart';
import 'package:thinkthen_flutter/src/complete.dart';
import '../../../checks/consumers/alpha/bin/complete_carriers.dart'
    show Builder, runCarrierCases;

void main() {
  test(
      'Flutter private facade preserves complete carriers and ten named builders',
      () {
    final builders = <String, Builder>{
      'decide': FlutterRequests.decide,
      'choose': FlutterRequests.choose,
      'tag': FlutterRequests.tag,
      'score': FlutterRequests.score,
      'filter': FlutterRequests.filter,
      'rank': FlutterRequests.rank,
      'find': FlutterRequests.find,
      'annotate': FlutterRequests.annotate,
      'recognize': FlutterRequests.recognize,
      'relate': FlutterRequests.relate
    };
    runCarrierCases('../../../php/fixtures/complete.json', builders);
  });
}
