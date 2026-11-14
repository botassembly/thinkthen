import dev.thinkthen.*;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Optional;
import java.util.OptionalDouble;
import java.util.OptionalInt;

/** Public answer, ordered-value and native form contracts. */
public final class JavaAnswerCases {
    private JavaAnswerCases() {}
    private static void check(boolean value, String message) {
        if (!value) throw new AssertionError(message);
    }
    private enum ControlChoice {
        FIRST;
        @Override public String toString() { return "left\tchoice\nvalue"; }
    }
    private static void printed(FunctionName function,Object value,String expected) {
        Row<Object,Object> row = new Answered<>(new RowInfo(function,1,Optional.empty(),Optional.empty()),null,value,OptionalDouble.empty(),NoDetail.INSTANCE);
        String saved=row.toJson();
        int hash=row.hashCode();
        check(row.toString().equals(expected),"fold leaves in " + function + ": " + row);
        check(row.answer()==value && row.toJson().equals(saved) && row.hashCode()==hash,"printing preserves raw answer, canonical JSON and equality in " + function);
    }
    private static void textLeaves() {
        var span=new Span("Morgan\tLee\nJr",9,22);
        var person=new Mention(span,"person\tkind");
        var company=new Mention(new Span("Example\rGroup",29,42),"company\rkind");
        printed(FunctionName.CHOOSE,ControlChoice.FIRST,"left choice value");
        printed(FunctionName.TAG,List.of("first\tlabel","last\r\nlabel"),"first label\tlast  label");
        printed(FunctionName.FIND,Optional.of(span),"Morgan Lee Jr");
        printed(FunctionName.RECOGNIZE,List.of(new Entity(person,1.0),new Entity(company,1.0)),"person kind: Morgan Lee Jr (9..22)\tcompany kind: Example Group (29..42)");
        printed(FunctionName.RELATE,List.of(new Relation(person,company,"works\tfor\nfirm",1.0,Optional.empty())),"Morgan Lee Jr --works for firm--> Example Group");
    }
    private static void nativeTextLeaves(String url,String command) throws Exception {
        String name="Morgan\tLee\nJr", kind="person\tkind", evidence="🦀 prefix " + name + " after";
        try(var client=ThinkThen.builder().backend(Backend.SYSTEMONE).url(java.net.URI.create(url)).apiKey("synthetic-loopback-key").retry(RetryPolicy.builder().maxRetries(0).build()).build()) {
            var names=client.recognize(evidence,List.of(kind));
            check(names.answer().size()==1,"one complete multiword name");
            var entity=names.answer().getFirst();
            check(names.toString().equals("person kind: Morgan Lee Jr (9..22)"),"canonical printed coordinates and folded name");
            var process=new ProcessBuilder(command,"recognize","--text",evidence,"--kind",kind);
            process.environment().put("THINKTHEN_BACKEND","systemone");
            process.environment().put("THINKTHEN_BASE_URL",url);
            process.environment().put("THINKTHEN_API_KEY","synthetic-loopback-key");
            var commandRun=process.start();
            String commandText=new String(commandRun.getInputStream().readAllBytes(),java.nio.charset.StandardCharsets.UTF_8);
            check(commandRun.waitFor()==0 && commandText.equals(names.toString()+"\n"),"printed name matches the command exactly");
            check(entity.text().equals(name) && entity.kind().equals(kind),"raw name and kind survive formatting");
            check(entity.start()==10 && entity.end()==23 && evidence.substring(entity.start(),entity.end()).equals(name),"host UTF-16 offsets still slice the source");
            var saved=new com.fasterxml.jackson.databind.ObjectMapper().readTree(names.toJson());
            check(saved.path("answer").path(0).path("start").asInt()==9 && saved.path("answer").path(0).path("end").asInt()==22,"canonical JSON offsets count characters");
            check(saved.path("answer").path(0).path("text").asText().equals(name),"canonical JSON retains raw controls");
            var form=client.annotate(AnnotationQuestions.builder().put("a\tname",DecideQuestion.of("Ready?")).put("b\r\nname",DecideQuestion.of("Ready?")).build(),"body");
            check(form.toString().equals("a name=true\tb  name=true"),"form keys fold while entry separators remain tabs");
            check(form.answer().entries().keySet().equals(java.util.Set.of("a\tname","b\r\nname")),"raw form names survive formatting");
            Row<?,?> failed;
            try { client.decide("FAIL WITNESS","body"); throw new AssertionError("scalar failure must throw"); }
            catch (FailedException failure) { failed=failure.rows().getFirst(); }
            check(failed instanceof Failed<?,?>,"native failure is a result");
            var error=((Failed<?,?>)failed).error();
            check(error.message().contains("\\t") && error.message().contains("\\n") && error.message().contains("\\r"),"native admission keeps visible escaped controls");
            check(failed.toString().equals("failed: " + error.code().wireName() + ": " + error.message()),"canonical admitted error text remains intact");
            check(new com.fasterxml.jackson.databind.ObjectMapper().readTree(failed.toJson()).path("error").path("message").asText().equals(error.message()),"failed canonical JSON remains intact");
        }
    }
    private static void formNameOrder(String command) throws Exception {
        String supplementary="q\uD801\uDC00", upperBmp="q\uFF21";
        check(supplementary.compareTo(upperBmp)<0,"witness reverses Java UTF-16 order");
        var questions=AnnotationQuestions.builder()
            .put(supplementary,DecideQuestion.of("Ready?",DecideOptions.builder().field(Pointer.of("/blank")).build()))
            .put(upperBmp,DecideQuestion.of("Ready?",DecideOptions.builder().field(Pointer.of("/missing")).build())).build();
        var original=java.util.Map.of("id","unicode-record","blank","","text","body");
        var options=AnnotateOptions.builder().name("unicode-form-run").build();
        try(var client=ThinkThen.builder().backend(Backend.STUB).build();
            var rows=client.annotateStream(questions,List.of(original,original),options)) {
            var forms=rows.toList();
            var form=forms.get(1);
            check(forms.size()==2 && form.isAnswered() && form.position()==2,"answered forms keep nontrivial positions");
            var first=form.answer().decide(upperBmp);
            var second=form.answer().decide(supplementary);
            check(first.isFailed() && second.isFailed(),"both ordered children fail independently");
            String saved=form.toJson();
            String expected=upperBmp+"="+first+"\t"+supplementary+"="+second;
            check(form.toString().equals(expected),"form prints in native UTF-8 question-name order");
            try { form.throwIfFailed(); throw new AssertionError("failed children must raise"); }
            catch(FailedException error) {
                check(error.code()==ErrorCode.RECORD_MISSING,"select first failed name in UTF-8 order");
                check(error.rows().size()==1,"one affected form counts once");
                Failed<?,?> selected=error.rows().getFirst();
                check(selected==first && selected.function()==FunctionName.DECIDE,"keep the selected ordinary failed child");
                check(selected.item()==original && selected.position()==form.position(),"selected failure keeps original object and parent position");
                check(selected.id().equals(form.id()) && selected.id().orElseThrow().equals(new JsonString("unicode-record")),"selected failure keeps parent id");
                check(selected.source().equals(form.source()) && selected.source().orElseThrow().name().orElseThrow().equals("unicode-form-run"),"selected failure keeps parent source");
                check(error.getMessage().equals(((Failed<?,?>)first).error().message()),"selected error message comes from the first failed child");
            }
            check(form.toJson().equals(saved),"ordering projections preserve raw answers and saved JSON");
            if(command!=null) {
                var json=new com.fasterxml.jackson.databind.ObjectMapper();
                var toolbox=json.createObjectNode();
                var entries=toolbox.putObject("questions");
                entries.putObject(supplementary).put("function","decide").put("question","Ready?").put("field","/blank");
                entries.putObject(upperBmp).put("function","decide").put("question","Ready?").put("field","/missing");
                var directory=java.nio.file.Files.createTempDirectory("java-form-order-");
                var definition=directory.resolve("questions.json");
                var records=directory.resolve("records.jsonl");
                java.nio.file.Files.writeString(definition,toolbox.toString());
                java.nio.file.Files.writeString(records,json.writeValueAsString(original)+"\n");
                var process=new ProcessBuilder(command,"annotate",definition.toString(),records.toString());
                process.environment().put("THINKTHEN_BACKEND","stub");
                var running=process.start();
                String printed=new String(running.getInputStream().readAllBytes(),java.nio.charset.StandardCharsets.UTF_8);
                String diagnostic=new String(running.getErrorStream().readAllBytes(),java.nio.charset.StandardCharsets.UTF_8);
                int exit=running.waitFor();
                check(exit==6 && printed.equals(original.get("id")+"\t"+expected+"\n"),"actual command preserves its record key, form order and failed exit: exit="+exit+", stdout="+printed+", stderr="+diagnostic);
            }
        }
    }
    public static void main(String[] args) throws Exception {
        if(args.length>0 && args[0].equals("--form-order")) {
            formNameOrder(args[1]);
            System.out.println("Java UTF-8 form order and failed-child contracts pass");
            return;
        }
        formNameOrder(null);
        textLeaves();
        if(args.length>0) nativeTextLeaves(args[0],args[1]);
        var answer = Row.class.getMethod("answer");
        var probability = Row.class.getMethod("probability");
        var unsureFlag = Row.class.getMethod("isUnsure");
        var info = new RowInfo(dev.thinkthen.FunctionName.DECIDE, 7L, Optional.empty(), Optional.empty());
        Row<Boolean,String> isRefund = new Answered<>(info, "original", false, OptionalDouble.of(0.1), NoDetail.INSTANCE);
        check(answer.invoke(isRefund).equals(false), "false remains a usable answer");
        check(probability.invoke(isRefund).equals(OptionalDouble.of(0.1)), "probability is available through Row");
        Row<Boolean,String> needsReview = new Unsure<>(info, "original", OptionalDouble.of(0.5), NoDetail.INSTANCE);
        check(unsureFlag.invoke(needsReview).equals(true), "unsure is directly testable");
        try { answer.invoke(needsReview); throw new AssertionError("unsure cannot become false or null"); }
        catch (java.lang.reflect.InvocationTargetException expected) {
            check(expected.getCause() instanceof NoAnswerException, "unsure uses the shared no-answer exception family");
            check(!(expected.getCause() instanceof ThinkThenException), "unsure is not an engine failure");
        }
        var code = ErrorCode.fromWire("record.missing");
        Row<Boolean,String> failed = new Failed<>(info, "original", new RowError(code, "missing record", OptionalInt.empty()));
        try { answer.invoke(failed); throw new AssertionError("failure cannot become false or null"); }
        catch (java.lang.reflect.InvocationTargetException expected) {
            check(expected.getCause() instanceof FailedException error && error.code() == code, "failure retains its engine code");
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
