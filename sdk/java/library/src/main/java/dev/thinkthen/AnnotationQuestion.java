package dev.thinkthen;

/**
 * A typed question that can be included in an annotation.
     * @param <A> the answer type
     * @param <D> the detail type
     */
public sealed interface AnnotationQuestion<A,D extends Detail> extends Question<A,D> permits DecideQuestion, ChooseQuestion, TagQuestion, ScoreQuestion, FindQuestion, RecognizeQuestion, RelateQuestion {}
