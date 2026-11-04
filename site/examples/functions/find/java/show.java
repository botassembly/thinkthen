import dev.thinkthen.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Optional;

public class Find {
    public static void main(String[] args) throws Exception {
        String message = Files.readString(Path.of("message.txt"));
        try (var client = ThinkThen.builder().build()) {
            // Empty when no line answers the question.
            Optional<Span> action = client.find(
                "Which line states the requested action?", message).answer();
            System.out.println(
                action.map(Span::text).orElse("no line answers"));
        }
    }
}
