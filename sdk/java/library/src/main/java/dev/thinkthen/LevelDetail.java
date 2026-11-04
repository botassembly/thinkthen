package dev.thinkthen;

/**
 * Probabilities in the same order as the question’s required levels.
 * @param levels the levels value
 */
public record LevelDetail(java.util.List<Double> levels) implements Detail {
    /** Create an immutable value snapshot.
     * @param levels the levels value

     */
    public LevelDetail { levels = java.util.List.copyOf(levels); } }
