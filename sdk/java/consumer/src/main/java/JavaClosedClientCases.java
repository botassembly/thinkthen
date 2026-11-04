import dev.thinkthen.*;
import dev.thinkthen.internal.DriverClient;
import dev.thinkthen.internal.DriverException;
import java.nio.charset.StandardCharsets;
import java.util.List;

/** Closed Java owners preserve the shared engine diagnostic. */
public final class JavaClosedClientCases {
    private JavaClosedClientCases() {}
    private static final String MESSAGE = "client: expected an open client; received a closed client";
    private static void equal(Object actual, Object expected) {
        if (!actual.equals(expected)) throw new AssertionError("Expected " + expected + ", got " + actual);
    }
    public static void main(String[] args) {
        var failures = new java.util.ArrayList<Throwable>();
        for (String entry : List.of("client", "open", "openRecords")) {
            try { check(entry); }
            catch (Throwable failure) { failures.add(failure); }
        }
        if (!failures.isEmpty()) {
            var failure = new AssertionError("Closed client cases failed: " + failures.size());
            failures.forEach(failure::addSuppressed);
            throw failure;
        }
    }
    private static void check(String entry) {
        if (entry.equals("client")) {
            var client = ThinkThen.builder().backend(Backend.STUB).build();
            equal(client.decide("Ready?", "alpha").status(), Status.ANSWERED);
            client.close();
            client.close();
            try {
                client.decide("Ready?", "alpha");
                throw new AssertionError("closed ThinkThen must refuse a call");
            } catch (ThinkThenException error) {
                equal(error.code(), ErrorCode.USAGE_CLOSED);
                equal(error.getMessage(), MESSAGE);
            }
            return;
        }
        var client = new DriverClient("{\"backend\":\"stub\"}".getBytes(StandardCharsets.UTF_8));
        client.close();
        client.close();
        try {
            if (entry.equals("open")) client.open(new byte[0]);
            else client.openRecords(new byte[0], List.of());
            throw new AssertionError("closed " + entry + " must refuse a call");
        } catch (DriverException error) {
            equal(error.error().name(), "usage.closed");
            equal(error.getMessage(), MESSAGE);
        }
    }
}
