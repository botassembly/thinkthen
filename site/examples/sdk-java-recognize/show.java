import dev.thinkthen.*;
import java.util.List;

public class Recognize {
    public static void main(String[] args) {
        try (var client = ThinkThen.builder().build()) {
            List<Entity> names = client.recognize(
                "Morgan works for Example Group.",
                List.of("person", "organization")).answer();
            for (Entity name : names) {
                System.out.println(name.kind() + " "
                    + name.text());
            }
        }
    }
}
