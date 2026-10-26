package io.thinkthen;

public record JsonBoolean(boolean value) implements JsonValue { public String toString() { return Boolean.toString(value); } }
