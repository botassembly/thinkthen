package thinkthen;
import java.util.List;
import thinkthen.Complete.*;
/** Prepared host data; execution awaits the complete C constructors and calls. */
public final class Requests { private Requests() {}
public record QuestionInput(OptionalValue<Question> question, OptionalValue<String> file) {
public static QuestionInput asked(Question question) { return new QuestionInput(OptionalValue.of(question), OptionalValue.absent()); }
public static QuestionInput questionFile(String path) { return new QuestionInput(OptionalValue.absent(), OptionalValue.of(path)); }
}
public record InputSource(OptionalValue<List<RecordInput>> records, OptionalValue<FileSource> files) {
public static InputSource records(List<RecordInput> records) { return new InputSource(OptionalValue.of(records), OptionalValue.absent()); }
public static InputSource files(FileSource files) { return new InputSource(OptionalValue.absent(), OptionalValue.of(files)); }
}
public record CompleteRequest(Function function, QuestionInput question, InputSource source, CallControls controls) {}
public static CompleteRequest decide(QuestionInput question, InputSource source, CallControls controls) { return new CompleteRequest(Function.DECIDE, question, source, controls); }
public static CompleteRequest choose(QuestionInput question, InputSource source, CallControls controls) { return new CompleteRequest(Function.CHOOSE, question, source, controls); }
public static CompleteRequest tag(QuestionInput question, InputSource source, CallControls controls) { return new CompleteRequest(Function.TAG, question, source, controls); }
public static CompleteRequest score(QuestionInput question, InputSource source, CallControls controls) { return new CompleteRequest(Function.SCORE, question, source, controls); }
public static CompleteRequest filter(QuestionInput question, InputSource source, CallControls controls) { return new CompleteRequest(Function.FILTER, question, source, controls); }
public static CompleteRequest rank(QuestionInput question, InputSource source, CallControls controls) { return new CompleteRequest(Function.RANK, question, source, controls); }
public static CompleteRequest find(QuestionInput question, InputSource source, CallControls controls) { return new CompleteRequest(Function.FIND, question, source, controls); }
public static CompleteRequest annotate(QuestionInput question, InputSource source, CallControls controls) { return new CompleteRequest(Function.ANNOTATE, question, source, controls); }
public static CompleteRequest recognize(QuestionInput question, InputSource source, CallControls controls) { return new CompleteRequest(Function.RECOGNIZE, question, source, controls); }
public static CompleteRequest relate(QuestionInput question, InputSource source, CallControls controls) { return new CompleteRequest(Function.RELATE, question, source, controls); }
}
