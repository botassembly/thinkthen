package dev.thinkthen;

/**
 * An immutable JSON array.
 * @param values the values value
 */
public record JsonArray(java.util.List<JsonValue> values) implements JsonValue {
    /** Create an immutable value snapshot.
     * @param values the values value

     */
    public JsonArray { values = java.util.List.copyOf(values); }
    /**
     * Return the to string value.
     * @return the toString result
     */
    public String toString() { return "[" + values.stream().map(Object::toString).collect(java.util.stream.Collectors.joining(",")) + "]"; } }
