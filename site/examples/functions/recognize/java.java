import java.util.List;
import thinkthen.Engine;
import thinkthen.Inputs;
import thinkthen.Inputs.RequestInputText;
import thinkthen.Inputs.RequestQuestionDefinition;
import thinkthen.Results.RecognizeEntities;

public class Recognize {
    public static void main(String[] args) {
        var kinds =
            new Inputs.Ordered<
                Inputs.AuthoredDescription>();
        kinds.entry("person", null);
        kinds.entry("product", null);
        kinds.entry("place", null);
        var ask = new RequestQuestionDefinition().value(
            new Inputs
                .RequestDefinitionRecognizeDefinition()
                .recognize(new Inputs.RecognizeQuestion()
                    .kinds(kinds)));
        var text = new RequestInputText().text(
            "Sam Patel says his Kindle arrived broken "
            + "in Denver.");

        try (var tt = new Engine()) {
            var facts = (RecognizeEntities) tt
                .recognize(ask, text)
                .join().value();
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
