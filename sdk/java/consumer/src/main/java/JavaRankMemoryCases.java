import dev.thinkthen.*;
import java.util.List;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.stream.Stream;

/** Exercise rank through a lazy source larger than the JVM heap. */
public final class JavaRankMemoryCases {
    private JavaRankMemoryCases() {}
    public static void main(String[] args) {
        for (boolean stream : List.of(false,true)) bounded(stream);
        snapshots();
    }
    private static void bounded(boolean stream) {
        var consumed = new AtomicInteger();
        try (var client = ThinkThen.builder().backend(Backend.STUB).build();
                var input = Stream.generate(() -> consumed.incrementAndGet() + ":" + "x".repeat(65536)).limit(2048);
                var ranked = stream ? client.rank("Priority?", input, List.of("low", "high"), RankOptions.builder().top(1).build())
                    : client.rank("Priority?", (Iterable<String>) input::iterator, List.of("low", "high"), RankOptions.builder().top(1).build())) {
            var rows = ranked.peek(row -> {
                if (row.isFailed()) throw new AssertionError("Expected successful bounded rank input at position "
                        + row.position() + " after consuming " + consumed.get() + " records: " + row.row());
            }).toList();
            if (consumed.get() != 2048 || rows.size() != 1 || rows.getFirst().answer() != 1
                    || ((JsonString) rows.getFirst().item()).value().length() < 65536) throw new AssertionError("rank must retain its winning original after consuming the lazy source");
        }
        System.out.println("Lazy rank consumes 128 MiB of text with a 64 MiB JVM heap: PASS");
    }
    private static void snapshots() {
        List<Ranked<JsonValue>> saved;
        var input = new java.util.LinkedHashMap<String,Object>();
        input.put("text","alpha"); input.put("nested", List.of("value"));
        try (var client = ThinkThen.builder().backend(Backend.STUB).recordCodec(value -> "{\"custom\":true}").build();
                var rows = client.rank("Priority?", java.util.Arrays.asList(input, new Object(), null), List.of("low","high"))) {
            saved = rows.toList();
        }
        if (saved.size() != 3 || saved.stream().map(Ranked::position).distinct().count() != 3) throw new AssertionError("rank must retain every answered and failed record");
        input.put("text","changed after call and client close");
        for (var row : saved) {
            if (row.position() == 1 && !row.item().equals(JsonValue.parse("{\"text\":\"alpha\",\"nested\":[\"value\"]}"))) throw new AssertionError("JSON snapshot must outlive its input, native row, call and client");
            if (row.position() == 2 && !row.item().equals(JsonValue.parse("{\"custom\":true}"))) throw new AssertionError("custom codec payload is preserved as JSON without an inverse codec");
            if (row.position() == 3 && (!row.isFailed() || !(row.item() instanceof JsonNull))) throw new AssertionError("missing rank records remain failed with JsonNull snapshots");
        }
        try (var client = ThinkThen.builder().backend(Backend.STUB).build();
                var rows = client.rank("Priority?", List.of(new Object()), List.of("low","high"))) {
            var failed = rows.findFirst().orElseThrow();
            if (!failed.isFailed() || !(failed.item() instanceof JsonNull)) throw new AssertionError("unrepresentable records retain their failed outcome");
        }
        try (var client = ThinkThen.builder().backend(Backend.STUB).recordCodec(value -> "{").build();
                var rows = client.rank("Priority?", List.of(new Object()), List.of("low","high"))) {
            var failed = rows.findFirst().orElseThrow();
            if (!failed.isFailed() || !(failed.item() instanceof JsonNull)) throw new AssertionError("malformed codec JSON retains its failed outcome and explicit null snapshot");
        }
        System.out.println("Owned rank snapshots and failed records: PASS");
    }
}
