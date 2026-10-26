package io.thinkthen;

public record AnnotationUnsure<A,D extends Detail>(double probability, java.util.Optional<D> detail) implements AnnotationOutcome<A,D> { public Status status() { return Status.UNSURE; } }
