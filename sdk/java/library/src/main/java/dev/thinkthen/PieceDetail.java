package dev.thinkthen;

/**
 * Boundary-tag probabilities for a matched text piece.
 * @param span the span value
 * @param tags the tags value
 */
public record PieceDetail(Span span, java.util.Map<BoundaryTag,Double> tags) {
    /** Create an immutable value snapshot.
     * @param tags the tags value

     */
    public PieceDetail { tags = Values.orderedMap(tags); } }
