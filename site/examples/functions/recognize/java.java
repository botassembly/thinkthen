import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import thinkthen.Engine;
import thinkthen.Inputs;
import thinkthen.Inputs.RequestInputText;
import thinkthen.Inputs.RequestQuestionDefinition;
import thinkthen.Results.RecognizeEntities;

public class Recognize {
    public static void main(String[] args) {
        var kinds = new LinkedHashMap<String, Object>();
        kinds.put("person", null);
        kinds.put("product", null);
        kinds.put("place", null);
        var ask = new RequestQuestionDefinition().value(
            Inputs.RequestDefinitionRecognizeDefinition
                .fromJson(Map.of("version", 1,
                    "recognize", Map.of("kinds", kinds))));
        var text = new RequestInputText().text(
            "Sam Patel says his Kindle arrived broken "
            + "in Denver.");

        try (var tt = new Engine()) {
            var facts = (RecognizeEntities) tt
                .recognize(ask, text)
                .join().recognize().getFirst().value();
            var names = facts.entities().stream()
                .map(one -> List.of(one.text(), one.kind()))
                .toList();
            assert names.equals(List.of(
                List.of("Sam Patel", "person"),
                List.of("Kindle",
                    "product"),
                List.of("Denver", "place")));
        }
    }
}
