package dev.thinkthen;

/**
 * A reusable typed find question.
 */
public final class FindQuestion implements AnnotationQuestion<java.util.Optional<Span>,NoDetail> {
    private final com.fasterxml.jackson.databind.JsonNode definition;
    private final @org.jspecify.annotations.Nullable String name;
    FindQuestion(com.fasterxml.jackson.databind.JsonNode definition, @org.jspecify.annotations.Nullable String name) { this.definition = definition.deepCopy(); this.name = name; }
    com.fasterxml.jackson.databind.JsonNode definition() { return definition.deepCopy(); }
    @org.jspecify.annotations.Nullable String name() { return name; }
    /**
     * Return this question with a caller-visible name.
     * @param name the caller-visible name
     * @return the named result
     */
    public FindQuestion named(String name) { return new FindQuestion(definition,java.util.Objects.requireNonNull(name,"name")); }
    /**
     * Return the engine function.
     * @return the function result
     */
    public FunctionName function() { return FunctionName.FIND; }
    /**
     * Create this typed value.
     * @param question the question to ask
     * @return the of result
     */
    public static FindQuestion of(String question) { return of(question, FindOptions.defaults()); }
    /**
     * Create this typed value.
     * @param question the question to ask
     * @param options the function-specific options
     * @return the of result
     */
    public static FindQuestion of(String question, FindOptions options) {
        var values = new java.util.HashMap<String,Object>(options.values());
        values.put("function","find");
        values.put("question",java.util.Objects.requireNonNull(question,"question"));
        return new FindQuestion(Values.tree(values),options.name());
    }
}
