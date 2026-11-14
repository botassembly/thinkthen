import dev.thinkthen.*;
import java.util.List;

public class Relate {
    public static void main(String[] args) {
        String message = "Morgan works for Example Group.\n"
            + "Please refund my order.";
        var worksFor = RelationRule.of(
            "person", "organization", "works_for");
        try (var client = ThinkThen.builder().build()) {
            List<Relation> relations = client.relate(
                message, List.of(worksFor)).answer();
            for (Relation relation : relations) {
                System.out.println(
                    relation.source().text() + " "
                    + relation.relation() + " "
                    + relation.target().text());
            }
        }
    }
}
