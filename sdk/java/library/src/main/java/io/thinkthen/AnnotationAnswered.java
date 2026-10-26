package io.thinkthen;

public record AnnotationAnswered<A,D extends Detail>(A answer, java.util.OptionalDouble probability, java.util.Optional<D> detail) implements AnnotationOutcome<A,D> { public Status status() { return Status.ANSWERED; } }
