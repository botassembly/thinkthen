import dev.thinkthen.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

public class Annotate {
    public static void main(String[] args) throws Exception {
        List<String> messages = Files.readAllLines(Path.of("messages.txt"));
        var questions = AnnotationQuestions.builder()
            .put("refund",
                DecideQuestion.of("Does the customer request a refund?"))
            .put("topic", ChooseQuestion.of("What is the message about?",
                List.of("billing", "shipping", "other")))
            .put("urgency", ScoreQuestion.of("How urgent is the request?",
                List.of("routine", "urgent")))
            .build();
        try (var client = ThinkThen.builder().build()) {
            for (String message : messages) {
                Annotations answers =
                    client.annotate(questions, message).answer();
                boolean isRefund = answers.decide("refund").answer();
                String topic = answers.choose("topic").answer();
                double urgency = answers.score("urgency").answer();
                System.out.println(isRefund + " " + topic + " " + urgency);
            }
        }
    }
}
