package dev.thinkthen;

/**
 * An exact JSON number.
 * @param value the value value
 */
public record JsonNumber(java.math.BigDecimal value) implements JsonValue {
    /** Create an immutable value snapshot.
     * @param value the value value

     */
    public JsonNumber { java.util.Objects.requireNonNull(value,"value"); }
    /**
     * Return the to string value.
     * @return the toString result
     */
    public String toString() { return value.toString(); } }
