import java.util.List;
import thinkthen.Door;
import thinkthen.Json;

void main() {
    try (var tt = new Door()) {
        var question = "Which labels fit this message?";
        var labels = """
            ["praise", "bug", "billing"]""";
        var message = """
            Love the new dashboard, but export crashes \
            the app,
            and I was charged twice.
            """;
        var fittingLabels = Json.parseObject(tt.call(
            "{\"tag\": " + Json.quote(question)
            + ", \"labels\": " + labels
            + ", \"evidence\": " + Json.quote(message)
            + "}")).get("value");
        assert fittingLabels.equals(
            List.of("praise", "bug", "billing"));
    }
}
