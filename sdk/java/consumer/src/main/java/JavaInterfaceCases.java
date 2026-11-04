import dev.thinkthen.*;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

/** Outside-in Java interface contracts against the packaged stub engine. */
public final class JavaInterfaceCases {
    private JavaInterfaceCases() {}
    private static void check(boolean value, String claim) { if (!value) throw new AssertionError(claim); }
    public static void main(String[] args) {
        try (var client = ThinkThen.builder().backend(Backend.STUB).build()) {
            boolean isRefund = client.decide("Refund?", "Please refund my order").answer();
            check(isRefund || !isRefund, "decide exposes its Boolean answer");
            var needsReview = client.decide("Refund?", "Please refund my order", DecideOptions.builder().threshold(Threshold.band(0.000001,1)).build());
            check(needsReview.isUnsure(), "band produces an unsure row");
            try { needsReview.answer(); throw new AssertionError("unsure must throw"); }
            catch (UnsureException expected) { check(expected.probability() == needsReview.probability().orElseThrow(), "unsure exception carries probability"); }
            for (Object input : List.of(List.of("a"), new String[]{"a"}, java.util.stream.Stream.of("a"), List.of("a").iterator(), new Object())) {
                try { client.decide("Ready?",input); throw new AssertionError("single call accepted " + input.getClass()); }
                catch (IllegalArgumentException expected) { }
            }
            List<String> records = Arrays.asList(null,"first","second",null,"third");
            var kept = new ArrayList<String>();
            List<Filtered<String>> outcomes;
            try (var rows = client.filterRows("Ready?",records,FilterOptions.builder().threshold(Threshold.cut(0.000001)).build())) { outcomes = rows.toList(); }
            try (var complaints = client.filter("Ready?",records,FilterOptions.builder().threshold(Threshold.cut(0.000001)).build())) {
                complaints.forEach(kept::add);
                throw new AssertionError("failed records must raise after kept items");
            } catch (ThinkThenException expected) {
                check(expected.failures().size() == 2, "one exception carries every failed row");
                check(expected.failures().get(0).position() == 1 && expected.failures().get(1).position() == 4, "failed rows retain input positions");
            }
            check(kept.size() == 3, "all nonmissing positive items arrive before the failure");
            check(kept.equals(outcomes.stream().filter(Filtered::kept).map(Filtered::item).toList()), "kept-only filter returns original matching items before failure");
            var descriptions = Map.of("billing","Invoices and refunds","shipping","Delivery and parcels");
            var route = client.choose("Which team?","refund",List.of("billing","shipping"),ChooseOptions.builder().descriptions(descriptions).build());
            check(route.isAnswered(), "label descriptions reach the shared engine");
            var refund = client.decide("Refund?","refund",DecideOptions.builder().yesMeans("Requests money back").noMeans("Does not request money back").build());
            check(refund.isAnswered(), "yes and no descriptions reach the shared engine");
            var priority = RankQuestion.of("Priority?",List.of("low","high"),RankOptions.builder().descriptions(Map.of("high","Urgent")).build());
            String describedIdentity;
            try (var ranked = client.rank(priority,List.of("alpha","beta"))) {
                var output = ranked.toList();
                check(output.size() == 2 && output.stream().allMatch(Ranked::isAnswered), "rank descriptions reach the packaged engine");
                describedIdentity = output.getFirst().row().source().orElseThrow().questionIdentity();
            }
            try (var ranked = client.rank("Priority?",List.of("alpha","beta"),List.of("low","high"))) {
                check(!describedIdentity.equals(ranked.findFirst().orElseThrow().row().source().orElseThrow().questionIdentity()), "rank descriptions contribute to question identity");
            }
            var saved = Toolbox.builder().put("priority",priority).build().toJson();
            check(saved.contains("Urgent"), "saved rank questions retain level descriptions");
            try (var ranked = client.rank(Toolbox.fromJson(saved).rank("priority").orElseThrow(),List.of("alpha","beta"))) {
                check(describedIdentity.equals(ranked.findFirst().orElseThrow().row().source().orElseThrow().questionIdentity()), "saved rank descriptions survive typed admission");
            }
            var questions = AnnotationQuestions.builder().put("z_first",DecideQuestion.of("Ready?"))
                .put("a_second",DecideQuestion.of("Ready?" )).put("m_third",DecideQuestion.of("Ready?")).build();
            var triage = client.annotate(questions,"alpha").answer();
            check(List.copyOf(triage.entries().keySet()).equals(List.of("z_first","a_second","m_third")), "annotation names retain caller order");
            try { AnnotationQuestions.builder().put("same",DecideQuestion.of("Ready?")).put("same",DecideQuestion.of("Ready?")); throw new AssertionError("duplicate annotation names must not overwrite"); }
            catch (IllegalArgumentException expected) { }
            var taxonomy = new LinkedHashMap<String,String>();
            taxonomy.put("z_first","first"); taxonomy.put("a_second","second"); taxonomy.put("m_third","third");
            var choice = client.choose("Which?","alpha",List.copyOf(taxonomy.keySet()),ChooseOptions.builder().descriptions(taxonomy).build());
            check(List.copyOf(((Answered<String,LabelDetail>) choice).detail().orElseThrow().labels().keySet()).equals(List.copyOf(taxonomy.keySet())), "probability map retains label order");
        }
        System.out.println("Java stub interface contracts pass");
    }
}
