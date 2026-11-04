package dev.thinkthen;

/**
 * A record that failed, retaining the shared engine error.
 * @param info the info value
 * @param error the error value
     * @param <A> the answer type
     * @param <D> the detail type
     */
public record Failed<A,D extends Detail>(RowInfo info, RowError error) implements Row<A,D> {
    /**
     * Return the probability when present. For decide and filter, it is the probability of yes.
     * @return the probability result
     */
    public java.util.OptionalDouble probability() { return java.util.OptionalDouble.empty(); }
    /**
     * Return whether this outcome is answered, unsure or failed.
     * @return the status result
     */
    public Status status() { return Status.FAILED; }     /** Return a compact outcome without the full source identity. */
    @Override public String toString() { return "Failed[" + "code=" + error.code().wireName() + ", position=" + info.position() + "]"; }
}
