package dev.thinkthen;

/**
 * Whether an engine outcome is answered, unsure or failed.
 */
public enum Status {
    /** The engine produced an answer. */
    ANSWERED,
    /** The answer falls in the uncertainty band. */
    UNSURE,
    /** The record failed. */
    FAILED
}
