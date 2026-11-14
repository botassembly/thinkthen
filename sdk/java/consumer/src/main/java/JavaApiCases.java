import dev.thinkthen.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;
import java.util.stream.Stream;
import javax.tools.ToolProvider;

/** Consumer expectations for the approved Java API, run against an installed JAR. */
public final class JavaApiCases {
    private JavaApiCases() {}

    static void check(boolean condition, String claim) {
        if (!condition) throw new AssertionError(claim);
    }

    static void equal(Object actual, Object expected, String claim) {
        if (!Objects.equals(actual, expected)) {
            throw new AssertionError(claim + ": expected " + expected + ", got " + actual);
        }
    }

    static <A, T> Answered<A, T> answered(Row<A, T> row) {
        return switch (row) {
            case Answered<A, T> answer -> answer;
            case Unsure<A, T> unsure -> throw new AssertionError("Unexpected unsure row at " + unsure.position());
            case Failed<A, T> failed -> throw new AssertionError(failed.error().code().wireName());
        };
    }

    static <A, T> void failed(Row<A, T> row, String code) {
        switch (row) {
            case Failed<A, T> failure -> {
                equal(failure.error().code().wireName(), code, "typed failed-row code");
                equal(failure.status(), Status.FAILED, "failed status");
                try {
                    failure.throwIfFailed();
                    throw new AssertionError("throwIfFailed must throw for a failed row");
                } catch (FailedException error) {
                    equal(error.code(), failure.error().code(), "explicit fail-fast preserves code enum");
                    equal(error.getMessage(), failure.error().message(), "explicit fail-fast preserves message");
                }
            }
            default -> throw new AssertionError("Expected failed row " + code);
        }
    }

    static void failedCall(java.util.function.Supplier<? extends Row<?,?>> call, String code) {
        try { call.get(); throw new AssertionError("Scalar failure must throw at the call: " + code); }
        catch (FailedException error) {
            equal(error.code().wireName(),code,"scalar failure preserves code");
            equal(error.rows().size(),1,"scalar failure retains its actual failed parent");
            failed(error.rows().getFirst(),code);
        }
    }

