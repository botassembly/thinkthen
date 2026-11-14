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
            JavaApiCases.check(answers.choose("topic").detail() instanceof LabelDetail,"named outcomes expose common detail");
            for (String name : java.util.List.of("absent","topic")) {
                try { answers.decide(name); throw new AssertionError("unknown or wrong-function name must throw"); }
                catch (ThinkThenException expected) { JavaApiCases.equal(expected.code().wireName(),"usage.argument","selector uses shared argument code"); }
            }
            JavaApiCases.check(output.get(1).detail() instanceof NoDetail, "failed detail is safe and absent");
            JavaApiCases.equal(new Span("text",2_000_000_000,2_000_000_004).start(),2_000_000_000,"span positions fit Java string indexes");
        }
        try (var server = new BackendFixture("unavailable"); var client = JavaOwnershipCases.remote(Backend.SYSTEMONE,server)) {
            for (Object blank : java.util.List.of("", "  ", Map.of(), Map.of("id","empty"), Map.of("text",""), Map.of("text",java.util.List.of()), Map.of("text",Map.of()))) {
                var options = DecideOptions.builder();
                if (blank instanceof Map<?,?> map && map.containsKey("text")) options.field(Pointer.of("/text"));
                JavaApiCases.failedCall(() -> client.decide("Ready?",blank,options.build()), "record.empty");
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
