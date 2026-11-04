package dev.thinkthen;

/**
 * A recognized mention and its probability.
 * @param mention the mention value
 * @param probability the probability value
 */
public record Entity(Mention mention, double probability) {}
