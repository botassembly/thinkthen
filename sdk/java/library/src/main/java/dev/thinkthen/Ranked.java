package dev.thinkthen;

/** An owned input snapshot and its rank, including failures.
     * @param <T> the snapshot type
     * @param item the owned snapshot value
     * @param row the row value
     */
public record Ranked<T>(T item, Row<Long,LevelDetail> row) {
    /** Return the answer, throwing on unsure or failed rows.
     * @return the answer result
     */
    public Long answer() { return row.answer(); }
    /** The engine's one-based input position.
     * @return the position result
     */
    public long position() { return row.position(); }
    /** The outcome for this original item.
     * @return the status result
     */
    public Status status() { return row.status(); }
    /** The confidence reported by the engine, when present.
     * @return the probability result
     */
    public java.util.OptionalDouble probability() { return row.probability(); }
    /** Whether the engine was unsure about this item.
     * @return the isUnsure result
     */
    public boolean isUnsure() { return row.isUnsure(); }
    /** Whether this item has an answer.
     * @return whether this item has an answer
     */
    public boolean isAnswered() { return row.isAnswered(); }
    /** Whether this item failed.
     * @return whether this item failed
     */
    public boolean isFailed() { return row.isFailed(); }
    /** Throw the original engine error if this item failed. */
    public void throwIfFailed() { row.throwIfFailed(); }
}
