package io.thinkthen;

public record Pointer(String value) { public static Pointer of(String value) { return new Pointer(java.util.Objects.requireNonNull(value)); } }
