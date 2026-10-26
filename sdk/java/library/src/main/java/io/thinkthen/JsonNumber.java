package io.thinkthen;

public record JsonNumber(java.math.BigDecimal value) implements JsonValue { public JsonNumber { java.util.Objects.requireNonNull(value); } public String toString() { return value.toString(); } }
