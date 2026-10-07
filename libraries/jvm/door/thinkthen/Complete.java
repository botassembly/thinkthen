package thinkthen;
import java.util.List;
import thinkthen.Ids.*;
import thinkthen.CompleteDetails.*;

/** Owned typed views of the native complete C boundary. */
public final class Complete {
    private Complete() {}
    public record OptionalValue<T>(boolean present, T value) {
        public static <T> OptionalValue<T> absent() { return new OptionalValue<>(false, null); }
        public static <T> OptionalValue<T> of(T value) { return new OptionalValue<>(true, value); }
    }
    public enum Function { DECIDE, CHOOSE, TAG, SCORE, FILTER, RANK, FIND, ANNOTATE, RECOGNIZE, RELATE }
    public enum ContentKind { TEXT, JSON, ABSENT }
    public enum RuleKind { DEFAULT, NULL, CUT, BAND }
    public enum Media { JPEG, PNG }
    public enum SourceUnit { LINE, WINDOW, FILE, IMAGE_FILE, JSONL }
    public enum ValueKind { NULL, BOOLEAN, AUTHORED }
    public enum AtomicKind { YES_NO, CHOICE, TAG, SCORE, FIND }
    public enum MemberState { SUCCESS, FAILURE }
    public enum MemberCause { MISSING_ANSWER, WRONG_KIND, MISSING_PROBABILITY, INVALID_PROBABILITY, INVALID_DISTRIBUTION, UNEXPECTED_PROBABILITY }
    public enum Origin { LIVE, CACHE, REPLAY, PROXY, MEMORY }
    public enum AttemptOutcome { OK, STATUS, TRANSPORT }
    public enum RelationMethod { YES_NO, CHOICE }
    public enum Direction { SOURCE_TO_TARGET, EITHER }
    public enum Stage { BOUNDARY, KIND, EDGE, RELATION }
    public enum IdentityKind { OBSERVATION, FAILURE }
    public enum StopCause { USAGE, LOCAL, NO_KEY, TRANSPORT, STATUS, TOO_LARGE, REPLY, BACKEND, CANCELLED, DEFECT, DEADLINE }
    public enum BatchKind { RECORDS, MAX }
    public enum EventKind { QUESTION, ROW }
    public record Content(ContentKind kind, String text, Object json) {}
    public record Rule(RuleKind kind, Double low, Double high) {}
    public record Choice(String name, OptionalValue<Content> description, OptionalValue<Double> weight) {}
    public record Relation(String name, String source, String target, OptionalValue<String> reads, Boolean either, Boolean single) {}
    public record QuestionMember(String name, Question question) {}
    public record Question(Function kind, Content text, OptionalValue<Content> yes, OptionalValue<Content> no, List<Choice> choices, Rule threshold, Rule relationThreshold, OptionalValue<String> model, OptionalValue<String> profile, OptionalValue<Long> batch, Boolean batchMax, Boolean none, List<String> on, List<QuestionMember> members, List<Choice> kinds, List<Relation> relations, OptionalValue<String> namePointer, OptionalValue<String> kindPointer, OptionalValue<QuestionAuthor> author) {
public Question(Function kind, Content text, OptionalValue<Content> yes, OptionalValue<Content> no, List<Choice> choices, Rule threshold, Rule relationThreshold, OptionalValue<String> model, OptionalValue<String> profile, OptionalValue<Long> batch, Boolean batchMax, Boolean none, List<String> on, List<QuestionMember> members, List<Choice> kinds, List<Relation> relations, OptionalValue<String> namePointer, OptionalValue<String> kindPointer) {this(kind,text,yes,no,choices,threshold,relationThreshold,model,profile,batch,batchMax,none,on,members,kinds,relations,namePointer,kindPointer,OptionalValue.absent());}
}

