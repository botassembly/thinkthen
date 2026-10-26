package io.thinkthen.internal;
import org.jspecify.annotations.Nullable;
public record Rule(@com.fasterxml.jackson.annotation.JsonProperty("relation") String name, String source, String target, @Nullable String reads, @Nullable Boolean either) {}
