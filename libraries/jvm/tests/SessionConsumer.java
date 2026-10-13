import thinkthen.*;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;

/** Installed calls, zero-send rejection and blocked-provider cancellation. */
public final class SessionConsumer {
    static Inputs.RequestQuestion question(String text) { return new Inputs.RequestQuestionText().text(text); }
    static Inputs.AuthoredQuestionText wording(String text) { return new Inputs.AuthoredQuestionText(text); }
    static Inputs.RequestQuestion authored(Inputs.RequestDefinition value) { return new Inputs.RequestQuestionDefinition().value(value); }
    static Inputs.RequestInputRecords records(String... values) {
        return new Inputs.RequestInputRecords().items(Arrays.stream(values).map(text -> new Inputs.RequestItem().original(new Inputs.RequestOriginalText().text(text))).toList());
    }
    static Inputs.RequestOptions options() { return new Inputs.RequestOptions().batch(new Inputs.RequestBatch(1)); }
    static void check(boolean value, String message) { if (!value) throw new AssertionError(message); }
    static Engine.OwnedCall done(CompletableFuture<Engine.OwnedCall> future) throws Exception {
        var call = future.get(10,TimeUnit.SECONDS);
        check(call.terminal().facts().state() == Presence.State.VALUE,"final facts missing");
        check(call.terminal().failure().state() != Presence.State.VALUE,"unexpected native failure");
        return call;
    }
    static void refuse(CompletableFuture<Engine.OwnedCall> call) {
        try { call.join(); throw new AssertionError("invalid typed input admitted"); }
        catch (CompletionException error) { check(error.getCause() instanceof NativeFailure,"native admission failure");
                var failure = (NativeFailure)error.getCause(); check(!failure.retryable() && failure.facts() == null, "native admission retryability and facts"); }
    }
    static void arrived(String name) throws Exception {
        long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(10);
        while (!Files.exists(Path.of("barrier/arrived-" + name))) {
            if (System.nanoTime() > deadline) throw new AssertionError("provider did not arrive");
            Thread.sleep(5);
        }
    }
    static void waitScheduler(Engine engine, Inputs.RequestQuestion question) throws Exception {
        var request = new Inputs.Request().call(new Inputs.RequestCall().function(Inputs.RequestFunction.DECIDE).question(question)
            .input(new Inputs.RequestInputFeed().name("input")));
        try (var scheduler = Executors.newSingleThreadExecutor(); var session = engine.startSession(request)) {
            CompletableFuture.runAsync(() -> {}, scheduler).get(1, TimeUnit.SECONDS);
            var threads = java.lang.management.ManagementFactory.getThreadMXBean();
            long before = threads.getTotalStartedThreadCount();
            var waiting = session.waitReady(false);
            check(session.waitReady(false) == waiting, "pending wait was replaced");
            CompletableFuture.runAsync(() -> check(!waiting.isDone(), "native wait blocked the host scheduler"), scheduler).get(1, TimeUnit.SECONDS);
            check(session.tryRead().packet() == null, "wait consumed or invented a result");
            check(threads.getTotalStartedThreadCount() - before <= 2, "blocking wait creates a new worker each time");
            synchronized (session) {
                var cancelled = session.waitReady(false);
                check(cancelled.cancel(false), "pending wait did not cancel");
                check(session.waitReady(false) == cancelled, "cancelled wait was replaced before its native worker returned");
            }
        }
    }
    public static void main(String[] ignored) throws Exception {
        if (ignored.length == 0) { thinkthen.JsonTest.main(new String[0]); PublicResultsCases.run(); }
        if (ignored.length == 1 && ignored[0].equals("smoke")) {
            try (var engine = new Engine()) {
                var call = done(engine.decide(question(System.getenv("THINKTHEN_TEST_SMOKE_QUESTION")),
                    new Inputs.RequestInputText().text(System.getenv("THINKTHEN_TEST_SMOKE_TEXT")), new Inputs.RequestOptions()));
                var row = (Results.SessionPacketDecideRow)call.packets().stream()
                    .filter(Results.SessionPacketDecideRow.class::isInstance).findFirst().orElseThrow();
                var answer = row.value().value().value();
                check(answer instanceof Boolean, "smoke answer is not boolean");
                System.out.println("smoke: " + answer);
            }
            return;
        }
        Engine.OwnedCall retained;
        Engine.SessionFailure retainedFailure;
        try (var engine = new Engine(new Inputs.EngineSettings().cache(new Inputs.CacheDocument(false)).maxRetries(0).batch(new Inputs.RequestBatch(1)))) {
            var question = question("Is it?");
            Files.writeString(Path.of("evidence.txt"), "session-decide");
            Files.writeString(Path.of("saved.json"), "{\"decide\":\"Is it?\"}");
            var source = new Inputs.RequestInputSource().source(new Inputs.RequestSource().paths(List.of("evidence.txt")).reading(new Inputs.RequestReader().unit(Inputs.SourceUnit.FILE)));
            var decide = done(engine.decide(new Inputs.RequestQuestionFile().path("saved.json"),source,options()));
            var row = (Results.SessionPacketDecideRow)decide.packets().stream().filter(Results.SessionPacketDecideRow.class::isInstance).findFirst().orElseThrow();
            check(Boolean.TRUE.equals(row.value().value().value()),"typed decide reading");
            check(Boolean.TRUE.equals(decide.decide()), "normalized decide answer");
            try { new Engine.OwnedCall(List.of(), decide.terminal()).decide(); throw new AssertionError("empty call answered"); }
            catch (Results.CallAnswers.CardinalityFailure expected) { check(expected.call().packets().isEmpty(), "cardinality refusal retains call"); }
            var choose = done(engine.choose(authored(new Inputs.RequestDefinitionAuthoredChoose().choose(wording("Which?")).options(new Inputs.AuthoredOptions(List.of("first","second")))),records("session-choose"),options()));
            check(choose.choose() != null, "normalized choose answer");
            check(choose.packets().stream().anyMatch(Results.SessionPacketChooseRow.class::isInstance),"choose row");
            var tag = done(engine.tag(authored(new Inputs.RequestDefinitionAuthoredTag().tag(wording("Which?")).labels(new Inputs.AuthoredLabels(List.of("first","second")))),records("session-tag"),options()));
            check(!tag.tag().isEmpty(), "normalized tag answer");
            check(tag.packets().stream().anyMatch(Results.SessionPacketTagRow.class::isInstance),"tag row");
            var score = done(engine.score(authored(new Inputs.RequestDefinitionAuthoredScore().score(wording("Which?")).levels(new Inputs.AuthoredLevels(List.of("Low.","High.")))),records("session-score"),options()));
            check(score.score() != null, "normalized score answer");
            check(score.packets().stream().anyMatch(Results.SessionPacketScoreRow.class::isInstance),"score row");
            check(done(engine.filter(authored(new Inputs.RequestDefinitionAuthoredDecide().decide(wording("Is it?")).threshold(new Inputs.AuthoredThreshold(0.5))),records("session-filter"),options())).packets().stream().anyMatch(Results.SessionPacketFilterRow.class::isInstance),"filter row");
            check(done(engine.rank(authored(new Inputs.RequestDefinitionAuthoredDecide().decide(wording("Is it?"))),records("session-rank"),options())).packets().stream().anyMatch(Results.SessionPacketRankAggregate.class::isInstance),"rank aggregate");
            check(done(engine.find(authored(new Inputs.RequestDefinitionAuthoredFind().find(wording("Which?"))),records("session-find","session-find-two"),options())).packets().stream().anyMatch(Results.SessionPacketFindAggregate.class::isInstance),"find aggregate");
            check(done(engine.annotate(authored(new Inputs.RequestDefinitionAnnotateDefinition().questions(Map.of("check",new Inputs.AnnotateQuestionDecide().decide(wording("Is it?"))))),records("session-annotate"),options())).packets().stream().anyMatch(Results.SessionPacketAnnotateRow.class::isInstance),"annotate row");
            check(done(engine.recognize(authored(new Inputs.RequestDefinitionRecognizeDefinition().recognize(new Inputs.RecognizeQuestion().kinds(Map.of("person",new Inputs.AuthoredDescription("A person's name."))))),records("Maria Chen"),options())).packets().stream().anyMatch(Results.SessionPacketRecognizeAggregate.class::isInstance),"recognize aggregate");
            var entities = new Inputs.RequestInputEntities().items(List.of(new Inputs.RequestItem().original(new Inputs.RequestOriginalJson().value(Map.of("name","First","kind","alert"))),new Inputs.RequestItem().original(new Inputs.RequestOriginalJson().value(Map.of("name","Second","kind","alert")))));
            check(done(engine.relate(authored(new Inputs.RequestDefinitionAuthoredRelate().relate(new Inputs.RelateQuestion().relations(List.of(new Inputs.AuthoredRelation().name("caused_by").source("alert").target("alert"))))),entities,options())).packets().stream().anyMatch(Results.SessionPacketRelateAggregate.class::isInstance),"relate aggregate");
            var request = new Inputs.Request().call(new Inputs.RequestCall().function(Inputs.RequestFunction.DECIDE).question(question).input(records("preview")));
            check(engine.plan(request).requests().intValueExact() == 1,"typed no-send plan");
            Files.createDirectories(Path.of("home/config/thinkthen/questions"));
            Files.writeString(Path.of("home/config/thinkthen/questions/saved.json"), "{\"decide\":\"Is it?\",\"name\":\"saved\"}");
            for (var selector : List.of(new Inputs.RequestQuestionName().name("saved"), new Inputs.RequestQuestionReference().reference("@saved"))) {
                check(engine.plan(new Inputs.Request().call(new Inputs.RequestCall().function(Inputs.RequestFunction.DECIDE).question(selector).input(source))).requests().intValueExact() == 1,"typed question lookup");
            }
            var item = new Inputs.RequestItem().original(new Inputs.RequestOriginalText().text("context-preview")).context(new Inputs.ContextSchema(""));
            check(engine.plan(new Inputs.Request().call(new Inputs.RequestCall().function(Inputs.RequestFunction.DECIDE).question(question).input(new Inputs.RequestInputRecords().items(List.of(item))).options(new Inputs.RequestOptions().context("shared")))).requests().intValueExact() == 1,"typed record context");
            var shortlist = new Inputs.RequestItem().original(new Inputs.RequestOriginalText().text("shortlist-preview")).options(List.of(new Inputs.OptionSchema().name("first"),new Inputs.OptionSchema().name("second")));
            check(engine.plan(new Inputs.Request().call(new Inputs.RequestCall().function(Inputs.RequestFunction.CHOOSE).question(authored(new Inputs.RequestDefinitionAuthoredChoose().choose(wording("Which?")).options(new Inputs.AuthoredOptions(List.of("first","second"))))).input(new Inputs.RequestInputRecords().items(List.of(shortlist))))).requests().intValueExact() == 1,"typed replacement shortlist");
            var states = new Inputs.RequestOptions().contextNull();
            check(Values.object(states.json()).containsKey("context") && Values.object(states.json()).get("context") == null,"input null collapsed");
            refuse(engine.decide(question,records("null-refusal"),states));
            states.omitContext();
            check(!Values.object(states.json()).containsKey("context"),"input omission collapsed");
            var badDefinition = new Inputs.RequestDefinitionAuthoredDecide().decide(wording("Is it?")).extension("unknown_authored_member",false);
            refuse(engine.decide(authored(badDefinition),records("unknown-refusal"),options()));
            refuse(engine.decide(question,new Inputs.RequestInputText().text("image-refusal").images(List.of(new Inputs.RequestImageBytes().media(Inputs.ImageMedia.IMAGEPNG).bytes("iVBORw0KGgo="))),options()));
            refuse(engine.decide(question,new Inputs.RequestInputText().images(List.of(new Inputs.RequestImageFile().path("absent-image.png"))),options()));

            try { engine.decide(question(""),records("malformed"),options()).join(); throw new AssertionError("malformed admitted"); }
            catch (CompletionException error) { check(error.getCause() instanceof NativeFailure,"native admission failure");
                var failure = (NativeFailure)error.getCause(); check(!failure.retryable() && failure.facts() == null, "native admission retryability and facts"); }
            try { engine.decide(question,records("\ud800"),options()).join(); throw new AssertionError("invalid Unicode admitted"); }
            catch (CompletionException error) { check(error.getCause() instanceof IllegalArgumentException,"Unicode representation failure"); }
            try { engine.decide(question,records("status-401"),options()).join(); throw new AssertionError("provider failure succeeded"); }
            catch (CompletionException error) { retainedFailure = (Engine.SessionFailure)error.getCause(); }
            waitScheduler(engine, question);
            var held = engine.decide(question,records("hold-jvm-java"),options());
            arrived("hold-jvm-java");
            var feedRequest = new Inputs.Request().call(new Inputs.RequestCall().function(Inputs.RequestFunction.DECIDE).question(question).input(new Inputs.RequestInputFeed().name("input")).options(options()));
            var descriptor = new Inputs.RequestSessionDescriptor().item(new Inputs.RequestItem().original(new Inputs.RequestOriginalText().text("session-independent")));
            check(done(engine.execute(feedRequest, List.of(descriptor))).terminal().facts().value().requestsSent().intValueExact() == 1,"unrelated feed session blocked");
            long started = System.nanoTime();
            check(held.cancel(false),"future did not cancel");
            check(held.isCancelled() && System.nanoTime() - started < TimeUnit.SECONDS.toNanos(2),"cancellation waited for provider");
            check(!Files.exists(Path.of("barrier/release-hold-jvm-java")),"provider was released early");
            var sampleAnswers = examples.JavaExamples.run(engine);
            check(sampleAnswers.size() == 10, "ten shipped Java examples");
            var relation = (Results.Relation)sampleAnswers.get(9);
            check(relation.value().getFirst().sourceName().equals("First") && relation.value().getFirst().targetName().equals("Second"), "direct relation endpoint names");
            retained = decide;
            try (var session = engine.startSession(new Inputs.Request().call(new Inputs.RequestCall().function(Inputs.RequestFunction.DECIDE).question(question).input(new Inputs.RequestInputFeed().name("input"))))) {
                session.finish(new Inputs.RequestReaderFailureIo());
                Results.SessionPacketTerminal terminal = null;
                long limit = System.nanoTime() + TimeUnit.SECONDS.toNanos(5);
                while (System.nanoTime() < limit) {
                    var next = session.tryRead();
                    if (next.packet() instanceof Results.SessionPacketTerminal settled) terminal = settled;
                    if (next.ended()) break;
                    session.waitReady(false).join();
                }
                check(terminal != null && terminal.failure().state() == Presence.State.VALUE,"owned reader failure");
            }
        }
        check(retained.terminal().facts().value().records().intValueExact() == 1,"result died with engine");
        check(retainedFailure.call().terminal().facts().value().requestsSent().intValueExact() == 1,"failure facts died with engine");
        try { retainedFailure.call().decide(); throw new AssertionError("failed call answered"); }
        catch (Engine.SessionFailure expected) { check(expected.call() == retainedFailure.call(), "answer helper retains actual failed prefix"); }
        check(retainedFailure.failure().error().message() != null,"typed failure died with engine");
        var missing = new Results.Facts(Map.of());
        check(missing.model().state() == Presence.State.MISSING,"missing presence");
        try { new Results.Plan(Map.of()).firstBodyUtf8(); throw new AssertionError("missing required nullable member admitted"); }
        catch (IllegalStateException expected) { }
        var nullMap = new LinkedHashMap<String,Object>(); nullMap.put("first_body_utf8",null); nullMap.put("future",false);
        var explicitNull = new Results.Plan(nullMap); nullMap.clear();
        check(explicitNull.firstBodyUtf8().state() == Presence.State.NULL && Boolean.FALSE.equals(Values.object(explicitNull.json()).get("future")),"null/false ownership");
        System.out.println("INSTALLED_JVM_SESSION_PASS");
    }
}
