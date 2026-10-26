package io.thinkthen;

public record Unsure<A,D extends Detail>(RowInfo info, double probability, java.util.Optional<D> detail) implements Row<A,D> { public Status status() { return Status.UNSURE; } }
