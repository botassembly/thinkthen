package io.thinkthen;

public final class TagQuestion implements AnnotationQuestion<java.util.List<String>,TagDetail> {
    private final com.fasterxml.jackson.databind.JsonNode definition;
    private final @org.jspecify.annotations.Nullable String name;
    TagQuestion(com.fasterxml.jackson.databind.JsonNode definition, @org.jspecify.annotations.Nullable String name) { this.definition = definition.deepCopy(); this.name = name; }
    com.fasterxml.jackson.databind.JsonNode definition() { return definition.deepCopy(); }
    @org.jspecify.annotations.Nullable String name() { return name; }
    public TagQuestion named(String name) { return new TagQuestion(definition,java.util.Objects.requireNonNull(name)); }
    public Function function() { return Function.TAG; }
    public static TagQuestion of(String question, java.util.List<String> labels) { return of(question, labels, TagOptions.defaults()); }
    public static TagQuestion of(String question, java.util.List<String> labels, TagOptions options) {
        var values = new java.util.HashMap<String,Object>(options.values());
        values.put("function","tag");
        values.put("question",java.util.Objects.requireNonNull(question)); values.put("labels",java.util.List.copyOf(labels));
        return new TagQuestion(Values.tree(values),options.name());
    }
}
