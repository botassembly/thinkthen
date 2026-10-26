package io.thinkthen;

public final class RecognizeQuestion implements AnnotationQuestion<java.util.List<Entity>,RecognizeDetail> {
    private final com.fasterxml.jackson.databind.JsonNode definition;
    private final @org.jspecify.annotations.Nullable String name;
    RecognizeQuestion(com.fasterxml.jackson.databind.JsonNode definition, @org.jspecify.annotations.Nullable String name) { this.definition = definition.deepCopy(); this.name = name; }
    com.fasterxml.jackson.databind.JsonNode definition() { return definition.deepCopy(); }
    @org.jspecify.annotations.Nullable String name() { return name; }
    public RecognizeQuestion named(String name) { return new RecognizeQuestion(definition,java.util.Objects.requireNonNull(name)); }
    public Function function() { return Function.RECOGNIZE; }
    public static RecognizeQuestion of(java.util.List<String> kinds) { return of(kinds, RecognizeOptions.defaults()); }
    public static RecognizeQuestion of(java.util.List<String> kinds, RecognizeOptions options) {
        var values = new java.util.HashMap<String,Object>(options.values());
        values.put("function","recognize");
        values.put("kinds",java.util.List.copyOf(kinds));
        return new RecognizeQuestion(Values.tree(values),options.name());
    }
}
