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
    static void arrived(String name) throws Exception {
        long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(10);
        while (!Files.exists(Path.of("barrier/arrived-" + name))) {
            if (System.nanoTime() > deadline) throw new AssertionError("provider did not arrive");
            Thread.sleep(5);
        }
    }
    public static void main(String[] ignored) throws Exception {
        Engine.OwnedCall retained;
        Engine.SessionFailure retainedFailure;
        try (var engine = new Engine(new Inputs.EngineSettings().cache(new Inputs.CacheDocument(false)).maxRetries(0).batch(new Inputs.RequestBatch(1)))) {
            var question = question("Is it?");
            var decide = done(engine.decide(question,records("session-decide"),options()));
            var row = (Results.SessionPacketDecideRow)decide.packets().stream().filter(Results.SessionPacketDecideRow.class::isInstance).findFirst().orElseThrow();
            check(Boolean.TRUE.equals(row.value().value().value()),"typed decide reading");
            var choose = done(engine.choose(authored(new Inputs.RequestDefinitionFieldsChoose().choose(wording("Which?")).options(new Inputs.AuthoredOptions(List.of("first","second")))),records("session-choose"),options()));
            check(choose.packets().stream().anyMatch(Results.SessionPacketChooseRow.class::isInstance),"choose row");
            var tag = done(engine.tag(authored(new Inputs.RequestDefinitionFieldsTag().tag(wording("Which?")).labels(new Inputs.AuthoredLabels(List.of("first","second")))),records("session-tag"),options()));
            check(tag.packets().stream().anyMatch(Results.SessionPacketTagRow.class::isInstance),"tag row");
            var score = done(engine.score(authored(new Inputs.RequestDefinitionFieldsScore().score(wording("Which?")).levels(new Inputs.AuthoredLevels(List.of("Low.","High.")))),records("session-score"),options()));
            check(score.packets().stream().anyMatch(Results.SessionPacketScoreRow.class::isInstance),"score row");
            check(done(engine.filter(authored(new Inputs.RequestDefinitionFieldsDecide().decide(wording("Is it?")).threshold(new Inputs.AuthoredThreshold(0.5))),records("session-filter"),options())).packets().stream().anyMatch(Results.SessionPacketFilterRow.class::isInstance),"filter row");
            check(done(engine.rank(authored(new Inputs.RequestDefinitionFieldsDecide().decide(wording("Is it?"))),records("session-rank"),options())).packets().stream().anyMatch(Results.SessionPacketRankAggregate.class::isInstance),"rank aggregate");
            check(done(engine.find(authored(new Inputs.RequestDefinitionFieldsFind().find(wording("Which?"))),records("session-find","session-find-two"),options())).packets().stream().anyMatch(Results.SessionPacketFindAggregate.class::isInstance),"find aggregate");
            check(done(engine.annotate(authored(new Inputs.RequestDefinitionFieldsQuestionsVersion().questions(Map.of("check",new Inputs.RequestDefinitionAnyOf7PropertiesQuestionsAdditionalPropertiesFieldsDecide().decide(wording("Is it?"))))),records("session-annotate"),options())).packets().stream().anyMatch(Results.SessionPacketAnnotateRow.class::isInstance),"annotate row");
            check(done(engine.recognize(authored(new Inputs.RequestDefinitionFieldsRecognizeVersion().recognize(new Inputs.RequestDefinitionFieldsRecognizeVersionPropertiesRecognize().kinds(Map.of("person",new Inputs.AuthoredDescription("A person's name."))))),records("Maria Chen"),options())).packets().stream().anyMatch(Results.SessionPacketRecognizeAggregate.class::isInstance),"recognize aggregate");
            var entities = new Inputs.RequestInputEntities().items(List.of(new Inputs.RequestItem().original(new Inputs.RequestOriginalJson().value(Map.of("name","First","kind","alert"))),new Inputs.RequestItem().original(new Inputs.RequestOriginalJson().value(Map.of("name","Second","kind","alert")))));
            check(done(engine.relate(authored(new Inputs.RequestDefinitionFieldsRelateVersion().relate(new Inputs.RequestDefinitionFieldsRelateVersionPropertiesRelate().relations(List.of(new Inputs.AuthoredRelation().name("caused_by").source("alert").target("alert"))))),entities,options())).packets().stream().anyMatch(Results.SessionPacketRelateAggregate.class::isInstance),"relate aggregate");
            var request = new Inputs.Request().call(new Inputs.RequestCallDecide().question(question).input(records("preview")));
            check(engine.plan(request).requests().intValueExact() == 1,"typed no-send plan");
            try { engine.decide(question(""),records("malformed"),options()).join(); throw new AssertionError("malformed admitted"); }
            catch (CompletionException error) { check(error.getCause() instanceof NativeFailure,"native admission failure"); }
            try { engine.decide(question,records("\ud800"),options()).join(); throw new AssertionError("invalid Unicode admitted"); }
            catch (CompletionException error) { check(error.getCause() instanceof IllegalArgumentException,"Unicode representation failure"); }
            try { engine.decide(question,records("status-401"),options()).join(); throw new AssertionError("provider failure succeeded"); }
            catch (CompletionException error) { retainedFailure = (Engine.SessionFailure)error.getCause(); }
            var held = engine.decide(question,records("hold-jvm-java"),options());
            arrived("hold-jvm-java");
            check(done(engine.decide(question,records("session-independent"),options())).terminal().facts().value().requestsSent().intValueExact() == 1,"unrelated session blocked");
            long started = System.nanoTime();
            check(held.cancel(false),"future did not cancel");
            check(held.isCancelled() && System.nanoTime() - started < TimeUnit.SECONDS.toNanos(2),"cancellation waited for provider");
            check(!Files.exists(Path.of("barrier/release-hold-jvm-java")),"provider was released early");
            retained = decide;
            try (var session = engine.startSession(new Inputs.Request().call(new Inputs.RequestCallDecide().question(question).input(new Inputs.RequestInputFeed().name("input"))))) {
                session.finish(new Inputs.RequestReaderFailureIo());
                Results.SessionPacketTerminal terminal = null;
                long limit = System.nanoTime() + TimeUnit.SECONDS.toNanos(5);
                while (System.nanoTime() < limit) {
                    var next = session.tryRead();
                    if (next.packet() instanceof Results.SessionPacketTerminal settled) terminal = settled;
                    if (next.ended()) break;
                    Thread.sleep(5);
                }
                check(terminal != null && terminal.failure().state() == Presence.State.VALUE,"owned reader failure");
            }
        }
        check(retained.terminal().facts().value().records().intValueExact() == 1,"result died with engine");
        check(retainedFailure.call().terminal().facts().value().requestsSent().intValueExact() == 1,"failure facts died with engine");
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
