import io.thinkthen.*;
import com.fasterxml.jackson.databind.JsonNode;
import java.net.URI;
import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.ArrayList;
import java.util.Iterator;
import java.util.concurrent.CancellationException;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;

/** Public API stress adapter. The shared runner owns workload selection and fixtures. */
public final class StressCases {
    private StressCases() {}

    static void check(Path directory) throws Exception {
        var command = new ArrayList<String>();
        command.add("python3"); command.add(directory.resolve("../scripts/stress-adapters.py").normalize().toString()); command.add("--surface"); command.add("java"); command.add("--");
        command.add(Path.of(System.getProperty("java.home"),"bin","java").toString());
        command.add("-Duser.home="+System.getProperty("user.home"));
        command.add("--enable-native-access=ALL-UNNAMED"); command.add("-cp"); command.add(System.getProperty("java.class.path")); command.add("StressCases");
        ConsumerCases.check(new ProcessBuilder(command).inheritIO().start().waitFor() == 0,"Java stress adapter failed");
    }
    private static Iterable<JsonValue> records(int count, int call, int basis) {
        return () -> new Iterator<>() {
            private int next;
            public boolean hasNext() { return next < count; }
            public JsonValue next() { int i=next++;return JsonValue.parse("{\"id\":\"call-"+call+"-record-"+(basis+i)+"\",\"body\":\"record "+i+"\"}"); }
        };
    }
    private static long consume(ThinkThen client, JsonNode sample, int call, java.util.concurrent.ConcurrentSkipListSet<Integer> widths) {
        int received=0; long failed=0;
        int basis=sample.get("identifier_base").intValue();
        call += basis;
        try (var stream = client.decideStream("Ready?", records(sample.get("records").intValue(), call, basis))) {
            var iterator=stream.iterator();
            while (iterator.hasNext()) {
                var value=ConformanceCases.row(iterator.next());
                widths.add(value.get("id").asText().length());
                ConsumerCases.check(value.get("position").intValue()==++received,"stress position changed");
                ConsumerCases.check(value.get("id").asText().equals("call-"+call+"-record-"+(basis+received-1)),"stress id changed");
                if (value.get("status").asText().equals("failed")) {
                    failed++;
                    ConsumerCases.equal(value.get("error").get("code"),sample.get("scenario").get("code"),"stress error changed");
                } else {
                    ConsumerCases.equal(value.get("status").asText(),"answered","stress status changed");
                    ConsumerCases.check(value.get("answer").asBoolean(),"stress answer changed");
                    ConsumerCases.equal(value.get("probability"),sample.get("probability"),"stress probability changed");
                }
                if (sample.get("scenario").get("status").asText().equals("answered") || sample.get("scenario").get("name").asText().equals("refused")) ConsumerCases.equal(value.get("status"),sample.get("scenario").get("status"),"stress status changed");
            }
        }
        ConsumerCases.check(received==sample.get("records").intValue() && (failed>0)==sample.get("scenario").get("status").asText().equals("failed"),"stress lost rows or failures");
        return failed;
    }
    private static void cancel(ThinkThen client, JsonNode sample) throws Exception {
        var stream = client.decideStream("Ready?", records(sample.get("records").intValue(), 0, 0));
        try (var pool=Executors.newVirtualThreadPerTaskExecutor()) {
            var pending=pool.submit(() -> { try { stream.findFirst();throw new AssertionError("cancelled stream returned a row"); } catch(CancellationException expected) {
                ConsumerCases.check(expected.getCause() instanceof ThinkThenException cause && cause.code()==ErrorCode.LOCAL_CANCELLED,"cancel lost engine code");
            } });
            var input=new java.io.BufferedReader(new java.io.InputStreamReader(System.in,java.nio.charset.StandardCharsets.UTF_8));
            ConsumerCases.equal(input.readLine(),"cancel","stress control event");
            stream.close(); pending.get();
        } finally { stream.close(); }
    }
    public static void main(String[] args) throws Exception {
        var sample=ConsumerCases.JSON.readTree(Files.readString(Path.of(System.getenv("THINKTHEN_STRESS_CASE"))));
        var c=sample.get("client");var r=c.get("retry");
        long calls=0,rows=0,failed=0;
        var widths=new java.util.concurrent.ConcurrentSkipListSet<Integer>();boolean cancelled=sample.get("cancel").asBoolean();
        try(var client=ThinkThen.builder().backend(Backend.valueOf(c.get("backend").asText().toUpperCase(java.util.Locale.ROOT)))
            .model(c.get("model").asText()).batch(c.get("batch").intValue()).url(URI.create(c.get("url").asText())).apiKey(c.get("api_key").asText())
            .retry(RetryPolicy.builder().maxRetries(r.get("max_retries").intValue()).initialBackoff(Duration.ofMillis(r.get("initial_backoff_ms").longValue())).attemptTimeout(Duration.ofMillis(r.get("attempt_timeout_ms").longValue())).build()).build()) {
            if(cancelled) { cancel(client,sample); calls=1; }
            else try(var pool=Executors.newVirtualThreadPerTaskExecutor()) {
                for(int round=0;round<sample.get("rounds").intValue();round++) {
                    var results=new ArrayList<Future<Long>>();
                    for(int i=0;i<sample.get("concurrency").intValue();i++) {
                        int call=round*sample.get("concurrency").intValue()+i;
                        results.add(pool.submit(() -> consume(client,sample,call,widths)));
                    }
                    for(var result:results) { failed+=result.get();calls++;rows+=sample.get("records").intValue(); }
                    var hook=System.getenv("THINKTHEN_STRESS_CHECKPOINT_COMMAND");
                    if(hook != null) ConsumerCases.check(new ProcessBuilder("sh","-c",hook).inheritIO().start().waitFor()==0,"resource checkpoint failed");
                }
            }
        }
        System.out.println(ConsumerCases.JSON.createObjectNode().put("calls",calls).put("rows",rows).put("failed",failed).put("cancelled",cancelled).put("identifier_width_mask",widths.stream().mapToLong(width -> 1L << width).reduce(0L, (left,right) -> left | right)));
    }
}
