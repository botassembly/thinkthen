import thinkthen.Engine;
import thinkthen.Inputs.RequestInputText;
import thinkthen.Inputs.RequestQuestionText;

public class Decide {
    public static void main(String[] args) {
        var question = new RequestQuestionText()
            .text("Does the customer ask for a refund?");
        var broken = new RequestInputText().text(
            "Please refund my order. It arrived broken.");

        try (var tt = new Engine()) {
            var isRefund = tt.decide(question, broken)
                .join().decide();
            assert Boolean.TRUE.equals(isRefund);
        }
    }
}
