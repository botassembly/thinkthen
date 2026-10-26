package io.thinkthen;

public record JsonArray(java.util.List<JsonValue> values) implements JsonValue { public JsonArray { values = java.util.List.copyOf(values); } public String toString() { return "[" + values.stream().map(Object::toString).collect(java.util.stream.Collectors.joining(",")) + "]"; } }
