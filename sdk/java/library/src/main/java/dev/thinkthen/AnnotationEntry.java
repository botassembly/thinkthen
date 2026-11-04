package dev.thinkthen;

/**
 * A named annotation outcome whose type identifies its function.
 */
public sealed interface AnnotationEntry permits AnnotationEntry.Decide, AnnotationEntry.Choose, AnnotationEntry.Tag, AnnotationEntry.Score, AnnotationEntry.Filter, AnnotationEntry.Find, AnnotationEntry.Recognize, AnnotationEntry.Relate {
    /** A typed AnnotationEntry member.
     * @param outcome the outcome value
     */
    public record Decide(AnnotationOutcome<Boolean,NoDetail> outcome) implements AnnotationEntry {}
    /** A typed AnnotationEntry member.
     * @param outcome the outcome value
     */
    public record Choose(AnnotationOutcome<String,LabelDetail> outcome) implements AnnotationEntry {}
    /** A typed AnnotationEntry member.
     * @param outcome the outcome value
     */
    public record Tag(AnnotationOutcome<java.util.List<String>,LabelDetail> outcome) implements AnnotationEntry {}
    /** A typed AnnotationEntry member.
     * @param outcome the outcome value
     */
    public record Score(AnnotationOutcome<Double,LevelDetail> outcome) implements AnnotationEntry {}
    /** A typed AnnotationEntry member.
     * @param outcome the outcome value
     */
    public record Filter(AnnotationOutcome<Boolean,NoDetail> outcome) implements AnnotationEntry {}
    /** A typed AnnotationEntry member.
     * @param outcome the outcome value
     */
    public record Find(AnnotationOutcome<java.util.Optional<Span>,NoDetail> outcome) implements AnnotationEntry {}
    /** A typed AnnotationEntry member.
     * @param outcome the outcome value
     */
    public record Recognize(AnnotationOutcome<java.util.List<Entity>,RecognizeDetail> outcome) implements AnnotationEntry {}
    /** A typed AnnotationEntry member.
     * @param outcome the outcome value
     */
    public record Relate(AnnotationOutcome<java.util.List<Relation>,RelationDetail> outcome) implements AnnotationEntry {}
}
