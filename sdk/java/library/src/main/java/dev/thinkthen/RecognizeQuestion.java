package dev.thinkthen;

/**
 * A reusable typed recognize question.
 */
public final class RecognizeQuestion implements AnnotationQuestion<java.util.List<Entity>,RecognizeDetail> {
    private final com.fasterxml.jackson.databind.JsonNode definition;
    private final @org.jspecify.annotations.Nullable String name;
    RecognizeQuestion(com.fasterxml.jackson.databind.JsonNode definition, @org.jspecify.annotations.Nullable String name) { this.definition = definition.deepCopy(); this.name = name; }
    com.fasterxml.jackson.databind.JsonNode definition() { return definition.deepCopy(); }
    @org.jspecify.annotations.Nullable String name() { return name; }
    /**
     * Return this question with a caller-visible name.
     * @param name the caller-visible name
     * @return the named result
     */
    public RecognizeQuestion named(String name) { return new RecognizeQuestion(definition,java.util.Objects.requireNonNull(name,"name")); }
    /**
     * Return the engine function.
     * @return the function result
     */
    public FunctionName function() { return FunctionName.RECOGNIZE; }
    /**
     * Create this typed value.
     * @param kinds the allowed recognized kinds
     * @return the of result
     */
    public static RecognizeQuestion of(java.util.List<String> kinds) { return of(kinds, RecognizeOptions.defaults()); }
    /**
     * Create this typed value.
     * @param kinds the allowed recognized kinds
     * @param options the function-specific options
     * @return the of result
     */
    public static RecognizeQuestion of(java.util.List<String> kinds, RecognizeOptions options) {
        var values = new java.util.HashMap<String,Object>(options.values());
        values.put("function","recognize");
        values.put("kinds",java.util.List.copyOf(kinds));
        return new RecognizeQuestion(Values.tree(values),options.name());
    }
}
