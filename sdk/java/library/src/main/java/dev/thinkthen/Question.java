package dev.thinkthen;

/**
 * A typed question with its function-specific answer and detail types.
     * @param <A> the answer type
     * @param <D> the detail type
     */
public sealed interface Question<A,D extends Detail> permits AnnotationQuestion, RankQuestion {
    /** Return this question’s engine function.
     * @return the engine function
     */
    FunctionName function();
}
