package dev.thinkthen;

/**
 * An immutable engine outcome with its input identity and source. Reading an unsure or failed answer throws.
     * @param <A> the answer type
     * @param <D> the detail type
     */
public sealed interface Row<A,D extends Detail> permits Answered, Unsure, Failed {
    /**
     * Return the info value.
     * @return the info result
     */
    RowInfo info();
    /**
     * Return the engine function.
     * @return the function result
     */
    default FunctionName function() { return info().function(); }
    /**
     * Return the position value.
     * @return the position result
     */
    default long position() { return info().position(); }
    /**
     * Return the id value.
     * @return the id result
     */
    default java.util.Optional<JsonValue> id() { return info().id(); }
    /**
     * Return the source value.
     * @return the source result
     */
    default java.util.Optional<Source> source() { return info().source(); }
    /**
     * Return whether this outcome is answered, unsure or failed.
     * @return the status result
     */
    Status status();
    /** Return the plain answer; throw if this row is unsure or failed.
     * @return the answer result
     */
    default A answer() {
        return switch (this) {
            case Answered<A,D> value -> value.answer();
            case Unsure<A,D> ignored -> throw new UnsureException(ignored.probability().orElseThrow());
            case Failed<A,D> value -> throw new ThinkThenException(value.error(),null);
        };
    }
    /** Probability of yes for decide/filter, or confidence in the selected answer.
     * @return the probability result
     */
    java.util.OptionalDouble probability();
    /** Return the optional detail; throw the engine error when this outcome failed.
     * @return the detail when present
     */
    default java.util.Optional<D> detail() {
        return switch (this) {
            case Answered<A,D> value -> value.detail();
            case Unsure<A,D> value -> value.detail();
            case Failed<A,D> value -> throw new ThinkThenException(value.error(),null);
        };
    }
    /** Whether this row has an answer.
     * @return the isAnswered result
     */
    default boolean isAnswered() { return status() == Status.ANSWERED; }
    /** Whether this row needs a decision from the caller.
     * @return the isUnsure result
     */
    default boolean isUnsure() { return status() == Status.UNSURE; }
    /** Whether processing this record failed.
     * @return the isFailed result
     */
    default boolean isFailed() { return status() == Status.FAILED; }
    /**
     * Throw the original engine failure if this outcome failed.
     */
    default void throwIfFailed() { if (this instanceof Failed<A,D> failed) throw new ThinkThenException(failed.error(),null); }
}
