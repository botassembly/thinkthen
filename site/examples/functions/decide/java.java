import thinkthen.Engine;
import thinkthen.Inputs.RequestInputText;
import thinkthen.Inputs.RequestQuestionText;
import thinkthen.Results.SessionPacketDecideRow;

public class Decide {
    public static void main(String[] args) {
        var question = new RequestQuestionText()
            .text("Does the customer ask for a refund?");
        var broken = new RequestInputText().text(
            "Please refund my order. It arrived broken.");
        var row = SessionPacketDecideRow.class;
        try (var tt = new Engine()) {
            var isRefund = tt.decide(question, broken)
                .join().packets().stream()
                .filter(row::isInstance)
                .map(row::cast)
                .findFirst().orElseThrow()
                .value().value().value();
            assert Boolean.TRUE.equals(isRefund);
        }
    }
}
