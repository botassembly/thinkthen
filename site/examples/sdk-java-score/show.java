import dev.thinkthen.*;
import java.util.List;

public class Score {
    public static void main(String[] args) {
        try (var client = ThinkThen.builder().build()) {
            double urgency = client.score(
                "How urgent is the request?",
                "Please refund my order.",
                List.of("routine", "urgent")).answer();
            System.out.println(urgency);
        }
    }
}
