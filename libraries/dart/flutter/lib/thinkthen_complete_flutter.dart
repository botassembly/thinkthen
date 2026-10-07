/// Complete typed Flutter facade over one owned native engine.
library;

import 'package:thinkthen_dart/thinkthen_complete.dart';
export 'package:thinkthen_dart/thinkthen_complete.dart' hide Engine;

final class ThinkThenCompleteFlutter {
  final Engine _engine;
  ThinkThenCompleteFlutter(String library, {String? settingsJson})
      : _engine = Engine.flutter(library, settingsJson: settingsJson);
  void close() => _engine.close();
  Cancellation cancellation() => _engine.cancellation();
  CompleteResult<DecideViewView> decide(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.decide(question, input, controls: controls);
  CompleteResult<ChooseViewView> choose(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.choose(question, input, controls: controls);
  CompleteResult<TagViewView> tag(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.tag(question, input, controls: controls);
  CompleteResult<ScoreViewView> score(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.score(question, input, controls: controls);
  CompleteResult<FilterViewView> filter(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.filter(question, input, controls: controls);
  CompleteResult<RankViewView> rank(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.rank(question, input, controls: controls);
  CompleteResult<FindViewView> find(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.find(question, input, controls: controls);
  CompleteResult<AnnotateViewView> annotate(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.annotate(question, input, controls: controls);
  CompleteResult<RecognizeViewView> recognize(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.recognize(question, input, controls: controls);
  CompleteResult<RelateViewView> relate(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.relate(question, input, controls: controls);
  Batch<DecideViewView> decideBatch(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.decideBatch(question, input, controls: controls);
  Batch<ChooseViewView> chooseBatch(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.chooseBatch(question, input, controls: controls);
  Batch<TagViewView> tagBatch(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.tagBatch(question, input, controls: controls);
  Batch<ScoreViewView> scoreBatch(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.scoreBatch(question, input, controls: controls);
  Batch<FilterViewView> filterBatch(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.filterBatch(question, input, controls: controls);
  Batch<AnnotateViewView> annotateBatch(Question question, Source input,
          {Controls controls = const Controls()}) =>
      _engine.annotateBatch(question, input, controls: controls);
}
