package io.thinkthen;

public final class FindQuestion implements AnnotationQuestion<java.util.Optional<Span>,NoDetail> {
    private final com.fasterxml.jackson.databind.JsonNode definition;
    private final @org.jspecify.annotations.Nullable String name;
    FindQuestion(com.fasterxml.jackson.databind.JsonNode definition, @org.jspecify.annotations.Nullable String name) { this.definition = definition.deepCopy(); this.name = name; }
    com.fasterxml.jackson.databind.JsonNode definition() { return definition.deepCopy(); }
    @org.jspecify.annotations.Nullable String name() { return name; }
    public FindQuestion named(String name) { return new FindQuestion(definition,java.util.Objects.requireNonNull(name)); }
    public Function function() { return Function.FIND; }
    public static FindQuestion of(String question) { return of(question, FindOptions.defaults()); }
    public static FindQuestion of(String question, FindOptions options) {
        var values = new java.util.HashMap<String,Object>(options.values());
        values.put("function","find");
        values.put("question",java.util.Objects.requireNonNull(question));
        return new FindQuestion(Values.tree(values),options.name());
    }
}
