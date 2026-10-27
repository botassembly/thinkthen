import io.thinkthen.*;
import consumer.StreamThreadAudit;
import java.util.List;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

/** Feature: streams reuse one driver thread and keep independent calls concurrent. */
@SuppressWarnings("try")
public final class JavaStreamThreadCases {
    private JavaStreamThreadCases() {}
    public static void main(String[] args) throws Exception {
        try (var client = ThinkThen.builder().backend(Backend.STUB).batch(1).build()) {
            var records = java.util.Collections.nCopies(1000,"alpha");
            for (Iterable<String> input : List.of(records, (Iterable<String>) records::iterator)) {
                try (var audit = new StreamThreadAudit(); var rows = client.decideStream("Ready?",input)) {
                    if (rows.count() != 1000) throw new AssertionError("Lost stream rows");
                }
            }
            var arrived = new CountDownLatch(100);
            var release = new CountDownLatch(1);
            try (var consumers = Executors.newFixedThreadPool(100)) {
                var results = new java.util.ArrayList<java.util.concurrent.Future<Long>>();
                try {
                    for (int i=0; i<100; i++) results.add(consumers.submit(() -> {
                        try (var rows = client.decideStream("Ready?",List.of("alpha","beta"))) {
                            return rows.peek(row -> {
                                arrived.countDown();
                                try { release.await(); }
                                catch (InterruptedException error) { throw new RuntimeException(error); }
                            }).count();
                        }
                    }));
                    if (!arrived.await(30,TimeUnit.SECONDS)) throw new AssertionError("Streams did not all produce a row");
                } finally { release.countDown(); }
                for (var result : results) if (result.get() != 2) throw new AssertionError("Concurrent stream lost rows");
            }
        }
        System.out.println("Java stream thread audit and 100 concurrent streams pass");
    }
}
