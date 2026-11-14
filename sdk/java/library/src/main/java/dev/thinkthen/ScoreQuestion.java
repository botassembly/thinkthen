package dev.thinkthen;

/**
 * A reusable typed score question.
 */
public final class ScoreQuestion implements AnnotationQuestion<Double,LevelDetail> {
    private final com.fasterxml.jackson.databind.JsonNode definition;
    private final @org.jspecify.annotations.Nullable String name;
    ScoreQuestion(com.fasterxml.jackson.databind.JsonNode definition, @org.jspecify.annotations.Nullable String name) { this.definition = definition.deepCopy(); this.name = name; }
    com.fasterxml.jackson.databind.JsonNode definition() { return definition.deepCopy(); }
    @org.jspecify.annotations.Nullable String name() { return name; }
    /**
     * Return this question with a caller-visible name.
     * @param name the caller-visible name
     * @return the named result
     */
    public ScoreQuestion named(String name) { return new ScoreQuestion(definition,java.util.Objects.requireNonNull(name,"name")); }
    /**
     * Return the engine function.
     * @return the function result
     */
    public FunctionName function() { return FunctionName.SCORE; }
    /**
     * Create this typed value.
     * @param question the question to ask
     * @param levels the levels in their required order
     * @return the of result
     */
    public static ScoreQuestion of(String question, java.util.List<String> levels) { return of(question, levels, ScoreOptions.defaults()); }
    /**
     * Create this typed value.
     * @param question the question to ask
     * @param levels the levels in their required order
     * @param options the function-specific options
     * @return the of result
     */
    public static ScoreQuestion of(String question, java.util.List<String> levels, ScoreOptions options) {
        var values = new java.util.HashMap<String,Object>(options.values());
        values.put("function","score");
        values.put("question",java.util.Objects.requireNonNull(question,"question")); values.put("levels",Questions.options(levels,values));
        return new ScoreQuestion(Values.tree(values),options.name());
    }

    public static <E extends Enum<E>> ScoreQuestion of(String question, Class<E> type) { return of(question,type,ScoreOptions.defaults()); }
    public static <E extends Enum<E>> ScoreQuestion of(String question, Class<E> type, ScoreOptions options) { return of(question,Domain.of(type).words(),options); }
}
