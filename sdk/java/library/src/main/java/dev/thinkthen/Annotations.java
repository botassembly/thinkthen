package dev.thinkthen;

/**
 * Named, typed annotation outcomes in the caller’s question order.
 */
public final class Annotations {
    private final java.util.Map<String,AnnotationEntry> entries;
    Annotations(java.util.Map<String,AnnotationEntry> entries) { this.entries = Values.orderedMap(entries); }
    /**
     * Return the immutable entries in insertion order.
     * @return the entries result
     */
    public java.util.Map<String,AnnotationEntry> entries() { return entries; }
    /**
     * Ask whether the record answers yes.
     * @param name the caller-visible name
     * @return the decide result
     */
    public AnnotationOutcome<Boolean,NoDetail> decide(String name) { if (entries.get(name) instanceof AnnotationEntry.Decide value) return value.outcome(); throw selectorFailure("decide", name); }
    /**
     * Choose one label for the record.
     * @param name the caller-visible name
     * @return the choose result
     */
    public AnnotationOutcome<String,LabelDetail> choose(String name) { if (entries.get(name) instanceof AnnotationEntry.Choose value) return value.outcome(); throw selectorFailure("choose", name); }
    /**
     * Select the matching labels for the record.
     * @param name the caller-visible name
     * @return the tag result
     */
    public AnnotationOutcome<java.util.List<String>,LabelDetail> tag(String name) { if (entries.get(name) instanceof AnnotationEntry.Tag value) return value.outcome(); throw selectorFailure("tag", name); }
    /**
     * Score the record against the required levels.
     * @param name the caller-visible name
     * @return the score result
     */
    public AnnotationOutcome<Double,LevelDetail> score(String name) { if (entries.get(name) instanceof AnnotationEntry.Score value) return value.outcome(); throw selectorFailure("score", name); }
    /**
     * Return the filter value.
     * @param name the caller-visible name
     * @return the filter result
     */
    public AnnotationOutcome<Boolean,NoDetail> filter(String name) { if (entries.get(name) instanceof AnnotationEntry.Filter value) return value.outcome(); throw selectorFailure("filter", name); }
    /**
     * Find a matching text span.
     * @param name the caller-visible name
     * @return the find result
     */
    public AnnotationOutcome<java.util.Optional<Span>,NoDetail> find(String name) { if (entries.get(name) instanceof AnnotationEntry.Find value) return value.outcome(); throw selectorFailure("find", name); }
    /**
     * Recognize typed mentions in the record.
     * @param name the caller-visible name
     * @return the recognize result
     */
    public AnnotationOutcome<java.util.List<Entity>,RecognizeDetail> recognize(String name) { if (entries.get(name) instanceof AnnotationEntry.Recognize value) return value.outcome(); throw selectorFailure("recognize", name); }
    /**
     * Find the allowed relations in the record.
     * @param name the caller-visible name
     * @return the relate result
     */
    public AnnotationOutcome<java.util.List<Relation>,RelationDetail> relate(String name) { if (entries.get(name) instanceof AnnotationEntry.Relate value) return value.outcome(); throw selectorFailure("relate", name); }
    private ThinkThenException selectorFailure(String function, String name) {
        return new ThinkThenException(new RowError(ErrorCode.fromWire("usage.argument"),
                "No " + function + " annotation named " + name + "; available names: " + entries.keySet(),
                java.util.OptionalInt.empty()), null);
    }
}
