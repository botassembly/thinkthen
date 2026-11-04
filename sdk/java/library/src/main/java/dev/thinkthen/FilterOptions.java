package dev.thinkthen;

/**
 * Immutable options for filter calls.
 */
public final class FilterOptions {
    private final java.util.Map<String,Object> values;
    private final @org.jspecify.annotations.Nullable String name;
    private FilterOptions(Builder builder) { values = Values.orderedMap(builder.values); name = builder.name; }
    java.util.Map<String,Object> values() { return values; }
    @org.jspecify.annotations.Nullable String name() { return name; }
    /**
     * Return the default options.
     * @return the defaults result
     */
    public static FilterOptions defaults() { return builder().build(); }
    /**
     * Start a builder for this value.
     * @return the builder result
     */
    public static Builder builder() { return new Builder(); }
    /**
     * Build a FilterOptions value without changing an existing value.
     */
    public static final class Builder {
        private final java.util.Map<String,Object> values = new java.util.HashMap<>();
        private @org.jspecify.annotations.Nullable String name;
        private Builder() {}
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
        /** Describe what a yes answer means for this question.
     * @param value the value value
     * @return the yesMeans result
     */
        public Builder yesMeans(String value) { values.put("true",java.util.Objects.requireNonNull(value,"yesMeans")); return this; }
        /** Describe what a no answer means for this question.
     * @param value the value value
     * @return the noMeans result
     */
        public Builder noMeans(String value) { values.put("false",java.util.Objects.requireNonNull(value,"noMeans")); return this; }
        /**
         * Set the cutoff or uncertainty band.
         * @param value the value to set
         * @return the threshold result
         */
        public Builder threshold(Threshold value) { values.put("threshold", value instanceof Threshold.Cut cut ? cut.value() : value); return this; }
        /**
         * Build this immutable value.
         * @return the build result
         */
        public FilterOptions build() { return new FilterOptions(this); }
    }
}
