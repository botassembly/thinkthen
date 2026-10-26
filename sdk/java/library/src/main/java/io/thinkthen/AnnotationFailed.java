package io.thinkthen;

public record AnnotationFailed<A,D extends Detail>(RowError error) implements AnnotationOutcome<A,D> { public Status status() { return Status.FAILED; } }
