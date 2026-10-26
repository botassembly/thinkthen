package io.thinkthen;

public record ItemRow<T,A,D extends Detail>(@org.jspecify.annotations.Nullable T item, Row<A,D> row) {}
