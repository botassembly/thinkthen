import java.util.List;
import thinkthen.Engine;
import thinkthen.Inputs.RequestInputRecords;
import thinkthen.Inputs.RequestItem;
import thinkthen.Inputs.RequestOriginalText;
import thinkthen.Inputs.RequestQuestionText;

public class Filter {
    public static void main(String[] args) {
        var question = new RequestQuestionText()
            .text("Is this a complaint?");
        var reviews = List.of(
            "Arrived a day early. Thank you!",
            "The zipper broke the first time I used it.",
            "Does this come in blue?",
            "The strap snapped on day two.");
        var records = new RequestInputRecords().items(
            reviews.stream()
                .map(text -> new RequestItem().original(
                    new RequestOriginalText().text(text)))
                .toList());
        try (var tt = new Engine()) {
            var complaints = tt.filter(question, records)
                .join().value();
            assert complaints.equals(
                List.of(reviews.get(1), reviews.get(3)));
        }
    }
}
