import static java.util.stream.Collectors.joining;

import java.math.BigDecimal;
import java.util.List;
import java.util.Map;
import thinkthen.Door;
import thinkthen.Json;

void main() {
    try (var tt = new Door()) {
        var question = "Is this urgent?";
        var inbox = List.of(
            "Newsletter: our autumn catalog is here. "
                + "No reply needed.",
            "Our checkout page is down and customers "
                + "cannot pay",
            "Reminder: your invoice is due in 30 days",
            "Please send the signed quote by 5 pm today");
        var records = inbox.stream()
            .map(Json::quote).collect(joining(", "));
        var byUrgency = (List<?>) Json.parseObject(tt.call(
            "{\"rank\": " + Json.quote(question)
            + ", \"records\": [" + records + "]}"))
            .get("value");
        var order = byUrgency.stream()
            .map(one -> ((Map<?, ?>) one).get("index"))
            .map(index -> ((BigDecimal) index).intValue())
            .toList();
        assert order.equals(List.of(1, 3, 2, 0));
    }
}
