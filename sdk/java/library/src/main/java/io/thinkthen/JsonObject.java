package io.thinkthen;

public record JsonObject(java.util.Map<String,JsonValue> values) implements JsonValue { public JsonObject { values = java.util.Map.copyOf(values); } public String toString() { return "{" + values.entrySet().stream().map(e -> Values.stringify(e.getKey()) + ":" + e.getValue()).collect(java.util.stream.Collectors.joining(",")) + "}"; } }
