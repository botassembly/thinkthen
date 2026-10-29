import io.thinkthen.*;
import java.net.URI;
import java.time.Duration;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.nio.file.Path;

public final class JavaSettingsCases {
    private JavaSettingsCases() {}
    static void snapshotsAndExecutionName() {
        var choices = new ArrayList<>(List.of("a","b"));
        var question = ChooseQuestion.of("Which?",choices,ChooseOptions.builder().name("named").build());
        choices.clear();
        try (var client = ThinkThen.builder().backend(Backend.STUB).retry(RetryPolicy.builder().maxRetries(0).initialBackoff(Duration.ZERO).attemptTimeout(Duration.ofSeconds(3)).build()).build()) {
            var first = JavaApiCases.answered(client.choose(question,"alpha"));
            var unnamed = JavaApiCases.answered(client.choose("Which?","alpha",List.of("a","b")));
            var second = JavaApiCases.answered(client.choose("Which?","alpha",List.of("a","b"),ChooseOptions.builder().name("second").build()));
            JavaApiCases.equal(first.answer(),unnamed.answer(),"execution name does not change the answer");
            JavaApiCases.equal(first.answer(),second.answer(),"another execution name retains the answer");
            JavaApiCases.check(!first.source().orElseThrow().questionIdentity().equals(second.source().orElseThrow().questionIdentity()),"execution name changes question identity");
            JavaApiCases.equal(first.source().orElseThrow().name().orElseThrow(),"named","source retains explicit execution name");
            var toolbox = Toolbox.builder().put("entry",ChooseQuestion.of("Which?",List.of("a","b"))).build();
            var selected = JavaApiCases.answered(client.choose(toolbox.choose("entry").orElseThrow(),"alpha"));
            JavaApiCases.equal(selected.source(),unnamed.source(),"selecting an entry assigns no execution name");
            var renamed = JavaApiCases.answered(client.choose(toolbox.choose("entry").orElseThrow().named("selected"),"alpha"));
            JavaApiCases.equal(renamed.source().orElseThrow().name().orElseThrow(),"selected","selected typed questions accept explicit execution names");
            try (var rows = client.rank("Priority?",List.of("alpha","omega"),List.of("low","high"),RankOptions.builder().name("ranked").build())) {
                var output = rows.toList();
                JavaApiCases.equal(output.size(),2,"named rank keeps every input");
                for (var item : output) JavaApiCases.equal(item.row().source().orElseThrow().name().orElseThrow(),"ranked","rank source retains execution name");
            }
            JavaApiCases.equal(first.detail().orElseThrow().labels().size(),2,"question snapshots mutable choices");
            JavaApiCases.failed(client.decide("Ready?",Map.of("body","\ud800")),"record.invalid");
            JavaApiCases.failed(client.decide("Ready?",Map.of(42,"alpha")),"record.invalid");
            var exact = client.decide("Ready?",Map.of("id",new java.math.BigInteger("18446744073709551615"),"body","alpha"));
            JavaApiCases.equal(exact.id().orElseThrow(),JsonValue.parse("18446744073709551615"),"host map keeps unsigned ID exact");
        }
        try { ThinkThen.builder().backend(Backend.STUB).retry(RetryPolicy.builder().initialBackoff(Duration.ofNanos(1)).build()).build().close(); throw new AssertionError("fractional engine milliseconds must fail admission"); }
        catch (ThinkThenException error) { JavaApiCases.equal(error.code(),ErrorCode.USAGE_SETTINGS,"duration admission retains engine code"); }
    }
    // Feature: typed retry settings enforce the engine-owned published bound.
    static void retryBounds(Path contract) throws Exception {
        var schema = ConsumerCases.JSON.readTree(contract.resolveSibling("client.schema.json").toFile());
        int maximum = schema.at("/$defs/Retry/properties/max_retries/maximum").intValue();
        JavaApiCases.check(maximum > 0, "Missing published retry bound");
        try (var valid = ThinkThen.builder().backend(Backend.STUB).retry(RetryPolicy.builder().maxRetries(maximum).build()).build()) {
            valid.decide("Ready?","alpha");
        }
        for (int retries : new int[] {-1, maximum + 1}) {
            try (var invalid = ThinkThen.builder().backend(Backend.STUB).retry(RetryPolicy.builder().maxRetries(retries).build()).build()) {
                invalid.decide("Ready?","alpha"); throw new AssertionError("Accepted retries outside the engine bound");
            } catch (ThinkThenException error) { JavaApiCases.equal(error.code(),ErrorCode.USAGE_SETTINGS,"retry bound code"); }
        }
    }
    static void registry(Path contract) throws Exception {
        var codes = io.thinkthen.internal.DriverClient.codes();
        JavaApiCases.equal(ErrorCode.values().length,codes.size(),"complete generated error registry");
        for (var entry : codes) {
            var code = ErrorCode.fromWire(entry.name());
            JavaApiCases.equal(code.value(),entry.value(),"native code number");
            JavaApiCases.equal(code.family().name().toLowerCase(java.util.Locale.ROOT),entry.family(),"native code family");
        }
    }
    // Feature: failed construction consumes the builder's direct API key.
    static void failedBuildClearsKey() {
        var builder = ThinkThen.builder().backend(Backend.OPENAI).apiKey("synthetic-builder-key").url(URI.create("http://example.invalid"));
        try { builder.build().close(); throw new AssertionError("unsafe URL accepted"); }
        catch (ThinkThenException expected) { JavaApiCases.equal(expected.code().wireName(),"usage.url","initial admission failure"); }
        try { builder.url(URI.create("https://example.invalid")).build().close(); throw new AssertionError("failed builder retained direct API key"); }
        catch (ThinkThenException expected) { JavaApiCases.equal(expected.code().wireName(),"usage.key-missing","builder forgets direct API key on failure"); }
    }
    public static void main(String[] args) throws Exception {
        if (args.length == 1 && args[0].equals("cleared-key")) { failedBuildClearsKey(); return; }
        var child = new ProcessBuilder(Path.of(System.getProperty("java.home"),"bin/java").toString(),
            "--enable-native-access=ALL-UNNAMED", "-cp", System.getProperty("java.class.path"), "JavaSettingsCases", "cleared-key");
        child.environment().remove("THINKTHEN_API_KEY"); child.environment().remove("OPENAI_API_KEY");
        var process = child.inheritIO().start();
        JavaApiCases.equal(process.waitFor(),0,"failed construction clears builder key");
        snapshotsAndExecutionName(); retryBounds(Path.of(args[0])); registry(Path.of(args[0]));
    }
}
