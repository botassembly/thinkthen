import dev.thinkthen.*;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.stream.Stream;

/** Ranking retains caller objects through lazy input and after the client closes. */
public final class JavaRankMemoryCases {
    private JavaRankMemoryCases() {}
    public static void main(String[] args) {
        for (boolean stream : List.of(false,true)) retained(stream);
        originals();
    }
    private static void retained(boolean stream) {
        var consumed = new AtomicInteger();
        var originals = new ArrayList<String>();
        List<Row<Long,String>> saved;
        try (var client = ThinkThen.builder().backend(Backend.STUB).build();
             var input = Stream.generate(() -> {
                 String value = consumed.incrementAndGet() + ":" + "x".repeat(1024);
                 originals.add(value);
                 return value;
             }).limit(64);
             var ranked = stream ? client.rank("Priority?",input,List.of("low","high"),RankOptions.builder().top(1).build())
                 : client.rank("Priority?",(Iterable<String>)input::iterator,List.of("low","high"),RankOptions.builder().top(1).build())) {
            saved = ranked.toList();
        }
        if (consumed.get() != 64 || saved.size() != 1 || saved.getFirst().answer() != 1)
            throw new AssertionError("rank consumes all lazy records before returning its top result");
        var first = saved.getFirst();
        if (first.item() != originals.get(Math.toIntExact(first.position()-1)))
            throw new AssertionError("rank result retains the caller's original after call and client close");
    }
    private static void originals() {
        List<Row<Long,Object>> saved;
        var input = new java.util.LinkedHashMap<String,Object>();
        input.put("text","alpha"); input.put("nested",List.of("value"));
        var custom = new Object();
        List<Object> originals = java.util.Arrays.asList(input,custom,null);
        try (var client = ThinkThen.builder().backend(Backend.STUB).recordCodec(value -> "{\"custom\":true}").build();
             var rows = client.rank("Priority?",originals,List.of("low","high"))) {
            saved = rows.toList();
        }
        if (saved.size() != 3 || saved.stream().map(Row::position).distinct().count() != 3)
            throw new AssertionError("rank retains every answered and failed record");
        var hashes = saved.stream().map(Row::hashCode).toList();
        input.put("text","changed after call and client close");
        for (int i=0;i<saved.size();i++) {
            var row = saved.get(i);
            if (row.item() != originals.get(Math.toIntExact(row.position()-1)))
                throw new AssertionError("rank must retain each original object without snapshot substitution");
            if (row.hashCode() != hashes.get(i)) throw new AssertionError("mutable originals cannot change result hashing");
            if (row.position() == 3 && !row.isFailed()) throw new AssertionError("missing original remains failed");
        }
        var unavailable = new Object();
        try (var client = ThinkThen.builder().backend(Backend.STUB).build();
             var rows = client.rank("Priority?",List.of(unavailable),List.of("low","high"))) {
            var failed = rows.findFirst().orElseThrow();
            if (!failed.isFailed() || failed.item() != unavailable)
                throw new AssertionError("conversion failures retain the unavailable caller object");
        }
        try (var client = ThinkThen.builder().backend(Backend.STUB).recordCodec(value -> "{").build();
             var rows = client.rank("Priority?",List.of(custom),List.of("low","high"))) {
            var failed = rows.findFirst().orElseThrow();
            if (!failed.isFailed() || failed.item() != custom)
                throw new AssertionError("malformed codec output keeps its failed original");
        }
        System.out.println("Owned rank originals and failed records: PASS");
    }
}
