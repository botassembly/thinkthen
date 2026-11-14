import dev.thinkthen.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

public class Relate {
    public static void main(String[] args) throws Exception {
        String message = Files.readString(Path.of("message.txt"));
        var worksFor = RelationRule.of("person", "organization", "works_for");
        try (var client = ThinkThen.builder().build()) {
            List<Relation> relations =
                client.relate(message, List.of(worksFor)).answer();
            for (Relation relation : relations) {
                System.out.println(relation.source().text()
                    + " --" + relation.relation() + "--> "
                    + relation.target().text());
            }
        }
    }
}
