package io.thinkthen;

public record TagDetail(java.util.Map<String,Double> labels) implements Detail { public TagDetail { labels = java.util.Map.copyOf(labels); } }
