import dev.thinkthen.*;

public class Unsure {
    public static void main(String[] args) {
        var bars = DecideOptions.builder()
            .threshold(Threshold.band(0.2, 0.8)).build();
        try (var client = ThinkThen.builder().build()) {
            var isRefund = client.decide(
                "Does the customer request a refund?",
                "I was charged twice for one invoice.", bars);
            try {
                if (isRefund.answer()) {
                    System.out.println("Send to refunds.");
                } else {
                    System.out.println("Leave in the inbox.");
                }
            } catch (UnsureException notSure) {
                System.out.println("Not sure. Ask a person.");
            }
        }
    }
}
