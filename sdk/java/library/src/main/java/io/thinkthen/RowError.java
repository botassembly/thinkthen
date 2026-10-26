package io.thinkthen;

public record RowError(ErrorCode code, String message, java.util.OptionalInt httpStatus) {}
