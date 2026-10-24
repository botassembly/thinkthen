import io.thinkthen.Client;
import io.thinkthen.Row;
import io.thinkthen.Rule;
import java.util.List;

public final class JavaLive {
    private JavaLive() {}
    public static void report(String function, Row<?> row) throws Exception {
        System.out.println(function + " " + ConsumerCases.row(row));
        if (row.status() == Row.Status.FAILED) throw new IllegalStateException(row.error().code());
    }
    public static void main(String[] args) throws Exception {
        String text = "Ada works for Acme.";
        try (var client = Client.builder().build()) {
            report("decide", client.decide("Does this state employment?", text));
            report("choose", client.choose("What topic is stated?", text, List.of("employment", "delivery")));
            report("tag", client.tag("Which topics are stated?", text, List.of("employment", "delivery")));
            report("score", client.score("How clearly is employment stated?", text, List.of("unclear", "clear")));
            try (var kept = client.filter("Does this state employment?", List.of(text, "A parcel arrived."))) { System.out.println("filter " + kept.toList()); }
            try (var ranked = client.rank("Does this state employment?", List.of(text, "A parcel arrived."))) { for (var row : ranked.toList()) report("rank", row); }
            report("find", client.find("Who works for Acme?", text));
            var toolbox = ConsumerCases.JSON.readTree("{\"questions\":{\"employment\":{\"decide\":\"Does this state employment?\"}}}");
            report("annotate", client.annotate(toolbox, text));
            report("recognize", client.recognize(text, List.of("person", "company")));
            report("relate", client.relate(text, List.of(new Rule("works_for", "person", "company", null, null))));
        }
    }
}
