package io.thinkthen;

public record ItemRow<T,A,D extends Detail>(@org.jspecify.annotations.Nullable T item, Row<A,D> row) {
    /** Whether this filter result keeps its original item. */
    public boolean kept() {
        return row instanceof Answered<?,?> answered && Boolean.TRUE.equals(answered.answer());
    }
}
