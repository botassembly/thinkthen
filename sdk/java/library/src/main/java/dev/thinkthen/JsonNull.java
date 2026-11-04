package dev.thinkthen;

/**
 * The JSON null value.
 */
public record JsonNull() implements JsonValue {
    /**
     * Return the to string value.
     * @return the toString result
     */
    public String toString() { return "null"; } }