    public record ImageInput(Media media, byte[] bytes, OptionalValue<String> filename) {}
    public record ImageView(Media media, byte[] bytes, Long width, Long height, OptionalValue<String> filename) {}
    public record RecordInput(OptionalValue<Content> original, OptionalValue<Content> context, List<Choice> options, List<ImageInput> images) {}
    public record FileSource(List<String> paths, SourceUnit unit, Long window, boolean imageReader) { public FileSource(List<String> paths, SourceUnit unit, Long window) {this(paths,unit,window,false);} }
    public record CallControls(OptionalValue<Content> context, OptionalValue<Long> batch, Boolean batchMax, Boolean attempts) {}
    public record Probability(String name, Double probability) {}
    public record DecideValue(ValueKind kind, Boolean booleanValue, OptionalValue<Content> authored) {}
    public record AtomicAnswer(AtomicKind kind, OptionalValue<Double> probability, OptionalValue<String> pick, OptionalValue<String> level, List<Probability> probabilities, OptionalValue<Double> confidence) {}
    public record Location(OptionalValue<String> file, OptionalValue<Long> firstLine, OptionalValue<Long> lastLine) {}
    public record MemberValue(Function function, OptionalValue<DecideValue> decide, OptionalValue<String> choose, OptionalValue<List<String>> tag, OptionalValue<Double> score) {}
    public record MemberFailure(FailureId failureId, MemberCause cause) {}
    public record MemberSuccess(AnswerId answerId, MemberValue value, AtomicAnswer answer, Rule threshold) {}
    public record AnnotationMember(String name, Digest request, Question question, MemberState state, OptionalValue<MemberSuccess> success, OptionalValue<MemberFailure> failure, QuestionAuthor author) {
public AnnotationMember(String name, Digest request, Question question, MemberState state, OptionalValue<MemberSuccess> success, OptionalValue<MemberFailure> failure) {this(name,request,question,state,success,failure,null);}
}

    public record Entity(String text, Long start, Long end, Long length, String kind, Double strength) {}
    public record EntityEdge(String relation, Entity source, Entity target, Double probability, Boolean either) {}
    public record Place(Long start, Long end) {}
    public record Piece(Long start, Long end, List<Probability> tags) {}
    public record NameSpan(Long start, Long end, OptionalValue<List<Probability>> kinds, OptionalValue<List<Probability>> edges) {}
    public record PairSpan(String relation, Place source, Place target, Double probability) {}
    public record RecognizeValue(List<Entity> entities, OptionalValue<List<EntityEdge>> relations) {}
    public record RecognizeAnswer(List<Piece> pieces, List<NameSpan> names, List<PairSpan> pairs) {}
    public record Endpoint(String name, String kind) {}
    public record Edge(String relation, Endpoint source, Endpoint target, Double probability, Boolean either) {}
    public record RelationSuccess(AnswerId answerId, Double probability, Boolean accepted) {}
    public record RelationAnswer(String relation, String reads, RelationMethod method, Direction direction, Endpoint source, OptionalValue<Endpoint> target, Digest request, MemberState state, OptionalValue<RelationSuccess> success, OptionalValue<MemberFailure> failure) {}
    public record TokenUsage(Long inputTokens, Long outputTokens) {}
    public record QuestionSource(Origin origin, String answeredBy) {}
    public record ObservationIdentity(IdentityKind kind, OptionalValue<ObservationId> observationId, OptionalValue<FailureId> failureId) {}
    public record ProfileWarning(String tunedFor, String running) {}
    public record BatchSetting(BatchKind kind, Long records) {}
    public record BatchWarning(BatchSetting tunedFor, BatchSetting running) {}
    public record Attempt(Long ordinal, Digest requestSha256, Long wallMs, AttemptOutcome outcome, SdkRequestId sdkRequestId, OptionalValue<Long> status, OptionalValue<Long> serverMs, OptionalValue<String> requestId) {}
    public record Meta(String tool, OptionalValue<Digest> questionSha256, OptionalValue<Digest> questionsSha256, String url, String model, OptionalValue<TokenUsage> usage, Long requestsSent, Boolean cached, List<Digest> requests, Long failedQuestions, OptionalValue<ProfileWarning> profileWarning, OptionalValue<BatchSetting> batchSetting, OptionalValue<BatchWarning> batchWarning, OptionalValue<Digest> contextSha256, OptionalValue<List<Attempt>> attempts, OptionalValue<Origin> origin, List<QuestionSource> questionSources, List<ObservationIdentity> observations, OptionalValue<String> answeredBy) {}
    public record CallFacts(CallId callId, Long cacheAnswers, OptionalValue<String> estimatedCostUsd, OptionalValue<Long> inputTokens, OptionalValue<String> model, OptionalValue<Long> outputTokens, Long records, Long requestsSent, Double seconds, OptionalValue<Long> commandMs) {}
    public record Stopped(OptionalValue<Long> at, StopCause cause, OptionalValue<Long> status, Boolean retryable) {}
    public record CompleteError(Long code, String message, Boolean retryable, OptionalValue<Stopped> stopped, OptionalValue<CallFacts> facts, OptionalValue<List<Attempt>> attempts) {}
    public record CommonRow(AnswerId answerId, OptionalValue<Content> input, OptionalValue<Question> question, OptionalValue<AtomicAnswer> answer, OptionalValue<Rule> threshold, OptionalValue<Location> position, OptionalValue<String> inputFile, Meta meta, OptionalValue<List<ImageView>> images, Details details, QuestionAuthor author) {
public CommonRow(AnswerId answerId, OptionalValue<Content> input, OptionalValue<Question> question, OptionalValue<AtomicAnswer> answer, OptionalValue<Rule> threshold, OptionalValue<Location> position, OptionalValue<String> inputFile, Meta meta, OptionalValue<List<ImageView>> images) {this(answerId,input,question,answer,threshold,position,inputFile,meta,images,null,null);}
}

