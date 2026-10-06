package thinkthen;
import java.util.List;
import thinkthen.Complete.*;
import thinkthen.Ids.AnswerId;
import thinkthen.Requests.*;
/** Integration contract; Door does not yet implement the complete C boundary. */
public interface CompleteEngine {
record CompleteCall<T>(String schema, Function function, AnswerId answerId, Meta meta, CallFacts facts, OptionalValue<List<Attempt>> attempts, List<T> rows, List<ObservationEvent> observations) {}
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
}
