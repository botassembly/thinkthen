package dev.thinkthen;

/**
 * An ordered set of named questions for one annotation call.
 */
public final class AnnotationQuestions {
    private final java.util.Map<String,AnnotationQuestion<?,?>> entries;
    private AnnotationQuestions(Builder builder) { entries = Values.orderedMap(builder.entries); }
    java.util.Map<String,Object> definitions() {
        var result = new java.util.LinkedHashMap<String,Object>();
        entries.forEach((name,question) -> result.put(name,Questions.definition(question)));
        return result;
    }
    /**
     * Start a builder for this value.
     * @return the builder result
     */
    public static Builder builder() { return new Builder(); }
    /**
     * Build a AnnotationQuestions value without changing an existing value.
     */
    public static final class Builder {
        private final java.util.Map<String,AnnotationQuestion<?,?>> entries = new java.util.LinkedHashMap<>();
        private Builder() {}
        /**
         * Add a named question. Repeated annotation names are refused.
         * @param name the caller-visible name
         * @param question the question to ask
         * @return the put result
         */
        public Builder put(String name, AnnotationQuestion<?,?> question) { if (entries.putIfAbsent(java.util.Objects.requireNonNull(name,"name"),java.util.Objects.requireNonNull(question,"question")) != null) throw new IllegalArgumentException("Repeated annotation name: " + name); return this; }
        /**
         * Build this immutable value.
         * @return the build result
         */
        public AnnotationQuestions build() { return new AnnotationQuestions(this); }
    }
}
