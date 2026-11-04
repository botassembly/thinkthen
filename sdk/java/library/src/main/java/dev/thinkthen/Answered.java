package dev.thinkthen;

/**
 * An answered record. A false Boolean is an ordinary usable answer.
 * @param info the info value
 * @param answer the answer value
 * @param probability the probability value
 * @param detail the detail value
     * @param <A> the answer type
     * @param <D> the detail type
     */
public record Answered<A,D extends Detail>(RowInfo info, A answer, java.util.OptionalDouble probability, java.util.Optional<D> detail) implements Row<A,D> {
    /**
     * Return whether this outcome is answered, unsure or failed.
     * @return the status result
     */
    public Status status() { return Status.ANSWERED; }     /** Return a compact outcome without the full source identity. */
    @Override public String toString() { return "Answered[" + "answer=" + answer + ", probability=" + probability + ", position=" + info.position() + "]"; }
}
