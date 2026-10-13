import thinkthen.Results;
import thinkthen.Values;
import thinkthen.Presence;
import java.util.Map;
import java.util.HashMap;
/** The shared Java result contract owns presence checks for every JVM facade. */
public final class PublicResultsCases {
    private PublicResultsCases() {}
    private static void check(boolean value) { if (!value) throw new AssertionError("shared result presence"); }
    public static void run() {
        var missing = new Results.AnnotationValueDecision(Map.of("kind", "decision"));
        check(!Values.object(missing.json()).containsKey("value"));
        try { missing.value(); throw new AssertionError("missing nullable member admitted"); } catch (IllegalStateException expected) { }
        var nullable = new HashMap<String,Object>(); nullable.put("kind", "decision"); nullable.put("value", null); nullable.put("future", false);
        var nil = new Results.AnnotationValueDecision(nullable);
        check(nil.value().state() == Presence.State.NULL);
        check(Boolean.FALSE.equals(Values.object(nil.json()).get("future")));
        check(Boolean.FALSE.equals(new Results.AnnotationValueDecision(Map.of("kind", "decision", "value", false)).value().value()));
        var terminal = new HashMap<String,Object>(); terminal.put("kind", "terminal"); terminal.put("facts", null);
        check(new Results.SessionPacketTerminal(terminal).facts().state() == Presence.State.NULL);
        check(new Results.AtomicBoolean(Map.of()).input().state() == Presence.State.MISSING);
        var input = new HashMap<String,Object>(); input.put("input", null);
        check(new Results.AtomicBoolean(input).input().state() == Presence.State.NULL);
        check(Boolean.FALSE.equals(new Results.AtomicBoolean(Map.of("input", false)).input().value()));
    }
}
