package dev.thinkthen;

/**
 * A supported engine backend. STUB performs local deterministic calls.
 */
public enum Backend {
    /** Use the System One backend. */
    SYSTEMONE,
    /** Use the OpenAI backend. */
    OPENAI,
    /** Use the local deterministic backend. */
    STUB
; String wireName() { return name().toLowerCase(java.util.Locale.ROOT); } }
