package dev.thinkthen;

/** A usage failure carrying the shared engine diagnostic. */
@SuppressWarnings("serial")
public final class UsageException extends ThinkThenException {
    UsageException(RowError error, @org.jspecify.annotations.Nullable Throwable cause) { super(error,cause); }
}
