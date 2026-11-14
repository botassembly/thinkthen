package dev.thinkthen;

/** Failed records, including every failure from a completed filter. */
@SuppressWarnings("serial")
public final class FailedException extends NoAnswerException {
    private final RowError error;
    private final java.util.List<Failed<?,?>> rows;
    FailedException(RowError error, java.util.List<? extends Failed<?,?>> rows) { super(error.message()); this.error = error; this.rows = java.util.List.copyOf(rows); }
    public ErrorCode code() { return error.code(); }
    public java.util.OptionalInt httpStatus() { return error.httpStatus(); }
    public java.util.List<Failed<?,?>> rows() { return rows; }
}
