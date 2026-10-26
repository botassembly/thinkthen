package io.thinkthen;

public record RelateDetail(java.util.List<Entity> names, java.util.List<Relation> pairs) implements Detail { public RelateDetail { names = java.util.List.copyOf(names); pairs = java.util.List.copyOf(pairs); } }
