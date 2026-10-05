import static java.nio.charset.StandardCharsets.UTF_8;

import thinkthen.Door;
import thinkthen.Door.Outcome;

public class Backends {
    public static void main(String[] args) {
        String[] settings = {
            "{\"backend\":\"typesafe\"}",
            "{\"backend\":\"liquid\"}",
            "{\"backend\":\"ollama\",\"base_url\":" +
                "\"http://localhost:11535/v1\"}",
        };
        for (String setting : settings) {
            try (var tt = new Door(setting)) {
                var question =
                    "Does the customer ask for a refund?";
                var brokenIsRefund = tt.decide(
                    question,
                    ("Please refund my order. "
                        + "It arrived broken.")
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
}
