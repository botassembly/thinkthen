package thinkthen;

/** Immediate native failure. Session failures retain their generated terminal. */
public final class NativeFailure extends RuntimeException {
    private static final long serialVersionUID = 1L;
    private final int code;
    private final boolean retryable;
    private final transient Object facts;
    public NativeFailure(int code, boolean retryable, String message, Object facts) {
        super(message);
        this.code = code; this.retryable = retryable; this.facts = Values.freeze(facts);
    }
    public ErrorKind kind() { return ErrorKind.read(code); }
    public int code() { return code; }
    public boolean retryable() { return retryable; }
    public Object facts() { return facts; }
}
