import dev.thinkthen.*;
import java.nio.file.Files;
import java.nio.file.Path;

public class Decide {
    public static void main(String[] args) throws Exception {
        String message = Files.readString(Path.of("message.txt"));
        try (var client = ThinkThen.builder().build()) {
            // answer() throws UnsureException when not sure,
            // ThinkThenException on failure.
            boolean isRefund = client.decide(
                "Does the customer request a refund?", message).answer();
            System.out.println(isRefund);
        }
    }
}
