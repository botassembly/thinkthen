import dev.thinkthen.*;
import java.util.*;

/** Host-specific result identity, equality, exception and coordinate contracts. */
public final class JavaIdealCases {
    private JavaIdealCases() {}
    private enum Topic { BILLING, SHIPPING, OTHER; @Override public String toString() { return name().toLowerCase(Locale.ROOT); } }
    private static void check(boolean value,String claim) { if (!value) throw new AssertionError(claim); }
    private static void failedFuture(java.util.concurrent.CompletableFuture<? extends Row<?,?>> future,ErrorCode code,Object original) {
        try { future.join(); throw new AssertionError("async scalar failure must complete exceptionally"); }
        catch (java.util.concurrent.CompletionException failure) {
            check(failure.getCause() instanceof FailedException,"async completion keeps the original failure type");
            var cause=(FailedException)failure.getCause();
            check(cause.code()==code,"async failure keeps its operation's exact diagnostic");
            check(cause.rows().size()==1 && cause.rows().getFirst().item()==original,"async failure keeps the failed parent and original ownership");
            check(NoAnswerException.class.isInstance(cause) && !ThinkThenException.class.isInstance(cause),"failure keeps the Java no-answer hierarchy");
        }
        check(future.isDone() && future.isCompletedExceptionally(),"failed future reaches exceptional completion");
    }
    public static void main(String[] args) throws Exception {
        String scenario=args[0];
        if (scenario.equals("failed")) {
            try (var client=ThinkThen.builder().retry(RetryPolicy.builder().maxRetries(0).build()).build()) {
                var original=Map.of("id","failed-original","text","body");
                var questions=AnnotationQuestions.builder().put("refund",DecideQuestion.of("Refund?")).build();
                List<java.util.function.Supplier<? extends Row<?,?>>> calls=List.of(
                    () -> client.decide("Ready?",original),
                    () -> client.choose("Topic?",original,Topic.class),
                    () -> client.tag("Topics?",original,Topic.class),
                    () -> client.score("Urgency?",original,List.of("routine","urgent")),
                    () -> client.find("Which line?",original),
                    () -> client.recognize(original,List.of("person")),
                    () -> client.relate(original,List.of(RelationRule.of("person","organization","works_for"))));
                for (var call:calls) {
                    try { call.get(); throw new AssertionError("scalar failure must throw at the call"); }
                    catch (FailedException failure) {
                        check(failure.code()==ErrorCode.BACKEND_STATUS && failure.httpStatus().orElseThrow()==503,"scalar failure preserves code and HTTP status");
                        check(failure.rows().size()==1,"scalar exception retains one actual failed parent");
                        var row=failure.rows().getFirst();
                        check(row.item()==original && row.id().orElseThrow().equals(new JsonString("failed-original")),"scalar failure keeps original identity and id");
                        check(row.position()==1 && row.isFailed(),"scalar failure keeps the shared failed row");
                    }
                }
                var form=client.annotate(questions,original);
                check(form.isAnswered() && form.answer().entries().values().stream().allMatch(Row::isFailed),"answered parent with every failed child returns from scalar annotate");
                var asyncForm=client.async().annotate(questions,original).join();
                check(asyncForm.isAnswered() && asyncForm.answer().entries().values().stream().allMatch(Row::isFailed),"async annotate preserves answered parent with every failed child");
                try { form.throwIfFailed(); throw new AssertionError("explicit form check must reject failed children"); }
                catch (FailedException failure) { check(failure.rows().size()==1 && failure.rows().getFirst().function()==FunctionName.DECIDE,"explicit check exposes the actual failed child"); }
                try (var rows=client.decideStream("Ready?",List.of(original))) {
                    var row=rows.toList().getFirst();
                    check(row.isFailed() && row.item()==original,"one-item bulk call retains its failed row");
                }
                failedFuture(client.async().decide("Ready?",original),ErrorCode.BACKEND_STATUS,original);
                failedFuture(client.async().annotate(questions,""),ErrorCode.RECORD_EMPTY,"");
            }
            System.out.println("Java ideal host contracts passed: "+scenario);
            return;
        }
        Row<Optional<Span>,String> retainedSpan=null;
        Row<Annotations,Map<String,Object>> retainedForm=null;
        try (var client=ThinkThen.builder().retry(RetryPolicy.builder().maxRetries(0).build()).build()) {
            var input=new LinkedHashMap<String,Object>(); input.put("text","Please refund my order.");
            Row<Boolean,LinkedHashMap<String,Object>> first;
            try (var rows=client.decideStream("Ready?",List.of(input))) { first=rows.findFirst().orElseThrow(); }
            if (scenario.equals("unsure")) {
                var band=DecideOptions.builder().threshold(Threshold.band(.2,.8)).build();
                var unsure=client.decide("Ready?",input,band);
                var asyncUnsure=client.async().decide("Ready?",input,band).join();
                check(unsure.isUnsure() && !unsure.isFailed() && asyncUnsure.isUnsure(),"scalar and async calls return unsure successfully");
                try { unsure.answer(); throw new AssertionError("unsure remains a loud answer read"); }
                catch (UnsureException failure) { check(failure.row()==unsure,"unsure read retains its successful returned row"); }
            }
            Row<Boolean,LinkedHashMap<String,Object>> other;
            try (var rows=client.decideStream("Ready?",List.of(new LinkedHashMap<>(input)))) { other=rows.findFirst().orElseThrow(); }
            check(first.item()==input,"the shared result retains exact original identity");
            check(first.equals(other) && first.hashCode()==other.hashCode(),"equal own fields ignore distinct original objects");
            var set=new HashSet<Row<?,?>>(); set.add(first); input.put("changed",new ArrayList<>(List.of("mutable")));
            check(set.contains(first) && first.equals(other),"mutating the original cannot change result equality or hashing");
            check(!first.equals(Boolean.TRUE),"a result never equals a bare answer");
            try (var rows=client.decideStream("Different?",List.of(input))) {
                check(!first.equals(rows.findFirst().orElseThrow()),"result source fields participate in equality");
            }
            Row<Topic,String> topic;
            try (var rows=client.chooseStream("Topic?",List.of("text"),Topic.class)) { topic=rows.findFirst().orElseThrow(); }
            check(!topic.equals(Topic.BILLING),"an enum result never equals its bare member");
            if (topic.isAnswered()) check(!topic.equals(client.choose("Topic?","text",List.of("billing","shipping","other"))),"enum and string answers retain distinct Java value types");
            check(first.detail() instanceof NoDetail,"absent detail is a safe sealed value");
            String printed=first.toString(); check(("result="+first).equals("result="+printed),"concatenation is the same safe formatting");
            var questions=AnnotationQuestions.builder().put("refund",DecideQuestion.of("Refund?",DecideOptions.builder().threshold(Threshold.band(.2,.8)).build()))
                .put("topic",ChooseQuestion.of("Topic?",Topic.class))
                .put("urgency",ScoreQuestion.of("Urgency?",List.of("routine","urgent"))).build();
            var form=client.annotate(questions,"text");
            if (scenario.endsWith("-failed") || scenario.equals("failed")) {
                try { form.throwIfFailed(); throw new AssertionError("failed parent or child must throw"); }
                catch (FailedException expected) { check(expected.rows().size()==1,"failed form counts one affected result"); }
            } else check(form.throwIfFailed()==form,"answered or unsure children return the same form unchanged");
            if (form.isAnswered()) {
                var equal=client.annotate(questions,new String("text"));
                check(form.equals(equal) && form.hashCode()==equal.hashCode(),"forms compare their own nested result fields");
                for (var entry:form.answer().entries().values()) {
                    check(entry instanceof Row<?,?>,"form entries are ordinary results");
                    if (entry.isUnsure()) {
                        try { entry.answer(); throw new AssertionError("unsure child has no answer"); }
                        catch (NoAnswerException expected) { check(expected instanceof UnsureException,"uncertainty uses no-answer parent"); }
                    }
                    if (entry.isFailed()) {
                        try { entry.answer(); throw new AssertionError("failed child has no answer"); }
                        catch (NoAnswerException expected) { check(expected instanceof FailedException,"record failure uses no-answer parent"); }
                    }
                }
            }
            if (scenario.equals("relations")) {
                var toolbox=Toolbox.builder().put("choice",ChooseQuestion.of("Topic?",Topic.class))
                    .put("labels",TagQuestion.of("Topics?",Topic.class)).build();
                String savedChoice=client.choose(toolbox.choose("choice"),"text").answer();
                List<String> savedLabels=client.tag(toolbox.tag("labels"),"text").answer();
                check(savedChoice.equals("billing") && savedLabels.getFirst().equals("billing"),"toolbox string selectors retain their declared answer types for enum-built questions");
                check(first.toString().equals("true") && topic.toString().equals("billing"),"decide and enum choose print command words");
                check(client.tag("Topics?","text",Topic.class).toString().equals("billing\tshipping\tother"),"tag prints labels without container syntax");
                check(client.score("Urgency?","text",List.of("routine","urgent")).toString().equals("1"),"score prints the command number");
                try (var ranked=client.rank("Urgency?",List.of("text"),List.of("routine","urgent"))) {
                    check(ranked.findFirst().orElseThrow().toString().equals("1"),"rank prints its rank without the original item");
                }
                String message="Morgan works for Example Group.\nPlease refund my order.";
                check(client.find("Which line?",message).toString().equals("Please refund my order."),"find prints only the matching line");
                check(client.recognize(message,List.of("person","organization")).toString().equals("person: Morgan (0..6)\torganization: Example Group (17..30)"),"recognize prints command names and offsets");
                check(client.relate(message,List.of(RelationRule.of("person","organization","works_for"))).toString().equals("Morgan --works_for--> Example Group"),"relate prints command links");
                check(form.toString().equals("refund=true\ttopic=billing\turgency=1"),"a form prints named entries in command order");
            }
            if (scenario.equals("yes")) {
                String text="é 🦀\nPlease refund my order.";
                var found=client.find("Which line?",text);
                retainedSpan=found;
                var span=found.answer().orElseThrow();
                check(span.start()==5 && span.end()==text.length(),"live offsets slice Java UTF-16 strings");
                check(text.substring(span.start(),span.end()).equals(span.text()),"the documented slice returns exactly the answer");
                var saved=new com.fasterxml.jackson.databind.ObjectMapper().readTree(found.toJson());
                check(saved.path("answer").path("start").asInt()==4,"saved offsets count characters");
                check(!saved.has("item"),"saved library rows omit the original record");
                var original=new LinkedHashMap<String,Object>(); original.put("action",text); original.put("names","🦀 Morgan works for Example Group.");
                var fields=AnnotationQuestions.builder().put("action",FindQuestion.of("Which line?",FindOptions.builder().field(Pointer.of("/action")).build()))
                    .put("names",RecognizeQuestion.of(List.of("person","organization"),RecognizeOptions.builder().field(Pointer.of("/names")).build())).build();
                Row<Annotations,Map<String,Object>> retained=client.annotate(fields,original);
                retainedForm=retained;
                var forms=new HashSet<Row<?,?>>(); forms.add(retained);
                int formHash=retained.hashCode();
                original.put("action","changed"); original.put("names","changed");
                check(forms.contains(retained) && retained.hashCode()==formHash,"nested form hashing excludes its mutable original");
                var nested=retained.answer().find("action").answer().orElseThrow();
                check(nested.start()==5 && retained.answer().find("action").toJson().contains("\"start\":4"),"detached entries preserve both coordinates after the original changes");
                check(retained.toJson().contains("\"start\":4"),"nested saved forms use canonical coordinates");
            }
        }
        if (retainedSpan != null) check(retainedSpan.answer().orElseThrow().start()==5 && retainedSpan.toJson().contains("\"start\":4"),"span and canonical coordinates survive client close");
        if (retainedForm != null) check(retainedForm.answer().find("action").toJson().contains("\"start\":4"),"detached form entries survive client close");
        System.out.println("Java ideal host contracts passed: "+scenario);
    }
}
