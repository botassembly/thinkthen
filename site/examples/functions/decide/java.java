import static java.nio.charset.StandardCharsets.UTF_8;

import thinkthen.Door;
import thinkthen.Door.Outcome;

void main() {
    try (var tt = new Door()) {
        var question =
            "Does the customer ask for a refund?";
        var text =
            "Please refund my order. It arrived broken.";
        var isRefund = tt.decide(
            question, text.getBytes(UTF_8)).value();
        assert Door.outcome(isRefund) == Outcome.YES;
    }
}
