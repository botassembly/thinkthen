package io.thinkthen;

public final class AnnotationQuestions {
    private final java.util.Map<String,AnnotationQuestion<?,?>> entries;
    private AnnotationQuestions(Builder builder) { entries = java.util.Map.copyOf(builder.entries); }
    java.util.Map<String,Object> definitions() {
        var result = new java.util.LinkedHashMap<String,Object>();
        entries.forEach((name,question) -> result.put(name,Questions.definition(question)));
        return result;
    }
    public static Builder builder() { return new Builder(); }
    public static final class Builder {
        private final java.util.Map<String,AnnotationQuestion<?,?>> entries = new java.util.LinkedHashMap<>();
        private Builder() {}
        public Builder put(String name, AnnotationQuestion<?,?> question) { entries.put(name,question); return this; }
        public AnnotationQuestions build() { return new AnnotationQuestions(this); }
    }
}
