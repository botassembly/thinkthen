package dev.thinkthen;

/** A setup failure carrying the shared engine diagnostic. */
@SuppressWarnings("serial")
public final class SetupException extends ThinkThenException {
    SetupException(RowError error, @org.jspecify.annotations.Nullable Throwable cause) { super(error,cause); }
}
