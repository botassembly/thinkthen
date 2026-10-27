import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import io.thinkthen.*;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;

/** Shared consumer input and assertions, plus the packaged stub entry point. */
public final class ConsumerCases {
    public static final ObjectMapper JSON = new ObjectMapper().setDefaultPropertyInclusion(
        JsonInclude.Value.construct(JsonInclude.Include.NON_NULL, JsonInclude.Include.ALWAYS));
    private ConsumerCases() {}
    public static List<Object> records(JsonNode request) {
        var values = new ArrayList<Object>();
        if (!request.has("records")) return values;
        request.get("records").forEach(node -> values.add(node.isTextual() ? node.textValue() : JsonValue.parse(node.toString())));
        return values;
    }
    public static void equal(Object actual, Object expected, String what) {
        if (!java.util.Objects.equals(actual, expected)) throw new AssertionError(what + "\nexpected " + expected + "\nactual " + actual);
    }
    public static void check(boolean value, String what) { if (!value) throw new AssertionError(what); }
    public static JsonNode canonicalRow(Row<?,?> row) {
        var value = ConformanceCases.row(row);
        row.source().ifPresent(source -> {
            var identity = value.putObject("source").put("backend", source.backend().name().toLowerCase(java.util.Locale.ROOT))
                .put("model", source.model()).put("question_identity", source.questionIdentity());
            source.name().ifPresent(name -> identity.put("name", name));
        });
        return value;
    }
    private static JsonNode expectedRows(JsonNode rows) {
        var expected = rows.deepCopy();
        stripMessages(expected);
        return expected;
    }
    private static void stripMessages(JsonNode value) {
        if (value.isObject() && value.path("error").isObject())
            ((com.fasterxml.jackson.databind.node.ObjectNode) value.get("error")).remove("message");
        for (var child : value) stripMessages(child);
    }
    // Feature: the installed public client runs all ten canonical stub functions.
    public static void main(String[] args) throws Exception {
        var cases = JSON.readTree(Path.of(args[0]).toFile());
        int count = 0;
        try (var client = ThinkThen.builder().backend(Backend.STUB).build()) {
            for (var sample : cases) {
                var actual = JSON.createArrayNode();
                try (var rows = ConformanceCases.execute(client, sample.get("request"))) {
                    rows.forEach(row -> actual.add(canonicalRow(row)));
                }
                equal(JSON.readTree(JSON.writeValueAsBytes(actual)), expectedRows(sample.get("expected")), "canonical stub rows");
                count++;
            }
        }
        equal(count, 10, "Java stub case count");
        System.out.println("Packaged Java: ten typed public stub cases passed without a service");
    }
}
