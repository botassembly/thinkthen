package dev.thinkthen;

/**
 * Label probabilities in the order of the question’s choices or labels.
 * @param labels the labels value
 */
public record LabelDetail(java.util.Map<String,Double> labels) implements Detail {
    /** Create an immutable value snapshot.
     * @param labels the labels value

     */
    public LabelDetail { labels = Values.orderedMap(labels); } }
