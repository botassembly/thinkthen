package dev.thinkthen;

/**
 * A reusable typed tag question.
 */
public final class TagQuestion<E> implements AnnotationQuestion<java.util.List<E>,LabelDetail> {
    private final com.fasterxml.jackson.databind.JsonNode definition;
    private final Domain<E> domain;
    private final @org.jspecify.annotations.Nullable String name;
    TagQuestion(com.fasterxml.jackson.databind.JsonNode definition, @org.jspecify.annotations.Nullable String name, Domain<E> domain) { this.domain = domain; this.definition = definition.deepCopy(); this.name = name; }
    com.fasterxml.jackson.databind.JsonNode definition() { return definition.deepCopy(); }
    @org.jspecify.annotations.Nullable String name() { return name; }
    /**
     * Return this question with a caller-visible name.
     * @param name the caller-visible name
     * @return the named result
     */
    public TagQuestion<E> named(String name) { return new TagQuestion<>(definition,java.util.Objects.requireNonNull(name,"name"),domain); }
    /**
     * Return the engine function.
     * @return the function result
     */
    public FunctionName function() { return FunctionName.TAG; }
    /**
     * Create this typed value.
     * @param question the question to ask
     * @param labels the labels in their required order
     * @return the of result
     */
    public static TagQuestion<String> of(String question, java.util.List<String> labels) { return of(question, labels, TagOptions.defaults()); }
    /**
     * Create this typed value.
     * @param question the question to ask
     * @param labels the labels in their required order
     * @param options the function-specific options
     * @return the of result
     */
    public static TagQuestion<String> of(String question, java.util.List<String> labels, TagOptions options) {
        var values = new java.util.HashMap<String,Object>(options.values());
        values.put("function","tag");
        values.put("question",java.util.Objects.requireNonNull(question,"question")); values.put("labels",Questions.options(labels,values));
        return new TagQuestion<>(Values.tree(values),options.name(),Domain.strings(labels));
    }

    Domain<E> domain() { return domain; }
    public static <E extends Enum<E>> TagQuestion<E> of(String question, Class<E> type) { return of(question,type,TagOptions.defaults()); }
    public static <E extends Enum<E>> TagQuestion<E> of(String question, Class<E> type, TagOptions options) {
        var domain = Domain.of(type);
        var raw = of(question,domain.words(),options);
        return new TagQuestion<>(raw.definition,raw.name,domain);
    }
}
