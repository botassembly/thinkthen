package dev.thinkthen;

/**
 * A JSON string.
 * @param value the value value
 */
public record JsonString(String value) implements JsonValue {
    /** Create an immutable value snapshot.
     * @param value the value value

     */
    public JsonString { java.util.Objects.requireNonNull(value,"value"); }
    /**
     * Return the to string value.
     * @return the toString result
     */
    public String toString() { return Values.stringify(value); } }
