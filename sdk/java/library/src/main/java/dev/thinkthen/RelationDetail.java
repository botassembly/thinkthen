package dev.thinkthen;

/**
 * Recognized names and candidate relations for a relation answer.
 * @param names the names value
 * @param pairs the pairs value
 */
public record RelationDetail(java.util.List<Entity> names, java.util.List<Relation> pairs) implements Detail {
    /** Create an immutable value snapshot.
     * @param names the names value

     */
    public RelationDetail { names = java.util.List.copyOf(names); pairs = java.util.List.copyOf(pairs); } }
