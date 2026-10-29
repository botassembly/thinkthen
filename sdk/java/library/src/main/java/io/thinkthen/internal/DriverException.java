package io.thinkthen.internal;

@SuppressWarnings("serial")
public final class DriverException extends RuntimeException {
    private static final long serialVersionUID = 1L;
    private final DriverError error;
    DriverException(ThinkThenException cause) {
        super(cause.getMessage(), cause);
        int value = 0;
        for (var code : DriverCode.values()) if (code.name().equals(cause.code())) value = code.value();
        error = new DriverError(value,cause.code(),cause.getMessage(),cause.httpStatus());
    }
    public DriverError error() { return error; }
}
