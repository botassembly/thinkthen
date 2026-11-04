import dev.thinkthen.*;
import java.net.URI;
import java.util.List;
import java.util.Map;

/** Feature: description mapping preserves values and delegates unknown names to the engine. */
public final class JavaDescriptionCases {
    private JavaDescriptionCases() {}
    private static ThinkThenException refused(Runnable action) {
        try { action.run(); throw new AssertionError("Unknown description was accepted"); }
        catch (ThinkThenException error) { return error; }
    }
    public static void main(String[] args) throws Exception {
        if (args.length != 0) { check(args[0]); return; }
        try (var fixture = new BackendFixture("unavailable")) {
            check(fixture.base());
            if (!fixture.command("count").equals("0")) throw new AssertionError("Admission made a backend request");
        }
    }
    private static void check(String url) {
        var descriptions = Map.of("a", "", "other", "unknown");
        try (var client = ThinkThen.builder().backend(Backend.OPENAI).url(URI.create(url))
                .apiKey("synthetic-local-key").retry(RetryPolicy.builder().maxRetries(0).build()).build()) {
            for (String verb : List.of("choose", "tag", "score", "rank")) {
                String field = switch (verb) { case "choose" -> "choices"; case "tag" -> "labels"; default -> "levels"; };
                var expected = refused(() -> Toolbox.fromDefinitions("{\"entry\":{\"function\":\"" + verb
                    + "\",\"question\":\"Which?\",\"" + field + "\":[{\"name\":\"a\",\"description\":\"\"},\"b\"],\"descriptions\":{\"other\":\"unknown\"}}}"));
                Question<?,?> typed = switch (verb) {
                    case "choose" -> ChooseQuestion.of("Which?", List.of("a", "b"), ChooseOptions.builder().descriptions(descriptions).build());
                    case "tag" -> TagQuestion.of("Which?", List.of("a", "b"), TagOptions.builder().descriptions(descriptions).build());
                    case "score" -> ScoreQuestion.of("Which?", List.of("a", "b"), ScoreOptions.builder().descriptions(descriptions).build());
                    default -> RankQuestion.of("Which?", List.of("a", "b"), RankOptions.builder().descriptions(descriptions).build());
                };
                var actual = refused(() -> Toolbox.builder().put("entry", typed).build());
                if (actual.code() != expected.code() || !actual.getMessage().equals(expected.getMessage()))
                    throw new AssertionError(verb + ": " + actual + " differs from engine " + expected);
                var runtime = refused(() -> {
                    switch (verb) {
                        case "choose" -> client.choose("Which?", "alpha", List.of("a", "b"), ChooseOptions.builder().descriptions(descriptions).build());
                        case "tag" -> client.tag("Which?", "alpha", List.of("a", "b"), TagOptions.builder().descriptions(descriptions).build());
                        case "score" -> client.score("Which?", "alpha", List.of("a", "b"), ScoreOptions.builder().descriptions(descriptions).build());
                        default -> { try (var rows = client.rank("Which?", List.of("alpha"), List.of("a", "b"), RankOptions.builder().descriptions(descriptions).build())) { rows.toList(); } }
                    }
                });
                if (runtime.code() != expected.code()) throw new AssertionError(verb + ": " + runtime);
            }
        }
        var question = ChooseQuestion.of("Which?", List.of("a", "b"), ChooseOptions.builder().descriptions(Map.of("a", "", "b", "Meaning")).build());
        var saved = Toolbox.builder().put("entry", question).build().toJson();
        var json = new com.fasterxml.jackson.databind.ObjectMapper();
        try {
            var choices = json.readTree(saved).path("entries").path("entry").path("question").path("choices");
            if (!choices.get(0).path("description").isTextual() || !choices.get(0).path("description").asText().isEmpty()
                    || !choices.get(1).path("description").asText().equals("Meaning")) throw new AssertionError(saved);
        } catch (java.io.IOException error) { throw new AssertionError(error); }
        System.out.println("Java descriptions preserve values and match the engine diagnostic");
    }
}
