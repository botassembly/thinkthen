package io.thinkthen;

public final class ChooseQuestion implements AnnotationQuestion<String,ChooseDetail> {
    private final com.fasterxml.jackson.databind.JsonNode definition;
    private final @org.jspecify.annotations.Nullable String name;
    ChooseQuestion(com.fasterxml.jackson.databind.JsonNode definition, @org.jspecify.annotations.Nullable String name) { this.definition = definition.deepCopy(); this.name = name; }
    com.fasterxml.jackson.databind.JsonNode definition() { return definition.deepCopy(); }
    @org.jspecify.annotations.Nullable String name() { return name; }
    public ChooseQuestion named(String name) { return new ChooseQuestion(definition,java.util.Objects.requireNonNull(name)); }
    public Function function() { return Function.CHOOSE; }
    public static ChooseQuestion of(String question, java.util.List<String> choices) { return of(question, choices, ChooseOptions.defaults()); }
    public static ChooseQuestion of(String question, java.util.List<String> choices, ChooseOptions options) {
        var values = new java.util.HashMap<String,Object>(options.values());
        values.put("function","choose");
        values.put("question",java.util.Objects.requireNonNull(question)); values.put("choices",java.util.List.copyOf(choices));
        return new ChooseQuestion(Values.tree(values),options.name());
    }
}