    // Feature: Java signatures retain each function's answer and original-record types.
    static void allFunctionsAndToolbox(ThinkThen client) {
        Row<Boolean, String> decide = client.decide("Ready?", "alpha");
        var bandOptions = DecideOptions.builder().threshold(Threshold.band(0.2, 0.8))
            .field(Pointer.of("/body")).build();
        var selectedField = client.decide("Ready?", Map.of("id", "r1", "body", "alpha"), bandOptions);
        equal(selectedField.id().orElseThrow(), JsonValue.parse("\"r1\""), "field selection preserves outer identifier");
        check(selectedField.status() != Status.FAILED, "typed band and pointer reach engine admission");
        Row<String, String> choose = client.choose("Which?", "alpha", List.of("a", "b"));
        Row<List<String>, String> tag = client.tag("What applies?", "alpha", List.of("a", "b"));
        Row<Double, String> score = client.score("How much?", "alpha", List.of("low", "high"));
        Row<Optional<Span>, String> find = client.find("Which line?", "first\nsecond");
        Row<List<Entity>, String> recognize = client.recognize("Ada Acme", List.of("person", "company"));
        var rule = RelationRule.builder("person", "company", "works_for").either(true).build();
        Row<List<Relation>, String> relate = client.relate("Ada Acme", List.of(rule));
        equal(decide.function(), dev.thinkthen.FunctionName.DECIDE, "typed function tag");
        equal(decide.position(), 1L, "single input position");
        equal(answered(decide).detail(), NoDetail.INSTANCE, "safe detail has the no-detail marker");
        check(List.of("a", "b").contains(answered(choose).answer()), "choice answer remains a label");
        Map<String, Double> labelProbabilities = ((LabelDetail) answered(tag).detail()).labels();
        equal(labelProbabilities.keySet(), java.util.Set.of("a", "b"), "tag detail retains all labels");
        List<Double> levels = ((LevelDetail) answered(score).detail()).levels();
        equal(levels.size(), 2, "score detail follows required levels");
        answered(find).answer().ifPresent(span -> equal("first\nsecond".substring(Math.toIntExact(span.start()),Math.toIntExact(span.end())), span.text(), "find span offsets"));
        for (Entity entity : answered(recognize).answer()) {
            Mention mention = entity.mention();
            check(entity.probability() >= 0 && entity.probability() <= 1, "entity probability type");
            equal("Ada Acme".substring(Math.toIntExact(mention.span().start()),Math.toIntExact(mention.span().end())), mention.span().text(), "entity UTF-16 offsets");
        }
        for (Relation relation : answered(relate).answer()) {
            equal(relation.relation(), "works_for", "relation uses the required rule");
            check(relation.probability() >= 0 && relation.probability() <= 1, "typed relation probability");
        }
        try {
            labelProbabilities.put("changed", 0.1);
            throw new AssertionError("detail map must be immutable");
        } catch (UnsupportedOperationException expected) { }
        try {
            levels.add(0.2);
            throw new AssertionError("detail list must be immutable");
        } catch (UnsupportedOperationException expected) { }

        var toolbox = Toolbox.fromDefinitions("""
            {"route":{"function":"choose","question":"Which?","choices":["a","b"]},
             "ready":{"function":"decide","question":"Ready?"}}
            """);
        ChooseQuestion<String> selected = toolbox.choose("route");
        Row<String, String> selectedRow = client.choose(selected, "alpha");
        equal(answered(selectedRow).answer(), answered(choose).answer(), "typed question carries choices");
        for (Runnable lookup : List.<Runnable>of(() -> toolbox.decide("route"), () -> toolbox.choose("absent"))) {
            try { lookup.run(); throw new AssertionError("wrong or missing typed selection must fail"); }
            catch (UsageException expected) { }
        }
        Row<Annotations, String> annotation = client.annotate(AnnotationQuestions.builder().put("route", selected).put("ready", toolbox.decide("ready")).build(), "alpha");
        Annotations values = answered(annotation).answer();
        Row<String,Object> route = values.choose("route");
        equal(answered(route).answer(), answered(choose).answer(), "annotation is an ordinary typed row");
        for (Runnable selector : List.<Runnable>of(
                () -> values.decide("absent"), () -> values.choose("absent"),
                () -> values.tag("absent"), () -> values.score("absent"),
                () -> values.find("absent"),
                () -> values.recognize("absent"), () -> values.relate("absent"),
                () -> values.decide("route"), () -> values.choose("ready"),
                () -> values.tag("ready"), () -> values.score("ready"),
                () -> values.find("ready"),
                () -> values.recognize("ready"), () -> values.relate("ready"))) {
            try { selector.run(); throw new AssertionError("unknown or wrong-function selector must throw"); }
            catch (ThinkThenException expected) {
                equal(expected.code().wireName(), "usage.argument", "selector failure uses shared argument code");
                check(expected.getMessage().contains("route") && expected.getMessage().contains("ready"), "selector failure lists available names");
            }
        }
        equal(values.entries().size(), 2, "annotation retains both entries");
        try {
            values.entries().clear();
            throw new AssertionError("annotation entries must be immutable");
        } catch (UnsupportedOperationException expected) { }
    }

