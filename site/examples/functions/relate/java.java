import java.util.List;
import java.util.Map;
import thinkthen.Engine;
import thinkthen.Inputs;
import thinkthen.Inputs.RequestInputRecords;
import thinkthen.Inputs.RequestItem;
import thinkthen.Inputs.RequestOriginalJson;
import thinkthen.Inputs.RequestQuestionDefinition;

public class Relate {
    public static void main(String[] args) {
        var makes = new Inputs.AuthoredRelation()
            .name("makes").source("company")
            .target("product").either(false);
        var ask = new RequestQuestionDefinition().value(
            new Inputs.RequestDefinitionAuthoredRelate()
                .relate(new Inputs.RelateQuestion()
                    .relations(List.of(makes))));
        var apple = Map.of(
            "name", "Apple", "kind", "company");
        var amazon = Map.of(
            "name", "Amazon", "kind", "company");
        var iphone = Map.of(
            "name", "iPhone", "kind", "product");
        var kindle = Map.of(
            "name", "Kindle", "kind", "product");
        var names = new RequestInputRecords().items(
            List.of(apple, amazon, iphone, kindle).stream()
                .map(one -> new RequestItem().original(
                    new RequestOriginalJson().value(one)))
                .toList());

        try (var tt = new Engine()) {
            var whoMakes = tt.relate(ask, names)
                .join().value();
            var pairs = whoMakes.stream()
                .map(edge -> List.of(
                    edge.sourceName(), edge.targetName()))
                .toList();
            assert pairs.equals(List.of(
                List.of("Apple", "iPhone"),
                List.of("Amazon", "Kindle")));
        }
    }
}
