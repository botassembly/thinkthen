import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.node.ObjectNode;
import io.thinkthen.*;
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
        var records = ConsumerCases.records(request);
        var definition = request.get("question");
        String function = definition.get("function").asText();
        if (function.equals("annotate")) {
            var options = AnnotateOptions.builder().evidence(Pointer.of(definition.path("evidence").asText("")));
            if (request.hasNonNull("name")) options.name(request.get("name").asText());
            return client.annotateStream(annotations(definition.get("questions")),records,options.build());
        }
        var box = ConsumerCases.JSON.createObjectNode(); box.set("entry",definition);
        var toolbox = Toolbox.parse(box.toString());
        return switch (function) {
            case "decide" -> client.decideStream(named(toolbox.decide("entry").orElseThrow(),request,DecideQuestion::named),records);
            case "choose" -> client.chooseStream(named(toolbox.choose("entry").orElseThrow(),request,ChooseQuestion::named),records);
            case "tag" -> client.tagStream(named(toolbox.tag("entry").orElseThrow(),request,TagQuestion::named),records);
            case "score" -> client.scoreStream(named(toolbox.score("entry").orElseThrow(),request,ScoreQuestion::named),records);
            case "filter" -> client.filter(named(toolbox.filter("entry").orElseThrow(),request,FilterQuestion::named),records).map(ItemRow::row);
            case "rank" -> client.rank(named(toolbox.rank("entry").orElseThrow(),request,RankQuestion::named),records).map(ItemRow::row);
            case "find" -> client.findStream(named(toolbox.find("entry").orElseThrow(),request,FindQuestion::named),records);
            case "recognize" -> client.recognizeStream(named(toolbox.recognize("entry").orElseThrow(),request,RecognizeQuestion::named),records);
            case "relate" -> client.relateStream(named(toolbox.relate("entry").orElseThrow(),request,RelateQuestion::named),records);
            default -> throw new AssertionError("Unknown function");
        };
    }
    private static <Q> Q named(Q question, JsonNode request, java.util.function.BiFunction<Q,String,Q> rename) {
        return request.hasNonNull("name") ? rename.apply(question,request.get("name").asText()) : question;
    }
    private static AnnotationQuestions annotations(JsonNode questions) {
        var box = Toolbox.parse(questions.toString()); var result = AnnotationQuestions.builder();
        questions.properties().forEach(entry -> {
            String name = entry.getKey();
            switch (entry.getValue().get("function").asText()) {
                case "decide" -> result.put(name,box.decide(name).orElseThrow());
                case "choose" -> result.put(name,box.choose(name).orElseThrow());
                case "tag" -> result.put(name,box.tag(name).orElseThrow());
                case "score" -> result.put(name,box.score(name).orElseThrow());
                case "filter" -> result.put(name,box.filter(name).orElseThrow());
                case "find" -> result.put(name,box.find(name).orElseThrow());
                case "recognize" -> result.put(name,box.recognize(name).orElseThrow());
                case "relate" -> result.put(name,box.relate(name).orElseThrow());
                default -> throw new AssertionError("Unexpected annotation function");
            }
        });
        return result.build();
    }
    private static JsonNode value(Object item) {
        if (item instanceof java.util.Optional<?> optional) return optional.map(ConformanceCases::value).orElse(ConsumerCases.JSON.nullNode());
        if (item instanceof Span span) return ConsumerCases.JSON.valueToTree(span);
        if (item instanceof Mention mention) { var result = (ObjectNode) value(mention.span()); result.put("kind",mention.kind()); return result; }
        if (item instanceof Entity entity) { var result = (ObjectNode) value(entity.mention()); result.put("probability",entity.probability()); return result; }
        if (item instanceof Relation relation) {
            var result = ConsumerCases.JSON.createObjectNode().put("relation",relation.relation()).put("probability",relation.probability());
            result.set("source",value(relation.source())); result.set("target",value(relation.target())); relation.either().ifPresent(either -> result.put("either",either)); return result;
        }
        if (item instanceof List<?> list) { var result = ConsumerCases.JSON.createArrayNode(); list.forEach(entry -> result.add(value(entry))); return result; }
        if (item instanceof Annotations annotations) {
            var result = ConsumerCases.JSON.createObjectNode();
            annotations.entries().forEach((name,entry) -> {
                String function; AnnotationOutcome<?,?> outcome;
                switch (entry) {
                    case AnnotationEntry.Decide e -> { function = "decide"; outcome = e.outcome(); }
                    case AnnotationEntry.Choose e -> { function = "choose"; outcome = e.outcome(); }
                    case AnnotationEntry.Tag e -> { function = "tag"; outcome = e.outcome(); }
                    case AnnotationEntry.Score e -> { function = "score"; outcome = e.outcome(); }
                    case AnnotationEntry.Filter e -> { function = "filter"; outcome = e.outcome(); }
                    case AnnotationEntry.Find e -> { function = "find"; outcome = e.outcome(); }
                    case AnnotationEntry.Recognize e -> { function = "recognize"; outcome = e.outcome(); }
                    case AnnotationEntry.Relate e -> { function = "relate"; outcome = e.outcome(); }
                }
                result.set(name,outcome(function,outcome));
            });
            return result;
        }
        return ConsumerCases.JSON.valueToTree(item);
    }
    private static JsonNode detail(Detail item) {
        if (item instanceof RecognizeDetail recognize) {
            var result = ConsumerCases.JSON.createObjectNode(); var pieces = result.putArray("pieces");
            for (var piece : recognize.pieces()) { var row = (ObjectNode) value(piece.span()); row.set("tags",value(piece.tags())); pieces.add(row); }
            result.set("names",value(recognize.names())); return result;
        }
        if (item instanceof RelateDetail relate) { var result = ConsumerCases.JSON.createObjectNode(); result.set("names",value(relate.names())); result.set("pairs",value(relate.pairs())); return result; }
        return ConsumerCases.JSON.valueToTree(item);
    }
    private static ObjectNode outcome(String function,AnnotationOutcome<?,?> item) {
        var result = ConsumerCases.JSON.createObjectNode().put("function",function).put("status",item.status().name().toLowerCase(java.util.Locale.ROOT));
        switch (item) {
            case AnnotationAnswered<?,?> a -> { result.set("answer",value(a.answer())); a.probability().ifPresent(p -> result.put("probability",p)); a.detail().ifPresent(d -> result.set("detail",detail(d))); }
            case AnnotationUnsure<?,?> u -> { result.put("probability",u.probability()); u.detail().ifPresent(d -> result.set("detail",detail(d))); }
            case AnnotationFailed<?,?> f -> { var error = result.putObject("error").put("code",f.error().code().wireName()); f.error().httpStatus().ifPresent(status -> error.put("http_status",status)); }
        }
        return result;
    }
    static ObjectNode row(Row<?,?> row) {
        AnnotationOutcome<?,?> outcome = switch (row) {
            case Answered<?,?> a -> new AnnotationAnswered<>(a.answer(),a.probability(),a.detail());
            case Unsure<?,?> u -> new AnnotationUnsure<>(u.probability(),u.detail());
            case Failed<?,?> f -> new AnnotationFailed<>(f.error());
        };
        var result = outcome(row.function().name().toLowerCase(java.util.Locale.ROOT),outcome);
        result.put("position",row.position()); row.id().ifPresent(id -> { try { result.set("id",ConsumerCases.JSON.readTree(id.toString())); } catch (java.io.IOException e) { throw new java.io.UncheckedIOException(e); } });
        return result;
    }
    private static JsonNode expected(JsonNode sample) {
        JsonNode result = sample.get("expected").deepCopy();
        if (!result.isArray()) return result;
        for (var row : result) {
            var record = sample.get("request").get("records").get(row.get("position").intValue()-1);
            if (record.isTextual()) { offsets(row.path("answer"),record.asText()); offsets(row.path("detail"),record.asText()); }
        }
        return result;
    }
    private static void offsets(JsonNode value,String text) {
        if (value.isObject() && value.has("start") && value.has("end")) {
            ((ObjectNode)value).put("start",text.offsetByCodePoints(0,value.get("start").intValue()));
            ((ObjectNode)value).put("end",text.offsetByCodePoints(0,value.get("end").intValue()));
        }
        for (var child : value) offsets(child,text);
    }
    private static void savedToolbox(Path directory) throws Exception {
        String fixture = Files.readString(directory.resolve("questions.json"));
        var loaded = Toolbox.fromJson(fixture);
        ConsumerCases.check(ValuesEqual(loaded.toJson(),fixture),"saved toolbox changed");
        ConsumerCases.check(loaded.get("ready").orElseThrow() instanceof FilterQuestion && loaded.get("missing").isEmpty(),"saved lookup");
        ConsumerCases.check(loaded.relate("names").isPresent(),"saved relate lost");
        try (var client = ThinkThen.builder().backend(Backend.STUB).build()) {
            var original = ChooseQuestion.of("Which topic?",List.of("billing","shipping"),ChooseOptions.builder().threshold(Threshold.cut(0.7)).evidence(Pointer.of("/body")).build());
            var record = java.util.Map.of("body","Bill");
            ConsumerCases.check(row(client.choose(loaded.choose("topic").orElseThrow(),record)).equals(row(client.choose(original,record))),"loaded execution changed");
            var named = original.named("triage");
            var constructed = Toolbox.builder().put("topic",named).build();
            ConsumerCases.check(row(client.choose(constructed.choose("topic").orElseThrow(),record)).equals(row(client.choose(named,record))),"constructed toolbox lost execution name");
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
                    System.out.println("Java admission: " + name + ": unsupported: typed client settings have no raw JSON loader");
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
    public static void main(String[] args) throws Exception {
        if (args[0].equals("abi")) {
            boolean accept = Boolean.parseBoolean(args[1]);
            try (var client = ThinkThen.builder().backend(Backend.STUB).build()) {
                ConsumerCases.check(accept,"Accepted incompatible ABI"); client.decide("Ready?","alpha");
            } catch (ThinkThenException error) { ConsumerCases.check(!accept && error.code() == ErrorCode.USAGE_ABI,"ABI refusal"); }
            return;
        }
        admissionCases(Path.of(args[0]).getParent());
        StressCases.check(Path.of(args[0]).getParent());
        Path directory = Path.of(args[0]); savedToolbox(directory.getParent().resolve("toolbox")); var manifest = ConsumerCases.JSON.readTree(directory.resolve("manifest.json").toFile()); int count=0;
        for (var backend : manifest.get("backends")) for (var name : manifest.get("cases")) {
            var file = directory.resolve(name.asText()); var sample = ConsumerCases.JSON.readTree(file.toFile());
            try (var fixture = new BackendFixture(file.toString());
                 var client = ThinkThen.builder().backend(Backend.valueOf(backend.asText().toUpperCase(java.util.Locale.ROOT))).url(URI.create(fixture.base())).apiKey("synthetic-key").model(manifest.get("client").get("model").asText())
                     .batch(sample.path("client").has("batch") ? sample.get("client").get("batch").intValue() : manifest.get("client").get("batch").intValue()).retry(RetryPolicy.builder().maxRetries(0).build()).recordCodec(value -> { try { return ConsumerCases.JSON.writeValueAsString(value); } catch (java.io.IOException e) { throw new java.io.UncheckedIOException(e); } }).build()) {
                JsonNode actual;
                var values = ConsumerCases.JSON.createArrayNode();
                try (var rows = execute(client,sample.get("request"))) {
                    for (var row : rows.toList()) {
                        ConsumerCases.equal(row.source().orElseThrow().backend().name().toLowerCase(java.util.Locale.ROOT),backend.asText(),"typed backend");
                        ConsumerCases.equal(row.source().orElseThrow().model(),"fixture-model","typed model");
                        ConsumerCases.check(!row.source().orElseThrow().questionIdentity().isEmpty(),"question identity"); values.add(row(row));
                    }
                    actual = values;
                } catch (ThinkThenException error) {
                    ConsumerCases.check(values.isEmpty(), "Admission returned rows");
                    actual = error(error.code().wireName());
                }
                ConsumerCases.equal(ConsumerCases.JSON.readTree(ConsumerCases.JSON.writeValueAsBytes(actual)),expected(sample),"canonical "+backend.asText()+"/"+name.asText());
                ConsumerCases.equal(fixture.command("count"),sample.get("requests").asText(),"canonical request count"); count++;
            }
        }
        ConsumerCases.check(count>0,"No Java canonical cases");
        System.out.println("Java public typed API: "+count+" canonical backend cases passed");
    }
}
