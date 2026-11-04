package dev.thinkthen;

/**
 * An immutable JSON object preserving field order.
 * @param values the values value
 */
public record JsonObject(java.util.Map<String,JsonValue> values) implements JsonValue {
    /** Create an immutable value snapshot.
     * @param values the values value

     */
    public JsonObject { values = Values.orderedMap(values); }
    /**
     * Return the to string value.
     * @return the toString result
     */
    public String toString() { return "{" + values.entrySet().stream().map(e -> Values.stringify(e.getKey()) + ":" + e.getValue()).collect(java.util.stream.Collectors.joining(",")) + "}"; } }
