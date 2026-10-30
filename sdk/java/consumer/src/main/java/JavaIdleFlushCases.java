import com.fasterxml.jackson.databind.ObjectMapper;
import com.sun.net.httpserver.HttpServer;
import io.thinkthen.*;
import java.net.InetSocketAddress;
import java.net.URI;
import java.time.Duration;
import java.util.ArrayList;
import java.util.Iterator;
import java.util.concurrent.CountDownLatch;

/** Feature: an idle producer splits an open batch without losing any rows. */
public final class JavaIdleFlushCases {
    private JavaIdleFlushCases() {}

    private static void check(boolean condition, String claim) {
        if (!condition) throw new AssertionError(claim);
    }

    private static void run(int count, java.util.List<Integer> pauses, java.util.List<Integer> expectedSizes) throws Exception {
        var json = new ObjectMapper();
        var arrivals = java.util.List.of(new CountDownLatch(1), new CountDownLatch(1));
        var sizes = new ArrayList<Integer>();
        var server = HttpServer.create(new InetSocketAddress("127.0.0.1", 0), 0);
        server.createContext("/v1", exchange -> {
            try (exchange) {
                var body = json.readTree(exchange.getRequestBody());
                var questions = body.path("questions");
                synchronized (sizes) {
                    sizes.add(questions.size());
                    if (sizes.size() <= arrivals.size()) arrivals.get(sizes.size() - 1).countDown();
                }
                var reply = json.createObjectNode();
                reply.put("model", "stress-model");
                var answers = reply.putObject("answers");
                questions.fieldNames().forEachRemaining(name ->
                    answers.putObject(name).put("type", "noul").put("noul", 0.9));
                var bytes = json.writeValueAsBytes(reply);
                exchange.getResponseHeaders().set("content-type", "application/json");
                exchange.sendResponseHeaders(200, bytes.length);
                exchange.getResponseBody().write(bytes);
            }
        });
        server.start();
        try (var client = ThinkThen.builder().backend(Backend.SYSTEMONE).model("stress-model")
                .batch(32).url(URI.create("http://127.0.0.1:" + server.getAddress().getPort() + "/v1"))
                .apiKey("loopback-only").retry(RetryPolicy.builder().maxRetries(0)
                    .attemptTimeout(Duration.ofHours(1)).build()).build()) {
            Iterable<JsonValue> input = () -> new Iterator<>() {
                private int next;
                public boolean hasNext() { return next < count; }
                public JsonValue next() {
                    if (pauses.contains(next)) {
                        try {
                            // The job owns the hang deadline. Only backend
                            // arrival releases this deliberately paused input.
                            arrivals.get(pauses.indexOf(next)).await();
                        } catch (InterruptedException error) {
                            Thread.currentThread().interrupt();
                            throw new AssertionError(error);
                        }
                    }
                    int position = ++next;
                    return JsonValue.parse("{\"id\":\"record-" + position + "\",\"body\":\"record " + position + "\"}");
                }
            };
            try (var rows = client.decideStream("Ready?", input)) {
                var values = rows.toList();
                check(values.size() == count, "idle split lost rows");
                for (int index = 0; index < values.size(); index++) {
                    var row = values.get(index);
                    check(row.position() == index + 1, "idle split changed row position");
                    check(row.id().orElseThrow().equals(JsonValue.parse("\"record-" + (index + 1) + "\"")), "idle split changed record ID");
                    check(row instanceof Answered<Boolean, NoDetail> answer && answer.answer()
                        && answer.probability().orElseThrow() == 0.9, "idle split changed the answer");
                }
            }
            synchronized (sizes) {
                check(sizes.equals(expectedSizes),
                    "unexpected backend pack sizes: " + sizes);
                System.out.println("pauses=" + pauses + " rows=" + count + " request_sizes=" + sizes);
            }
        } finally { server.stop(0); }
    }

    public static void main(String[] args) throws Exception {
        run(32, java.util.List.of(), java.util.List.of(32));
        run(32, java.util.List.of(1), java.util.List.of(1, 31));
        run(64, java.util.List.of(20, 40), java.util.List.of(20, 20, 24));
    }
}
