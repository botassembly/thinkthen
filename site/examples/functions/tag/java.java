import java.util.List;
import thinkthen.Engine;
import thinkthen.Inputs.AuthoredLabels;
import thinkthen.Inputs.AuthoredQuestionText;
import thinkthen.Inputs.RequestDefinitionFieldsTag;
import thinkthen.Inputs.RequestInputText;
import thinkthen.Inputs.RequestQuestionDefinition;
import thinkthen.Results.SessionPacketTagRow;

public class Tag {
    public static void main(String[] args) {
        var question = "Which labels fit this message?";
        var labels = List.of("praise", "bug", "billing");
        var fits = new RequestQuestionDefinition().value(
            new RequestDefinitionFieldsTag()
                .tag(new AuthoredQuestionText(question))
                .labels(new AuthoredLabels(labels)));
        var message = new RequestInputText().text(
            "Love the new dashboard, "
            + "but export crashes the app,\n"
            + "and I was charged twice.\n");
        var row = SessionPacketTagRow.class;
        try (var tt = new Engine()) {
            var fittingLabels = tt.tag(fits, message)
                .join().packets().stream()
                .filter(row::isInstance)
                .map(row::cast)
                .findFirst().orElseThrow()
                .value().value();
            assert fittingLabels.equals(labels);
        }
    }
}
