import com.fasterxml.jackson.databind.JsonNode;
import io.thinkthen.*;
import java.nio.file.Path;

/** Shared option cases run only through typed public questions and functions. */
public final class OptionCases {
    private OptionCases() {}
    public static JsonNode execute(ThinkThen client, JsonNode request) throws Exception {
        var output = ConsumerCases.JSON.createArrayNode();
        try (var rows = ConformanceCases.execute(client, request)) {
            rows.forEach(row -> output.add(ConsumerCases.canonicalRow(row)));
            return ConsumerCases.JSON.readTree(ConsumerCases.JSON.writeValueAsBytes(output));
        } catch (ThinkThenException error) { return ConformanceCases.error(error.code().wireName()); }
    }
    // Feature: typed question options preserve canonical answers and execution names.
    public static void main(String[] args) throws Exception {
        var cases = ConsumerCases.JSON.readTree(Path.of(args[0]).toFile());
        try (var client = ThinkThen.builder().backend(Backend.STUB).build()) {
            for (var sample : cases) {
                String reason = sample.path("not_applicable").path("java").asText();
                if (!reason.isBlank()) {
                    System.out.println("Java options " + sample.get("name").asText() + ": not applicable: " + reason);
                    continue;
                }
                if (sample.has("expected_error")) {
                    var saved = ConsumerCases.JSON.createObjectNode();
                    saved.putObject("entries").putObject("case").set("question",sample.get("request").get("question"));
                    try { Toolbox.fromJson(saved.toString()); throw new AssertionError("Accepted invalid shared option"); }
                    catch (ThinkThenException error) { ConsumerCases.equal(error.code().wireName(),sample.get("expected_error").asText(),sample.get("name").asText()); }
                    continue;
                }
                var actual = execute(client, sample.get("request"));
                var expected = sample.has("expected_error") ? ConformanceCases.error(sample.get("expected_error").asText())
                    : execute(client, sample.get("equivalent_request"));
                ConsumerCases.equal(actual, expected, sample.get("name").asText());
                if (!sample.has("expected_error")) {
                    ConsumerCases.equal(actual, sample.get("expected"), "canonical option rows retain source identity");
                    ConsumerCases.equal(actual.size(), sample.get("expected").size(), "option rows");
                }
            }
        }
        System.out.println("Installed Java shared typed option combinations passed");
    }
}
