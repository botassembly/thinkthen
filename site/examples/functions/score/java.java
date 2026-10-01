import java.math.BigDecimal;
import thinkthen.Door;
import thinkthen.Json;

void main() {
    try (var tt = new Door()) {
        var question = "How urgent is this?";
        var levels = """
            ["Routine.", "Soon.", "Immediate."]""";
        var text = "Our checkout page is down "
            + "and customers cannot pay.\n";
        var urgency = Json.parseObject(tt.call(
            "{\"score\": " + Json.quote(question)
            + ", \"levels\": " + levels
            + ", \"evidence\": " + Json.quote(text) + "}"))
            .get("value");
        assert urgency.equals(new BigDecimal("2.0"));
    }
}
