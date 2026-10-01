import static java.util.stream.Collectors.joining;

import java.util.List;
import java.util.Map;
import thinkthen.Door;
import thinkthen.Json;

void main() {
    try (var tt = new Door()) {
        var question =
            "Which line gives the refund deadline?";
        var policy = List.of(
            "Returns need the original receipt.",
            "Refunds are issued within 30 days "
                + "of purchase.",
            "Shipping is free on orders over $50.",
            "Gift cards cannot be exchanged for cash.");
        var units = policy.stream()
            .map(Json::quote).collect(joining(", "));
        var refundDeadline = (Map<?, ?>) Json.parseObject(
            tt.call("{\"find\": " + Json.quote(question)
                + ", \"units\": [" + units + "]}"))
            .get("value");
        assert refundDeadline.get("unit")
            .equals(policy.get(1));
    }
}
