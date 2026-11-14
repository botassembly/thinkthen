import dev.thinkthen.*;
import java.lang.management.ManagementFactory;
import java.util.Iterator;
import java.util.stream.IntStream;

/** Feature: an iterator owns its active stream even after the stream is discarded. */
public final class JavaStreamReachabilityCases {
    private JavaStreamReachabilityCases() {}

    private static long collections() {
        return ManagementFactory.getGarbageCollectorMXBeans().stream()
            .mapToLong(bean -> Math.max(0, bean.getCollectionCount())).sum();
    }

    public static void main(String[] args) {
        long before = collections();
        int count = 64;
        try (var client = ThinkThen.builder().backend(Backend.STUB).batch(1).build()) {
            Iterable<String> input = () -> IntStream.range(0, count).mapToObj(i -> "message " + i).iterator();
            // Deliberately retain only the public iterator, as an application may.
            Iterator<Row<Boolean, String>> rows = client.decideStream("Ready?", input).iterator();
            int received = 0;
            while (rows.hasNext()) {
                var row = rows.next();
                row.throwIfFailed();
                if (row.position() != ++received) throw new AssertionError("iterator lost row order");
                System.gc();
            }
            if (received != count) throw new AssertionError("iterator lost rows: " + received);
            System.out.println("Iterator delivered " + received + " rows across " + (collections() - before) + " GC collections");
        }
    }
}
