package thinkthen;
import java.math.BigDecimal;
import java.util.List;
import java.util.Map;

/** JSON interchange retains exact numbers and rejects ambiguous input. */
public final class JsonTest {
    private static void rejects(Runnable read, String label) {
        try { read.run(); }
        catch (IllegalStateException | IllegalArgumentException expected) { return; }
        throw new AssertionError("accepted " + label);
    }
    public static void main(String[] args) {
        Map<String, Object> facts = FixtureJson.parseObject("{\"records\":1,\"requests_sent\":1,\"cache_answers\":0,\"seconds\":1e-6,\"model\":\"a\\\"b\",\"later\":{\"x\":[1]}}");
        if (!new BigDecimal("1").equals(facts.get("records")) || !"a\"b".equals(facts.get("model")) || !facts.containsKey("later"))
            throw new AssertionError("facts with an unknown member: " + facts);
        Object envelope = FixtureJson.parse("{ \"facts\" : {}, \"value\" : {\"items\":[\"a,b\",null]} }");
        if (!List.of("facts", "value").equals(List.copyOf(((Map<?, ?>) envelope).keySet())))
            throw new AssertionError("member order");
        if (!"\"a\\u0000\\\\\\\"\"".equals(Json.quote("a\0\\\""))) throw new AssertionError("quote " + Json.quote("a\0\\\""));
        for(char control=0;control<32;control++) if(!Json.quote(String.valueOf(control)).equals("\""+String.format("\\u%04x",(int)control)+"\"")) throw new AssertionError("control escape "+(int)control);
        if(!"\"café😀\"".equals(Json.quote("café😀"))) throw new AssertionError("Unicode quote");
        rejects(() -> FixtureJson.parse("{\"value\":1,\"value\":2}"), "duplicate member");
        rejects(() -> FixtureJson.parse("{\"value\":1} x"), "trailing text");
        rejects(() -> FixtureJson.parseObject("[1]"), "array as object");
        System.out.println("JSON_READER_PASS");
    }
}
