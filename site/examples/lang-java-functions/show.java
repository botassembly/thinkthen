import dev.thinkthen.*;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.stream.Stream;

public class Functions {
    public static void main(String[] args) {
        String refund = "Please refund my order.";
        List<Map<String, String>> messages = List.of(
            Map.of("id", "a1", "text", "Please refund my order."),
            Map.of("id", "a2", "text", "Where is my parcel?"),
            Map.of("id", "a3", "text", "I was charged twice for one invoice."));
        String text =
            "Morgan works for Example Group.\nPlease refund my order.\n";
        List<String> urgencyLevels = List.of("routine", "urgent");
        List<String> topicChoices = List.of("billing", "shipping", "other");
        String asksRefund = "Does the customer request a refund?";
        String howUrgent = "How urgent is the request?";
        Pointer field = Pointer.of("/text");

        // answer() throws UnsureException when not sure,
        // ThinkThenException on failure.
        try (var client = ThinkThen.builder().build()) {
            boolean isRefund = client.decide(asksRefund, refund).answer();
            System.out.println("decide: " + isRefund);

            try (Stream<Map<String, String>> refunds = client.filter(
                    asksRefund, messages,
                    FilterOptions.builder().field(field).build())) {
                refunds.forEach(kept ->
                    System.out.println("filter: " + kept.get("id")));
            }

            String topic = client.choose(
                "What is the message about?", refund, topicChoices).answer();
            System.out.println("choose: " + topic);

            List<String> topics = client.tag("Which topics appear?",
                "I was charged twice and my parcel is late again.",
                List.of("billing", "shipping")).answer();
            System.out.println("tag: " + topics);

            double urgency = client.score(
                howUrgent, refund, urgencyLevels).answer();
            System.out.println("score: " + urgency);

            try (Stream<Ranked<JsonValue>> byUrgency = client.rank(
                    howUrgent, messages, urgencyLevels,
                    RankOptions.builder().field(field).build())) {
                byUrgency.forEach(ranked -> {
                    long urgencyRank = ranked.answer();
                    var message = (JsonObject) ranked.item();
                    var id = (JsonString) message.values().get("id");
                    System.out.println(
                        "rank: " + urgencyRank + " " + id.value());
                });
            }

            Optional<Span> action = client.find(
                "Which line states the requested action?", text).answer();
            action.ifPresent(span ->
                System.out.println("find: \"" + span.text() + "\""));

            List<Entity> names = client.recognize(
                "Morgan works for Example Group.",
                List.of("person", "organization")).answer();
            for (Entity name : names) {
                System.out.println("recognize: " + name.mention().kind()
                    + " " + name.mention().span().text());
            }

            List<Relation> relations = client.relate(text,
                List.of(RelationRule.of("person", "organization", "works_for")))
                .answer();
            for (Relation relation : relations) {
                System.out.println("relate: " + relation.source().span().text()
                    + " " + relation.relation()
                    + " " + relation.target().span().text());
            }

            var questions = AnnotationQuestions.builder()
                .put("refund", DecideQuestion.of(asksRefund))
                .put("topic",
                    ChooseQuestion.of(
                        "What is the message about?", topicChoices))
                .build();
            Annotations answers = client.annotate(questions, refund).answer();
            System.out.println(
                "annotate: refund " + answers.decide("refund").answer());
            System.out.println(
                "annotate: topic " + answers.choose("topic").answer());
        }
    }
}
