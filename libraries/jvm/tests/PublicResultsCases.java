import thinkthen.Results;
import thinkthen.Inputs;
import thinkthen.Presence;
import thinkthen.SessionCases;
import java.util.Map;
import java.util.HashMap;
/** The shared Java result contract owns presence checks for every JVM facade. */
public final class PublicResultsCases {
    private PublicResultsCases() {}
    private static void check(boolean value) { if (!value) throw new AssertionError("shared result presence"); }
    public static void run() {
        for (var method : thinkthen.NamedCalls.class.getDeclaredMethods())
            if (method.getGenericReturnType().getTypeName().contains("CallResult<?, ?"))
                check(java.lang.reflect.Modifier.isPrivate(method.getModifiers()));
        for (var type : java.util.List.<Class<?>>of(Results.Plan.class, Inputs.Request.class, Inputs.RequestQuestion.class))
            for (var method : type.getMethods()) check(!method.getName().equals("json") && !method.getName().equals("fromJson"));
        var missing = new Results.AnnotationValueDecision(Map.of("kind", "decision"));
        check(!((Map<?,?>)SessionCases.tree(missing)).containsKey("value"));
        try { missing.value(); throw new AssertionError("missing nullable member admitted"); } catch (IllegalStateException expected) { }
        var nullable = new HashMap<String,Object>(); nullable.put("kind", "decision"); nullable.put("value", null); nullable.put("future", false);
        var nil = new Results.AnnotationValueDecision(nullable);
        check(nil.value().state() == Presence.State.NULL);
        check(Boolean.FALSE.equals(((Map<?,?>)SessionCases.tree(nil)).get("future")));
        check(Boolean.FALSE.equals(new Results.AnnotationValueDecision(Map.of("kind", "decision", "value", false)).value().value()));
        var terminal = new HashMap<String,Object>(); terminal.put("kind", "terminal"); terminal.put("facts", null);
        check(new Results.SessionPacketTerminal(terminal).facts().state() == Presence.State.NULL);
        check(new Results.AtomicBoolean(Map.of()).input().state() == Presence.State.MISSING);
        var input = new HashMap<String,Object>(); input.put("input", null);
        check(new Results.AtomicBoolean(input).input().state() == Presence.State.NULL);
        check(Boolean.FALSE.equals(new Results.AtomicBoolean(Map.of("input", false)).input().value()));
    }
}
