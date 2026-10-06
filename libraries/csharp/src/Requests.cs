namespace ThinkThen;
public sealed record QuestionInput(Optional<Question> Question, Optional<string> File) {
public static QuestionInput Asked(Question question) => new(Optional.Some(question), default);
public static QuestionInput QuestionFile(string path) => new(default, Optional.Some(path));
}
public sealed record InputSource(Optional<IReadOnlyList<RecordInput>> Records, Optional<FileSource> Files) {
public static InputSource FromRecords(IReadOnlyList<RecordInput> records) => new(Optional.Some(records), default);
public static InputSource FromFiles(FileSource files) => new(default, Optional.Some(files));
}
/// <summary>Prepared host data; execution awaits the complete C constructors and calls.</summary>
public sealed record CompleteRequest(Function Function, QuestionInput Question, InputSource Source, CallControls Controls);
public static class Requests {
public static CompleteRequest Decide(QuestionInput question, InputSource source, CallControls controls) => new(Function.Decide, question, source, controls);
public static CompleteRequest Choose(QuestionInput question, InputSource source, CallControls controls) => new(Function.Choose, question, source, controls);
public static CompleteRequest Tag(QuestionInput question, InputSource source, CallControls controls) => new(Function.Tag, question, source, controls);
public static CompleteRequest Score(QuestionInput question, InputSource source, CallControls controls) => new(Function.Score, question, source, controls);
public static CompleteRequest Filter(QuestionInput question, InputSource source, CallControls controls) => new(Function.Filter, question, source, controls);
public static CompleteRequest Rank(QuestionInput question, InputSource source, CallControls controls) => new(Function.Rank, question, source, controls);
public static CompleteRequest Find(QuestionInput question, InputSource source, CallControls controls) => new(Function.Find, question, source, controls);
public static CompleteRequest Annotate(QuestionInput question, InputSource source, CallControls controls) => new(Function.Annotate, question, source, controls);
public static CompleteRequest Recognize(QuestionInput question, InputSource source, CallControls controls) => new(Function.Recognize, question, source, controls);
public static CompleteRequest Relate(QuestionInput question, InputSource source, CallControls controls) => new(Function.Relate, question, source, controls);
}
