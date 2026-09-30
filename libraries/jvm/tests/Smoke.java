import java.nio.charset.StandardCharsets;
import thinkthen.Door;

// The replay smoke (ticket 0335): one decide through the environment-reading
// door, with the question and text sdlc/scripts/smoke names.
public class Smoke {
    public static void main(String[] args) {
        try (var door = new Door()) {
            var answer = door.decide(System.getenv("THINKTHEN_SMOKE_QUESTION"),
                    System.getenv("THINKTHEN_SMOKE_TEXT").getBytes(StandardCharsets.UTF_8));
            System.out.println("smoke: " + switch (Door.outcome(answer.value())) {
                case YES -> "true"; case NO -> "false"; default -> "null"; });
        }
    }
}
