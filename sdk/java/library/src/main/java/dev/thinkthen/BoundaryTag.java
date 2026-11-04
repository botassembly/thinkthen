package dev.thinkthen;

/**
 * A recognition boundary label reported by the engine.
 */
public enum BoundaryTag {
    /** The beginning of a mention. */
    BEGIN,
    /** The end of a mention. */
    END,
    /** Inside a mention. */
    INSIDE,
    /** Outside a mention. */
    OUT,
    /** A single-piece mention. */
    SINGLE
}
