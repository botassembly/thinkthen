package io.thinkthen;

public final class FindOptions {
    private final java.util.Map<String,Object> values;
    private final @org.jspecify.annotations.Nullable String name;
    private FindOptions(Builder builder) { values = java.util.Map.copyOf(builder.values); name = builder.name; }
    java.util.Map<String,Object> values() { return values; }
    @org.jspecify.annotations.Nullable String name() { return name; }
    public static FindOptions defaults() { return builder().build(); }
    public static Builder builder() { return new Builder(); }
    public static final class Builder {
        private final java.util.Map<String,Object> values = new java.util.HashMap<>();
        private @org.jspecify.annotations.Nullable String name;
        private Builder() {}
        public Builder evidence(Pointer value) { values.put("evidence", value.value()); return this; }
        public Builder name(String value) { name = java.util.Objects.requireNonNull(value); return this; }
        public Builder threshold(Threshold.Cut value) { values.put("threshold", value instanceof Threshold.Cut cut ? cut.value() : value); return this; }
        public Builder threshold(double value) { return threshold(Threshold.cut(value)); }
        public FindOptions build() { return new FindOptions(this); }
    }
}
