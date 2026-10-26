package io.thinkthen;

public final class DecideQuestion implements AnnotationQuestion<Boolean,NoDetail> {
    private final com.fasterxml.jackson.databind.JsonNode definition;
    private final @org.jspecify.annotations.Nullable String name;
    DecideQuestion(com.fasterxml.jackson.databind.JsonNode definition, @org.jspecify.annotations.Nullable String name) { this.definition = definition.deepCopy(); this.name = name; }
    com.fasterxml.jackson.databind.JsonNode definition() { return definition.deepCopy(); }
    @org.jspecify.annotations.Nullable String name() { return name; }
    public DecideQuestion named(String name) { return new DecideQuestion(definition,java.util.Objects.requireNonNull(name)); }
    public Function function() { return Function.DECIDE; }
    public static DecideQuestion of(String question) { return of(question, DecideOptions.defaults()); }
    public static DecideQuestion of(String question, DecideOptions options) {
        var values = new java.util.HashMap<String,Object>(options.values());
        values.put("function","decide");
        values.put("question",java.util.Objects.requireNonNull(question));
        return new DecideQuestion(Values.tree(values),options.name());
    }
}
