import io.thinkthen.Backend;
import io.thinkthen.ThinkThen;
import java.util.concurrent.CompletableFuture;

/** A loader error must complete the caller's wait instead of stranding it. */
public final class JavaLoaderFailureCases {
    private JavaLoaderFailureCases() {}
    public static void main(String[] args) throws Exception {
        if (args.length == 1 && args[0].equals("load")) {
            try (var client = ThinkThen.builder().backend(Backend.STUB).build()) {
                client.decide("Ready?", "alpha");
            }
            return;
        }
        String message = CompletableFuture.supplyAsync(() -> {
            try (var client = ThinkThen.builder().backend(Backend.STUB).build()) {
                client.decide("Ready?","alpha");
                throw new AssertionError("Unsupported architecture loaded an engine");
            } catch (UnsatisfiedLinkError error) { return error.getMessage(); }
        }).get();
        if (!message.startsWith("No packaged ThinkThen native engine")) throw new AssertionError(message);
    }
}
