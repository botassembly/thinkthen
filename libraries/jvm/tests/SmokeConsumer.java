import thinkthen.*;
import java.util.concurrent.TimeUnit;

/** Replay one cached decision through the installed package. */
public final class SmokeConsumer {
    public static void main(String[] ignored) throws Exception {
        try (var engine = new Engine()) {
            var call = engine.decide(new Inputs.RequestQuestionText().text(System.getenv("THINKTHEN_TEST_SMOKE_QUESTION")),
                new Inputs.RequestInputText().text(System.getenv("THINKTHEN_TEST_SMOKE_TEXT")),
                new Inputs.RequestOptions()).get(10, TimeUnit.SECONDS);
            if (call.terminal().facts().state() != Presence.State.VALUE) throw new AssertionError("final facts missing");
            if (call.terminal().failure().state() == Presence.State.VALUE) throw new AssertionError("unexpected native failure");
            var row = (Results.SessionPacketDecideRow)call.packets().stream()
                .filter(Results.SessionPacketDecideRow.class::isInstance).findFirst().orElseThrow();
            var answer = row.value().value().value();
            if (!(answer instanceof Boolean)) throw new AssertionError("smoke answer is not boolean");
            System.out.println("smoke: " + answer);
        }
    }
}
