package dev.thinkthen;

/**
 * A typed AnnotationUnsure value.
 * @param probability the probability value
 * @param detail the detail value
     * @param <A> the answer type
     * @param <D> the detail type
     */
public record AnnotationUnsure<A,D extends Detail>(java.util.OptionalDouble probability, java.util.Optional<D> detail) implements AnnotationOutcome<A,D> {
    /**
     * Return whether this outcome is answered, unsure or failed.
     * @return the status result
     */
    public Status status() { return Status.UNSURE; } }
