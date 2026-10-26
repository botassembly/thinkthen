package io.thinkthen;

public record KindDetail(int start, int end, java.util.Map<String,Double> kinds) { public KindDetail { kinds = java.util.Map.copyOf(kinds); } }
