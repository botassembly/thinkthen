package dev.thinkthen;

/**
 * A typed AnnotationFailed value.
 * @param error the error value
     * @param <A> the answer type
     * @param <D> the detail type
     */
public record AnnotationFailed<A,D extends Detail>(RowError error) implements AnnotationOutcome<A,D> {
    /**
     * Return the probability when present. For decide and filter, it is the probability of yes.
     * @return the probability result
     */
    public java.util.OptionalDouble probability() { return java.util.OptionalDouble.empty(); }
    /**
     * Return whether this outcome is answered, unsure or failed.
     * @return the status result
     */
    public Status status() { return Status.FAILED; } }
