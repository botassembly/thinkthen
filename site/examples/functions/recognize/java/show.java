import dev.thinkthen.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

public class Recognize {
    public static void main(String[] args) throws Exception {
        String message = Files.readString(Path.of("message.txt"));
        try (var client = ThinkThen.builder().build()) {
            List<Entity> names = client.recognize(
                message, List.of("person", "organization")).answer();
            for (Entity name : names) {
                System.out.println(name.kind() + ": "
                    + name.text());
            }
        }
    }
}
