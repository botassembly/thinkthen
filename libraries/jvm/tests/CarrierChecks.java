import java.util.List;
import thinkthen.Complete.*;
import thinkthen.CompleteReaders;
import thinkthen.Ids.*;
import thinkthen.Json;
import thinkthen.Requests;

/** Pure fixture checks, independent of compatibility and native calls. */
public final class CarrierChecks {
    private CarrierChecks() {}
    public static void main(String[] args) { run(); }
    private static void require(boolean value) { if (!value) throw new AssertionError("owned carrier contract"); }
    private static void refuses(Runnable action) {
        try { action.run(); } catch (IllegalArgumentException | IllegalStateException | ArithmeticException expected) { return; }
        throw new AssertionError("invalid serialized field accepted");
    }
    public static void run() {
        String id = "a".repeat(64);
        CallFacts facts = CompleteReaders.facts("{\"call_id\":\""+id+"\",\"cache_answers\":0,\"records\":0,\"requests_sent\":0,\"seconds\":0,\"input_tokens\":0,\"estimated_cost_usd\":\"0.000000\"}");
        require(facts.callId().value().equals(id) && facts.inputTokens().present() && facts.inputTokens().value() == 0 && !facts.outputTokens().present());
        require(facts.estimatedCostUsd().value().equals("0.000000"));
        refuses(() -> CompleteReaders.facts("{\"cache_answers\":0,\"records\":0,\"requests_sent\":0,\"seconds\":0}"));
        refuses(() -> new AnswerId("A".repeat(64)));
        refuses(() -> CompleteReaders.atomic("{\"kind\":\"other\"}"));
        refuses(() -> CompleteReaders.atomic("{\"kind\":\"yes_no\",\"probability\":null}"));
        refuses(() -> CompleteReaders.atomic("{\"kind\":\"tag\",\"probabilities\":{\"a\":null}}"));
        refuses(() -> CompleteReaders.probabilities("{\"a\":1,\"a\":0}"));
        AtomicAnswer choice = CompleteReaders.atomic("{\"kind\":\"choice\",\"pick\":\"β\",\"probabilities\":{\"β\":0.7,\"a\":0.3},\"confidence\":0}");
        require(choice.probabilities().getFirst().name().equals("β") && choice.confidence().present() && choice.confidence().value() == 0);
        for (String fixture : List.of("{\"kind\":\"yes_no\",\"probability\":0}", "{\"kind\":\"tag\",\"probabilities\":{}}", "{\"kind\":\"score\",\"level\":\"a\",\"probabilities\":{\"a\":1}}", "{\"kind\":\"find\",\"pick\":\"a\",\"probabilities\":{\"a\":1}}")) CompleteReaders.atomic(fixture);
        var question = Requests.QuestionInput.questionFile("β.json");
        var source = Requests.InputSource.files(new FileSource(List.of("β.png", "β.png"), SourceUnit.IMAGE_FILE, 0L));
        var controls = new CallControls(OptionalValue.absent(), OptionalValue.absent(), false, true);
        var requests = List.of(Requests.decide(question, source, controls), Requests.choose(question, source, controls), Requests.tag(question, source, controls), Requests.score(question, source, controls), Requests.filter(question, source, controls), Requests.rank(question, source, controls), Requests.find(question, source, controls), Requests.annotate(question, source, controls), Requests.recognize(question, source, controls), Requests.relate(question, source, controls));
        var functions = List.of(Function.DECIDE, Function.CHOOSE, Function.TAG, Function.SCORE, Function.FILTER, Function.RANK, Function.FIND, Function.ANNOTATE, Function.RECOGNIZE, Function.RELATE);
        for (int i=0; i<requests.size(); i++) require(requests.get(i).function() == functions.get(i));
        require(question.file().present() && !question.question().present() && source.files().value().paths().size() == 2);
        var presentEmpty = OptionalValue.of(List.<Probability>of());
        var span = new NameSpan(1L, 3L, presentEmpty, OptionalValue.absent());
        require(span.kinds().present() && !span.edges().present() && span.end() == 3);
        var record = new RecordInput(OptionalValue.of(new Content(ContentKind.TEXT, "", null)), OptionalValue.absent(), List.of(), List.of(new ImageInput(Media.PNG, new byte[]{1,2}, OptionalValue.absent())));
        require(record.original().present() && record.original().value().text().isEmpty());
        var authored = new Content(ContentKind.JSON, "", Json.parseObject("{\"false\":false,\"null\":null}"));
        require(((java.util.Map<?, ?>) authored.json()).containsKey("null"));
        var identity = new ObservationIdentity(IdentityKind.OBSERVATION, OptionalValue.of(new ObservationId("d".repeat(64))), OptionalValue.absent());
        var duplicates = List.of(identity, identity);
        require(duplicates.size() == 2 && duplicates.get(0).observationId().value().equals(duplicates.get(1).observationId().value()));
        var error = new CompleteError(5L, "cancelled", false, OptionalValue.absent(), OptionalValue.of(facts), OptionalValue.of(List.of()));
        require(error.facts().present() && error.attempts().present() && error.attempts().value().isEmpty());
        // The typed success/failure arms cannot conflate failure with successful null.
        var failed = new MemberFailure(new FailureId("b".repeat(64)), MemberCause.MISSING_PROBABILITY);
        require(failed.cause() == MemberCause.MISSING_PROBABILITY);
    }
}
