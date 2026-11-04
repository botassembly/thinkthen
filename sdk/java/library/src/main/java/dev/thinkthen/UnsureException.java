package dev.thinkthen;

/** Reading an unsure answer requires the caller to choose how to handle uncertainty. */
public final class UnsureException extends java.util.NoSuchElementException {
    private static final long serialVersionUID = 1L;
    /** The probability reported for the unsure answer. */
    private final double probability;
    /** The probability reported by the unsure row.
     * @return the probability result
     */
    public double probability() { return probability; }
    /** Create an exception for an unsure row or annotation outcome.
     * @param probability the probability value
     */
    public UnsureException(double probability) { super("The answer is unsure; check isUnsure() before reading it"); this.probability = probability; }
}
