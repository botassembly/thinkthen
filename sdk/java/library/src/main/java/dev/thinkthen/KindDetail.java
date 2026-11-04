package dev.thinkthen;

/**
 * The kind probabilities for one matched span.
 * @param text the matched text
 * @param start the start value
 * @param end the end value
 * @param kinds the kinds value
 */
public record KindDetail(String text, long start, long end, java.util.Map<String,Double> kinds) {
    /** Create an immutable value snapshot.
     * @param kinds the kinds value

     */
    public KindDetail { kinds = Values.orderedMap(kinds); } }