    // Feature: JSON identifiers retain their exact string or 64-bit integer type.
    static void idsAndMissingItems(ThinkThen client) {
        for (String id : List.of("\"00042\"", "-9223372036854775808", "9223372036854775807", "18446744073709551615", "0")) {
            JsonValue record = JsonValue.parse("{\"id\":" + id + ",\"body\":\"alpha\"}");
            var row = client.decide("Ready?", record);
            equal(row.id().orElseThrow(), JsonValue.parse(id), "identifier preserves JSON kind and exact integer");
            check(row.status() != Status.FAILED, "valid identifier reaches the engine");
        }
        for (String id : List.of("1.0", "1e0")) {
            JsonValue record = JsonValue.parse("{\"id\":" + id + ",\"body\":\"alpha\"}");
            var row = client.decide("Ready?", record);
            equal(row.id().orElseThrow(), JsonValue.parse("1"), "whole-number float identifier becomes that integer");
            check(row.status() != Status.FAILED, "whole-number float identifier reaches the engine");
        }
        List<JsonValue> invalid = List.of("true", "null", "1.5", "[]", "{}", "18446744073709551616", "-9223372036854775809")
            .stream().map(id -> JsonValue.parse("{\"id\":" + id + ",\"body\":\"alpha\"}")).toList();
        try (var rows = client.decideStream("Ready?", invalid)) {
            var output = rows.toList();
            equal(output.size(), invalid.size(), "invalid IDs never disappear");
            for (int i = 0; i < output.size(); i++) {
                failed(output.get(i), "record.invalid");
                equal(output.get(i).position(), (long) i + 1, "invalid identifier still occupies its position");
            }
        }
        var first = new String("alpha");
        var last = new String("omega");
        List<String> input = Arrays.asList(first, null, last);
        try (var rows = client.decideStream("Ready?", input)) {
            var output = rows.toList();
            equal(output.size(), 3, "decide stream keeps every original position");
            for (int i = 0; i < output.size(); i++) {
                check(output.get(i).item() == input.get(i), "every result preserves original identity");
                equal(output.get(i).position(), (long)i+1, "stream retains original position");
            }
            failed(output.get(1), "record.missing");
        }
        try (Stream<Row<Long,String>> rows = client.rank("Urgency?", input, List.of("low", "high"), RankOptions.builder().top(1).build())) {
            var output = rows.toList();
            equal(output.size(), 2, "rank top keeps one answer and every failure");
            var top = output.getFirst();
            equal(answered(top).answer(), 1L, "rank answer is distinct from input position");
            check(top.item() == input.get(Math.toIntExact(top.position())-1), "rank preserves the caller object");
            var missing = output.getLast();
            check(missing.item() == null, "rank keeps missing original");
            equal(missing.position(), 2L, "rank failure position");
            failed(missing, "record.missing");
        }
        failedCall(() -> client.decide("Ready?", null), "record.missing");
        failedCall(() -> client.decide("Ready?", JsonValue.parse("null")), "record.missing");
    }

    // Feature: Java admits only the supported method shapes at compilation.
    static void requiredSignatures() throws Exception {
        var compiler = ToolProvider.getSystemJavaCompiler();
        check(compiler != null, "signature cases require the Java 22 JDK");
        String good = """
            import dev.thinkthen.*; import java.util.*; import java.util.function.*;
            final class SignatureCase { void call(ThinkThen c) {
                Function<String,String> identity = value -> value;
                FunctionName function = FunctionName.DECIDE;
                boolean isReady = c.decide("Ready?","alpha").answer();
                c.choose("Which?", "x", List.of("a", "b"));
                c.tag("Tags?", "x", List.of("a"));
                c.score("Scale?", "x", List.of("low", "high"));
                c.recognize("x", List.of("person"));
                c.relate("x", List.of(RelationRule.builder("person", "company", "works_for").build()));
                c.rank("Rank?", List.of("x"), List.of("low", "high"));
                c.tag("Tags?", "x", List.of("a"), TagOptions.builder().threshold(Threshold.cut(0.7)).build());
            }}
            """;
        check(compiles(compiler, good), "required-list and cut signatures compile");
        check(!compiles(compiler, "import dev.thinkthen.internal.Client; final class SignatureCase {}"),
            "classpath consumers cannot compile against a second client");
        Path artifact = Path.of(ThinkThen.class.getProtectionDomain().getCodeSource().getLocation().toURI());
        try (var jar = new java.util.jar.JarFile(artifact.toFile())) {
            check(jar.stream().noneMatch(entry -> entry.getName().startsWith("dev/thinkthen/internal/Client")),
                "built jar contains no internal Client classes");
        }
        for (String call : List.of("c.choose(\"Q?\", \"x\")", "c.tag(\"Q?\", \"x\")", "c.score(\"Q?\", \"x\")", "c.recognize(\"x\")", "c.relate(\"x\")", "c.rank(\"Q?\", List.of(\"x\"))", "AnnotationQuestions.builder().put(\"r\", RankQuestion.of(\"Q?\", List.of(\"low\", \"high\")))", "c.annotate(Toolbox.builder().build(), \"x\")", "ChooseOptions.builder().threshold(Threshold.band(0.2, 0.8))", "TagOptions.builder().threshold(Threshold.band(0.2, 0.8))", "DecideOptions.builder().threshold(\"0.2:0.8\")")) {
            String source = "import dev.thinkthen.*; import java.util.*; final class SignatureCase { void call(ThinkThen c) { " + call + "; }}";
            check(!compiles(compiler, source), "unsupported API must fail compilation: " + call);
        }
    }

