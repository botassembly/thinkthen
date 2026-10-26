package io.thinkthen;

public final class FilterOptions {
    private final java.util.Map<String,Object> values;
    private final @org.jspecify.annotations.Nullable String name;
    private FilterOptions(Builder builder) { values = java.util.Map.copyOf(builder.values); name = builder.name; }
    java.util.Map<String,Object> values() { return values; }
    @org.jspecify.annotations.Nullable String name() { return name; }
    public static FilterOptions defaults() { return builder().build(); }
    public static Builder builder() { return new Builder(); }
    public static final class Builder {
        private final java.util.Map<String,Object> values = new java.util.HashMap<>();
        private @org.jspecify.annotations.Nullable String name;
        private Builder() {}
        public Builder evidence(Pointer value) { values.put("evidence", value.value()); return this; }
        public Builder name(String value) { name = java.util.Objects.requireNonNull(value); return this; }
        public Builder threshold(Threshold value) { values.put("threshold", value instanceof Threshold.Cut cut ? cut.value() : value); return this; }
        public FilterOptions build() { return new FilterOptions(this); }
    }
}
