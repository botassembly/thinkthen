import dev.thinkthen.*;
import java.util.List;
import java.util.Map;

public class Example {
    public static void main(String[] args) {
        List<Map<String, String>> messages = List.of(
            Map.of("id", "a1", "text", "Please refund my order."),
            Map.of("id", "a2", "text", "Where is my parcel?"),
            Map.of("id", "a3", "text", "I was charged twice for one invoice."));
        var question = "Does the customer request a refund?";
        var options = FilterOptions.builder()
            .field(Pointer.of("/text")).build();
        try (var client = ThinkThen.builder().build();
             var refunds = client.filter(question, messages, options)) {
            refunds.forEach(message -> System.out.println(
                message.get("id") + " " + message.get("text")));
        } catch (FailedException failure) {
            for (var failed : failure.rows()) {
                System.out.println("try again later: " + failed.item());
            }
        }
    }
}