    private static boolean compiles(javax.tools.JavaCompiler compiler, String source) throws Exception {
        Path directory = Files.createTempDirectory("java-api-signature-");
        try {
            Path file = directory.resolve("SignatureCase.java");
            Files.writeString(file, source);
            try (var manager = compiler.getStandardFileManager(null, null, null)) {
                var diagnostics = new javax.tools.DiagnosticCollector<javax.tools.JavaFileObject>();
                return compiler.getTask(null, manager, diagnostics, List.of("--release", "22", "-proc:none", "-classpath", System.getProperty("java.class.path"), "-d", directory.toString()), null, manager.getJavaFileObjects(file)).call();
            }
        } finally {
            try (var files = Files.walk(directory)) {
                for (var file : files.sorted(java.util.Comparator.reverseOrder()).toList()) Files.delete(file);
            }
        }
    }

    static void modulePath() throws Exception {
        var directory = Files.createTempDirectory("java-module-consumer-");
        try {
            var info = directory.resolve("module-info.java");
            var source = directory.resolve("ModuleConsumer.java");
            Files.writeString(info,"module consumer { requires dev.thinkthen; }");
            Files.writeString(source,"""
                package consumer;
                import dev.thinkthen.*;
                import org.jspecify.annotations.Nullable;
                import java.util.*;
                final class ModuleConsumer {
                    void call(ThinkThen client, List<@Nullable String> records) {
                        try (var rows = client.filter("Ready?", records)) { rows.forEach(System.out::println); }
                        try (var rows = client.rank("Priority?", records, List.of("low", "high"))) { rows.forEach(System.out::println); }
                    }
                }
                """);
            String modules = Arrays.stream(System.getProperty("java.class.path").split(java.io.File.pathSeparator))
                .filter(path -> path.endsWith(".jar") && (path.contains("thinkthen-java-") || path.contains("jackson-") || path.contains("jspecify-")))
                .collect(java.util.stream.Collectors.joining(java.io.File.pathSeparator));
            var compiler = ToolProvider.getSystemJavaCompiler();
            var options = List.of("--release","22","-Xlint:all","-Werror","--module-path",modules,"-d",directory.resolve("classes").toString());
            try (var manager = compiler.getStandardFileManager(null,null,null)) {
                check(compiler.getTask(null,manager,null,options,null,manager.getJavaFileObjects(info,source)).call(),"JPMS consumer can use nullable filter and rank inputs");
            }
            Files.writeString(source,"package consumer; import dev.thinkthen.internal.DriverClient; final class ModuleConsumer {}");
            try (var manager = compiler.getStandardFileManager(null,null,null)) {
                var diagnostics = new javax.tools.DiagnosticCollector<javax.tools.JavaFileObject>();
                check(!compiler.getTask(null,manager,diagnostics,options,null,manager.getJavaFileObjects(info,source)).call(),"JPMS keeps the shared native driver private");
            }
        } finally {
            try (var files = Files.walk(directory)) { for (var path : files.sorted(java.util.Comparator.reverseOrder()).toList()) Files.delete(path); }
        }
    }

    public static void main(String[] args) throws Exception {
        try (var client = ThinkThen.builder().backend(Backend.STUB).build()) {
            allFunctionsAndToolbox(client);
            idsAndMissingItems(client);
        }
        if (args.length == 1 && args[0].equals("--stub-only")) {
            System.out.println("Packaged Java: ten typed public functions and host rows pass without a compiler");
            return;
        }
        requiredSignatures();
        modulePath();
    }
}
