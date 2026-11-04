import dev.thinkthen.*;

public class Example {
    public static void main(String[] args) {
        try (var client = ThinkThen.builder().build()) {
            boolean isRefund = client.decide(
                "Does the customer request a refund?",
                "Please refund my order.").answer();
            System.out.println(isRefund);
        }
    }
}
