package dev.thinkthen;

/**
 * A reusable typed rank question.
 */
public final class RankQuestion implements Question<Long,LevelDetail> {
    private final com.fasterxml.jackson.databind.JsonNode definition;
    private final @org.jspecify.annotations.Nullable String name;
    RankQuestion(com.fasterxml.jackson.databind.JsonNode definition, @org.jspecify.annotations.Nullable String name) { this.definition = definition.deepCopy(); this.name = name; }
    com.fasterxml.jackson.databind.JsonNode definition() { return definition.deepCopy(); }
    @org.jspecify.annotations.Nullable String name() { return name; }
    /**
     * Return this question with a caller-visible name.
     * @param name the caller-visible name
     * @return the named result
     */
    public RankQuestion named(String name) { return new RankQuestion(definition,java.util.Objects.requireNonNull(name,"name")); }
    /**
     * Return the engine function.
     * @return the function result
     */
    public FunctionName function() { return FunctionName.RANK; }
    /**
     * Create this typed value.
     * @param question the question to ask
     * @param levels the levels in their required order
     * @return the of result
     */
    public static RankQuestion of(String question, java.util.List<String> levels) { return of(question, levels, RankOptions.defaults()); }
    /**
     * Create this typed value.
     * @param question the question to ask
     * @param levels the levels in their required order
     * @param options the function-specific options
     * @return the of result
     */
    public static RankQuestion of(String question, java.util.List<String> levels, RankOptions options) {
        var values = new java.util.HashMap<String,Object>(options.values());
        values.put("function","rank");
        values.put("question",java.util.Objects.requireNonNull(question,"question")); values.put("levels",Questions.options(levels,values));
        return new RankQuestion(Values.tree(values),options.name());
    }

    public static <E extends Enum<E>> RankQuestion of(String question, Class<E> type) { return of(question,type,RankOptions.defaults()); }
    public static <E extends Enum<E>> RankQuestion of(String question, Class<E> type, RankOptions options) { return of(question,Domain.of(type).words(),options); }
}
