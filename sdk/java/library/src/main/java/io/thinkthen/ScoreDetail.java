package io.thinkthen;

public record ScoreDetail(java.util.List<Double> levels) implements Detail { public ScoreDetail { levels = java.util.List.copyOf(levels); } }
