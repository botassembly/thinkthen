import io.thinkthen.*;
import java.net.URI;
import java.time.Duration;
import java.util.ArrayList;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.TimeUnit;

/** Feature: one client starts every active call while earlier replies remain held. */
public final class JavaConcurrencyCases {
    private JavaConcurrencyCases() {}
    public static void main(String[] args) throws Exception {
        try (var fixture = new BackendFixture("headers");
             var client = ThinkThen.builder().backend(Backend.SYSTEMONE).url(URI.create(fixture.base()))
                 .apiKey("loopback-only").retry(RetryPolicy.builder().maxRetries(0)
                     .attemptTimeout(Duration.ofSeconds(60)).build()).build()) {
            var calls = new ArrayList<CompletableFuture<Row<Boolean,NoDetail>>>();
            try {
                for (int i = 0; i < 8; i++) calls.add(client.async().decide("Ready?", "record " + i));
                var reader = java.util.concurrent.Executors.newSingleThreadExecutor(Thread.ofPlatform().daemon().factory());
                try {
                    var arrivals = reader.submit(() -> {
                        for (int i = 0; i < 8; i++) JavaApiCases.equal(fixture.command("arrived"),"arrived","every call reaches the backend");
                        return true;
                    });
                    // A hang guard only; arrival events, rather than elapsed time, prove concurrency.
                    JavaApiCases.check(arrivals.get(15,TimeUnit.SECONDS),"all calls arrived");
                } finally { reader.shutdownNow(); }
            } finally { for (var call : calls) call.cancel(true); }
            for (int i = 0; i < calls.size(); i++) JavaApiCases.equal(fixture.command("closed"),"closed","every held call cancels");
        }
    }
}
