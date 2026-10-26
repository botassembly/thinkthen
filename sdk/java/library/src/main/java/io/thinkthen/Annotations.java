package io.thinkthen;

public final class Annotations {
    private final java.util.Map<String,AnnotationEntry> entries;
    Annotations(java.util.Map<String,AnnotationEntry> entries) { this.entries = java.util.Map.copyOf(entries); }
    public java.util.Map<String,AnnotationEntry> entries() { return entries; }
    public java.util.Optional<AnnotationOutcome<Boolean,NoDetail>> decide(String name) { return entries.get(name) instanceof AnnotationEntry.Decide value ? java.util.Optional.of(value.outcome()) : java.util.Optional.empty(); }
    public java.util.Optional<AnnotationOutcome<String,ChooseDetail>> choose(String name) { return entries.get(name) instanceof AnnotationEntry.Choose value ? java.util.Optional.of(value.outcome()) : java.util.Optional.empty(); }
    public java.util.Optional<AnnotationOutcome<java.util.List<String>,TagDetail>> tag(String name) { return entries.get(name) instanceof AnnotationEntry.Tag value ? java.util.Optional.of(value.outcome()) : java.util.Optional.empty(); }
    public java.util.Optional<AnnotationOutcome<Double,ScoreDetail>> score(String name) { return entries.get(name) instanceof AnnotationEntry.Score value ? java.util.Optional.of(value.outcome()) : java.util.Optional.empty(); }
    public java.util.Optional<AnnotationOutcome<Boolean,NoDetail>> filter(String name) { return entries.get(name) instanceof AnnotationEntry.Filter value ? java.util.Optional.of(value.outcome()) : java.util.Optional.empty(); }
    public java.util.Optional<AnnotationOutcome<java.util.Optional<Span>,NoDetail>> find(String name) { return entries.get(name) instanceof AnnotationEntry.Find value ? java.util.Optional.of(value.outcome()) : java.util.Optional.empty(); }
    public java.util.Optional<AnnotationOutcome<java.util.List<Entity>,RecognizeDetail>> recognize(String name) { return entries.get(name) instanceof AnnotationEntry.Recognize value ? java.util.Optional.of(value.outcome()) : java.util.Optional.empty(); }
    public java.util.Optional<AnnotationOutcome<java.util.List<Relation>,RelateDetail>> relate(String name) { return entries.get(name) instanceof AnnotationEntry.Relate value ? java.util.Optional.of(value.outcome()) : java.util.Optional.empty(); }
}
