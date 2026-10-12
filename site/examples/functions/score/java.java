import java.util.List;
import thinkthen.Engine;
import thinkthen.Inputs.AuthoredLevels;
import thinkthen.Inputs.AuthoredQuestionText;
import thinkthen.Inputs.RequestDefinitionFieldsScore;
import thinkthen.Inputs.RequestInputText;
import thinkthen.Inputs.RequestQuestionDefinition;
import thinkthen.Results.SessionPacketScoreRow;

public class Score {
    public static void main(String[] args) {
        var question = "How urgent is this?";
        var levels =
            List.of("Routine.", "Soon.", "Immediate.");
        var scale = new RequestQuestionDefinition().value(
            new RequestDefinitionFieldsScore()
                .score(new AuthoredQuestionText(question))
                .levels(new AuthoredLevels(levels)));
        var outage = new RequestInputText().text(
            "Our checkout page is down "
            + "and customers cannot pay.\n");
        var row = SessionPacketScoreRow.class;
        try (var tt = new Engine()) {
            var urgency = tt.score(scale, outage)
                .join().packets().stream()
                .filter(row::isInstance)
                .map(row::cast)
                .findFirst().orElseThrow()
                .value().value().doubleValue();
            assert urgency == 2.0;
        }
    }
}
