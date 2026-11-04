import dev.thinkthen.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.stream.Stream;

public class Rank {
    public static void main(String[] args) throws Exception {
        List<String> messages = Files.readAllLines(Path.of("messages.txt"));
        // Every message, most urgent first.
        // answer() is its rank, starting at 1.
        try (var client = ThinkThen.builder().build();
             Stream<Ranked<JsonValue>> byUrgency = client.rank(
                 "How urgent is the request?", messages,
                 List.of("routine", "urgent"))) {
            byUrgency.forEach(ranked -> {
                long urgencyRank = ranked.answer();
                // A text message comes back as a JsonString.
                String message = ((JsonString) ranked.item()).value();
                System.out.println(urgencyRank + " " + message);
            });
        }
    }
}
