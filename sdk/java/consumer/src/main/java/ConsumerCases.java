import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ObjectNode;
import io.thinkthen.CallOptions;
import io.thinkthen.Cancel;
import io.thinkthen.Client;
import io.thinkthen.Question;
import io.thinkthen.Row;
import io.thinkthen.Rule;
import io.thinkthen.ThinkThenException;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.Executors;
import java.util.concurrent.atomic.AtomicInteger;

public final class ConsumerCases {
    public static final ObjectMapper JSON = new ObjectMapper().setDefaultPropertyInclusion(
        JsonInclude.Value.construct(JsonInclude.Include.NON_NULL, JsonInclude.Include.ALWAYS));
    private ConsumerCases() {}
    public static List<Object> records(JsonNode request) {
        var values = new ArrayList<Object>();
        request.get("records").forEach(node -> values.add(node.isTextual() ? node.textValue() : node));
        return values;
    }
    public static List<String> strings(JsonNode node) {
        var values = new ArrayList<String>();
        node.forEach(value -> values.add(value.textValue()));
        return values;
    }
    public static List<Rule> rules(JsonNode node) {
        var values = new ArrayList<Rule>();
        node.forEach(value -> values.add(JSON.convertValue(value, Rule.class)));
        return values;
    }
    public static JsonNode row(Row<?> row) throws Exception {
        ObjectNode value = JSON.createObjectNode();
        value.set("id", row.id());
        value.put("status", row.status().name().toLowerCase(java.util.Locale.ROOT));
        if (row.status() != Row.Status.FAILED) value.set("answer", JSON.valueToTree(row.answer()));
        if (row.p() != null) value.put("p", row.p());
        if (row.error() != null) value.set("error", JSON.valueToTree(row.error()));
        if (row.detail() != null) value.set("detail", row.detail());
        return JSON.readTree(JSON.writeValueAsBytes(value));
    }
    public static void equal(Object actual, Object expected, String what) {
        if (!java.util.Objects.equals(actual, expected)) throw new AssertionError(what + "\nexpected " + expected + "\nactual " + actual);
    }
    public static void check(boolean value, String what) { if (!value) throw new AssertionError(what); }
    public static void expectCode(String code, Runnable call) {
        try { call.run(); throw new AssertionError("Expected " + code); }
        catch (ThinkThenException error) { equal(error.code(), code, "native error code"); }
    }
    public static void main(String[] args) throws Exception {
        JsonNode cases = JSON.readTree(Path.of(args[0]).toFile());
        int count = 0;
        try (var client = Client.builder().adapter("stub").build()) {
            for (var sample : cases) {
                var request = sample.get("request");
                var input = records(request);
                var value = request.get("question").get("value");
                var options = request.path("options");
                var actual = JSON.createArrayNode();
                switch (request.get("function").textValue()) {
                    case "decide" -> { try (var rows = client.decideStream(value.textValue(), input)) { for (var row : rows.toList()) actual.add(row(row)); } }
                    case "choose" -> actual.add(row(client.choose(value.textValue(), input.getFirst(), strings(options.get("options")))));
                    case "tag" -> actual.add(row(client.tag(value.textValue(), input.getFirst(), strings(options.get("labels")))));
                    case "score" -> actual.add(row(client.score(value.textValue(), input.getFirst(), strings(options.get("levels")))));
                    case "find" -> actual.add(row(client.find(value.textValue(), input.getFirst())));
                    case "annotate" -> actual.add(row(client.annotate(value, input.getFirst())));
                    case "recognize" -> actual.add(row(client.recognize(input.getFirst(), strings(value))));
                    case "relate" -> actual.add(row(client.relate(input.getFirst(), rules(value))));
                    case "rank" -> { try (var rows = client.rank(value.textValue(), input)) { for (var row : rows.toList()) actual.add(row(row)); } }
                    case "filter" -> {
                        var expected = new ArrayList<Object>();
                        for (var row : sample.get("expected")) if (row.path("answer").asBoolean()) expected.add(input.get(row.get("id").intValue() - 1));
                        try (var kept = client.filter(value.textValue(), input)) {
                            var selectedInputs = kept.toList();
                            equal(selectedInputs, expected, "original filter inputs");
                            for (int i = 0; i < expected.size(); i++) check(selectedInputs.get(i) == expected.get(i), "filter preserves input object identity");
                        }
                        count++; continue;
                    }
                    default -> throw new AssertionError("Unknown fixture function");
                }
                equal(actual, sample.get("expected"), request.get("function").textValue());
                count++;
            }
            if (args.length > 1 && args[1].equals("--stub-only")) {
                equal(count, 10, "Java stub case count");
                System.out.println("Packaged Java: ten shared stub cases passed without a service");
                return;
            }
            try (var countClient = Client.builder().adapter("stub").batch(1).build()) {
                check(countClient.decide("Ready?", "alpha").status() != Row.Status.FAILED, "integer batch syntax");
            }
            var largestBatch = new java.math.BigInteger("18446744073709551615");
            try (var countClient = Client.builder().adapter("stub").batch(largestBatch).build()) {
                check(countClient != null, "unsigned native batch admission");
            }
            expectCode("usage.batch", () -> Client.builder().adapter("stub").batch(new java.math.BigInteger("18446744073709551616")).build());
            expectCode("usage.batch", () -> Client.builder().adapter("stub").batch(0.5).build());
            expectCode("usage.threshold", () -> client.decide("Ready?", "text", CallOptions.defaults().withThreshold("wrong")));
            var cancel = new Cancel();
            cancel.cancel();
            expectCode("local.cancelled", () -> client.decide("Ready?", "text", CallOptions.defaults().withCancel(cancel)));
            var row = client.decide("Ready?", Map.of("id", Map.of("nested", "original"), "body", "alpha"));
            ((ObjectNode) row.id()).put("mutated", true);
            check(!row.id().has("mutated"), "id mutation isolation");
            ((ObjectNode) row.detail()).put("mutated", true);
            check(!row.detail().has("mutated"), "detail mutation isolation");
            var annotation = client.annotate(cases.get(7).get("request").get("question").get("value"), "alpha");
            ((ObjectNode) annotation.answer().get("ready")).put("mutated", true);
            check(!annotation.answer().get("ready").has("mutated"), "annotation mutation isolation");
            var toolbox = cases.get(7).get("request").get("question").get("value");
            var selected = Question.toolbox(toolbox, "ready");
            ((ObjectNode) toolbox.get("questions").get("ready")).put("decide", "changed");
            equal(client.decide(selected, "alpha", CallOptions.defaults()).answer(), false, "question snapshots caller JSON");
            var unicodePath = Path.of(args[0]).getParent().getParent().resolve("conformance/find-unicode.json");
            var unicodeRequest = JSON.readTree(unicodePath.toFile()).get("request");
            var unicodeText = unicodeRequest.get("records").get(0).textValue();
            try (var fixture = new BackendFixture(unicodePath.toString()); var remote = Client.builder().url(fixture.base()).build()) {
                var span = remote.find(unicodeRequest.get("question").get("value").textValue(), unicodeText).answer();
                check(span != null, "unicode span is nonempty");
                equal(unicodeText.substring(span.start(), span.end()), span.text(), "UTF-16 offsets");
            }
            var sourceError = new IllegalStateException("source failed");
            Iterable<Object> broken = () -> new java.util.Iterator<>() {
                public boolean hasNext() { throw sourceError; }
                public Object next() { throw new AssertionError("No record should be requested"); }
            };
            try (var rows = client.decideStream("Ready?", broken)) { rows.toList(); throw new AssertionError("Expected source exception"); }
            catch (IllegalStateException error) { check(error == sourceError, "source exception identity"); }
            var read = new AtomicInteger();
            Iterable<String> source = () -> new java.util.Iterator<>() {
                public boolean hasNext() { return read.get() < 10000; }
                public String next() { read.incrementAndGet(); return "alpha"; }
            };
            try (var rows = client.decideStream("Ready?", source)) {
                check(rows.spliterator().trySplit() == null, "native streams never split");
            }
            check(read.get() == 0, "stream close does not consume input");
            try (var rows = client.decideStream("Ready?", source)) { check(rows.findFirst().isPresent(), "partial stream has a row"); }
            check(read.get() < 10000, "short circuit leaves input unread");
        }
        try (var monitor = new BackendFixture("unavailable");
             var client = Client.builder().url(monitor.base()).keySource("THINKTHEN_API_KEY").build()) {
            for (String invalid : new String[]{"\ud800", "\udc00"}) {
                expectCode("usage.record", () -> client.decide("Ready?", invalid));
                try (var rows = client.decideStream("Ready?", List.of(invalid))) {
                    expectCode("usage.record", () -> rows.toList());
                }
            }
            equal(monitor.command("count"), "0", "invalid Java UTF-16 never sends");
        }
        Row<Boolean> retained;
        try (var client = Client.builder().adapter("stub").build()) { retained = client.decide("Ready?", "alpha"); }
        check(retained.detail() != null, "row detail survives closed call and client");
        var closed = Client.builder().adapter("stub").build();
        closed.close();
        try { closed.decide("Ready?", "alpha"); throw new AssertionError("Expected closed client error"); }
        catch (IllegalStateException expected) { }
        var racingClient = Client.builder().adapter("stub").build();
        try (var workers = Executors.newVirtualThreadPerTaskExecutor()) {
            var start = new CountDownLatch(1);
            var opened = workers.submit(() -> { start.await(); try { racingClient.decide("Ready?", "alpha"); } catch (IllegalStateException expected) { } return null; });
            var closer = workers.submit(() -> { start.countDown(); racingClient.close(); });
            opened.get(); closer.get();
        } finally { racingClient.close(); }
        try (var unavailable = new BackendFixture("unavailable");
             var client = Client.builder().url(unavailable.base()).keySource("THINKTHEN_API_KEY").build()) {
            expectCode("backend.transport", () -> client.decide("Ready?", "alpha"));
            equal(unavailable.command("count"), "0", "unavailable reservation accepts no request");
        }
        try (var held = new BackendFixture("headers");
             var client = Client.builder().url(held.base()).keySource("THINKTHEN_API_KEY").build();
             var workers = Executors.newVirtualThreadPerTaskExecutor()) {
            var cancel = new Cancel();
            var call = workers.submit(() -> expectCode("local.cancelled", () -> client.decide("Ready?", "alpha", CallOptions.defaults().withCancel(cancel))));
            equal(held.command("arrived"), "arrived", "Java single reached held request");
            cancel.cancel();
            equal(held.command("closed"), "closed", "Java cancellation closes backend connection");
            call.get();
            equal(held.command("count"), "1", "one Java cancellation request");
        }
        try (var held = new BackendFixture("body");
             var client = Client.builder().url(held.base()).keySource("THINKTHEN_API_KEY").build();
             var workers = Executors.newVirtualThreadPerTaskExecutor()) {
            var rows = client.decideStream("Ready?", List.of("alpha"));
            var call = workers.submit(() -> {
                try {
                    var iterator = rows.iterator();
                    if (iterator.hasNext()) {
                        var cancelledRow = iterator.next();
                        equal(cancelledRow.status(), Row.Status.FAILED, "concurrent close preserves only native failed row");
                        equal(cancelledRow.error().code(), "local.cancelled", "native cancelled row code");
                    }
                }
                catch (ThinkThenException error) { equal(error.code(), "local.cancelled", "active pull observes native cancellation"); }
            });
            equal(held.command("arrived"), "arrived", "Java stream reached held body");
            rows.close();
            equal(held.command("closed"), "closed", "closing Java stream aborts active pull");
            call.get();
        }
        equal(count, 10, "shared cases count");
        System.out.println("Installed Java: ten shared stub cases and host ownership checks passed");
    }
}
