import java.util.List;
import thinkthen.Engine;
import thinkthen.Inputs.RequestInputRecords;
import thinkthen.Inputs.RequestItem;
import thinkthen.Inputs.RequestOriginalText;
import thinkthen.Inputs.RequestQuestionText;

public class Rank {
    public static void main(String[] args) {
        var question = new RequestQuestionText()
            .text("Is this urgent?");
        var inbox = List.of(
            "Newsletter: our autumn catalog is here. "
                + "No reply needed.",
            "Our checkout page is down "
                + "and customers cannot pay",
            "Reminder: your invoice is due in 30 days",
            "Please send the signed quote by 5 pm today");
        var records = new RequestInputRecords().items(
            inbox.stream()
                .map(text -> new RequestItem().original(
                    new RequestOriginalText().text(text)))
                .toList());
        try (var tt = new Engine()) {
            var order = tt.rank(question, records)
                .join().value().stream()
                .map(one -> one.intValue()).toList();
            assert order.equals(List.of(1, 3, 2, 0));
        }
    }
}
