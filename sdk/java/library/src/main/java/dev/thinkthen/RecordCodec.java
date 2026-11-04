package dev.thinkthen;

/**
 * Convert a host object to JSON without exposing JSON-library types in the public API.
 */
@FunctionalInterface
public interface RecordCodec {
    /** Convert a host record to JSON.
     * @param value the host record
     * @return its lossless JSON representation
     */
    String toJson(Object value);
}
