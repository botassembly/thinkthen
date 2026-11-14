package dev.thinkthen;

/** A internal failure carrying the shared engine diagnostic. */
@SuppressWarnings("serial")
public final class InternalException extends ThinkThenException {
    InternalException(RowError error, @org.jspecify.annotations.Nullable Throwable cause) { super(error,cause); }
}
