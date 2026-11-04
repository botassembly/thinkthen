package dev.thinkthen;

/**
 * Matched text and its start-inclusive, end-exclusive UTF-16 offsets.
 * @param text the text value
 * @param start the start value
 * @param end the end value
 */
public record Span(String text, long start, long end) {}
