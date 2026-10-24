import com.fasterxml.jackson.databind.JsonNode;
import io.thinkthen.CallOptions;
import io.thinkthen.Client;
import io.thinkthen.Question;
import io.thinkthen.Row;
import io.thinkthen.ThinkThenException;
import java.nio.file.Path;
import java.util.List;
import java.util.stream.Stream;

public final class OptionCases {
    private OptionCases() {}
    public static Question question(JsonNode request) {
        var value = request.get("question");
        return value.get("type").asText().equals("toolbox")
            ? Question.toolbox(value.get("value"), value.get("name").asText())
            : Question.inline(value.get("value").asText());
    }
    public static JsonNode execute(Client client, JsonNode request, boolean single) throws Exception {
        var options = CallOptions.defaults();
        var values = request.path("options");
        if (values.has("threshold")) options = options.withThreshold(values.get("threshold").asText());
        if (values.has("levels") && request.get("function").asText().equals("rank"))
            options = options.withLevels(ConsumerCases.strings(values.get("levels")));
        var input = ConsumerCases.records(request);
        var question = question(request);
        var function = request.get("function").asText();
        var output = ConsumerCases.JSON.createArrayNode();
        boolean saved = request.get("question").get("type").asText().equals("toolbox");
        try {
            if (single && !function.equals("rank")) {
                for (var record : input) {
                    Row<?> row = switch (function) {
                        case "choose" -> saved ? client.choose(question, record, options) : client.choose(request.get("question").get("value").asText(), record, ConsumerCases.strings(values.get("options")), options);
                        case "tag" -> saved ? client.tag(question, record, options) : client.tag(request.get("question").get("value").asText(), record, ConsumerCases.strings(values.get("labels")), options);
                        case "score" -> saved ? client.score(question, record, options) : client.score(request.get("question").get("value").asText(), record, ConsumerCases.strings(values.get("levels")), options);
                        case "annotate" -> client.annotate(request.get("question").get("value"), record, options);
                        default -> throw new AssertionError(function);
                    };
                    output.add(ConsumerCases.row(row));
                }
            } else {
                try (Stream<? extends Row<?>> rows = switch (function) {
                    case "choose" -> saved ? client.chooseStream(question, input, options) : client.chooseStream(request.get("question").get("value").asText(), input, ConsumerCases.strings(values.get("options")), options);
                    case "tag" -> saved ? client.tagStream(question, input, options) : client.tagStream(request.get("question").get("value").asText(), input, ConsumerCases.strings(values.get("labels")), options);
                    case "score" -> saved ? client.scoreStream(question, input, options) : client.scoreStream(request.get("question").get("value").asText(), input, ConsumerCases.strings(values.get("levels")), options);
                    case "rank" -> client.rank(question, input, options);
                    case "annotate" -> client.annotateStream(request.get("question").get("value"), input, options);
                    default -> throw new AssertionError(function);
                }) { for (var row : rows.toList()) output.add(ConsumerCases.row(row)); }
            }
            return output;
        } catch (ThinkThenException error) { return ConformanceCases.error(error.code()); }
    }
    public static void main(String[] args) throws Exception {
        var cases = ConsumerCases.JSON.readTree(Path.of(args[0]).toFile());
        try (var client = Client.builder().adapter("stub").build()) {
            for (var sample : cases) for (boolean single : List.of(false, true)) {
                var actual = execute(client, sample.get("request"), single);
                var expected = sample.has("expected_error") ? ConformanceCases.error(sample.get("expected_error").asText())
                    : execute(client, sample.get("equivalent_request"), single);
                ConsumerCases.equal(actual, expected, sample.get("name").asText());
                if (!sample.has("expected_error")) ConsumerCases.equal(actual.size(), sample.get("request").get("records").size(), "option rows");
            }
        }
        System.out.println("Installed Java shared option combinations passed");
    }
}
