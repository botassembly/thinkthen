using System.Text.Json;
using System.Text.Json.Serialization;
namespace ThinkThen;

/// <summary>Presence is separate from the value, including zero and empty arrays.</summary>
public readonly record struct Optional<T>(bool Present, T Value);
public static class Optional { public static Optional<T> Some<T>(T value) => new(true, value); }

public enum Function { Decide, Choose, Tag, Score, Filter, Rank, Find, Annotate, Recognize, Relate }
public enum ContentKind { Text, Json }
public enum RuleKind { Default, Null, Cut, Band }
public enum Media { Jpeg, Png }
public enum SourceUnit { Line, Window, File, ImageFile }
public enum ValueKind { Null, Boolean, Authored }
public enum AtomicKind { YesNo, Choice, Tag, Score, Find }
public enum MemberState { Success, Failure }
public enum MemberCause { MissingAnswer, WrongKind, MissingProbability, InvalidProbability, InvalidDistribution, UnexpectedProbability }
public enum Origin { Live, Cache, Replay, Proxy, Memory }
public enum AttemptOutcome { Ok, Status, Transport }
public enum RelationMethod { YesNo, Choice }
public enum Direction { SourceToTarget, Either }
public enum Stage { Boundary, Kind, Edge, Relation }
public enum IdentityKind { Observation, Failure }
public enum StopCause { Usage, Local, NoKey, Transport, Status, TooLarge, Reply, Backend, Cancelled, Defect, Deadline }
public enum BatchKind { Records, Max }
public enum EventKind { Question, Row }
public sealed record Content(ContentKind Kind, string Text, [property: JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingDefault)] JsonElement Json);
public sealed record Rule(RuleKind Kind, double Low, double High);
public sealed record Choice(string Name, Optional<Content> Description, Optional<double> Weight);
public sealed record Relation(string Name, string Source, string Target, Optional<string> Reads, bool Either, bool Single);
public sealed record QuestionMember(string Name, Question Question);
public sealed record Question(Function Kind, Content Text, Optional<Content> Yes, Optional<Content> No, IReadOnlyList<Choice> Choices, Rule Threshold, Rule RelationThreshold, Optional<string> Model, Optional<string> Profile, Optional<ulong> Batch, bool BatchMax, bool None, IReadOnlyList<string> On, IReadOnlyList<QuestionMember> Members, IReadOnlyList<Choice> Kinds, IReadOnlyList<Relation> Relations, Optional<string> NamePointer, Optional<string> KindPointer);
public sealed record ImageInput(Media Media, byte[] Bytes, Optional<string> Filename);
public sealed record ImageView(Media Media, byte[] Bytes, ulong Width, ulong Height, Optional<string> Filename);
public sealed record RecordInput(Optional<Content> Original, Optional<Content> Context, IReadOnlyList<Choice> Options, IReadOnlyList<ImageInput> Images);
public sealed record FileSource(IReadOnlyList<string> Paths, SourceUnit Unit, ulong Window);
public sealed record CallControls(Optional<Content> Context, Optional<ulong> Batch, bool BatchMax, bool Attempts);
public sealed record Probability(string Name, double Value);
public sealed record DecideValue(ValueKind Kind, bool Boolean, Optional<Content> Authored);
public sealed record AtomicAnswer(AtomicKind Kind, Optional<double> Probability, Optional<string> Pick, Optional<string> Level, IReadOnlyList<Probability> Probabilities, Optional<double> Confidence);
public sealed record Location(Optional<string> File, Optional<ulong> FirstLine, Optional<ulong> LastLine);
public sealed record MemberValue(Function Function, Optional<DecideValue> Decide, Optional<string> Choose, Optional<IReadOnlyList<string>> Tag, Optional<double> Score);
public sealed record MemberFailure(FailureId FailureId, MemberCause Cause);
public sealed record MemberSuccess(AnswerId AnswerId, MemberValue Value, AtomicAnswer Answer, Rule Threshold);
public sealed record AnnotationMember(string Name, Digest Request, Question Question, MemberState State, Optional<MemberSuccess> Success, Optional<MemberFailure> Failure);
public sealed record Entity(string Text, ulong Start, ulong End, ulong Length, string Kind, double Strength);
public sealed record EntityEdge(string Relation, Entity Source, Entity Target, double Probability, bool Either);
public sealed record Place(ulong Start, ulong End);
public sealed record Piece(ulong Start, ulong End, IReadOnlyList<Probability> Tags);
public sealed record NameSpan(ulong Start, ulong End, Optional<IReadOnlyList<Probability>> Kinds, Optional<IReadOnlyList<Probability>> Edges);
public sealed record PairSpan(string Relation, Place Source, Place Target, double Probability);
public sealed record RecognizeValue(IReadOnlyList<Entity> Entities, Optional<IReadOnlyList<EntityEdge>> Relations);
public sealed record RecognizeAnswer(IReadOnlyList<Piece> Pieces, IReadOnlyList<NameSpan> Names, IReadOnlyList<PairSpan> Pairs);
public sealed record Endpoint(string Name, string Kind);
public sealed record Edge(string Relation, Endpoint Source, Endpoint Target, double Probability, bool Either);
public sealed record RelationSuccess(AnswerId AnswerId, double Probability, bool Accepted);
public sealed record RelationAnswer(string Relation, string Reads, RelationMethod Method, Direction Direction, Endpoint Source, Optional<Endpoint> Target, Digest Request, MemberState State, Optional<RelationSuccess> Success, Optional<MemberFailure> Failure);
public sealed record TokenUsage(ulong InputTokens, ulong OutputTokens);
public sealed record QuestionSource(Origin Origin, string AnsweredBy);
public sealed record ObservationIdentity(IdentityKind Kind, Optional<ObservationId> ObservationId, Optional<FailureId> FailureId);
public sealed record ProfileWarning(string TunedFor, string Running);
public sealed record BatchSetting(BatchKind Kind, ulong Records);
public sealed record BatchWarning(BatchSetting TunedFor, BatchSetting Running);
public sealed record Attempt(ulong Ordinal, Digest RequestSha256, ulong WallMs, AttemptOutcome Outcome, SdkRequestId SdkRequestId, Optional<ulong> Status, Optional<ulong> ServerMs, Optional<string> RequestId);
public sealed record Meta(string Tool, Optional<Digest> QuestionSha256, Optional<Digest> QuestionsSha256, string Url, string Model, Optional<TokenUsage> Usage, ulong RequestsSent, bool Cached, IReadOnlyList<Digest> Requests, ulong FailedQuestions, Optional<ProfileWarning> ProfileWarning, Optional<BatchSetting> BatchSetting, Optional<BatchWarning> BatchWarning, Optional<Digest> ContextSha256, Optional<IReadOnlyList<Attempt>> Attempts, Optional<Origin> Origin, IReadOnlyList<QuestionSource> QuestionSources, IReadOnlyList<ObservationIdentity> Observations, Optional<string> AnsweredBy);
public sealed record CallFacts(CallId CallId, ulong CacheAnswers, Optional<string> EstimatedCostUsd, Optional<ulong> InputTokens, Optional<string> Model, Optional<ulong> OutputTokens, ulong Records, ulong RequestsSent, double Seconds, Optional<ulong> CommandMs);
public sealed record Stopped(Optional<ulong> At, StopCause Cause, Optional<ulong> Status, bool Retryable);
public sealed record CompleteError(ulong Code, string Message, bool Retryable, Optional<Stopped> Stopped, Optional<CallFacts> Facts, Optional<IReadOnlyList<Attempt>> Attempts);
public sealed record CommonRow(AnswerId AnswerId, Optional<Content> Input, Optional<Question> Question, Optional<AtomicAnswer> Answer, Optional<Rule> Threshold, Optional<Location> Position, Optional<string> InputFile, Meta Meta, Optional<IReadOnlyList<ImageView>> Images);
public sealed record DecideRow(CommonRow Common, DecideValue Value);
public sealed record ChooseRow(CommonRow Common, Optional<string> Value);
public sealed record TagRow(CommonRow Common, IReadOnlyList<string> Value);
public sealed record ScoreRow(CommonRow Common, double Value);
public sealed record FilterRow(CommonRow Common, bool Value);
public sealed record RankRow(CommonRow Common, Optional<ulong> Value, Optional<string> QuestionName);
public sealed record FindRow(CommonRow Common, Optional<Content> Value, Optional<ulong> Index);
public sealed record AnnotateRow(CommonRow Common, IReadOnlyList<AnnotationMember> Answers);
public sealed record RecognizeRow(CommonRow Common, RecognizeValue Value, RecognizeAnswer Answer);
public sealed record RelateRow(CommonRow Common, IReadOnlyList<Edge> Value, IReadOnlyList<RelationAnswer> Questions);
public sealed record RowValue(Function Function, Optional<DecideRow> Decide, Optional<ChooseRow> Choose, Optional<TagRow> Tag, Optional<ScoreRow> Score, Optional<FilterRow> Filter, Optional<RankRow> Rank, Optional<FindRow> Find, Optional<AnnotateRow> Annotate, Optional<RecognizeRow> Recognize, Optional<RelateRow> Relate);
public sealed record ObservedProbabilities(Optional<double> Yes, Optional<IReadOnlyList<Probability>> Named);
public sealed record ObservationSuccess(AnswerId AnswerId, ObservationId ObservationId, MemberValue Value, ObservedProbabilities Probabilities, Optional<double> Confidence);
public sealed record QuestionObservation(ulong Index, Optional<string> Member, Optional<Stage> Stage, ulong Position, Digest QuestionSha256, string Model, string Url, IReadOnlyList<Digest> Requests, ulong RequestsSent, bool Cached, ulong FailedQuestions, Optional<TokenUsage> Usage, IReadOnlyList<QuestionSource> QuestionSources, MemberState State, Optional<ObservationSuccess> Success, Optional<MemberFailure> Failure);
public sealed record RowObservation(ulong Index, RowValue Value);
public sealed record ObservationEvent(EventKind Kind, Optional<QuestionObservation> Question, Optional<RowObservation> Row);
