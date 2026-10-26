package io.thinkthen;

public final class RelationRule {
    private final java.util.Map<String,Object> values;
    private RelationRule(Builder builder) { values = java.util.Map.copyOf(builder.values); }
    java.util.Map<String,Object> values() { return values; }
    public static Builder builder(String source, String target, String relation) { return new Builder(source,target,relation); }
    public static final class Builder {
        private final java.util.Map<String,Object> values = new java.util.HashMap<>();
        private Builder(String source,String target,String relation) { values.put("source",source); values.put("target",target); values.put("relation",relation); }
        public Builder reads(String value) { values.put("reads",value); return this; }
        public Builder either(boolean value) { values.put("either",value); return this; }
        public RelationRule build() { return new RelationRule(this); }
    }
}
