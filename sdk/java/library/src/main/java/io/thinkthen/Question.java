package io.thinkthen;

public sealed interface Question<A,D extends Detail> permits AnnotationQuestion, RankQuestion { Function function(); }
