import java.math.BigDecimal;
import java.util.List;
import java.util.Map;
import thinkthen.Door;
import thinkthen.Json;

/** The door's JSON reader is tolerant, and Door.field tells null from a failure (ADR 0112 section 4). */
public final class JsonTest {
    private static void rejects(Runnable read, String label) {
        try { read.run(); }
        catch (IllegalStateException | IllegalArgumentException expected) { return; }
        throw new AssertionError("accepted " + label);
    }
    public static void main(String[] args) {
        Map<String, Object> facts = Json.parseObject("{\"records\":1,\"requests_sent\":1,\"cache_answers\":0,\"seconds\":1e-6,\"model\":\"a\\\"b\",\"later\":{\"x\":[1]}}");
        if (!new BigDecimal("1").equals(facts.get("records")) || !"a\"b".equals(facts.get("model")) || !facts.containsKey("later"))
            throw new AssertionError("facts with an unknown member: " + facts);
        Object envelope = Json.parse("{ \"facts\" : {}, \"value\" : {\"items\":[\"a,b\",null]} }");
        if (!List.of("facts", "value").equals(List.copyOf(((Map<?, ?>) envelope).keySet())))
            throw new AssertionError("member order");
        if (!"\"a\\u0000\\\\\\\"\"".equals(Json.quote("a\0\\\""))) throw new AssertionError("quote " + Json.quote("a\0\\\""));
        rejects(() -> Json.parse("{\"value\":1,\"value\":2}"), "duplicate member");
        rejects(() -> Json.parse("{\"value\":1} x"), "trailing text");
        rejects(() -> Json.parseObject("[1]"), "array as object");
        if (!(Door.field(null) instanceof Door.AnnotatedField.Unresolved)) throw new AssertionError("null is unresolved");
        for (Object answer : new Object[]{Boolean.TRUE, "billing", List.of("billing", "urgent"), new BigDecimal("1.2")})
            if (!new Door.AnnotatedField.Answered(answer).equals(Door.field(answer))) throw new AssertionError("answered " + answer);
        Object failure = Json.parse("{\"failed\":{\"kind\":\"backend\",\"cause\":\"missing_probability\",\"later\":1}}");
        if (!new Door.AnnotatedField.Failed("backend", "missing_probability").equals(Door.field(failure)))
            throw new AssertionError("failure with an unknown member");
        rejects(() -> Door.field(Json.parse("{\"failed\":null}")), "null failure");
        rejects(() -> Door.field(Json.parse("{\"team\":\"billing\"}")), "object answer");
        System.out.println("JSON_READER_AND_FIELD_PASS");
    }
}
