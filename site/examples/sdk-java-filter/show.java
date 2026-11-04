import dev.thinkthen.*;
import java.util.List;
import java.util.stream.Stream;

public class Filter {
    public static void main(String[] args) {
        List<String> messages = List.of(
            "Please refund my order.",
            "Where is my parcel?",
            "I was charged twice for one invoice.");
        try (var client = ThinkThen.builder().build();
             Stream<String> refunds = client.filter(
                 "Does the customer request a refund?",
                 messages)) {
            refunds.forEach(System.out::println);
        }
    }
}
