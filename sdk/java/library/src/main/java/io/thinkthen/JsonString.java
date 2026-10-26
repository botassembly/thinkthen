package io.thinkthen;

public record JsonString(String value) implements JsonValue { public JsonString { java.util.Objects.requireNonNull(value); } public String toString() { return Values.stringify(value); } }
