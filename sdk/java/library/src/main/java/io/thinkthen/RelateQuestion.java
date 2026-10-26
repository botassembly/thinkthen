package io.thinkthen;

public final class RelateQuestion implements AnnotationQuestion<java.util.List<Relation>,RelateDetail> {
    private final com.fasterxml.jackson.databind.JsonNode definition;
    private final @org.jspecify.annotations.Nullable String name;
    RelateQuestion(com.fasterxml.jackson.databind.JsonNode definition, @org.jspecify.annotations.Nullable String name) { this.definition = definition.deepCopy(); this.name = name; }
    com.fasterxml.jackson.databind.JsonNode definition() { return definition.deepCopy(); }
    @org.jspecify.annotations.Nullable String name() { return name; }
    public RelateQuestion named(String name) { return new RelateQuestion(definition,java.util.Objects.requireNonNull(name)); }
    public Function function() { return Function.RELATE; }
    public static RelateQuestion of(java.util.List<RelationRule> rules) { return of(rules, RelateOptions.defaults()); }
    public static RelateQuestion of(java.util.List<RelationRule> rules, RelateOptions options) {
        var values = new java.util.HashMap<String,Object>(options.values());
        values.put("function","relate");
        values.put("rules",rules.stream().map(RelationRule::values).toList());
        return new RelateQuestion(Values.tree(values),options.name());
    }
}
