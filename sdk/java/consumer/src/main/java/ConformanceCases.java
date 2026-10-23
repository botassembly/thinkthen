import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import io.thinkthen.CallOptions;
import io.thinkthen.Cancel;
import io.thinkthen.Client;
import io.thinkthen.Row;
import io.thinkthen.ThinkThenException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.stream.Stream;

public final class ConformanceCases {
    private ConformanceCases() {}
    public static JsonNode error(String code) { return ConsumerCases.JSON.createObjectNode().put("error", code); }
    public static JsonNode expected(JsonNode sample) {
        var request = sample.get("request");
        JsonNode expected = sample.get("expected").deepCopy();
        if (request.get("function").asText().equals("filter") && expected.isArray()) {
            var selected = ConsumerCases.JSON.createArrayNode();
            var inputs = request.get("records");
            for (int i = 0; i < expected.size(); i++) {
                var row = expected.get(i);
                if (row.path("status").asText().equals("answered") && row.path("answer").asBoolean()) selected.add(inputs.get(i));
            }
            return selected;
        }
        if (expected.isArray()) {
            for (int i = 0; i < expected.size(); i++) {
                var record = request.get("records").get(i);
                if (record.isTextual()) {
                    projectOffsets(expected.get(i).path("answer"), record.asText());
                    projectOffsets(expected.get(i).path("detail"), record.asText());
                }
            }
        }
        return expected;
    }
    private static void projectOffsets(JsonNode value, String text) {
        if (value.isObject() && value.has("start") && value.has("end")) {
            ((ObjectNode) value).put("start", text.offsetByCodePoints(0, value.get("start").intValue()));
            ((ObjectNode) value).put("end", text.offsetByCodePoints(0, value.get("end").intValue()));
        }
        for (var child : value) projectOffsets(child, text);
    }
    public static JsonNode projectRow(JsonNode request, Row<?> row) throws Exception {
        var value = ConsumerCases.row(row);
        if (value.path("error").isObject()) ((ObjectNode) value.get("error")).remove("message");
        if (request.get("function").asText().equals("annotate")) {
            for (var outcome : value.path("answer")) if (outcome.path("error").isObject()) ((ObjectNode) outcome.get("error")).remove("message");
        }
        return value;
    }
    private static Stream<? extends Row<?>> rows(Client client, JsonNode request, CallOptions options) {
        var inputs = ConsumerCases.records(request);
        var value = request.get("question").get("value");
        var settings = request.path("options");
        return switch (request.get("function").asText()) {
            case "decide" -> client.decideStream(value.asText(), inputs, options);
            case "choose" -> client.chooseStream(value.asText(), inputs, ConsumerCases.strings(settings.get("options")), options);
            case "tag" -> client.tagStream(value.asText(), inputs, ConsumerCases.strings(settings.get("labels")), options);
            case "score" -> client.scoreStream(value.asText(), inputs, ConsumerCases.strings(settings.get("levels")), options);
            case "find" -> client.findStream(value.asText(), inputs, options);
            case "annotate" -> client.annotateStream(value, inputs, options);
            case "recognize" -> client.recognizeStream(inputs, ConsumerCases.strings(value), options);
            case "relate" -> client.relateStream(inputs, ConsumerCases.rules(value), options);
            case "rank" -> client.rank(value.asText(), inputs, options);
            default -> throw new AssertionError("Unknown conformance function");
        };
    }
    public static void main(String[] args) throws Exception {
        int count = 0;
        try (var paths = Files.list(Path.of(args[0]))) {
            for (var path : paths.filter(file -> file.toString().endsWith(".json")).sorted().toList()) {
                var sample = ConsumerCases.JSON.readTree(path.toFile());
                var request = sample.get("request");
                var clientSettings = sample.path("client");
                try (var server = new BackendFixture(path.toString())) {
                    JsonNode actual;
                    var output = ConsumerCases.JSON.createArrayNode();
                    try {
                        var builder = Client.builder().url(server.base()).keySource("THINKTHEN_API_KEY");
                        if (clientSettings.has("batch")) builder.batch(clientSettings.get("batch").longValue());
                        if (clientSettings.has("model")) builder.model(clientSettings.get("model").asText());
                        try (var client = builder.build()) {
                            var cancel = new Cancel();
                            if (sample.path("cancelled").asBoolean()) cancel.cancel();
                            var options = CallOptions.defaults().withCancel(cancel);
                            var values = request.path("options");
                            if (values.has("threshold")) options = options.withThreshold(values.get("threshold").asText());
                            if (values.has("on")) options = options.withOn(values.get("on").asText());
                            if (values.has("top")) options = options.withTop(values.get("top").intValue());
                            if (request.get("function").asText().equals("filter")) {
                                try (var kept = client.filter(request.get("question").get("value").asText(), ConsumerCases.records(request), options)) {
                                    kept.forEach(record -> output.add(ConsumerCases.JSON.valueToTree(record)));
                                }
                            } else {
                                try (var results = rows(client, request, options)) {
                                    var iterator = results.iterator();
                                    while (iterator.hasNext()) output.add(projectRow(request, iterator.next()));
                                }
                            }
                            actual = output;
                        }
                    } catch (ThinkThenException error) {
                        actual = error(error.code());
                    }
                    if (sample.has("expected_rows")) ConsumerCases.equal(output, sample.get("expected_rows"), "Java rows before terminal error");
                    ConsumerCases.equal(actual, expected(sample), "Java conformance " + path.getFileName());
                    ConsumerCases.equal(server.command("count"), sample.get("requests").asText(), "Java conformance request count");
                }
                count++;
            }
        }
        ConsumerCases.check(count > 0, "No conformance cases ran");
        System.out.println("Installed Java: " + count + " shared conformance cases passed");
    }
}
