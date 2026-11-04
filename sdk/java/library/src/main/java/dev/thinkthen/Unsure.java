package dev.thinkthen;

/**
 * A record whose probability falls inside the configured uncertainty band.
 * @param info the info value
 * @param probability the probability value
 * @param detail the detail value
     * @param <A> the answer type
     * @param <D> the detail type
     */
public record Unsure<A,D extends Detail>(RowInfo info, java.util.OptionalDouble probability, java.util.Optional<D> detail) implements Row<A,D> {
    /**
     * Return whether this outcome is answered, unsure or failed.
     * @return the status result
     */
    public Status status() { return Status.UNSURE; }     /** Return a compact outcome without the full source identity. */
    @Override public String toString() { return "Unsure[" + "probability=" + probability + ", position=" + info.position() + "]"; }
}
