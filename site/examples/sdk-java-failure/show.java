import dev.thinkthen.*;

public class Failure {
    public static void main(String[] args) {
        try (var client = ThinkThen.builder().build()) {
            client.decide(
                "Does the customer request a refund?", "");
        } catch (ThinkThenException failure) {
            System.out.println(failure.code().wireName());
        }
    }
}
