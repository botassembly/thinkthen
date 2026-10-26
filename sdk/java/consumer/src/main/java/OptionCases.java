import com.fasterxml.jackson.databind.JsonNode;
import io.thinkthen.internal.CallOptions;
import io.thinkthen.internal.Client;
import io.thinkthen.internal.Question;
import io.thinkthen.internal.Row;
import io.thinkthen.internal.ThinkThenException;
import java.nio.file.Path;
import java.util.List;

public final class OptionCases {
    private OptionCases() {}
    public static Question question(JsonNode request) {
        if (request.hasNonNull("name")) throw new AssertionError("Explicit execution names await public API support; use the unnamed construction cases");
        var toolbox = ConsumerCases.JSON.createObjectNode();
        toolbox.set("entry", request.get("question"));
        return Question.toolbox(toolbox, "entry");
    }
    public static List<JsonNode> supportedCases(JsonNode cases) {
        var selected = new java.util.ArrayList<JsonNode>();
        var unsupported = new java.util.ArrayList<String>();
        for (var sample : cases) {
            if (!sample.path("request").hasNonNull("name")) {
                selected.add(sample);
                continue;
            }
            String name = sample.path("name").asText();
            String replacement = name.replaceFirst("^named-", "unnamed-");
            JsonNode variant = null;
            for (var candidate : cases) if (candidate.path("name").asText().equals(replacement)) variant = candidate;
            ConsumerCases.check(variant != null && !replacement.equals(name), "missing unnamed construction variant for " + name);
            var request = sample.get("request").deepCopy();
            ((com.fasterxml.jackson.databind.node.ObjectNode) request).remove("name");
            ConsumerCases.equal(variant.get("request"), request, "unnamed variant preserves the complete question and records");
            unsupported.add(name);
        }
        if (!unsupported.isEmpty()) System.out.println("Explicit execution-name coverage awaits public API support: " + unsupported);
        return selected;
    }
    public static JsonNode execute(Client client, JsonNode request, boolean single) throws Exception {
        var question = question(request);
        var input = ConsumerCases.records(request);
        var function = request.get("question").get("function").asText();
        var options = CallOptions.defaults();
        var output = ConsumerCases.JSON.createArrayNode();
        try {
            if (single && !function.equals("rank") && !input.isEmpty()) {
                for (var record : input) {
                    Row<?> row = switch (function) {
                        case "decide", "filter" -> client.decide(question, record, options);
                        case "choose" -> client.choose(question, record, options);
                        case "tag" -> client.tag(question, record, options);
                        case "score" -> client.score(question, record, options);
                        case "find" -> client.find(question, record, options);
                        case "recognize" -> client.recognize(question, record, options);
                        case "relate" -> client.relate(question, record, options);
                        case "annotate" -> client.annotate(question, record, options);
                        default -> throw new AssertionError(function);
                    };
                    output.add(ConsumerCases.canonicalRow(row));
                }
            } else {
                try (var rows = ConsumerCases.execute(client, request)) {
                    for (var row : rows.toList()) output.add(ConsumerCases.canonicalRow(row));
                }
            }
            return output;
        } catch (ThinkThenException error) { return ConformanceCases.error(error.code()); }
    }
    public static void main(String[] args) throws Exception {
        var cases = ConsumerCases.JSON.readTree(Path.of(args[0]).toFile());
        try (var client = Client.builder().adapter("stub").build()) {
            for (var sample : supportedCases(cases)) for (boolean single : List.of(false, true)) {
                var actual = execute(client, sample.get("request"), single);
                var expected = sample.has("expected_error") ? ConformanceCases.error(sample.get("expected_error").asText())
                    : execute(client, sample.get("equivalent_request"), single);
                ConsumerCases.equal(actual, expected, sample.get("name").asText());
                if (!sample.has("expected_error") && (!single || sample.path("request").path("question").path("function").asText().equals("rank")))
                    ConsumerCases.equal(actual, sample.get("expected"), "canonical option rows retain source identity");
                if (!sample.has("expected_error")) ConsumerCases.equal(actual.size(), sample.get("expected").size(), "option rows");
            }
        }
        System.out.println("Installed Java shared option combinations passed");
    }
}
