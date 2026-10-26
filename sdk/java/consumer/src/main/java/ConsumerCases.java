import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ObjectNode;
import io.thinkthen.internal.CallOptions;
import io.thinkthen.internal.Cancel;
import io.thinkthen.internal.Client;
import io.thinkthen.internal.Question;
import io.thinkthen.internal.Row;
import io.thinkthen.internal.Rule;
import io.thinkthen.internal.ThinkThenException;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.stream.Stream;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.Executors;
import java.util.concurrent.atomic.AtomicInteger;

public final class ConsumerCases {
    public static final ObjectMapper JSON = new ObjectMapper().setDefaultPropertyInclusion(
        JsonInclude.Value.construct(JsonInclude.Include.NON_NULL, JsonInclude.Include.ALWAYS));
    private ConsumerCases() {}
    public static List<Object> records(JsonNode request) {
        var values = new ArrayList<Object>();
        if (!request.has("records")) return values;
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
    public static JsonNode canonicalRow(Row<?> row) throws Exception {
        ObjectNode value = JSON.createObjectNode();
        value.put("function", row.function());
        value.put("position", row.position());
        value.put("status", row.status().name().toLowerCase(java.util.Locale.ROOT));
        if (!row.id().isMissingNode()) value.set("id", row.id());
        if (row.status() == Row.Status.ANSWERED) value.set("answer", JSON.valueToTree(row.answer()));
        if (row.probability() != null) value.put("probability", row.probability());
        if (!row.source().isMissingNode()) value.set("source", row.source());
        if (row.error() != null) value.set("error", JSON.valueToTree(row.error()));
        if (row.detail() != null) value.set("detail", row.detail());
        return JSON.readTree(JSON.writeValueAsBytes(value));
    }
    public static JsonNode fixtureRow(Row<?> row) throws Exception {
        var value = canonicalRow(row);
        ((ObjectNode) value).remove("source");
        stripMessages(value);
        return value;
    }
    private static void stripMessages(JsonNode value) {
        if (value.isObject() && value.path("error").isObject()) ((ObjectNode)value.get("error")).remove("message");
        for (var child : value) stripMessages(child);
    }
    public static void equal(Object actual, Object expected, String what) {
        if (!java.util.Objects.equals(actual, expected)) throw new AssertionError(what + "\nexpected " + expected + "\nactual " + actual);
    }
    public static void check(boolean value, String what) { if (!value) throw new AssertionError(what); }
    public static void expectCode(String code, Runnable call) {
        try { call.run(); throw new AssertionError("Expected " + code); }
        catch (ThinkThenException error) { equal(error.code(), code, "native error code"); }
        catch (java.util.concurrent.CancellationException error) { equal(code, "local.cancelled", "standard JVM cancellation"); }
    }
    static Stream<? extends Row<?>> execute(Client client, JsonNode request) {
        var records = ConsumerCases.records(request);
        var toolbox = ConsumerCases.JSON.createObjectNode();
        toolbox.set("entry", request.get("question"));
        var question = Question.toolbox(toolbox, "entry");
        var options = CallOptions.defaults();
        return switch (request.get("question").get("function").asText()) {
            case "decide" -> client.decideStream(question, records, options);
            case "choose" -> client.chooseStream(question, records, options);
            case "tag" -> client.tagStream(question, records, options);
            case "score" -> client.scoreStream(question, records, options);
            case "filter" -> client.filterRows(question, records, options);
            case "rank" -> client.rank(question, records, options);
            case "find" -> client.findStream(question, records, options);
            case "recognize" -> client.recognizeStream(question, records, options);
            case "relate" -> client.relateStream(question, records, options);
            case "annotate" -> client.annotateStream(question, records, options);
            default -> throw new AssertionError("Unknown canonical function");
        };
    }
    public static void main(String[] args) throws Exception {
        JsonNode cases = JSON.readTree(Path.of(args[0]).toFile());
        int count = 0;
        try (var client = Client.builder().adapter("stub").build()) {
            for (var sample : cases) {
                var request = sample.get("request");
                var input = records(request);
                var definition = request.get("question");
                var toolbox = JSON.createObjectNode();
                toolbox.set("entry", definition);
                var question = Question.toolbox(toolbox, "entry");
                var options = CallOptions.defaults();
                var actual = JSON.createArrayNode();
                String function = definition.get("function").asText();
                try (var rows = execute(client, request)) {
                    for (var row : rows.toList()) actual.add(canonicalRow(row));
                }
                equal(actual, sample.get("expected"), function);
                if (function.equals("filter")) {
                    var expected = new ArrayList<Object>();
                    for (var row : sample.get("expected")) if (row.path("answer").asBoolean()) expected.add(input.get(row.get("position").intValue() - 1));
                    try (var kept = client.filter(question, input, options)) {
                        var selectedInputs = kept.toList();
                        equal(selectedInputs, expected, "original filter inputs");
                        for (int i = 0; i < expected.size(); i++) check(selectedInputs.get(i) == expected.get(i), "filter preserves input object identity");
                    }
                    if (!(args.length > 1 && args[1].equals("--stub-only"))) {
                        var originals = List.of(JSON.createObjectNode().put("body", "refund"), JSON.createObjectNode().put("body", "parcel"));
                        var filterPath = Path.of(args[0]).getParent().getParent().resolve("conformance/typed/filter-rows.json");
                        try (var fixture = new BackendFixture(filterPath.toString());
                             var remote = Client.builder().url(fixture.base()).apiKey("synthetic-key").build();
                             var kept = remote.filter(question, originals, options.withOn("/body"))) {
                            var selectedInputs = kept.toList();
                            equal(selectedInputs.size(), 1, "canonical filter identity witness keeps refund");
                            check(selectedInputs.getFirst() == originals.getFirst(), "filter retains the mutable input object");
                        }
                    }
                }
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
            var record = JSON.createObjectNode().put("id", "original").put("body", "alpha");
            var row = client.choose("Which?", record, List.of("first", "second"));
            record.put("id", "changed");
            equal(row.id().asText(), "original", "id mutation isolation");
            ((ObjectNode) row.detail()).put("mutated", true);
            check(!row.detail().has("mutated"), "detail mutation isolation");
            JsonNode annotationRequest = null;
            for (var sample : cases) if (sample.path("request").path("question").path("function").asText().equals("annotate")) annotationRequest = sample.get("request");
            check(annotationRequest != null, "construction corpus includes annotation");
            var annotationDefinition = annotationRequest.get("question");
            var wrapper = JSON.createObjectNode();
            wrapper.set("entry", annotationDefinition);
            var annotation = client.annotate(Question.toolbox(wrapper, "entry"), records(annotationRequest).getFirst(), CallOptions.defaults());
            ((ObjectNode) annotation.answer().get("decision")).put("mutated", true);
            check(!annotation.answer().get("decision").has("mutated"), "annotation mutation isolation");
            var toolbox = annotationDefinition.get("questions").deepCopy();
            var selected = Question.toolbox(toolbox, "decision");
            var beforeMutation = canonicalRow(client.decide(selected, "alpha", CallOptions.defaults()));
            ((ObjectNode) toolbox.get("decision")).put("question", "changed");
            equal(canonicalRow(client.decide(selected, "alpha", CallOptions.defaults())), beforeMutation, "question snapshots caller JSON and execution identity");
            var unicodePath = Path.of(args[0]).getParent().getParent().resolve("conformance/typed/find-unicode.json");
            var unicodeRequest = JSON.readTree(unicodePath.toFile()).get("request");
            var unicodeText = unicodeRequest.get("records").get(0).textValue();
            try (var fixture = new BackendFixture(unicodePath.toString()); var remote = Client.builder().url(fixture.base()).build()) {
                var span = remote.find(unicodeRequest.get("question").get("question").textValue(), unicodeText).answer();
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
             var client = Client.builder().url(monitor.base()).backend("systemone").apiKey("synthetic-key").build()) {
            for (String invalid : new String[]{"\ud800", "\udc00"}) {
                equal(client.decide("Ready?", invalid).error().code(), "record.invalid", "invalid single UTF-16 stays a failed row");
                try (var rows = client.decideStream("Ready?", List.of(invalid))) {
                    var failures = rows.toList();
                    equal(failures.size(), 1, "invalid UTF-16 has one row");
                    equal(failures.getFirst().error().code(), "record.invalid", "invalid streamed UTF-16 stays a failed row");
                }
            }
            equal(monitor.command("count"), "0", "invalid Java UTF-16 never sends");
        }
        Row<Boolean> retained;
        try (var client = Client.builder().adapter("stub").build()) { retained = client.decide("Ready?", "alpha"); }
        check(retained.source().has("question_identity"), "row source survives closed call and client");
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
             var client = Client.builder().url(unavailable.base()).backend("systemone").apiKey("synthetic-key").build()) {
            equal(client.decide("Ready?", "alpha").error().code(), "backend.transport", "transport failure stays a failed row");
            equal(unavailable.command("count"), "0", "unavailable reservation accepts no request");
        }
        try (var held = new BackendFixture("headers");
             var client = Client.builder().url(held.base()).backend("systemone").apiKey("synthetic-key").build();
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
             var client = Client.builder().url(held.base()).backend("systemone").apiKey("synthetic-key").build();
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
                catch (java.util.concurrent.CancellationException expected) { }
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
