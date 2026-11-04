package dev.thinkthen;

/** The answer to one named question in an annotation.
     * @param <A> the answer type
     * @param <D> the detail type
     */
public sealed interface AnnotationOutcome<A,D extends Detail> permits AnnotationAnswered, AnnotationUnsure, AnnotationFailed {
    /** The engine outcome for this question.
     * @return the status result
     */
    Status status();
    /** Return the plain answer; throw if this outcome is unsure or failed.
     * @return the answer result
     */
    default A answer() {
        return switch (this) {
            case AnnotationAnswered<A,D> value -> value.answer();
            case AnnotationUnsure<A,D> ignored -> throw new UnsureException(ignored.probability().orElseThrow());
            case AnnotationFailed<A,D> value -> throw new ThinkThenException(value.error(),null);
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
            case AnnotationAnswered<A,D> value -> value.detail();
            case AnnotationUnsure<A,D> value -> value.detail();
            case AnnotationFailed<A,D> value -> throw new ThinkThenException(value.error(),null);
        };
    }
    /** Whether this question has an answer.
     * @return the isAnswered result
     */
    default boolean isAnswered() { return status() == Status.ANSWERED; }
    /** Whether this question needs a decision from the caller.
     * @return the isUnsure result
     */
    default boolean isUnsure() { return status() == Status.UNSURE; }
    /** Whether this question failed.
     * @return the isFailed result
     */
    default boolean isFailed() { return status() == Status.FAILED; }
    /** Throw the engine error when this question failed. */
    default void throwIfFailed() { if (this instanceof AnnotationFailed<A,D> value) throw new ThinkThenException(value.error(),null); }
}
