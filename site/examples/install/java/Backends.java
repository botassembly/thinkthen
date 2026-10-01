import static java.nio.charset.StandardCharsets.UTF_8;

import thinkthen.Door;
import thinkthen.Door.Outcome;

public class Backends {
    public static void main(String[] args) {
        try (var tt = new Door()) {
            var question =
                "Does the customer ask for a refund?";
            var brokenIsRefund = tt.decide(
                question,
                "Please refund my order. It arrived broken."
                    .getBytes(UTF_8));
            var thanksIsRefund = tt.decide(
                question,
                "Thanks for the quick help yesterday!"
                    .getBytes(UTF_8));
            assert Door.outcome(brokenIsRefund.value())
                == Outcome.YES;
            assert Door.outcome(thanksIsRefund.value())
                == Outcome.NO;
        }
    }
}
