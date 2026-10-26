package io.thinkthen;

@SuppressWarnings("serial")
public final class ThinkThenException extends RuntimeException {
    private static final long serialVersionUID = 1L;
    private final RowError error;
    ThinkThenException(RowError error, @org.jspecify.annotations.Nullable Throwable cause) { super(error.message(),cause); this.error = error; }
    public ErrorCode code() { return error.code(); }
    public java.util.OptionalInt httpStatus() { return error.httpStatus(); }
}
