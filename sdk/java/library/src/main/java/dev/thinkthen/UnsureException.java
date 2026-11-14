package dev.thinkthen;

/** The result has no sufficiently likely answer. This is not a call failure. */
@SuppressWarnings("serial")
public final class UnsureException extends NoAnswerException {
    private final Row<?,?> row;
    UnsureException(Row<?,?> row) { super("The answer is unsure; check isUnsure() before reading it"); this.row = row; }
    public double probability() { return row.probability().orElseThrow(); }
    public Row<?,?> row() { return row; }
}
