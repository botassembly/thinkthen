import dev.thinkthen.Backend;
import dev.thinkthen.ErrorCode;
import dev.thinkthen.ThinkThen;
import dev.thinkthen.ThinkThenException;
import java.util.concurrent.CompletableFuture;

/** A loader failure is the coded error on every attempt, and completes the caller's wait. */
public final class JavaLoaderFailureCases {
    private JavaLoaderFailureCases() {}
    public static void main(String[] args) throws Exception {
        if (args[0].equals("load")) {
            try (var client = ThinkThen.builder().backend(Backend.STUB).build()) {
                client.decide("Ready?", "alpha");
            }
            return;
        }
        for (int attempt = 0; attempt < 2; attempt++) {
            String message = CompletableFuture.supplyAsync(() -> {
                try (var client = ThinkThen.builder().backend(Backend.STUB).build()) {
                    client.decide("Ready?","alpha");
                    throw new AssertionError("Unloadable engine answered");
                } catch (ThinkThenException error) {
                    if (error.code() != ErrorCode.LOCAL_IO) throw new AssertionError(error);
                    return error.getMessage();
                }
            }).get();
            if (!message.contains(args[0])) throw new AssertionError(message);
        }
    }
}
