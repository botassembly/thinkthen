import dev.thinkthen.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

public class Score {
    public static void main(String[] args) throws Exception {
        String message = Files.readString(Path.of("message.txt"));
        try (var client = ThinkThen.builder().build()) {
            double urgency = client.score(
                "How urgent is the request?", message,
                List.of("routine", "urgent")).answer();
            System.out.println(urgency);
        }
    }
}
