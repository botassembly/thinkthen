package dev.thinkthen;

/**
 * An unchecked engine failure with its stable code and optional failed filter rows.
 */
@SuppressWarnings("serial")
public class ThinkThenException extends RuntimeException {
    private static final long serialVersionUID = 1L;
    /** The shared engine failure. */
    private final RowError error;
    ThinkThenException(RowError error, @org.jspecify.annotations.Nullable Throwable cause) { super(error.message(),cause); this.error = error; }
    /** The shared error family.
     * @return the family result
     */
    public ErrorCode.Family family() { return error.code().family(); }
    /**
     * Return the stable shared engine error code.
     * @return the code result
     */
    public ErrorCode code() { return error.code(); }
    /**
     * Return the backend HTTP status when one exists.
     * @return the httpStatus result
     */
    public java.util.OptionalInt httpStatus() { return error.httpStatus(); }
}
