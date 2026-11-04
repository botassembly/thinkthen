import dev.thinkthen.*;
import java.util.Optional;

public class Find {
    public static void main(String[] args) {
        String message = "Morgan works for Example Group.\n"
            + "Please refund my order.";
        try (var client = ThinkThen.builder().build()) {
            Optional<Span> action = client.find(
                "Which line states the requested action?",
                message).answer();
            System.out.println(
                action.map(Span::text).orElse("no line answers"));
        }
    }
}
