import dev.thinkthen.*;
import java.util.List;
import java.util.stream.Stream;

public class Rank {
    public static void main(String[] args) {
        List<String> messages = List.of(
            "Please refund my order.",
            "Where is my parcel?",
            "I was charged twice for one invoice.");
        try (var client = ThinkThen.builder().build();
             Stream<Ranked<JsonValue>> byUrgency = client.rank(
                 "How urgent is the request?", messages,
                 List.of("routine", "urgent"))) {
            byUrgency.forEach(ranked -> {
                long rank = ranked.answer();
                var text = ((JsonString) ranked.item()).value();
                System.out.println(rank + " " + text);
            });
        }
    }
}
