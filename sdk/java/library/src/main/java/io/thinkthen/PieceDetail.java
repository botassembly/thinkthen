package io.thinkthen;

public record PieceDetail(Span span, java.util.Map<BoundaryTag,Double> tags) { public PieceDetail { tags = java.util.Map.copyOf(tags); } }
