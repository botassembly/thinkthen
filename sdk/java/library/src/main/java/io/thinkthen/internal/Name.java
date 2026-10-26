package io.thinkthen.internal;
public record Name(String text, int start, int end, String kind, @com.fasterxml.jackson.annotation.JsonProperty("probability") double p) {}
