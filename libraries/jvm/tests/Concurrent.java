import thinkthen.Door;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.concurrent.CompletableFuture;

/** Independent overlapping callers after the matrix has closed its engine. */
public class Concurrent {
    public static void main(String[] args) throws Exception {
        Door.TypedResult<Door.Answer>[] owned;
        try (Door engine = new Door()) {
            @SuppressWarnings("unchecked") CompletableFuture<Door.TypedResult<Door.Answer>>[] calls = new CompletableFuture[3];
            for (int i = 0; i < calls.length; i++) {
                final int index = i;
                calls[i] = CompletableFuture.supplyAsync(() -> engine.decide("Is it?", ("parallel-independent-" + index).getBytes(StandardCharsets.UTF_8)));
            }
            Path barrier=Path.of(System.getenv("TT_BARRIER_DIR"));
            try {
                for (int index=0;index<2;index++) {
                    Path marker=barrier.resolve("arrived-parallel-independent-"+index);
                    for (int retry=0;retry<1000 && !Files.exists(marker);retry++) Thread.sleep(5);
                    if (!Files.exists(marker)) throw new AssertionError("both calls must overlap before release");
                }
            } finally {
                for (int index=0;index<2;index++) Files.writeString(barrier.resolve("release-parallel-independent-"+index), "");
            }
            @SuppressWarnings("unchecked") Door.TypedResult<Door.Answer>[] results = new Door.TypedResult[calls.length];
            for (int index=0;index<calls.length;index++) {
                results[index]=calls[index].join();
                if (results[index].value().outcome()!=1 || results[index].facts().requestsSent()!=1)
                    throw new AssertionError("overlapping caller result/facts");
            }
            owned=results;
        }
        if (owned[0].facts()==owned[1].facts() || !"jev-1.13.0".equals(owned[0].facts().model()) ||
                !"jev-1.13.0".equals(owned[1].facts().model())) throw new AssertionError("owned facts after close");
        System.out.println("INDEPENDENT_CONCURRENT_PASS");
    }
}
