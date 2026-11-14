import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import dev.thinkthen.*;
import java.net.URI;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.stream.Stream;

/** Installed public Java API against the shared canonical cases. */
public final class ConformanceCases {
    private ConformanceCases() {}
    public static JsonNode error(String code) { return ConsumerCases.JSON.createObjectNode().put("error",code); }
    static Stream<? extends Row<?,?>> execute(ThinkThen client, JsonNode request) {
        return execute(client,request,ConsumerCases.JSON.nullNode());
    }
    static Stream<? extends Row<?,?>> execute(ThinkThen client, JsonNode request, JsonNode expected) {
        var records = ConsumerCases.records(request);
        var definition = request.get("question");
        String function = definition.get("function").asText();
        if (function.equals("annotate")) {
            var options = AnnotateOptions.builder().field(Pointer.of(definition.path("field").asText("")));
            if (request.hasNonNull("name")) options.name(request.get("name").asText());
            return client.annotateStream(annotations(definition.get("questions")),records,options.build());
        }
        var box = ConsumerCases.JSON.createObjectNode(); box.set("entry",definition);
        var toolbox = Toolbox.fromDefinitions(box.toString());
        return switch (function) {
            case "decide" -> client.decideStream(named(toolbox.decide("entry"),request,DecideQuestion::named),records);
            case "choose" -> client.chooseStream(named(toolbox.choose("entry"),request,ChooseQuestion::named),records);
            case "tag" -> client.tagStream(named(toolbox.tag("entry"),request,TagQuestion::named),records);
            case "score" -> client.scoreStream(named(toolbox.score("entry"),request,ScoreQuestion::named),records);
            case "filter" -> throw new AssertionError("Filter uses its public kept-record projection");
            case "rank" -> client.rank(named(toolbox.rank("entry"),request,RankQuestion::named),records);
            case "find" -> client.findStream(named(toolbox.find("entry"),request,FindQuestion::named),records);
            case "recognize" -> client.recognizeStream(named(toolbox.recognize("entry"),request,RecognizeQuestion::named),records);
            case "relate" -> client.relateStream(named(toolbox.relate("entry"),request,RelateQuestion::named),records);
            default -> throw new AssertionError("Unknown function");
        };
    }
    private static <Q> Q named(Q question, JsonNode request, java.util.function.BiFunction<Q,String,Q> rename) {
        return request.hasNonNull("name") ? rename.apply(question,request.get("name").asText()) : question;
    }
    private static AnnotationQuestions annotations(JsonNode questions) {
        var box = Toolbox.fromDefinitions(questions.toString()); var result = AnnotationQuestions.builder();
        questions.properties().forEach(entry -> {
            String name = entry.getKey();
            switch (entry.getValue().get("function").asText()) {
                case "decide" -> result.put(name,box.decide(name));
                case "choose" -> result.put(name,box.choose(name));
                case "tag" -> result.put(name,box.tag(name));
                case "score" -> result.put(name,box.score(name));
                case "find" -> result.put(name,box.find(name));
                case "recognize" -> result.put(name,box.recognize(name));
                case "relate" -> result.put(name,box.relate(name));
                default -> throw new AssertionError("Unexpected annotation function");
            }
        });
        return result.build();
    }
    static ObjectNode row(Row<?,?> row) {
        try {
            var value = (ObjectNode) ConsumerCases.JSON.readTree(row.toJson());
            value.remove("source");
            row.source().flatMap(Source::name).ifPresent(name -> value.putObject("source").put("name",name));
            stripMessages(value);
            return value;
        } catch (java.io.IOException error) { throw new java.io.UncheckedIOException(error); }
    }
    private static void stripMessages(JsonNode value) {
        if (value.isObject() && value.path("error").isObject())
            ((ObjectNode)value.get("error")).remove("message");
        for (var child : value) stripMessages(child);
    }
    static JsonNode filterResult(ThinkThen client, JsonNode request) {
        return filterResult(client,request,false);
    }
    static JsonNode filterResult(ThinkThen client, JsonNode request, boolean includeSource) {
        var records = ConsumerCases.records(request);
        var definitions = ConsumerCases.JSON.createObjectNode(); definitions.set("entry",request.get("question"));
        var question = named(Toolbox.fromDefinitions(definitions.toString()).filter("entry"),request,FilterQuestion::named);
        var result = ConsumerCases.JSON.createObjectNode();
        var kept = result.putArray("kept"); var failed = result.putArray("failed");
        try (var rows = client.filter(question, records)) {
            rows.forEach(item -> {
                ConsumerCases.check(records.stream().anyMatch(original -> original == item),"filter returns the caller's original object");
                if (item instanceof JsonValue value) {
                    try { kept.add(ConsumerCases.JSON.readTree(value.toString())); }
                    catch (java.io.IOException error) { throw new java.io.UncheckedIOException(error); }
                } else kept.add(ConsumerCases.JSON.valueToTree(item));
            });
        } catch (FailedException error) {
            error.rows().forEach(row -> {
                ConsumerCases.check(row.item() == records.get(Math.toIntExact(row.position()-1)),"failed filter row retains original object");
                ConsumerCases.check(!row.source().orElseThrow().questionIdentity().isBlank(),"failed filter row keeps source identity");
                failed.add(includeSource ? ConsumerCases.canonicalRow(row) : row(row));
            });
        }
        return result;
    }
    static JsonNode expectedFilter(JsonNode request, JsonNode expected) {
        if (!expected.isArray()) return expected;
        var result = ConsumerCases.JSON.createObjectNode();
        var kept = result.putArray("kept"); var failed = result.putArray("failed");
        for (var row : expected) {
            if (row.path("status").asText().equals("failed")) failed.add(row);
            else if (row.path("answer").asBoolean(false)) kept.add(request.get("records").get(row.get("position").intValue()-1));
        }
        return result;
    }
    private static JsonNode expected(JsonNode sample) {
        return sample.get("request").path("question").path("function").asText().equals("filter")
            ? expectedFilter(sample.get("request"),sample.get("expected")) : sample.get("expected").deepCopy();
    }
    private static void savedToolbox(Path directory) throws Exception {
        String fixture = Files.readString(directory.resolve("questions.json"));
        var loaded = Toolbox.fromJson(fixture);
        ConsumerCases.check(ValuesEqual(loaded.toJson(),fixture),"saved toolbox changed");
        ConsumerCases.check(loaded.get("ready").orElseThrow() instanceof FilterQuestion && loaded.get("missing").isEmpty(),"saved lookup");
        ConsumerCases.check(loaded.relate("names") instanceof RelateQuestion,"saved relate lost");
        try (var client = ThinkThen.builder().backend(Backend.STUB).build()) {
            var original = ChooseQuestion.of("Which topic?",List.of("billing","shipping"),ChooseOptions.builder().threshold(Threshold.cut(0.7)).field(Pointer.of("/body")).build());
            var record = java.util.Map.of("body","Bill");
            ConsumerCases.check(row(client.choose(loaded.choose("topic"),record)).equals(row(client.choose(original,record))),"loaded execution changed");
            var named = original.named("triage");
            var constructed = Toolbox.builder().put("topic",named).build();
            ConsumerCases.check(row(client.choose(constructed.choose("topic"),record)).equals(row(client.choose(named,record))),"constructed toolbox lost execution name");
            var saved = Toolbox.builder().put("topic",original).build();
            ConsumerCases.check(ValuesEqual(Toolbox.fromJson(saved.toJson()).toJson(),saved.toJson()),"constructed save lost choices");
        }
        for (var malformed : ConsumerCases.JSON.readTree(Files.readString(directory.resolve("malformed.json")))) {
            try { Toolbox.fromJson(malformed.asText()); throw new AssertionError("Accepted malformed saved toolbox"); }
            catch (ThinkThenException error) { ConsumerCases.check(error.code() == ErrorCode.USAGE_QUESTION,"saved admission code"); }
        }
    }
    private static void admissionCases(Path directory) throws Exception {
        int count = 0;
        for (var sample : ConsumerCases.JSON.readTree(Files.readString(directory.resolve("public-contract/usage-admission.json")))) {
            if (!sample.path("kind").asText().equals("admission")) continue;
            String name = sample.get("name").asText();
            String boundary = sample.get("boundary").asText();
            String expected = sample.get("message").asText();
            JsonNode input = sample.get("input");
            String route;
            switch (boundary) {
                case "settings" -> {
                    String reason = sample.path("not_applicable").path("java").asText();
                    ConsumerCases.check(!reason.isBlank(), "Unmarked Java admission skip: " + name);
                    System.out.println("Java admission: " + name + ": not applicable: " + reason);
                    count++;
                    continue;
                }
                case "toolbox" -> route = "direct: Toolbox.fromJson admits the saved toolbox unchanged";
                case "question" -> {
                    var saved = ConsumerCases.JSON.createObjectNode();
                    saved.putObject("entries").putObject("case").set("question", input);
                    input = saved;
                    ConsumerCases.check(expected.startsWith("question."), "Unexpected question diagnostic context");
                    expected = "toolbox.entries.[0].question." + expected.substring("question.".length());
                    route = "public projection: raw question inside a saved Toolbox.fromJson entry";
                }
                default -> throw new AssertionError("Unclassified admission boundary: " + boundary);
            }
            try { Toolbox.fromJson(input.toString()); throw new AssertionError("Accepted admission case: " + name); }
            catch (ThinkThenException error) {
                ConsumerCases.equal(error.code().wireName(), sample.get("code").asText(), "admission code: " + name);
                ConsumerCases.equal(error.getMessage(), expected, "admission message: " + name);
                ConsumerCases.check(!error.getMessage().contains("secret-placeholder"), "admission leaked a secret: " + name);
            }
            System.out.println("Java admission: " + name + ": " + route);
            count++;
        }
        ConsumerCases.check(count > 0, "No Java admission cases classified");
    }
    private static boolean ValuesEqual(String left, String right) throws Exception { return ConsumerCases.JSON.readTree(left).equals(ConsumerCases.JSON.readTree(right)); }
    // Feature: the asynchronous public facade preserves the shared named evidence case.
    private static void asyncCase(Path directory, JsonNode manifest) throws Exception {
        var file = directory.resolve("decide-field-name.json");
        var sample = ConsumerCases.JSON.readTree(file.toFile());
        var request = sample.get("request");
        var definitions = ConsumerCases.JSON.createObjectNode();
        definitions.set("entry",request.get("question"));
        var question = Toolbox.fromDefinitions(definitions.toString()).decide("entry").named(request.get("name").asText());
        for (var backend : manifest.get("backends")) {
            try (var fixture = new BackendFixture(file.toString());
                 var client = ThinkThen.builder().backend(Backend.valueOf(backend.asText().toUpperCase(java.util.Locale.ROOT)))
                     .url(URI.create(fixture.base())).apiKey("synthetic-key").model(manifest.get("client").get("model").asText())
                     .retry(RetryPolicy.builder().maxRetries(0).build()).build()) {
                var result = client.async().decide(question,ConsumerCases.records(request).getFirst()).get(30,java.util.concurrent.TimeUnit.SECONDS);
                var source = result.source().orElseThrow();
                ConsumerCases.equal(source.backend().name().toLowerCase(java.util.Locale.ROOT),backend.asText(),"async backend");
                ConsumerCases.equal(source.model(),manifest.get("client").get("model").asText(),"async model");
                ConsumerCases.check(!source.questionIdentity().isEmpty(),"async question identity");
                var actual = ConsumerCases.JSON.createArrayNode().add(row(result));
                ConsumerCases.equal(ConsumerCases.JSON.readTree(ConsumerCases.JSON.writeValueAsBytes(actual)),expected(sample),"shared async named field rows");
                ConsumerCases.equal(fixture.command("count"),sample.get("requests").asText(),"async request count");
            }
        }
    }

