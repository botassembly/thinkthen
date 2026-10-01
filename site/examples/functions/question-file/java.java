import static java.nio.charset.StandardCharsets.UTF_8;

import java.nio.file.Files;
import java.nio.file.Path;
import thinkthen.Door;
import thinkthen.Door.Outcome;

void main() throws Exception {
    try (var tt = new Door()) {
        var refund =
            Files.readString(Path.of("refund.json"));
        var text = "I would like to return this "
            + "and get my money back.\n";
        var isRefund =
            tt.decide(refund, text.getBytes(UTF_8)).value();
        assert Door.outcome(isRefund) == Outcome.YES;
    }
}
