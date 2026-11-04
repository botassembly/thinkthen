import dev.thinkthen.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.stream.Stream;

public class Filter {
    public static void main(String[] args) throws Exception {
        List<String> messages = Files.readAllLines(Path.of("messages.txt"));
        // Only the messages that answer yes.
        // A failed message throws after the kept ones.
        try (var client = ThinkThen.builder().build();
             Stream<String> refunds = client.filter(
                 "Does the customer request a refund?", messages)) {
            refunds.forEach(System.out::println);
        }
    }
}
