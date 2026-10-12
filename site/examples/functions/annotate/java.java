import java.math.BigDecimal;
import java.util.Map;
import thinkthen.Engine;
import thinkthen.Inputs.RequestInputText;
import thinkthen.Inputs.RequestQuestionFile;
import thinkthen.Results.SessionPacketAnnotateRow;

public class Annotate {
    public static void main(String[] args) {
        var form =
            new RequestQuestionFile().path("form.json");
        var report = new RequestInputText()
            .text("Steps: click Log in. Nobody gets in.");
        var row = SessionPacketAnnotateRow.class;
        try (var tt = new Engine()) {
            var triage = tt.annotate(form, report)
                .join().packets().stream()
                .filter(row::isInstance)
                .map(row::cast)
                .findFirst().orElseThrow()
                .value().value().json();
            assert triage.equals(Map.of(
                "steps", true,
                "area", "login",
                "impact", new BigDecimal("1.98")));
        }
    }
}
