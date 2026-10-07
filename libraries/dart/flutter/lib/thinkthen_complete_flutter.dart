/// Complete typed Flutter facade over one owned native engine.
library;

import 'package:thinkthen_dart/thinkthen_complete.dart';
export 'package:thinkthen_dart/thinkthen_complete.dart' hide Engine;

final class ThinkThenCompleteFlutter implements CompleteApi {
  final Engine _engine;
  ThinkThenCompleteFlutter(String library, {String? settingsJson})
      : _engine = Engine.flutter(library, settingsJson: settingsJson);
  void close() => _engine.close();
  Cancellation cancellation() => _engine.cancellation();
  CompleteResult<DecideView> decide(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.decide(question, input, controls: controls);
  CompleteResult<ChooseView> choose(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.choose(question, input, controls: controls);
  CompleteResult<TagView> tag(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.tag(question, input, controls: controls);
  CompleteResult<ScoreView> score(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.score(question, input, controls: controls);
  CompleteResult<FilterView> filter(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.filter(question, input, controls: controls);
  CompleteResult<RankView> rank(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.rank(question, input, controls: controls);
  CompleteResult<FindView> find(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.find(question, input, controls: controls);
  CompleteResult<AnnotateView> annotate(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.annotate(question, input, controls: controls);
  CompleteResult<RecognizeView> recognize(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.recognize(question, input, controls: controls);
  CompleteResult<RelateView> relate(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.relate(question, input, controls: controls);
  Batch<DecideView> decideBatch(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.decideBatch(question, input, controls: controls);
  Batch<ChooseView> chooseBatch(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.chooseBatch(question, input, controls: controls);
  Batch<TagView> tagBatch(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.tagBatch(question, input, controls: controls);
  Batch<ScoreView> scoreBatch(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.scoreBatch(question, input, controls: controls);
  Batch<FilterView> filterBatch(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.filterBatch(question, input, controls: controls);
  Batch<AnnotateView> annotateBatch(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.annotateBatch(question, input, controls: controls);
}
