package dev.thinkthen;

/**
 * Immutable options for tag calls.
 */
public final class TagOptions {
    private final java.util.Map<String,Object> values;
    private final @org.jspecify.annotations.Nullable String name;
    private TagOptions(Builder builder) { values = Values.orderedMap(builder.values); name = builder.name; }
    java.util.Map<String,Object> values() { return values; }
    @org.jspecify.annotations.Nullable String name() { return name; }
    /**
     * Return the default options.
     * @return the defaults result
     */
    public static TagOptions defaults() { return builder().build(); }
    /**
     * Start a builder for this value.
     * @return the builder result
     */
    public static Builder builder() { return new Builder(); }
    /**
     * Build a TagOptions value without changing an existing value.
     */
    public static final class Builder {
        private final java.util.Map<String,Object> values = new java.util.HashMap<>();
        private @org.jspecify.annotations.Nullable String name;
        private Builder() {}
        /** Describe each named label or level without changing its public answer name.
     * @param value the value value
     * @return the descriptions result
     */
        public Builder descriptions(java.util.Map<String,String> value) { values.put("descriptions",Values.orderedMap(value)); return this; }
        /**
         * Select record field with a JSON pointer.
         * @param value the value to set
         * @return the field result
         */
        public Builder field(Pointer value) { values.put("field", value.value()); return this; }
        /**
         * Return the name value.
         * @param value the value to set
         * @return the name result
         */
        public Builder name(String value) { name = java.util.Objects.requireNonNull(value,"value"); return this; }
        /**
         * Set the cutoff or uncertainty band.
         * @param value the value to set
         * @return the threshold result
         */
        public Builder threshold(Threshold.Cut value) { values.put("threshold", value instanceof Threshold.Cut cut ? cut.value() : value); return this; }
        /**
         * Set the cutoff or uncertainty band.
         * @param value the value to set
         * @return the threshold result
         */
        public Builder threshold(double value) { return threshold(Threshold.cut(value)); }
        /**
         * Build this immutable value.
         * @return the build result
         */
        public TagOptions build() { return new TagOptions(this); }
    }
}
