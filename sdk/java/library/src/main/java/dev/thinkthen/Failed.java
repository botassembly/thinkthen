package dev.thinkthen;

/** An immutable failed result. The original item is excluded from value equality. */
public record Failed<A,T>(RowInfo info, @org.jspecify.annotations.Nullable T item, RowError error) implements Row<A,T> {
    public Status status() { return Status.FAILED; }
    public java.util.OptionalDouble probability() { return java.util.OptionalDouble.empty(); }
    public Detail detail() { return NoDetail.INSTANCE; }
    @Override public boolean equals(Object other) { return ResultValues.equal(this,other); }
    @Override public int hashCode() { return ResultValues.hash(this); }
    @Override public String toString() { return ResultValues.print(this); }
}
