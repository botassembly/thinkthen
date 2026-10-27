import io.thinkthen.*;
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

    static <A, D extends Detail> Answered<A, D> answered(Row<A, D> row) {
        return switch (row) {
            case Answered<A, D> answer -> answer;
            case Unsure<A, D> unsure -> throw new AssertionError("Unexpected unsure row at " + unsure.position());
            case Failed<A, D> failed -> throw new AssertionError(failed.error().code().wireName());
        };
    }

    static <A, D extends Detail> void failed(Row<A, D> row, String code) {
        switch (row) {
            case Failed<A, D> failure -> {
                equal(failure.error().code().wireName(), code, "typed failed-row code");
                equal(failure.status(), Status.FAILED, "failed status");
                try {
                    failure.raiseForStatus();
                    throw new AssertionError("raiseForStatus must throw for a failed row");
                } catch (ThinkThenException error) {
                    equal(error.code(), failure.error().code(), "explicit fail-fast preserves code enum");
                    equal(error.getMessage(), failure.error().message(), "explicit fail-fast preserves message");
                }
            }
            default -> throw new AssertionError("Expected failed row " + code);
        }
    }

    // Feature: Java's signatures retain each function's answer and detail types.
    static void allFunctionsAndToolbox(ThinkThen client) {
        Row<Boolean, NoDetail> decide = client.decide("Ready?", "alpha");
        var bandOptions = DecideOptions.builder().threshold(Threshold.band(0.2, 0.8))
            .evidence(Pointer.of("/body")).build();
        Row<Boolean, NoDetail> selectedEvidence = client.decide("Ready?", Map.of("id", "r1", "body", "alpha"), bandOptions);
        equal(selectedEvidence.id().orElseThrow(), JsonValue.parse("\"r1\""), "evidence selection preserves outer identifier");
        check(selectedEvidence.status() != Status.FAILED, "typed band and pointer reach engine admission");
        Row<String, ChooseDetail> choose = client.choose("Which?", "alpha", List.of("a", "b"));
        Row<List<String>, TagDetail> tag = client.tag("What applies?", "alpha", List.of("a", "b"));
        Row<Double, ScoreDetail> score = client.score("How much?", "alpha", List.of("low", "high"));
        Row<Optional<Span>, NoDetail> find = client.find("Which line?", "first\nsecond");
        Row<List<Entity>, RecognizeDetail> recognize = client.recognize("Ada Acme", List.of("person", "company"));
        var rule = RelationRule.builder("person", "company", "works_for").either(true).build();
        Row<List<Relation>, RelateDetail> relate = client.relate("Ada Acme", List.of(rule));
        equal(decide.function(), io.thinkthen.Function.DECIDE, "typed function tag");
        equal(decide.position(), 1L, "single input position");
        equal(answered(decide).detail(), Optional.empty(), "no-detail marker stays absent");
        check(List.of("a", "b").contains(answered(choose).answer()), "choice answer remains a label");
        Map<String, Double> labelProbabilities = answered(tag).detail().orElseThrow().labels();
        equal(labelProbabilities.keySet(), java.util.Set.of("a", "b"), "tag detail retains all labels");
        List<Double> levels = answered(score).detail().orElseThrow().levels();
        equal(levels.size(), 2, "score detail follows required levels");
        answered(find).answer().ifPresent(span -> equal("first\nsecond".substring(span.start(), span.end()), span.text(), "find span offsets"));
        for (Entity entity : answered(recognize).answer()) {
            Mention mention = entity.mention();
            check(entity.probability() >= 0 && entity.probability() <= 1, "entity probability type");
            equal("Ada Acme".substring(mention.span().start(), mention.span().end()), mention.span().text(), "entity UTF-16 offsets");
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

        var toolbox = Toolbox.parse("""
            {"route":{"function":"choose","question":"Which?","choices":["a","b"]},
             "ready":{"function":"decide","question":"Ready?"}}
            """);
        ChooseQuestion selected = toolbox.choose("route").orElseThrow();
        Row<String, ChooseDetail> selectedRow = client.choose(selected, "alpha");
        equal(answered(selectedRow).answer(), answered(choose).answer(), "typed question carries choices");
        check(toolbox.decide("route").isEmpty(), "wrong typed selector is empty");
        check(toolbox.choose("absent").isEmpty(), "missing selector is empty");
        Row<Annotations, NoDetail> annotation = client.annotate(AnnotationQuestions.builder().put("route", selected).put("ready", toolbox.decide("ready").orElseThrow()).build(), "alpha");
        Annotations values = answered(annotation).answer();
        AnnotationOutcome<String, ChooseDetail> route = values.choose("route").orElseThrow();
        switch (route) {
            case AnnotationAnswered<String, ChooseDetail> value -> equal(value.answer(), answered(choose).answer(), "annotation is a typed label");
            case AnnotationUnsure<String, ChooseDetail> ignored -> throw new AssertionError("stub choice must answer");
            case AnnotationFailed<String, ChooseDetail> value -> throw new AssertionError(value.error().message());
        }
        check(values.decide("route").isEmpty(), "annotation selector preserves function type");
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
        List<JsonValue> invalid = List.of("true", "null", "1.0", "1e0", "1.5", "[]", "{}", "18446744073709551616", "-9223372036854775809")
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
        try (Stream<ItemRow<String, Boolean, NoDetail>> rows = client.filter("Ready?", input)) {
            var output = rows.toList();
            equal(output.size(), 3, "filter retains missing and unkept inputs");
            for (int i = 0; i < output.size(); i++) {
                check(output.get(i).item() == input.get(i), "filter preserves every original object");
                equal(output.get(i).row().position(), (long) i + 1, "filter retains original position");
            }
            failed(output.get(1).row(), "record.missing");
        }
        try (Stream<ItemRow<String, Long, RankDetail>> rows = client.rank("Urgency?", input, List.of("low", "high"), RankOptions.builder().top(1).build())) {
            var output = rows.toList();
            equal(output.size(), 2, "rank top keeps one answer and every failure");
            var top = output.getFirst();
            equal(answered(top.row()).answer(), 1L, "rank answer is distinct from input position");
            check(top.item() == input.get(Math.toIntExact(top.row().position()) - 1), "rank joins the correct original object");
            var missing = output.getLast();
            check(missing.item() == null, "rank keeps missing original");
            equal(missing.row().position(), 2L, "rank failure position");
            failed(missing.row(), "record.missing");
        }
        failed(client.decide("Ready?", null), "record.missing");
        failed(client.decide("Ready?", JsonValue.parse("null")), "record.missing");
    }

    // Feature: Java admits only the supported method shapes at compilation.
    static void requiredSignatures() throws Exception {
        var compiler = ToolProvider.getSystemJavaCompiler();
        check(compiler != null, "signature cases require the Java 22 JDK");
        String good = """
            import io.thinkthen.*; import java.util.*;
            final class SignatureCase { void call(ThinkThen c) {
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
        check(!compiles(compiler, "import io.thinkthen.internal.Client; final class SignatureCase {}"),
            "classpath consumers cannot compile against a second client");
        Path artifact = Path.of(ThinkThen.class.getProtectionDomain().getCodeSource().getLocation().toURI());
        try (var jar = new java.util.jar.JarFile(artifact.toFile())) {
            check(jar.stream().noneMatch(entry -> entry.getName().startsWith("io/thinkthen/internal/Client")),
                "built jar contains no internal Client classes");
        }
        for (String call : List.of("c.choose(\"Q?\", \"x\")", "c.tag(\"Q?\", \"x\")", "c.score(\"Q?\", \"x\")", "c.recognize(\"x\")", "c.relate(\"x\")", "c.rank(\"Q?\", List.of(\"x\"))", "AnnotationQuestions.builder().put(\"r\", RankQuestion.of(\"Q?\", List.of(\"low\", \"high\")))", "c.annotate(Toolbox.builder().build(), \"x\")", "TagOptions.builder().threshold(Threshold.band(0.2, 0.8))", "DecideOptions.builder().threshold(\"0.2:0.8\")", "c.decide(\"Q?\", \"x\").answer()")) {
            String source = "import io.thinkthen.*; import java.util.*; final class SignatureCase { void call(ThinkThen c) { " + call + "; }}";
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
            Files.writeString(info,"module consumer { requires io.thinkthen; }");
            Files.writeString(source,"""
                package consumer;
                import io.thinkthen.*;
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
            Files.writeString(source,"package consumer; import io.thinkthen.internal.DriverClient; final class ModuleConsumer {}");
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
