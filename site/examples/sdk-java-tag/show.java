import dev.thinkthen.*;
import java.util.List;

public class Tag {
    public static void main(String[] args) {
        try (var client = ThinkThen.builder().build()) {
            List<String> topics = client.tag(
                "Which topics appear?",
                "I was charged twice and my parcel is late again.",
                List.of("billing", "shipping")).answer();
            System.out.println(topics);
        }
    }
}
