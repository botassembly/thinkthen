package io.thinkthen;

public sealed interface AnnotationEntry permits AnnotationEntry.Decide, AnnotationEntry.Choose, AnnotationEntry.Tag, AnnotationEntry.Score, AnnotationEntry.Filter, AnnotationEntry.Find, AnnotationEntry.Recognize, AnnotationEntry.Relate {
    record Decide(AnnotationOutcome<Boolean,NoDetail> outcome) implements AnnotationEntry {}
    record Choose(AnnotationOutcome<String,ChooseDetail> outcome) implements AnnotationEntry {}
    record Tag(AnnotationOutcome<java.util.List<String>,TagDetail> outcome) implements AnnotationEntry {}
    record Score(AnnotationOutcome<Double,ScoreDetail> outcome) implements AnnotationEntry {}
    record Filter(AnnotationOutcome<Boolean,NoDetail> outcome) implements AnnotationEntry {}
    record Find(AnnotationOutcome<java.util.Optional<Span>,NoDetail> outcome) implements AnnotationEntry {}
    record Recognize(AnnotationOutcome<java.util.List<Entity>,RecognizeDetail> outcome) implements AnnotationEntry {}
    record Relate(AnnotationOutcome<java.util.List<Relation>,RelateDetail> outcome) implements AnnotationEntry {}
}
