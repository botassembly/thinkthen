package io.thinkthen;

public final class RankQuestion implements Question<Long,RankDetail> {
    private final com.fasterxml.jackson.databind.JsonNode definition;
    private final @org.jspecify.annotations.Nullable String name;
    RankQuestion(com.fasterxml.jackson.databind.JsonNode definition, @org.jspecify.annotations.Nullable String name) { this.definition = definition.deepCopy(); this.name = name; }
    com.fasterxml.jackson.databind.JsonNode definition() { return definition.deepCopy(); }
    @org.jspecify.annotations.Nullable String name() { return name; }
    public RankQuestion named(String name) { return new RankQuestion(definition,java.util.Objects.requireNonNull(name)); }
    public Function function() { return Function.RANK; }
    public static RankQuestion of(String question, java.util.List<String> levels) { return of(question, levels, RankOptions.defaults()); }
    public static RankQuestion of(String question, java.util.List<String> levels, RankOptions options) {
        var values = new java.util.HashMap<String,Object>(options.values());
        values.put("function","rank");
        values.put("question",java.util.Objects.requireNonNull(question)); values.put("levels",java.util.List.copyOf(levels));
        return new RankQuestion(Values.tree(values),options.name());
    }
}
