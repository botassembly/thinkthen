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
             var refundChecks =
                 client.filterRows(question, messages, options)) {
            refundChecks.forEach(each -> {
                boolean kept =
                    each.row() instanceof Answered<Boolean, NoDetail> a
                    && a.answer();
                System.out.println(each.item().get("id") + " "
                    + each.row().status() + " kept=" + kept);
            });
        }
    }
}
