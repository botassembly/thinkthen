package io.thinkthen.internal;
import org.jspecify.annotations.Nullable;
public record Relation(RelationName source, RelationName target, String relation, @com.fasterxml.jackson.annotation.JsonProperty("probability") double p, @Nullable Boolean either) {}
