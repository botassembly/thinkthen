package dev.thinkthen;

/**
 * A reusable typed relate question.
 */
public final class RelateQuestion implements AnnotationQuestion<java.util.List<Relation>,RelationDetail> {
    private final com.fasterxml.jackson.databind.JsonNode definition;
    private final @org.jspecify.annotations.Nullable String name;
    RelateQuestion(com.fasterxml.jackson.databind.JsonNode definition, @org.jspecify.annotations.Nullable String name) { this.definition = definition.deepCopy(); this.name = name; }
    com.fasterxml.jackson.databind.JsonNode definition() { return definition.deepCopy(); }
    @org.jspecify.annotations.Nullable String name() { return name; }
    /**
     * Return this question with a caller-visible name.
     * @param name the caller-visible name
     * @return the named result
     */
    public RelateQuestion named(String name) { return new RelateQuestion(definition,java.util.Objects.requireNonNull(name,"name")); }
    /**
     * Return the engine function.
     * @return the function result
     */
    public FunctionName function() { return FunctionName.RELATE; }
    /**
     * Create this typed value.
     * @param rules the allowed relation rules
     * @return the of result
     */
    public static RelateQuestion of(java.util.List<RelationRule> rules) { return of(rules, RelateOptions.defaults()); }
    /**
     * Create this typed value.
     * @param rules the allowed relation rules
     * @param options the function-specific options
     * @return the of result
     */
    public static RelateQuestion of(java.util.List<RelationRule> rules, RelateOptions options) {
        var values = new java.util.HashMap<String,Object>(options.values());
        values.put("function","relate");
        values.put("rules",rules.stream().map(RelationRule::values).toList());
        return new RelateQuestion(Values.tree(values),options.name());
    }
}
