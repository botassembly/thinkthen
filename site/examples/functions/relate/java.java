import java.util.List;
import java.util.Map;
import thinkthen.Engine;
import thinkthen.Inputs;
import thinkthen.Inputs.RequestInputRecords;
import thinkthen.Inputs.RequestItem;
import thinkthen.Inputs.RequestOriginalJson;
import thinkthen.Inputs.RequestQuestionDefinition;
import thinkthen.Results.SessionPacketRelateAggregate;

public class Relate {
    public static void main(String[] args) {
        var sings = Map.of("name", "sings",
            "source", "singer", "target", "song",
            "either", false);
        var ask = new RequestQuestionDefinition().value(
            Inputs.RequestDefinitionFieldsRelateVersion
                .fromJson(Map.of("version", 1, "relate",
                    Map.of("relations", List.of(sings)))));
        var paul = Map.of(
            "name", "Paul McCartney", "kind", "singer");
        var ringo = Map.of(
            "name", "Ringo Starr", "kind", "singer");
        var yesterday = Map.of(
            "name", "Yesterday", "kind", "song");
        var garden = Map.of(
            "name", "Octopus's Garden", "kind", "song");
        var names = new RequestInputRecords().items(
            List.of(paul, ringo, yesterday, garden).stream()
                .map(one -> new RequestItem().original(
                    new RequestOriginalJson().value(one)))
                .toList());
        var related = SessionPacketRelateAggregate.class;
        try (var tt = new Engine()) {
            var whoSings = tt.relate(ask, names)
                .join().packets().stream()
                .filter(related::isInstance)
                .map(related::cast)
                .findFirst().orElseThrow()
                .value().value();
            var pairs = whoSings.stream()
                .map(edge -> List.of(
                    edge.source().json(),
                    edge.target().json()))
                .toList();
            assert pairs.equals(List.of(
                List.of(paul, yesterday),
                List.of(ringo, garden)));
        }
    }
}
