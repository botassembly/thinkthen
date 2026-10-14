import java.util.List;
import thinkthen.Engine;
import thinkthen.Inputs.RequestInputRecords;
import thinkthen.Inputs.RequestItem;
import thinkthen.Inputs.RequestOriginalText;
import thinkthen.Inputs.RequestQuestionText;

public class Find {
    public static void main(String[] args) {
        var question = new RequestQuestionText()
            .text("Which line gives the refund deadline?");
        var policy = List.of(
            "Returns need the original receipt.",
            "Refunds are issued within 30 days "
                + "of purchase.",
            "Shipping is free on orders over $50.",
            "Gift cards cannot be exchanged for cash.");
        var records = new RequestInputRecords().items(
            policy.stream()
                .map(text -> new RequestItem().original(
                    new RequestOriginalText().text(text)))
                .toList());

        try (var tt = new Engine()) {
            var refundDeadline = tt.find(question, records)
                .join().value();
            assert refundDeadline.equals(policy.get(1));
        }
    }
}
