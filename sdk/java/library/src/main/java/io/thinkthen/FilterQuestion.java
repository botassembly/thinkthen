package io.thinkthen;

public final class FilterQuestion implements AnnotationQuestion<Boolean,NoDetail> {
    private final com.fasterxml.jackson.databind.JsonNode definition;
    private final @org.jspecify.annotations.Nullable String name;
    FilterQuestion(com.fasterxml.jackson.databind.JsonNode definition, @org.jspecify.annotations.Nullable String name) { this.definition = definition.deepCopy(); this.name = name; }
    com.fasterxml.jackson.databind.JsonNode definition() { return definition.deepCopy(); }
    @org.jspecify.annotations.Nullable String name() { return name; }
    public FilterQuestion named(String name) { return new FilterQuestion(definition,java.util.Objects.requireNonNull(name)); }
    public Function function() { return Function.FILTER; }
    public static FilterQuestion of(String question) { return of(question, FilterOptions.defaults()); }
    public static FilterQuestion of(String question, FilterOptions options) {
        var values = new java.util.HashMap<String,Object>(options.values());
        values.put("function","filter");
        values.put("question",java.util.Objects.requireNonNull(question));
        return new FilterQuestion(Values.tree(values),options.name());
    }
}
