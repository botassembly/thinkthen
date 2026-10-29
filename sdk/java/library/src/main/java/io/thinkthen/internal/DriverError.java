package io.thinkthen.internal;

public record DriverError(int value, String name, String message, Integer httpStatus) {}
