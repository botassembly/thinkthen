package io.thinkthen;

public record RowInfo(Function function, long position, java.util.Optional<JsonValue> id, java.util.Optional<Source> source) {}
