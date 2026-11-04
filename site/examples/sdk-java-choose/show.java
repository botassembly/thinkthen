import dev.thinkthen.*;
import java.util.List;

public class Choose {
    public static void main(String[] args) {
        try (var client = ThinkThen.builder().build()) {
            String topic = client.choose(
                "What is the message about?",
                "Please refund my order.",
                List.of("billing", "shipping", "other"))
                .answer();
            System.out.println(topic);
        }
    }
}
