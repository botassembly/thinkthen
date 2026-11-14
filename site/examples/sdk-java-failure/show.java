import dev.thinkthen.*;

public class Failure {
    public static void main(String[] args) {
        try (var client = ThinkThen.builder().build()) {
            var refundCheck = client.decide("Does the customer request a refund?", "");
            System.out.println(refundCheck);
        }
    }
}
