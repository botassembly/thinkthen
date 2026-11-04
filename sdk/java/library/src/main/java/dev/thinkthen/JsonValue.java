package dev.thinkthen;

/**
 * An immutable JSON value that preserves its JSON type and exact numeric representation.
 */
public sealed interface JsonValue permits JsonNull, JsonBoolean, JsonNumber, JsonString, JsonArray, JsonObject {
    /** Read one JSON value, preserving JSON types and numeric precision.
     * @param json the JSON text
     * @return the parsed value
     */
    static JsonValue parse(String json) { return Values.json(Values.parse(json)); }
}
