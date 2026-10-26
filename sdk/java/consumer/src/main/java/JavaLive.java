import io.thinkthen.*;
import java.util.List;

public final class JavaLive {
    private JavaLive() {}
    public static void report(String function, Row<?,?> row) {
        System.out.println(function + " " + ConformanceCases.row(row));
        row.raiseForStatus();
    }
    public static void report(String function,io.thinkthen.internal.Row<?> row) throws Exception {
        System.out.println(function + " " + ConsumerCases.row(row));
        if (row.status() == io.thinkthen.internal.Row.Status.FAILED) throw new IllegalStateException(row.error().code());
    }
    public static void main(String[] args) {
        String text = "Ada works for Acme.";
        try (var client = ThinkThen.builder().build()) {
            report("decide",client.decide("Does this state employment?",text));
            report("choose",client.choose("What topic is stated?",text,List.of("employment","delivery")));
            report("tag",client.tag("Which topics are stated?",text,List.of("employment","delivery")));
            report("score",client.score("How clearly is employment stated?",text,List.of("unclear","clear")));
            try (var rows = client.filter("Does this state employment?",List.of(text,"A parcel arrived."))) { rows.forEach(item -> report("filter",item.row())); }
            try (var rows = client.rank("How clearly is employment stated?",List.of(text,"A parcel arrived."),List.of("unclear","clear"))) { rows.forEach(item -> report("rank",item.row())); }
            report("find",client.find("Who works for Acme?",text));
            var questions = AnnotationQuestions.builder().put("employment",DecideQuestion.of("Does this state employment?")).build();
            report("annotate",client.annotate(questions,text));
            report("recognize",client.recognize(text,List.of("person","company")));
            report("relate",client.relate(text,List.of(RelationRule.builder("person","company","works_for").build())));
        }
    }
}
