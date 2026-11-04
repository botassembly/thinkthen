import dev.thinkthen.*;
import java.util.Arrays;
import java.util.Map;

/** Original-record annotation streams and direct named outcomes. */
public final class JavaAnnotationCases {
    private JavaAnnotationCases() {}
    public static void main(String[] args) throws Exception {
        var questions = AnnotationQuestions.builder().put("refund",DecideQuestion.of("Refund?"))
            .put("topic",ChooseQuestion.of("Which?",java.util.List.of("billing","shipping"))).build();
        var original = Map.of("id","first","text","Please refund this order.");
        try (var client = ThinkThen.builder().backend(Backend.STUB).build();
             var stream = client.annotateStream(questions,Arrays.asList(original,null,original))) {
            var output = stream.toList();
            JavaApiCases.check(output.size() == 3,"annotation preserves every input");
            JavaApiCases.check(output.get(0).item() == original && output.get(2).item() == original,"annotation keeps original object identity");
            JavaApiCases.check(output.get(1).item() == null && output.get(1).isFailed(),"annotation keeps a missing original and its failure");
            JavaApiCases.equal(output.get(2).position(),3L,"annotation keeps input positions");
            var answers = output.getFirst().answer();
            answers.decide("refund").answer();
            JavaApiCases.check(answers.choose("topic").detail().isPresent(),"named outcomes expose common detail");
            for (String name : java.util.List.of("absent","topic")) {
                try { answers.decide(name); throw new AssertionError("unknown or wrong-function name must throw"); }
                catch (ThinkThenException expected) { JavaApiCases.equal(expected.code().wireName(),"usage.argument","selector uses shared argument code"); }
            }
            try { output.get(1).row().detail(); throw new AssertionError("failed detail must throw"); }
            catch (ThinkThenException expected) { JavaApiCases.equal(expected.code().wireName(),"record.missing","detail preserves shared failure"); }
            JavaApiCases.equal(new Span("text",4_000_000_000L,4_000_000_004L).start(),4_000_000_000L,"span positions retain signed 64-bit values");
        }
        try (var server = new BackendFixture("unavailable"); var client = JavaOwnershipCases.remote(Backend.SYSTEMONE,server)) {
            for (Object blank : java.util.List.of("", "  ", Map.of(), Map.of("id","empty"), Map.of("text",""), Map.of("text",java.util.List.of()), Map.of("text",Map.of()))) {
                var options = DecideOptions.builder();
                if (blank instanceof Map<?,?> map && map.containsKey("text")) options.field(Pointer.of("/text"));
                try { client.decide("Ready?",blank,options.build()); throw new AssertionError("a blank single call must raise"); }
                catch (ThinkThenException expected) { JavaApiCases.equal(expected.code().wireName(),"record.empty","blank error comes from the shared engine"); }
            }
            JavaApiCases.equal(server.command("count"),"0","blank single calls make no backend request");
        }
        try (var client = ThinkThen.builder().backend(Backend.STUB).build(); var rows = client.decideStream("Ready?",java.util.List.of("alpha"," ","beta"))) {
            var outcomes = rows.toList();
            JavaApiCases.check(outcomes.size() == 3 && outcomes.getFirst().isAnswered() && outcomes.get(1).isFailed() && outcomes.getLast().isAnswered(),"blank stream rows retain both neighboring answers");
            JavaApiCases.equal(((Failed<?,?>) outcomes.get(1)).error().code().wireName(),"record.empty","blank stream row keeps engine code");
        }
        System.out.println("Java annotation original-record and blank-call contracts pass");
    }
}
