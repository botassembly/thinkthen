package io.thinkthen;

public record Relation(Mention source, Mention target, String relation, double probability, java.util.Optional<Boolean> either) {}
