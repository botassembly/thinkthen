package dev.thinkthen;

/** An immutable unsure result. The original item is excluded from value equality. */
public record Unsure<A,T>(RowInfo info, @org.jspecify.annotations.Nullable T item, java.util.OptionalDouble probability, Detail detail) implements Row<A,T> {
    public Status status() { return Status.UNSURE; }

    @Override public boolean equals(Object other) { return ResultValues.equal(this,other); }
    @Override public int hashCode() { return ResultValues.hash(this); }
    @Override public String toString() { return ResultValues.print(this); }
}
