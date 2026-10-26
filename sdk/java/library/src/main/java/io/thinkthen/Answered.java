package io.thinkthen;

public record Answered<A,D extends Detail>(RowInfo info, A answer, java.util.OptionalDouble probability, java.util.Optional<D> detail) implements Row<A,D> { public Status status() { return Status.ANSWERED; } }
