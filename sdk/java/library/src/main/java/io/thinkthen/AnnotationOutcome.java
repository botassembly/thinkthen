package io.thinkthen;

public sealed interface AnnotationOutcome<A,D extends Detail> permits AnnotationAnswered, AnnotationUnsure, AnnotationFailed { Status status(); }
