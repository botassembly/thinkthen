package dev.thinkthen;

/**
 * A reusable typed choose question.
 */
public final class ChooseQuestion implements AnnotationQuestion<String,LabelDetail> {
    private final com.fasterxml.jackson.databind.JsonNode definition;
    private final @org.jspecify.annotations.Nullable String name;
    ChooseQuestion(com.fasterxml.jackson.databind.JsonNode definition, @org.jspecify.annotations.Nullable String name) { this.definition = definition.deepCopy(); this.name = name; }
    com.fasterxml.jackson.databind.JsonNode definition() { return definition.deepCopy(); }
    @org.jspecify.annotations.Nullable String name() { return name; }
    /**
     * Return this question with a caller-visible name.
     * @param name the caller-visible name
     * @return the named result
     */
    public ChooseQuestion named(String name) { return new ChooseQuestion(definition,java.util.Objects.requireNonNull(name,"name")); }
    /**
     * Return the engine function.
     * @return the function result
     */
    public FunctionName function() { return FunctionName.CHOOSE; }
    /**
     * Create this typed value.
     * @param question the question to ask
     * @param choices the allowed answer labels in their required order
     * @return the of result
     */
    public static ChooseQuestion of(String question, java.util.List<String> choices) { return of(question, choices, ChooseOptions.defaults()); }
    /**
     * Create this typed value.
     * @param question the question to ask
     * @param choices the allowed answer labels in their required order
     * @param options the function-specific options
     * @return the of result
     */
    public static ChooseQuestion of(String question, java.util.List<String> choices, ChooseOptions options) {
        var values = new java.util.HashMap<String,Object>(options.values());
        values.put("function","choose");
        values.put("question",java.util.Objects.requireNonNull(question,"question")); values.put("choices",Questions.options(choices,values));
        return new ChooseQuestion(Values.tree(values),options.name());
    }
}
