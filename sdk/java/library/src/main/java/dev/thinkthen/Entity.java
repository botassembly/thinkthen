package dev.thinkthen;

/**
 * A recognized mention and its probability.
 * @param mention the mention value
 * @param probability the probability value
 */
public record Entity(Mention mention, double probability) {
    public String kind() { return mention.kind(); }
    public String text() { return mention.span().text(); }
    public int start() { return mention.span().start(); }
    public int end() { return mention.span().end(); }
}
