package dev.thinkthen;

/**
 * Input identity and source metadata for one engine outcome.
 * @param function the function value
 * @param position the position value
 * @param id the id value
 * @param source the source value
 */
public record RowInfo(FunctionName function, long position, java.util.Optional<JsonValue> id, java.util.Optional<Source> source) {}
