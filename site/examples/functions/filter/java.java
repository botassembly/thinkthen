import static java.util.stream.Collectors.joining;

import java.util.List;
import thinkthen.Door;
import thinkthen.Json;

void main() {
    try (var tt = new Door()) {
        var question = "Is this a complaint?";
        var reviews = List.of(
            "Arrived a day early. Thank you!",
            "The zipper broke the first time I used it.",
            "Does this come in blue?",
            "The strap snapped on day two.");
        var records = reviews.stream()
            .map(Json::quote).collect(joining(", "));
        var complaints = Json.parseObject(tt.call(
            "{\"filter\": " + Json.quote(question)
            + ", \"records\": [" + records + "]}"))
            .get("value");
        assert complaints.equals(
            List.of(reviews.get(1), reviews.get(3)));
    }
}
