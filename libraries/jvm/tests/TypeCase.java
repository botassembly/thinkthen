import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import thinkthen.Door;
import thinkthen.Json;

/**
 * With one argument, args[0] is one JSON-door request and stdout its reply.
 * "fields REQUEST" reads each annotate row member through Door.field; "plan
 * INPUT" reads one thinkthen.plan-input/1 object and prints Door.plan's object;
 * "limits" checks ticket 0291's zero budgets and zero cap.
 */
public final class TypeCase {
    public static void main(String[] args) {
        if(args.length==1&&args[0].equals("native")){try(Door e=new Door(System.getenv("TT_NATIVE_SETTINGS"))){NativeChecks.run(e);}System.out.println("{\"native\":\"pass\"}");return;}
        if(args.length==2&&args[0].equals("complete")){var d=Json.parseObject(args[1]);try(Door e=new Door((String)d.get("engine_settings"));Door.Token token=e.token()){System.out.println(NativeCases.run(e,d,token));}catch(Door.NativeFailure f){System.out.println(NativeCases.failure(f));}return;}
        String mode = args.length == 2 || args[0].equals("limits") ? args[0] : "";
        try (Door door = new Door()) {
            System.out.println(switch (mode) {
                case "plan" -> plan(door, Json.parseObject(args[1]));
                case "fields" -> fields(door.call(args[1]));
                case "limits" -> limits(door);
                default -> door.call(args[0]);
            });
        } catch (Door.NativeFailure failure) {
            System.out.println("{\"failed\":{\"kind\":\"" + failure.failure.kind().name().toLowerCase() + "\",\"code\":" + failure.failure.code() + "}}");
        }
    }

    static String plan(Door door, Map<String, Object> input) {
        Object question = input.get("question");
        String asked = question instanceof String text ? text : write(question);
        String[] texts = ((List<?>) input.get("input")).toArray(String[]::new);
        String settings = input.get("settings") instanceof Map<?, ?> given ? write(given) : null;
        return write(door.plan((String) input.get("verb"), asked, texts, settings));
    }

    static String fields(String reply) {
        List<Map<String, String>> states = new ArrayList<>();
        for (Object row : (List<?>) Json.parseObject(reply).get("value")) {
            Map<String, String> state = new LinkedHashMap<>();
            ((Map<?, ?>) row).forEach((name, member) -> state.put((String) name, switch (Door.field(member)) {
                case Door.AnnotatedField.Unresolved u -> "unresolved";
                case Door.AnnotatedField.Answered a -> "answered";
                case Door.AnnotatedField.Failed f -> "failed " + f.kind() + " " + f.cause();
            }));
            states.add(state);
        }
        return write(states);
    }

    /** A zero cap and a zero budget each refuse before sending. Relate gets two entities, since one has no pair to ask. */
    static String limits(Door door) {
        try (Door capped = new Door("{\"max_requests_total\":0,\"cache\":false}")) {
            Door.Failure cap = refused(() -> capped.decide("Is it?", "capped".getBytes(StandardCharsets.UTF_8)));
            if (cap.kind() != Door.FailureKind.USAGE || cap.code() != 1 || !cap.message().contains("process send budget"))
                throw new AssertionError("cap: " + cap);
        }
        for (Runnable call : new Runnable[]{
                () -> door.call("{\"decide\":\"Is it?\",\"evidence\":\"zero-call\"}", 0, null),
                () -> door.recognize("{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A person's name.\"}}}", "zero-recognize".getBytes(StandardCharsets.UTF_8), 0, null),
                () -> door.relate("{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"caused_by\",\"source\":\"alert\",\"target\":\"alert\"}]}}",
                    new byte[][]{"{\"name\":\"A\",\"kind\":\"alert\"}".getBytes(StandardCharsets.UTF_8), "{\"name\":\"B\",\"kind\":\"alert\"}".getBytes(StandardCharsets.UTF_8)}, 0, null)}) {
            Door.Failure zero = refused(call);
            if (zero.kind() != Door.FailureKind.DEADLINE || zero.code() != 3) throw new AssertionError("zero budget: " + zero);
        }
        return "{\"limits\":\"pass\"}";
    }

    static Door.Failure refused(Runnable call) {
        try { call.run(); } catch (Door.NativeFailure failure) { return failure.failure; }
        throw new AssertionError("a limited call succeeded");
    }

    /** Write a Json value back as JSON text. */
    static String write(Object value) {
        if (value == null) return "null";
        if (value instanceof String text) return Json.quote(text);
        if (value instanceof List<?> list) return "[" + String.join(",", list.stream().map(TypeCase::write).toList()) + "]";
        if (value instanceof Map<?, ?> map) {
            List<String> members = new ArrayList<>();
            map.forEach((name, member) -> members.add(Json.quote((String) name) + ":" + write(member)));
            return "{" + String.join(",", members) + "}";
        }
        return value.toString();
    }
}
