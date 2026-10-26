package io.thinkthen;

public record RankDetail(java.util.List<Double> levels) implements Detail { public RankDetail { levels = java.util.List.copyOf(levels); } }
