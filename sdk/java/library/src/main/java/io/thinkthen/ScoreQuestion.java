package io.thinkthen;

public final class ScoreQuestion implements AnnotationQuestion<Double,ScoreDetail> {
    private final com.fasterxml.jackson.databind.JsonNode definition;
    private final @org.jspecify.annotations.Nullable String name;
    ScoreQuestion(com.fasterxml.jackson.databind.JsonNode definition, @org.jspecify.annotations.Nullable String name) { this.definition = definition.deepCopy(); this.name = name; }
    com.fasterxml.jackson.databind.JsonNode definition() { return definition.deepCopy(); }
    @org.jspecify.annotations.Nullable String name() { return name; }
    public ScoreQuestion named(String name) { return new ScoreQuestion(definition,java.util.Objects.requireNonNull(name)); }
    public Function function() { return Function.SCORE; }
    public static ScoreQuestion of(String question, java.util.List<String> levels) { return of(question, levels, ScoreOptions.defaults()); }
    public static ScoreQuestion of(String question, java.util.List<String> levels, ScoreOptions options) {
        var values = new java.util.HashMap<String,Object>(options.values());
        values.put("function","score");
        values.put("question",java.util.Objects.requireNonNull(question)); values.put("levels",java.util.List.copyOf(levels));
        return new ScoreQuestion(Values.tree(values),options.name());
    }
}
