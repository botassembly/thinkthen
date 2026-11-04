import dev.thinkthen.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

public class Choose {
    public static void main(String[] args) throws Exception {
        String message = Files.readString(Path.of("message.txt"));
        try (var client = ThinkThen.builder().build()) {
            String topic = client.choose(
                "What is the message about?", message,
                List.of("billing", "shipping", "other")).answer();
            System.out.println(topic);
        }
    }
}
