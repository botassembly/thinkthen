package io.thinkthen.internal;

public record DriverError(int value, String name, String family, String message, Integer httpStatus) {}
