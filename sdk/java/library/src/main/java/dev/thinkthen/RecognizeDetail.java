package dev.thinkthen;

/**
 * Recognition boundary and kind details, including matched text and UTF-16 offsets.
 * @param pieces the pieces value
 * @param names the names value
 */
public record RecognizeDetail(java.util.List<PieceDetail> pieces, java.util.List<KindDetail> names) implements Detail {
    /** Create an immutable value snapshot.
     * @param pieces the pieces value

     */
    public RecognizeDetail { pieces = java.util.List.copyOf(pieces); names = java.util.List.copyOf(names); } }
