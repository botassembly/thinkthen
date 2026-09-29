import java.nio.charset.StandardCharsets;
import thinkthen.Door;

public class InstalledJava {
    public static void main(String[] args) {
        try (var door = new Door()) {
            var answer = door.decide("Is it?", "release-java".getBytes(StandardCharsets.UTF_8));
            if (answer.outcome() != 1 || answer.probability() != 0.9) throw new AssertionError(answer);
            System.out.println("INSTALLED_JAVA_PASS");
        }
    }
}
