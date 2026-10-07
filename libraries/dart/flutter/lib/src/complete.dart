// Private Flutter facade; keeps its surface separate until actual complete C execution.
import 'package:thinkthen_dart/src/complete/models.dart';
import 'package:thinkthen_dart/src/complete/values.dart';
import 'package:thinkthen_dart/src/complete/requests.dart' as dart;
export 'package:thinkthen_dart/src/complete/models.dart';
export 'package:thinkthen_dart/src/complete/values.dart';
export 'package:thinkthen_dart/src/complete/requests.dart' show Request;

export 'package:thinkthen_dart/src/complete/decision.dart';

abstract final class FlutterRequests {
  static dart.Request decide(Carrier question, Selection input,
          [Controls? controls]) =>
      dart.Requests.decide(question, input, controls);
  static dart.Request choose(Carrier question, Selection input,
          [Controls? controls]) =>
      dart.Requests.choose(question, input, controls);
  static dart.Request tag(Carrier question, Selection input,
          [Controls? controls]) =>
      dart.Requests.tag(question, input, controls);
  static dart.Request score(Carrier question, Selection input,
          [Controls? controls]) =>
      dart.Requests.score(question, input, controls);
  static dart.Request filter(Carrier question, Selection input,
          [Controls? controls]) =>
      dart.Requests.filter(question, input, controls);
  static dart.Request rank(Carrier question, Selection input,
          [Controls? controls]) =>
      dart.Requests.rank(question, input, controls);
  static dart.Request find(Carrier question, Selection input,
          [Controls? controls]) =>
      dart.Requests.find(question, input, controls);
  static dart.Request annotate(Carrier question, Selection input,
          [Controls? controls]) =>
      dart.Requests.annotate(question, input, controls);
  static dart.Request recognize(Carrier question, Selection input,
          [Controls? controls]) =>
      dart.Requests.recognize(question, input, controls);
  static dart.Request relate(Carrier question, Selection input,
          [Controls? controls]) =>
      dart.Requests.relate(question, input, controls);
}
