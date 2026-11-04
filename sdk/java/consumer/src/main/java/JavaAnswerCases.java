import dev.thinkthen.*;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Optional;
import java.util.OptionalDouble;
import java.util.OptionalInt;

/** Public answer and ordered-value contracts, without a backend or native loader. */
public final class JavaAnswerCases {
    private JavaAnswerCases() {}
    private static void check(boolean value, String message) {
        if (!value) throw new AssertionError(message);
    }
    public static void main(String[] args) throws Exception {
        var answer = Row.class.getMethod("answer");
        var probability = Row.class.getMethod("probability");
        var unsureFlag = Row.class.getMethod("isUnsure");
        var info = new RowInfo(dev.thinkthen.FunctionName.DECIDE, 7L, Optional.empty(), Optional.empty());
        Row<Boolean,NoDetail> isRefund = new Answered<>(info, false, OptionalDouble.of(0.1), Optional.empty());
        check(answer.invoke(isRefund).equals(false), "false remains a usable answer");
        check(probability.invoke(isRefund).equals(OptionalDouble.of(0.1)), "probability is available through Row");
        Row<Boolean,NoDetail> needsReview = new Unsure<>(info, OptionalDouble.of(0.5), Optional.empty());
        check(unsureFlag.invoke(needsReview).equals(true), "unsure is directly testable");
        try { answer.invoke(needsReview); throw new AssertionError("unsure cannot become false or null"); }
        catch (java.lang.reflect.InvocationTargetException expected) {
            check(expected.getCause() instanceof java.util.NoSuchElementException, "unsure uses the standard missing-value exception family");
            check(!(expected.getCause() instanceof ThinkThenException), "unsure is not an engine failure");
        }
        var code = ErrorCode.fromWire("record.missing");
        Row<Boolean,NoDetail> failed = new Failed<>(info, new RowError(code, "missing record", OptionalInt.empty()));
        try { answer.invoke(failed); throw new AssertionError("failure cannot become false or null"); }
        catch (java.lang.reflect.InvocationTargetException expected) {
            check(expected.getCause() instanceof ThinkThenException error && error.code() == code, "failure retains its engine code");
        }
        check(probability.invoke(failed).equals(OptionalDouble.empty()), "failed rows have no probability");
        var labels = new LinkedHashMap<String,Double>();
        labels.put("z_first", 0.3); labels.put("a_second", 0.7); labels.put("m_third", 0.0);
        var detail = new LabelDetail(labels);
        check(List.copyOf(detail.labels().keySet()).equals(List.copyOf(labels.keySet())), "label order follows the caller");
        labels.clear();
        check(detail.labels().size() == 3, "result owns an immutable snapshot");
        try { detail.labels().clear(); throw new AssertionError("result map is immutable"); }
        catch (UnsupportedOperationException expected) { }
        System.out.println("Java public answer and ordered-value contracts pass");
    }
}
