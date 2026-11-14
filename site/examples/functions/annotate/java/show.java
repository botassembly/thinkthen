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
                var form = client.annotate(questions, message);
                System.out.println(form.toJson());
            }
        }
    }
}
