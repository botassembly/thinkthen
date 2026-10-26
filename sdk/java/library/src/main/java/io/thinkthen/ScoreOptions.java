package io.thinkthen;

public final class ScoreOptions {
    private final java.util.Map<String,Object> values;
    private final @org.jspecify.annotations.Nullable String name;
    private ScoreOptions(Builder builder) { values = java.util.Map.copyOf(builder.values); name = builder.name; }
    java.util.Map<String,Object> values() { return values; }
    @org.jspecify.annotations.Nullable String name() { return name; }
    public static ScoreOptions defaults() { return builder().build(); }
    public static Builder builder() { return new Builder(); }
    public static final class Builder {
        private final java.util.Map<String,Object> values = new java.util.HashMap<>();
        private @org.jspecify.annotations.Nullable String name;
        private Builder() {}
        public Builder evidence(Pointer value) { values.put("evidence", value.value()); return this; }
        public Builder name(String value) { name = java.util.Objects.requireNonNull(value); return this; }
        public ScoreOptions build() { return new ScoreOptions(this); }
    }
}
