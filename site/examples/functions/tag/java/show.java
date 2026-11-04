import dev.thinkthen.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

public class Tag {
    public static void main(String[] args) throws Exception {
        String message = Files.readString(Path.of("message.txt"));
        try (var client = ThinkThen.builder().build()) {
            List<String> topics = client.tag(
                "Which topics appear?", message,
                List.of("billing", "shipping")).answer();
            System.out.println(topics);
        }
    }
}
