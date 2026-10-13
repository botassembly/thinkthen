import thinkthen.Results;
import thinkthen.Inputs;
import thinkthen.Presence;
import java.util.Map;
import java.util.HashMap;
/** The shared Java result contract owns presence checks for every JVM facade. */
public final class PublicResultsCases {
    private PublicResultsCases() {}
    private static void check(boolean value) { if (!value) throw new AssertionError("shared result presence"); }
    public static void run() {
        for (var type : java.util.List.<Class<?>>of(Results.Plan.class, Inputs.Request.class, Inputs.RequestQuestion.class))
            for (var method : type.getMethods()) check(!method.getName().equals("json") && !method.getName().equals("fromJson"));
        var original = new HashMap<String,Object>(); original.put("input", null);
        var kept = new Results.SessionPacketFilterRow(Map.of("function", "filter", "kind", "row", "value", original));
        var call = new thinkthen.Engine.OwnedCall(java.util.List.of(kept), new Results.SessionPacketTerminal(Map.of("kind", "terminal")));
        check(call.filter().size() == 1 && call.filter().getFirst() == null);

        var missing = new Results.AnnotationValueDecision(Map.of("kind", "decision"));
        try { missing.value(); throw new AssertionError("missing nullable member admitted"); } catch (IllegalStateException expected) { }
        var nullable = new HashMap<String,Object>(); nullable.put("kind", "decision"); nullable.put("value", null); nullable.put("future", false);
        var nil = new Results.AnnotationValueDecision(nullable);
        check(nil.value().state() == Presence.State.NULL);
        check(Boolean.FALSE.equals(new Results.AnnotationValueDecision(Map.of("kind", "decision", "value", false)).value().value()));
        var terminal = new HashMap<String,Object>(); terminal.put("kind", "terminal"); terminal.put("facts", null);
        check(new Results.SessionPacketTerminal(terminal).facts().state() == Presence.State.NULL);
        check(new Results.AtomicBoolean(Map.of()).input().state() == Presence.State.MISSING);
        var input = new HashMap<String,Object>(); input.put("input", null);
        check(new Results.AtomicBoolean(input).input().state() == Presence.State.NULL);
        check(Boolean.FALSE.equals(new Results.AtomicBoolean(Map.of("input", false)).input().value()));
    }
}
