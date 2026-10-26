package io.thinkthen;

public record ChooseDetail(java.util.Map<String,Double> labels) implements Detail { public ChooseDetail { labels = java.util.Map.copyOf(labels); } }
