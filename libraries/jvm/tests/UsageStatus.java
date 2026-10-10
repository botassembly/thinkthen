import thinkthen.*;
import java.nio.file.*;
import java.util.concurrent.TimeUnit;

public class UsageStatus {
    static void check(boolean value) { if (!value) throw new AssertionError("usage status behavior"); }
    public static void main(String[] args) throws Exception {
        var engine = new Engine(new Inputs.EngineSettings().cache(new Inputs.CacheDocument(false)).maxRetries(0));
        if (args[0].equals("disabled")) {
            check(engine.usagePersistence().state() == UsagePersistence.State.Disabled);
            check(engine.finishUsageStatus().state() == UsagePersistence.State.Disabled);
            engine.close(); return;
        }
        check(engine.usagePersistence().state() == UsagePersistence.State.Written);
        var call = engine.decide(new Inputs.RequestQuestionText().text("Is it?"), new Inputs.RequestInputText().text("usage-java-" + args[0])).get(10, TimeUnit.SECONDS);
        check(call.terminal().facts().value().requestsSent().intValueExact() == 1);
        check(engine.usagePersistence().state() == UsagePersistence.State.Pending);
        Files.writeString(Path.of(args[1], "pending"), "");
        if (args[0].equals("written")) while (!Files.exists(Path.of(args[1], "released"))) Thread.sleep(5);
        var status = engine.finishUsageStatus();
        check(status.state() == (args[0].equals("written") ? UsagePersistence.State.Written : UsagePersistence.State.Failed));
        check(engine.usagePersistence().equals(status));
        engine.close();
        check(call.packets().stream().filter(Results.SessionPacketDecideRow.class::isInstance).map(Results.SessionPacketDecideRow.class::cast).anyMatch(row -> Boolean.TRUE.equals(row.value().value().value())));
        check(call.terminal().facts().value().requestsSent().intValueExact() == 1);
        check(args[0].equals("written") ? status.advice() == null : status.advice().equals("check the usage folder permissions and free space"));
        try { engine.usagePersistence(); throw new AssertionError("closed engine observed"); } catch (IllegalStateException expected) { }
        try { engine.finishUsageStatus(); throw new AssertionError("closed engine finalized"); } catch (IllegalStateException expected) { }
        System.out.println("JAVA_USAGE_PASS");
    }
}
