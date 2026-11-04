package dev.thinkthen;

/**
 * A shared engine error, including its stable code and optional HTTP status.
 * @param code the code value
 * @param message the message value
 * @param httpStatus the httpStatus value
 */
public record RowError(ErrorCode code, String message, java.util.OptionalInt httpStatus) {}
