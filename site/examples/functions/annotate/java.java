import java.math.BigDecimal;
import java.util.Map;
import thinkthen.Engine;
import thinkthen.Results.*;
import thinkthen.Inputs.RequestInputText;
import thinkthen.Inputs.RequestQuestionFile;

public class Annotate {
    public static void main(String[] args) {
        var form =
            new RequestQuestionFile().path("form.json");
        var report = new RequestInputText()
            .text("Steps: click Log in. Nobody gets in.");

        try (var tt = new Engine()) {
            var triage = tt.annotate(form, report)
                .join().annotate().getFirst()
                .value().value();
            assert triage.equals(Map.of(
                "steps", new AnnotatedFieldBoolean(true),
                "area", new AnnotatedFieldString("login"),
                "impact", new AnnotatedFieldNumber(
                    new BigDecimal("1.98"))));
        }
    }
}
