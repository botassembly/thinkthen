import dev.thinkthen.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

public class Rank {
    public static void main(String[] args) throws Exception {
        List<String> messages = Files.readAllLines(Path.of("messages.txt"));
        // Every message, most urgent first.
        // answer() is its rank, starting at 1.
        try (var client = ThinkThen.builder().build();
             var byUrgency = client.rank(
                 "How urgent is the request?", messages,
                 List.of("routine", "urgent"))) {
            byUrgency.forEach(ranked -> {
                long urgencyRank = ranked.answer();
                String message = ranked.item();
                System.out.println(urgencyRank + " " + message);
            });
        }
    }
}
