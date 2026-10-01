import static java.nio.charset.StandardCharsets.UTF_8;

import thinkthen.Door;
import thinkthen.Door.Outcome;

public class FirstCall {
    public static void main(String[] args) {
        try (var tt = new Door()) {
            var question =
                "Does the customer ask for a refund?";
            var isRefund = tt.decide(
                question,
                "Please refund my order. It arrived broken."
                    .getBytes(UTF_8));
            assert Door.outcome(isRefund.value())
                == Outcome.YES;

            var refund = "{\"decide\": \"" + question
                + "\", \"threshold\": \"0.2:0.8\"}";
            isRefund = tt.decide(
                refund,
                "I want to send this back."
                    .getBytes(UTF_8));
            assert Door.outcome(isRefund.value())
                == Outcome.NOT_SURE;
        }
    }
}
