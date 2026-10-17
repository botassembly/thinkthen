import java.util.List;
import thinkthen.Engine;
import thinkthen.Inputs.AuthoredLevels;
import thinkthen.Inputs.AuthoredQuestionText;
import thinkthen.Inputs.RequestDefinitionAuthoredScore;
import thinkthen.Inputs.RequestInputText;
import thinkthen.Inputs.RequestQuestionDefinition;

public class Score {
    public static void main(String[] args) {
        var question = "How urgent is this?";
        var levels =
            List.of("Routine.", "Soon.", "Immediate.");
        var scale = new RequestQuestionDefinition().value(
            new RequestDefinitionAuthoredScore()
                .score(new AuthoredQuestionText(question))
                .levels(new AuthoredLevels(levels)));
        var outage = new RequestInputText().text(
            "Our checkout page is down "
            + "and customers cannot pay.\n");

        try (var tt = new Engine()) {
            var urgency = tt.score(scale, outage)
                .join().value();
            assert urgency.doubleValue() == 2.0;
        }
    }
}
