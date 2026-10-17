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
    static <T extends Engine.OwnedCall> T done(CompletableFuture<T> future) throws Exception {
        var call = future.get(10,TimeUnit.SECONDS);
        check(call.terminal().facts().state() == Presence.State.VALUE,"final facts missing");
        check(call.terminal().failure().state() != Presence.State.VALUE,"unexpected native failure");
        return call;
    }
    static void refuse(CompletableFuture<? extends Engine.OwnedCall> call) {
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
            idleWaitDoesNotPoll();
            check(session.waitReady(false) == waiting, "pending wait was replaced");
            CompletableFuture.runAsync(() -> check(!waiting.isDone(), "native wait blocked the host scheduler"), scheduler).get(1, TimeUnit.SECONDS);
            check(session.tryRead().packet() == null, "wait consumed or invented a result");
            check(threads.getTotalStartedThreadCount() - before <= 2, "blocking wait creates a new worker each time");
            synchronized (session) {
                var cancelled = session.waitReady(false);
                check(cancelled.cancel(false), "pending wait did not cancel");
                check(session.waitReady(false) == cancelled, "cancelled wait was replaced before its native worker returned");
            }
            Thread.sleep(100);
            check(session.waitReady(false) != waiting, "cancel did not wake the native readiness worker");
            session.finish();
            session.waitReady(false).get(2, TimeUnit.SECONDS);
            check(threads.getTotalStartedThreadCount() - before <= 2, "cancel wake grew worker count");
        }
        try (var session = engine.startSession(request)) {
            var ready = session.waitReady(false);
            session.finish();
            ready.get(2, TimeUnit.SECONDS);
            check(session.tryRead().packet() != null, "readiness did not wake for a result");
        }
    }
    static long workerSwitches() throws Exception {
        long total = 0;
        try (var tasks = Files.list(Path.of("/proc/self/task"))) {
            for (var task : tasks.toList()) {
                if (!Files.readString(task.resolve("comm")).startsWith("thinkthen-sessi")) continue;
                for (String line : Files.readAllLines(task.resolve("status"))) {
                    if (line.startsWith("voluntary_ctxt_switches:") || line.startsWith("nonvoluntary_ctxt_switches:"))
                        total += Long.parseLong(line.substring(line.indexOf(':') + 1).trim());
                }
            }
        }
        return total;
    }
    static void idleWaitDoesNotPoll() throws Exception {
        if (!Files.isDirectory(Path.of("/proc/self/task"))) return;
        Thread.sleep(100);
        long before = workerSwitches();
        Thread.sleep(600);
        check(workerSwitches() - before <= 3, "idle readiness worker wakes on a timer");
    }
    static void planFunctions(Engine engine) {
        var functions = Inputs.RequestFunction.values();
        var questions = List.of(examples.SampleInputs.decide(), examples.SampleInputs.choose(),
            examples.SampleInputs.tag(), examples.SampleInputs.score());
        for (var function : functions) {
            var question = switch (function) {
                case CHOOSE -> questions.get(1); case TAG -> questions.get(2); case SCORE -> questions.get(3);
                case FIND -> examples.SampleInputs.find(); case ANNOTATE -> examples.SampleInputs.annotate();
                case RECOGNIZE -> examples.SampleInputs.recognize(); case RELATE -> examples.SampleInputs.relate();
                default -> questions.getFirst();
            };
            var request = new Inputs.Request().call(new Inputs.RequestCall().function(function)
                .question(question).input(records("plan-preview")));
            switch (function) {
                case FIND, ANNOTATE, RECOGNIZE, RELATE -> {
                    try { engine.plan(request); throw new AssertionError("unsupported plan admitted: " + function); }
                    catch (NativeFailure error) { check(error.kind() == ErrorKind.USAGE && !error.retryable() && error.facts() == null, "unsupported plan refusal"); }
                }
                default -> {
                    var plan = engine.plan(request);
                    check(plan.requests().signum() > 0 && plan.records().intValueExact() == 1, "plan counts: " + function);
                    check(plan.estimatedBytes().signum() > 0 && plan.largestRequestBytes().signum() > 0, "plan byte estimates");
                    check(plan.estimatedInputTokens() != null && plan.largestRequestEstimatedInputTokens().signum() > 0, "plan token estimates");
                    check(!plan.tokenEstimateMethod().isEmpty() && plan.upperBound() != null && plan.firstBodyUtf8().value().contains("plan-preview"), "plan metadata");
                }
            }
        }
    }
    static void feederError(Engine engine, Inputs.RequestQuestion question) throws Exception {
        var failure = new AssertionError("second feed item failed");
        var request = new Inputs.Request().call(new Inputs.RequestCall().function(Inputs.RequestFunction.DECIDE)
            .question(question).input(new Inputs.RequestInputFeed().name("input"))
            .options(new Inputs.RequestOptions().batch(new Inputs.RequestBatch(100))));
        Iterable<Inputs.RequestSessionDescriptor> feed = () -> new Iterator<>() {
            int items;
            public boolean hasNext() { return true; }
            public Inputs.RequestSessionDescriptor next() {
                if (++items == 2) throw failure;
                check(items == 1, "feeder continued after its error");
                return new Inputs.RequestSessionDescriptor().item(new Inputs.RequestItem()
                    .original(new Inputs.RequestOriginalText().text("feeder-error-buffered")));
            }
        };
        var call = engine.execute(request, feed);
        try { call.get(2, TimeUnit.SECONDS); throw new AssertionError("feeder error succeeded"); }
        catch (ExecutionException error) { check(error.getCause() == failure, "feeder error was wrapped or replaced"); }
        // The installed runner's exact backend arrival count rejects any send from this feed.
    }
    public static void main(String[] ignored) throws Exception {
        if (ignored.length == 0) { thinkthen.JsonTest.main(new String[0]); PublicResultsCases.run(); }
        Results.Plan retainedPlan;
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
            check(decide.value().equals(List.of(true)), "one input source keeps list cardinality");
            check(decide.probability().equals(List.of(0.9)), "settled yes probability");
            check(decide.results().size() == 1 && decide.facts().requestsSent().intValueExact() == 1, "one result exposes receipts and facts");
            var choose = done(engine.choose(authored(new Inputs.RequestDefinitionAuthoredChoose().choose(wording("Which?")).options(new Inputs.AuthoredOptions(List.of("first","second")))),records("session-choose"),options()));
            check(choose.value() != null, "normalized choose answer");
            check(choose.packets().stream().anyMatch(Results.SessionPacketChooseRow.class::isInstance),"choose row");
            var tag = done(engine.tag(authored(new Inputs.RequestDefinitionAuthoredTag().tag(wording("Which?")).labels(new Inputs.AuthoredLabels(List.of("first","second")))),records("session-tag"),options()));
            check(!tag.value().isEmpty(), "normalized tag answer");
            check(tag.packets().stream().anyMatch(Results.SessionPacketTagRow.class::isInstance),"tag row");
            var score = done(engine.score(authored(new Inputs.RequestDefinitionAuthoredScore().score(wording("Which?")).levels(new Inputs.AuthoredLevels(List.of("Low.","High.")))),records("session-score"),options()));
            check(score.value() != null, "normalized score answer");
            check(score.packets().stream().anyMatch(Results.SessionPacketScoreRow.class::isInstance),"score row");
            check(done(engine.filter(authored(new Inputs.RequestDefinitionAuthoredDecide().decide(wording("Is it?")).threshold(new Inputs.AuthoredThreshold(0.5))),records("session-filter"),options())).value().equals(List.of("session-filter")),"filter returns original inputs");
            check(done(engine.rank(authored(new Inputs.RequestDefinitionAuthoredDecide().decide(wording("Is it?"))),records("session-rank"),options())).value().equals(List.of(java.math.BigInteger.ZERO)),"rank returns original zero-based indexes");
            check(done(engine.find(authored(new Inputs.RequestDefinitionAuthoredFind().find(wording("Which?"))),records("session-find","session-find-two"),options())).packets().stream().anyMatch(Results.SessionPacketFindAggregate.class::isInstance),"find aggregate");
            check(done(engine.annotate(authored(new Inputs.RequestDefinitionAnnotateDefinition().questions(new Inputs.Ordered<Inputs.AnnotateQuestion>().entry("check",new Inputs.AnnotateQuestionDecide().decide(wording("Is it?"))))),records("session-annotate"),options())).packets().stream().anyMatch(Results.SessionPacketAnnotateRow.class::isInstance),"annotate row");
            var recognized = done(engine.recognize(authored(new Inputs.RequestDefinitionRecognizeDefinition().recognize(new Inputs.RecognizeQuestion().kinds(new Inputs.Ordered<Inputs.AuthoredDescription>().entry("person",new Inputs.AuthoredDescription("A person's name."))))),records("Maria Chen"),options()));
            var proposals = ((Results.RecognitionOddsRecognizeAnswer)recognized.results().getFirst().answer()).proposals();
            var kept = proposals.stream().filter(Results.RecognitionProposal::kept).findFirst().orElseThrow();
            check(recognized.probability().getFirst().getFirst().equals(kept.spanProbability().doubleValue()), "recognize returns the kept span probability");
            var entities = new Inputs.RequestInputEntities().items(List.of(new Inputs.RequestItem().original(new Inputs.RequestOriginalJson().value(Map.of("name","First","kind","alert"))),new Inputs.RequestItem().original(new Inputs.RequestOriginalJson().value(Map.of("name","Second","kind","alert")))));
            var related = done(engine.relate(authored(new Inputs.RequestDefinitionAuthoredRelate().relate(new Inputs.RelateQuestion().relations(List.of(new Inputs.AuthoredRelation().name("caused_by").source("alert").target("alert"))))),entities,options()));
            check(related.probability().equals(related.value().stream().map(edge -> edge.probability().doubleValue()).toList()), "relate probabilities follow edge order");
            var request = new Inputs.Request().call(new Inputs.RequestCall().function(Inputs.RequestFunction.DECIDE).question(question).input(records("preview")));
            retainedPlan = engine.plan(request);
            check(retainedPlan.requests().intValueExact() == 1,"typed no-send plan");
            check(retainedPlan.firstBodyUtf8().value().contains("preview"),"typed plan body");
            try { engine.plan(new Inputs.Request().call(new Inputs.RequestCall().function(Inputs.RequestFunction.DECIDE).question(question("")).input(records("plan-refusal")))); throw new AssertionError("malformed plan admitted"); }
            catch (NativeFailure expected) { check(!expected.retryable() && expected.facts() == null,"native plan admission facts"); }
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
            check(((Map<?,?>)thinkthen.SessionCases.tree(states)).containsKey("context") && ((Map<?,?>)thinkthen.SessionCases.tree(states)).get("context") == null,"input null collapsed");
            refuse(engine.decide(question,records("null-refusal"),states));
            states.omitContext();
            check(!((Map<?,?>)thinkthen.SessionCases.tree(states)).containsKey("context"),"input omission collapsed");
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
            planFunctions(engine);
            feederError(engine, question);
            thinkthen.AuthoredOrderCases.javaCaller();
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
            var multiple = done(engine.decide(question, records("multiple-first", "multiple-second"), options()));
            check(multiple.value().equals(List.of(true, true)), "list answers retain input cardinality");
            check(multiple.probability().size() == 2 && multiple.results().size() == 2, "list odds and receipts align");
            var sampleAnswers = examples.JavaExamples.run(engine);
            check(sampleAnswers.size() == 10, "ten shipped Java examples");
            var relation = (List<?>)sampleAnswers.get(9);
            check(((Results.RelatedEntityEdge)relation.getFirst()).sourceName().equals("First") && ((Results.RelatedEntityEdge)relation.getFirst()).targetName().equals("Second"), "direct relation endpoint names");
            ResultValuesCases.run(engine);
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
        check(retainedPlan.records().intValueExact() == 1 && retainedPlan.firstBodyUtf8().value().contains("preview"),"plan died with native owner or engine");
        check(retained.terminal().facts().value().records().intValueExact() == 1,"result died with engine");
        check(retainedFailure.call().terminal().facts().value().requestsSent().intValueExact() == 1,"failure facts died with engine");
        check(retainedFailure.failure().error().message() != null,"typed failure died with engine");
        var missing = new Results.Facts(Map.of());
        check(missing.model().state() == Presence.State.MISSING,"missing presence");
        try { new Results.Plan(Map.of()).firstBodyUtf8(); throw new AssertionError("missing required nullable member admitted"); }
        catch (IllegalStateException expected) { }
        var nullMap = new LinkedHashMap<String,Object>(); nullMap.put("first_body_utf8",null); nullMap.put("future",false);
        var explicitNull = new Results.Plan(nullMap); nullMap.clear();
        check(explicitNull.firstBodyUtf8().state() == Presence.State.NULL && Boolean.FALSE.equals(((Map<?,?>)thinkthen.SessionCases.tree(explicitNull)).get("future")),"null/false ownership");
        System.out.println("INSTALLED_JVM_SESSION_PASS");
    }
}