    public static void main(String[] args) throws Exception {
        if (args[0].equals("abi")) {
            boolean accept = Boolean.parseBoolean(args[1]);
            try (var client = ThinkThen.builder().backend(Backend.STUB).build()) {
                ConsumerCases.check(accept,"Accepted incompatible ABI"); client.decide("Ready?","alpha");
            } catch (ThinkThenException error) { ConsumerCases.check(!accept && error.code() == ErrorCode.USAGE_ABI,"ABI refusal"); }
            return;
        }
        if (args.length == 2 && args[1].equals("--entry-forms")) {
            var directory = Path.of(args[0]);
            var rankCase = ConsumerCases.JSON.readTree(directory.resolve("usage-annotation-rank.json").toFile());
            String reason = rankCase.path("not_applicable").path("java").asText();
            ConsumerCases.check(!reason.isBlank(),"Unmarked Java rank-inside-annotate case");
            JavaApiCases.requiredSignatures();
            System.out.println("Java rank inside annotate: public compiler refusal: " + reason);
            OptionCases.main(new String[]{directory.getParent().getParent().resolve("construction/options.json").toString()});
            asyncCase(directory,ConsumerCases.JSON.readTree(directory.resolve("manifest.json").toFile()));
            return;
        }
        admissionCases(Path.of(args[0]).getParent());
        Path directory = Path.of(args[0]); savedToolbox(directory.getParent().resolve("toolbox")); var manifest = ConsumerCases.JSON.readTree(directory.resolve("manifest.json").toFile()); int count=0; int notApplicable=0;
        for (var backend : manifest.get("backends")) for (var name : manifest.get("cases")) {
            var file = directory.resolve(name.asText()); var sample = ConsumerCases.JSON.readTree(file.toFile());
            String reason = sample.path("not_applicable").path("java").asText();
            if (!reason.isBlank()) {
                System.out.println("Java canonical " + name.asText() + ": not applicable: " + reason);
                notApplicable++;
                continue;
            }
            try (var fixture = new BackendFixture(file.toString())) {
                JsonNode actual;
                var values = ConsumerCases.JSON.createArrayNode();
                boolean cancelled = sample.path("cancelled").asBoolean(false);
                try (var client = ThinkThen.builder().backend(Backend.valueOf(backend.asText().toUpperCase(java.util.Locale.ROOT))).url(URI.create(fixture.base())).apiKey("synthetic-key").model(manifest.get("client").get("model").asText())
                     .batch(sample.path("client").has("batch") ? sample.get("client").get("batch").intValue() : manifest.get("client").get("batch").intValue()).retry(RetryPolicy.builder().maxRetries(sample.path("client").path("retry").path("max_retries").asInt(manifest.path("client").path("retry").path("max_retries").asInt(0))).build()).recordCodec(value -> { try { return ConsumerCases.JSON.writeValueAsString(value); } catch (java.io.IOException e) { throw new java.io.UncheckedIOException(e); } }).build()) {
                    if (cancelled) Thread.currentThread().interrupt();
                    if (sample.get("request").path("question").path("function").asText().equals("filter")) {
                        actual = filterResult(client,sample.get("request"));
                    } else {
                    try (var rows = execute(client,sample.get("request"),sample.get("expected"))) {
                        for (var row : rows.toList()) {
                            ConsumerCases.equal(row.source().orElseThrow().backend().name().toLowerCase(java.util.Locale.ROOT),backend.asText(),"typed backend");
                            ConsumerCases.equal(row.source().orElseThrow().model(),"fixture-model","typed model");
                            ConsumerCases.check(!row.source().orElseThrow().questionIdentity().isEmpty(),"question identity");
                            ConsumerCases.equal(row.source().orElseThrow().name().orElse(null),sample.get("request").path("name").isTextual() ? sample.get("request").get("name").asText() : null,"execution name"); values.add(row(row));
                        }
                    }
                    actual = values;
                    }
                } catch (ThinkThenException error) {
                    ConsumerCases.check(values.isEmpty(), "Admission returned rows");
                    var failure = (ObjectNode) error(error.code().wireName());
                    error.httpStatus().ifPresent(status -> failure.put("http_status",status));
                    actual = failure;
                } catch (java.util.concurrent.CancellationException error) {
                    ConsumerCases.check(cancelled && values.isEmpty(), "Unexpected cancellation or cancelled rows");
                    ConsumerCases.check(error.getCause() instanceof ThinkThenException cause && cause.code() == ErrorCode.LOCAL_CANCELLED, "Cancellation lost its engine code");
                    actual = error("local.cancelled");
                } finally { Thread.interrupted(); }
                ConsumerCases.equal(ConsumerCases.JSON.readTree(ConsumerCases.JSON.writeValueAsBytes(actual)),expected(sample),"canonical "+backend.asText()+"/"+name.asText());
                ConsumerCases.equal(fixture.command("count"),sample.get("requests").asText(),"canonical request count"); count++;
            }
        }
        asyncCase(directory,manifest);
        ConsumerCases.check(count>0,"No Java canonical cases");
        System.out.println("Java public typed API: "+count+" canonical backend cases passed; "+notApplicable+" not applicable");
    }
}
