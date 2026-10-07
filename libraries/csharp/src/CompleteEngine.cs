namespace ThinkThen;
public sealed record CompleteCall<T>(string Schema, Optional<Function> Function, Optional<AnswerId> AnswerId, Optional<Meta> Meta, Optional<CallFacts> Facts, Optional<IReadOnlyList<Attempt>> Attempts, IReadOnlyList<T> Rows, IReadOnlyList<ObservationEvent> Observations) {public Optional<CompleteError> Error {get;init;}};
/// <summary>Typed complete native execution.</summary>
public interface ICompleteEngine {
CompleteCall<DecideRow> DecideComplete(QuestionInput question, InputSource source, CallControls controls, TimeSpan? budget = null, CancellationToken cancellation = default);
CompleteCall<ChooseRow> ChooseComplete(QuestionInput question, InputSource source, CallControls controls, TimeSpan? budget = null, CancellationToken cancellation = default);
CompleteCall<TagRow> TagComplete(QuestionInput question, InputSource source, CallControls controls, TimeSpan? budget = null, CancellationToken cancellation = default);
CompleteCall<ScoreRow> ScoreComplete(QuestionInput question, InputSource source, CallControls controls, TimeSpan? budget = null, CancellationToken cancellation = default);
CompleteCall<FilterRow> FilterComplete(QuestionInput question, InputSource source, CallControls controls, TimeSpan? budget = null, CancellationToken cancellation = default);
CompleteCall<RankRow> RankComplete(QuestionInput question, InputSource source, CallControls controls, TimeSpan? budget = null, CancellationToken cancellation = default);
CompleteCall<FindRow> FindComplete(QuestionInput question, InputSource source, CallControls controls, TimeSpan? budget = null, CancellationToken cancellation = default);
CompleteCall<AnnotateRow> AnnotateComplete(QuestionInput question, InputSource source, CallControls controls, TimeSpan? budget = null, CancellationToken cancellation = default);
CompleteCall<RecognizeRow> RecognizeComplete(QuestionInput question, InputSource source, CallControls controls, TimeSpan? budget = null, CancellationToken cancellation = default);
CompleteCall<RelateRow> RelateComplete(QuestionInput question, InputSource source, CallControls controls, TimeSpan? budget = null, CancellationToken cancellation = default);
}
