package dev.thinkthen;

/**
 * A source mention, target mention and relation with its probability.
 * @param source the source value
 * @param target the target value
 * @param relation the relation value
 * @param probability the probability value
 * @param either the either value
 */
public record Relation(Mention source, Mention target, String relation, double probability, java.util.Optional<Boolean> either) {}
