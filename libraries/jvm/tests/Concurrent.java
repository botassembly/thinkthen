import thinkthen.Door;
import java.nio.charset.StandardCharsets;
import java.util.concurrent.CompletableFuture;

/** Independent overlapping callers after the matrix has closed its engine. */
public class Concurrent {
    public static void main(String[] args) {
        try (Door engine = new Door()) {
            @SuppressWarnings("unchecked") CompletableFuture<Door.Answer>[] calls = new CompletableFuture[3];
            for (int i = 0; i < calls.length; i++) {
                final int index = i;
                calls[i] = CompletableFuture.supplyAsync(() -> engine.decide("Is it?", ("parallel-independent-" + index).getBytes(StandardCharsets.UTF_8)));
            }
            for (CompletableFuture<Door.Answer> call : calls)
                if (call.join().outcome() != 1) throw new AssertionError("overlapping caller result");
        }
        System.out.println("INDEPENDENT_CONCURRENT_PASS");
    }
}
