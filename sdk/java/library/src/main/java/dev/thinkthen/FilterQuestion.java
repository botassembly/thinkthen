package dev.thinkthen;

/**
 * A reusable typed filter question.
 */
public final class FilterQuestion implements AnnotationQuestion<Boolean,NoDetail> {
    private final com.fasterxml.jackson.databind.JsonNode definition;
    private final @org.jspecify.annotations.Nullable String name;
    FilterQuestion(com.fasterxml.jackson.databind.JsonNode definition, @org.jspecify.annotations.Nullable String name) { this.definition = definition.deepCopy(); this.name = name; }
    com.fasterxml.jackson.databind.JsonNode definition() { return definition.deepCopy(); }
    @org.jspecify.annotations.Nullable String name() { return name; }
    /**
     * Return this question with a caller-visible name.
     * @param name the caller-visible name
     * @return the named result
     */
    public FilterQuestion named(String name) { return new FilterQuestion(definition,java.util.Objects.requireNonNull(name,"name")); }
    /**
     * Return the engine function.
     * @return the function result
     */
    public FunctionName function() { return FunctionName.FILTER; }
    /**
     * Create this typed value.
     * @param question the question to ask
     * @return the of result
     */
    public static FilterQuestion of(String question) { return of(question, FilterOptions.defaults()); }
    /**
     * Create this typed value.
     * @param question the question to ask
     * @param options the function-specific options
     * @return the of result
     */
    public static FilterQuestion of(String question, FilterOptions options) {
        var values = new java.util.HashMap<String,Object>(options.values());
        values.put("function","filter");
        values.put("question",java.util.Objects.requireNonNull(question,"question"));
        return new FilterQuestion(Values.tree(values),options.name());
    }
}
