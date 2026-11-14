package dev.thinkthen;

/** An immutable answered result. The original item is excluded from value equality. */
public record Answered<A,T>(RowInfo info, @org.jspecify.annotations.Nullable T item, A answer, java.util.OptionalDouble probability, Detail detail) implements Row<A,T> {
    public Status status() { return Status.ANSWERED; }

    @Override public boolean equals(Object other) { return ResultValues.equal(this,other); }
    @Override public int hashCode() { return ResultValues.hash(this); }
    @Override public String toString() { return ResultValues.print(this); }
}
