package io.thinkthen;

@FunctionalInterface
public interface RecordCodec { String toJson(Object value); }
