package dev.thinkthen;

/**
 * A JSON pointer selecting record field.
 * @param value the value value
 */
public record Pointer(String value) {
    /**
     * Create this typed value.
     * @param value the value to set
     * @return the of result
     */
    public static Pointer of(String value) { return new Pointer(java.util.Objects.requireNonNull(value,"value")); } }
