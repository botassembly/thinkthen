package io.thinkthen.internal;

public final class ThinkThenException extends RuntimeException {
    private static final long serialVersionUID = 1L;
    private final String code;
    private final @org.jspecify.annotations.Nullable Integer httpStatus;
    ThinkThenException(String code, String message) { this(code, message, null); }
    ThinkThenException(String code, String message, @org.jspecify.annotations.Nullable Integer httpStatus) {
        super(message);
        this.code = code;
        this.httpStatus = httpStatus;
    }
    public @org.jspecify.annotations.Nullable Integer httpStatus() { return httpStatus; }
    public String code() { return code; }
}