    public record DecideRow(CommonRow common, DecideValue value) {}
    public record ChooseRow(CommonRow common, OptionalValue<String> value) {}
    public record TagRow(CommonRow common, List<String> value) {}
    public record ScoreRow(CommonRow common, Double value) {}
    public record FilterRow(CommonRow common, Boolean value) {}
    public record RankRow(CommonRow common, OptionalValue<Long> value, OptionalValue<String> questionName, List<RankRow> members) {
public RankRow(CommonRow common, OptionalValue<Long> value, OptionalValue<String> questionName) {this(common,value,questionName,List.of());}
}

    public record FindRow(CommonRow common, OptionalValue<Content> value, OptionalValue<Long> index) {}
    public record AnnotateRow(CommonRow common, List<AnnotationMember> answers) {}
    public record RecognizeRow(CommonRow common, RecognizeValue value, RecognizeAnswer answer, SourceRecognition located) {
public RecognizeRow(CommonRow common, RecognizeValue value, RecognizeAnswer answer) {this(common,value,answer,null);}
}

    public record RelateRow(CommonRow common, List<Edge> value, List<RelationAnswer> questions, SourceRelations located) {
public RelateRow(CommonRow common, List<Edge> value, List<RelationAnswer> questions) {this(common,value,questions,null);}
}

    public record RowValue(Function function, OptionalValue<DecideRow> decide, OptionalValue<ChooseRow> choose, OptionalValue<TagRow> tag, OptionalValue<ScoreRow> score, OptionalValue<FilterRow> filter, OptionalValue<RankRow> rank, OptionalValue<FindRow> find, OptionalValue<AnnotateRow> annotate, OptionalValue<RecognizeRow> recognize, OptionalValue<RelateRow> relate) {}
    public record ObservedProbabilities(OptionalValue<Double> yes, OptionalValue<List<Probability>> named) {}
    public record ObservationSuccess(AnswerId answerId, OptionalValue<ObservationId> observationId, MemberValue value, ObservedProbabilities probabilities, OptionalValue<Double> confidence) {}
    public record QuestionObservation(Long index, OptionalValue<String> member, OptionalValue<Stage> stage, Long position, Digest questionSha256, String model, String url, List<Digest> requests, Long requestsSent, Boolean cached, Long failedQuestions, OptionalValue<TokenUsage> usage, List<QuestionSource> questionSources, MemberState state, OptionalValue<ObservationSuccess> success, OptionalValue<MemberFailure> failure) {}
    public record RowObservation(Long index, RowValue value) {}
    public record ObservationEvent(EventKind kind, OptionalValue<QuestionObservation> question, OptionalValue<RowObservation> row, Details details, QuestionAuthor author) {
public ObservationEvent(EventKind kind, OptionalValue<QuestionObservation> question, OptionalValue<RowObservation> row) {this(kind,question,row,null,null);}
}

}
