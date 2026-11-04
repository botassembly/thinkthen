package dev.thinkthen;

/**
 * A relation between source and target kinds, with optional reading and direction restrictions.
 */
public final class RelationRule {
    private final java.util.Map<String,Object> values;
    private RelationRule(Builder builder) { values = Values.orderedMap(builder.values); }
    java.util.Map<String,Object> values() { return values; }
    /** Build a relation rule without optional restrictions.
     * @param source the source value
     * @param target the target value
     * @param relation the relation value
     * @return the of result
     */
    public static RelationRule of(String source, String target, String relation) { return builder(source,target,relation).build(); }
    /**
     * Start a builder for this value.
     * @param source the source kind
     * @param target the target kind
     * @param relation the relation name
     * @return the builder result
     */
    public static Builder builder(String source, String target, String relation) { return new Builder(source,target,relation); }
    /**
     * Build a RelationRule value without changing an existing value.
     */
    public static final class Builder {
        private final java.util.Map<String,Object> values = new java.util.HashMap<>();
        private Builder(String source,String target,String relation) { values.put("source",source); values.put("target",target); values.put("relation",relation); }
        /**
         * Set the relation’s reading description.
         * @param value the value to set
         * @return the reads result
         */
        public Builder reads(String value) { values.put("reads",value); return this; }
        /**
         * Set whether the relation can be read in either direction.
         * @param value the value to set
         * @return the either result
         */
        public Builder either(boolean value) { values.put("either",value); return this; }
        /**
         * Build this immutable value.
         * @return the build result
         */
        public RelationRule build() { return new RelationRule(this); }
    }
}
