import io.thinkthen.Backend;
import io.thinkthen.ThinkThen;
import io.thinkthen.ThinkThenException;

/** Run in a fresh JVM with the shared incompatible-ABI engine packaged in the JAR. */
public final class JavaAbiRefusalCases {
    private JavaAbiRefusalCases() {}

    public static void main(String[] args) {
        if (args.length >= 1 && args[0].equals("row")) {
            try (var client = ThinkThen.builder().backend(Backend.STUB).build()) {
                try { client.decide("Ready?",args.length > 1 && (args[1].equals("unanswered-answer") || args[1].startsWith("failed-")) ? null : "alpha"); throw new AssertionError("Accepted a malformed native row"); }
                catch (ThinkThenException error) { JavaApiCases.equal(error.code().wireName(),"usage.abi","malformed row code"); }
            }
            return;
        }
        try (var client = ThinkThen.builder().backend(Backend.STUB).build()) {
            client.decide("Ready?", "alpha");
            throw new AssertionError("The loader accepted an incompatible engine ABI");
        } catch (ThinkThenException error) {
            JavaApiCases.equal(error.code().wireName(), "usage.abi", "loader refuses incompatible ABI with a typed code");
        }
    }
}
