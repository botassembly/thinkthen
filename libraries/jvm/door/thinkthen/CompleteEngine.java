package thinkthen;
import java.util.List;
import thinkthen.Complete.*;
import thinkthen.Ids.AnswerId;
import thinkthen.Requests.*;
/** Named typed native calls implemented by each JVM consumer. */
public interface CompleteEngine {
record CompleteCall<T>(String schema, OptionalValue<Function> function, OptionalValue<AnswerId> answerId, OptionalValue<Meta> meta, OptionalValue<CallFacts> facts, OptionalValue<List<Attempt>> attempts, List<T> rows, List<ObservationEvent> observations, OptionalValue<CompleteError> error) {}
CompleteCall<DecideRow> decideComplete(QuestionInput question, InputSource source, CallControls controls, long deadlineMs, Door.Token token);
CompleteCall<ChooseRow> chooseComplete(QuestionInput question, InputSource source, CallControls controls, long deadlineMs, Door.Token token);
CompleteCall<TagRow> tagComplete(QuestionInput question, InputSource source, CallControls controls, long deadlineMs, Door.Token token);
CompleteCall<ScoreRow> scoreComplete(QuestionInput question, InputSource source, CallControls controls, long deadlineMs, Door.Token token);
CompleteCall<FilterRow> filterComplete(QuestionInput question, InputSource source, CallControls controls, long deadlineMs, Door.Token token);
CompleteCall<RankRow> rankComplete(QuestionInput question, InputSource source, CallControls controls, long deadlineMs, Door.Token token);
CompleteCall<FindRow> findComplete(QuestionInput question, InputSource source, CallControls controls, long deadlineMs, Door.Token token);
CompleteCall<AnnotateRow> annotateComplete(QuestionInput question, InputSource source, CallControls controls, long deadlineMs, Door.Token token);
CompleteCall<RecognizeRow> recognizeComplete(QuestionInput question, InputSource source, CallControls controls, long deadlineMs, Door.Token token);
CompleteCall<RelateRow> relateComplete(QuestionInput question, InputSource source, CallControls controls, long deadlineMs, Door.Token token);
NativeBatch<DecideRow> decideBatch(QuestionInput q,InputSource s,CallControls c,long deadlineMs,Door.Token token);
NativeBatch<ChooseRow> chooseBatch(QuestionInput q,InputSource s,CallControls c,long deadlineMs,Door.Token token);
NativeBatch<TagRow> tagBatch(QuestionInput q,InputSource s,CallControls c,long deadlineMs,Door.Token token);
NativeBatch<ScoreRow> scoreBatch(QuestionInput q,InputSource s,CallControls c,long deadlineMs,Door.Token token);
NativeBatch<FilterRow> filterBatch(QuestionInput q,InputSource s,CallControls c,long deadlineMs,Door.Token token);
NativeBatch<AnnotateRow> annotateBatch(QuestionInput q,InputSource s,CallControls c,long deadlineMs,Door.Token token);
}
