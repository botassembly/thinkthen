import 'input.dart';
import 'result.dart';
import 'views.dart';
import 'engine.dart' show Cancellation, Batch;

/// Shared typed signature contract for separately executed Dart and Flutter doors.
abstract interface class CompleteApi {
  void close();
  Cancellation cancellation();
  CompleteResult<DecideView> decide(Question question, Source input,
      {Controls controls = const Controls()});
  CompleteResult<ChooseView> choose(Question question, Source input,
      {Controls controls = const Controls()});
  CompleteResult<TagView> tag(Question question, Source input,
      {Controls controls = const Controls()});
  CompleteResult<ScoreView> score(Question question, Source input,
      {Controls controls = const Controls()});
  CompleteResult<FilterView> filter(Question question, Source input,
      {Controls controls = const Controls()});
  CompleteResult<RankView> rank(Question question, Source input,
      {Controls controls = const Controls()});
  CompleteResult<FindView> find(Question question, Source input,
      {Controls controls = const Controls()});
  CompleteResult<AnnotateView> annotate(Question question, Source input,
      {Controls controls = const Controls()});
  CompleteResult<RecognizeView> recognize(Question question, Source input,
      {Controls controls = const Controls()});
  CompleteResult<RelateView> relate(Question question, Source input,
      {Controls controls = const Controls()});
  Batch<DecideView> decideBatch(Question question, Source input,
      {Controls controls = const Controls()});
  Batch<ChooseView> chooseBatch(Question question, Source input,
      {Controls controls = const Controls()});
  Batch<TagView> tagBatch(Question question, Source input,
      {Controls controls = const Controls()});
  Batch<ScoreView> scoreBatch(Question question, Source input,
      {Controls controls = const Controls()});
  Batch<FilterView> filterBatch(Question question, Source input,
      {Controls controls = const Controls()});
  Batch<AnnotateView> annotateBatch(Question question, Source input,
      {Controls controls = const Controls()});
}
