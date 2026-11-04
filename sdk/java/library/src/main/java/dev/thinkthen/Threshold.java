package dev.thinkthen;

/**
 * A decision cutoff or uncertainty band validated by the shared engine.
 */
public sealed interface Threshold permits Threshold.Cut, Threshold.Band {
    /** A typed Threshold member.
     * @param value the value value
     */
    public record Cut(double value) implements Threshold {}
    /** A typed Threshold member.
     * @param low the low value
     * @param high the high value
     */
    public record Band(double low, double high) implements Threshold {}
    /** A typed Threshold member.
     * @param value the value value
     * @return the cut result
     */
    public static Cut cut(double value) { return new Cut(value); }
    /** A typed Threshold member.
     * @param low the low value
     * @param high the high value
     * @return the band result
     */
    public static Band band(double low, double high) { return new Band(low, high); }
}
