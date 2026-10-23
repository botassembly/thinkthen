package io.thinkthen;
import org.jspecify.annotations.Nullable;
public record Relation(RelationName source, RelationName target, String relation, double p, @Nullable Boolean either) {}
