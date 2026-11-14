import dev.thinkthen.*;
import java.nio.file.Files;
import java.nio.file.Path;

public class Decide {
    public static void main(String[] args) throws Exception {
        String message = Files.readString(Path.of("message.txt"));
        try (var client = ThinkThen.builder().build()) {
            // A scalar failure throws FailedException at the call.
            // Reading an unsure answer throws UnsureException.
            boolean isRefund = client.decide(
                "Does the customer request a refund?", message).answer();
            System.out.println(isRefund);
        }
    }
}
