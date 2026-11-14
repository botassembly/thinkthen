import dev.thinkthen.*;
import java.util.List;

public class Annotate {
    public static void main(String[] args) {
        List<String> messages = List.of(
            "Please refund my order.",
            "Where is my parcel?",
            "I was charged twice for one invoice.");
        var questions = AnnotationQuestions.builder()
            .put("refund", DecideQuestion.of(
                "Does the customer request a refund?"))
            .put("topic", ChooseQuestion.of(
                "What is the message about?",
                List.of("billing", "shipping", "other")))
            .put("urgency", ScoreQuestion.of(
                "How urgent is the request?",
                List.of("routine", "urgent")))
            .build();
        try (var client = ThinkThen.builder().build()) {
            for (String message : messages) {
                var form = client.annotate(questions, message);
                System.out.println(form.toJson());
            }
        }
    }
}
