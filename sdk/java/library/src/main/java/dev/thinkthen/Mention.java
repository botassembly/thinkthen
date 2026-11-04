package dev.thinkthen;

/**
 * A matched span and its recognized kind.
 * @param span the span value
 * @param kind the kind value
 */
public record Mention(Span span, String kind) {}
