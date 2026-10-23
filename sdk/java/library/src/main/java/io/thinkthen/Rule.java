package io.thinkthen;
import org.jspecify.annotations.Nullable;
public record Rule(String name, String source, String target, @Nullable String reads, @Nullable Boolean either) {}
