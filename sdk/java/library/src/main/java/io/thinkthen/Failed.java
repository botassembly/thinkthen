package io.thinkthen;

public record Failed<A,D extends Detail>(RowInfo info, RowError error) implements Row<A,D> { public Status status() { return Status.FAILED; } }
