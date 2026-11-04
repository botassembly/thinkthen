package dev.thinkthen;

/**
 * A JSON Boolean.
 * @param value the value value
 */
public record JsonBoolean(boolean value) implements JsonValue {
    /**
     * Return the to string value.
     * @return the toString result
     */
    public String toString() { return Boolean.toString(value); } }
